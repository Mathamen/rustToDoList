# rustToDoList

## Hello World! Descrição

Este projeto implementa uma lista de tarefas (ToDoList) em Rust. Ele permite adicionar, iniciar, completar e remover tarefas, além de listar as tarefas disponíveis.

### Funcionalidades
* Adicionar tarefas
* Iniciar tarefas
* Completar tarefas
* Remover tarefas
* Listar tarefas disponíveis
* Voltar o Estado de uma tarefa
* Salvar e carregar tarefas de um arquivo JSON

  
### Pré-requisitos
Certifique-se de ter o Rust instalado em seu sistema. Você pode encontrar instruções de instalação no site oficial do Rust.

### Como usar
* Clone este repositório em seu ambiente local.
* Navegue até o diretório raiz do projeto.
* Execute o comando cargo run para compilar e executar o projeto.
Comandos disponíveis

Ao executar o programa, você será apresentado a um menu com as seguintes opções:

* Adicionar tarefa
* Iniciar tarefa
* Completar tarefa
* Remover tarefa
* Listar tarefas
* Voltar para o estado não iniciada
* Sair

Escolha o número correspondente à ação desejada e siga as instruções na tela.

### Testes automatizados
Este projeto inclui testes automatizados para garantir o correto funcionamento das funcionalidades. Para executar os testes, você pode usar o comando cargo test, em cada crate. Neste projeto foi utilizado o tarpaulin. Para a crate listadetarefas, o tarpaulin apontou 82.26% de cobertura. Para a crate entradaesaida, apresentou X.

