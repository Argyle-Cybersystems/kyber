fn main() {
  // TODO: conditional compilation
  cc::Build::new()
    .include("src/avx2/")
    .file("src/avx2/basemul.S")
    .file("src/avx2/fq.S")
    .file("src/avx2/invntt.S")
    .file("src/avx2/ntt.S")
    .file("src/avx2/shuffle.S")
    .compile("pqc_kyber");
}