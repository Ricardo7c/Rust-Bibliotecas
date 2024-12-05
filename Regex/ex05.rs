use regex::Regex;

fn replace_spaces(texto: &str) -> String{
    let padrao = Regex::new(r"\s+").unwrap();
    padrao.replace_all(texto, "_").to_string()
}

fn main(){
    let texto = "Rust Lang 2024";
    let novo = replace_spaces(texto);
    println!("{}", novo);
}