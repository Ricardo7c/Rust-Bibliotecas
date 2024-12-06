# `serde` e `serde_json` no Rust  

---

## **01. Serializar uma Estrutura**

**Objetivo:** Criar uma estrutura em Rust e convertê-la em uma string JSON.  

**Instruções:**

1. Crie uma estrutura chamada `User` com os campos:
   - `id: u32`
   - `nome: String`
   - `email: String`
2. Use `serde` para habilitar a serialização da estrutura.
3. Converta uma instância de `User` em JSON e imprima o resultado.

**Requisitos:**

- Adicionar as crates `serde` e `serde_json` ao projeto.
- Implementar o trait `Serialize` com `#[derive(Serialize)]`.

**Exemplo de saída esperada:**

```json
{"id":1,"nome":"Alice","email":"alice@example.com"}
```

---

## **02. Desserializar uma String JSON**

**Objetivo:** Ler uma string JSON e convertê-la em uma instância de uma estrutura.  

**Instruções:**

1. Use a estrutura `User` do exercício anterior.
2. Crie uma string JSON no formato:

   ```json
   {"id":2,"nome":"Bob","email":"bob@example.com"}
   ```

3. Use `serde_json` para convertê-la em uma instância de `User`.
4. Imprima os valores dos campos da estrutura.

**Requisitos:**

- Implementar o trait `Deserialize` com `#[derive(Deserialize)]`.

**Exemplo de saída esperada:**

```plaintext
ID: 2
Name: Bob
Email: bob@example.com
```

---

## **03. Serialização e Desserialização com Campos Opcionais**

**Objetivo:** Trabalhar com campos opcionais na serialização e desserialização.  

**Instruções:**

1. Crie uma estrutura chamada `Product` com os campos:
   - `id: u32`
   - `name: String`
   - `price: Option<f64>`
2. Crie uma instância de `Product` com o campo `price` como `None`.
3. Serialize a estrutura para JSON e imprima o resultado.
4. Desserialize a string JSON de volta em uma instância da estrutura.

**Requisitos:**

- Usar o tipo `Option`.

**Exemplo de saída esperada:**

```json
{"id":101,"name":"Widget"}
```

---

## **04. Alterar a Serialização com Atributos**

**Objetivo:** Usar atributos do `serde` para personalizar o formato do JSON.  

**Instruções:**

1. Crie uma estrutura `Person` com os campos:
   - `first_name: String`
   - `last_name: String`
   - `age: u32`
2. Personalize o nome dos campos no JSON para:
   - `first_name` → `firstName`
   - `last_name` → `lastName`
3. Crie uma instância e serialize para JSON.

**Requisitos:**

- Usar o atributo `#[serde(rename = "...")]`.

**Exemplo de saída esperada:**

```json
{"firstName":"John","lastName":"Doe","age":30}
```

---

## **05. Trabalhar com Vetores e JSON**

**Objetivo:** Serializar e desserializar vetores usando `serde_json`.  

**Instruções:**

1. Crie um vetor de estruturas `User` (reutilize a do Exercício 1).
2. Serialize o vetor em uma string JSON e imprima o resultado.
3. Desserialize a string JSON de volta em um vetor e itere sobre ele, imprimindo os campos de cada usuário.

**Requisitos:**

- Trabalhar com coleções em Rust.

**Exemplo de saída esperada:**

```json
[{"id":1,"name":"Alice","email":"alice@example.com"},{"id":2,"name":"Bob","email":"bob@example.com"}]
```

```plaintext
User: Alice (alice@example.com)
User: Bob (bob@example.com)
```

---

## **06. Escrever um CSV com Dados**

**Objetivo**: Criar e escrever dados em um arquivo CSV.

**Instruções**:

1. Crie uma estrutura chamada `Person` com os campos:
   - `id: u32`
   - `name: String`
   - `age: u32`
2. Crie um vetor com 3 instâncias de `Person` e grave os dados em um arquivo chamado `data.csv`.

**Requisitos**:

- Use `serde::Serialize` para facilitar a serialização.

**Exemplo de saída no arquivo `data.csv`**:

```poweshell
id,name,age
1,Alice,30
2,Bob,25
3,Charlie,35
```

---

## **07. Leitura Básica de CSV**

**Objetivo**: Aprender a ler um arquivo CSV e exibir seu conteúdo no terminal.

**Instruções**:

1. Use a crate `csv` para ler o arquivo linha por linha.
2. Imprima cada registro no terminal.

**Requisitos**:

- Ler e exibir os registros como vetores de strings.

**Exemplo de saída**:

```poweshell
["1", "Alice", "30"]
["2", "Bob", "25"]
["3", "Charlie", "35"]
```

---

## **08. Filtrar Dados de um CSV**

**Objetivo**: Filtrar e exibir apenas registros que atendam a uma condição.

**Instruções**:

1. Use o mesmo arquivo `data.csv` do exercício anterior.
2. Leia o arquivo e exiba apenas os registros onde a idade (`age`) é maior que 30.

**Requisitos**:

- Converta a coluna `age` de string para inteiro.
- Trate possíveis erros de conversão.

**Exemplo de saída**:

```poweshell
["3", "Charlie", "35"]
```

---

## **09. Conversão de CSV para JSON**

**Objetivo**: Ler dados de um arquivo CSV e exportá-los para um arquivo JSON.

**Instruções**:

1. Crie um arquivo CSV chamado `data.csv` com o seguinte conteúdo:

   ```powershell
   id,name,age
   1,Alice,30
   2,Bob,25
   3,Charlie,35
   ```

2. Leia o CSV e converta os dados para uma estrutura chamada `Person`, definida como:

   ```rust
   #[derive(Debug, serde::Deserialize, serde::Serialize)]
   struct Person {
       id: u32,
       name: String,
       age: u32,
   }
   ```

3. Escreva os dados convertidos em um arquivo JSON chamado `from_csv.json`.

**Requisitos**:

- Use as crates `serde`, `serde_json` e `csv`.
- O arquivo JSON gerado deve ser formatado como o exemplo abaixo:

```json
[
    {"id": 1, "name": "Alice", "age": 30},
    {"id": 2, "name": "Bob", "age": 25},
    {"id": 3, "name": "Charlie", "age": 35}
]
```

**Passos sugeridos**:

- Use o `csv::Reader` para ler o arquivo CSV e desserializar os registros para a estrutura `Person`.
- Use o `serde_json::to_writer_pretty` para gravar os dados em formato JSON.

---

## **10. Estatísticas de um Arquivo CSV**

**Objetivo**: Calcular estatísticas simples a partir de dados CSV.

**Instruções**:

1. Leia os dados e calcule:
   - A media das idades das pessoas armazenas em `data.csv`.

**Requisitos**:

- Converta a coluna `age` de string para `f32`.
- Imprima os resultados no terminal.

**Exemplo de saída**:

```poweshell
A media das idades é: 30 anos
```

---
