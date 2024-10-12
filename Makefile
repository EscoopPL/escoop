include config/config.mk

SOURCES_CXX=$(shell find src -name '*.cpp')
HEADERS_CXX=$(shell find src -name '*.hpp')
OBJ_CXX=$(subst cpp,o,$(subst src/,build/,$(SOURCES_CXX)))

.PHONEY: always clean run build softclean

.SILENT:

all: always $(OBJ_CXX) $(BIN_DIR)/esc install

build: always $(OBJ_CXX) $(BIN_DIR)/esc

$(BIN_DIR)/esc: $(OBJ_CXX)
	@echo Linking $(OBJ_CXX)
	$(CXX) -O2 -g $^ -o $@
	@echo $(OBJ_CXX) linked

build/%.o: src/%.cpp $(HEADERS)
	@echo Compiling $<, Producing $@
	$(CXX) -std=c++20 -c -g -o $@ $<
	@echo $< Compiled, $@ Produced

install: $(BIN_DIR)/esc
	cp $(BIN_DIR)/esc /usr/local/bin

always:
	clear
	mkdir -p $(BUILD_DIR)
	mkdir -p $(BIN_DIR)

clean: softclean
	rm -rf $(BUILD_DIR)

softclean:
	rm -rf $(BIN_DIR)

run: always $(BIN_DIR)/esc install
	esc tests/helloworld/runfiles/entrypoint.es