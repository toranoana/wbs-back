# wbs tool backend api repository

main repository: <https://github.com/toranoana/wbs-back>

under src directory

- db
  - model and repository
- graphql
  - graphql scheme and resolver
- handlers
  - custom actix handlers
- utils
  - utility library
- middlewares
  - actix middlewares

## require

- development
  - docker
- production or staging
  - Rust 2018
  - PostgreSQL 12

## Project Setup

### development

```
./prepare.sh
docker-compose up -d
bin/docker_shell.sh
cargo build
```

### production

```
cp .env.sample .env
cp .env.sample .env.local
cargo build
```

## prepare

```
diesel setup
diesel migration run
```

## run

### development

```
cargo watch -x 'run'
```

### production

```
cargo build --release
```

After, Please add `./target/release/wbs_api` to daemon (ex `systemd`)

## data insert

- access to `http://localhost:3000/graphiql`
  - open graphql console

### Add User

example

```
mutation {
  createUser(newUser: {displayName: "test3"}) {
    id
    displayName
  }
}
```
