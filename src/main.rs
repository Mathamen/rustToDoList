use std::io;
use listadetarefas::*;
//teste
fn tratar_input_string() -> Result<String, io::Error> {
    let mut string = String::new();
    io::stdin().read_line(&mut string)?;
    let string = string.trim().to_string();
    if string.is_empty() {
        Err(io::Error::new(io::ErrorKind::InvalidData, "String vazia"))
    } else {
        Ok(string)
    }
}

fn trigger_continue() {
    let mut entrada = String::new();
    println!("Pressione Enter para continuar...");
    let _ = io::stdin().read_line(&mut entrada);
}

fn tratar_input_int() -> Result<usize, io::Error> {
    tratar_input_string()?
        .parse::<usize>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn main() {
    let mut lista_de_tarefas = ListaDeTarefas::new();

    // Carregar tarefas de um arquivo JSON, se existir
    if let Err(erro) = lista_de_tarefas.carregar_de_json("tarefas.json") {
        eprintln!("Erro ao carregar as tarefas: {}", erro);
    }

    loop {
        limpar_console();
        println!("Escolha uma ação:");
        println!("1. Adicionar tarefa");
        println!("2. Iniciar tarefa");
        println!("3. Completar tarefa");
        println!("4. Remover tarefa");
        println!("5. Listar tarefas");
        println!("6. Sair");

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
                        lista_de_tarefas.listar_tarefas();
                        match tratar_input_int() {
                            Ok(indice) => {
                                lista_de_tarefas.iniciar_tarefa(indice -1);
                            }
                            Err(err) => {
                                println!("Erro ao ler o índice: {}", err);
                            }
                        }
                    }
                    "3" => {
                        println!("Escolha a tarefa a ser marcada como concluída, pelo índice:");
                        lista_de_tarefas.listar_tarefas();
                        match tratar_input_int() {
                            Ok(indice) => {
                                lista_de_tarefas.completar_tarefa(indice-1);
                            }
                            Err(err) => {
                                println!("Erro ao ler o índice: {}", err);
                            }
                        }
                    }
                    "4" => {
                        println!("Escolha a tarefa a ser removida, pelo índice:");
                        lista_de_tarefas.listar_tarefas();
                        match tratar_input_int() {
                            Ok(indice) => {
                                lista_de_tarefas.remover_tarefa(indice-1);
                            }
                            Err(err) => {
                                println!("Erro ao ler o índice: {}", err);
                            }
                        }
                    }
                    "5" => {
                        lista_de_tarefas.listar_tarefas();
                        trigger_continue();
                    }
                    "6" => {
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
