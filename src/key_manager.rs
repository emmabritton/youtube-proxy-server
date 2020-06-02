use std::collections::HashMap;

pub struct KeyManager {
    key_quotas: HashMap<String, usize>,
    last_used: usize,
}

impl KeyManager {
    pub fn new(keys: Vec<String>) -> KeyManager {
        let mut map = HashMap::with_capacity(keys.len());
        for key in keys {
            map.insert(key, 10000);
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

    pub fn get_key(&mut self, cost: usize) -> Option<String> {
        let keys: Vec<String> = self.key_quotas.iter()
            .map(|(key, _)| key)
            .cloned()
            .collect();

        for i in self.last_used..self.key_quotas.len() {
            let quota = self.key_quotas[&keys[i]];
            let key = keys[i].clone();
            if quota >= cost {
                self.key_quotas.insert(key.clone(), self.key_quotas[&key] - cost);
                self.set_last_used(i);
                return Some(key);
            }
        }
        None
    }

    pub fn set_key_as_expired(&mut self, key: String) {
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

    fn str_vec_to_string_vec(list: Vec<&'static str>) -> Vec<String> {
        list.iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_keys_are_rotated() {
        //GIVEN key manager with multiple keys
        let mut key_manager = KeyManager::new(str_vec_to_string_vec(vec!["key1", "key2"]));
        //WHEN multiple keys are requested
        let first = key_manager.get_key(100);
        let second = key_manager.get_key(100);
        //THEN check they are not the same one
        assert!(first.is_some());
        assert!(second.is_some());
        assert_ne!(first.unwrap(), second.unwrap());
    }

    #[test]
    fn test_consuming_keys() {
        //GIVEN key manager with one key
        let mut key_manager = KeyManager::new(str_vec_to_string_vec(vec!["key1"]));
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
        let mut key_manager = KeyManager::new(str_vec_to_string_vec(vec!["key1", "key2"]));
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
        let key_manager = KeyManager::new(str_vec_to_string_vec(vec!["key1", "key2"]));
        //WHEN keys are used
        let status = key_manager.get_status();
        //THEN check status output
        assert!(status.iter().any(|(k, q)| k == &0 && q == &10000));
        assert!(status.iter().any(|(k, q)| k == &1 && q == &10000));
    }
}