# Buhtig

CLI para consumir dados da API do GitHub sobre atividade de usuarios e repositorios.

## Requisitos

- Rust 1.70+
- Token de acesso pessoal do GitHub

## Instalação

1. Clone o repositório:
```bash
git clone https://github.com/ma-alves/buhtig.git
cd buhtig
```

2. Crie um Personal Access Token no GitHub com permissão de acesso a `Metadata`. Acesse: https://github.com/settings/personal-access-tokens

3. Configure o token:
```bash
cp .env.example .env
```

Edite o arquivo `.env` e insira seu token:
```
GITHUB_TOKEN=seu_token_aqui
```

## Build

```bash
cargo build --release
```

O binário será gerado em `target/release/buhtig`.

## Uso do CLI

### Comando `user`

Busca eventos de um usuario do GitHub.

```bash
buhtig user <username>
```

Exemplo:
```bash
buhtig user ma-alves
```

### Comando `repo`

Busca eventos de um repositório específico.

```bash
buhtig repo <owner> <repo>
```

Exemplo:
```bash
buhtig repo rust-lang rust
```

### Opções globais

```bash
buhtig --help      # Exibe a ajuda
buhtig --version   # Exibe a versao
```

## Dependencias

- [clap](https://docs.rs/clap/latest/clap/) - Parsing de argumentos CLI
- [reqwest](https://docs.rs/reqwest/latest/reqwest/) - Cliente HTTP
- [serde](https://docs.rs/serde/latest/serde/) - Serializacao/Deserializacao JSON
- [dotenv](https://docs.rs/dotenv/latest/dotenv/) - Variaveis de ambiente
