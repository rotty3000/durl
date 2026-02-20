# Copyright 2026 Raymond Auge <rayauge@doublebite.com>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

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

.PHONY: all build clean compress copyright lint setup test $(TARGETS)

all: build

build: $(TARGETS)

compress: build
	@echo "Compressing binaries with UPX..."
	@find $(OUT_DIR) -type f -name "$(BINARY_NAME)*" -exec upx --best {} +

copyright:
	@echo "Updating copyright years..."
	@find . -type f \( -name "*.rs" -o -name "Makefile" -o -name "*.yml" -o -name "*.toml" -o -name "*.rb" \) \
		-not -path "./target/*" \
		-not -path "./.git/*" \
		-exec sed -i "s/Copyright [0-9]\{4\}/Copyright $(shell date +%Y)/g" {} +

lint: copyright
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

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
	rustup component add clippy rustfmt
