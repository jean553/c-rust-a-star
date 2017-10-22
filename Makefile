CC=gcc
SOURCEDIR=src
LIBS=-lSDL2

c_rust_a_star:
	$(CC) $(SOURCEDIR)/main.c -o c_rust_a_star $(LIBS)
