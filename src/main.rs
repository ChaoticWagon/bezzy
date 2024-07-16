use crate::spline::CubicBezier;

mod spline;
mod point;
mod curve;

fn main() {
    let s1 = CubicBezier {
        points: [
            spline::Point::new(0.0, 0.0),
            spline::Point::new(1.0, 1.0),
            spline::Point::new(2.0, 1.0),
            spline::Point::new(3.0, 0.0),
        ],
    };

    // println!("{}, {}", &s1.get_point(0.5).unwrap().x, &s1.get_point(0.5).unwrap().y);

    for i in 0..=100 {
        let t: f64 = i as f64 / 100.0;
        println!("{}, {}", &s1.get_point(t).unwrap().x, &s1.get_point(t).unwrap().y);
    }

}
