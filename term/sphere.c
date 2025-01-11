


#include <math.h>
#include <stdio.h>
#include <string.h>



#define HEIGHT 70
#define WIDTH 120
#define PI 3.14159265

typedef struct Vec3 {
    float x, y, z;
} Vec3;

typedef struct Sphere {
    float rad;
    Vec3 center;
} Sphere;

typedef struct Line {
    Vec3 point;
    Vec3 slope;
} Line;

typedef struct Buffer {
    char visual[WIDTH * HEIGHT];
} Buffer;

float dot(Vec3 a, Vec3 b) {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

void normalize(Vec3* vec) {
    float length = sqrt(dot(*vec, *vec));
    vec->x /= length;
    vec->y /= length;
    vec->z /= length;
}

Vec3 add(Vec3 a, Vec3 b) {
    return (Vec3){ a.x + b.x, a.y + b.y, a.z + b.z };
}

Vec3 sub(Vec3 a, Vec3 b) {
    return (Vec3){ a.x - b.x, a.y - b.y, a.z - b.z };
}

Vec3 mul(Vec3 vec, float scalar) {
    return (Vec3){ vec.x * scalar, vec.y * scalar, vec.z * scalar };
}

int line_sphere(Line line, Sphere sphere, Vec3* intersection) {
    Vec3 diff = sub(line.point, sphere.center);

    float a = dot(line.slope, line.slope);
    float b = 2 * dot(diff, line.slope);
    float c = dot(diff, diff) - sphere.rad * sphere.rad;

    float discrim = b * b - 4 * a * c;
    if (discrim < 0) return 0;

    float sqrt_discrim = sqrt(discrim);
    float x = (-b + sqrt_discrim) / (2 * a);

    *intersection = add(line.point, mul(line.slope, x));

    return discrim > 0;
}

void clear(Buffer* buffer) {
    memset(buffer->visual, ' ', WIDTH * HEIGHT);
}

void display(Buffer* buffer) {
    printf("\x1b[H");
    char frame_buffer[WIDTH * HEIGHT + 1];
    frame_buffer[WIDTH * HEIGHT] = '\0';
    for (int i = 0; i < WIDTH * HEIGHT; i += 1) {
        if (i % WIDTH) {
            frame_buffer[i] = buffer->visual[i];
            continue;
        }
        frame_buffer[i] = '\n';
    }
    printf("%s", frame_buffer);
}

void set(Buffer* buffer, int x, int y, char chr) {
    if (x >= 0 && x< WIDTH && y >= 0 && y < HEIGHT) {
        buffer->visual[y * WIDTH + x] = chr;
    }
}

char luminosity_to_char(float lumin) {
    char chars[] = " .,;-=%@";
    return chars[(int)(lumin * (sizeof(chars)-1))];
}

void render_screen(Buffer* buffer, Sphere sphere, Vec3 camera, Vec3 light) {
    float halfwidth = (float)WIDTH / 2;
    float halfheight = (float)HEIGHT / 2;
    float fovx = 3;
    float fovy = 2;
    
    for (int x = 0; x < WIDTH; x += 1) {
        for (int y = 0; y < HEIGHT; y += 1) {

            float theta = (x - halfwidth) / halfwidth * PI / fovx;
            float phi = (y - halfheight) / halfheight * PI / fovy;

            Vec3 direction = {
                cos(theta) * cos(phi),
                sin(phi),
                sin(theta) * cos(phi),
            };

            Line ray = { camera, direction };
            
            Vec3 inter;
            int hits = line_sphere(ray, sphere, &inter);

            if (hits > 0) {
                Vec3 normal = {
                    inter.x - sphere.center.x,
                    inter.y - sphere.center.y,
                    inter.z - sphere.center.z
                };
                normalize(&normal);
                Vec3 lightsource = light;
                normalize(&lightsource);
                float luminosity = dot(normal, lightsource);
                if (luminosity < 0) luminosity = 0;

                char color = luminosity_to_char(luminosity);
                set(buffer, x, y, color);
            }
        }
    }
}

int main() {
    Sphere sphere = (Sphere){ 12, (Vec3){ 0, 0, 0 } };
    Buffer buffer;
    Vec3 camera = { 20, 0, 0 };
    Vec3 light = { 3, 2, 2 };

    // ansi escape to clear terminal
    printf("\x1b[2J");
    // ansi escape to make cursor-line invisible
    printf("\033[?25l");
    
    clear(&buffer);
    render_screen(&buffer, sphere, camera, light);
    display(&buffer);
    clear(&buffer);

    return 0;
}
