use std::io::{self, Write};

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Operator(char),
}

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
        let expr = expr.trim().replace(' ', "");

        if expr.is_empty() {
            return Err("Expressão vazia".to_string());
        }

        self.evaluate(&expr)
    }

    fn evaluate(&self, expr: &str) -> Result<f64, String> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut current_num = String::new();

        for ch in expr.chars() {
            match ch {
                '+' | '-' | '*' | '/' => {
                    if !current_num.is_empty() {
                        tokens.push(Token::Number(
                            current_num
                                .parse::<f64>()
                                .map_err(|_| "Número inválido".to_string())?,
                        ));
                        current_num.clear();
                    }
                    tokens.push(Token::Operator(ch));
                }
                '0'..='9' | '.' => {
                    current_num.push(ch);
                }
                _ => return Err(format!("Caractere inválido: '{}'", ch)),
            }
        }

        if !current_num.is_empty() {
            tokens.push(Token::Number(
                current_num
                    .parse::<f64>()
                    .map_err(|_| "Número inválido".to_string())?,
            ));
        }

        if tokens.is_empty() {
            return Err("Nenhum número encontrado".to_string());
        }

        self.evaluate_tokens(tokens)
    }

    fn evaluate_tokens(&self, mut tokens: Vec<Token>) -> Result<f64, String> {
        // Primeira passagem: processar * e / (maior precedência)
        let mut i = 0;
        while i < tokens.len() {
            // Verifica se o token na posição i é um operador * ou /
            let is_mul_div = match &tokens[i] {
                Token::Operator(op) => *op == '*' || *op == '/',
                _ => false,
            };

            if is_mul_div {
                if i == 0 || i + 1 >= tokens.len() {
                    return Err("Operador em posição inválida".to_string());
                }

                let left = match &tokens[i - 1] {
                    Token::Number(n) => *n,
                    _ => return Err("Operando esquerdo inválido".to_string()),
                };

                let right = match &tokens[i + 1] {
                    Token::Number(n) => *n,
                    _ => return Err("Operando direito inválido".to_string()),
                };

                let op = match &tokens[i] {
                    Token::Operator(o) => *o,
                    _ => unreachable!(),
                };

                let result = match op {
                    '*' => left * right,
                    '/' => {
                        if right == 0.0 {
                            return Err("Divisão por zero".to_string());
                        }
                        left / right
                    }
                    _ => unreachable!(),
                };

                // Remove os 3 tokens (left, op, right) e substitui pelo resultado
                tokens.remove(i + 1); // remove right
                tokens.remove(i);     // remove op
                tokens[i - 1] = Token::Number(result); // substitui left pelo resultado
                i = i.saturating_sub(1);
            } else {
                i += 1;
            }
        }

        // Segunda passagem: processar + e - (menor precedência)
        let mut result = match &tokens[0] {
            Token::Number(n) => *n,
            _ => return Err("Expressão inválida: esperado número no início".to_string()),
        };

        let mut i = 1;
        while i < tokens.len() {
            let op = match &tokens[i] {
                Token::Operator(o) => *o,
                _ => return Err("Expressão inválida: esperado operador".to_string()),
            };

            if i + 1 >= tokens.len() {
                return Err("Expressão incompleta: falta operando após operador".to_string());
            }

            let num = match &tokens[i + 1] {
                Token::Number(n) => *n,
                _ => return Err("Expressão inválida: esperado número após operador".to_string()),
            };

            result = match op {
                '+' => result + num,
                '-' => result - num,
                _ => return Err(format!("Operador inesperado: '{}'", op)),
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
        println!("   CALCULADORA AVANÇADA - Rust");
        println!("{}", "=".repeat(40));
        println!("Digite uma expressão (ex: 2 + 3 * 4)");
        println!("Comandos:");
        println!("  h  → Ver histórico");
        println!("  c  → Limpar histórico");
        println!("  q  → Sair");
        println!("{}", "=".repeat(40));
    }
}

fn main() {
    let mut calculator = Calculator::new();

    println!("🧮 Bem-vindo à Calculadora em Rust!");
    println!("Suporta: +, -, *, / com precedência correta (ex: 2+3*4 = 14)\n");

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
                println!("Até logo! 👋");
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
            _ => match calculator.parse_expression(input) {
                Ok(result) => {
                    println!("✓ Resultado: {}", result);
                    calculator.add_to_history(input, result);
                }
                Err(e) => {
                    println!("✗ Erro: {}", e);
                }
            },
        }
    }
}
