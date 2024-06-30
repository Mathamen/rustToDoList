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
* Clone este repositório em seu ambiente local. -> ```git clone https://github.com/Mathamen/rustToDoList```
* Navegue até o diretório raiz do projeto. -> ```cd rustToDoList```
* Execute o comando cargo run para compilar e executar o projeto. -> ```cargo run```
  
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
Este projeto inclui testes automatizados para garantir o correto funcionamento das funcionalidades.

Como testar com tarpaulin?
Em root, digite ``` cargo-tarpaulin --out html ```

Caso não queira testar, o tarpaulin indica cobertura de 95.26% de todo o código, ou seja, 181/190 linhas.