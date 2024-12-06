use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    id: u32,
    nome: String,
    preco: Option<f64>
}

fn main(){
    let p1 = Product{
        id: 101,
        nome: "Widget".to_string(),
        preco:None,
    };
    let serializado = serde_json::to_string(&p1).unwrap();
    println!("{}", &serializado);
    
    let deserializado: Product = serde_json::from_str(&serializado).unwrap();
    println!("{:?}", deserializado);
}