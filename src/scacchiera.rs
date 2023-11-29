pub mod scacchiera {
    //! Modulo che contiene la funzione per creare una scacchiera di gioco.

    use core::fmt;

    const ROWS: usize = 8;
    const COLS: usize = 8;

    #[derive(Debug)]
    pub struct Scacchiera([[Option<&'static str>; COLS]; ROWS]);

    impl Default for Scacchiera {
        fn default() -> Self {
            Scacchiera([
                [
                    Some("t"),
                    Some("c"),
                    Some("a"),
                    Some("d"),
                    Some("r"),
                    Some("a"),
                    Some("c"),
                    Some("t"),
                ],
                [
                    Some("p"),
                    Some("p"),
                    Some("p"),
                    Some("p"),
                    Some("p"),
                    Some("p"),
                    Some("p"),
                    Some("p"),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some("P"),
                    Some("P"),
                    Some("P"),
                    Some("P"),
                    Some("P"),
                    Some("P"),
                    Some("P"),
                    Some("P"),
                ],
                [
                    Some("T"),
                    Some("C"),
                    Some("A"),
                    Some("D"),
                    Some("R"),
                    Some("A"),
                    Some("C"),
                    Some("T"),
                ],
            ])
        }
    }

    use colored::Colorize;

    const NUM_SCACCHIERA: [i8; 8] = [8, 7, 6, 5, 4, 3, 2, 1];
    const CHAR_SCACCHIERA: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];

    impl Scacchiera {
        pub fn check_position(x: &str, y: i8, msg: &'static str) -> Result<(), &'static str> {
            if !(CHAR_SCACCHIERA.contains(&x) && NUM_SCACCHIERA.contains(&y)) {
                return Err(msg);
            }
            Ok(())
        }

        pub fn move_pezzo(&mut self, start: &str, end: &str) -> Result<(), &str> {
            let (row_start, cols_start) = start.split_at(1);
            let cols_start = cols_start.trim().parse::<i8>().unwrap();
            let (row_end, cols_end) = end.split_at(1);
            let cols_end = cols_end.trim().parse::<i8>().unwrap();
            dbg!(row_start, cols_start);
            dbg!(row_end, cols_end);

            match Scacchiera::check_position(
                row_start,
                cols_start,
                "Posizione di partenza non valida",
            ) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
            match Scacchiera::check_position(row_end, cols_end, "Posizione di arrivo non valida") {
                Ok(_) => {}
                Err(e) => return Err(e),
            };

            // controllo se il pezzo esiste e se può muovere
            let index_col = |col: i8| NUM_SCACCHIERA.iter().position(|x| x == &col).unwrap();
            let index_row = |row: &str| {
                CHAR_SCACCHIERA
                    .iter()
                    .rev()
                    .position(|x| x == &row)
                    .unwrap()
            };

            let index_col_end = index_col(cols_end);
            let index_row_end = index_row(row_end);
            let index_col_start = index_col(cols_start);
            let index_row_start = index_row(row_start);

            if let Some(pezzo) = self.0[index_col(cols_start)][index_row(row_start)] {
                match pezzo.to_lowercase().as_str() {
                    "p" => {
                        if row_start != row_end {
                            return Err("Mossa non eseguibile");
                        }
                        if cols_start == 7 || cols_start == 2 {
                            if cols_end == cols_start - 2 || cols_end == cols_start + 2 {
                                self.0[index_col_end][index_row_end] = Some(pezzo);
                                self.0[index_col_start][index_row_start] = None;
                            } else {
                                self.0[index_col_start + 1][index_row_start] = Some(pezzo);
                                self.0[index_col_start][index_row_start] = None;
                            }
                        } else {
                            self.0[index_col_start + 1][index_row_start] = Some(pezzo);
                            self.0[index_col_start][index_row_start] = None;
                        }
                    }
                    "a" => {}
                    "c" => {}
                    "t" => {}
                    "d" => {}
                    "r" => {}
                    _ => {}
                }
                // self.0[index_col(cols_end)][index_row(row_end)] = Some(pezzo);
            } else {
                return Err("Il pezzo non esiste o non puoi muovere");
            }

            Ok(())
        }

        fn check_move_pedone() {}
    }

    impl fmt::Display for Scacchiera {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let color = [
                colored::CustomColor::new(255, 207, 159), // chiaro
                colored::CustomColor::new(210, 140, 69),  // scuro
            ];

            for row in self.0.iter().enumerate() {
                write!(f, "{:2}", NUM_SCACCHIERA[row.0].to_string().normal())?; // numeri a lato

                let mut flag = if row.0 % 2 == 0 { true } else { false };
                let color_block = |flag: bool| -> (bool, colored::CustomColor) {
                    if flag == true {
                        return (!flag, color[0]);
                    } else {
                        return (!flag, color[1]);
                    }
                };

                for col in row.1.iter().rev() {
                    let (x, color) = color_block(flag);
                    flag = x;
                    write!(
                        f,
                        "{:^3}",
                        match *col {
                            Some(str) => {
                                let pezzo = match str {
                                    "p" => "♟",
                                    "t" => "♜",
                                    "c" => "♞",
                                    "a" => "♝",
                                    "d" => "♛",
                                    "r" => "♚",
                                    "P" => "♙",
                                    "T" => "♖",
                                    "C" => "♘",
                                    "A" => "♗",
                                    "D" => "♕",
                                    "R" => "♔",
                                    _ => todo!("Codice non trovato"),
                                };
                                pezzo.to_string().black().on_custom_color(color)
                            }
                            None => " ".on_custom_color(color),
                        }
                    )?;
                }
                print!("\n");
            }
            print!("  ");
            CHAR_SCACCHIERA.into_iter().for_each(|el| {
                print!("{:^3}", el.to_string().normal());
            });
            Ok(())
        }
    }
}
