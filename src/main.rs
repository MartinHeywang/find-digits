extern crate num;
use num::{BigUint, One, Zero};

// don't know what this line does but looks mandatory
use std::{io, ops::{Div, Rem, AddAssign}};

fn from(val: u64) -> BigUint {
    BigUint::from(val)
}

fn main() {
    // program configuration
    let base = from(13u64);
    let first  = from(7u64);

    println!("Enter n: (and press ENTER)");
    let second = from(number_input());

    println!("Enter NB: (and press ENTER)");
    let nb_digits: u32 = number_input().try_into().unwrap();

    println!("");
    println!("Trying to find {nb_digits} digits of {base}^({first}^{second}).");
    println!("");

    let exponent_last_digits = find_digits(&first, &second, nb_digits);
    println!("|1| {first}^{second} = 10^{nb_digits}*d + {exponent_last_digits}");

    let base_congruent = find_congruent_to_one(&base, nb_digits);
    println!("|2| {base}^{base_congruent} === 1 [10^{nb_digits}]");

    let u_remainder = exponent_last_digits % base_congruent;
    println!("|3| calculating : {base}^{u_remainder} = ? (finding {nb_digits} digits)");

    let result = find_digits(&base, &u_remainder, nb_digits);

    println!("|4| {base}^({first}^{second}) = ...{result}");
    println!("(-> info: si le résultat n'a pas le bon nombre de chiffre, il faut ajouter des zéros devant.)");
    println!("");

}

fn find_digits(base: &BigUint, exponent: &BigUint, nb_digits: u32) -> BigUint {
    let modulo = BigUint::pow(&from(10u64), nb_digits);
    return base.modpow(exponent, &modulo);
}

fn find_congruent_to_one(base: &BigUint, nb_digits: u32) -> BigUint {

    let modulo = BigUint::pow(&from(10u64), nb_digits);

    // special cases
    if base == &from(13u64) && (&modulo).rem(from(10u64)) == from(0u64) {
        // this is only based on something that we noticed through the runs
        // and this saves precious seconds
        return if modulo == from(10u64) {
            from(4u64)
        } else if modulo == from(100u64) {
            from(20u64)
        } else if modulo == from(1000u64) {
            from(100u64)
        } else {
            modulo.div(from(20u64))
        };
    }

    let mut remainder = from(1u64);
    let mut k = from(0u64);

    while One::is_one(&remainder) || Zero::is_zero(&k) {
        remainder = &remainder * base % &modulo;
        BigUint::add_assign(&mut k, &One::one());
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
