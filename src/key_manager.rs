use std::collections::HashMap;

const DEFAULT_QUOTA: usize = 10000;

pub struct KeyManager {
    key_quotas: HashMap<String, usize>,
    last_used: usize,
}

impl KeyManager {
    pub fn new(keys: Vec<String>) -> KeyManager {
        let mut map = HashMap::with_capacity(keys.len());
        for key in keys {
            map.insert(key, DEFAULT_QUOTA);
        }

        return KeyManager {
            key_quotas: map,
            last_used: 0,
        };
    }
}

impl KeyManager {
    fn set_last_used(&mut self, idx: usize) {
        self.last_used = idx + 1;
        if self.last_used >= self.key_quotas.len() {
            self.last_used = 0;
        }
    }

    pub fn reset_keys(&mut self) {
        self.key_quotas.iter_mut()
            .for_each(|(_, v)| *v = DEFAULT_QUOTA);
    }

    pub fn get_key(&mut self, cost: usize) -> Option<String> {
        let keys: Vec<String> = self.key_quotas.iter()
            .map(|(key, _)| key)
            .cloned()
            .collect();

        let last_key_used = self.last_used;
        let mut i = self.last_used;
        loop {
            let quota = self.key_quotas[&keys[i]];
            if quota >= cost {
                let key = keys[i].clone();
                self.key_quotas.insert(key.clone(), quota - cost);
                self.set_last_used(i);
                return Some(key);
            } else {
                i += 1;
                if i >= self.key_quotas.len() {
                    i = 0;
                }
                if i == last_key_used {
                    return None;
                }
            }
        }
    }

    pub fn set_key_as_expired(&mut self, key: String) {
        if self.key_quotas[&key] == DEFAULT_QUOTA {
            eprintln!("Key {} has probably permanently expired", key);
        }
        self.key_quotas.insert(key, 0);
    }

    pub fn get_status(&self) -> HashMap<usize, usize> {
        self.key_quotas
            .iter()
            .enumerate()
            .map(|(i, (_, v))| {
                (i, *v)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl KeyManager {
        pub(crate) fn new_test(list: Vec<&'static str>) -> KeyManager {
            let keys = list.iter()
                .map(|s| s.to_string())
                .collect();
            KeyManager::new(keys)
        }
    }

    #[test]
    fn test_keys_are_rotated() {
        //GIVEN key manager with multiple keys
        let mut key_manager = KeyManager::new_test(vec!["key1", "key2"]);
        //WHEN multiple keys are requested
        let first = key_manager.get_key(100);
        let second = key_manager.get_key(100);
        //THEN check they are not the same one
        assert!(first.is_some());
        assert!(second.is_some());
        assert_ne!(first.unwrap(), second.unwrap());
    }

    #[test]
    fn test_keys_are_rotated_in_loop() {
        //GIVEN key manager with multiple keys
        let mut key_manager = KeyManager::new_test(vec!["key1", "key2", "key3", "key4"]);
        //WHEN keys are requested at least twice
        let mut loop_keys = Vec::with_capacity(12);
        for i in 0..12 {
            loop_keys.insert(i, key_manager.get_key(100))
        }
        //THEN check keys are returned in same order
        assert!(loop_keys.iter().all(|key| key.is_some()));
        assert_eq!(loop_keys[0], loop_keys[4]);
        assert_eq!(loop_keys[0], loop_keys[8]);
        assert_eq!(loop_keys[1], loop_keys[5]);
        assert_eq!(loop_keys[1], loop_keys[9]);
        assert_eq!(loop_keys[2], loop_keys[6]);
        assert_eq!(loop_keys[2], loop_keys[10]);
        assert_eq!(loop_keys[3], loop_keys[7]);
        assert_eq!(loop_keys[3], loop_keys[11]);
    }

    #[test]
    fn test_key_rotation_after_expiry() {
        //GIVEN key manager with multiple keys
        let mut key_manager = KeyManager::new_test(vec!["key1", "key2", "key3"]);
        let first_key = key_manager.get_key(100);
        let second_key = key_manager.get_key(100);
        let third_key = key_manager.get_key(100);
        let first_key_again = key_manager.get_key(100);
        key_manager.set_key_as_expired(second_key.as_ref().unwrap().to_string());
        key_manager.set_key_as_expired(third_key.as_ref().unwrap().to_string());
        //WHEN all remaining keys in this loop are expired and a key is requested
        let key_after_expiry = key_manager.get_key(100);
        //THEN key should be key1
        assert!(first_key.is_some());
        assert!(second_key.is_some());
        assert!(third_key.is_some());
        assert!(first_key_again.is_some());
        assert!(key_after_expiry.is_some());
    }

    #[test]
    fn test_consuming_keys() {
        //GIVEN key manager with one key
        let mut key_manager = KeyManager::new_test(vec!["key1"]);
        //WHEN key is used up and another is requested
        let first = key_manager.get_key(10000);
        let second = key_manager.get_key(10000);
        //THEN check no key is returned
        assert!(first.is_some());
        assert!(second.is_none());
    }

    #[test]
    fn test_expiring_keys() {
        //GIVEN key manager with multiple keys
        let mut key_manager = KeyManager::new_test(vec!["key1", "key2"]);
        //WHEN the first key is expired and another is requested
        let first = key_manager.get_key(5000);
        let second = key_manager.get_key(5000);
        let third = key_manager.get_key(5000);
        let fourth = key_manager.get_key(5000);
        let fifth = key_manager.get_key(5000);
        //THEN check the second key is returned
        assert!(first.is_some());
        assert!(second.is_some());
        assert!(third.is_some());
        assert!(fourth.is_some());
        assert!(fifth.is_none());
    }

    #[test]
    fn test_status() {
        //GIVEN key manager with keys
        let key_manager = KeyManager::new_test(vec!["key1", "key2"]);
        //WHEN keys are used
        let status = key_manager.get_status();
        //THEN check status output
        assert!(status.iter().any(|(k, q)| k == &0 && q == &10000));
        assert!(status.iter().any(|(k, q)| k == &1 && q == &10000));
    }
}