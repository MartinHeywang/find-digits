use std::io;

fn main() {
    // program configuration
    let base: u64 = 13;
    let first: u64 = 7;

    println!("Enter n: (and press ENTER)");
    let second: u64 = number_input();

    println!("Enter NB: (and press ENTER)");
    let nb_digits: u32 = number_input().try_into().unwrap();

    println!("");
    println!("Trying to find {nb_digits} digits of {base}^({first}^{second}).");
    println!("");

    let exponent_last_digits = find_digits(first, second, nb_digits);
    println!("|1| {first}^{second} = 10^{nb_digits}*d + {exponent_last_digits}");

    let base_congruent = find_congruent_to_one(base, nb_digits);
    println!("|2| {base}^{base_congruent} === 1 [10^{nb_digits}]");

    let u_remainder = exponent_last_digits % base_congruent;
    println!("|3| calculating : {base}^{u_remainder} = ? (finding {nb_digits} digits)");

    let result: u64 = find_digits(base, u_remainder, nb_digits);

    println!("|4| {base}^({first}^{second}) = ...{result}");
    println!("(-> info: si le résultat n'a pas le bon nombre de chiffre, il faut ajouter des zéros devant.)");
    println!("");

}

fn find_digits(base: u64, exponent: u64, nb_digits: u32) -> u64 {

    let modulo: u64 = u64::pow(10, nb_digits);
    let mut current: u64 = 1;

    for _ in 0..exponent {
        current = current * base % modulo
    }

    return current;
}

fn find_congruent_to_one(base: u64, nb_digits: u32) -> u64 {

    let modulo: u64 = u64::pow(10, nb_digits);

    // special cases
    if base == 13 && modulo % 10 == 0 {
        // this is only based on something that we noticed through the runs
        // and this saves precious seconds
        return if modulo == 10 {
            4
        } else if modulo == 100 {
            20
        } else if modulo == 1000 {
            100
        } else {
            modulo / 20
        };
    }

    let mut remainder: u64 = 1;
    let mut k: u64 = 0;

    while remainder != 1 || k == 0 {
        remainder = remainder * base % modulo;
        k += 1;
    }

    return k;
}

fn number_input() -> u64 {

    let mut str = String::new();

    loop {
        // read the line
        match io::stdin().read_line(&mut str) {
            // if Ok, then try to parse the number
            Ok(_) => match str.trim().parse() {
                Ok(val) => {
                    println!("OK!");
                    return val;
                },
                Err(_) => {
                    println!("ERR! Please input a number. Retry? ");
                    continue;
                }
            },

            // if Err, prompt the user to try again
            Err(_) => {
                println!("ERR! Failed to read line. Retry?");
                continue;
            }
        }
    };
}
