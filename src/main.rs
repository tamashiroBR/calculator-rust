use std::io::{self, Write};

struct Calculator {
    history: Vec<String>,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }

    fn parse_expression(&self, expr: &str) -> Result<f64, String> {
        let expr = expr.trim().replace(" ", "");

        // Validação básica
        if expr.is_empty() {
            return Err("Expressão vazia".to_string());
        }

        // Suporta operações simples: +, -, *, /
        self.evaluate(&expr)
    }

    fn evaluate(&self, expr: &str) -> Result<f64, String> {
        // Tokenizar a expressão
        let mut tokens = Vec::new();
        let mut current_num = String::new();

        for ch in expr.chars() {
            match ch {
                '+' | '-' | '*' | '/' => {
                    if !current_num.is_empty() {
                        tokens.push(Token::Number(current_num.parse::<f64>()
                            .map_err(|_| "Número inválido".to_string())?));
                        current_num.clear();
                    }
                    tokens.push(Token::Operator(ch));
                }
                '0'..='9' | '.' => {
                    current_num.push(ch);
                }
                _ => return Err("Caractere inválido".to_string()),
            }
        }

        if !current_num.is_empty() {
            tokens.push(Token::Number(current_num.parse::<f64>()
                .map_err(|_| "Número inválido".to_string())?));
        }

        if tokens.is_empty() {
            return Err("Nenhum número encontrado".to_string());
        }

        // Avaliar com precedência
        self.evaluate_tokens(tokens)
    }

    fn evaluate_tokens(&self, mut tokens: Vec<Token>) -> Result<f64, String> {
        // Primeiro, processar * e /
        let mut i = 0;
        while i < tokens.len() {
            if let Token::Operator(op) = tokens.get(i) {
                if *op == '*' || *op == '/' {
                    if i == 0 || i == tokens.len() - 1 {
                        return Err("Operador em posição inválida".to_string());
                    }

                    let left = match tokens.get(i - 1) {
                        Some(Token::Number(n)) => *n,
                        _ => return Err("Operando inválido".to_string()),
                    };

                    let right = match tokens.get(i + 1) {
                        Some(Token::Number(n)) => *n,
                        _ => return Err("Operando inválido".to_string()),
                    };

                    let result = match *op {
                        '*' => left * right,
                        '/' => {
                            if right == 0.0 {
                                return Err("Divisão por zero".to_string());
                            }
                            left / right
                        }
                        _ => unreachable!(),
                    };

                    tokens.remove(i + 1);
                    tokens.remove(i);
                    tokens[i - 1] = Token::Number(result);
                    i = i.saturating_sub(1);
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        // Depois, processar + e -
        let mut result = match tokens.get(0) {
            Some(Token::Number(n)) => *n,
            _ => return Err("Expressão inválida".to_string()),
        };

        let mut i = 1;
        while i < tokens.len() {
            let op = match tokens.get(i) {
                Some(Token::Operator(o)) => *o,
                _ => return Err("Expressão inválida".to_string()),
            };

            let num = match tokens.get(i + 1) {
                Some(Token::Number(n)) => *n,
                _ => return Err("Expressão inválida".to_string()),
            };

            result = match op {
                '+' => result + num,
                '-' => result - num,
                _ => return Err("Operador inválido".to_string()),
            };

            i += 2;
        }

        Ok(result)
    }

    fn add_to_history(&mut self, expr: &str, result: f64) {
        self.history.push(format!("{} = {}", expr, result));
    }

    fn show_history(&self) {
        if self.history.is_empty() {
            println!("Histórico vazio.");
            return;
        }

        println!("\n{}", "=".repeat(40));
        println!("HISTÓRICO");
        println!("{}", "=".repeat(40));
        for (i, entry) in self.history.iter().enumerate() {
            println!("{}. {}", i + 1, entry);
        }
        println!("{}\n", "=".repeat(40));
    }

    fn show_menu(&self) {
        println!("\n{}", "=".repeat(40));
        println!("CALCULADORA AVANÇADA");
        println!("{}", "=".repeat(40));
        println!("Digite uma expressão (ex: 2 + 3 * 4)");
        println!("Comandos especiais:");
        println!("  'h' - Ver histórico");
        println!("  'c' - Limpar histórico");
        println!("  'q' - Sair");
        println!("{}", "=".repeat(40));
    }
}

enum Token {
    Number(f64),
    Operator(char),
}

fn main() {
    let mut calculator = Calculator::new();

    println!("🧮 Bem-vindo à Calculadora Avançada!");
    println!("Suporta: +, -, *, / com precedência correta\n");

    loop {
        calculator.show_menu();
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler entrada");

        let input = input.trim();

        match input {
            "q" => {
                println!("Até logo!");
                break;
            }
            "h" => {
                calculator.show_history();
            }
            "c" => {
                calculator.history.clear();
                println!("✓ Histórico limpo!");
            }
            "" => continue,
            _ => {
                match calculator.parse_expression(input) {
                    Ok(result) => {
                        println!("✓ Resultado: {}", result);
                        calculator.add_to_history(input, result);
                    }
                    Err(e) => {
                        println!("✗ Erro: {}", e);
                    }
                }
            }
        }
    }
}
