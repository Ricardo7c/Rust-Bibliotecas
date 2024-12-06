# **`text-input`**  

É uma crate criada por min, para facilitar a captura de entrada de texto interativa no terminal. Ela permite que você exiba uma mensagem ao usuário e capture sua resposta de forma síncrona, retornando o texto inserido como uma `String` (com espaços e quebras de linha removidos). Essa biblioteca é ideal para aplicativos de linha de comando em Rust que precisam de entradas diretas e limpas.

## Exemplo de uso

```rust
use text_input::text;

fn main() {
    let name = text("What's your name? ");
    print!("Hi, {}", name);
}
```

**Saída no terminal:**

```powershell
What's your name? Rust
Hi, Rust
```
