#include <stdio.h>

void fn(void){}
void fn2(void){}

int main(int x) {
  if (x == 5) {
    fn();
  } else {
    fn2();
  }
  printf("hello there");

  return 0;
}


