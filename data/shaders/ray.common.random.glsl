
// Generate a random unsigned int from two unsigned int values, using 16 pairs
// of rounds of the Tiny Encryption Algorithm. See Zafar, Olano, and Curtis,
// "GPU Random Numbers via the Tiny Encryption Algorithm"
uint tea(uint val0, uint val1) {
  uint v0 = val0;
  uint v1 = val1;
  uint s0 = 0;
  for(uint n = 0; n < 16; n++) {
    s0 += 0x9e3779b9;
    v0 += ((v1 << 4) + 0xa341316c) ^ (v1 + s0) ^ ((v1 >> 5) + 0xc8013ea4);
    v1 += ((v0 << 4) + 0xad90777d) ^ (v0 + s0) ^ ((v0 >> 5) + 0x7e95761e);
  }
  return v0;
}

// Generate a random unsigned int in [0, 2^24) given the previous RNG state
// using the Numerical Recipes linear congruential generator
uint lcg(inout uint prev) {
  uint LCG_A = 1664525u;
  uint LCG_C = 1013904223u;
  prev       = (LCG_A * prev + LCG_C);
  return prev & 0x00FFFFFF;
}

// Generate a random float in [0, 1) given the previous RNG state
float rnd(inout uint prev) {
  return (float(lcg(prev)) / float(0x01000000));
}

Random randomInit(uint a, uint b) {
  uint seed = tea(a, b);
  return Random(seed);
}

float randomNext(inout Random r) {
  return rnd(r.seed);
}

vec3 randomNextSphereUnit(inout Random r) {
  vec3 p;
  float len;
  do {
    float a = randomNext(r);
    float b = randomNext(r);
    float c = randomNext(r);
    p = 2.0 * vec3(a, b, c) - vec3(1, 1, 1);
    len = length(p);
  } while (len*len >= 1.0);
  return p;
}

vec3 randomNextUnitDisk(inout Random r) {
  vec3 p;
  do {
    float a = randomNext(r);
    float b = randomNext(r);
    p = 2.0 * vec3(a, b, 0) - vec3(1, 1, 0);
  } while (dot(p, p) >= 1.0);
  return p;
}
