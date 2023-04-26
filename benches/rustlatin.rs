#[macro_use]
extern crate bencher;

use bencher::Bencher;
use rustlatin::*;

const SHORT_STR: &str = "scope is hot";

fn long_str() -> String {
    return "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum".to_string().repeat(1000);
}

fn bench_rustlatin(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin(SHORT_STR);
        rustlatin(&str);
    });
}

fn bench_rustlatin_map(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_map(SHORT_STR);
        rustlatin_map(&str);
    });
}

fn bench_rustlatin_faster(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_faster(SHORT_STR);
        rustlatin_faster(&str);
    });
}

fn bench_rustlatin_fastest(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_fastest(SHORT_STR);
        rustlatin_fastest(&str);
    });
}

fn bench_rustlatin_fastest_simd(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_fastest_simd(SHORT_STR);
        rustlatin_fastest_simd(&str);
    });
}

fn bench_rustlatin_fastest_match(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_fastest_match(SHORT_STR);
        rustlatin_fastest_match(&str);
    });
}

fn bench_rustlatin_fastest_map(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_fastest_map(SHORT_STR);
        rustlatin_fastest_map(&str);
    });
}

fn bench_rustlatin_fastester(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_fastester(SHORT_STR);
        rustlatin_fastester(&str);
    });
}
fn bench_rustlatin_fastester2(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_fastester2(SHORT_STR);
        rustlatin_fastester2(&str);
    });
}
fn bench_rustlatin_fastester3(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_fastester3(SHORT_STR);
        rustlatin_fastester3(&str);
    });
}
fn bench_rustlatin_fastester4(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_fastester3(SHORT_STR);
        rustlatin_fastester3(&str);
    });
}
fn bench_rustlatin_fastester5(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_fastester3(SHORT_STR);
        rustlatin_fastester3(&str);
    });
}

fn bench_rustlatin_rayon(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_rayon(SHORT_STR);
        rustlatin_rayon(&str);
    });
}

fn bench_rustlatin_rayon_map(b: &mut Bencher) {
    let str = long_str();
    b.iter(|| {
        rustlatin_rayon_map(SHORT_STR);
        rustlatin_rayon_map(&str);
    });
}

benchmark_group!(
    benches,
    bench_rustlatin,
    bench_rustlatin_map,
    bench_rustlatin_faster,
    bench_rustlatin_fastest,
    bench_rustlatin_fastest_simd,
    bench_rustlatin_fastest_match,
    bench_rustlatin_fastest_map,
    bench_rustlatin_fastester,
    bench_rustlatin_fastester2,
    bench_rustlatin_fastester3,
    bench_rustlatin_fastester4,
    bench_rustlatin_fastester5,
    bench_rustlatin_rayon,
    bench_rustlatin_rayon_map,
);

benchmark_main!(benches);
