# User Authentication Service

Authentication Service using Rust with Actix Web 

# Usage

## SSL libs + Clang/LLVM
sudo apt install openssl libssl-dev clang llvm-dev libclang-dev

## SQLX CLI
cargo install --git https://github.com/launchbadge/sqlx sqlx-cli

## Run docker-compose (postgres database)
docker-compose up -d

## Run migrations
sqlx mig run

## Run the server (http://localhost:3000)
cargo run

