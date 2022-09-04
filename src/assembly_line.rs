// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
const TOTAL: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let discard_rate = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    };
    discard_rate * (speed as f64) * TOTAL
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let discard_rate = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    };

    (discard_rate * (speed as f64) * TOTAL / 60.0).floor() as u32
}