#[allow(dead_code)]
#[allow(unused_variables)]

fn scope_and_shadowing() {
	let a = 123;

	{
		let b = 456;
		println!("inside, b = {}", b);

		let a = 777;
		println!("inside, a = {}", a);
	}

	println!("outside, a = {}", a);
}
