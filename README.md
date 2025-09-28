# Overview

This project is a **terminal-based Hangman game** written in Rust. The game randomly selects words from different categories (animals, places, things, holidays) and provides a short description as a hint. Players guess letters, and each incorrect guess adds a piece to the ASCII-art hangman until either the word is solved or the player runs out of attempts.  

The purpose of writing this software was to strengthen my skills as a software engineer by practicing Rust fundamentals, such as ownership and borrowing, data structures, and control flow. This project allowed me to explore how Rust handles memory safely and how to design a simple yet interactive program with structured code.  

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

- **Editor:** Visual Studio Code with Rust Analyzer extension for code completion and inline error checking.  
- **Build Tool:** Cargo, which comes with the Rust toolchain, for compiling and running the program.  
- **Programming Language:** Rust (edition 2021).  
- **Libraries/Crates:**  
  - [`rand`](https://crates.io/crates/rand) — used to randomly select a word and its description from the word list.  

# Useful Websites

- [Rust Book](https://doc.rust-lang.org/book/) — official guide for learning Rust concepts like ownership and borrowing.  
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — quick reference for Rust syntax and patterns.  
- [crates.io](https://crates.io) — Rust’s crate registry, useful for finding external libraries like `rand`.  
- [VS Code Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) — extension for Rust development in VS Code.  

# Future Work

- Add **difficulty levels** (easy, medium, hard) to adjust the number of mistakes allowed.  
- Support **whole word guessing** in addition to single letters.  
- Add **color output** to make the ASCII-art hangman more visually engaging.  
- Store and load words from an external file instead of using a hard-coded list.