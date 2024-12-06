use text_input::text;

fn main() {
    let name = text("Qual seu nome? ");
    print!("Oi, {}", name);
}
