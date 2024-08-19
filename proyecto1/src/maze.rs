use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_maze(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("Error al abrir el archivo del laberinto");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| {
            line.expect("Error al leer la línea")
                .chars()
                .collect()
        })
        .collect()
}
