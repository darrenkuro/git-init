BIN_NAME	:= gitinit
TARGET		:= target/release/$(BIN_NAME)
BIN_DIR		:= $(or $(CARGO_HOME),$(HOME)/.cargo)/bin
DEST		:= $(BIN_DIR)/$(BIN_NAME)

.DEFAULT_GOAL := install

.PHONY: build
build:
	@cargo build --release 2>/dev/null

.PHONY: install
install: build
	@echo "📦 Installing $(BIN_NAME) to $(DEST)"
	@ln -sf "$(realpath $(TARGET))" "$(DEST)"

