5 exercícios simples focados em regex para praticar.  

Certifique-se de adicionar a dependência `regex` no arquivo `Cargo.toml` antes de começar:

```toml
[dependencies]
regex = "1"
```

---

### **Exercício 1: Validação de Emails**

**Objetivo:** Verificar se uma string é um endereço de email válido.

**Instruções:**

1. Escreva uma função chamada `is_valid_email` que recebe uma string e retorna um `bool`.
2. Use regex para validar emails com o formato básico: `username@domain.com`.

**Regex sugerido:**

```regex
^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$
```

**Exemplo de entrada e saída:**

```rust
assert!(is_valid_email("teste@example.com")); // true
assert!(!is_valid_email("teste.com"));       // false
```

---

### **Exercício 2: Capturando Datas no Formato DD/MM/AAAA**

**Objetivo:** Extrair datas de um texto.

**Instruções:**

1. Escreva uma função chamada `extract_dates` que recebe uma string e retorna um vetor de datas encontradas.
2. Use regex para capturar datas no formato `DD/MM/AAAA`.

**Regex sugerido:**

```regex
\b\d{2}/\d{2}/\d{4}\b
```

**Exemplo de entrada e saída:**

```rust
let text = "Hoje é 04/12/2024, amanhã será 05/12/2024.";
let dates = extract_dates(text);
assert_eq!(dates, vec!["04/12/2024", "05/12/2024"]);
```

---

### **Exercício 3: Remover Caracteres Não Alfanuméricos**

**Objetivo:** Limpar uma string, removendo todos os caracteres que não são letras ou números.

**Instruções:**

1. Escreva uma função chamada `clean_string` que recebe uma string e retorna a string "limpa".
2. Use regex para substituir os caracteres indesejados por uma string vazia.

**Regex sugerido:**

```regex
[^a-zA-Z0-9]+
```

**Exemplo de entrada e saída:**

```rust
let cleaned = clean_string("Hello, World! 2024...");
assert_eq!(cleaned, "HelloWorld2024");
```

---

### **Exercício 4: Detectar Palavras que Começam com Letra Maiúscula**

**Objetivo:** Encontrar todas as palavras que começam com letras maiúsculas em um texto.

**Instruções:**

1. Escreva uma função chamada `find_capitalized_words` que recebe uma string e retorna um vetor com as palavras encontradas.
2. Use regex para capturar palavras que começam com uma letra maiúscula.

**Regex sugerido:**

```regex
\b[A-Z][a-z]*\b
```

**Exemplo de entrada e saída:**

```rust
let text = "Alice and Bob went to the Park.";
let words = find_capitalized_words(text);
assert_eq!(words, vec!["Alice", "Bob", "Park"]);
```

---

### **Exercício 5: Substituir Espaços por Underscores**

**Objetivo:** Transformar todos os espaços em underscores (`_`) em uma string.

**Instruções:**

1. Escreva uma função chamada `replace_spaces` que recebe uma string e retorna a string com os espaços substituídos por underscores.
2. Use regex para localizar os espaços e substituí-los.

**Regex sugerido:**

```regex
\s+
```

**Exemplo de entrada e saída:**

```rust
let result = replace_spaces("Hello World! How are you?");
assert_eq!(result, "Hello_World!_How_are_you?");
```

---
