

struct OkVal {
    char* source;
};


union Result__u {
    char*  Ok;
    char*  Err;

};

struct Result {
    int variant;
    union Result__u value;
};

;



int main() {

    struct Result m = (struct Result) {
        .variant = 0,
        .value = { .Ok = "hello world" }
    };

    m = (struct Result) {
        .variant = 1,
        .value = { .Err = "hello world" }
    };



    return 0;
}

