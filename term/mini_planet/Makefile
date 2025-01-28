CXX = g++
CXXFLAGS = -std=c++11 -Wall -I./include

SRC = main.cpp
OBJ = main.o
EXE = main.exe

LIBRARY_PATH =
LIBS = 

$(EXE): $(OBJ)
	$(CXX) $(OBJ) -o $(EXE)

$(OBJ): $(SRC)
	$(CXX) $(CXXFLAGS) -c $(SRC)

clean:
	del /f $(OBJ) $(EXE)

run:
	$(EXE)

