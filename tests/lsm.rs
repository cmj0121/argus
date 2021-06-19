// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.

#[cfg(test)]
mod tests {
    fn _test_lsm(mut layer: argus::lsm::LSMTree) {
        let key: Vec<u8> = vec![0, 0, 0];
        let value: Vec<u8> = vec![1, 2, 3];

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
        assert!(get_status.unwrap().is_none());
        assert_eq!(layer.count(), 0);
    }

    #[test]
    fn test_default_lsm() {
        let layer: argus::lsm::LSMTree = argus::lsm::LSMTree::mem();
        _test_lsm(layer);
    }

    #[test]
    fn test_empty_layer_lsm() {
        let key: Vec<u8> = vec![0, 0, 0];
        let value: Vec<u8> = vec![1, 2, 3];

        let mut layer: argus::lsm::LSMTree = argus::lsm::LSMTree::new();

        assert!(layer.get(&key).is_ok());
        assert!(layer.get(&key).unwrap().is_none());
        assert!(layer.set(&key, &value).is_err());
        assert!(layer.del(&key).is_ok());
        assert_eq!(layer.del(&key).unwrap(), false);
        assert_eq!(layer.count(), 0);
    }
}
