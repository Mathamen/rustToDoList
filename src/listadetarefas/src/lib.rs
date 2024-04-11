use std::io;
use serde::{Serialize, Deserialize};
use std::fs::File;
use chrono::{Datelike, Local};




#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Estado {
    NaoIniciada,
    EmAndamento,
    Concluida,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Tarefa {
    data: String,
    descricao: String,
    estado: Estado,
}

impl Tarefa {
    fn new(descricao: String) -> Tarefa {
        let full = Local::now();
        let date = format!("{}/{}/{}", full.day(), full.month(), full.year());
        Tarefa {
            data: date,
            descricao,
            estado: Estado::NaoIniciada,
        }
    }
}

pub struct ListaDeTarefas {
    tarefas: Vec<Tarefa>,
}

impl ListaDeTarefas {
    pub fn new() -> ListaDeTarefas {
        ListaDeTarefas { tarefas: Vec::new() }
    }

    pub fn adicionar_tarefa(&mut self, descricao: String) {
        let new_tarefa = Tarefa::new(descricao);
        self.tarefas.push(new_tarefa);
        self.salvar_em_json("tarefas.json").unwrap_or_else(|err| {
            eprintln!("Erro ao salvar as tarefas: {}", err);
        });
    }

    pub fn iniciar_tarefa(&mut self, indice: usize) {
        if let Some(tarefa) = self.tarefas.get_mut(indice) {
            tarefa.estado = Estado::EmAndamento;
            self.salvar_em_json("tarefas.json").unwrap_or_else(|err| {
                eprintln!("Erro ao salvar as tarefas: {}", err);
            });
        } else {
            println!("Índice inválido!");
        }
    }

    pub fn completar_tarefa(&mut self, indice: usize) {
        if let Some(tarefa) = self.tarefas.get_mut(indice) {
            // colocar aqui a lógica para travar de mudar uma não listada

            tarefa.estado = Estado::Concluida;
            self.salvar_em_json("tarefas.json").unwrap_or_else(|err| {
                eprintln!("Erro ao salvar as tarefas: {}", err);
            });
        } else {
            println!("Índice inválido!");
        }
    }

    pub fn remover_tarefa(&mut self, indice: usize) {
        if indice < self.tarefas.len() {
            self.tarefas.remove(indice);
            self.salvar_em_json("tarefas.json").unwrap_or_else(|err| {
                eprintln!("Erro ao salvar as tarefas: {}", err);
            });
        } else {
            println!("Índice inválido!");
        }
    }

    pub fn listar_tarefas(&self, estado1: Option<Estado>, estado2: Option<Estado>) {
        // Check if both options are None
        if estado1.is_none() && estado2.is_none() {
            println!("Lista de Tarefas:");
            for (indice, tarefa) in self.tarefas.iter().enumerate() {
                let status = match tarefa.estado {
                    Estado::NaoIniciada => " (Não Iniciada)",
                    Estado::EmAndamento => " (Em Andamento)",
                    Estado::Concluida => " (Concluída)",
                };
                println!("{} : {}: {}{}", indice + 1, tarefa.data, tarefa.descricao, status);
            }
        }
        // Check if estado1 is Some
        if let Some(estado1) = estado1 {
            // Check if estado2 is None
            if estado2.is_none() {
                for (indice, tarefa) in self.tarefas.iter().enumerate() {
                    if tarefa.estado == estado1 {
                        let status = match tarefa.estado {
                            Estado::NaoIniciada => " (Não Iniciada)",
                            Estado::EmAndamento => " (Em Andamento)",
                            Estado::Concluida => " (Concluída)",
                        };
                        println!("{} : {}: {}{}", indice + 1, tarefa.data, tarefa.descricao, status);
                    }
                }
            } else {
                // Both estado1 and estado2 are Some, borrow the value of estado2
                let estado2 = estado2.as_ref().unwrap();
                for (indice, tarefa) in self.tarefas.iter().enumerate() {
                    if tarefa.estado == estado1 || tarefa.estado == *estado2 {
                        let status = match tarefa.estado {
                            Estado::NaoIniciada => " (Não Iniciada)",
                            Estado::EmAndamento => " (Em Andamento)",
                            Estado::Concluida => " (Concluída)",
                        };
                        println!("{} : {}: {}{}", indice + 1, tarefa.data, tarefa.descricao, status);
                    }
                }
            }
        }
    }



    pub fn salvar_em_json(&self, nome_arquivo: &str) -> io::Result<()> {
        let arquivo = File::create(nome_arquivo)?;
        serde_json::to_writer(arquivo, &self.tarefas)?;
        Ok(())
    }

    pub fn carregar_de_json(&mut self, nome_arquivo: &str) -> io::Result<()> {
        let arquivo = File::open(nome_arquivo)?;
        let tarefas: Vec<Tarefa> = serde_json::from_reader(arquivo)?;
        self.tarefas = tarefas;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adicionar_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"));
        assert_eq!(lista.tarefas.len(), 1);
    }

    #[test]
    fn test_iniciar_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"));
        lista.iniciar_tarefa(0);
        assert_eq!(lista.tarefas[0].estado, Estado::EmAndamento);
    }

    #[test]
    fn test_completar_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"));
        lista.completar_tarefa(0);
        assert_eq!(lista.tarefas[0].estado, Estado::Concluida);
    }

    #[test]
    fn test_remover_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"));
        lista.remover_tarefa(0);
        assert_eq!(lista.tarefas.len(), 0);
    }

    #[test]
    fn test_carregar_salvar_json() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"));
        lista.salvar_em_json("test.json").unwrap();
        let mut lista2 = ListaDeTarefas::new();
        lista2.carregar_de_json("test.json").unwrap();
        assert_eq!(lista.tarefas, lista2.tarefas);
    }

    #[test]
    fn test_listar_tarefas() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Tarefa 1"));
        lista.adicionar_tarefa(String::from("Tarefa 2"));
        lista.iniciar_tarefa(0);
        lista.completar_tarefa(1);

        // Test listing all tasks
        let mut output: Vec<String> = Vec::new();
        let result = std::panic::catch_unwind(|| {
            lista.listar_tarefas(None, None);
        });
        assert!(result.is_ok(), "Failed to list all tasks");

        // Test listing tasks by state
        let result = std::panic::catch_unwind(|| {
            lista.listar_tarefas(Some(Estado::NaoIniciada), None);
        });
        assert!(result.is_ok(), "Failed to list tasks by state (Não Iniciada)");

        let result = std::panic::catch_unwind(|| {
            lista.listar_tarefas(Some(Estado::EmAndamento), None);
        });
        assert!(result.is_ok(), "Failed to list tasks by state (Em Andamento)");

        let result = std::panic::catch_unwind(|| {
            lista.listar_tarefas(Some(Estado::Concluida), None);
        });
        assert!(result.is_ok(), "Failed to list tasks by state (Concluída)");

        // Test listing tasks by combining states
        let result = std::panic::catch_unwind(|| {
            lista.listar_tarefas(Some(Estado::NaoIniciada), Some(Estado::Concluida));
        });
        assert!(result.is_ok(), "Failed to list tasks by combining states (Não Iniciada and Concluída)");
    }

}




