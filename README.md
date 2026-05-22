# 🧮 Calculadora Avançada CLI (Rust)

Uma calculadora de linha de comando segura e rápida construída em Rust, capaz de avaliar expressões matemáticas com precedência correta de operadores.

## 🚀 Funcionalidades

- Avaliação de expressões matemáticas (ex: `2 + 3 * 4`)
- Suporte a operadores básicos: `+`, `-`, `*`, `/`
- Precedência correta (multiplicação/divisão antes de adição/subtração)
- Histórico de cálculos da sessão
- Comandos interativos (`h` para histórico, `c` para limpar, `q` para sair)
- Tratamento seguro de erros (ex: divisão por zero, sintaxe inválida)

## 🛠️ Tecnologias

- Rust (Edição 2021)
- Cargo (Gerenciador de pacotes e build system)

## 📦 Como executar

1. Certifique-se de ter o Rust instalado (`rustup`)
2. Clone o repositório
3. Navegue até o diretório do projeto:
   ```bash
   cd calculator-rust
   ```
4. Compile e execute:
   ```bash
   cargo run
   ```

## 🧠 Arquitetura

O projeto implementa um avaliador de expressões (parser) simples que:
1. Tokeniza a string de entrada em números e operadores
2. Avalia a lista de tokens respeitando a precedência
3. Retorna o resultado (`f64`) ou um erro (`String`) seguro

## 📄 Licença

Este projeto está sob a licença MIT.
