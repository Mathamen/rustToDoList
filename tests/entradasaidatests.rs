use entradasaida::limpar_console;

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use entradasaida::{abrir_arquivo, adicionar_tarefa, atribuir_comando_enum, Entrada, give_texto, print_handler, tratar_input_int, tratar_input_string, trim_margin};
    use listadetarefas::ListaDeTarefas;
    use mock::IOMock;
    use super::*;

    #[test]
    fn test_atribuir_entrada_enum_e_adicionar() {
        let comando = String::from("0");
        assert_eq!(atribuir_comando_enum(comando), Entrada::ValorInvalido);

        let comando = String::from("1");
        assert_eq!(atribuir_comando_enum(comando), Entrada::Adicionar);

        let comando = String::from("2");
        assert_eq!(atribuir_comando_enum(comando), Entrada::Iniciar);

        let comando = String::from("3");
        assert_eq!(atribuir_comando_enum(comando), Entrada::Completar);

        let comando = String::from("4");
        assert_eq!(atribuir_comando_enum(comando), Entrada::Remover);

        let comando = String::from("5");
        assert_eq!(atribuir_comando_enum(comando), Entrada::Listar);

        let comando = String::from("6");
        assert_eq!(atribuir_comando_enum(comando), Entrada::Rollback);

        let comando = String::from("7");
        assert_eq!(atribuir_comando_enum(comando), Entrada::EditarTarefa);
        let comando = String::from("8");
        assert_eq!(atribuir_comando_enum(comando), Entrada::Sair);

        let mut lista= ListaDeTarefas::new();
        let comando=String::from("hello");
        adicionar_tarefa(&mut lista,comando,"test.json");
        let comando=String::from("");
        adicionar_tarefa(&mut lista,comando,"test.json");

    }


    #[test]
    fn test_give_texto(){
        let s= "
        Escolha uma ação:
        1. Adicionar tarefa
        2. Iniciar tarefa
        3. Completar tarefa
        4. Remover tarefa
        5. Listar tarefas
        6. Voltar par estado não iniciada
        7. Sair ";
        give_texto();
    }
    #[test]
    fn test_trim_margin(){
        let s ="a";
        assert_eq!(trim_margin(s),s);
    }

    #[test]
    fn test_limpar_console(){
        limpar_console();
    }
    #[test]
    fn test_print_handler(){
        print_handler("a");
    }


    #[test]
    fn test_abrir_arquivo(){
        abrir_arquivo("test.json");
    }
    #[test]
    fn test_tratar_input_string_ok() {
        let input_data = "hello\n";
        let mut io_mock = IOMock {
            reader: Cursor::new(input_data),
            writer: Vec::new(),
        };
        assert_eq!(tratar_input_string(&mut io_mock).unwrap(), "hello");
    }

    #[test]
    fn test_tratar_input_string_empty() {
        let input_data = "\n";
        let mut io_mock = IOMock {
            reader: Cursor::new(input_data),
            writer: Vec::new(),
        };
        assert!(tratar_input_string(&mut io_mock).is_err());
    }

    #[test]
    fn test_tratar_input_int_ok() {
        let input_data = "42\n";
        let mut io_mock = IOMock {
            reader: Cursor::new(input_data),
            writer: Vec::new(),
        };
        assert_eq!(tratar_input_int(&mut io_mock).unwrap(), 42);
    }

    #[test]
    fn test_tratar_input_int_invalid() {
        let input_data = "invalid\n";
        let mut io_mock = IOMock {
            reader: Cursor::new(input_data),
            writer: Vec::new(),
        };
        assert!(tratar_input_int(&mut io_mock).is_err());
    }

    #[test]
    fn test_atribuir_comando_enum() {
        assert_eq!(atribuir_comando_enum("1".to_string()), Entrada::Adicionar);
        assert_eq!(atribuir_comando_enum("8".to_string()), Entrada::Sair);
        assert_eq!(atribuir_comando_enum("invalid".to_string()), Entrada::ValorInvalido);
    }

   // #[test]
    //fn test_trim_margin() {
    //    let input = "  line1\n  line2\n  line3\n";
    //    let expected = "line1\nline2\nline3";
   //     assert_eq!(trim_margin(input), expected);
    //}


    #[test]
    fn test_adicionar_tarefa() {
        let mut lista_de_tarefas = ListaDeTarefas::new();
        adicionar_tarefa(&mut lista_de_tarefas, "Nova tarefa".to_string(),"test.json");
        assert_eq!(lista_de_tarefas.tarefas.len(), 1);
    }

    #[test]
    fn test_adicionar_tarefa_empty() {
        let mut lista_de_tarefas = ListaDeTarefas::new();
        adicionar_tarefa(&mut lista_de_tarefas, "".to_string(),"test.json");
        assert_eq!(lista_de_tarefas.tarefas.len(), 0);
    }



}