use std::io::stdin;
use std::{time,fmt::Write};
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle};
use indicatif::{ProgressBar,ProgressStyle,ProgressState};
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

    let mut data1 = Vec::new();

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

    let iterations:u128 = loop {

        println!("Input number of iterations:");

        stdin().read_line(&mut input_iterations)
        .ok()
        .expect("Failed to read line");

        let trimmed = input_iterations.trim();

        match trimmed.parse::<u128>() {
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
    }

    if !(debugging_on == "yes" || debugging_on == "y" || debugging_on == "Yes" || debugging_on == "YES" || debugging_on == "n" || debugging_on == "no" || debugging_on == "No" || debugging_on == "NO") {
        println!("No such option. Please try again.");
        println!("");
    }

    // define progressbar, has to use 'as' keyword since it expects u64
    let progressbar = ProgressBar::new(iterations as u64);

    //progressbar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap().progress_chars("##>"));

    progressbar.set_style(ProgressStyle::with_template("{spinner:.green} [Time Elapsed - {elapsed_precise}] [{wide_bar:.cyan/blue}] ({pos:>7}/{len:7}) (ETA - {eta})").unwrap().with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()).progress_chars("#>-"));

    //progressbar.set_style(ProgressStyle::with_template("{spisnner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap().with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()).progress_chars("#>-"));

    //let mut edge_length = starting_volume.powf(1.0/3.0);
    let mut height = 0.;

    // best values
    let mut best_height = 0.;
    let mut best_sides = 0.;
    let mut best_sa = 0.;
    let mut best_precentage_diff = 0.;

    let mut low_x:i64 = 0;
    let mut high_x:i64 = 0;
    let mut low_y:i64 = 0;
    let mut high_y:i64 = 0;

    // mode 1 is a box-based rectangular prism
    if mode == 1. {
        for _x in 0..iterations {
            let result = calc_cubic_rectangular_prism(height, starting_volume, should_round);
            let percentage_diff = calc_percentage_diff(start_sa, result.1, should_round);
    
            // updates best values
            if percentage_diff >= 0 as f64 && percentage_diff > best_precentage_diff {
                best_height = height;
                best_sides = result.0;
                best_sa = result.1;
                best_precentage_diff = percentage_diff;
            }

            // updating graph x bounds
            if height > 0 as f64 && height > high_x as f64 {
                high_x = height as i64;
            }

            // update graphing values
            let mut tmp_vec = vec![(height, result.1)];
            data1.append(&mut tmp_vec);

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

        for _y in 0..iterations {
                let result = calc_cubic_rectangular_prism(height, starting_volume, should_round);

                height = 0.;

                if height <= high_x as f64 {
                    low_x = height as i64;

                    height += height_increment;
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

            // update graphing values
            let mut tmp_vec = vec![(height, result.1)];
            data1.append(&mut tmp_vec);

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
    }

    let time_elapsed:time::Duration = progressbar.elapsed();
    let seconds_time_elapsed = time_elapsed.as_secs_f64().round();

    progressbar.finish_and_clear();

    println!("Low-X: {}, High-X: {}", low_x, high_x);

    println!("");
    println!("Calculation complete!");
    println!("Time elapsed: {} seconds.", seconds_time_elapsed);
    println!("");

    println!("Best percentage reduction: ~{}%", best_precentage_diff);
    println!("{}{} squared --> {}{} squared", start_sa, unit, best_sa, unit);
    println!("Stats - Height: ~{}{}, Edge Length/Radius: ~{}{}", best_height, unit, best_sides, unit);
    println!("");

    // draws graph on complete
    draw_graph(&data1, low_x, high_x, 750, 10000);

    //draw_graph(&data1, 0, 150, 750, 2000);
    
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

fn draw_graph(vector:&Vec<(f64,f64)>, low_x_range:i64, high_x_range:i64, low_y_range:i64, high_y_range:i64) {
    let cloned_vec = vector.clone();
    //println!("{:?}", cloned_vec);
    // We create our scatter plot from the data
    let s1: Plot = Plot::new(cloned_vec).point_style(
        PointStyle::new()
            .marker(PointMarker::Circle) // setting the marker to be a circle
            .size(0.8)
            .colour("#07fc03"),
    ); // and a custom colour

    /*// We can plot multiple data sets in the same view
    let data2 = vec![(-1.4, 2.5), (7.2, -0.3)];
    let s2: Plot = Plot::new(data2).point_style(
        PointStyle::new() // uses the default marker
            .colour("#35C788"),
    ); // and a different colour*/

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        //.add(s2)
        //.x_range(-5., 10.)
        //.y_range(-2., 6.)
        .x_range(low_x_range as f64, high_x_range as f64)
        .y_range(low_y_range as f64, high_y_range as f64)
        .x_label("Height")
        .y_label("Surface Area");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("output//output-graph.svg").unwrap();
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