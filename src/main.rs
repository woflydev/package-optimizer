use std::io::stdin;
use std::{time};
use indicatif::{ProgressBar,ProgressStyle};
//use duration_string::DurationString;

const SYSTEM_PI:f64 = std::f64::consts::PI;

fn main() {
    //Declare a mutable input string
    let mut input_unit = String::new();
    let mut input_volume = String::new();
    let mut input_iterations = String::new();
    let mut input_height_increment = String::new();
    let mut input_start_sa = String::new();
    let mut input_mode = String::new();
    let mut input_round = String::new();
    let mut input_debugging = String::new();

    // main infinite loop of the program
    loop {
        let unit:&str = loop {

            println!("What unit will input values be in?");
    
            stdin().read_line(&mut input_unit)
            .ok()
            .expect("Failed to read line");

            let trimmed = input_unit.trim();

            /*match trimmed.parsex::<String>() {
                Ok(result) => break result,
                Err(..) => {
                    println!("");
                    println!("")
                },
            };*/
            
            //input_unit.clear();

            break trimmed;
        };

        println!("");

        let starting_volume:f64 = loop {

            println!("Input baseline volume:");
    
            stdin().read_line(&mut input_volume)
            .ok()
            .expect("Failed to read line");
    
            let trimmed = input_volume.trim();
    
            match trimmed.parse::<f64>() {
                Ok(result) => break result,
                Err(..) => {
                    println!("");
                    println!("Please input a valid integer.")
                },
            };
    
            input_volume.clear();
        };

        println!("");

        let start_sa:f64 = loop {
    
            println!("Input the surface area of the original package:");
    
            stdin().read_line(&mut input_start_sa)
            .ok()
            .expect("Failed to read line");
    
            let trimmed = input_start_sa.trim();
    
            match trimmed.parse::<f64>() {
                Ok(result) => break result,
                Err(..) => {
                    println!("");
                    println!("Please input a valid integer.");
                },
            };
    
            input_start_sa.clear();
        };
    
        println!("");
    
        let iterations:u32 = loop {
    
            println!("Input number of iterations:");
    
            stdin().read_line(&mut input_iterations)
            .ok()
            .expect("Failed to read line");
    
            let trimmed = input_iterations.trim();
    
            match trimmed.parse::<u32>() {
                Ok(result) => break result,
                Err(..) => {
                    println!("");
                    println!("Please input a valid integer.");
                },
            };
    
            input_iterations.clear();
        };
    
        println!("");
    
        let height_increment:f64 = loop {
    
            println!("Input the height increment:");
    
            stdin().read_line(&mut input_height_increment)
            .ok()
            .expect("Failed to read line");
    
            let trimmed = input_height_increment.trim();
    
            match trimmed.parse::<f64>() {
                Ok(result) => break result,
                Err(..) => {
                    println!("");
                    println!("Please input a valid integer.");
                },
            };
    
            input_height_increment.clear();
        };

        println!("");

        let mode:f64 = loop {
    
            println!("Input mode (1 - Cube based rectangular prism / 2 - Cylinder):");
    
            stdin().read_line(&mut input_mode)
            .ok()
            .expect("Failed to read line");
    
            let trimmed = input_mode.trim();
    
            match trimmed.parse::<f64>() {
                Ok(result) => break result,
                Err(..) => {
                    println!("");
                    println!("Please input a valid integer.");
                },
            };
    
            input_mode.clear();
        };

        println!("");

        let should_round:&str = loop {
    
            println!("Should the program round values to the nearest integer? (y/n):");
    
            stdin().read_line(&mut input_round)
            .ok()
            .expect("Failed to read line");
    
            let trimmed = input_round.trim();
    
            /*match trimmed.parse::<&str>() {
                Ok(result) => break result,
                Err(..) => {
                    println!("");
                    println!("Please input a valid integer.");
                },
            };*/
    
            break trimmed;
        };

        println!("");

        let debugging_on:&str = loop {
    
            println!("Debugging on? Note that this will render the program slower by 80 to 120 times. (y/n):");
    
            stdin().read_line(&mut input_debugging)
            .ok()
            .expect("Failed to read line");
    
            let trimmed = input_debugging.trim();
    
            /*match trimmed.parse::<&str>() {
                Ok(result) => break result,
                Err(..) => {
                    println!("");
                    println!("Please input a valid integer.");
                },
            };*/
    
            break trimmed;
        };

        println!("");

        if !(should_round == "yes" || should_round == "y" || should_round == "Yes" || should_round == "YES" || should_round == "n" || should_round == "no" || should_round == "No" || should_round == "NO") {
            println!("No such option. Please try again.");
            println!("");

            input_round.clear();
            continue;
        }

        if !(debugging_on == "yes" || debugging_on == "y" || debugging_on == "Yes" || debugging_on == "YES" || debugging_on == "n" || debugging_on == "no" || debugging_on == "No" || debugging_on == "NO") {
            println!("No such option. Please try again.");
            println!("");

            input_debugging.clear();
            continue;
        }

        // define progressbar, has to use 'as' keyword since it expects u64
        let progressbar = ProgressBar::new(iterations as u64);
    
        progressbar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap().progress_chars("##>"));

        //let mut edge_length = starting_volume.powf(1.0/3.0);
        let mut height = 1.;

        // best values
        let mut best_height = 1.;
        let mut best_sides = 1.;
        let mut best_sa = 1.;
        let mut best_precentage_diff = 1.;

        // mode 1 is a box-based rectangular prism
        if mode == 1. {
            for _x in 0..iterations {
                let result = calc_cubic_rectangular_prism(height, starting_volume, should_round);
                let percentage_diff = calc_percentage_diff(start_sa, result.1, should_round);
        
                // updates best values
                if percentage_diff >= 0 as f64 && percentage_diff >= best_precentage_diff {
                    best_height = height;
                    best_sides = result.0;
                    best_sa = result.1;
                    best_precentage_diff = percentage_diff;
                }

                /*
                println!("Height: ~{}{}", height, unit);
                println!("Length of base edges: ~{}{}", result.0, unit);
                println!("Surface Area: ~{}{}", result.1, unit);
                println!("Percentage reduction from original: ~{}%", percentage_diff);
                println!("");*/
        
                height += height_increment;

                if !(debugging_on == "yes" || debugging_on == "y" || debugging_on == "Yes" || debugging_on == "YES") {
                    progressbar.inc(1);
                }

                else {
                    println!("Height: ~{}{}", height, unit);
                    println!("Base Edge Length: ~{}{}", result.0, unit);
                    println!("Diameter: ~{}{}", result.0 * 2 as f64, unit);
                    println!("Surface Area: ~{}{}", result.1, unit);
                    println!("Percentage reduction from original: ~{}%", percentage_diff);
                    println!("");
                }

            }
        }

        // mode 2 is a cylinder
        else if mode == 2. {
            for _y in 0..iterations {
                let radius = calc_radius(height, starting_volume);
                let result = calc_cylinder(height, starting_volume, radius, should_round);
                let percentage_diff = calc_percentage_diff(start_sa, result.1, should_round);
        
                // updates best values
                if percentage_diff >= 0 as f64 && percentage_diff >= best_precentage_diff {
                    best_height = height;
                    best_sides = result.0;
                    best_precentage_diff = percentage_diff;
                }

                /*println!("Height: ~{}{}", height, unit);
                println!("Radius: ~{}{}", result.0, unit);
                println!("Diameter: ~{}{}", result.0 * 2 as f64, unit);
                println!("Surface Area: ~{}{}", result.1, unit);
                println!("Percentage reduction from original: ~{}%", percentage_diff);
                println!("");*/
        
                height += height_increment;

                if !(debugging_on == "yes" || debugging_on == "y" || debugging_on == "Yes" || debugging_on == "YES") {
                    progressbar.inc(1);
                }

                else {
                    println!("Height: ~{}{}", height, unit);
                    println!("Radius: ~{}{}", result.0, unit);
                    println!("Diameter: ~{}{}", result.0 * 2 as f64, unit);
                    println!("Surface Area: ~{}{}", result.1, unit);
                    println!("Percentage reduction from original: ~{}%", percentage_diff);
                    println!("");
                }
            }
        }

        else {
            println!("No such mode. Please try again.");
            println!("");

            input_mode.clear();
            continue;
        }

        let time_elapsed:time::Duration = progressbar.elapsed();
        let seconds_time_elapsed = time_elapsed.as_secs_f64().round();

        progressbar.finish_and_clear();

        println!("");
        println!("Calculation complete!");
        println!("Time elapsed: {} seconds.", seconds_time_elapsed);
        println!("");

        println!("Best percentage reduction: ~{}%", best_precentage_diff);
        println!("{}{} squared --> {}{} squared", start_sa, unit, best_sa, unit);
        println!("Stats - Height: ~{}{}, Edge Length/Radius: ~{}{}", best_height, unit, best_sides, unit);
        println!("");
    
        // flush input strings
        input_volume.clear();
        input_iterations.clear();
        input_height_increment.clear();
        input_start_sa.clear();
        input_mode.clear();
        input_unit.clear();
        input_round.clear();
        input_debugging.clear();

        // old thread sleep code
        //thread::sleep(time::Duration::from_secs(10000));
    }
}

fn calc_cubic_rectangular_prism(height:f64, volume:f64, round:&str) -> (f64, f64) {
    //return 6 as f64 * edge.powi(2);
    
    let base_area:f64 = volume / height;
    let base_edge_length = base_area.sqrt();

    let surface_area = 2 as f64 * (base_edge_length * base_edge_length + height * base_edge_length + height * base_edge_length);

    if round == "yes" || round == "y" || round == "Yes" || round == "YES" {
        return (base_edge_length.round(), surface_area.round());
    }

    return (base_edge_length, surface_area);
}

fn calc_cylinder(height:f64, _volume:f64, radius:f64, round:&str) -> (f64, f64) {
    let surface_area = 2 as f64 * SYSTEM_PI * radius * height + 2 as f64 * SYSTEM_PI * radius.powi(2);
    
    if round == "yes" || round == "y" || round == "Yes" || round == "YES" {
        return (radius.round(), surface_area.round());
    }
    
    return (radius, surface_area);
}

// radius formula from the volume and height of cylinder
fn calc_radius(height:f64, volume:f64) -> f64 {
    let expression = volume / SYSTEM_PI * height;
    return expression.sqrt();
}

fn calc_percentage_diff(start_sa:f64, calced_sa:f64, round:&str) -> f64 {
    let difference = start_sa - calced_sa;

    if round == "yes" || round == "y" || round == "Yes" || round == "YES" {
        let percentage = difference / start_sa * 100.;
        return percentage.round();
    }

    return difference / start_sa * 100.;
}