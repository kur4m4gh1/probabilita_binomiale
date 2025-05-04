use std::io;

fn main() {
    let n: u32;
    let k: u32;
    let p: f32;

    n = loop {
        println!("Inserisci il numero di possibilità: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input non valido");

        match input.trim().parse::<u32>() {
            Ok(num) => {
                println!("Hai inserito il numero: {}", num);
                break num; // Esci dal loop dopo un input valido
            },
            Err(_) => {
                println!("Input non valido, riprova.");
                continue; // Riprova se l'input non è valido
            }
        };
    };

    k = loop {
        println!("Inserisci il numero di successi: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input non valido");

        match input.trim().parse::<u32>() {
            Ok(num) => {
                println!("Hai inserito il numero: {}", num);
                break num; // Esci dal loop dopo un input valido
            },
            Err(_) => {
                println!("Input non valido, riprova.");
                continue; // Riprova se l'input non è valido
            }
        };
    };

    p = loop {
        println!("Inserisci la probabilità (f32 da 0 a 1): ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input non valido");

        match input.trim().parse::<f32>() {
            Ok(num) if num >= 0.0 && num <= 1.0 => {
                println!("Hai inserito il numero: {}", num);
                break num; // Esci dal loop dopo un input valido
            },
            Ok(_) => {
                println!("Il numero deve essere compreso tra 0 e 1. Riprova.");
                continue; // Se il numero non è valido, continua il loop
            },
            Err(_) => {
                println!("Input non valido, riprova.");
                continue; // Riprova se l'input non è valido
            }
        };
    };

    let bin_p: f32 = ((fact(n)/(fact(k)*fact(n-k))) as f32 * (p.powi(k as i32))*((1.0-p).powi((n-k) as i32))) * 100.0;

    println!("La probabilità è di {bin_p}%");
}

fn fact(n: u32) -> u32{
    if n == 0 {
        1
    } else {
        (1..=n).product()
    }
}
