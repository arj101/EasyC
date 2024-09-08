#include <stdio.h>
#include <stdlib.h>

int main() {
    char op = '+';

    int a = 4;
    for (int i=0 ; i < 10 ; i++) {
        printf("%d\n", i);
    }

    char* mem1 = (char*)malloc(128);
    char* mem2 = (char*)malloc(256);
    {
        {
            mem1[0] = op;
            switch (op) {
            case '+' : {
                printf("Add\n");
                break;
            }
            case '-' : {
                printf("Sub\n");
                break;
            }
            case '*' : {
                printf("Mul\n");
                break;
            }
            case '/' : {
                printf("Div\n");
                break;
            }
            }
        }
//deferred statements
        free(mem1);
        free(mem2);
    }


    return 0;
}

