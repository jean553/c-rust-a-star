#include <stdio.h>
#include <stdlib.h>

#include <SDL2/SDL.h>

/**
 *
 */
int main(int argc, char* argv[]) {

    if (argc != 5) {
        printf("Unexpected parameters amount.");
        return 1;
    }

    SDL_Window* window = SDL_CreateWindow(
        "C-Rust A* example",
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        640,
        480,
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
