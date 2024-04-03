use std::io::{self, Read, stdout, Write};
use std::process::Command;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use chrono::{Utc, TimeZone, Local};
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;
//use crate::Estado::Concluida;

#[derive(Serialize, Deserialize)]
enum Estado {
    NaoIniciada,
    EmAndamento,
    Concluida,
}

#[derive(Serialize, Deserialize)]
pub struct Tarefa {
    data: String,
    descricao: String,
    estado: Estado,
}

impl Tarefa {
    fn new(descricao: String) -> Tarefa {
        Tarefa {
            data:  Local::today().format("%d/%m/%Y").to_string(),
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
          //  if (let Estado::Concluida == tarefa.estado){
            //    println!("Você não pode terminar uma tarefa já feita");
            //    return
           //}

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

    pub fn listar_tarefas(&self) {
        println!("Lista de Tarefas:");
        for (indice, tarefa) in self.tarefas.iter().enumerate() {
            let status = match tarefa.estado {
                Estado::NaoIniciada => " (Não Iniciada)",
                Estado::EmAndamento => " (Em Andamento)",
                Estado::Concluida => " (Concluída)",
            };
            println!("{} : {}: {}{}",indice + 1,tarefa.data, tarefa.descricao, status);
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

pub fn limpar_console() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/c", "cls"]).status();
    } else {
        let _ = Command::new("sh").arg("-c").arg("clear").status();
    }
}
