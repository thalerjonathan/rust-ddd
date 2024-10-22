export RUST_LOG=debug

cargo run --release -- --db-url "postgres://postgres:postgres@localhost:5433/referees?application_name=rustddd&options=-c search_path%3Dreferees" --server-host localhost:4000
