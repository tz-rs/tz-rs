PORT = 8090
BIN = florencenet.sh

LINT_COMMAND = cargo clippy
LINT_FLAGS = --all -- -D warnings

start:
	./$(BIN) start --rpc-port $(PORT)
stop:
	./$(BIN) stop
lint:
	$(LINT_COMMAND) $(LINT_FLAGS)
