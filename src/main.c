#include <stdio.h>
#include <stdlib.h>

#include <SDL2/SDL.h>

#define NODE_DIMENSION 50
#define MAX_WIDTH_OR_HEIGHT 100

/**
 *
 */
int main(int argc, char* argv[]) {

    if (argc != 5) {
        printf("Unexpected parameters amount.");
        return 1;
    }

    int width = atoi(argv[1]);
    int height = atoi(argv[2]);

    if (width > MAX_WIDTH_OR_HEIGHT || height > MAX_WIDTH_OR_HEIGHT) {
        printf("The width or the height cannot exceed 100.");
        return 1;
    }

    SDL_Window* window = SDL_CreateWindow(
        "C-Rust A* example",
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        width * NODE_DIMENSION,
        height * NODE_DIMENSION,
        SDL_WINDOW_OPENGL
    );

    if (window == NULL) {
        printf("The window cannot be created.");
        return 1;
    }

    SDL_Event event;
    unsigned short run = 1;

    while(run) {

        SDL_PollEvent(&event);

        if (event.type == SDL_QUIT) {
            run = 0;
        }
    }

    SDL_DestroyWindow(window);

    SDL_Quit();
    return 0;
}
