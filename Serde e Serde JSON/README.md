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

### **04. Alterar a Serialização com Atributos**

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

### **05. Trabalhar com Vetores e JSON**

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
