# Copyright (C) 2026 The Delta-V Authors
# SPDX-License-Identifier: Apache-2.0

.DEFAULT_GOAL := help

CARGO ?= cargo
MVN   ?= mvn
BUF   ?= buf

MVNFLAGS   ?= -B --no-transfer-progress
CARGOFLAGS ?= --locked

# Reference the schemas are checked against for backwards compatibility. Resolved
# from the local clone so the check needs no network access or repo credentials.
# The '#' must stay escaped: unescaped it starts a make comment, which silently
# truncates this to '.git' and makes buf compare HEAD against itself.
BUF_AGAINST ?= .git\#ref=origin/main

.PHONY: help
help: ## List available targets
	@grep -hE '^[a-zA-Z0-9_-]+:.*?## ' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2}'

.PHONY: verify
verify: lint build test ## Run the full quality gate (lint + build + test)

.PHONY: lint
lint: lint-proto lint-rust ## Run every linter

.PHONY: lint-proto
lint-proto: ## Lint the Protobuf schemas
	$(BUF) lint

.PHONY: lint-rust
lint-rust: ## Check Rust formatting and run clippy
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: breaking
breaking: ## Check the schemas for breaking changes against $(BUF_AGAINST)
	$(BUF) breaking --against "$(BUF_AGAINST)"

.PHONY: build
build: build-java build-rust ## Build the Java and Rust artifacts

.PHONY: build-java
build-java: ## Compile the Java stubs
	$(MVN) $(MVNFLAGS) clean compile

.PHONY: build-rust
build-rust: ## Build the Rust crate
	$(CARGO) build $(CARGOFLAGS) --all-targets

.PHONY: package
package: ## Build the release artifacts (JAR + sources JAR)
	$(MVN) $(MVNFLAGS) clean package

.PHONY: test
test: test-java test-rust ## Run the Java and Rust test suites

.PHONY: test-java
test-java: ## Run the Java tests
	$(MVN) $(MVNFLAGS) test

.PHONY: test-rust
test-rust: ## Run the Rust tests
	$(CARGO) test $(CARGOFLAGS)

.PHONY: generate
generate: ## Generate Java and Go stubs into gen/ via buf
	$(BUF) generate

.PHONY: clean
clean: ## Remove all build output
	$(MVN) $(MVNFLAGS) clean
	$(CARGO) clean
	rm -rf gen
