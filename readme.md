Playing around with the rustlatin challenge.

```bash
❯ cargo +nightly bench
    Finished bench [optimized] target(s) in 0.32s
     Running unittests src/lib.rs (target/release/deps/rustlatin-fda939ba18243ff9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/rustlatin.rs (target/release/deps/rustlatin-f2c1894e60728b66)

running 10 tests
test bench_rustlatin               ... bench:   7,253,762 ns/iter (+/- 751,113)
test bench_rustlatin_faster        ... bench:   6,413,026 ns/iter (+/- 825,818)
test bench_rustlatin_fastest       ... bench:     983,737 ns/iter (+/- 134,003)
test bench_rustlatin_fastest_map   ... bench:   1,523,020 ns/iter (+/- 206,129)
test bench_rustlatin_fastest_match ... bench:     981,517 ns/iter (+/- 147,447)
test bench_rustlatin_fastest_simd  ... bench:   1,005,431 ns/iter (+/- 131,675)
test bench_rustlatin_fastester     ... bench:     991,658 ns/iter (+/- 134,062)
test bench_rustlatin_map           ... bench:   7,309,614 ns/iter (+/- 816,046)
test bench_rustlatin_rayon         ... bench:   5,187,745 ns/iter (+/- 2,566,256)
test bench_rustlatin_rayon_map     ... bench:   5,065,354 ns/iter (+/- 2,719,429)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured
```