#![allow(dead_code, unused_imports)]

// use https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/


// see https://media.geeksforgeeks.org/wp-content/uploads/linesegments.png

#[derive(PartialEq)]
enum Orientation {
    CLOCKWISE,
    COUNTERCLOCKWISE,
    COLLINEAR
}


pub struct Point {
    pub x: f64,
    pub y: f64
}


impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        return Point { x: x, y: y }
    }
}


fn get_orientation(p: &Point, q: &Point, r: &Point) -> Orientation {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);

    if val > 0.0 {
        return Orientation::CLOCKWISE;
    } 
    else if val < 0.0 {
        return Orientation::COUNTERCLOCKWISE;
    } else {
        return Orientation::COLLINEAR;
    }

}

/// Given three collinear points p, q, r, the function checks if 
/// point q lies on line segment 'pr' 
fn on_segment(p: &Point, q: &Point, r: &Point) -> bool {

    if  (q.x <= max(p.x, r.x)) && 
        (q.x >= min(p.x, r.x)) && 
        (q.y <= max(p.y, r.y)) && 
        (q.y >= min(p.y, r.y)) 
    {
        return true;
    }

    return false;
}



/// function that returns true if 
/// the line segment 'p1q1' and 'p2q2' intersect.
pub fn determine_intersection_exists(p1:& Point, q1: &Point, p2: &Point, q2: &Point) -> bool {
    let o1 = get_orientation(&p1,& q1, &p2);
    let o2 = get_orientation(&p1,& q1, &q2);
    let o3 = get_orientation(&p2,& q2, &p1);
    let o4 = get_orientation(&p2,&q2, &q1);

    // general case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // p1 , q1 and p2 are collinear and p2 lies on segment p1q1
    if (o1 == Orientation::COLLINEAR) && on_segment(&p1,& p2, &q1) {
        return true;
    }
        
  
    // p1 , q1 && q2 are collinear and q2 lies on segment p1q1
    if (o2 == Orientation::COLLINEAR) && on_segment(&p1,& q2,& q1) {
        return true;
    }
        
  
    // p2 , q2 and p1 are collinear and p1 lies on segment p2q2
    if (o3 == Orientation::COLLINEAR) && on_segment(&p2, &p1,& q2) {
        return true;
    }
        
  
    // p2 , q2 and q1 are collinear and q1 lies on segment p2q2
    if (o4 == Orientation::COLLINEAR) && on_segment(&p2,& q1,& q2) {
        return true;
    }
        
    return false;

}


fn max(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    }
    b
}


fn min (a: f64, b: f64) -> f64 {
    if a < b {
        return a;
    }
    b
}



mod tests {
    use super::determine_intersection_exists;
    use super::Point;

    #[test]
    fn test_intersection_return_true() {
        assert!(determine_intersection_exists(
            &Point::new(1.0, 0.0), 
            &Point::new(2.0, 2.0), 
            &Point::new(0.0, 3.0), 
            &Point::new(2.0, 1.0)
        ))
    }

    #[test]
    fn test_intersection_return_false() {
        assert!(!determine_intersection_exists(
            &Point::new(1.0, 0.0), 
            &Point::new(2.0, 2.0), 
            &Point::new(-4.0, 3.0),
            &Point::new(-2.0, 1.0)
        ));
    }   

}

