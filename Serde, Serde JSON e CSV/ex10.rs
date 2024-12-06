use csv::ReaderBuilder;
use std::fs::File;

fn main(){
    // Abra o arquivo data.csv
    let file = File::open("data.csv").unwrap();
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Itere sobre os registros e filtre os que têm idade maior que 30
    let mut soma = 0.0;
    let mut count = 0.0;
    for result in reader.records() {
        let record = result.unwrap(); // Lê um registro do CSV

        // Tenta acessar a coluna "age" e convertê-la para um número
        if let Some(age_str) = record.get(2) {
            if let Ok(age) = age_str.parse::<u32>() {
                soma += age as f32;
                count += 1 as f32;

            } else {
                eprintln!("Erro ao converter a idade: {}", age_str);
            }
        }
        
    }
    let media = soma/ count;
    println!("A media das idades é: {} anos", media);

}
