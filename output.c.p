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

int main(){
	struct foo a = { 
            .variant = 0,
            .value = { .A = 435 }
            };
	if (0 == a.variant) {
int x = a.value.A;
{
		printf("sffjsfnf");
	}}
	return 0;
}


