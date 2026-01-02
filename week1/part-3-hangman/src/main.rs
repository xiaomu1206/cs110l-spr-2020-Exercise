// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::collections::{HashMap, HashSet};
use std::io;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    let mut seen: HashMap<char,Vec<usize>> = HashMap::new();
    for (i,c) in secret_word_chars.iter().enumerate() {
        seen.entry(*c).or_insert(Vec::new()).push(i);
    }

    let mut guessed: HashSet<char> = HashSet::new();
    let mut current_state_word: Vec<char> = vec!['-'; secret_word_chars.len()];
    let mut guess_chances = NUM_INCORRECT_GUESSES;
    let mut has_guessed_word: String = String::new();
    loop {
        if guess_chances <= 0 {
            println!("Sorry, you ran out of guesses");
            break;
        }
        let display:String = current_state_word.iter().collect();
        if display == secret_word {
            println!("Congratulations you guessed the secret word: {}!", display);
            return;
        }

        println!("Welcome to CS110L Hangman!");
        println!("The word so far is {}", display);
        println!("You have guessed the following letters: {}", has_guessed_word);
        println!("You have {} guesses left", guess_chances);

        let mut input = String::new();
        print!("Please guess a letter:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        has_guessed_word.push_str(&input.trim());

        let input = input.trim();
        if input.len() != 1 {
            println!("Please enter exactly one letter");
            guess_chances -= 1;
            continue;
        }

        let c = input.chars().next().unwrap();
        if guessed.contains(&c) {
            println!("You have already guessed this letter!");
            continue;
        } else {
            guessed.insert(c);
        }

        if let Some(indices) = seen.get(&c) {
            for &idx in indices {
                current_state_word[idx] = c;
            }
        } else {
            println!("Sorry, that letter is not in the word");
            guess_chances -= 1;
        }
    }

}
