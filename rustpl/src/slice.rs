#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

fn use_slice(slice: &mut[i32]) {
	println!("first elem = {}, last elem {}", slice[0], slice.len());
	slice[0] = 4321;
}

fn ft_slices() {
	let mut data = [1,2,3,4,5];

	use_slice(&mut data);
	println!("{:?}", data);
}
