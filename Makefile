GITBOOK = gitbook
RUSTC = rustc
QUIET = -A unused-variable -A dead-code -A dead-assignment
RUSTC_NT = rustc --no-trans --test $(QUIET)
WHITELIST = src/borrow/freeze.rs \
						src/attribute/custom.rs \
						src/crates/executable.rs \
						src/lifetime/lifetime.rs \
						src/lifetime/reference-bad.rs \
						src/mod/nested.rs \
						src/move/assignment.rs \
						src/move/pass-by-value.rs \
						src/variables/declare.rs \
            src/variables/variables.rs
srcs = $(filter-out $(WHITELIST),$(wildcard */*/*.rs))

.PHONY: all book clean test serve

all:
	mkdir -p output/examples
	ln -sf ../src/README.md output/README.md
	$(RUSTC) src/update.rs
	./update
	rm update

book:
	mkdir -p output
	cd output && $(GITBOOK) build
	./fix-edit-button.sh

clean:
	rm -rf output

test:
	$(foreach src,$(srcs),$(RUSTC_NT) $(src) || exit;)
	./check-line-length.sh

serve:
	mkdir -p output
	cd output && $(GITBOOK) serve
