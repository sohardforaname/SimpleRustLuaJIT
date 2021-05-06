use std::collections::{HashMap, HashSet};

pub fn calc_set_map_len<K, V>(set_map:&HashMap<K, HashSet<V>>) -> usize {
    let mut size = 0;
    for set in set_map.iter() {
        size += set.1.len();
    }
    size
}