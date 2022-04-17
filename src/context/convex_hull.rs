use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

fn ccw(a: Point, b: Point, c: Point) -> i8 {
    let signed_area = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if signed_area < 0.0 {
        // Clockwise
        -1
    } else if signed_area > 0.0 {
        // Counter Clockwise
        1
    } else {
        // Co-Linear
        0
    }
}

fn sort_by_y(points: &mut Vec<Point>) -> Vec<Point> {
    let mut points_copy = points.clone();
    // Sorting the points array
    points_copy.sort_by(|p1, p2| {
        let result = p1.y.partial_cmp(&p2.y).unwrap();
        if result == Ordering::Equal {
            return p1.x.partial_cmp(&p2.x).unwrap();
        }
        result
    });
    points_copy
}

fn sort_by_polar_order(points: &mut Vec<Point>, po: Point) -> Vec<Point> {
    let mut points_copy = points.clone();
    points_copy.sort_by(|q1, q2| {
        let dx1 = q1.x - po.x;
        let dy1 = q1.y - po.y;
        let dx2 = q2.x - po.x;
        let dy2 = q2.y - po.y;

        if dy1 >= 0.0 && dy2 < 0.0 {
            return Ordering::Less;
        }
        // q1 above; q2 below
        else if dy2 >= 0.0 && dy1 < 0.0 {
            return Ordering::Greater;
        }
        // q1 below; q2 above
        else if dy1 == 0.0 && dy2 == 0.0 {
            // 3-collinear and horizontal
            return if dx1 >= 0.0 && dx2 < 0.0 {
                Ordering::Less
            } else if dx2 >= 0.0 && dx1 < 0.0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            };
        } else {
            let res = -ccw(po.clone(), q1.clone(), q2.clone());
            return if res == -1 {
                Ordering::Less
            } else if res == 0 {
                Ordering::Equal
            } else {
                Ordering::Greater
            };
        }
    });
    points_copy
}

pub fn graham_scan(points: &mut Vec<Point>) -> Vec<Point> {
    let mut hull: Vec<Point> = vec![];
    // Points Sorted by Y-Coordinate
    let mut y_sorted = sort_by_y(points);
    // Lowest Point with respect to the Y-Coordinate
    let po = y_sorted[0].clone();
    // Points Sorted by the Polar Angle with respect to the po
    let sorted_by_polar = sort_by_polar_order(&mut y_sorted, po.clone());

    hull.push(po.clone());
    hull.push(sorted_by_polar[1].clone());

    for i in 2..points.len() {
        let mut top = hull.pop().unwrap();

        while hull.len() > 1
            && ccw(
                hull.last().unwrap().clone(),
                top.clone(),
                sorted_by_polar[i].clone(),
            ) <= 0
        {
            top = hull.pop().unwrap();
        }
        hull.push(top);
        hull.push(sorted_by_polar[i].clone());
    }
    hull
}

#[cfg(test)]
mod tests {
    use crate::context::convex_hull::{graham_scan, sort_by_polar_order, sort_by_y, Point};

    #[test]
    fn check_sort_by_coord() {
        let mut points: Vec<Point> = vec![
            Point { x: 1.0, y: 2.0 },
            Point { x: -90.0, y: 24.0 },
            Point { x: 22.0, y: 3.0 },
            Point { x: 0.0, y: 2.0 },
            Point { x: 1.0, y: 2.0 },
        ];
        let sorted = sort_by_y(&mut points);
        assert_eq!(
            sorted,
            vec![
                Point { x: 0.0, y: 2.0 },
                Point { x: 1.0, y: 2.0 },
                Point { x: 1.0, y: 2.0 },
                Point { x: 22.0, y: 3.0 },
                Point { x: -90.0, y: 24.0 }
            ]
        );
    }

    #[test]
    fn check_sort_by_polar_angle() {
        let mut points: Vec<Point> = vec![
            Point { x: 1.0, y: 2.0 },
            Point { x: -90.0, y: 24.0 },
            Point { x: 22.0, y: 3.0 },
            Point { x: 0.0, y: 2.0 },
            Point { x: 1.0, y: 2.0 },
        ];
        let mut by_y = sort_by_y(&mut points);
        let po = by_y[0].clone();
        let sorted = sort_by_polar_order(&mut by_y, po);
        assert_eq!(
            sorted,
            vec![
                Point { x: 0.0, y: 2.0 },
                Point { x: 1.0, y: 2.0 },
                Point { x: 1.0, y: 2.0 },
                Point { x: 22.0, y: 3.0 },
                Point { x: -90.0, y: 24.0 }
            ]
        );
    }

    #[test]
    fn graham_scan_test() {
        let mut points: Vec<Point> = vec![
            Point { x: 1.0, y: 2.0 },
            Point { x: -90.0, y: 24.0 },
            Point { x: 22.0, y: 3.0 },
            Point { x: 0.0, y: 2.0 },
            Point { x: 1.0, y: 2.0 },
        ];
        let hull = graham_scan(&mut points);
        assert_eq!(
            hull,
            vec![
                Point { x: 0.0, y: 2.0 },
                Point { x: 1.0, y: 2.0 },
                Point { x: 22.0, y: 3.0 },
                Point { x: -90.0, y: 24.0 }
            ]
        );
    }
}
