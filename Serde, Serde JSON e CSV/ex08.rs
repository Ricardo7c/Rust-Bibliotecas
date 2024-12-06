use csv::ReaderBuilder;
use std::fs::File;

fn main(){
    // Abra o arquivo data.csv
    let file = File::open("data.csv").unwrap();
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Itere sobre os registros e filtre os que têm idade maior que 30
    for result in reader.records() {
        let record = result.unwrap(); // Lê um registro do CSV

        // Tenta acessar a coluna "age" e convertê-la para um número
        if let Some(age_str) = record.get(2) {
            if let Ok(age) = age_str.parse::<u32>() {
                if age > 30 {
                    println!("{:?}", record.iter().collect::<Vec<&str>>());
                }
            } else {
                eprintln!("Erro ao converter a idade: {}", age_str);
            }
        }
    }

}
