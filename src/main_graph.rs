use std::io::stdin;
use std::{time,fmt::Write};
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle};
use indicatif::{ProgressBar,ProgressStyle,ProgressState};
//use rand::Rng;

fn main() {
    // Scatter plots expect a list of pairs
    /*data1 = vec![
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];*/

    let mut input_volume = String::new();
    let mut input_sample_amount = String::new();
    let mut input_sample_distance = String::new();

    let starting_volume:f64 = loop {

        println!("/* CALC MODULE */ > Input baseline volume:");

        stdin().read_line(&mut input_volume)
        .ok()
        .expect("Failed to read line");

        let trimmed = input_volume.trim();

        match trimmed.parse::<f64>() {
            Ok(result) => {
                println!("");
                break result;
            }
            Err(..) => {
                println!("");
                println!("/* SYSTEM */ > Please input a valid integer or decimal.")
            },
        };

        input_volume.clear();
    };

    let sample_amount:u128 = loop {

        println!("/* CALC MODULE */ > Input desired amount of samples:");

        stdin().read_line(&mut input_sample_amount)
        .ok()
        .expect("Failed to read line");

        let trimmed = input_sample_amount.trim();

        match trimmed.parse::<u128>() {
            Ok(result) => {
                println!("");
                break result;
            }
            Err(..) => {
                println!("");
                println!("/* SYSTEM */ > Please input a valid integer.")
            },
        };

        input_sample_amount.clear();
    };

    let sample_distance:f64 = loop {

        println!("/* GRAPHING MODULE */ > Input sample distance (0.1 for default):");

        stdin().read_line(&mut input_sample_distance)
        .ok()
        .expect("Failed to read line");

        let trimmed = input_sample_distance.trim();

        match trimmed.parse::<f64>() {
            Ok(result) => {
                println!("");
                break result;
            }
            Err(..) => {
                println!("");
                println!("/* SYSTEM */ > Please input a valid integer or decimal.")
            },
        };

        input_sample_distance.clear();
    };

    println!("");

    //let mut rng = rand::thread_rng();
    let mut data1 = Vec::new();
    
    let mut height:f64 = 1.;
    //let sample_gap = 0.1;

    let mut _low_x:u64 = 0;
    let mut high_x:u64 = 0;
    let mut low_y:u64 = 0;
    let mut high_y:u64 = 0;

    // define progressbar, has to use 'as' keyword since it expects u64
    let progressbar = ProgressBar::new(sample_amount as u64);
    progressbar.set_style(ProgressStyle::with_template("{spinner:.green} [Time Elapsed - {elapsed_precise}] [{wide_bar:.cyan/blue}] ({pos:>7}/{len:7}) (ETA - {eta})").unwrap().with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()).progress_chars("#>-"));

    // start calculation process
    for _i in 0..sample_amount {
        let result = calc_cubic_rectangular_prism(height, starting_volume, "no");

        let mut tmp_vec = vec![(height, result.1)];
        data1.append(&mut tmp_vec);

        // updating graph x bounds
        if height > 0 as f64 && height >= high_x as f64 {
            high_x = height as u64 + 10;
        }

        if result.1 > 0 as f64 && result.1 >= high_y as f64 {
            high_y = result.1 as u64 + 10;
        }

        height += sample_distance;

        progressbar.inc(1);
    }

    let mut temp_lowest_x:u64 = 10000000000000000000;
    let mut temp_lowest_y:u64 = 10000000000000000000;

    // foreach cycles through the vector in order of addition apparently
    for sample in &data1 {
        //println!("{:?}", sample);
        
        if sample.0 > 0 as f64 && sample.0 <= temp_lowest_x as f64 {
            _low_x = sample.0 as u64;
            temp_lowest_x = sample.0 as u64;
        }

        if sample.1 > 0 as f64 && sample.1 <= temp_lowest_y as f64 {
            low_y = sample.1 as u64;
            temp_lowest_y = sample.1 as u64;
        }
    }

    progressbar.finish_and_clear();

    println!("");
    println!("/* SYSTEM */ > Generating graph... please be patient, as this may take a while!");
    println!("/* SYSTEM */ > It is recommended to keep graph sizes at around 1 million samples on Desktop computers.");
    println!("/* SYSTEM */ > Please note that for sample numbers from 100 million to beyond, the graph may take some time to load when opening on lower performance devices!");

    //draw_graph(data1, 0, 300, 700, 3000);
    //draw_graph(data1, low_x, high_x, 700, high_y);
    draw_graph(data1, 0, high_x + 10 as u64, low_y - 100 as u64, high_y + 100 as u64);
    //draw_graph(data1, low_x, high_x, low_y, high_y);

    //let iteration = rng.gen_range(20..30);

    /*for _x in 0..iteration {
        let random_x:f64 = rng.gen_range(0 as f64..100 as f64);
        let random_y:f64 = rng.gen_range(0 as f64..100 as f64);
        
        let mut tmp_vec = vec![(random_x, random_y)];
        data1.append(&mut tmp_vec);
    }*/

    let time_elapsed:time::Duration = progressbar.elapsed();
    let seconds_time_elapsed = time_elapsed.as_secs_f64().round();

    println!("");
    println!("/* CALC MODULE */ > Calculation complete!");
    println!("/* CALC MODULE */ > Time elapsed: {} seconds.", seconds_time_elapsed);
    println!("");
    println!("/* SYSTEM */ > To view the graph, enter the [output] folder and open [output-graph.svg] with a browser of your choice.");
    println!("");
}

fn draw_graph(vector:Vec<(f64,f64)>, low_x:u64, high_x:u64, low_y:u64, high_y:u64) {
    let cloned_vec = vector.clone();
    //println!("{:?}", cloned_vec);

    // We create our scatter plot from the data
    let s1: Plot = Plot::new(cloned_vec).point_style(
        PointStyle::new()
            .marker(PointMarker::Circle)
            .size(2.) // setting the marker to be a circle
            .colour("#DD3355"),
    ); // and a custom colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        //.add(s2)
        //.x_range(-5., 10.)
        //.y_range(-2., 6.)
        .x_range(low_x as f64, high_x as f64)
        .y_range(low_y as f64, high_y as f64)
        .x_label("Height")
        .y_label("Surface Area");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("output//output-graph.svg").unwrap();

    println!("");
    println!("/* GRAPHING MODULE */ > Graph creation successful!");
    println!("");
    println!("/* GRAPHING MODULE */ > Graph Statistics:");
    println!("/* GRAPHING MODULE */ > Lower Y Bounds is {:?} / Higher Y Bounds is {:?}.", low_y, high_y);
    println!("/* GRAPHING MODULE */ > Lower X Bounds is {:?} / Higher X Bounds is {:?}.", low_x, high_x);
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