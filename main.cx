

int main() {
    char op = '+';

    int a = 4;
    for i in 0 to 10 {
        printf("%d\n", i);
    }

    char* mem1 = (char*)malloc(128);
    char* mem2 = (char*)malloc(256);
    defer(free(mem1); free(mem2);) {
        mem1[0] = op;
        match (op) {
            case '+' {
                printf("Add\n");
            }
            case '-' {
                printf("Sub\n");
            }
            case '*' {
                printf("Mul\n");
            }
            case '/' {
                printf("Div\n");
            }
        }
    }


  return 0;
}
