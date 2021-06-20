// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.

#[cfg(test)]
mod tests {
    use byteorder::{ByteOrder, LittleEndian};

    fn test_layer_stress(layer: &mut Box<dyn argus::layer::Layer>, count: u32) {
        let total: u64 = layer.count();
        let mut inc_count: u64;

        for index in 0..count {
            let mut data = [0; 4];
            // encode the u32 to Vec<u8> as little endian
            LittleEndian::write_u32(&mut data, index);

            assert!(layer.set(&data.to_vec(), &data.to_vec()).is_ok());
            assert_eq!(layer.count(), (index as u64) + 1);
        }

        inc_count = 0;
        for _ in layer.keys() {
            inc_count += 1
        }
        assert_eq!(
            inc_count - total,
            count as u64,
            "increase count not equals: {} / {}",
            inc_count - total,
            count
        );

        for index in 0..count {
            let mut data = [0; 4];
            // encode the u32 to Vec<u8> as little endian
            LittleEndian::write_u32(&mut data, index);
            let key = data.to_vec();

            let get_status = layer.get(&key);
            assert!(get_status.is_ok());
            assert!(get_status.unwrap().is_some());

            let mut del_status = layer.del(&key);
            assert!(del_status.is_ok());
            assert_eq!(del_status.unwrap(), true);

            del_status = layer.del(&key);
            assert!(del_status.is_ok());
            assert_eq!(del_status.unwrap(), false);
        }

        assert_eq!(layer.count(), 0);
        inc_count = 0;
        for _ in layer.keys() {
            inc_count += 1
        }
        assert_eq!(
            inc_count - total,
            0,
            "increase count not equals: {} / 0",
            inc_count - total,
        );
    }

    fn test_layer_basic(layer: &mut Box<dyn argus::layer::Layer>) {
        let key: Vec<u8> = vec![1, 2, 3];
        let value: Vec<u8> = vec![0, 0, 0];

        let mut get_status = layer.get(&key);
        assert!(get_status.is_ok());
        assert!(get_status.unwrap().is_none());
        assert_eq!(layer.count(), 0);

        assert!(layer.set(&key, &value).is_ok());
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
        assert!(get_status.unwrap().is_some());
        assert_eq!(layer.count(), 0);
    }

    #[test]
    fn test_not_exist() {
        let layer = argus::layer::new("not-exist");
        assert!(layer.is_none());
    }

    #[test]
    fn test_memory_layer() {
        let mut layer = argus::layer::new("mem").unwrap();

        test_layer_basic(&mut layer);
        test_layer_stress(&mut layer, 128);
    }
}

// vim: set tabstop=4 sw=4 expandtab:
