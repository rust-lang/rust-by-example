struct Point(f32, f32);

impl Point {
    fn distance(&self, Point(x2, y2): Point) -> f32 {
        let Point(x1, y1) = *self;
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }
    
    fn translate(&self, dx: f32, dy: f32) -> Point {
        let Point(x, y) = *self;
        Point(x + dy, y + dx)
        // Whoops, this method was implemented incorrectly!
    }
}

#[test]
fn distance_test() {
    let origin = Point(0.0, 0.0);
    let pt = Point(4.0, 3.0);
    assert_eq!(origin.distance(pt), 5.0);
}

#[test]
fn translate_test() {
    let pt = Point(1.0, 4.5);
    let Point(new_x, new_y) = pt.translate(5.0, 3.0);
    // This will fail since the translate method is incorrect.
    assert_eq!(new_x, 6.0);
    assert_eq!(new_y, 7.5);
}
