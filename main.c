

struct OkVal {
    char* source;
};

enum Result {
    Ok(char* ),
    Err(char* ),
};



int main() {

    struct Result m = Result::Ok("hello world");

    m = Result::Err("hello world");



  return 0;
}
