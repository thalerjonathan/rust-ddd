export RUST_LOG=debug

cargo run --release -- --db-url "postgres://postgres:postgres@localhost:5436/fixtures?application_name=rustddd&options=-c search_path%3Drustddd" --server-host localhost:4003 --redis-url "redis://default:rustddd@127.0.0.1:6379/"

