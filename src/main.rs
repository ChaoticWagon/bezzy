use crate::point::Point;
use crate::spline::BezierSpline;

mod spline;
mod point;
mod curve;

fn main() {
    let mut spline: BezierSpline = BezierSpline::new();
    spline.add_knot(Point::new(0.0, 0.0));
    spline.add_knot(Point::new(1.0, 0.0));
    spline.add_knot(Point::new(2.0, 0.0));
    spline.add_knot(Point::new(3.0, 0.0));

    println!("{:?}", spline.points);

    // println!("{}, {}", &s1.get_point(0.5).unwrap().x, &s1.get_point(0.5).unwrap().y);

    for i in 0..=100 {
        let t: f64 = i as f64 / 100.0;
        println!("{}, {}", &spline.get_point(t).unwrap().x, &spline.get_point(t).unwrap().y);
    }
}
