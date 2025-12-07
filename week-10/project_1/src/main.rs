// Laptop Struct
struct Laptop {
    _name: String,
    price: u32,
}

impl Laptop {
    // Calculating Cost of three
    fn total_for_three(&self) -> u32 {
        3 * self.price
    }
}

fn main() {
    let hp = Laptop {
        _name: String::from("HP"),
        price: 650_000,
    };

    let ibm = Laptop {
        _name: String::from("IBM"),
        price: 755_000,
    };

    let toshiba = Laptop {
        _name: String::from("Toshiba"),
        price: 550_000,
    };

    let dell = Laptop {
        _name: String::from("Dell"),
        price: 850_000,
    };

    // Calculate total cost
    let total =
        hp.total_for_three() +
        ibm.total_for_three() +
        toshiba.total_for_three() +
        dell.total_for_three();

    println!("Total cost for purchasing 3 laptops from each brand: ₦{}", total);
}
