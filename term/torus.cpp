


#include <chrono>
#include <iostream>
#include <thread>
#include <vector>



#define MAJORRAD 50
#define MINORRAD 25
#define HEIGHT 60
#define WIDTH 240
#define FRAMEDELAY 5
#define ROTX 0.04
#define ROTY 0.07
#define ROTZ 0.02
#define THETADELTA 0.1
#define PHIDELTA 0.03
#define SCALINGX 80
#define SCALINGY 60
#define VIEWDISTANCE 250



const float TAU = 6.28319;
const float LIGHT[] = {-2, 3, 3};
const char GRAD[] = ".,:;+**?%%#@@";
const char BACKGROUND = ' ';

typedef struct Vec3 {
    float x, y, z;

    static Vec3 cons(float x, float y, float z) {
        return Vec3 { x, y, z };
    }

    const Vec3 rotationx(float theta) {
        float nx = this->x;
        float ny = this->y * cos(theta) - this->z * sin(theta);
        float nz = this->y * sin(theta) + this->z * cos(theta);
        return Vec3::cons(nx, ny, nz);
    }

    const Vec3 rotationy(float theta) {
        float nx = this->x * cos(theta) + this->z * sin(theta);
        float ny = this->y;
        float nz = -this->x * sin(theta) + this->z * cos(theta);
        return Vec3::cons(nx, ny, nz);
    }

    const Vec3 rotationz(float theta) {
        float nx = this->x * cos(theta) - this->y * sin(theta);
        float ny = this->x * sin(theta) + this->y * cos(theta);
        float nz = this->z;
        return Vec3::cons(nx, ny, nz);
    }

    const Vec3 rotationchain(Vec3 angles) {
        float nx = this->x, ny = this->y, nz = this->z;
        Vec3 vec = Vec3::cons(nx, ny, nz);
        vec = vec.rotationy(angles.y);
        vec = vec.rotationx(angles.x);
        vec = vec.rotationz(angles.z);
        return vec;
    }

    const float dot_prod(Vec3 other) {
        return this->x * other.x + this->y * other.y + this->z * other.z;
    }

    const Vec3 normalize() {
        float length = sqrt(this->x * this->x + this->y * this->y + this->z * this->z);
        if (length == 0) length = 1;
        return Vec3::cons(this->x / length, this->y / length, this->z / length);
    }
} Vec3;

Vec3 get_light() {
    return Vec3::cons(LIGHT[0], LIGHT[1], LIGHT[2]).normalize();
}

char brightness_char(float luminosity) {
    if (luminosity < 0) luminosity = 0;
    return GRAD[(int)(luminosity * (strlen(GRAD)-1))];
}

class Buffer {
public:
    int height, width;
    std::vector<char> visual;
    std::vector<bool> colored;
    std::vector<float> zbuffer;

    static Buffer cons(int height, int width) {
        return Buffer{
            .height = height,
            .width = width,
            .visual = std::vector<char>(height * width, BACKGROUND),
            .colored = std::vector<bool>(width * height, false),
            .zbuffer = std::vector<float>(height * width, 0),
        };
    }

    void display_frame() {
        std::cout << "\x1b[H";
        std::string frame_buffer;
        for (int i = 0; i < this->width * this->height; i += 1) {
            if (i % this->width) {
                if (this->colored[i] && this->visual[i] != BACKGROUND) {
                    frame_buffer += "\x1b[38;2;89;44;4m";
                    frame_buffer += "\x1b[1m";
                    frame_buffer += "\x1b[40m";
                }
                else if (this->visual[i] != BACKGROUND) {
                    frame_buffer += "\x1b[38;2;173;158;95m";
                    frame_buffer += "\x1b[1m";
                    frame_buffer += "\x1b[40m";
                }
                else  {
                    frame_buffer += "\x1b[0m";
                }
                frame_buffer.push_back(this->visual[i]);
                continue;
            }
            frame_buffer.push_back('\n');
        }
        frame_buffer += "\x1b[0m";
        std::cout << frame_buffer;
        std::cout.flush();
    }

    const int halfheight() {
        return this->height / 2;
    }

    const int halfwidth() {
        return this->width / 2;
    }

    const int index(int x, int y) {
        return y * this->width + x;
    }

    const bool inbounds(int x, int y) {
        return x < this->width && y < this->height;
    }

    void clear() {
        std::fill(visual.begin(), visual.end(), BACKGROUND);
        std::fill(zbuffer.begin(), zbuffer.end(), 0);
    }
};

typedef struct Torus {
    float radmajor;
    float radminor;
    Vec3 angle;
    Vec3 angledelta;

    static Torus cons(float radmajor, float radminor, Vec3 angle, Vec3 angledelta) {
        return Torus { radmajor, radminor, angle, angledelta };
    }

    void rotate() {
        angle.x += angledelta.x;
        angle.y += angledelta.y;
        angle.z += angledelta.z;
    }
} Torus;

Vec3 calculate_normal(float theta, float phi, float radmajor, float radminor) {
    float costheta = cos(theta);
    float sintheta = sin(theta);
    float cosphi = cos(phi);
    float sinphi = sin(phi);

    Vec3 dtheta = Vec3::cons(
        -radminor * sintheta * cosphi,
        -radminor * sintheta * sinphi,
        radminor * costheta
    );
    Vec3 dphi = Vec3::cons(
        -(radmajor + radminor * costheta) * sinphi,
        (radmajor + radminor * costheta) * cosphi,
        0
    );

    Vec3 normal = Vec3::cons(
        dtheta.y * dphi.z - dtheta.z * dphi.y,
        dtheta.z * dphi.x - dtheta.x * dphi.z,
        dtheta.x * dphi.y - dtheta.y * dphi.x
    );

    return normal.normalize();
}

void render_torus(Buffer* buffer, Torus* torus) {
    for (float theta = 0; theta < TAU; theta += THETADELTA) {
        for (float phi = 0; phi < TAU; phi += PHIDELTA) {
            float x = (torus->radmajor + torus->radminor * cos(theta)) * cos(phi);
            float y = (torus->radmajor + torus->radminor * cos(theta)) * sin(phi);
            float z = torus->radminor * sin(theta);
            Vec3 point = Vec3::cons(x, y, z);
            point = point.rotationchain(torus->angle);

            Vec3 normal = calculate_normal(theta, phi, torus->radmajor, torus->radminor);
            normal = normal.rotationchain(torus->angle);

            x = point.x, y = point.y, z = point.z + VIEWDISTANCE;
            if (z <= 0) return;

            float invz = 1 / z;
            float modifierx = invz * SCALINGX;
            float modifiery = invz * SCALINGY;

            int xp = (int)(buffer->halfwidth() + modifierx * x);
            int yp = (int)(buffer->halfheight() + modifiery * y);

            int idx = buffer->index(xp, yp);
            if (buffer->inbounds(xp, yp) && invz > buffer->zbuffer[idx]) {
                float luminosity = normal.dot_prod(get_light());
                char chr = brightness_char(luminosity);
                buffer->visual[idx] = chr;
                buffer->zbuffer[idx] = invz;
                if (theta < TAU / 2 && theta > TAU / 16) {
                    buffer->colored[idx] = true;
                }
                else {
                    buffer->colored[idx] = false;
                }
            }
        }
    }
}

int main() {
    Torus torus = Torus::cons(MAJORRAD, MINORRAD,
                              Vec3::cons(TAU / 8, TAU / 2, 0), Vec3::cons(ROTX, ROTY, ROTZ));
    Buffer buffer = Buffer::cons(HEIGHT, WIDTH);

    std::cout << "\x1b[2J";
    std::cout << "\x1b[?25l";

    while (true == true) {
        buffer.clear();
        render_torus(&buffer, &torus);
        buffer.display_frame();
        torus.rotate();

        std::this_thread::sleep_for(std::chrono::milliseconds(FRAMEDELAY));
    }

    return 0;
}
