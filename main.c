
int main() {
  int b = 2;
  int a = 2;

  b = expif a > 5 {
        printf("a is greater than 5");
        10
  } else {
        printf("a <= 5");
        0
    };

    printf("%d", b);

    for i in 0 to 10 {
        printf("%d", i);
    }

    char* mem = (char*)malloc(2048);
    char* mem2 = (char*)malloc(1024);

    defer(free(mem); free(mem2);) {
        for i in 0 to 30 {
            mem[i] = 'a';
            mem2[i] = 'b';
        }
    }

  return 0;
}
