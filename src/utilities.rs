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


pub fn text_bar_with_label(label: &str, value: i32, max_value: i32, increments: i32) -> String {
    format!(
        "{:<12} {}",
        format!("{}: ({}/{})", label.to_string(), value, max_value),
        text_bar(value, max_value, increments)
    )
}
