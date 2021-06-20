// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
#[cfg(test)]
mod tests {
    use byteorder::{ByteOrder, LittleEndian};

    fn _test_lsm(layer: &mut argus::lsm::LSM) {
        let key: Vec<u8> = vec![0, 0, 0];
        let value: Vec<u8> = vec![1, 2, 3];

        let mut get_status = layer.get(&key);
        assert!(get_status.is_ok());
        assert!(get_status.unwrap().is_none());
        assert_eq!(layer.count(), 0);

        let set_status = layer.set(&key, &value);
        assert!(
            set_status.is_ok(),
            "cannot set: {}",
            set_status.err().unwrap()
        );
        get_status = layer.get(&key);
        assert!(get_status.is_ok());
        assert!(get_status.unwrap().is_some());
        assert_eq!(layer.count(), 1);

        let mut del_status = layer.del(&key);
        assert!(del_status.is_ok());
        assert_eq!(del_status.unwrap(), true);
        assert_eq!(layer.count(), 0);

        del_status = layer.del(&key);
        assert!(del_status.is_ok());
        assert_eq!(del_status.unwrap(), false);
        assert_eq!(layer.count(), 0);

        get_status = layer.get(&key);
        assert!(get_status.is_ok());
        assert!(get_status.unwrap().is_none());
        assert_eq!(layer.count(), 0);
    }

    fn _test_stress_lsm(layer: &mut argus::lsm::LSM, count: u32) {
        for c in 0..count {
            let mut data = [0; 4];
            // encode the u32 to Vec<u8> as little endian
            LittleEndian::write_u32(&mut data, c);

            let resp = layer.set(&data.to_vec(), &data.to_vec());
            assert!(
                resp.is_ok(),
                "cannot not set #{} item: {}",
                c,
                resp.err().unwrap()
            );
        }
    }

    #[test]
    fn test_default_lsm() {
        let mut layer: argus::lsm::LSM = argus::lsm::LSM::mem();
        _test_lsm(&mut layer);
        _test_stress_lsm(&mut layer, 1024);
    }

    #[test]
    fn test_multi_mem_layer() {
        let mut layer: argus::lsm::LSM = argus::lsm::LSM::new()
            .add_layer(argus::layer::memory::NAME, 2)
            .unwrap()
            .add_layer(argus::layer::memory::NAME, 4)
            .unwrap()
            .add_layer(argus::layer::memory::NAME, 0)
            .unwrap();

        _test_lsm(&mut layer);
        _test_stress_lsm(&mut layer, 1024);
    }

    #[test]
    #[should_panic]
    fn test_non_exist_layer() {
        argus::lsm::LSM::new()
            .add_layer("NOT EXIST LAYER", 0)
            .expect("should not get the layer");
    }

    #[test]
    fn test_empty_layer_lsm() {
        let key: Vec<u8> = vec![0, 0, 0];
        let value: Vec<u8> = vec![1, 2, 3];

        let mut layer: argus::lsm::LSM = argus::lsm::LSM::new();

        assert!(layer.get(&key).is_ok());
        assert!(layer.get(&key).unwrap().is_none());
        assert!(layer.set(&key, &value).is_err());
        assert!(layer.del(&key).is_ok());
        assert_eq!(layer.del(&key).unwrap(), false);
        assert_eq!(layer.count(), 0);
    }
}
