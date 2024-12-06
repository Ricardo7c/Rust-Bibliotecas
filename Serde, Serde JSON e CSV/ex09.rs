use std::fs::File;
use serde::{Deserialize, Serialize};
use csv::ReaderBuilder;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    id: u32,
    name: String,
    age: u32,
}

fn main() {
    // Abre o arquivo.
    let arquivo = File::open("data.csv").unwrap();

    // Armazena o conteudo.
    let mut conteudo = ReaderBuilder::new().has_headers(true).from_reader(arquivo);

    // Crai um vetor vazio para receber as instancias de pessoa.
    let mut persons: Vec<Person> = Vec::new();

    // Pega cada registro em conteudo, gera uma instancia de pessoa e adiciona ao vetor
    for registro in conteudo.deserialize() {
        let person: Person = registro.unwrap();
        persons.push(person);
    }

    // Cria ou abre o arquivo Json para escrita.
    let json_file = File::create("from_csv.json").unwrap();

    // Grava os dados do vetor formatado no arquivo json
    serde_json::to_writer_pretty(json_file, &persons).unwrap();

    // Exibi uma msg ao final do processo
    println!("Dados exportados para 'from_csv.json' com sucesso!");

}