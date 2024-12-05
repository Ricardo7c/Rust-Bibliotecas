use regex::Regex;

fn find_capitalized_words(text: &str) -> Vec<String>{
    let padrao = Regex::new(r"\b[A-Z][a-z]*\b").unwrap();
    padrao.find_iter(text).map(|x| x.as_str().to_string()).collect()
}

fn main(){
    let text = "Rust é a melhor Linguagem da Atualidade";
    println!("{:?}", find_capitalized_words(text));
}