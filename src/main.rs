use std::io::{self, Write};

use scacchiera::scacchiera::Scacchiera;

pub mod scacchiera;

fn main() {
    let mut s = Scacchiera::default();
    println!("{}", s);

    loop {
        let mut start = String::new();
        let mut end = String::new();
        println!("Inserisci il punto di inizio: ");
        io::stdin().read_line(&mut start).unwrap();
        println!("Inserisci il punto di fine: ");
        io::stdin().read_line(&mut end).unwrap();

        if start.contains("q") || end.contains("q") {
            std::process::exit(1)
        }

        match s.move_pezzo(&start, &end) {
            Ok(_) => {}
            Err(e) => eprintln!("Errore: {}", e),
        };
        println!("{}", s);
    }
}
