use std::io::{Read, Write};

#[derive(Default)]
struct Model {
    green: [Option<char>; 5],
    yellow: Vec<(char, usize)>,
    not: Vec<char>,
}
impl Model {
    fn matches(&self, x: &str) -> bool {
        x.len() == 5
            && x.chars().zip(&self.green).all(|(a, b)| b.map_or(true, |b| a == b))
            && self.yellow.iter().all(|(c, i)| x.contains(*c) && x.chars().enumerate().all(|(i2, c2)| c2 != *c || i2 != *i))
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
            eprintln!("It's not on the list!");
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
        println!("Guesses:");
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
        let pick = usize::from_str_radix(pick.trim(), 10).unwrap();
        let pick = three[pick];

        print!("Green: ");
        stdout.flush().unwrap();
        let mut green = String::new();
        stdin.read_line(&mut green).unwrap();
        let mut all_green = green.trim_end().len() == 5;
        for (i, c) in green.trim_end().chars().enumerate() {
            if c != ' ' && c != '_' {
                model.green[i] = Some(c);
            } else {
                all_green = false;
            }
        }
        if all_green {
            println!("Got '{}' in {}", pick, iter + 1);
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
            if model.yellow.iter().all(|(c, _)| *c != i) && model.green.iter().all(|x| *x != Some(i)) {
                model.not.push(i);
            }
        }

        iter += 1;
    }
}
