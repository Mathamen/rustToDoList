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
    pub descricao: String,
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
    pub fn adicionar_tarefa(&mut self, descricao: String, nome_arquivo: &str) {
        let new_tarefa = Tarefa::new(descricao);
        self.tarefas.push(new_tarefa);
        self.salvar_em_json(nome_arquivo).unwrap_or_else(|err| {
           // eprintln!("Erro ao salvar as tarefas: {}", err);
        });
    }
    pub fn iniciar_tarefa(&mut self, indice: usize, nome_arquivo: &str) {
        if let Some(tarefa) = self.tarefas.get_mut(indice) {
            if tarefa.estado == Estado::NaoIniciada {
                tarefa.estado = Estado::EmAndamento;
            self.salvar_em_json(nome_arquivo).unwrap_or_else(|err| {
               // eprintln!("Erro ao salvar as tarefas: {}", err);
            });
            } else{
                println!("Você escolheu uma inválida!")
            }
        } else {
            println!("Índice inválido!");
        }
    }
    pub fn completar_tarefa(&mut self, indice: usize, nome_arquivo: &str) {
        if let Some(tarefa) = self.tarefas.get_mut(indice) {
            if tarefa.estado != Estado::Concluida{
            tarefa.estado = Estado::Concluida;
            }else{
                println!("Esta tarefa já estava concluída, inválido!")
            }
            self.salvar_em_json(nome_arquivo).unwrap_or_else(|err| {
              //  eprintln!("Erro ao salvar as tarefas: {}", err);
            });
        } else {
            println!("Índice inválido!");
        }
    }
    pub fn remover_tarefa(&mut self, indice: usize, nome_arquivo: &str) {
        if indice < self.tarefas.len() {
            self.tarefas.remove(indice);
            self.salvar_em_json(nome_arquivo).unwrap_or_else(|err| {
               // eprintln!("Erro ao salvar as tarefas: {}", err);
            });
        } else {
            println!("Índice inválido!");
        }
    }

    pub fn editar_tarefa(&mut self, indice: usize, descricao: String, nome_arquivo: &str){
        if let Some(tarefa) = self.tarefas.get_mut(indice) {
            tarefa.descricao = descricao;
        }
        self.salvar_em_json(nome_arquivo).unwrap_or_else(|err|{});
    }



    pub fn listar_tarefas(&self, estado1: Option<Estado>, estado2: Option<Estado>) {
        println!("Lista de Tarefas:");

        for (indice, tarefa) in self.tarefas.iter().enumerate() {
            let status = match tarefa.estado {
                Estado::NaoIniciada => " (Não Iniciada)",
                Estado::EmAndamento => " (Em Andamento)",
                Estado::Concluida => " (Concluída)",
            };

            if let Some(est1) = estado1.as_ref() {
                if *est1 == tarefa.estado {
                    println!("{} : {}: {}{}", indice + 1, tarefa.data, tarefa.descricao, status);
                    continue;
                }
            }

            if let Some(est2) = estado2.as_ref() {
                if *est2 == tarefa.estado {
                    println!("{} : {}: {}{}", indice + 1, tarefa.data, tarefa.descricao, status);
                    continue;
                }
            }

            if estado1.is_none() && estado2.is_none() {
                println!("{} : {}: {}{}", indice + 1, tarefa.data, tarefa.descricao, status);
            }
        }
    }


    pub fn rollback_tarefa(&mut self, indice: usize, nome_arquivo: &str){
        if let Some(tarefa) = self.tarefas.get_mut(indice) {
            if tarefa.estado != Estado::NaoIniciada{
                tarefa.estado = Estado::NaoIniciada;
            }else{
                println!("Esta tarefa já estava sem ser iniciada, inválido!")
            }
            self.salvar_em_json(nome_arquivo).unwrap_or_else(|err| {
              //  eprintln!("Erro ao salvar as tarefas: {}", err);
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










