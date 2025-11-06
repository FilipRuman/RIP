#include "file.c" // types
#include <cstdio>
#include <stddef.h> // NULL, size_t
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
  Vec v = {.x = 3, .y = 4};
  Mode m = MODE_B;

  i32 idx = v.x - 1;   // tests arithmetic + index expressions
  i32 val = data[idx]; // array indexing

  // Typedef shadowing - variable named `i32`
  i32 i32 = identity(7);

  // Nested parentheses + sequencing + local shadowing in inner scope
  {
    int val = mul(v.x, v.y);
    global = val; // write to global to check symbol resolution
  }

  // Multiple declarators, mixed init
  int a = 2;
  int b = a + 3;
  int c;

  c = a + b + val + global + m + i32;

  return c;
}
