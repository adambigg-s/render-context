


#include <math.h>
#include <stdio.h>
#include <string.h>



#define HEIGHT 50
#define WIDTH 100
#define PI 3.14159

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

Vec3 add(Vec3 a, Vec3 b) {
    return (Vec3){ a.x + b.x, a.y + b.y, a.z + b.z };
}

Vec3 sub(Vec3 a, Vec3 b) {
    return (Vec3){ a.x - b.x, a.y - b.y, a.z - b.z };
}

Vec3 mul(Vec3 vec, float scalar) {
    return (Vec3){ vec.x * scalar, vec.y * scalar, vec.z * scalar };
}

int line_sphere(Line line, Sphere sphere, Vec3* intersection1, Vec3* intersection2) {
    Vec3 diff = sub(line.point, sphere.center);

    float a = dot(line.slope, line.slope);
    float b = 2 * dot(diff, line.slope);
    float c = dot(diff, diff) - sphere.rad * sphere.rad;

    float discrim = b * b - 4 * a * c;

    if (discrim < 0) return 0;

    float sqrt_discrim = sqrt(discrim);
    float t1 = (-b - sqrt_discrim) / (2 * a);
    float t2 = (-b + sqrt_discrim) / (2 * a);

    *intersection1 = add(line.point, mul(line.slope, t1));
    *intersection2 = add(line.point, mul(line.slope, t2));

    return discrim > 0 ? 2 : 1;
}

void clear(Buffer* buffer) {
    memset(buffer->visual, ' ', WIDTH * HEIGHT);
}

void display(Buffer buffer) {
    for (int y = 0; y < HEIGHT; y += 1) {
        for (int x = 0; x < WIDTH; x += 1) {
            putchar(buffer.visual[y * WIDTH + x]);
        }
        putchar('\n');
    }
}

void set(Buffer* buffer, int x, int y, char chr) {
    if (x >= 0 && x< WIDTH && y >= 0 && y < HEIGHT) {
        buffer->visual[y * WIDTH + x] = chr;
    }
}

void render_screen(Buffer* buffer, Sphere sphere, Vec3 camera) {
    float halfwidth = (float)WIDTH / 2;
    float halfheight = (float)HEIGHT / 2;
    float fov = 4;
    
    for (int x = 0; x < WIDTH; x += 1) {
        for (int y = 0; y < HEIGHT; y += 1) {

            float theta = (x - halfwidth) / halfwidth * PI / fov;
            float phi = (y - halfheight) / halfheight * PI / fov;

            Vec3 direction = {
                cos(theta) * cos(phi),
                sin(phi),
                sin(theta) * cos(phi),
            };

            Line ray = { camera, direction };
            
            Vec3 inter1, inter2;
            int hits = line_sphere(ray, sphere, &inter1, &inter2);

            if (hits > 0) {
                set(buffer, x, y, '#');
            }
        }
    }
}

int main() {
    Sphere sphere = (Sphere){ 12, (Vec3){ 0, 0, 0 } };
    Buffer buffer;
    Vec3 camera = { 20, 5, 0 };

    clear(&buffer);
    render_screen(&buffer, sphere, camera);
    display(buffer);
    clear(&buffer);

    return 0;
}
