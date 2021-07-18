.PHONY: all clean help doc test check

BIN := target/release/argus

all: $(BIN) 	# build all sub-project

$(BIN): $(wildcard src/*.rs) linter
	cargo build --release

linter:	# run static analysis lint
	cargo check
	cargo fmt

test:	# run test
	cargo test
	cargo update
	cargo bench

clean:		# clean-up environment
	cargo clean

help:		# show this message
	@printf "Usage: make [OPTION]\n"
	@printf "\n"
	@perl -nle 'print $$& if m{^[\w-]+:.*?#.*$$}' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?#"} {printf "    %-18s %s\n", $$1, $$2}'

doc:		# show the document
	cargo doc --open

INSTALL_PATH := /usr/local/bin 
install: $(BIN)	# install the argus to INSTALL_PATH
	install -m755 $< $(INSTALL_PATH)
