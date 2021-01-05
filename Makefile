CARGO=cargo

all: clean
	@$(CARGO) run

build:
	@$(CARGO) build

test:
	@$(CARGO) test

clean:
	@$(CARGO) clean

format:
	@$(CARGO) clippy
	@$(CARGO) fmt

ci:
	@$(CARGO) clippy -- -Dwarnings
	@$(CARGO) fmt -- --check
	@$(CARGO) test
