# Rust to Python bindings
Goal of this project to learn and test how to create Rust to Python bindings

![CI/CD Build Status](https://github.com/github/docs/actions/workflows/CI.yml/badge.svg?event=push)

## (Desired) Features
- Cloud first
- Kubernetes compatible
- Python bindings
- Fault tolerance
- Lazy evaluation

## Getting Started
### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Conda](https://docs.conda.io/en/latest/miniconda.html)
- [Docker](https://docs.docker.com/get-docker/)

### Python module
Execute `make conda-env` command on the terminal to get Python dependencies installed and
to create conda environment.

### Rust Module
Steps to build and use Rust module in Python code
1. If you are on Macbook with M1 chip THEN execute `rustup target add x86_64-apple-darwin` command in shell. This step needs to be executed only once when you are setting up your development environment.
2. Execute `make maturin`command to build the Rust module. If this succeeds then you will see `Installed agni-0.1.0` message.
3. If step 2 is successful then execute `pip list` command, you should see `agni` dependency listed.
4. Now you can use `agni` in your Python code.
