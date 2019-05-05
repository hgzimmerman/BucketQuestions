#!/usr/bin/env bash

cd ../frontend;
npm install && npm run-script build
cd ../backend
bash ../docker/wait_for_it.sh db:5432 -q -- diesel setup --migration-dir db/migrations && cargo run --bin server_bin --release -- --server-lib-root ./server --port 8080 --staging
