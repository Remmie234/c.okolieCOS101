fn main() {
	let toshibo:f64 = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_000.00;
	let dell:f64 = 2_850_000.00;
	let acer:f64 = 250_000.00;
	let qty:f64 = 2.00 + 1.00 + 3.00 + 3.00 + 1.00;

	//Calculating Sum
	let sum:f64 = (2.00 * toshibo) + (1.00 * mac) + (3.00 * hp) + (3.00 * dell) + (1.00 * acer);
	println!("The sum of the following sales record is N{}", sum);

	//Calculating Average
	let avg:f64 = sum / qty;
	println!("The Average of the following sales record is {}", avg);
	
}