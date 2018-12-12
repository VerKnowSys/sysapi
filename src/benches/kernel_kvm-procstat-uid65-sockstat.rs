#[macro_use]
extern crate bencher;

extern crate sysapi;

use bencher::Bencher;

use crate::sysapi::soload::processes_of_uid;


fn bench_processes_of_uid_full(b: &mut Bencher) {
    b.iter(|| processes_of_uid(65))
}


benchmark_group!(benches, bench_processes_of_uid_full, bench_processes_of_uid_short);
benchmark_main!(benches);
