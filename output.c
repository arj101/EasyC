#include <stdio.h>
#include <string.h>



        union Result__u {
        char Ok;
int Err;

};

            struct Result {
            int variant;
            union Result__u value;
        };
        
;



int main() {
  struct Result r = { 
            .variant = 1,
            .value = { .Err = 4 }
            };

  int a = 6;
  int b;

  if (1 == r.variant ) {
int code = r.value.Err;
{
    printf("Error %d\n", code);
  }}

  for (int i=0 ;i < 5 ; i++) {
     printf("%d\n", i);
  }

  if (a == 6 ) { printf("A is 6\n");;
b = 5
  ;}else {b = 3
  ;};

  printf("\n\n%d\n", b);

  



  return 0;
}

