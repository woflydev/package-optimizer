use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle};
use rand::Rng;

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

    let mut rng = rand::thread_rng();
    let mut data1 = vec![(0.,0.)];

    for _x in 0..5 {
        let random_x:f64 = rng.gen_range(0 as f64..10 as f64);
        let random_y:f64 = rng.gen_range(0 as f64..6 as f64);
        
        let mut tmp_vec = vec![(random_x, random_y)];

        data1.append(&mut tmp_vec);
    }

    // We create our scatter plot from the data
    let s1: Plot = Plot::new(data1).point_style(
        PointStyle::new()
            .marker(PointMarker::Circle) // setting the marker to be a circle
            .colour("#DD3355"),
    ); // and a custom colour

    // We can plot multiple data sets in the same view
    let data2 = vec![(-1.4, 2.5), (7.2, -0.3)];
    let s2: Plot = Plot::new(data2).point_style(
        PointStyle::new() // uses the default marker
            .colour("#35C788"),
    ); // and a different colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .add(s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("output//scatter.svg").unwrap();
}
