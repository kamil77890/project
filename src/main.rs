use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    let words = vec!["program", "komputer", "rust", "konsola", "zadanie"];

    let chosen_word = words.choose(&mut rand::thread_rng()).unwrap(); //roandomowy wyraz
    let chosen_word_chars: HashSet<char> = chosen_word.chars().collect();// tworzenie liter z wyrazów

    let mut guessed_letters: HashSet<char> = HashSet::new();
    let mut correct_guesses = HashSet::new();

    loop {
        let mut display_word = String::new(); // logiaka
        for letter in chosen_word.chars() {
            if guessed_letters.contains(&letter) {
                display_word.push(letter);
            } else {
                display_word.push('_');
            }
            display_word.push(' ');
        }

        println!("Wylosowany wyraz: {}", display_word);

        println!("Podaj literę: ");
        io::stdout().flush().unwrap();
        let input = String::new(); // Input

        let guessed_letter = match input.trim().chars().next() { // sprawdza czyjak nie ma litery to kontynuuje
            Some(letter) => letter,
            None => {
                println!("Nie podano litery!");
                continue;
            }
        };

        if chosen_word_chars.contains(&guessed_letter) { // czy dobrze
            println!("dobrze");
            correct_guesses.insert(guessed_letter);
        } else {
            println!("zle");
        }

        guessed_letters.insert(guessed_letter);

        if chosen_word_chars.difference(&correct_guesses).count() == 0 {
            println!("wygrana!"); //win
            break;
        }
    }
}
