.PHONY: install
install:
	@echo "+$@"
	@cargo build --release
	@cp ./target/release/landlord ~/bin/landlord
