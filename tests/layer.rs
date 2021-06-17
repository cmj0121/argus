// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_not_exist() {
        let layer = argus::layer::new("not-exist");
        assert!(layer.is_none());
    }

    #[test]
    fn test_memory_layer() {
        assert!(argus::layer::new("mem").is_some());

        let key: Vec<u8> = vec![1, 2, 3];
        let value: Vec<u8> = vec![0, 0, 0];
        let mut layer = argus::layer::new("mem").unwrap();

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
    fn test_memory_layer_save() {
        let mut layer = argus::layer::new("mem").unwrap();
        let rows: HashMap<Vec<u8>, Vec<u8>> = {
            let mut m: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
            m.insert(vec![0, 1, 2], vec![0, 1, 2]);
            m.insert(vec![0, 1, 3], vec![1, 1, 2]);
            m.insert(vec![0, 1, 4], vec![2, 1, 2]);
            m
        };

        assert!(layer.save("mem", &rows).is_ok());
    }

    #[test]
    fn test_memory_layer_trace() {
        let mut layer = argus::layer::new("mem").unwrap();

        for index in 0..128 {
            let key: Vec<u8> = (0..10).map(|v| v + index).collect();
            let value: Vec<u8> = vec![1, 2, 3];

            assert!(layer.set(&key, &value).is_ok());
            assert_eq!(layer.count(), (index as usize) + 1);
        }

        for index in 0..128 {
            let key: Vec<u8> = (0..10).map(|v| v + index).collect();

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
    }
}
