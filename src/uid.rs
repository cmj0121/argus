// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! The UID generator
use chrono::{DateTime, NaiveDateTime, Utc};
use log::{info, trace, warn};
use rand::{thread_rng, Rng};
use std::fmt;

/// The UID (unique identifier) is the 128-bit fixed-length time-based sortable
/// identifier which is encoded to Crockford Base32 by-default.
///
///  The UID contains the 48-bit time information and 80 bit randomness bits, which
///  has 8-bits used for the cluster and 8-bits used for the process.
#[derive(Clone, PartialEq, Eq)]
pub struct Uid(pub u128);

impl Uid {
    // the built-in 128-bits layout
    pub const TIME_BITS: u8 = 48;
    pub const RAND_BITS: u8 = 80;
    // the customized 16-bits used in the randomness bits
    pub const CLUSTER_ID_BITS: u8 = 8;
    pub const PROCESS_ID_BITS: u8 = 8;

    pub const UID_LEN: usize = 26;
    const ALPHABET: &'static [u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

    /// get the cluster ID
    pub fn cluster_id(&self) -> u8 {
        let cid: u8 = ((self.0 >> Uid::PROCESS_ID_BITS) & ((1 << Uid::CLUSTER_ID_BITS) - 1)) as u8;
        cid
    }

    /// get the process ID
    pub fn process_id(&self) -> u8 {
        let pid: u8 = (self.0 & ((1 << Uid::PROCESS_ID_BITS) - 1)) as u8;
        pid
    }
}

/// To use the `{}` marker the trait `fmt::Display` MUST BE implemented
/// by manually  for the UID type.
///
/// ref: https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
impl fmt::Display for Uid {
    // This trait requires `fmt` with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buff: [u8; Uid::UID_LEN] = [0; Uid::UID_LEN];
        let mut value: u128 = self.0;

        for idx in 0..Uid::UID_LEN {
            buff[Uid::UID_LEN - idx - 1] = Uid::ALPHABET[(value & 0x1F) as usize];
            value >>= 5;
        }

        let val = String::from_utf8(buff.to_vec())
            .expect("unexpected error when encoding Crockford's base32");
        // write the Crockford's base32 encoding as the UID string
        write!(f, "{}", val)
    }
}

impl fmt::Debug for Uid {
    // This trait requires `fmt` with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write the Crockford's base32 encoding as the UID string
        write!(
            f,
            "{} (T: {}, CID: {}, PID: {})",
            self,
            DateTime::<Utc>::from(self).to_rfc3339(),
            self.cluster_id(),
            self.process_id(),
        )
    }
}

/// Transfer the UID to the DateTime<Utc>
impl From<&Uid> for DateTime<Utc> {
    fn from(uid: &Uid) -> Self {
        let ms: u128 = uid.0 >> Uid::RAND_BITS;
        let sec: i64 = (ms / 1000) as i64;
        let nsec: u32 = ((ms % 1000) * 1000) as u32;

        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(sec, nsec), Utc)
    }
}

/// #panic
///
/// Parse the string as the UID, raise panic if the format is invalid.
impl From<String> for Uid {
    fn from(s: String) -> Self {
        if s.len() != Uid::UID_LEN {
            // not the valid UID check by the length
            panic!("invalid UID: {}", s);
        }

        let mut val: u128 = 0;
        let bytes: &[u8] = s.as_bytes();
        for idx in 0..Uid::UID_LEN {
            let pos: u128 = match Uid::ALPHABET.iter().position(|&x| x == bytes[idx]) {
                None => panic!("invalid UID: {}", s),
                Some(x) => x as u128,
            };

            val = (val << 5) | pos;
        }

        Self(val)
    }
}

/// The Uid builder and be used to generate the UID. The builder can
/// generate tye UID by many methods, include the basic `gen` to generate
/// the UID without monotonic.
pub struct UidBuilder {
    // the random generator
    rng: rand::rngs::ThreadRng,
    // record as the latest uid to make sure the monotonic
    latest: u128,
    // the customized cluster / process id in the randomness bits
    cluster_id: Option<u8>,
    process_id: Option<u8>,
}

impl UidBuilder {
    /// specified the cluster ID used in the UID builder
    pub fn cluster_id(self, cluster_id: u8) -> Self {
        trace!("create UID builder with cid: {}", cluster_id);
        Self {
            cluster_id: Some(cluster_id),
            ..self
        }
    }

    /// specified the process ID used in the UID builder
    pub fn process_id(self, process_id: u8) -> Self {
        trace!("create UID builder with pid: {}", process_id);
        Self {
            process_id: Some(process_id),
            ..self
        }
    }

    /// generate the UID
    pub fn gen(&mut self) -> Uid {
        // generate the UID by the non-strict mode
        self.gen_by_strict(false).unwrap()
    }

    pub fn gen_by_strict(&mut self, strict: bool) -> Option<Uid> {
        let ms: u64 = Utc::now().timestamp_millis() as u64;
        self.gen_by_ms(ms, strict)
    }

    pub fn gen_by_ms(&mut self, ms: u64, strict: bool) -> Option<Uid> {
        let mut randomness: u128 = self.rng.gen();
        randomness &= (1 << Uid::RAND_BITS) - 1;

        loop {
            let uid = self.gen_uid(ms, randomness);

            if strict && (uid >> Uid::RAND_BITS) == (self.latest >> Uid::RAND_BITS) {
                trace!("same milliseconds: {}", uid >> Uid::RAND_BITS);

                if uid < self.latest {
                    info!("should be monotonic: {:X} < {:X}", uid, self.latest);
                    randomness = match self.monotonic_rand() {
                        None => {
                            // reach the maximal UID in the current milliseconds
                            warn!("build-out the UID on {}", ms);
                            return None;
                        }
                        Some(r) => r,
                    };
                    continue;
                }
            }

            self.latest = uid;
            break;
        }

        Some(Uid(self.latest))
    }

    /// generate the UID manual
    pub fn gen_uid(&self, ms: u64, randomness: u128) -> u128 {
        let mut uid: u128 = 0;

        uid |= (ms as u128) << Uid::RAND_BITS;
        uid |= self.rand_bits(randomness);

        uid
    }

    // re-generate the randomness by the current CID/PID setting
    fn rand_bits(&self, randomness: u128) -> u128 {
        let mut rand: u128 = randomness;

        if self.cluster_id.is_some() {
            trace!("define the CID, override");
            rand &= !((((1 << Uid::CLUSTER_ID_BITS) - 1) as u128) << Uid::PROCESS_ID_BITS);
            rand |= (self.cluster_id.unwrap() as u128) << Uid::PROCESS_ID_BITS;
        }

        if self.process_id.is_some() {
            trace!("define the CID, override");
            rand &= !(((1 << Uid::PROCESS_ID_BITS) - 1) as u128);
            rand |= self.process_id.unwrap() as u128;
        }

        rand &= (1 << Uid::RAND_BITS) - 1;
        trace!("set the randomness: {:X}", rand);
        rand
    }

    // get the maximal randomness in the current builder setting
    fn max_rand(&self) -> u128 {
        let max: u128 = (1 << Uid::RAND_BITS) - 1;

        self.rand_bits(max)
    }

    // generate the monotonic randomness
    fn monotonic_rand(&mut self) -> Option<u128> {
        let randomness = self.latest & ((1 << Uid::RAND_BITS) - 1);
        let max_rand = self.max_rand();

        trace!(
            "try set the monotonic rand between: {:X} ~ {:X}",
            randomness,
            max_rand
        );
        match max_rand - randomness {
            d if d <= 0 => None,
            _ => Some(self.rng.gen_range(randomness + 1..max_rand + 1)),
        }
    }
}

/// create the UID builder with default config
pub fn builder() -> UidBuilder {
    UidBuilder {
        rng: thread_rng(),
        latest: 0,
        cluster_id: None,
        process_id: None,
    }
}

// vim: set tabstop=4 sw=4 expandtab:
