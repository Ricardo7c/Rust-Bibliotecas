use csv::Reader;

fn main(){
    let mut arquivo = Reader::from_path("data.csv").unwrap();
    for cada in arquivo.records(){
        match cada {
            Ok(record) => println!("{:?}", record),
            Err(err) => eprintln!("Erro ao ler registro: {}", err),
        }
    }
}