extern crate chrono;
extern crate high_line;
use high_line::ask;
use chrono::naive::NaiveDate;
use chrono::{Datelike, Local};
use std::str::FromStr;

fn main() {
    // Desired API:

    /*
    ask("What is your name?").prompt();

    ask::<u64>("What is your lucky number?").prompt();

    ask("Enter your birth date:")
        .parse(Date::from_str)
        .transform(|date| Date::today() - d)
        .transform(|duration| duration.in_years())
        .validate(|age| age >= 21)
        .error_prompt("Must be of legal drinking age");

    ask::<Option<String>>("What is your password?").prompt();

    ask::<Option<bool>>("Do you want to continue? (q to exit)")
        .escape_with("q")
        .prompt();
    */

    // Final API:

    let name: String = ask("What is your name?").prompt();

    let lucky_number: u64 = ask("What is your lucky number?").prompt();

    let age = ask("Enter your birth date (YYYY-MM-DD):")
        .parse(|s| NaiveDate::from_str(&s))
        .transform(|birth_date| {
            let today = Local::today().naive_local();
            if birth_date < today {
                let birth_year = birth_date.year();
                let current_year = today.year();
                let birthday =
                    NaiveDate::from_ymd(current_year, birth_date.month(), birth_date.day());
                let mut age = current_year - birth_year;
                if today < birthday {
                    age -= 1;
                }
                Some(age)
            } else {
                None
            }
        })
        .validate(|age| *age >= 21)
        .error_prompt("Must be of legal drinking age");

    let password = ask("What is your password?").prompt::<Option<String>>();

    let mut numbers = vec![];
    loop {
        if let Some(n) = ask("Give me a number (q to quit):")
            .parse_as::<u64>()
            .exit_on("q")
            .and_on("Q")
            .error_prompt("Enter a number or 'q' to exit")
        {
            numbers.push(n);
        } else {
            break;
        }
    }

    println!();
    println!("Nice to meet you {}", name);
    println!("How does it feel to be {}?", age);
    if !numbers.contains(&lucky_number) {
        println!(
            "If {} is your lucky number, why didn't you include it in the numbers you gave me?",
            lucky_number
        );
    }
    println!();
    println!("Your lucky number {} was chosen!", lucky_number);
    println!("Maybe that wasn't so lucky, you've been chosen to hack.");
    println!(
        "You just gave us your password: {}",
        password.unwrap_or(String::new())
    );
}
