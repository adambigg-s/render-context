


#include <cmath>
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

    void attenuate(float lighting) {
        if (lighting < 0.15) {
            lighting = 0.15;
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

    void rotatex(float angle) {
        float x = this->x, y = this->y, z = this->z;
        float sint = sin(angle), cost = cos(angle);
        this->x = x;
        this->y = y * cost - z * sint;
        this->z = y * sint + z * cost;
    }

    // void rotatey(float angle) {
    //     float x = this->x, y = this->y, z = this->z;
    //     float sint = sin(angle), cost = cos(angle);
    //     this->x = x * cost + z * sint;
    //     this->y = y;
    //     this->z = -x * sint + z * cost;
    // }

    void rotatez(float angle) {
        float x = this->x, y = this->y, z = this->z;
        float sint = sin(angle), cost = cos(angle);
        this->x = x * cost - y * sint;
        this->y = x * sint + y * cost;
        this->z = z;
    }

    // void rotatezyx(Vec3 angles) {
    //     this->rotatez(angles.z);
    //     this->rotatey(angles.y);
    //     this->rotatex(angles.x);
    // }

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
    
    static Texture cons(string path) {
        vector<Color> texture;
        int width = 0;
        int height = 0;

        vector<string> lines;
        string line;
        ifstream file(path);
        while(getline(file, line)) {
            lines.push_back(line);
            height += 1;
        }
        file.close();

        for (string& line : lines) {
            istringstream streamline(line);
            string token;
            int currwidth = 0;

            while (getline(streamline, token, ' ')) {
                istringstream token_stream(token);
                string component;
                vector<int> rgb;

                while(getline(token_stream, component, ';')) {
                    rgb.push_back(stoi(component));
                }
                texture.push_back(Color::cons(rgb[0], rgb[1], rgb[2]));
                currwidth += 1;
            }

            width = currwidth;
        }

        return Texture { height, width, texture };
    }

    const Color get_at(float xfrac, float yfrac) {
        int x = xfrac * width, y = yfrac * height;
        int xtrans = width-1 - x;
        return texture[y * width + xtrans];
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

                if (buffer->inbounds(screenx, screeny)) {
                    if (world.x > buffer->get_depth(screenx, screeny)) {
                        continue;
                    }
                    Color color = planet->texture.get_at(theta / TAU, phi / PI);
                    float lighting = normal.inner_prod(&lightsource);
                    color.attenuate(lighting);

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
    Planet planet = Planet::cons(27, Vec3::cons(0, 0, 0), EARTHPATH, 0.45);
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
