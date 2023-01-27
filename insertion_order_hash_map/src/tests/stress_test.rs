use std::collections::HashMap;
use std::ops::Add;
use std::time::{Duration, Instant};

use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};

use crate::InsertionOrderHashMap;

#[test]
fn stress_test() {
    const HALF_DURATION: Duration = Duration::from_millis(1000);

    let begin = Instant::now();
    let end = begin.add(HALF_DURATION);

    let mut data = StressTestData {
        iohm: InsertionOrderHashMap::new(),
        map: Default::default(),
        order: Default::default(),
        rng: rand::thread_rng(),
    };

    while Instant::now().cmp(&end).is_le() {
        for _ in 0..4 {
            data.set_and_check();
        }

        data.unset_and_check();
    }

    println!(">>>> Len: {}", data.map.len());

    while !data.iohm.nodes.is_empty() {
        data.set_and_check();

        for _ in 0..4 {
            data.unset_and_check();
        }
    }
}

type Key = u32;
type Value = i32;

struct StressTestData {
    iohm: InsertionOrderHashMap<Key, Value>,
    map: HashMap<Key, Value>,
    order: Vec<Key>,
    rng: ThreadRng,
}
impl StressTestData {
    fn set_and_check(&mut self) {
        self.set();
        self.check();
    }

    fn set(&mut self) {
        let key: Key = self.rng.gen();
        let value: Value = self.rng.gen();

        let result = self.iohm.set(key, value);

        let previous_value = self.map.insert(key, value);
        assert_eq!(result, previous_value);

        if previous_value.is_none() {
            self.order.push(key);
        }
    }

    fn unset_and_check(&mut self) {
        self.unset();
        self.check();
    }

    fn unset(&mut self) {
        if let Some(index) = (0..self.order.len()).choose(&mut self.rng) {
            let key = self.order.splice(index..=index, []).next().unwrap();

            let result = self.iohm.unset(&key);

            let value = self.map.remove(&key);
            assert_eq!(result, value);
        }
    }

    fn check(&self) {
        let expected_keys: Vec<_> = self.order.iter().collect();
        let keys: Vec<_> = self.iohm.keys().collect();
        assert_eq!(keys, expected_keys);

        for key in &self.order {
            assert_eq!(self.iohm.get(key), self.map.get(key));
        }
    }
}
