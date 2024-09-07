

int main() {
    struct Result res = read_file("./foo");
    if let Result::Ok(file) = res {
        printf("Opened file");
    }
    return 0;
}

enum Result {
    Ok(int),
    Err(char*)
};
