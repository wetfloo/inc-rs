#![allow(unused, dead_code)]

fn main() {
    let point = Point::default();
    let line = Polyline(Vec::new()); // empty line

    takes_point(point); // copied implicitly
    takes_point(point); // copied implicitly

    // Wouldn't compile, line isn't Copy
    // takes_line(line);
    // takes_line(line);

    takes_line(line.clone());
    takes_line(line.clone());
}

#[derive(Default, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Clone)]
struct Polyline(Vec<Point>);

fn takes_point(point: Point) {}

fn takes_line(line: Polyline) {}
