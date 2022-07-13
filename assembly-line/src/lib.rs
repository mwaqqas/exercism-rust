// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PROD_AT_LOWEST_SPEED: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let cars_prod_wo_errors: f64 = CARS_PROD_AT_LOWEST_SPEED * speed as f64;

    match speed {
        1..=4 => cars_prod_wo_errors,
        5..=8 => cars_prod_wo_errors * 0.9,
        9..=10 => cars_prod_wo_errors * 0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
