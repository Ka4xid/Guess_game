use std::string::*;

struct GuessHolder {
    answer: String,
    placeholder: String,
}

impl GuessHolder {

    fn new(word: &str) -> Self {
        Self {
            answer: String::from(word),
            placeholder: "_".repeat(word.len()),
        }
    }

    fn replace_if_contains(&mut self, character: &String) {

        for (i, c) in self.answer.chars().enumerate() {
            if c == character.chars().next().unwrap() {
                self.placeholder.remove(i);
                self.placeholder.insert(i, c);
            }
        }
    }

    fn is_done(&self) -> bool {
        if self.placeholder == self.answer {
            true
        } else {
            false
        }
    }
}

fn main() {

    let mut guess = GuessHolder::new("qweeerty");

    println!("Word is {} chars long: {}", guess.placeholder.len(), guess.placeholder);

    while !guess.is_done() {

        let mut character = String::new();

        match std::io::stdin().read_line(&mut character) {
            Ok(_n) => {

                character = String::from(character.trim());
                if character.len() == 1 {
                    print!("\r");

                    println!("Your guess: {}", character);

                    guess.replace_if_contains(&character);

                    println!("{}", guess.placeholder);

                } else {
                    println!("Enter 1 symbol!");
                }

            }
            Err(error) => println!("Unknown symbol {}!", error),
        }
    }

    println!("You win!");
}
