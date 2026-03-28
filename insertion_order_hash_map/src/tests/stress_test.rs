use rand::{Rng, rngs::ThreadRng, seq::IteratorRandom};
use std::collections::HashMap;
use std::ops::Add;
use std::time::{Duration, Instant};

use crate::InsertionOrderHashMap;
use crate::tests::consistency;

#[test]
fn stress_test() {
    const HALF_DURATION: Duration = Duration::from_millis(1000);

    let begin = Instant::now();
    let end = begin.add(HALF_DURATION);

    let mut data = StressTestData {
        iohm: InsertionOrderHashMap::new(),
        map: HashMap::default(),
        order: Vec::default(),
        rng: rand::thread_rng(),
    };

    while Instant::now().cmp(&end).is_le() {
        for _ in 0..4 {
            data.insert_and_check();
        }

        data.remove_and_check();
    }

    println!(">>>> Len: {}", data.map.len());

    while !data.iohm.nodes.is_empty() {
        data.insert_and_check();

        for _ in 0..4 {
            data.remove_and_check();
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
    fn insert_and_check(&mut self) {
        self.insert();
        self.check();
    }

    fn insert(&mut self) {
        let key: Key = self.rng.r#gen();
        let value: Value = self.rng.r#gen();

        let result = self.iohm.insert(key, value);

        let previous_value = self.map.insert(key, value);
        assert_eq!(result, previous_value);

        if previous_value.is_none() {
            self.order.push(key);
        }
    }

    fn remove_and_check(&mut self) {
        self.remove();
        self.check();
    }

    fn remove(&mut self) {
        if let Some(index) = (0..self.order.len()).choose(&mut self.rng) {
            let key = self.order.splice(index..=index, []).next().unwrap();

            let result = self.iohm.remove(&key);

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

        consistency::assert(&self.iohm);
    }
}
