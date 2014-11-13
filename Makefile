GITBOOK = gitbook
RUSTC = rustc
STRICT = -D deprecated
QUIET = -A unused_variables -A dead-code -A unused_assignments -A experimental
RUSTC_NT = $(RUSTC) --no-trans --test $(QUIET) ${STRICT}
WHITELIST = examples/attribute/cfg/custom/custom.rs \
						examples/borrow/borrow.rs \
	 					examples/borrow/freeze/freeze.rs \
	 					examples/borrow/mut/mut.rs \
	 					examples/bounds/bounds.rs \
	 					examples/constants/constants.rs \
	 					examples/crates/link/executable.rs \
	 					examples/lifetime/borrow/borrow.rs \
	 					examples/mod/mod.rs \
	 					examples/print/print.rs \
	 					examples/type/cast/cast.rs \
	 					examples/type/type.rs \
	 					examples/variables/declare/declare.rs \
	 					examples/variables/mut/mut.rs \
	 					examples/variables/scope/scope.rs \
	 					examples/vec/vec.rs \

srcs = $(filter-out $(WHITELIST),$(shell find examples -name '*.rs'))

.PHONY: all book clean test serve

all:
	./setup-stage.sh
	$(RUSTC) src/update.rs --out-dir bin
	bin/update

book:
	cd stage && $(GITBOOK) build
	./fix-edit-button.sh
	./add-relinks.sh

clean:
	rm -rf {bin,stage}

test:
	$(foreach src,$(srcs),$(RUSTC_NT) $(src) || exit;)
	./check-line-length.sh

serve:
	cd stage && $(GITBOOK) serve
