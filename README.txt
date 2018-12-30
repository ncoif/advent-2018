My try to learn rust with https://adventofcode.com/ this year

# To format the code

`cargo fmt`

# To build

`cargo build --release`
`cargo test`

# To run

`cargo run <day> <answer>`

# To run all tests and performance

`./all.sh`

Rust questions:
- String vs str
- compare & and * with c syntax
- PartialEq vs Eq, PartialOrd vs Ord
- clone vs copy
- iter() vs into_iter()


For day 23, using z3:
- git clone https://github.com/Z3Prover/z3.git
python scripts/mk_make.py
cd build
make
sudo make install
