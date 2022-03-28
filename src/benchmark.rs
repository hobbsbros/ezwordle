// crate::benchmark

use util::words;
use indicatif::ProgressBar;

fn score_word(guess: String, secret: String) -> String {
    let mut result: String = String::new();
    for (secret_char, guess_char) in secret.chars().zip(guess.chars()) {
        if secret_char == guess_char {
            result.push('.');
        } else if secret.contains(guess_char) {
            result.push('/');
        } else {
            result.push('x');
        }
    }
    result.clone()
}

fn main() {
    // Get the list of allowed words
    let words: Vec<String> = words::get_words();

    let count: usize = words.len();
    let mut total: usize = 0;

    println!("Playing {} games of Wordle...", count);
    let bar = ProgressBar::new(count as u64);

    for secret in words.iter() {
        let mut wordlist: Vec<String> = words::get_words();
        let mut score: usize = 1;

        bar.inc(1);

        // Play a Wordle game

        let mut guess = String::from("raise");
        if guess == *secret {
            total += score;
            continue;
        }
        let combo = score_word(guess.clone(), (*secret).clone());
        wordlist = words::get_matches(guess.clone(), wordlist.clone(), combo.clone());

        'inner: for _ in 0..5 {
            guess = words::guess(wordlist.clone(), false).0;
            score += 1;
            if guess.clone() == *secret {
                total += score;
                break 'inner;
            }
            let combo = score_word(guess.clone(), (*secret).clone());
            wordlist = words::get_matches(guess.clone(), wordlist.clone(), combo);
        }
    }
    bar.finish();
    println!("\nGot an average score of {}", (total as f64)/(count as f64));
}