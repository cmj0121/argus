SUBDIR=

.PHONY: all clean help doc $(SUBDIR)

all: $(SUBDIR) 	# build all sub-project
	@cargo check
	@cargo fmt
	@cargo test
	@cargo update
	@cargo bench
	cargo build --release

clean:		# clean-up environment
	cargo clean

help:		# show this message
	@printf "Usage: make [OPTION]\n"
	@printf "\n"
	@perl -nle 'print $$& if m{^[\w-]+:.*?#.*$$}' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?#"} {printf "    %-18s %s\n", $$1, $$2}'

doc:		# show the document
	cargo doc --open

$(SUBDIR):
	$(MAKE) -C $@ $(MAKECMDGOALS)
