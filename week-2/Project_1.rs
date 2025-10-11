fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	// Compound Interest
	let a = p * ( 1.0 + ( r / 100.0 )).powf(n);
	println!("The amount is N{}", a);
	let cl = a - p;
	println!("The compound interest is N{}", cl);
}