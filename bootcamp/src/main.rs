fn main() {
    println!("Welcome to Encode Club's Solana Bootcamp!");

    fizzbuzz();
}

fn fizzbuzz() {
    let mut fizzbuzz_count = 0;

    for n in 1..=301 {
        match (n % 3, n % 5 ) {
            (0, 0) => {
                println!("{}: fizz buzz", n);
                fizzbuzz_count += 1;
            },
            (0, _) => println!("{}: fizz", n),
            (_, 0) => println!("{}: buzz", n),
            _ => println!("{}", n)
        }
    }

    println!("✨ 'Fizz buzz' count occurred {} times!", fizzbuzz_count);
}
