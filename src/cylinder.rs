use std::collections::HashMap;

const SYSTEM_PI:f64 = std::f64::consts::PI;

fn main() {
    let mut current_iteration = 0;
    let radius_iteration_amount = 4;
    let height_iteration_amount = 4;
    let radius_increment = 1.;
    let height_increment = 1.;

    let mut current_radius:f64 = 1.;
    let mut current_height:f64 = 1.;

    let baseline_height:f64 = current_height;

    let mut hash = HashMap::new();

    for _x in 1..radius_iteration_amount {
        for _y in 1..height_iteration_amount {
            hash.insert(current_iteration, calculate(current_radius, current_height));

            current_height += height_increment;

            // update iteration
            current_iteration += 1;
        }

        // reset the height so we can iterate again
        current_height = baseline_height;

        // update radius to account for all combinations
        current_radius += radius_increment;
    }

    // makes a mutable copy of the hashmap for memory safety
    print_all_values(&mut hash);
}

fn calculate(radius:f64, height:f64) -> (f64, f64) {
    let surface_area = (2 as f64 * SYSTEM_PI * radius * height) + 2 as f64 * SYSTEM_PI * radius.powi(2);
    let volume = SYSTEM_PI * radius.powi(2) * height;

    let result_tuple = (surface_area, volume);

    let converted = convert_to_cm(result_tuple);

    return converted;
}

fn convert_to_cm(tuple:(f64, f64)) -> (f64, f64) {
    let root:f64 = 10.;

    let cm_surface_area = tuple.0 / root.powi(2);
    let cm_volume = tuple.1 / root.powi(3);

    return (cm_surface_area, cm_volume);
}

fn print_all_values(map: &mut HashMap<i32, (f64, f64)>) {
    for (key, value) in &*map {
        //println!("Radius: {:} || Height: {:} || Final Surface Area: {:}", radius, height, result);
        println!("Key: {:} || Final Surface Area: {:} || Final Volume: {:}", key, value.0, value.1);
    }

    map.clear();
}