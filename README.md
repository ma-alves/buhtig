# buhtig
CLI para consumir dados da API do GitHub sobre atividade de usuários e em repositórios. Projeto inspirado em uma das propostas do [roadmap.sh](https://roadmap.sh/backend/projects).

## Crates
- [clap](https://docs.rs/clap/latest/clap/) - CLI  
- [reqwest](https://docs.rs/reqwest/latest/reqwest/) - HTTP Client  
- [serde](https://docs.rs/serde/latest/serde/) - JSON Serializer/Deserializer  

## Configuração
1. Clone o repositório:
```bash
git clone https://github.com/ma-alves/buhtig.git
```
2. Crie um personal access token [aqui](https://github.com/settings/personal-access-tokens) com permissão de acesso a metadata em repositórios.
3. Ajuste seu token de acesso no `.env`:
```bash
cp .env.example .env
```
4. WIP