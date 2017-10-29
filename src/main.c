#include <stdio.h>
#include <stdlib.h>

#include <SDL2/SDL.h>

#define RENDERING_DRIVER_INDEX -1

#define NODE_DIMENSION 50
#define MAX_WIDTH_OR_HEIGHT 10
#define PATH_MAX_LENGTH 100
#define PATH_DEFAULT_VALUE 150

#define GREEN_COLOR_RED_AMOUNT 0
#define GREEN_COLOR_GREEN_AMOUNT 255
#define GREEN_COLOR_BLUE_AMOUNT 0

#define RED_COLOR_RED_AMOUNT 255
#define RED_COLOR_GREEN_AMOUNT 0
#define RED_COLOR_BLUE_AMOUNT 0

#define BLUE_COLOR_RED_AMOUNT 0
#define BLUE_COLOR_GREEN_AMOUNT 0
#define BLUE_COLOR_BLUE_AMOUNT 255

#define GREY_COLOR_RED_AMOUNT 192
#define GREY_COLOR_GREEN_AMOUNT 192
#define GREY_COLOR_BLUE_AMOUNT 192

#define COLORS_OPACITY 255

typedef struct Positions {
    uint8_t horizontal;
    uint8_t vertical;
} Positions;

extern void get_path(
    uint8_t* path,
    uint8_t width,
    uint8_t height,
    uint8_t departure,
    uint8_t arrival
);

extern struct Positions get_positions(
    uint8_t width,
    uint8_t index
);

/**
 *
 */
int main(int argc, char* argv[]) {

    if (argc != 5) {
        printf("Unexpected parameters amount.");
        return 1;
    }

    unsigned int width = atoi(argv[1]);
    unsigned int height = atoi(argv[2]);
    unsigned int departure = atoi(argv[3]);
    unsigned int arrival = atoi(argv[4]);

    if (width > MAX_WIDTH_OR_HEIGHT || height > MAX_WIDTH_OR_HEIGHT) {
        printf("The width or the height must be between 0 and 10.");
        return 1;
    }

    uint8_t path[PATH_MAX_LENGTH];
    for (int i = 0; i < PATH_MAX_LENGTH; i += 1) {
        path[i] = PATH_DEFAULT_VALUE;
    }

    get_path(
        path,
        width,
        height,
        departure,
        arrival
    );

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

    SDL_Surface* screen = SDL_GetWindowSurface(window);

    if (screen == NULL) {
        printf("The screen cannot be created.");
        return 1;
    }

    SDL_FillRect(
        screen,
        NULL,
        SDL_MapRGB(screen->format, 0, 0, 0)
    );

    SDL_Rect departure_rectangle;
    SDL_Rect arrival_rectangle;

    struct Positions departure_position = get_positions(
        width,
        departure
    );

    struct Positions arrival_position = get_positions(
        width,
        arrival
    );

    departure_rectangle.x = departure_position.horizontal * NODE_DIMENSION;
    departure_rectangle.y = departure_position.vertical * NODE_DIMENSION;
    departure_rectangle.w = NODE_DIMENSION;
    departure_rectangle.h = NODE_DIMENSION;

    arrival_rectangle.x = arrival_position.horizontal * NODE_DIMENSION;
    arrival_rectangle.y = arrival_position.vertical * NODE_DIMENSION;
    arrival_rectangle.w = NODE_DIMENSION;
    arrival_rectangle.h = NODE_DIMENSION;

    SDL_Renderer* renderer = SDL_CreateRenderer(
        window,
        RENDERING_DRIVER_INDEX,
        SDL_RENDERER_ACCELERATED
    );

    SDL_SetRenderDrawColor(
        renderer,
        GREEN_COLOR_RED_AMOUNT,
        GREEN_COLOR_GREEN_AMOUNT,
        GREEN_COLOR_BLUE_AMOUNT,
        COLORS_OPACITY
    );

    SDL_RenderFillRect(
        renderer,
        &departure_rectangle
    );

    SDL_SetRenderDrawColor(
        renderer,
        RED_COLOR_RED_AMOUNT,
        RED_COLOR_GREEN_AMOUNT,
        RED_COLOR_BLUE_AMOUNT,
        COLORS_OPACITY
    );

    SDL_RenderFillRect(
        renderer,
        &arrival_rectangle
    );

    int current_index = 0;
    int nodes_amount = 0;

    while(path[nodes_amount] != arrival) {
        nodes_amount += 1;
    }

    SDL_SetRenderDrawColor(
        renderer,
        BLUE_COLOR_RED_AMOUNT,
        BLUE_COLOR_GREEN_AMOUNT,
        BLUE_COLOR_BLUE_AMOUNT,
        COLORS_OPACITY
    );

    SDL_Rect* path_nodes = malloc(sizeof(SDL_Rect) * nodes_amount);
    Positions* nodes_positions = malloc(sizeof(Positions) * nodes_amount);

    for (int i = 0; i < nodes_amount; i += 1) {

        nodes_positions[i] = get_positions(
            width,
            path[i]
        );

        path_nodes[i].x = nodes_positions[i].horizontal * NODE_DIMENSION;
        path_nodes[i].y = nodes_positions[i].vertical * NODE_DIMENSION;
        path_nodes[i].w = NODE_DIMENSION;
        path_nodes[i].h = NODE_DIMENSION;

        SDL_RenderFillRect(
            renderer,
            &path_nodes[i]
        );
    }

    SDL_Event event;
    unsigned short run = 1;

    SDL_UpdateWindowSurface(window);

    while(run) {

        SDL_RenderPresent(renderer);

        SDL_PollEvent(&event);

        if (event.type == SDL_QUIT) {
            run = 0;
        }
    }

    SDL_DestroyWindow(window);

    SDL_Quit();
    return 0;
}
