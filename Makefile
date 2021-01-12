CARGO=cargo

all: clean
	@$(CARGO) run

run:
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
	@$(CARGO) fmt -- --check
	@$(CARGO) build
