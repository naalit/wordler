use std::io::{Read, Write};

/// All the information we know about the word
#[derive(Default)]
struct Model {
    /// Characters that we know the location of in the word
    green: [Option<char>; 5],
    /// Characters we know are in the word, along with a location we know they're *not*
    yellow: Vec<(char, usize)>,
    /// Characters we know are *not* in the word
    not: Vec<char>,
}
impl Model {
    /// Whether the provided word is possible.
    /// It must be 5 letters long, and it must match the information we have.
    fn matches(&self, x: &str) -> bool {
        x.len() == 5
            && x.chars()
                .zip(&self.green)
                .all(|(a, b)| b.map_or(true, |b| a == b))
            && self.yellow.iter().all(|(c, i)| {
                x.contains(*c) && x.chars().enumerate().all(|(i2, c2)| c2 != *c || i2 != *i)
            })
            && self.not.iter().all(|c| !x.contains(*c))
    }
}

fn main() {
    let mut file = std::fs::File::open("unigram_freq.csv").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut words = Vec::new();
    for i in data.lines() {
        words.push(i.split(',').next().unwrap());
    }

    let mut model = Model::default();
    let mut start = 0;
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut iter = 1;
    loop {
        while start < words.len() && !model.matches(words[start]) {
            start += 1;
        }
        if start >= words.len() {
            eprintln!("It looks like we've exhausted all possible words.");
            eprintln!("Chances are you skipped a valid word or entered the results wrong.");
            std::process::exit(1);
        }

        let mut three = Vec::new();
        for &i in &words[start..] {
            if model.matches(i) {
                three.push(i);
                if three.len() >= 3 {
                    break;
                }
            }
        }
        println!("\nGuesses:");
        for (i, s) in three.iter().enumerate() {
            println!("  {}: {}", i, s);
        }
        print!("Pick: ");
        stdout.flush().unwrap();
        let mut pick = String::new();
        stdin.read_line(&mut pick).unwrap();
        // This is used when it's not a word wordle will accept - e.g. names
        if pick.trim() == "n" {
            start += 1;
            continue;
        }
        // If they just hit enter, count that as 0
        let pick = if pick.trim().is_empty() {
            0
        } else {
            usize::from_str_radix(pick.trim(), 10).unwrap()
        };
        let pick = three[pick];

        print!("Green: ");
        stdout.flush().unwrap();
        let mut green = String::new();
        stdin.read_line(&mut green).unwrap();
        let mut all_green = green.trim_end().len() == 5;
        for (i, c) in green.trim_end().chars().enumerate() {
            if c != ' ' && c != '_' {
                model.green[i] = Some(c);
                // We don't actually remove the letter from `model.yellow`, because it could be at
                // other spots in the word too, so we need to remember which spots we ruled out.
            } else {
                all_green = false;
            }
        }
        if all_green {
            println!("Got '{}' in {}", pick, iter);
            return;
        }

        print!("Yellow: ");
        stdout.flush().unwrap();
        let mut yellow = String::new();
        stdin.read_line(&mut yellow).unwrap();
        for (i, c) in yellow.trim_end().chars().enumerate() {
            if c != ' ' && c != '_' {
                model.yellow.push((c, i));
            }
        }

        for i in pick.chars() {
            if model.yellow.iter().all(|(c, _)| *c != i)
                && model.green.iter().all(|x| *x != Some(i))
            {
                model.not.push(i);
            }
        }

        iter += 1;
    }
}
