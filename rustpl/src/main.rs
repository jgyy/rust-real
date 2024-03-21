#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

fn use_slice(slice: &mut [i32]) {
	println!("first elem = {}, ");
}

fn ft_slices() {
	let mut data = [1,2,3,4,5];

	use_slice(&data[1..4]);
}

fn main() {
	ft_slices();
}
