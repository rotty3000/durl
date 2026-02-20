BINARY_NAME = durl
OUT_DIR = dist

# Targets
LINUX_AMD64   = x86_64-unknown-linux-musl
LINUX_ARM64   = aarch64-unknown-linux-musl
# MACOS_AMD64   = x86_64-apple-darwin
# MACOS_ARM64   = aarch64-apple-darwin
WINDOWS_AMD64 = x86_64-pc-windows-gnu
# WINDOWS_ARM64 = aarch64-pc-windows-msvc

TARGETS = $(LINUX_AMD64) $(LINUX_ARM64) $(WINDOWS_AMD64)

.PHONY: all build clean compress setup test $(TARGETS)

all: build

build: $(TARGETS)

compress: build
	@echo "Compressing binaries with UPX..."
	@find $(OUT_DIR) -type f -name "$(BINARY_NAME)*" -exec upx --best {} +

test:
	cargo test

$(TARGETS):
	@echo "Building for $@..."
	@if [ "$@" = "x86_64-unknown-linux-gnu" ]; then \
		cargo build --release --target $@; \
		mkdir -p $(OUT_DIR)/$@; \
		if [ -f target/$@/release/$(BINARY_NAME) ]; then \
			cp target/$@/release/$(BINARY_NAME) $(OUT_DIR)/$@/ ; \
		fi; \
	else \
		cross build --release --target $@ --target-dir target/cross; \
		mkdir -p $(OUT_DIR)/$@; \
		if [ -f target/cross/$@/release/$(BINARY_NAME).exe ]; then \
			cp target/cross/$@/release/$(BINARY_NAME).exe $(OUT_DIR)/$@/ ; \
		elif [ -f target/cross/$@/release/$(BINARY_NAME) ]; then \
			cp target/cross/$@/release/$(BINARY_NAME) $(OUT_DIR)/$@/ ; \
		fi; \
	fi

clean:
	rm -rf $(OUT_DIR)
	cargo clean

setup:
	rustup target add $(TARGETS)
