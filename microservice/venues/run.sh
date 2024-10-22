export RUST_LOG=debug

cargo run --release -- --db-url "postgres://postgres:postgres@localhost:5434/venues?application_name=rustddd&options=-c search_path%3Dvenues" --server-host localhost:4001

