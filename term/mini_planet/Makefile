
CXX = g++
DEBUG_FLAGS = -g
RELEASE_FLAGS = -O2

SRC = main.cpp
DEBUG_OBJ = main_debug.o
RELEASE_OBJ = main_release.o
DEBUG_EXE = main_debug.exe
RELEASE_EXE = main_release.exe

.PHONY: all clean run release

all: $(DEBUG_EXE)

$(DEBUG_EXE): $(DEBUG_OBJ)
	$(CXX) $(DEBUG_OBJ) -o $(DEBUG_EXE)

$(DEBUG_OBJ): $(SRC)
	$(CXX) $(DEBUG_FLAGS) -c $(SRC) -o $(DEBUG_OBJ)

release: $(RELEASE_EXE)

$(RELEASE_EXE): $(RELEASE_OBJ)
	$(CXX) $(RELEASE_OBJ) -o $(RELEASE_EXE)

$(RELEASE_OBJ): $(SRC)
	$(CXX) $(RELEASE_FLAGS) -c $(SRC) -o $(RELEASE_OBJ)

clean:
	del /f $(DEBUG_OBJ) $(RELEASE_OBJ) $(DEBUG_EXE) $(RELEASE_EXE)

run: $(DEBUG_EXE)
	$(DEBUG_EXE)

release-run: $(RELEASE_EXE)
	$(RELEASE_EXE)

