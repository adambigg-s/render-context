


#include <math.h>
#include <stdio.h>



typedef struct Vec3 {
    float x, y, z;
} Vec3;

Vec3 Vec3Cons(float x, float y, float z) {
    Vec3 output = { x, y, z };
    return output;
}

void rotatex(Vec3* target, float a) {
    float x = target->x, y = target->y, z = target->z;
    target->x = x;
    target->y = y * cos(a) - z * sin(a);
    target->z = y * sin(a) + z * cos(a);
}

void rotatey(Vec3* target, float b) {
    float x = target->x, y = target->y, z = target->z;
    target->x = x * cos(b) + z * sin(b);
    target->y = y;
    target->z = -x * sin(b) + z * cos(b);
}

void rotatez(Vec3* target, float c) {
    float x = target->x, y = target->y, z = target->z;
    target->x = x * cos(c) - y * sin(c);
    target->y = x * sin(c) + y * cos(c);
    target->z = z;
}

void rotationchain(Vec3* target, Vec3 angles) {
    rotatex(target, angles.x);
    rotatey(target, angles.y);
    rotatez(target, angles.z);
}

float dot(Vec3 a, Vec3 b) {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

void normalize(Vec3* target) {
    float length = sqrt(dot(*target, *target));
    target->x /= length;
    target->y /= length;
    target->z /= length;
}

typedef struct ViewModel {
    Vec3 pos;
    float rot;
    float tilt;
} Viewmodel;

int main() {
    return 0;
}
