
enum Result {
    Ok(int),
    Err(char * ),
};

struct Result read_file(char* f) {

}

int main() {
    struct Result r = read_file("file.txt");

    if let Result::Ok(i) = r {
    } else  {
        printf("Err: %s\n", e);
    }

  return 0;
}
