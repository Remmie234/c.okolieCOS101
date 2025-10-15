fn main() {
	let toshibo:f64 = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_000.00;
	let dell:f64 = 2_850_000.00;
	let acer:f64 = 250_000.00;
	let qty:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;

	//Calculate Sum
	let sum:f64 = (2.0 * toshibo) + (1.0 * mac) + (3.0 * hp) + (3.0 * dell) + (1.0 * acer);
	println!("The sum of the following sales record is N{}", sum);

	//Calculate Average
	let avg:f64 = sum / qty;
	println!("The Average of the following sales record is {}", avg);
	
}