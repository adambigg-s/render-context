
#include <cmath>
#include <cstddef>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

const float PI = 3.14159265;
const float TAU = 2 * PI;
const string EARTHPATH = "earth_map.txt";

class Color {
public:
    int red, green, blue;

    static Color cons(int r, int g, int b) {
        return Color { r, g, b };
    }

    void darken(float lighting) {
        if (lighting < 0.1) {
            lighting = 0.1;
        }
        red = (int)(red * lighting);
        green = (int)(green * lighting);
        blue = (int)(blue * lighting);
    }

    string to_ansi_back() {
        if (red == 0 && green == 0 && blue == 0) {
            return "\x1b[0m ";
        }
        return "\x1b[48;2;" + to_string(red) + ";" + to_string(green)
               + ";" + to_string(blue) + "m ";
    }
};

class Buffer {
public:
    int height, width;
    vector<Color> pixels;
    vector<float> depth;

    static Buffer cons(int height, int width) {
        return Buffer {
            height, width,
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
    float x, y, z;

    static Vec3 cons(float x, float y, float z) {
        return Vec3 { x, y, z };
    }

    void rotatez(float angle) {
        float x = this->x, y = this->y, z = this->z;
        float sint = sin(angle), cost = cos(angle);
        this->x = x * cost - y * sint;
        this->y = x * sint + y * cost;
    }

    void rotatex(float angle) {
        float x = this->x, y = this->y, z = this->z;
        float sint = sin(angle), cost = cos(angle);
        this->y = y * cost - z * sint;
        this->z = y * sint + z * cost;
    }

    void normalize() {
        float length = sqrt(this->inner_prod(this));
        x /= length; y /= length; z /= length;
    }

    const float inner_prod(Vec3* other) {
        return x * other->x + y * other->y + z * other->z;
    }
};

class Texture {
public:
    int height, width;
    vector<Color> texture;

    static Texture cons(string texpath) {
        ifstream file(texpath);

        vector<Color> texture_data;
        string line;
        int width = 0, height = 0;

        while (getline(file, line)) {
            istringstream line_stream(line);
            string pixel;
            int curr_width = 0;
            while (line_stream >> pixel) {
                size_t first_delim = pixel.find(';');
                size_t second_delim = pixel.find(';', first_delim + 1);
                int red = stoi(pixel.substr(0, first_delim));
                int green = stoi(pixel.substr(first_delim + 1, second_delim - first_delim - 1));
                int blue = stoi(pixel.substr(second_delim + 1));
                texture_data.push_back(Color::cons(red, green, blue));
                curr_width += 1;
            }
            width = curr_width;
            height += 1;
        }
        file.close();

        return Texture { height, width, texture_data };
    }

    const Color get_at(float xfrac, float yfrac) {
        int x = xfrac * width, y = yfrac * height;
        return texture[y * width + x];
    }
};

class Planet {
public:
    float radius;
    float rotation;
    float tilt;
    Vec3 position;
    Texture texture;

    static Planet cons(float radius, Vec3 position, string texpath, float tilt) {
        return Planet { radius, 0, tilt, position, Texture::cons(texpath) };
    }
};

class Renderer {
public:
    Planet* planet;
    Buffer* buffer;
    Vec3 camera;
    Vec3 lightsource;
    string frame_buffer;

    void render_planet() {
        float dphi = 0.015;
        float dtheta = 0.5 * dphi;
        for (float phi = 0; phi < PI; phi += dphi) {
            for (float theta = 0; theta < TAU; theta += dtheta) {
                float worldx = planet->radius * sin(phi) * cos(theta);
                float worldy = planet->radius * sin(phi) * sin(theta);
                float worldz = planet->radius * cos(phi);
                Vec3 world = Vec3::cons(worldx, worldy, worldz);
                
                world.rotatez(planet->rotation);
                world.rotatex(planet->tilt);
                
                Vec3 normal = world;
                normal.normalize();
                
                world.x += planet->position.x;
                world.y += planet->position.x;
                world.z += planet->position.z;
                world.x -= camera.x;
                world.y -= camera.y;
                world.z -= camera.z;
                
                float scalex = 100;
                float scaley = scalex * 0.5;
                float screenx = (int)(world.y / world.x * scalex + buffer->halfwidth());
                float screeny = (int)(-world.z / world.x * scaley + buffer->halfheight());

                if (buffer->inbounds(screenx, screeny)
                    && world.x < buffer->get_depth(screenx, screeny))
                {
                    Color color = planet->texture.get_at(theta / TAU, phi / PI);
                    float lighting = normal.inner_prod(&lightsource);
                    color.darken(lighting);

                    buffer->set(screenx, screeny, color, world.x);
                }
            }
        }
    }

    void display_to_screen() {
        cout << "\x1b[H";
        frame_buffer.clear();
        for (int y = 0; y < buffer->height; y += 1) {
            frame_buffer += "\x1b[1m";
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
    Planet planet = Planet::cons(27, Vec3::cons(0, 0, 0), EARTHPATH, 0.15);
    Vec3 camera_pos = Vec3::cons(-55, 0, 0);
    Vec3 lighting = Vec3::cons(-1, 1, 0.5);
    lighting.normalize();

    Renderer renderer = Renderer { &planet, &buffer, camera_pos, lighting };

    cout << "\x1b[?25l";
    while (true) {
        {
            renderer.lightsource.rotatez(0.01);
            renderer.planet->rotation -= 0.01;
        }
        buffer.clear();
        renderer.render_planet();
        renderer.display_to_screen();
    }
    
    return 0;
}
