.PHONY: build

# wasmtime releases: https://github.com/bytecodealliance/wasmtime/releases

WASMTIME_ARCH=x86_64
WASMTIME_VERSION=v0.21.0
WASMTIME_PKG_NAME=wasmtime-$(WASMTIME_VERSION)-$(WASMTIME_ARCH)-linux

build: bin/wasmtime

bin/wasmtime:
	mkdir -p bin
	curl -sL https://github.com/bytecodealliance/wasmtime/releases/download/$(WASMTIME_VERSION)/$(WASMTIME_PKG_NAME).tar.xz | tar -xz
	mv $(WASMTIME_PKG_NAME)/wasmtime bin/
	rm -rf $(WASMTIME_PKG_NAME)
