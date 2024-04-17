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
    pub estado: Estado,
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
    pub tarefas: Vec<Tarefa>,
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
            if tarefa.estado == Estado::NaoIniciada {
                tarefa.estado = Estado::EmAndamento;
            self.salvar_em_json("tarefas.json").unwrap_or_else(|err| {
                eprintln!("Erro ao salvar as tarefas: {}", err);
            });
            } else{
                println!("Você escolheu uma inválida!")
            }
        } else {
            println!("Índice inválido!");
        }
    }
    pub fn completar_tarefa(&mut self, indice: usize) {
        if let Some(tarefa) = self.tarefas.get_mut(indice) {
            if tarefa.estado != Estado::Concluida{
            tarefa.estado = Estado::Concluida;
            }else{
                println!("Esta tarefa já estava concluída, inválido!")
            }
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
        println!("Lista de Tarefas:");

        // Filtra as tarefas
        let filtered_tasks = self.tarefas.iter().filter(|tarefa| {
            if let Some(ref estado1) = estado1 {
                if tarefa.estado != *estado1 {return false;}
            }

            if let Some(ref estado2) = estado2 {
                if tarefa.estado != *estado2 {return false;}
            }
            true
        });

        // Imprimindo das filtradas
        for (indice, tarefa) in filtered_tasks.enumerate() {
            let status = match tarefa.estado {
                Estado::NaoIniciada => " (Não Iniciada)",
                Estado::EmAndamento => " (Em Andamento)",
                Estado::Concluida => " (Concluída)",
            };
            println!("{} : {}: {}{}", indice + 1, tarefa.data, tarefa.descricao, status);
        }
    }

    pub fn rollback_tarefa(&mut self, indice: usize){
        if let Some(tarefa) = self.tarefas.get_mut(indice) {
            if tarefa.estado != Estado::NaoIniciada{
                tarefa.estado = Estado::NaoIniciada;
            }else{
                println!("Esta tarefa já estava sem ser iniciada, inválido!")
            }
            self.salvar_em_json("tarefas.json").unwrap_or_else(|err| {
                eprintln!("Erro ao salvar as tarefas: {}", err);
            });
        } else {
            println!("Índice inválido!");
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










