use std::io;
use std::io::Error;
use std::process::Command;
use listadetarefas::{Estado, ListaDeTarefas, Tarefa};

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



//testada
pub fn limpar_console() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/c", "cls"]).status();
    } else {
        let _ = Command::new("sh").arg("-c").arg("clear").status();
    }
}


#[derive(PartialEq, Debug)]
pub enum Entrada{
    Adicionar,
    Iniciar,
    Completar,
    Remover,
    Listar,
    Rollback,
    EditarTarefa,
    Sair,
    ValorInvalido
}



//testada
pub fn atribuir_comando_enum(entrada: String) -> Entrada{
        match entrada.as_str() {
            "1" => {return Entrada::Adicionar; }
            "2" => { return Entrada::Iniciar; }
            "3" => { return Entrada::Completar; }
            "4" => { return Entrada::Remover; }
            "5" => { return Entrada::Listar; }
            "6" => { return Entrada::Rollback; }
            "7" => { return Entrada::EditarTarefa; }
            "8" => {return Entrada::Sair;}
            _ =>   { return Entrada::ValorInvalido; }
        }
    }


//testada
pub fn print_handler(s: &str) {
    println!("{}", s);
}



//testada
pub fn trim_margin(s: &str) -> String {
    s.lines()
        .map(|line| line.trim_start())
        .collect::<Vec<_>>()
        .join("\n")
}



//testada
pub fn give_texto() -> &'static str {
    let s = "
        Escolha uma ação:
        1. Adicionar tarefa
        2. Iniciar tarefa
        3. Completar tarefa
        4. Remover tarefa
        5. Listar tarefas
        6. Voltar par estado não iniciada
        7. Mudar descrição de tarefa
        8. Sair ";
    return s;
}



//testada
pub fn abrir_arquivo() -> ListaDeTarefas {
    let mut lista_de_tarefas = ListaDeTarefas::new();

    // Carregar tarefas de um arquivo JSON, se existir
    if let Err(_err) = lista_de_tarefas.carregar_de_json("tarefas.json") {
        print_handler("Erro ao carregar as tarefas, um arquivo será criado");
    }

    return lista_de_tarefas;

}


pub fn adicionar_tarefa(mut lista_de_tarefas:&mut ListaDeTarefas, input: String) {
    if (input.eq("")){
        print_handler("A string está vazia. Você será retornado ao menu");
        return;
    }

    lista_de_tarefas.adicionar_tarefa(input);
    print_handler("Tarefa adicionada com sucesso!");
}











pub fn loop_principal() {
    let mut lista_de_tarefas = abrir_arquivo();

    loop {
        limpar_console();
        let s= give_texto();
        print_handler(&trim_margin(s));
        let comando;
        match tratar_input_string() {
            Ok(entrada) => {comando =atribuir_comando_enum(entrada);}
            Err(_) => {comando = Entrada::ValorInvalido;}
        }

        match comando{
            Entrada::Adicionar => {
                print_handler("Digite a descrição da tarefa:");
                match tratar_input_string() {
                    Ok(input) if !input.is_empty() => { //outra checagem de string vazia
                        adicionar_tarefa(&mut lista_de_tarefas, input);
                    }
                    _ => {} //string vazia aqui
                }

            }
            Entrada::Iniciar => {
                print_handler("Escolha a tarefa a ser iniciada, pelo índice:");
                lista_de_tarefas.listar_tarefas(Option::from(Estado::NaoIniciada), None);
                match tratar_input_int() {
                    Ok(indice) => {
                        lista_de_tarefas.iniciar_tarefa(indice - 1);
                    }
                    Err(_err) => { print_handler("Erro ao ler o índice"); }
                }

            }
            Entrada::Completar => {
                println!("Escolha a tarefa a ser marcada como concluída, pelo índice:");
                lista_de_tarefas.listar_tarefas(Option::from(Estado::NaoIniciada), Option::from(Estado::EmAndamento));
                match tratar_input_int() {
                    Ok(indice) => {
                        lista_de_tarefas.completar_tarefa(indice - 1);
                    }
                    Err(_err) => {} //erro ao ler o índice
                }
            }
            Entrada::Remover => {
                println!("Escolha a tarefa a ser removida, pelo índice:");
                lista_de_tarefas.listar_tarefas(None, None);
                match tratar_input_int() {
                    Ok(indice) => {
                        lista_de_tarefas.remover_tarefa(indice - 1);
                    }
                    Err(_) => { } //erro ao ler o índice
                }
            }

            Entrada::Listar => { lista_de_tarefas.listar_tarefas(None, None);  trigger_continue();}
            Entrada::Rollback => {
                print_handler("Escolha a tarefa, pelo índice, a ser retornada ao estado inicial");
                lista_de_tarefas.listar_tarefas(Option::from(Estado::EmAndamento),Option::from(Estado::Concluida));
                match tratar_input_int() {
                    Ok(indice) => {
                        lista_de_tarefas.rollback_tarefa(indice-1);
                    }
                    Err(_err) => {} //erro ao ler o índice
                }

            }
            Entrada::EditarTarefa => {
                println!("Escolha a tarefa a ser editada, pelo índice");
                lista_de_tarefas.listar_tarefas(None, None);
                match tratar_input_int() {
                    Ok(indice) => {
                        println!("Digite agora a nova descrição da tarefa");
                        match tratar_input_string() {
                            Ok(descricao) => {
                                lista_de_tarefas.editar_tarefa(indice - 1, descricao);
                            }
                            Err(_) => {}
                        }

                    }
                    Err(_) => { } //erro ao ler o índice
                }
            }
            Entrada::Sair => {
                break;
            }
            Entrada::ValorInvalido => {
                println!("Escolha inválida");
            }
        }

    }
}

