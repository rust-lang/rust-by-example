GITBOOK = gitbook -t 'Rust by Example'
RUSTC=printf "\033[32;1mRustc:\033[33m %s\033[m\n" $@; rustc
RFLAGS = --allow dead-assignment --allow dead-code --allow unused-variable
RUSTC_NT=$(RUSTC) $(RFLAGS) --no-trans --test
srcs = $(wildcard */*/*.rs)
PROG:=$(patsubst src/%.rs,build/%,$(srcs))

.SILENT:
.PHONY: all book clean test serve

all: build/update
	mkdir -p output/examples
	ln -sf ../src/README.md output/README.md
	build/update

book:
	cd output && $(GITBOOK) build
	./fix-edit-button.sh

clean:
	rm -rf build output/examples
	rm -f output/{README,SUMMARY}.md

build/%: src/%.rs
	mkdir -p $(shell dirname $@)
	$(RUSTC) $(RFLAGS) $< -o $@ || exit

compile: $(PROG)

test:
	$(foreach src,$(srcs),$(RUSTC_NT) $(src) || exit;)

serve:
	cd output && $(GITBOOK) serve
