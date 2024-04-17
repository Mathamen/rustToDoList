use entradasaida::limpar_console;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, BufRead};
    #[test]
    fn test_trigger_continue() {
        let input = "\n"; // Isso daqui é uma quebra de linha
        let reader = io::Cursor::new(input); // Simulação do stdin
        let mut stdin = io::BufReader::new(reader);

        // Simulando a leitura de linha
        let mut line = String::new();
        let _ = stdin.read_line(&mut line);

        // Teste se apertar enter funciona mesmo. Não tem asserteq para esse teste
    }
    #[test]
    fn test_limpar_console() {
        // Teste de pânico
        limpar_console();
    }

}