#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

struct Point {
	x: f64,
	y: f64
}

Struct Line {
	start: Point,
	end: Point
}

fn structures() {
	let p = Point { x: 3.0, y: 4.0 };
	println!("point p is at ({}, {})", p.x, p.y);
}
