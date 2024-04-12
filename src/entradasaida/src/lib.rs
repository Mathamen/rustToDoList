use std::fs::File;
use std::io;
use std::process::Command;
use listadetarefas::{Estado, ListaDeTarefas, Tarefa};
use chrono::Local;

pub fn tratar_input_string() -> Result<String, io::Error> {
    let mut string = String::new();
    io::stdin().read_line(&mut string)?;
    let string = string.trim().to_string();
    if string.is_empty() {
        Err(io::Error::new(io::ErrorKind::InvalidData, "String vazia"))
    } else {
        Ok(string)
    }
}

pub fn tratar_input_int() -> Result<usize, io::Error> {
    tratar_input_string()?
        .parse::<usize>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn trigger_continue() {
    let mut entrada = String::new();
    print_handler("Pressione Enter para continuar...");
    let _ = io::stdin().read_line(&mut entrada);
}



pub fn limpar_console() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/c", "cls"]).status();
    } else {
        let _ = Command::new("sh").arg("-c").arg("clear").status();
    }
}

enum Entrada{
    Adicionar,
    Iniciar,
    Completar,
    Remover,
    Listar,
    Rollback,
    Sair,
    ValorInvalido
}


pub fn atribuir_comando_enum()-> Entrada{
    match tratar_input_string() {
    Ok(entrada) => {
        match entrada.as_str() {
            "1" => {return Entrada::Adicionar; }
            "2" => { return Entrada::Iniciar; }
            "3" => { return Entrada::Completar; }
            "4" => { return Entrada::Remover; }
            "5" => { return Entrada::Listar; }
            "6" =>{ return Entrada::Rollback; }
            "7" => { return Entrada::Sair; }
            _ => { return Entrada::ValorInvalido; }
        }
    }
    Err(err) => {
        println!("Erro de entrada: {}", err);
        return Entrada::ValorInvalido;
    }
}

}


pub fn print_handler(s: &str) {
    println!("{}", s);
}

fn trim_margin(s: &str) -> String {
    s.lines()
        .map(|line| line.trim_start())
        .collect::<Vec<_>>()
        .join("\n")
}





pub fn loop_principal() {
    let mut lista_de_tarefas = ListaDeTarefas::new();

    // Carregar tarefas de um arquivo JSON, se existir
    if let Err(erro) = lista_de_tarefas.carregar_de_json("tarefas.json") {
        eprintln!("Erro ao carregar as tarefas: {}", erro);
    }

    loop {
        limpar_console();
        let s = "
        Escolha uma ação:
        1. Adicionar tarefa
        2. Iniciar tarefa
        3. Completar tarefa
        4. Remover tarefa
        5. Listar tarefas
        6. Voltar par estado não iniciada
        7. Sair ";
        print_handler(&trim_margin(s));
        let comando= atribuir_comando_enum();

        match comando{
            Entrada::Adicionar => {
                println!("Digite a descrição da tarefa:");
                match tratar_input_string() {
                    Ok(input) if !input.is_empty() => {
                        lista_de_tarefas.adicionar_tarefa(input);
                    }
                    _ => {
                        println!("String vazia, você será encaminhado ao menu");
                        trigger_continue();
                    }
                }

            }
            Entrada::Iniciar => {
                println!("Escolha a tarefa a ser iniciada, pelo índice:");
                lista_de_tarefas.listar_tarefas(Option::from(Estado::NaoIniciada), None);
                match tratar_input_int() {
                    Ok(indice) => {
                        lista_de_tarefas.iniciar_tarefa(indice - 1);
                    }
                    Err(err) => {
                        println!("Erro ao ler o índice: {}", err);
                    }
                }

            }
            Entrada::Completar => {
                println!("Escolha a tarefa a ser marcada como concluída, pelo índice:");
                lista_de_tarefas.listar_tarefas(Option::from(Estado::NaoIniciada), Option::from(Estado::EmAndamento));
                match tratar_input_int() {
                    Ok(indice) => {
                        lista_de_tarefas.completar_tarefa(indice - 1);
                    }
                    Err(err) => {
                        println!("Erro ao ler o índice: {}", err);
                        trigger_continue();
                    }
                }
            }
            Entrada::Remover => {
                println!("Escolha a tarefa a ser removida, pelo índice:");
                lista_de_tarefas.listar_tarefas(None, None);
                match tratar_input_int() {
                    Ok(indice) => {
                        lista_de_tarefas.remover_tarefa(indice - 1);
                    }
                    Err(err) => {
                        println!("Erro ao ler o índice: {}", err);
                    }
                }
            }
            Entrada::Listar => {
                lista_de_tarefas.listar_tarefas(None, None);
                trigger_continue();
            }
            Entrada::Rollback => {}
            Entrada::Sair => {
                break;
            }
            Entrada::ValorInvalido => {
                println!("Escolha inválida");
                trigger_continue();
            }
        }

    }
}



























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
