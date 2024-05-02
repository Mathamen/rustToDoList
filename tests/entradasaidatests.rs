use entradasaida::limpar_console;


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, BufRead, Cursor, Read, Write};
    use std::process::{Command, Stdio};
    use entradasaida::{abrir_arquivo, adicionar_tarefa, atribuir_comando_enum, Entrada, give_texto,  print_handler, tratar_input_int, tratar_input_string, trigger_continue, trim_margin};
    use listadetarefas::ListaDeTarefas;


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
        adicionar_tarefa(&mut lista,comando);
        let comando=String::from("");
        adicionar_tarefa(&mut lista,comando);

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
        abrir_arquivo();
    }



}