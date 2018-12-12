#[macro_use]
extern crate bencher;
extern crate sysapi;

use bencher::Bencher;

use crate::sysapi::soload::processes_of_uid_short;


fn bench_processes_of_uid_short(b: &mut Bencher) {
    b.iter(|| processes_of_uid_short(65))
}


benchmark_group!(benches, bench_processes_of_uid_full, bench_processes_of_uid_short);
benchmark_main!(benches);
