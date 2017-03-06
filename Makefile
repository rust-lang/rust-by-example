GITBOOK = node_modules/.bin/gitbook
YASPELLER = node_modules/.bin/yaspeller
RUSTC = rustc
STRICT = -D deprecated
QUIET = -A unused-variables -A dead-code -A unused-assignments
RUSTC_NT = $(RUSTC) -Z no-trans --test $(QUIET) # ${STRICT}
WHITELIST = examples/attribute/cfg/custom/custom.rs \
            examples/scope/borrow/borrow.rs \
            examples/scope/borrow/freeze/freeze.rs \
            examples/scope/borrow/mut/mut.rs \
            examples/trait/bounds/bounds.rs \
            examples/custom_types/constants/constants.rs \
            examples/crates/link/executable.rs \
            examples/scope/lifetime/borrow/borrow.rs \
            examples/mod/mod.rs \
            examples/hello/print/print.rs \
            examples/cast/cast.rs \
            examples/primitives/primitives.rs \
            examples/variable_bindings/scope/scope.rs \
            examples/variable_bindings/mut/mut.rs \
            examples/variable_bindings/declare/declare.rs \
            examples/std/vec/vec.rs

srcs = $(filter-out $(WHITELIST),$(shell find examples -name '*.rs'))

.PHONY: all html epub pdf clean test serve

all:
	./setup-stage.sh
	cargo run

html: node_modules/gitbook
	$(GITBOOK) build stage
	./fix-edit-button.sh
	./add-relinks.sh

epub:	node_modules/gitbook
	$(GITBOOK) epub stage

pdf:	node_modules/gitbook
	$(GITBOOK) pdf stage

clean:
	rm -rf bin stage

test: node_modules/yaspeller
	@$(foreach src,$(srcs),$(RUSTC_NT) $(src) || exit;)
	./check-line-length.sh
	./check-line-length-md.sh
	./check-links.sh
	$(YASPELLER) --only-errors README.md CONTRIBUTING.md examples

serve: node_modules/gitbook
	$(GITBOOK) serve stage

node_modules/gitbook:
	npm install gitbook@1.5.0

node_modules/yaspeller:
	npm install yaspeller
