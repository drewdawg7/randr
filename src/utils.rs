use std::collections::HashMap;
use rand::Rng;

/// Create a text-based progress bar.
///
/// # Arguments
/// * `value` - Current value
/// * `max_value` - Maximum value
/// * `increments` - Number of segments in the bar
pub fn text_bar(value: i32, max_value: i32, increments: i32) -> String {
    let value = value.min(max_value);

    let filled = (value * increments) / max_value;

    let mut bar = String::from("[");
    for i in 0..increments {
        if i < filled {
            bar.push('■');
        } else {
            bar.push('□');
        }
    }
    bar.push(']');

    bar
}

/// Create a text-based progress bar with a label.
///
/// # Arguments
/// * `label` - Label to display before the bar
/// * `value` - Current value
/// * `max_value` - Maximum value
/// * `increments` - Number of segments in the bar
pub fn text_bar_with_label(label: &str, value: i32, max_value: i32, increments: i32) -> String {
    format!(
        "{:<12} {}",
        format!("{}: ({}/{})", label.to_string(), value, max_value),
        text_bar(value, max_value, increments)
    )
}

/// Generic weighted random selection utility.
///
/// Given a HashMap of weights, randomly selects a key based on the weights.
/// Returns None if the total weight is 0 or the map is empty.
///
/// # Example
/// ```
/// use game::utils::weighted_select;
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

