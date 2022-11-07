use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle};
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

    //let mut rng = rand::thread_rng();
    let mut data1 = Vec::new();
    
    let mut height:f64 = 1.;
    let height_increment = 1.;

    let mut low_x:u64 = 0;
    let mut high_x:u64 = 0;
    let mut low_y:u64 = 0;
    let mut high_y:u64 = 0;

    for _i in 0..200 {
        let result = calc_cubic_rectangular_prism(height, 1532 as f64, "no");

        let mut tmp_vec = vec![(height, result.1)];
        data1.append(&mut tmp_vec);

        // updating graph x bounds
        if height > 0 as f64 && height > high_x as f64 {
            high_x = height as u64;
        }

        height += height_increment;
    }

    //draw_graph(data1, 0, 300, 700, 3000);
    draw_graph(data1, 0, high_x, 700, 3000);
}

fn draw_graph(vector:Vec<(f64,f64)>, low_x:u64, high_x:u64, low_y:u64, high_y:u64) {
    let cloned_vec = vector.clone();
    println!("{:?}", cloned_vec);

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
        .x_range(low_x as f64, high_x as f64)
        .y_range(low_y as f64, high_y as f64)
        .x_label("Height")
        .y_label("Surface Area");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("output//scatter.svg").unwrap();
}

fn calc_cubic_rectangular_prism(height:f64, volume:f64, round:&str) -> (f64, f64) {
    let base_area:f64 = volume / height;
    let base_edge_length = base_area.sqrt();

    let surface_area = 2 as f64 * (base_edge_length * base_edge_length + height * base_edge_length + height * base_edge_length);

    if round == "yes" || round == "y" || round == "Yes" || round == "YES" {
        return (base_edge_length.round(), surface_area.round());
    }

    return (base_edge_length, surface_area);
}