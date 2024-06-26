use listadetarefas::{Estado, ListaDeTarefas};

#[cfg(test)]
mod tests {
    use std::{fs, io};
    use std::io::{Cursor, empty, Write};
    use std::path::Path;
    use assert_cmd::Command;
    use std::process::Command as SysCommand;

    use entradasaida::{loop_principal, tratar_input_int, tratar_input_string, trigger_continue};
    use mock::IOMock;

    use super::*;


    #[test]
    fn test_adicionar_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"),"test.json");
        assert_eq!(lista.tarefas.len(), 1);
    }

    #[test]
    fn test_iniciar_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"),"test.json");
        lista.iniciar_tarefa(0,"test.json");
        assert_eq!(lista.tarefas[0].estado, Estado::EmAndamento);
        lista.iniciar_tarefa(0,"test.json");
        lista.iniciar_tarefa(20,"test.json");
    }

    #[test]
    fn test_completar_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"),"test.json");
        assert_eq!(lista.tarefas[0].estado, Estado::NaoIniciada);
        lista.completar_tarefa(0,"test.json");
        assert_eq!(lista.tarefas[0].estado, Estado::Concluida);
        lista.completar_tarefa(0,"test.json");

        lista.adicionar_tarefa(String::from("Teste2"),"test.json");
        lista.iniciar_tarefa(1,"test.json");
        assert_eq!(lista.tarefas[1].estado, Estado::EmAndamento);
        lista.completar_tarefa(1,"test.json");
        assert_eq!(lista.tarefas[1].estado, Estado::Concluida);

        lista.completar_tarefa(20,"test.json");

    }

    #[test]
    fn test_remover_tarefa() {
        let mut lista = ListaDeTarefas::new();
        lista.remover_tarefa(0,"test.json");
        lista.adicionar_tarefa(String::from("Teste"),"test.json");
        lista.remover_tarefa(0,"test.json");
        assert_eq!(lista.tarefas.len(), 0);
        lista.remover_tarefa(20,"test.json");
    }

    #[test]
    fn test_carregar_salvar_json() {
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"),"test.json");
        lista.salvar_em_json("test.json").unwrap();
        let mut lista2 = ListaDeTarefas::new();
        lista2.carregar_de_json("test.json").unwrap();
        assert_eq!(lista.tarefas, lista2.tarefas);
    }

    #[test]
    fn test_listar_tarefas() {
        let mut lista = ListaDeTarefas::new();
        lista.listar_tarefas(None, None);
        lista.adicionar_tarefa(String::from("Tarefa 1"),"test.json");
        lista.adicionar_tarefa(String::from("Tarefa 2"),"test.json");
        lista.adicionar_tarefa(String::from("Tarefa 3"),"test.json");
        lista.iniciar_tarefa(0,"test.json");
        lista.completar_tarefa(1,"test.json");

        // Testando listar as tarefas
        let output: Vec<String> = Vec::new();
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
        lista.adicionar_tarefa(String::from("Tarefa 1"),"test.json");
        lista.adicionar_tarefa(String::from("Tarefa 2"),"test.json");
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
        lista.adicionar_tarefa(String::from("Teste"),"test.json");
        lista.completar_tarefa(0,"test.json");
        lista.rollback_tarefa(0,"test.json");
        assert_eq!(lista.tarefas[0].estado, Estado::NaoIniciada);

        lista.adicionar_tarefa(String::from("Teste2"),"test.json");
        lista.iniciar_tarefa(1,"test.json");
        lista.rollback_tarefa(1,"test.json");
        assert_eq!(lista.tarefas[0].estado, Estado::NaoIniciada);

        lista.rollback_tarefa(0,"test.json");
        lista.rollback_tarefa(5,"test.json");
    }

    #[test]
    fn test_editar_descricao(){
        let mut lista = ListaDeTarefas::new();
        lista.adicionar_tarefa(String::from("Teste"),"test.json");
        lista.editar_tarefa(0, String::from("hello"),"test.json");
        assert_eq!(lista.tarefas[0].descricao, String::from("hello"))
    }
    #[test]
    fn test_tratar_input_string() {
        let input = "Hello, world!\n";
        let mut io_mock = IOMock::new(input.as_bytes(), Vec::new());
        let result = tratar_input_string(&mut io_mock);
        assert_eq!(result.unwrap(), "Hello, world!");
    }

    #[test]
    fn test_tratar_input_string_empty() {
        let input = "\n";
        let mut io_mock = IOMock::new(input.as_bytes(), Vec::new());
        let result = tratar_input_string(&mut io_mock);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_tratar_input_int() {
        let input = "42\n";
        let mut io_mock = IOMock::new(input.as_bytes(), Vec::new());
        let result = tratar_input_int(&mut io_mock);
        assert_eq!(result.unwrap(), 42);


    }

    #[test]
    fn test_tratar_input_int_empty() {
        let input = "\n";
        let mut io_mock = IOMock::new(input.as_bytes(), Vec::new());
        let result = tratar_input_int(&mut io_mock);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_trigger_continue() {
        let mut io_mock = IOMock::new(empty(), Vec::new());
        trigger_continue(&mut io_mock);
    }

    #[test]
    fn test_loop_principal() {
        let input_data = "\
            1\n\
            Nova tarefa\n\
            5\n\
            \n\
            \n\

            1\n\
            \n\
            \n\

            2\n\
            1\n\
            \n\
            \n\
            2\n\
            45\n\
            \n\
            \n\
            2\n\
            sagbosg\n\
            \n\
            \n\
            3\n\
            1\n\
            3\n\
            1\n\
            3\n\
            kdhgbsa\n\
            \n\
            \n\


            6\n\
            1\n\
            6\n\
            1\n\
            6\n\
            \n\
            \n\


           7\n\
           1\n\
           teste\n\
           7\n\
           \n\
           7\n\
           asogab\n\
           \n\
           \n\





            4\n\
            \n\
            4\n\
            asfasf\n\
            4\n\
            1\n\
            4\n\
            2131\n\













            8\n\
            8\n\
            \n\
            \n\
            ";


        let input = Cursor::new(input_data);
        let output = Cursor::new(Vec::new());
        let mut io_control = IOMock { reader: input, writer: output };

        loop_principal(&mut io_control,"test.json");

        let output_str = String::from_utf8(io_control.writer.into_inner()).unwrap();
    }




}
