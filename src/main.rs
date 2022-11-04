extern crate num;
use num::{BigUint, One, Zero};
use std::{
    io,
    ops::{AddAssign, Div, Rem},
};

// don't know what this line does but looks mandatory
use std::io::stdout;

use crossterm::{cursor::*, execute, style::*, terminal::*};

fn from(val: u64) -> BigUint {
    BigUint::from(val)
}
fn wait(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis));
}

fn main() {
    let mut stdout = stdout();

    // print d'introduction
    execute!(
        stdout,
        SetAttribute(Attribute::Bold),
        Print("\n### Saisie des valeurs\n"),
        SetAttribute(Attribute::Reset),
        Print("\nCe programme te permet d'obtenir les "),
        SetForegroundColor(Color::Blue),
        Print("NB "),
        ResetColor,
        Print("derniers chiffres décimaux de 13^(7^"),
        SetForegroundColor(Color::Blue),
        Print("n"),
        ResetColor,
        Print(")."),
        Print("\n"),
    )
    .unwrap();

    // program configuration
    let base = from(13u64);
    let first = from(7u64);

    // 0ème étape : demander à l'utilisateur les informations
    execute!(
        stdout,
        Print("Quel est ton "),
        SetForegroundColor(Color::Blue),
        Print("n "),
        ResetColor,
        Print("dans 13^(7^"),
        SetForegroundColor(Color::Blue),
        Print("n"),
        ResetColor,
        Print(") ? "),
        SetForegroundColor(Color::Cyan),
    )
    .unwrap();
    let second = from(number_input());

    execute!(
        stdout,
        ResetColor,
        Print("Combien de chiffres souhaites-tu obtenir ? "),
        SetForegroundColor(Color::Cyan)
    )
    .unwrap();
    let nb_digits: u32 = number_input().try_into().unwrap();

    execute!(
        stdout,
        ResetColor,
        SetAttribute(Attribute::Bold),
        Print("\n### Recherche des "),
        SetForegroundColor(Color::Cyan),
        Print(format!("{nb_digits} ")),
        // ResetColor also resets the weight...
        SetForegroundColor(Color::Reset),
        Print("derniers chiffres décimaux de "),
        Print(format!("{base}")),
        Print("^("),
        Print(format!("{first}")),
        Print("^"),
        SetForegroundColor(Color::Blue),
        Print(format!("{second}")),
        ResetColor,
        Print(")...\n\n"),
        SetAttribute(Attribute::Reset)
    )
    .unwrap();

    // ------------
    // 1ère étape : trouver les derniers chiffres de l'exposant 7^second

    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("|1| "),
        ResetColor,
        Print("Trouver les "),
        SetForegroundColor(Color::Cyan),
        Print(format!("{nb_digits} ")),
        Print("derniers chiffres décimaux de "),
        Print(format!("{first}")),
        Print("^"),
        SetForegroundColor(Color::Blue),
        Print(format!("{second}")),
        Print("\n"),
        ResetColor,
    )
    .unwrap();

    let exponent_last_digits = find_digits(&first, &second, nb_digits);

    wait(200);

    execute!(
        stdout,
        MoveUp(1),
        Clear(ClearType::FromCursorDown),
        SetForegroundColor(Color::Green),
        Print("|1| "),
        ResetColor,
        Print(format!("{first}^")),
        SetForegroundColor(Color::Blue),
        Print(format!("{second}")),
        ResetColor,
        Print(" = ..."),
        SetForegroundColor(Color::Magenta),
        Print(format!("{exponent_last_digits}")),
        ResetColor,
        Print("\n"),
    )
    .unwrap();

    // ------------
    // 2ème étape : trouver une puissance de 13 qui congrue à 1 modulo 10^nb_digits

    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("|2| "),
        ResetColor,
        Print("Trouver un k tel que 13^k === 1 [10^"),
        SetForegroundColor(Color::Blue),
        Print(format!("{nb_digits}")),
        ResetColor,
        Print("]\n"),
    )
    .unwrap();

    let base_congruent = find_congruent_to_one(&base, nb_digits);
    wait(200);

    execute!(
        stdout,
        MoveUp(1),
        Clear(ClearType::FromCursorDown),
        SetForegroundColor(Color::Green),
        Print("|2| "),
        ResetColor,
        Print("13^"),
        SetForegroundColor(Color::Magenta),
        Print(format!("{base_congruent} ")),
        ResetColor,
        Print("=== 1 [10^"),
        SetForegroundColor(Color::Blue),
        Print(format!("{nb_digits}")),
        ResetColor,
        Print("]\n"),
    )
    .unwrap();

    // ------------
    // 3ème étape : trouver une puissance de 13 qui congrue à 13^(7^second) modulo 10^nb_digits

    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("|3| "),
        ResetColor,
        Print("Trouver une puissance de 13 qui congrue à 13^(7^"),
        SetForegroundColor(Color::Blue),
        Print(format!("{second}")),
        ResetColor,
        Print(") modulo 10^"),
        SetForegroundColor(Color::Blue),
        Print(format!("{nb_digits}")),
        ResetColor,
        Print("\n"),
    )
    .unwrap();

    let u_remainder = exponent_last_digits % base_congruent;
    wait(200);

    execute!(
        stdout,
        MoveUp(1),
        Clear(ClearType::FromCursorDown),
        SetForegroundColor(Color::Green),
        Print("|3| "),
        ResetColor,
        Print("13^"),
        SetForegroundColor(Color::Magenta),
        Print(format!("{u_remainder} ")),
        ResetColor,
        Print("=== 13^(7^"),
        SetForegroundColor(Color::Blue),
        Print(format!("{second}")),
        ResetColor,
        Print(") [10^"),
        SetForegroundColor(Color::Blue),
        Print(format!("{nb_digits}")),
        ResetColor,
        Print("]\n")
    )
    .unwrap();

    // ------------
    // 4ème étape : trouver les derniers chiffres de la puissance obtenue à l'étape 3

    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("|4| "),
        ResetColor,
        Print("Trouver les derniers chiffres de 13^"),
        SetForegroundColor(Color::Magenta),
        Print(format!("{u_remainder}")),
        ResetColor,
        Print("\n"),
    )
    .unwrap();

    let result = find_digits(&base, &u_remainder, nb_digits);
    wait(200);

    execute!(
        stdout,
        MoveUp(1),
        Clear(ClearType::FromCursorDown),
        SetForegroundColor(Color::Green),
        Print("|4| "),
        ResetColor,
        Print("Trouvé!\n")
    )
    .unwrap();

    let formatted_result = format_result(&result, nb_digits);

    execute!(
        stdout,
        Print("\n"),
        Print("Les "),
        SetForegroundColor(Color::Cyan),
        Print(format!("{nb_digits} ")),
        ResetColor,
        Print("derniers chiffres décimaux de 13^(7^"),
        SetForegroundColor(Color::Blue),
        Print(format!("{second}")),
        ResetColor,
        Print(") sont : "),
        SetForegroundColor(Color::Red),
        SetAttribute(Attribute::Bold),
        Print(format!("{formatted_result}")),
        ResetColor,
        SetAttribute(Attribute::Reset),
        Print("\n\n"),
    )
    .unwrap();

    println!("Appuie sur entrée pour terminer le processus.");

    let mut keep_window_open = String::new();
    std::io::stdin().read_line(&mut keep_window_open).unwrap();
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
                    return val;
                }
                Err(_) => {
                    continue;
                }
            },

            // if Err, prompt the user to try again
            Err(_) => {
                continue;
            }
        }
    }
}

fn format_result(result: &BigUint, nb_digits: u32) -> String {
    let mut string = result.to_string();

    for _ in 0..((<u32 as TryInto<usize>>::try_into(nb_digits).unwrap()) - string.len()) {
        string.insert_str(0, "0");
    }

    let mut with_spaces = String::new();
    let offset = string.len() % 3;

    for k in 0..string.len() {
        if (k) % 3 == offset && k != 0 {
            with_spaces.push(' ');
        }
        with_spaces.push_str(&string[k..k + 1]);
    }

    with_spaces
}
