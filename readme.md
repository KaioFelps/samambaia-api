# Hubbitos
Hubbitos é um fã-site de Habbo com as seguintes funcionalidades:
- Fluxo e gerenciamento de usuários (login, registro, edição);
- Controle de usuários de equipe (criação, apagamento, edição, busca);
- Notícias (publicar notícias, se autorizado, visualizar notícias, alterar e apagar);
- Comentários em notícias (escrever, inativar se autorizado);
- Denúncia dos comentários (criação de uma denúncia, resolução e/ou apagamento se autorizado).

O fã-site implementa um sistema de autorizações por cargos (roles) e permissões para cada cargo através de heranças.

# hubbitos-backennd
Backend de fã-site feito para o Hubbitos, mas não exclusivo deste. É o projeto responsável por implementar todas as funcionalidades e assegurar as autorizações e permissões.

## Instalando a aplicação
Para rodar o hubbitos-backend, será necessário:
1. Rustup ([como instalar](https://www.rust-lang.org/tools/install));
2. Docker ([como instalar](https://docs.docker.com/get-docker/));

**Passo a passo:**
1. clone este repositório:
```bash
git clone https://github.com/KaioFelps/hubbitos-backend.git
```
2. entre no repositório:
```bash
cd hubbitos-backend
```
3. copie todas as informações do arquivo `.env.sample` para um arquivo nomeado `.env` e preencha as informações necessárias:
```dotenv
DATABASE_URL=postgresql://database_user:database_password@localhost:port/database_name?schema=public
JWT_SECRET=
RUST_ENV=DEVELOPMENT
RUST_LOG=error
```
4. utilize o docker-compose para inicializar todos os serviços necessários para o funcionamento da aplicação:
```bash
docker-compose up -d
```

### Rodar em desenvolvimento
1. inicie a aplicação em desenvolvimento utilizando o cargo:
```bash
cargo run
```

### Compilar para produção
1. utilize o comando:
```bash
cargo build --release
```

Leia a [documentação do Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#:~:text=Cargo%20is%20Rust's%20build%20system,on%2C%20and%20building%20those%20libraries.) para mais informações quanto aos seus comandos.

## Estrutura da aplicação
A aplicação implementa a Arquitetura Limpa (clean architecture) até onde for conveniente. Abaixo, está uma tabela contendo os diretórios da aplicação e suas respectivas "funções":
| Diretório | Finalidade |
| :---  | :---      |
| cli/  | contém a micro-aplicação de linha de comando Hubbitos CLI. |
| entities/ | diretório gerado e gerenciado pela ORM utilizada pela aplicação. |
| migration/    | diretório gerado e gerenciado pela ORM utilizada pela aplicação. |
| src/  | contém todo o código fonte da aplicação. |
| src/core/ | tipos utilizados por todos os domínios da aplicação. |
| src/domain/   | contém todos os arquivos relacionados ao núcleo da aplicação, independentes de serviços externos. |
| src/domain/cryptography/ | traits (interfaces) para manejos relacionados a criptografia (hasher, comparador). |
| src/domain/domain_entities/  | entidades de domínio da aplicação. |
| src/domain/factories/    | fábricas dos "services" das aplicações, responsáveis pela injeção das dependência dos serviços. |
| src/domain/politics/     | funções agrupadas com base nos domínios que fazem verificações a nível do núcleo da aplicação. |
| src/domain/repositories/ | traits (interfaces) de contrato dos repositórios de cada entidade de domínio, com os métodos para manipulá-las (criar, apagar, salvar, etc). |
| src/domain/services/ | casos de uso; serviços da aplicação. São responsáveis por implementar as regras de negócio e algorítmos de cada caso de uso da aplicação. |
| src/env_config/   | contém a configuração de variáveis de ambiente (não substitui o `.env`, apenas providencia uma API para acessar as propriedades do arquivo `.env`). |
| src/errors/   | contém os erros que podem ser escalados pela aplicação. Um erro contém uma mensagem e um código HTTP. |
| src/infra/    | contém todas as dependências que serão injetadas pelas fábricas nos services. |
| src/infra/cryptography/ | implementação das traits (interfaces de contrato) do diretório `/domain/cryptography` utilizando serviços externos. |
| src/infra/http/   | contém todos os arquivos de adaptação da aplicação para requisições http. |
| src/infra/http/controllers/   | contém os controllers (structs que implementam a trait `ControllerTrait` e registram as rotas) da aplicação.  |
| src/infra/http/dtos/  | abrange os DataObects (dtos) da aplicação: structs responsáveis por validar os corpos das requisições https feitas nos controllers.   |
| src/infra/http/extractors/    | contém os extractors, isso é, estruturas que extraem informações dos corpos das requisições "parsiadas" pro seu próprio tipo. |
| src/infra/http/middlewares/   | contém os middlewares. |
| src/infra/http/presenters/    | contém os presenters da aplicação (structs com métodos responsáveis por formatar entidades de domínio em structs serializáveis para json).    |
| src/infra/http/routes/        | contém as structs que implementam a trait `RouteTrait`, que garante o método `register` utilizado para registrar todas as rotas do escopo no servidor.    |
| src/infra/jwt/jwt_service.rs    | serviço responsável por lidar com o JWT (json-web-token), providenciando uma API.   |
| src/infra/sea/  | contém todas as implementações das traits (interfaces de contrato) necessárias utilizando o [Sea-ORM](https://github.com/SeaQL/sea-orm).    |
| src/infra/sea/mappers/  | contempla "structs" (estruturas) com métodos para converter entidades de domínio em modelos do Sea-ORM e vice-versa.    |
| src/infra/sea/repositories/ | implementação dos contratos dos repositórios de domínio utilizando Sea-ORM. |
| src/infra/sea/sea_service.rs    | struct" (estrutura) contendo uma conexão com o banco de dados a ser fornecida para operações utilizando o Sea-ORM.  |
| src/libs/ | contem funções "wrappers" de funções frequentemente usadas de bibliotecas third-parties.  |
| src/util/ | contém funções auxiliares utilizadas pelos "services" ou qualquer outro algoritmo/função. |
| src/main.rs   | ponto de entrada de qualquer aplicação Rust. |
| test/ | diretório para testes que fogem do contexto de um arquivo. Inutilizado no momento. |

## Bibliografia
Lista com "links" das documentações e outros sites úteis para se inteirar das principais bibliotecas e outros serviços utilizados nessa aplicação:
- [Rust Book](https://doc.rust-lang.org/book/): livro/documentação oficial do Rust Lang;
- [Tokio](https://tokio.rs/): runtime que possibilita trabalhar com Rust de maneira assíncrona;
- [Mockall](https://crates.io/crates/mockall): biblioteca para "mockar" repositórios e possibilitar testes unitários;
- [sea-orm](https://github.com/SeaQL/sea-orm): ORM utilizada para interagir com o banco de dados;
- [Postgre](https://www.postgresql.org/): banco de dados utilizado;
- [Docker](https://docs.docker.com/get-docker/): serviço de "containers" para levantar serviços em desenvolvimento (como bancos de dados).

# Hubbitos CLI
Hubbitos CLI (Hubbitos Command Line Interface) é uma linha de comandos em processo de desenvolvimento que objetiva facilitar a criação de novas funcionalidades seguindo o padrão já estabelecido pela aplicação. Atingimos esse objetivo através de convenções e templates.

Para mais informações sobre como utilizar a Hubbitos CLI acesse a [documentação](cli/readme.md).

---

Obs.: todos os arquivos, com exceção deste, estão disponíveis somente em inglês.