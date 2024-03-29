#[allow(dead_code)]
#[allow(unused_variables)]

fn operators() {
	let mut a = 2 + 3 * 4;
	println!("{}", a);
	a = a + 1;
	a -= 2;

	println!("remainder of {} / {} = {}", a, 3, (a % 3));

	let a_cubed = i32::pow(a, 3);
	println!("{} cubed is {}", a, a_cubed);

	let b = 2.5;
	let b_cubed = f64::powi(b, 3);
	let b_to_pi = f64::powf(b, std::f64::consts::PI);
	println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

	let c = 1 | 2;
	println!("1|2 = {}", c);
	let two_to_10 = 1 << 10;
	println!("2^10 = {}", two_to_10);
}
