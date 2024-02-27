#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

union IntOrFloat {
	i: i32,
	f: f32,
}

fn process_value(iof: IntOrFloat) {
	unsafe {
		match iof {
			IntOrFloat { i: 42 } => {
				println!("meaning of life value");
			}

			IntOrFloat { f } => {
				println!("value = {}", f);
			}
		}
	}
}

fn ft_union() {
	let mut iof = IntOrFloat { i: 123 };
	iof.i = 234;

	let value = unsafe { iof.i };
	println!("iof.i = {}", value);

	process_value(IntOrFloat{i:42});
}
