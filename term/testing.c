

#include <stdio.h>


void print_tower()
{
    for (int i = 0; i < 30; i += 1) {
        for (int j = 0; j <= i; j += 1) {
            printf("D");
        }
        printf("\n");
    }
}

int main()
{
    printf("hello world");
    print_tower();
    return 0;
}

