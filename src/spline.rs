use crate::curve::CubicBezier;
use crate::point::Point;

pub struct BezierSpline<> {
    pub points: Vec<Point>,
    pub curves: Vec<CubicBezier>,
}

impl<> BezierSpline<> {
    pub fn new() -> BezierSpline {
        BezierSpline {
            points: Vec::new(),
            curves: Vec::new(),
        }
    }

    pub fn add_knot(&'static mut self, point: Point) {
        match self.points.last() {
            Some(..) => {
                // add 2 points in between the previous point in the spline and the new one
                let distance = self.points.last().unwrap().distance_to(&point);
                let step = distance / 3.0;

                self.points.push(Point::new(&point.x - (step * 2.0), &point.y - (step * 2.0)));
                self.points.push(Point::new(&point.x - step, &point.y - step));

                // add the new knot
                self.points.push(point);

                // create a new Bézier curve
                self.curves.push(CubicBezier::new(self.points.get((self.points.len() - 4)..(self.points.len() - 1)).unwrap().try_into().unwrap()));
            }
            None => {
                // nothing in points so we add the first knot
                self.points.push(point)
            }
        }
    }


    pub fn get_point(&self, t: f64) -> Result<Point, &'static str> {
        if t < 0.0 || t > 1.0 {
            return Err("t must be between 0 and 1");
        }

        let mut t = t * self.curves.len() as f64;
        let curve_index = t as usize;
        t = t - curve_index as f64;

        self.curves[curve_index].get_point(t)
    }
}
// now I have to assert that the first curve's last point is the same as the second curve's first point
// and I also need a way to create a new curve from one curve and adding 3 points to it