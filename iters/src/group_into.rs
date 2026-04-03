use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub trait GroupInto: Iterator + Sized {
    fn group_into_hash_map<K, V>(self, mut key_fn: impl FnMut(&Self::Item) -> K) -> HashMap<K, V>
    where
        K: Eq + Hash,
        V: Default + Extend<Self::Item>,
    {
        let mut map: HashMap<K, V> = HashMap::new();
        for item in self {
            let key = key_fn(&item);
            map.entry(key).or_default().extend(Some(item));
        }
        map
    }

    fn group_into_btree_map<K, V>(self, mut key_fn: impl FnMut(&Self::Item) -> K) -> BTreeMap<K, V>
    where
        K: Ord,
        V: Default + Extend<Self::Item>,
    {
        let mut map: BTreeMap<K, V> = BTreeMap::new();
        for item in self {
            let key = key_fn(&item);
            map.entry(key).or_default().extend(Some(item));
        }
        map
    }
}

impl<I> GroupInto for I where I: Iterator {}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashSet};

    use super::*;

    const INPUT: &str = "the quick brown fox jumps over the lazy dog";

    const EXPECTED_VOWELS: &str = "euioouoeeao";
    const EXPECTED_CONSONANTS: &str = "thqckbrwnfxjmpsvrthlzydg";
    const EXPECTED_WHITESPACE: &str = "        ";

    fn key_fn(c: char) -> &'static str {
        if c == ' ' {
            "whitespace"
        } else if "aeiou".contains(c) {
            "vowels"
        } else {
            "consonants"
        }
    }

    #[test]
    fn group_into_hash_map() {
        let grouped: HashMap<_, String> = INPUT.chars().group_into_hash_map(|c| key_fn(*c));

        assert_eq!(
            grouped.keys().collect::<HashSet<_>>(),
            HashSet::from([&"vowels", &"consonants", &"whitespace"])
        );
        assert_eq!(grouped["vowels"], EXPECTED_VOWELS);
        assert_eq!(grouped["consonants"], EXPECTED_CONSONANTS);
        assert_eq!(grouped["whitespace"], EXPECTED_WHITESPACE);
    }

    #[test]
    fn group_into_btree_map() {
        let grouped: BTreeMap<_, String> = INPUT.chars().group_into_btree_map(|c| key_fn(*c));

        assert_eq!(
            grouped.keys().collect::<BTreeSet<_>>(),
            BTreeSet::from([&"vowels", &"consonants", &"whitespace"])
        );
        assert_eq!(grouped["vowels"], EXPECTED_VOWELS);
        assert_eq!(grouped["consonants"], EXPECTED_CONSONANTS);
        assert_eq!(grouped["whitespace"], EXPECTED_WHITESPACE);
    }
}
