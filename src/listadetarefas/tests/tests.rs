use listadetarefas::{Estado, ListaDeTarefas};

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
    fn test_salvar_json(){
        let mut lista = ListaDeTarefas::new();

    }

    #[test]
    fn test_iniciar_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"));
        lista.iniciar_tarefa(0);
        assert_eq!(lista.tarefas[0].estado, Estado::EmAndamento);
       // assert_eq!(lista.tarefas[1].estado, None);
    }

    #[test] //Este teste já não tem mais o que fazer nele
    fn test_completar_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"));
        assert_eq!(lista.tarefas[0].estado, Estado::NaoIniciada);
        lista.completar_tarefa(0);
        assert_eq!(lista.tarefas[0].estado, Estado::Concluida);

        lista.adicionar_tarefa(String::from("Teste2"));
        lista.iniciar_tarefa(1);
        assert_eq!(lista.tarefas[1].estado, Estado::EmAndamento);
        lista.completar_tarefa(1);
        assert_eq!(lista.tarefas[1].estado, Estado::Concluida);

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

}

