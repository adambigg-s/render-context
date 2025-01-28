CXX = g++
CXXFLAGS = -std=c++11 -Wall -I./include

SRC = main.cpp
OBJ = main.o
EXE = main.exe

LIBRARY_PATH = ./lib
LIBS = -lraylib -lopengl32 -lgdi32 -lwinmm

$(EXE): $(OBJ)
	$(CXX) $(OBJ) -o $(EXE) -L$(LIBRARY_PATH) $(LIBS)

$(OBJ): $(SRC)
	$(CXX) $(CXXFLAGS) -c $(SRC)

clean:
	del /f $(OBJ) $(EXE)

run:
	$(EXE)

