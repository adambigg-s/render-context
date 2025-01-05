


#include <math.h>
#include <stdio.h>
#include <string.h>
#include <windows.h>



#define WIDTH 200
#define HEIGHT 60
#define CUBEWIDTH 30
#define SCALING_X 250
#define SCALING_Y 150
#define DELTA 1.2
#define FRAME_DELAY 5
#define CAMDISTANCE 350
#define ROT_DELTA_X 0.01
#define ROT_DELTA_Y 0.04
#define ROT_DELTA_Z 0.005


const float LIGHT_SOURCE[] = {1, 0.5, -0.5};
const char LIGHT_GRAD[] = ".`^',:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

float dot_prod(float ax, float ay, float az, float bx, float by, float bz);
void norm(float* x, float* y, float* z);

typedef struct Cube {
    float sidelen;
    float a, b, c;
    float buffer_z[WIDTH * HEIGHT];
    char buffer_screen[WIDTH * HEIGHT];
} Cube;

float euler_rotate_u(int i, int j, int k, Cube* cube) {
    float a = cube->a, b = cube->b, c = cube->c;
    return i * cos(b) * cos(c)
           + j * (sin(a) * sin(b) * cos(c)
                  - cos(a) * sin(c))
           + k * (cos(a) * sin(b) * cos(c)
                  + sin(a) * sin(c));
}

float euler_rotate_v(int i, int j, int k, Cube* cube) {
    float a = cube->a, b = cube->b, c = cube->c;
    return i * cos(b) * sin(c)
           + j * (sin(a) * sin(b) * sin(c)
                  + cos(a) * cos(c))
           + k * (cos(a) * sin(b) * sin(c)
                  - sin(a) * cos(c));
}

float euler_rotate_w(int i, int j, int k, Cube* cube) {
    float a = cube->a, b = cube->b, c = cube->c;
    return i * (-sin(b))
           + j * sin(a) * cos(b)
           + k * cos(a) * cos(b);
}

float dot_prod(float ax, float ay, float az, float bx, float by, float bz) {
    return ax * bx + ay * by + az * bz;
}

void norm(float* x, float* y, float* z) {
    float length = sqrt(dot_prod(*x, *y, *z, *x, *y, *z));
    if (length > 0) {
        *x /= length;
        *y /= length;
        *z /= length;
    }
}

void rotate_normal(float nx, float ny, float nz, float* nxp, float* nyp, float* nzp, Cube* cube) {
    *nxp = euler_rotate_u(nx, ny, nz, cube);
    *nyp = euler_rotate_v(nx, ny, nz, cube);
    *nzp = euler_rotate_w(nx, ny, nz, cube);
}

void calculate_lighting(float nx, float ny, float nz, char* chr) {
    float lx = LIGHT_SOURCE[0], ly = LIGHT_SOURCE[1], lz = LIGHT_SOURCE[2];
    norm(&lx, &ly, &lz);
    float intensity = dot_prod(nx, ny, nz, lx, ly, lz);

    if (intensity < 0) intensity = 0;

    int idx = (int)(intensity * (strlen(LIGHT_GRAD) - 1));
    *chr = LIGHT_GRAD[idx];
}

void calculate_cube_surface(float cubex, float cubey, float cubez,
                            float normx, float normy, float normz,
                            Cube* cube) {
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

        float nx, ny, nz;
        rotate_normal(normx, normy, normz, &nx, &ny, &nz, cube);
        char chr;
        calculate_lighting(nx, ny, nz, &chr);
        cube->buffer_screen[idx] = chr;
    }
}

void clear_buffers(Cube* cube) {
    memset(cube->buffer_screen, ' ', sizeof(cube->buffer_screen));
    memset(cube->buffer_z, 0, sizeof(cube->buffer_z));
}

void render_cube(Cube* cube) {
    for (float cubex = -cube->sidelen; cubex <= cube->sidelen; cubex += DELTA) {
        for (float cubey = -cube->sidelen; cubey <= cube->sidelen; cubey += DELTA) {
            calculate_cube_surface(cubex, cubey, -cube->sidelen, 0, 0, -1, cube);
            calculate_cube_surface(cube->sidelen, cubey, cubex, 1, 0, 0, cube);
            calculate_cube_surface(-cube->sidelen, cubey, cubex, -1, 0, 0, cube);
            calculate_cube_surface(-cubex, cubey, cube->sidelen, 0, 0, 1, cube);
            calculate_cube_surface(cubex, -cube->sidelen, -cubey, 0, -1, 0, cube);
            calculate_cube_surface(cubex, cube->sidelen, cubey, 0, 1, 0, cube);
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

void rotate_cube(Cube* cube) {
    cube->a += ROT_DELTA_X;
    cube->b += ROT_DELTA_Y;
    cube->c += ROT_DELTA_Z;
}

int main() {
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

