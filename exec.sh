cat ./headers.c > "$2"
cargo run --bin dump $1 >> "$2"
gcc "$2"
./a.out
