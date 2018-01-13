extern crate high_line;
use high_line::ask;

fn main() {
    ask("What is your name?").prompt();

    /*ask::<u64>("What is your lucky number?").prompt();

    ask("Enter your birth date:")
        .parse(Date::from_str)
        .transform(|date| Date::today() - d)
        .transform(|duration| duration.in_years())
        .validate(|age| age >= 21)
        .error_prompt("Must be of legal drinking age");

    ask::<Option<String>>("What is your password?").prompt();

    ask::<Option<bool>>("Do you want to continue? (q to exit)")
        .escape_with("q")
        .prompt();*/
}

/*

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        prompt("What is your age?").ask();

        prompt("What is your age?")
            .parse_as::<u64>()
            .error("Please enter a number?")
            .ask();

        prompt("What is your age?")
            .parse_as::<u64>()
            .validate(|i| i > 21, "Must be over 21")
            .ask();

        prompt("What is your age?")
            .parse(|s| u64::from_str(s))
            .validate(|i| i > 21, "Must be of legal drinking age")
            .ask();

        prompt("What is your age?")
            .validate::<u64>(|i| i > 21, "Must be of legal drinking age")
            .ask();

        prompt("What is your age?")
            .validate::<u64>(|i| i > 21)
            .error("Must be of legal drinking age")
            .ask();

        prompt::<u64>("What is your age?").ask();

        prompt::<u64>("What is your age?")
            .validate(|i| i > 21)
            .error("Must be of legal drinking age")
            .ask();

        prompt::<u64>("What is your age?")
            .transform(|i| i - 20)
            .validate(|i| i > 0)
            .ask_or("Must be of legal drinking age");

        prompt("Enter your birth date")
            .parse(Date::from_str)
            .transform(|date| Date::today() - d)
            .transform(|duration| duration.in_years() )
            .validate(|age| age >= 21)
            .ask_or("Must be of legal drinking age");

        ask("Enter your birth date")
            .parse(Date::from_str)
            .transform(|date| Date::today() - d)
            .transform(|duration| duration.in_years() )
            .validate(|age| age >= 21)
            .error_prompt("Must be of legal drinking age");

        ask("What is your birth date?").prompt();


        ask("What is your password?")
            .allow_blank()
            .prompt();

        ask::<Option<String>>("What is your password?")
            .prompt(); // Option<String>

        ask::<Option<String>>("What is your password?")
            .escape_with("q")
            .prompt();

enum Color { Red, Yellow, Green };

        prompt("What color is the stoplight?")
            .parse(|s| Color::from_str(s))
            .error_message("Must be a stoplight color")
            .validate(|c| {
                match(c) {
                    Red => Ok(),
                    _ => Err("must be Red"),
                }
            })
            .ask();

        prompt("What color is the stoplight?")
            .parse(|s| Color::from_str(s))
            .error(|et| "Must be a stoplight color")
            .validate(|c| {
                match(c) {
                    Red => Ok(),
                    _ => Err("must be Red"),
                }
            })
            .error(|et| "Actually it has to be Red")
            .ask();

        prompt("What color is the stoplight?")
            .transform(String::capitalize)
            .parse(|s| Color::from_str(s))
            .validate(|c| {
                match(c) {
                    Red => Ok(),
                    _ => Err("must be Red"),
                }
            })
            .transform(String::capitalize)
            .error("The color really has to be Red.")
            .ask();

        prompt("What color is the stoplight?")
            .parse(|s| Color::from_str(s))
            .validate()
            .validate(|c| {
                match(c) {
                    Red => Ok(),
                    _ => Err("must be Red"),
                }
            })
            .error("The color really has to be Red.")
            .ask();

            What color is the stoplight?
            Please enter a valid value.
            What color is the stoplight? asldkfjlaksdjf
            Please enter a valid value.
            What color is the stoplight? green
            // move on the program
    }
}
*/
