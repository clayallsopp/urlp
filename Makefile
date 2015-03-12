# lovingly borrowed from https://github.com/ogham/exa/blob/master/Makefile
PREFIX ?= /usr/local

BUILD = target/release/urlp

$(BUILD):
	@which rustc > /dev/null || { echo "urlp requires Rust-lang to compile. For installation instructions, please visit http://rust-lang.org/"; exit 1; }
	cargo build --release

build: $(BUILD)

INSTALL = $(PREFIX)/bin/urlp

$(INSTALL):
	cp target/release/urlp $(PREFIX)/bin/

install: build $(INSTALL)

.PHONY: install