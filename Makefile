BINARY_NAME = durl
OUT_DIR = dist

# Targets
LINUX_AMD64   = x86_64-unknown-linux-gnu
LINUX_ARM64   = aarch64-unknown-linux-gnu
MACOS_AMD64   = x86_64-apple-darwin
MACOS_ARM64   = aarch64-apple-darwin
WINDOWS_AMD64 = x86_64-pc-windows-gnu
WINDOWS_ARM64 = aarch64-pc-windows-gnullvm

TARGETS = $(LINUX_AMD64) $(LINUX_ARM64) $(MACOS_AMD64) $(MACOS_ARM64) $(WINDOWS_AMD64) $(WINDOWS_ARM64)

.PHONY: all build clean setup test $(TARGETS)

all: build

build: $(TARGETS)

test:
	cargo test

$(TARGETS):
	@echo "Building for $@..."
	@rustup target add $@ > /dev/null 2>&1 || true
	cargo build --release --target $@
	@mkdir -p $(OUT_DIR)/$@
	@if [ -f target/$@/release/$(BINARY_NAME).exe ]; then \
		cp target/$@/release/$(BINARY_NAME).exe $(OUT_DIR)/$@/ ; \
	elif [ -f target/$@/release/$(BINARY_NAME) ]; then \
		cp target/$@/release/$(BINARY_NAME) $(OUT_DIR)/$@/ ; \
	fi

clean:
	rm -rf $(OUT_DIR)
	cargo clean

setup:
	rustup target add $(TARGETS)
