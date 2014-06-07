GITBOOK = gitbook
RUSTC = rustc
QUIET = -A unused-variable -A dead-code -A dead-assignment -A experimental
RUSTC_NT = $(RUSTC) --no-trans --test $(QUIET)
WHITELIST = examples/borrow/freeze.rs \
						examples/attribute/custom.rs \
						examples/crates/executable.rs \
						examples/lifetime/lifetime.rs \
						examples/lifetime/reference-bad.rs \
						examples/mod/nested.rs \
						examples/move/assignment.rs \
						examples/move/pass-by-value.rs \
						examples/variables/declare.rs \
            examples/variables/variables.rs
srcs = $(filter-out $(WHITELIST),$(shell find examples -name '*.rs'))

.PHONY: all book clean test serve

all:
	./setup-stage.sh
	$(RUSTC) src/update.rs
	./update
	rm update

book:
	cd stage && $(GITBOOK) build
	./fix-edit-button.sh
	./add-relinks.sh

clean:
	rm -rf stage

test:
	$(foreach src,$(srcs),$(RUSTC_NT) $(src) || exit;)
	./check-line-length.sh

serve:
	cd stage && $(GITBOOK) serve
