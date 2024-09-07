

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
            .variant = 1,
            .value = { .B = 3 }
            };


	if (1 == a.variant) {
            char x = a.value.B;
        {
		printf("sffjsfnf");
	}}
	return 0;
}


