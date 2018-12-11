#[macro_use]
extern crate bencher;


use bencher::Bencher;
use self::common::utils::{processes_of_uid, processes_of_uid_short};


fn bench_processes_of_uid_full(b: &mut Bencher) {
    b.iter(|| processes_of_uid(65))
}


fn bench_processes_of_uid_short(b: &mut Bencher) {
    b.iter(|| processes_of_uid_short(65))
}


benchmark_group!(benches, bench_processes_of_uid_full, bench_processes_of_uid_short);
benchmark_main!(benches);
