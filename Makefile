RUSTC=rustc
TARGET=bin/main

MAIN_FILE ?= 
INPUT_FILE ?= 

build: $(TARGET)

$(TARGET): $(MAIN_FILE)
	@mkdir -p bin
	$(RUSTC) $(MAIN_FILE) -o $(TARGET)

run:
	@./$(TARGET) $(INPUT_FILE)