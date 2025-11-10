25 + (12 - 55);

#include "endian.h"
#include "file.c"
#include <cstdio>
#include <stddef.h>
#include <stdio.h>
typedef int i32;

typedef struct {
  i32 x;
  i32 y;
} Vec;

typedef enum { MODE_A, MODE_B = 5, MODE_C } Mode;

static i32 global = 10;
static i32 data[4] = {1, 2, 3, 4};

i32 mul(i32 a, i32 b) { return a * b; }

i32 weird_decl = sizeof(Vec) + sizeof(Mode);

i32 identity(i32 v) { return v; }

int main() {
  Vec v = {3, 4};
  Mode m = MODE_B;

  i32 idx = v.x - 1;
  i32 val = data[idx];

  i32 i32 = identity(7);

  {
    int val = mul(v.x, v.y);
    global = val;
  }

  int a = 2;
  int *ptr = &a;
  int b = *ptr + 3;
  int c;

  int len = 25;
  for (int i = 0; i < len; i++) {
    printf("25 %d", i);
  }
  int i = 0;
  while (i < len) {
    printf("25 %d", i);
  }

  if (i < len) {
    c++;
    printf("i < len");
    printf("i < len");
    printf("i < len");
  } else if (i > len * 2) {
    b--;
    printf("Some text");
  } else {
    return c * 25;
  }

  c = a + b + val + global + m + i32;

  return c;
}
