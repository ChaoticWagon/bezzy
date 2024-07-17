use crate::curve::CubicBezier;
use crate::point::Point;

pub struct BezierSpline {
    pub points: Vec<Point>,
    pub curves: Vec<CubicBezier>,
}

impl BezierSpline {
    pub fn new() -> BezierSpline {
        BezierSpline {
            points: Vec::new(),
            curves: Vec::new(),
        }
    }

    pub fn add_knot(&mut self, point: Point) {
        match self.points.last() {
            Some(last) => {
                // add 2 points in between the previous point in the spline and the new one
                let dx =  point.x - last.x;
                let step_x = dx / 3.0;

                let dy = point.y - last.y;
                let step_y = dy / 3.0;

                self.points.push(Point::new(&point.x - (step_x * 2.0), &point.y - (step_y * 2.0)));
                self.points.push(Point::new(&point.x - step_x, &point.y - step_y));

                // add the new knot
                self.points.push(point);

                // create a new Bézier curve
                let thing: [Point; 4] = [
                    self.points[self.points.len() - 4],
                    self.points[self.points.len() - 3],
                    self.points[self.points.len() - 2],
                    self.points[self.points.len() - 1],
                ];
                self.curves.push(CubicBezier::new(thing));
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

        if t == 1.0 {
            return self.curves.last().unwrap().get_point(t);
        }

        let mut t = t * self.curves.len() as f64;
        let curve_index = t as usize;
        t = t - curve_index as f64;

        self.curves[curve_index].get_point(t)
    }
}
// now I have to assert that the first curve's last point is the same as the second curve's first point
// and I also need a way to create a new curve from one curve and adding 3 points to it