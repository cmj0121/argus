// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.

#[cfg(test)]
mod tests {
    #[test]
    fn test_lsm_tree() {
        let key: &Vec<u8> = &vec![1, 2, 3, 4];
        let value: &Vec<u8> = &vec![5, 5, 3, 8];
        let mut tree: argus::lsmt::LSMTree = argus::lsmt::LSMTree::new();

        assert!(tree.get(key).is_none());
        assert!(tree.set(key, value));
        assert!(tree.get(key).is_some());
        assert_eq!(tree.get(key).unwrap(), value);
        assert!(tree.del(key));
        assert!(tree.get(key).is_none());
    }

    #[test]
    fn test_lsm_tree_64() {
        let mut tree: argus::lsmt::LSMTree = argus::lsmt::LSMTree::new();

        for index in 0..64 {
            let key: &Vec<u8> = &(0..10).map(|v| v + index).collect();
            let value: &Vec<u8> = &vec![5, 5, 3, index];

            assert!(tree.get(key).is_none());
            assert!(tree.set(key, value));
            assert!(tree.get(key).is_some());
            assert_eq!(tree.get(key).unwrap(), value);
        }

        for index in 0..64 {
            let key: &Vec<u8> = &(0..10).map(|v| v + index).collect();

            assert!(tree.get(key).is_some());
            assert!(tree.del(key));
            assert!(tree.get(key).is_none());
        }

        for index in 0..64 {
            let key: &Vec<u8> = &(0..10).map(|v| v + index).collect();

            assert!(tree.get(key).is_none());
        }
    }
}

// vim: set tabstop=4 sw=4 expandtab:
