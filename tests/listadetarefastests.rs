use listadetarefas::{Estado, ListaDeTarefas};

#[cfg(test)]
mod tests {
    use std::fs;
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
        lista.iniciar_tarefa(0);
        lista.iniciar_tarefa(20);
    }

    #[test]
    fn test_completar_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"));
        assert_eq!(lista.tarefas[0].estado, Estado::NaoIniciada);
        lista.completar_tarefa(0);
        assert_eq!(lista.tarefas[0].estado, Estado::Concluida);
        lista.completar_tarefa(0);

        lista.adicionar_tarefa(String::from("Teste2"));
        lista.iniciar_tarefa(1);
        assert_eq!(lista.tarefas[1].estado, Estado::EmAndamento);
        lista.completar_tarefa(1);
        assert_eq!(lista.tarefas[1].estado, Estado::Concluida);

        lista.completar_tarefa(20);

    }

    #[test]
    fn test_remover_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.remover_tarefa(0);
        lista.adicionar_tarefa(String::from("Teste"));
        lista.remover_tarefa(0);
        assert_eq!(lista.tarefas.len(), 0);
        lista.remover_tarefa(20);
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
        lista.listar_tarefas(None, None);
        lista.adicionar_tarefa(String::from("Tarefa 1"));
        lista.adicionar_tarefa(String::from("Tarefa 2"));
        lista.adicionar_tarefa(String::from("Tarefa 3"));
        lista.iniciar_tarefa(0);
        lista.completar_tarefa(1);

        // Testando listar as tarefas
        let mut output: Vec<String> = Vec::new();
        let result = std::panic::catch_unwind(|| {
            lista.listar_tarefas(None, None);
        });
        assert!(result.is_ok(), "Failed to list all tasks");

        // Testes de listar por estado
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

        // Teste combinando dois estados
        let result = std::panic::catch_unwind(|| {
            lista.listar_tarefas(Some(Estado::NaoIniciada), Some(Estado::Concluida));
        });
        assert!(result.is_ok(), "Failed to list tasks by combining states (Não Iniciada and Concluída)");

        let result = std::panic::catch_unwind(|| {
            lista.listar_tarefas(Some(Estado::Concluida), Some(Estado::EmAndamento));
        });
        assert!(result.is_ok(), "Failed to list tasks by state (Não Iniciada e EmAndamento)");

        let result = std::panic::catch_unwind(|| {
            lista.listar_tarefas(None, Some(Estado::EmAndamento));
        });
        assert!(result.is_ok(), "Failed to list tasks by state (Não Iniciada e EmAndamento)");

        let result = std::panic::catch_unwind(|| {
            lista.listar_tarefas(None, Some(Estado::Concluida));
        });
        assert!(result.is_ok(), "Failed to list tasks by state (Não Iniciada e EmAndamento)");

        let result = std::panic::catch_unwind(|| {
            lista.listar_tarefas(None, Some(Estado::NaoIniciada));
        });
        assert!(result.is_ok(), "Failed to list tasks by state (Não Iniciada e EmAndamento)");
    }

    #[test]
    fn test_salvar_e_carregar_json() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Tarefa 1"));
        lista.adicionar_tarefa(String::from("Tarefa 2"));
        let nome_arquivo = "test.json";
        assert!(lista.salvar_em_json(nome_arquivo).is_ok());
        let mut lista_carregada = ListaDeTarefas::new();
        assert!(lista_carregada.carregar_de_json(nome_arquivo).is_ok());
        assert_eq!(lista.tarefas, lista_carregada.tarefas);
        fs::remove_file(nome_arquivo).unwrap();
    }

    #[test]
    fn test_carregar_json_invalido() {
        let nome_arquivo = "invalid.json";
        fs::write(nome_arquivo, "Este não é um JSON válido").unwrap();
        let mut lista = ListaDeTarefas::new();
        let resultado = lista.carregar_de_json(nome_arquivo);
        assert!(resultado.is_err(), "Deveria falhar ao carregar um JSON inválido");
        fs::remove_file(nome_arquivo).unwrap();
    }


    #[test]
    fn test_rollback_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"));
        lista.completar_tarefa(0);
        lista.rollback_tarefa(0);
        assert_eq!(lista.tarefas[0].estado, Estado::NaoIniciada);

        lista.adicionar_tarefa(String::from("Teste2"));
        lista.iniciar_tarefa(1);
        lista.rollback_tarefa(1);
        assert_eq!(lista.tarefas[0].estado, Estado::NaoIniciada);

        lista.rollback_tarefa(0);
        lista.rollback_tarefa(5);
    }



}
