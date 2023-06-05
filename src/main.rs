use std::io;
use primitive_types::U256;

fn main() {
    let mut n = String::new();

    println!("Enter a number:");

    io::stdin().read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n.trim().parse()
        .expect("Please type a number!");

    println!("{}", fibonacci(n));

fn fibonacci(n: u32) -> U256 {
    match n {
        0 => U256::from(0),
        1 => U256::from(1),
        _ => {
            let mut a = U256::from(0);
            let mut b = U256::from(1);
            for _ in 2..=n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

}
