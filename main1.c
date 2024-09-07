#include <stdio.h>
#include <string.h>

int palindrom(char *s, int len) {
  for (int i = 0; i <= len/2; i++) {
    if (s[i] != s[len-i-1]) return 0;
  }

  return 1;
}
int main() {
  char* s = "abba";

  if (palindrom(s, strlen(s)) == 1) {
    printf("Is palindrome");
  } else {
    printf("Is not palindrome");
  }


  return 0;
}
