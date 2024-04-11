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
    Voltar,
    Sair
}


pub fn atribuir_entrada_enum(){

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


        match tratar_input_string() {
            Ok(entrada) => {
                match entrada.as_str() {
                    "1" => {
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
                    "2" => {
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
                    "3" => {
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
                    "4" => {
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
                    "5" => {
                        lista_de_tarefas.listar_tarefas(None, None);
                        trigger_continue();
                    }
                    "7" => {
                        break;
                    }
                    _ => {
                        println!("Escolha inválida");
                        trigger_continue();
                    }
                }
            }
            Err(err) => {
                println!("Erro de entrada: {}", err);
            }
        }
    }
}




























pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn teste_continue(){

    }

}

