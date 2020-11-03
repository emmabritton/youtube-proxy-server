use chrono::{Timelike, Utc, Duration, DateTime};
use std::ops::Add;
use std::thread;
use std::sync::{Mutex, Arc};
use crate::key_manager::KeyManager;

const RESET_HOUR: u32 = 9;

pub fn start_reset_timer(key_manager: Arc<Mutex<KeyManager>>) {
    let _handler = thread::spawn(move || {
        let mut key_manager = key_manager.lock().unwrap();
        loop {
            thread::sleep(std::time::Duration::from_millis(calc_wait_time(Utc::now())));
            trigger(&mut key_manager);
        }
    });
}

fn calc_wait_time(now: DateTime<Utc>) -> u64 {
    let reset_time: DateTime<Utc> = if now.hour() < RESET_HOUR {
        Utc::now().with_hour(RESET_HOUR).unwrap()
            .with_minute(1).unwrap()
            .with_second(0).unwrap()
    } else {
        Utc::now().with_hour(RESET_HOUR).unwrap()
            .with_minute(1).unwrap()
            .with_second(0).unwrap()
            .add(Duration::days(1))
    };
    0_i64.max(reset_time.timestamp_millis() - now.timestamp_millis()) as u64
}

fn trigger(key_manager: &mut KeyManager) {
    key_manager.reset_keys()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_wait_time_before_9() {
        let now = Utc::now().with_hour(8).unwrap()
            .with_minute(0).unwrap()
            .with_second(0).unwrap();

        let ms = calc_wait_time(now);

        assert_eq!(ms, 3660000)
    }

    #[test]
    fn check_wait_time_after_9() {
        let now = Utc::now().with_hour(10).unwrap()
            .with_minute(0).unwrap()
            .with_second(0).unwrap();

        let ms = calc_wait_time(now);

        assert_eq!(ms, 82860000)
    }
}