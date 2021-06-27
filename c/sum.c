// This code is from: https://cs.wellesley.edu/~cs240/f20/slides/x86-basics.pdf
void sumstore(long x, long y,
              long *dest) {
  long t = x + y;
  *dest = t;
}
