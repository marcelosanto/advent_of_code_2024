use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut results = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let numbers_vec: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();

                lines.push(numbers_vec)
            }
            Err(e) => eprintln!("Erro ao ler a linha: {}", e),
        }
    }

    for line in lines.iter() {
        if is_valid_sequence(&line) {
            results += 1;
        }
    }
    println!("results {:?}: lines {:?}", results, lines.len());
    Ok(())
}

fn is_valid_sequence(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    let increasing = numbers[1] > numbers[0];

    numbers.windows(2).fold(true, |acc, window| {
        let diff = (window[1] - window[0]).abs();
        let is_consistent = if increasing {
            window[1] > window[0]
        } else {
            window[1] < window[0]
        };
        acc && (1..=3).contains(&diff) && is_consistent
    })
}

fn verificar_sequencia(numeros: &Vec<i32>) -> bool {
    let mut count = 0;
    let mut truee = true;

    let mut aumentando = false;
    let mut diminuindo = false;
    let diferenca = |x: i32, y: i32| {
        let diff = (x - y).abs();
        (1..=3).contains(&diff)
    };

    for i in 0..numeros.len() {
        if count == 0 {
            if numeros[i] > numeros[1] {
                diminuindo = true;
                count = numeros[i];
            } else if numeros[1] > numeros[i] {
                aumentando = true;
                count = numeros[i];
            } else {
                truee = false;
                break;
            }
        } else if diminuindo && count > numeros[i] && diferenca(count, numeros[i]) {
            count = numeros[i];
        } else if aumentando && count < numeros[i] && diferenca(count, numeros[i]) {
            count = numeros[i];
        } else {
            truee = false;
            break;
        }
    }

    return truee;
}
