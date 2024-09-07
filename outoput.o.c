#include <stdio.h>

union foo__u {
    int A;
    char B;
    int d2344;

};

struct foo {
    int variant;
    union foo__u value;
};

;

int main() {
    struct foo a = {
        .variant = 0,
        .value = { .A = 435 }
    };

    if (0 == a.variant) {
        int x = a.value.A;
        {
            printf("sffjsfnf");
        }
    }

    struct foo tmp = {
        .variant = 1,
        .value = { .B = 355 }
    };


    int a = 5 == 424 ? ({3; 3;}) : ({424; 44;});


    if (1 == a.variant) {
        char y = a.value.B;
        {
            printf("b");
        }
    }

    return 0;
}


