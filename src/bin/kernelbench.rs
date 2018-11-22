#[macro_use]
extern crate bencher;
extern crate sysapi;

use bencher::Bencher;
use sysapi::utils::*;


// #[bench]
fn bench_processes_of_pid_full(b: &mut Bencher) {
    b.iter(|| processes_of_pid(0))
}


// #[bench]
fn bench_processes_of_pid_short(b: &mut Bencher) {
    b.iter(|| processes_of_pid_short(0))
}


benchmark_group!(benches, bench_processes_of_pid_full, bench_processes_of_pid_short);
benchmark_main!(benches);
