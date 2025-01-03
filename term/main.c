


#include <math.h>
#include <stdio.h>
#include <string.h>
#include <windows.h>



#define WIDTH 88
#define HEIGHT 44
#define CUBEWIDTH 15
#define SCALING_CONSTANT 120
#define DELTA 1.2
#define FRAME_DELAY 2
#define CAMDISTANCE 175
#define ROT_DELTA_X 0.1
#define ROT_DELTA_Y 0.05
#define ROT_DELTA_Z 0.02



typedef struct Cube
{
  float width;
  float a, b, c;
  float buffer_z[WIDTH * HEIGHT];
  char buffer_screen[WIDTH * HEIGHT];
} Cube;

float euler_rotate_u(int i, int j, int k, Cube *cube)
{
    float a = cube->a, b = cube->b, c = cube->c;
    return i * cos(b) * cos(c)
         + j * (sin(a) * sin(b) * cos(c) - cos(a) * sin(c))
         + k * (cos(a) * sin(b) * cos(c) + sin(a) * sin(c));
}

float euler_rotate_v(int i, int j, int k, Cube *cube)
{
    float a = cube->a, b = cube->b, c = cube->c;
    return i * cos(b) * sin(c)
         + j * (sin(a) * sin(b) * sin(c) + cos(a) * cos(c))
         + k * (cos(a) * sin(b) * sin(c) - sin(a) * cos(c));
}

float euler_rotate_w(int i, int j, int k, Cube *cube)
{
    float a = cube->a, b = cube->b, c = cube->c;
    return i * (-sin(b))
         + j * sin(a) * cos(b)
         + k * cos(a) * cos(b);
}

char int_to_chr(int idx)
{
  char chrs[] = {'.', '%', '/', ':', '*', '!'};
  return chrs[idx];
}

void calculate_cube_surface(float cubex, float cubey, float cubez, char chr, Cube *cube)
{
  float x = euler_rotate_u(cubex, cubey, cubez, cube);
  float y = euler_rotate_v(cubex, cubey, cubez, cube);
  float z = euler_rotate_w(cubex, cubey, cubez, cube) + CAMDISTANCE;

  if (z <= 0) return;

  float ooz = 1 / z * SCALING_CONSTANT;
  int xp = (int)((float)WIDTH / 2 + ooz * x);
  int yp = (int)((float)HEIGHT / 2 - ooz * y);

  int idx = yp * WIDTH + xp;
  if (idx >= 0 && idx < WIDTH * HEIGHT) {
    if (ooz > cube->buffer_z[idx]) {
      cube->buffer_z[idx] = ooz;
      cube->buffer_screen[idx] = chr;
    }
  }
}

void render_cube(Cube *cube)
{
  for (float cubex = -cube->width; cubex <= cube->width; cubex += DELTA) {
    for (float cubey = -cube->width; cubey <= cube->width; cubey += DELTA) {
        calculate_cube_surface(cubex, cubey, -cube->width, int_to_chr(0), cube);
        calculate_cube_surface(cube->width, cubey, cubex, int_to_chr(1), cube);
        calculate_cube_surface(-cube->width, cubey, cubex, int_to_chr(2), cube);
        calculate_cube_surface(-cubex, cubey, cube->width, int_to_chr(3), cube);
        calculate_cube_surface(cubex, -cube->width, -cubey, int_to_chr(4), cube);
        calculate_cube_surface(cubex, cube->width, cubey, int_to_chr(5), cube);
    }
  }
}

void clear_buffers(Cube *cube)
{
  memset(cube->buffer_screen, ' ', sizeof(cube->buffer_screen));
  memset(cube->buffer_z, 0, sizeof(cube->buffer_z));
}

void display_frame(Cube *cube)
{
  printf("\x1b[H");
  for (int i = 0; i < WIDTH * HEIGHT; i += 1) {
    char chr = '\n';
    if (i % WIDTH) {
      chr = cube->buffer_screen[i];
    }
    putchar(chr);
  }
}

int main()
{
  Cube cube = {
    .width = CUBEWIDTH,
    .a = 0,
    .b = 0,
    .c = 0,
  };

  printf("\x1b[2J");

  while (1 == 1) {
    clear_buffers(&cube);
    render_cube(&cube);
    display_frame(&cube);

    cube.a += ROT_DELTA_X;
    cube.b += ROT_DELTA_Y;
    cube.c += ROT_DELTA_Z;

    Sleep(FRAME_DELAY);
  }
}

