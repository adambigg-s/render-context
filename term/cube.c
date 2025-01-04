


#include <math.h>
#include <stdio.h>
#include <string.h>
#include <windows.h>



#define WIDTH 200
#define HEIGHT 60
#define CUBEWIDTH 25
#define SCALING_X 175
#define SCALING_Y 100
#define DELTA 0.9
#define FRAME_DELAY 10
#define CAMDISTANCE 200
#define ROT_DELTA_X 0.01
#define ROT_DELTA_Y 0.04
#define ROT_DELTA_Z 0.005



typedef struct Cube
{
    float sidelen;
    float a, b, c;
    float buffer_z[WIDTH * HEIGHT];
    char buffer_screen[WIDTH * HEIGHT];
} Cube;

float euler_rotate_u(int i, int j, int k, Cube* cube)
{
    float a = cube->a, b = cube->b, c = cube->c;
    return i * cos(b) * cos(c)
           + j * (sin(a) * sin(b) * cos(c)
                  - cos(a) * sin(c))
           + k * (cos(a) * sin(b) * cos(c)
                  + sin(a) * sin(c));
}

float euler_rotate_v(int i, int j, int k, Cube* cube)
{
    float a = cube->a, b = cube->b, c = cube->c;
    return i * cos(b) * sin(c)
           + j * (sin(a) * sin(b) * sin(c)
                  + cos(a) * cos(c))
           + k * (cos(a) * sin(b) * sin(c)
                  - sin(a) * cos(c));
}

float euler_rotate_w(int i, int j, int k, Cube* cube)
{
    float a = cube->a, b = cube->b, c = cube->c;
    return i * (-sin(b))
           + j * sin(a) * cos(b)
           + k * cos(a) * cos(b);
}

char intchr(int idx) {
    char chrs[] = {' ', '$', ',', '!', '~', '>', '|'};
    return chrs[idx];
}

void calculate_cube_surface(float cubex, float cubey, float cubez, char chr, Cube* cube) {
    float x = euler_rotate_u(cubex, cubey, cubez, cube);
    float y = euler_rotate_v(cubex, cubey, cubez, cube);
    float z = euler_rotate_w(cubex, cubey, cubez, cube) + CAMDISTANCE;

    float inv_z = 1 / z;
    float depth_multiplier_x = SCALING_X * inv_z;
    float depth_multiplier_y = SCALING_Y * inv_z;
    int xp = (int)((float)WIDTH / 2 + depth_multiplier_x * x);
    int yp = (int)((float)HEIGHT / 2 - depth_multiplier_y * y);

     int idx = yp * WIDTH + xp;
     if (z <= 0) return;
     if (idx >= 0 && idx < WIDTH * HEIGHT && inv_z > cube->buffer_z[idx]) {
         cube->buffer_z[idx] = inv_z;
         cube->buffer_screen[idx] = chr;
     }
}

void clear_buffers(Cube* cube) {
    memset(cube->buffer_screen, intchr(0), sizeof(cube->buffer_screen));
    memset(cube->buffer_z, 0, sizeof(cube->buffer_z));
}

void render_cube(Cube* cube) {
    for (float cubex = -cube->sidelen; cubex <= cube->sidelen; cubex += DELTA) {
        for (float cubey = -cube->sidelen; cubey <= cube->sidelen; cubey += DELTA) {
            calculate_cube_surface(cubex, cubey, -cube->sidelen, intchr(1), cube);
            calculate_cube_surface(cube->sidelen, cubey, cubex, intchr(2), cube);
            calculate_cube_surface(-cube->sidelen, cubey, cubex, intchr(3), cube);
            calculate_cube_surface(-cubex, cubey, cube->sidelen, intchr(4), cube);
            calculate_cube_surface(cubex, -cube->sidelen, -cubey, intchr(5), cube);
            calculate_cube_surface(cubex, cube->sidelen, cubey, intchr(6), cube);
        }
    }
}

void display_frame(Cube* cube) {
    printf("\x1b[H");
    char frame_buffer[WIDTH * HEIGHT + 1];
    frame_buffer[WIDTH * HEIGHT] = '\0';
    for (int i = 0; i < WIDTH * HEIGHT; i += 1) {
        if (i % WIDTH) {
            frame_buffer[i] = cube->buffer_screen[i];
            continue;
        }
        frame_buffer[i] = '\n';
    }
    printf("%s", frame_buffer);
}

void rotate_cube(Cube* cube)
{
    cube->a += ROT_DELTA_X;
    cube->b += ROT_DELTA_Y;
    cube->c += ROT_DELTA_Z;
}

int main()
{
    Cube cube = {
        .sidelen = CUBEWIDTH,
        .a = 0,
        .b = 0,
        .c = 0,
    };

    // ansi escape to clear terminal
    printf("\x1b[2J");
    // ansi escape to make cursor-line invisible
    printf("\033[?25l");

     while (1 == 1) {
         clear_buffers(&cube);
         render_cube(&cube);
         display_frame(&cube);
         rotate_cube(&cube);

         Sleep(FRAME_DELAY);
     }
}
