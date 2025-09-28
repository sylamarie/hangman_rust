use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io::{self, Write};

/// ASCII stages from full lives -> 0 lives (index by mistakes)
const ASCII_STAGES: &[&str] = &[
r#"
  +---+
  |   |
      |
      |
      |
      |
========="#,
r#"
  +---+
  |   |
  O   |
      |
      |
      |
========="#,
r#"
  +---+
  |   |
  O   |
  |   |
      |
      |
========="#,
r#"
  +---+
  |   |
  O   |
 /|   |
      |
      |
========="#,
r#"
  +---+
  |   |
  O   |
 /|\  |
      |
      |
========="#,
r#"
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
========="#,
r#"
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
========="#,
];

/// Game struct demonstrates object-oriented style (struct + impl)
struct Hangman {
    word: Vec<char>,
    display: Vec<char>,
    guessed: HashSet<char>,
    max_mistakes: usize,
    mistakes: usize,
    description: String,   // NEW
}


impl Hangman {
    fn new(secret: String, description: String, max_mistakes: usize) -> Self {
        let word: Vec<char> = secret.chars().collect();
        let display = word.iter().map(|_| '_').collect();
        Hangman {
            word,
            display,
            guessed: HashSet::new(),
            max_mistakes,
            mistakes: 0,
            description,   // store description
        }
    }

    /// Borrowing &self to show the current ASCII art and status (no ownership transfer)
    fn show_status(&self) {
        let stage_index = usize::min(self.mistakes, ASCII_STAGES.len() - 1);
        println!("{}", ASCII_STAGES[stage_index]);
        println!();
        // show the display spaced: _ a _ b ...
        let spaced: String = self.display.iter().map(|c| format!("{} ", c)).collect();
        println!("Word: {}", spaced.trim_end());
        println!("Guessed: {:?}", self.guessed);
        println!(
            "Mistakes: {}/{}",
            self.mistakes,
            self.max_mistakes
        );
    }

    /// Attempts to guess a letter; borrows &mut self to update game state.
    /// Returns a tuple (was_new_guess, was_correct)
    fn guess(&mut self, ch: char) -> (bool, bool) {
        let ch = ch.to_ascii_lowercase();
        if self.guessed.contains(&ch) {
            return (false, self.word.contains(&ch)); // not a new guess
        }
        self.guessed.insert(ch);

        // Check if the letter exists in the word and update display
        let mut correct = false;
        for (i, &c) in self.word.iter().enumerate() {
            if c == ch {
                self.display[i] = c;
                correct = true;
            }
        }

        if !correct {
            self.mistakes += 1;
        }

        (true, correct)
    }

    /// Check win condition
    fn is_won(&self) -> bool {
        !self.display.contains(&'_')
    }

    /// Check loss condition
    fn is_lost(&self) -> bool {
        self.mistakes >= self.max_mistakes
    }
}

/// Helper to choose a random word from a slice of &str. This demonstrates slicing usage.
fn pick_random_word(word_list: &[(&str, &str)]) -> (String, String) {
    let &(word, desc) = word_list.choose(&mut rand::thread_rng()).unwrap();
    (word.to_string(), desc.to_string())
}


/// Read a single letter from stdin; returns Option<char>
fn read_letter() -> Option<char> {
    print!("Enter a letter (or '!' to quit): ");
    io::stdout().flush().ok()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    let s = input.trim().to_lowercase();
    if s == "!" {
        return None;
    }
    // accept only single character inputs that are alphabetic
    if s.chars().count() == 1 {
        let ch = s.chars().next().unwrap();
        if ch.is_alphabetic() {
            return Some(ch);
        }
    }
    println!("Please enter a single alphabetic character.");
    read_letter()
}

fn main() {
    // Immutable config: word list is a slice (showing slicing)
let words: &[(&str, &str)] = &[
    // Animals
    ("elephant", "A large animal with a trunk."),
    ("kangaroo", "An animal that carries its baby in a pouch."),
    ("giraffe", "The tallest animal with a long neck."),
    ("penguin", "A bird that cannot fly and lives in cold regions."),
    ("dolphin", "A smart marine animal known for jumping in water shows."),

    // Places
    ("paris", "The capital of France."),
    ("tokyo", "The capital of Japan."),
    ("sahara", "The largest hot desert in the world."),
    ("amazon", "The largest rainforest on Earth."),
    ("nile", "The longest river in Africa."),

    // Things
    ("guitar", "A musical instrument with six strings."),
    ("bicycle", "A two-wheeled vehicle powered by pedaling."),
    ("computer", "An electronic device used for programming and browsing."),
    ("umbrella", "Something you use when it rains."),
    ("camera", "A device used to take pictures."),

    // Holidays
    ("christmas", "A holiday celebrated on December 25th."),
    ("halloween", "A spooky holiday with costumes and candy."),
    ("thanksgiving", "A holiday with turkey and family feasts."),
    ("easter", "A holiday with eggs and the bunny."),
    ("valentine", "A holiday for love and friendship."),
];


    println!("Welcome to Rust Hangman (terminal ASCII)!");
    println!("You can type '!' to quit at any prompt.");
    println!("Try to guess the word, one letter at a time.");
    println!("You can make up to 6 mistakes. Good luck!\n");

    let (secret, desc) = pick_random_word(words);
    let max_mistakes = 6usize;
    let mut game = Hangman::new(secret, desc, max_mistakes);
    
    // print the hint right after creating the game
    println!("Hint: {}", game.description);
    println!();


    // Main game loop (demonstrates loops, conditionals, etc.)
    loop {
        game.show_status();

        if game.is_won() {
            println!("\n🎉 Congratulations — you won!");
            break;
        }

        if game.is_lost() {
            // reveal word on loss
            let actual: String = game.word.iter().collect();
            println!("\n💀 You lost. The word was: {}", actual);
            break;
        }

        match read_letter() {
            None => {
                println!("Quitting the game. Goodbye!");
                break;
            }
            Some(ch) => {
                let (new_guess, correct) = game.guess(ch);
                if !new_guess {
                    println!("You already guessed '{}'. Try another letter.", ch);
                } else if correct {
                    println!("Good guess! '{}' is in the word.", ch);
                } else {
                    println!("Sorry, '{}' is NOT in the word.", ch);
                }
            }
        }
        println!(); // blank line between turns
    }

    println!("Thanks for playing!");
}
