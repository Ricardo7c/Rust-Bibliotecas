use regex::Regex;

fn is_valid_email(email: &str) -> bool{
    let padrao = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if padrao.is_match(email){
        return true;
    }
    false
}

fn main(){
    println!("{}", is_valid_email("email@google.com")); //true
    println!("{}", is_valid_email("google.com"));       //false
}