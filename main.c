


enum Result {
    Ok(int),
    Err(struct Info),
    IDontCare(char[]),
};


enum ABC {
FS,
    AB
};

int main() {

    struct Result r = Result::Ok(42);

    if let Result::Ok(x) = r {
        printf("Ok: %d", x);
        if let Result::Err(x) = r {
            printf("Ok: %d", x);
        }
    }


    int x = 4;
    for i in 0 to 10 {
    printf("Hello, world!");
    if let Result::IDontCare(x) = r {
        printf("Ok: %d", x);
    }
    for j in 0 to 10 {
        x = j * i;
      printf("Hello, world!");
    }
    for j in 0 to 10 {
      printf("Hello, world!");
      int a = 424;
      a *= 25;
      if let Result::Err(x) = r {
          printf("Ok: %d", x);
      }
    }
  }
  int a = 424;
  a *= 25;

  int a = 424;
  a *= 25;



  return 0;
}
