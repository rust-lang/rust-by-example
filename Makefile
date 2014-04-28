GITBOOK = gitbook -t 'Rust by Example'
RUSTC = rustc
QUIET = -A unused-variable -A dead-code -A dead-assignment
RUSTC_NT = rustc --no-trans --test $(QUIET)
srcs = $(wildcard */*/*.rs)

.PHONY: all book clean test serve

all:
	mkdir -p output/examples
	ln -sf ../src/README.md output/README.md
	$(RUSTC) src/update.rs
	./update
	rm update

book:
	cd output && $(GITBOOK) build
	./fix-edit-button.sh

clean:
	rm -rf output/examples
	rm output/{README,SUMMARY}.md

test:
	$(foreach src,$(srcs),$(RUSTC_NT) $(src) || exit;)
	./check-line-length.sh

serve:
	cd output && $(GITBOOK) serve
