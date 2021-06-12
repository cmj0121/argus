// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.

#[cfg(test)]
mod tests {
    #[test]
    fn test_uid_nil() {
        let uid: argus::uid::Uid = argus::uid::Uid(0);

        assert_eq!(uid.to_string(), "00000000000000000000000000");

        let dup_uid: argus::uid::Uid = argus::uid::Uid::from(uid.to_string());
        assert_eq!(uid, dup_uid);
    }

    #[test]
    fn test_latest_uid() {
        let uid: argus::uid::Uid = argus::uid::Uid(!0);

        assert_eq!(uid.to_string(), "7ZZZZZZZZZZZZZZZZZZZZZZZZZ");

        let dup_uid: argus::uid::Uid = argus::uid::Uid::from(uid.to_string());
        assert_eq!(uid, dup_uid);
    }

    #[test]
    fn test_builder_with_pid() {
        let mut builder = argus::uid::builder().process_id(0x72);
        let uid: argus::uid::Uid = builder.gen();

        assert_eq!(uid.process_id(), 0x72);

        let dup_uid: argus::uid::Uid = argus::uid::Uid::from(uid.to_string());
        assert_eq!(uid, dup_uid);
    }

    #[test]
    fn test_builder_with_cid() {
        let mut builder = argus::uid::builder().cluster_id(0x72);
        let uid: argus::uid::Uid = builder.gen();

        assert_eq!(uid.cluster_id(), 0x72);

        let dup_uid: argus::uid::Uid = argus::uid::Uid::from(uid.to_string());
        assert_eq!(uid, dup_uid);
    }

    #[test]
    fn test_uid() {
        let mut b = argus::uid::builder();
        let uid: argus::uid::Uid = b.gen();

        let dup_uid: argus::uid::Uid = argus::uid::Uid::from(uid.to_string());
        assert_eq!(uid, dup_uid);
    }

    #[test]
    fn builder_builder() {
        let mut builder = argus::uid::builder();

        for _ in 0..32 {
            // can generate at-last 32 UID with strict
            assert!(builder.gen_by_strict(true).is_some());
        }

        for _ in 0..4096 {
            // can generate any number of UID without strict
            assert!(builder.gen_by_strict(false).is_some());
        }
    }
}

// vim: set tabstop=4 sw=4 expandtab:
