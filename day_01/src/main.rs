use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "./input.txt";

    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut lines2 = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let test: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();

                lines.push(test[0].clone());
                lines2.push(test[1].clone());
            }
            Err(e) => eprintln!("Erro ao ler a linha: {}", e),
        }
    }

    lines.sort();
    lines2.sort();

    let mut result = 0;

    for i in 0..lines.len() {
        if lines[i] > lines2[i] {
            result += lines[i] - lines2[i];
        } else {
            result += lines2[i] - lines[i];
        }
    }

    println!("{} - {} = {}", lines[0], lines2[0], result);

    Ok(())
}
