use std::collections::HashMap;
use rand::Rng;

/// Generic weighted random selection utility.
///
/// Given a HashMap of weights, randomly selects a key based on the weights.
/// Returns None if the total weight is 0 or the map is empty.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// let mut weights = HashMap::new();
/// weights.insert("common", 50);
/// weights.insert("rare", 10);
/// let result = weighted_select(&weights);
/// ```
pub fn weighted_select<K: Copy>(weights: &HashMap<K, i32>) -> Option<K> {
    let total: i32 = weights.values().sum();
    if total == 0 {
        return None;
    }
    let mut roll = rand::thread_rng().gen_range(0..total);
    for (key, weight) in weights {
        roll -= weight;
        if roll < 0 {
            return Some(*key);
        }
    }
    None
}
