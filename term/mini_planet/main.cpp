
#include <cmath>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

const float PI = 3.14159265;
const float TAU = 2 * PI;

class Color {
public:
    int red;
    int green;
    int blue;

    static Color cons(int r, int g, int b) {
        return Color { r, g, b };
    }

    void darken(float lighting) {
        red = (int)(red * lighting) - 1;
        green = (int)(green * lighting) - 1;
        blue = (int)(blue * lighting) - 1;
    }

    string to_ansi_back() {
        return "\x1b[48;2;" + to_string(red) + ";" + to_string(green)
               + ";" + to_string(blue) + "m" + ' ';
    }
};

class Buffer {
public:
    int height;
    int width;
    vector<Color> pixels;
    vector<float> depth;

    static Buffer cons(int height, int width) {
        return Buffer {
            height,
            width,
            vector<Color>(width * height, Color::cons(0, 0, 0)),
            vector<float>(width * height, 1E9),
        };
    }

    void set(int x, int y, Color color, float depth) {
        int idx = this->idx(x, y);
        this->pixels[idx] = color;
        this->depth[idx] = depth;
    }

    void clear() {
        pixels.assign(width * height, Color::cons(0, 0, 0));
        depth.assign(width * height, 1E9);
    }

    const float get_depth(int x, int y) {
        int idx = this->idx(x, y);
        return depth[idx];
    }

    const int halfheight() {
        return height / 2;
    }

    const int halfwidth() {
        return width / 2;
    }
    
    const bool inbounds(int x, int y) {
        return x < width && y < height;
    }

    const int idx(int x, int y) {
        return y * width + x;
    }
};

class Vec3 {
public:
    float x; float y; float z;

    static Vec3 cons(float x, float y, float z) {
        return Vec3 { x, y, z };
    }

    void rotatez(float angle) {
        float x = this->x; float y = this->y; float z = this->z;
        float sint = sin(angle);
        float cost = cos(angle);
        this->x = x * cost - y * sint;
        this->y = x * sint + y * cost;
        this->z = z;
    }

    void normalize() {
        float length = sqrt(this->inner_prod(this));
        x /= length; y /= length; z /= length;
    }

    const float inner_prod(Vec3* other) {
        return x * other->x + y * other->y + z * other->z;
    }
};

class Planet {
public:
    float radius;
    Vec3 position;
    vector<Color> texture;
};

class Renderer {
public:
    Planet* planet;
    Buffer* buffer;
    Vec3 camera;
    Vec3 lightsource;
    string frame_buffer;

    void render_planet() {
        float dphi = 0.03;
        float dtheta = 0.5 * dphi;
        for (float phi = -PI; phi < PI; phi += dphi) {
            for (float theta = 0; theta < TAU; theta += dtheta) {
                float worldx = planet->radius * sin(phi) * cos(theta);
                float worldy = planet->radius * sin(phi) * sin(theta);
                float worldz = planet->radius * cos(phi);

                Vec3 world = Vec3::cons(worldx, worldy, worldz);
                Vec3 normal = world;
                normal.normalize();
                world.x += planet->position.x;
                world.y += planet->position.y;
                world.z += planet->position.z;
                world.x -= camera.x;
                
                if (world.x <= 0) {
                    continue;
                }

                float scalex = 100;
                float scaley = scalex * 0.5;
                float screenx = (int)(world.y / world.x * scalex + buffer->halfwidth());
                float screeny = (int)(-world.z / world.x * scaley + buffer->halfheight());

                if (buffer->inbounds(screenx, screeny)) {
                    if (world.x < buffer->get_depth(screenx, screeny)) {
                        Color color = Color::cons(253, 253, 253);
                        float lighting = normal.inner_prod(&lightsource);
                        color.darken(lighting);
                        buffer->set(screenx, screeny, color, world.x);
                    }
                }
            }
        }
    }

    void display_to_screen() {
        cout << "\x1b[H";
        frame_buffer.clear();
        for (int y = 0; y < buffer->height; y += 1) {
            for (int x = 0; x < buffer->width; x += 1) {
                int idx = buffer->idx(x,y);
                frame_buffer += buffer->pixels[idx].to_ansi_back();
            }
            frame_buffer += "\x1b[0m\n";
        }
        cout << frame_buffer;
        cout.flush();
    }
};

int main() {
    Buffer buffer = Buffer::cons(70, 200);
    Planet planet = Planet { 25, Vec3::cons(0, 0, 0) };
    Vec3 camera_pos = Vec3::cons(-70, 0, 0);
    Vec3 lighting = Vec3::cons(1, 1, 1);
    lighting.normalize();
    
    Renderer renderer = Renderer { &planet, &buffer, camera_pos, lighting };

    while (true) {
        buffer.clear();
        renderer.lightsource.rotatez(0.1);
        renderer.render_planet();
        renderer.display_to_screen();
    }
    
    return 0;
}
