extern crate rand;

use rand::*;
use std::string::*;
use std::io::*;
use std::fs::File;


struct GameRound {
    answer: String,
    placeholder: String,
}

impl GameRound {
    fn new(random_word: &String) -> Self {
        let new_game = Self {   answer: random_word.to_string(),
                                placeholder: "_".repeat(random_word.len()), };

        println!("Secret word is {} symbols long {}",
                 new_game.placeholder.len(),
                 new_game.placeholder);

        new_game
    }

    fn guess(&mut self, character: &str) {
        let mut char_found = false;

        for (i, c) in self.answer.chars().enumerate() {
            if c == character.chars().next().unwrap() {
                self.placeholder.remove(i);
                self.placeholder.insert(i, c);
                char_found = true;
            }
        }
        if char_found {
            println!("There is such character! Your guess is: {}", self.placeholder);
        } else {
            println!("There is no such character!");
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


struct WordBase {
    answers_base: Vec<String>,
}

impl WordBase {

    fn new(filename: String) -> Self {

        let mut temp_base: Vec<String> = Vec::new();

        let f = File::open(filename);

        match f {
            Ok(handle) => {
                let mut lines = BufReader::new(&handle).lines();

                for (_i, line) in lines.enumerate() {
                    temp_base.push(line.unwrap());
                }
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }

        Self {
            answers_base: temp_base,
        }
    }

    fn get_random_word(&self) -> String {
        let random_answer = self.answers_base[thread_rng().gen_range(0, self.answers_base.len())].clone();

        random_answer
    }
}

fn main() {
    let word_base: WordBase = WordBase::new(String::from("base.txt"));

    loop {
        let mut input = String::new();

        println!("Start new game? [y/n] ");
        match stdin().read_line(&mut input) {
            Ok(_n) => {
                if input.trim() == "y" {
                    play_process(word_base.get_random_word());
                } else 
                if input.trim() == "n" {
                    println!("k bye!");
                    std::process::exit(0);
                }
            }
            Err(error) => println!("Error {}", error)
        }

        
    }

}


fn play_process(word: String) {

    let mut game = GameRound::new(&word);

    while !game.is_done() {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_n) => {
                if input.trim().len() == 1 {
                    game.guess(input.trim());
                } else {
                    println!("Enter only 1 character!");
                }
            }
            Err(error) => println!("Unknown symbol {}!", error),
        }
    }
    println!("You win!");

}