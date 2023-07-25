use std::{cmp::Ordering, env, io};

fn main() {
    println!("Guess the number!");
    let secret_number = get_secret_number(env::args()).expect("No secret number is specified");

    // Why re-create this buffer every time when it can be reused?
    let mut guess = String::new();
    loop {
        println!("Please input your guess.");

        // read_line appends, need to clean before proceeding
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = match get_guess_number(&guess) {
            Some(n) => n,
            None => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn get_secret_number<I, S>(args: I) -> Option<u32>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let secret_number = args.skip(1).take(1).last()?;
    secret_number.as_ref().trim().parse().ok()
}

fn get_guess_number<S: AsRef<str>>(line: S) -> Option<u32> {
    line.as_ref().trim().parse().ok()
}

// Some simple unit tests. This program is quite annoying to test because it
// depends on the IO in the most direct way possible.
#[cfg(test)]
mod tests {
    use crate::{get_guess_number, get_secret_number};

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec!["name of the program".to_string(), $($x.to_string()),*]);
    }

    #[test]
    fn get_secret_number_valid() {
        let expected_number: u32 = 69;
        let args = vec_of_strings![expected_number.to_string()];
        let actual_number = get_secret_number(args.iter()).expect("Couldn't parse secret number");
        assert_eq!(expected_number, actual_number);
    }

    #[test]
    fn get_secret_number_empty() {
        let args = vec_of_strings![];
        let opt = get_secret_number(args.iter());
        assert!(opt.is_none());
    }

    #[test]
    fn get_secret_number_invalid() {
        let args = vec_of_strings!["Hey!"];
        let opt = get_secret_number(args.iter());
        assert!(opt.is_none());
    }

    #[test]
    fn get_guess_number_valid() {
        let expected_number: u32 = 32;
        let line = format!("    {}    ", expected_number);
        let actual_number = get_guess_number(&line).expect("Couldn't parse actual number");
        assert_eq!(expected_number, actual_number);
    }

    #[test]
    fn get_guess_number_invalid() {
        let line = "Random meaningless input";
        let opt = get_guess_number(line);
        assert!(opt.is_none());
    }

    #[test]
    fn get_guess_number_empty() {
        let line = "";
        let opt = get_guess_number(line);
        assert!(opt.is_none());
    }
}
