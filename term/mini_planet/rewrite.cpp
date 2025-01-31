


#include <iostream>
#include <string>
#include <vector>



using namespace std;

constexpr float PI = 3.1415926;
constexpr float TAU = 2 * PI;

struct Vec3 {
    float x, y, z;

    static Vec3 cons(float x, float y, float z) {
        return Vec3 { x, y, z };
    }

    const float innner_prod(Vec3* other) {
        return this->x * other->x + this->y * other->y + this->z * other->z;
    }

    void normalize() {
        float length = sqrt(this->innner_prod(this));
        x /= length; y /= length; z /= length;
    }

    void rotatex(float angle) {
        float x = this->x, y = this->y, z = this->z;
        float sint = sin(angle), cost = cos(angle);
        this->x = x;
        this->y = y * cost - z * sint;
        this->z = y * sint + z * cost;
    }

    void rotatey(float angle) {
        float x = this->x, y = this->y, z = this->z;
        float sint = sin(angle), cost = cos(angle);
        this->x = x * cost + z * sint;
        this->y = y;
        this->z = -x * sint + z * cost;
    }

    void rotatez(float angle) {
        float x = this->x, y = this->y, z = this->z;
        float sint = sin(angle), cost = cos(angle);
        this->x = x * cost + y * sint;
        this->y = x * sint + y * cost;
        this->z = z;
    }
};

struct Color {
    int red, green, blue;

    static Color cons(int r, int g, int b) {
        return Color { min(r, 255), min(g, 255), min(b, 255) };
    }

    const string to_ansi_back() {
        return "\x1b;48;2;" + to_string(red) + ";"
               + to_string(green) + ";" + to_string(blue) + "m ";
    }
};

struct Buffer {
    int height, width;
    vector<Color> pixels;
    vector<float> depth;

    static Buffer cons(int height, int width) {
        return Buffer { height, width,
            vector<Color>(height * width, Color::cons(0, 0, 0)),
            vector<float>(height * width, 1E9), 
        };
    }

    void set(int x, int y, Color color, float depth) {
        if (this->inbounds(x, y)) {
            int idx = this->idx(x, y);
            this->pixels[idx] = color;
            this->depth[idx] = depth;
        }
    }

    void clear() {
        pixels.assign(width * height, Color::cons(0, 0, 0));
        depth.assign(width * height, 1E9);
    }
    
    const int idx(int x, int y) {
        return y * width + x;
    }

    const bool inbounds(int x, int y) {
        return x >= 0 && x < width && y >= 0 && y < height;
    }
};

struct Planet {
    float radius;
    Vec3 center;
};

struct Renderer {
    Buffer& buffer;
    Planet& planet;
    Vec3 camera;
    Vec3 lightsource;

    void render_planet() {
        float dphi = 0.1;
        float dtheta = 0.5 * dphi;
        for (float phi = 0; phi < PI; phi += dphi) {
            for (float theta = 0; theta < TAU; theta += dtheta) {
                // spherical to cartesian
                float worldx = planet.radius * sin(theta) * sin(phi);
                float worldy = planet.radius * sin(theta) * sin(phi);
                float worldz = planet.radius * cos(theta);
                Vec3 world = Vec3::cons(worldx, worldy, worldz);

                // world to view
                world.x += planet.center.x;
                world.y += planet.center.y;
                world.z += planet.center.z;
                world.x -= camera.x;
                world.y -= camera.y;
                world.z -= camera.z;

                float screenx = (int)(world.y / world.x + (float)buffer.width / 2);
                float screeny = (int)(world.z / world.x + (float)buffer.height/ 2);

                if (buffer.inbounds(screenx, screeny)) {
                    
                }
            }
        }
    }
};

int main() {
    cout << "is working?";
    return 0;
}
