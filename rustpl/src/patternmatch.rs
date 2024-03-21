#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

enum Color {
	Red,
	Green,
	Blue,
	RgbColor(u8, u8, u8),
	CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8},
}

fn how_many(x: i32) -> &'static str {
	match x {
		0 => "no",
		1 | 2 => "one or two",
		12 => "a dozen",
		z @ 9..=11 => "lots of",
		_ if (x % 2 == 0) => "some",
		_ => "a few"
	}
}

fn ft_pattern_matching() {
	for x in 0..13 {
		println!("{}: I have {} oranges", x, how_many(x));
	}

	let point = (3,4);

	match point {
		(0, 0) => println!("origin"),
		(0, y) => println!("x axis, y = {}", y),
		(x, 0) => println!("y axis, x = {}", x),
		(_, y) => println!("(?,{})", y),
	}
	let c: Color = Color::RgbColor(10, 0, 0);

	match c {
		Color::Red => println!("r"),
		Color::Green => println!("g"),
		Color::Blue => println!("b"),
		Color::RgbColor(0, 0, 0) => println!("black"),
		Color::CmykColor{black: 255,..} => println!("black"),
		Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
		_ => ()
	}
}
