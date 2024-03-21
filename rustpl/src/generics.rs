#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

struct Point<T> {
	x: T,
	y: T
}

struct Line<T> {
	start: Point<T>,
	end: Point<T>
}

fn ft_generics() {
	let a:Point<f64> = Point { x: 0.3, y: 4f64 };
	let b = Point { x: 1.2, y: 3.4 };

	let myline = Line { start: a, end: b };
}
