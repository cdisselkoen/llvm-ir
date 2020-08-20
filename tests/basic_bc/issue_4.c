struct output {
  double a0;
  double a1;
  double a2;
};

struct output run(float a) {
  struct output o;
  o.a0 = a;
  return o;
}
