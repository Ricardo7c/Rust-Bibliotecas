# Praticando Regex no Rust  

---

## **01. Validação de Emails**

**Objetivo:** Verificar se uma string é um endereço de email válido.

**Instruções:**

1. Escreva uma função chamada `is_valid_email` que recebe uma string e retorna um `bool`.
2. Use regex para validar emails com o formato básico: `username@domain.com`.

**Regex sugerido:**

```regex
^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$
```

---

## **02. Capturando Datas no Formato DD/MM/AAAA**

**Objetivo:** Extrair datas de um texto.

**Instruções:**

1. Escreva uma função chamada `extract_dates` que recebe uma string e retorna um vetor de datas encontradas.
2. Use regex para capturar datas no formato `DD/MM/AAAA`.

**Regex sugerido:**

```regex
\b\d{2}/\d{2}/\d{4}\b
```

---

## **03. Remover Caracteres Não Alfanuméricos ou espaços em branco**

**Objetivo:** Limpar uma string, removendo todos os caracteres que não são letras, números ou espaços.

**Instruções:**

1. Escreva uma função chamada `clean_string` que recebe uma string e retorna a string "limpa".
2. Use regex para substituir os caracteres indesejados por uma string vazia.

**Regex sugerido:**

```regex
[^a-zA-Z0-9\s]+
```

---

## **04. Detectar Palavras que Começam com Letra Maiúscula**

**Objetivo:** Encontrar todas as palavras que começam com letras maiúsculas em um texto.

**Instruções:**

1. Escreva uma função chamada `find_capitalized_words` que recebe uma string e retorna um vetor com as palavras encontradas.
2. Use regex para capturar palavras que começam com uma letra maiúscula.

**Regex sugerido:**

```regex
\b[A-Z][a-z]*\b
```

---

## **05. Substituir Espaços por Underscores**

**Objetivo:** Transformar todos os espaços em underscores (`_`) em uma string.

**Instruções:**

1. Escreva uma função chamada `replace_spaces` que recebe uma string e retorna a string com os espaços substituídos por underscores.
2. Use regex para localizar os espaços e substituí-los.

**Regex sugerido:**

```regex
\s+
```
