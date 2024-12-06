use std::fs::File;
use csv::{Error, Writer};
use serde::Serialize;

#[derive(Serialize)]
struct Person {
    id: u32,
    name: String,
    age: u32
}


fn gravar_csv(arquivo: &mut Writer<File>, pessoa: Person) -> Result<String, Error>{
    arquivo.serialize(pessoa)?;
    arquivo.flush()?;
    Ok("Arquivo gravado com sucesso!".to_string())
}


fn main(){
    let mut arquivo = Writer::from_path("data.csv").unwrap();

    let lista = vec![
        Person {id: 1, name: "Alice".to_string(), age: 30},
        Person {id: 2, name: "Bob".to_string(), age: 25},
        Person {id: 3, name: "Charlie".to_string(), age: 35}
    ];


    for cada in lista{
        match gravar_csv(&mut arquivo, cada) {
            Ok(valor) => println!("{}", valor),
            Err(err) => eprintln!("Erro ao ler registro: {}", err),
        }
    }

}