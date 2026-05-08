use std::{hint::black_box, sync::LazyLock, time::Duration};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use ct_regex::{Regex as _, regex};
use regex::Regex;

regex!(Needle = r"needle");
regex!(Alpha = r"[a-zA-Z]+");
regex!(PhoneNum = r"(0|(?<country_code>\+[0-9]+))(?<number>[0-9]{9})");
regex!(Email = r"([[:word:]]+)@(?<domain>([[:word:]]+)(\.[[:word:]]+))");

static NEEDLE: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"^needle$").unwrap()
);
static ALPHA: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"^[a-zA-Z]+$").unwrap()
);
static PHONE_NUM: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"^(0|(?<country_code>\+[0-9]+))(?<number>[0-9]{9})$").unwrap()
);
static EMAIL: LazyLock<Regex> = LazyLock::new(
    // Interestingly, regex really doesn't like it if you make this    |
    // last group optional. ct_regex doesn't care though.              V
    || Regex::new(r"^([[:word:]]+)@(?<domain>([[:word:]]+)(\.[[:word:]]+))$").unwrap()
);

const EMAILS: [&str; 6] = [
    "me@example.com",
    "spam@example.com",
    "example@gmail.com",
    "dotless@email",
    "@example.com",
    "example.com"
];

const PHONE_NUMS: [&str; 6] = [
    "+12123456789",
    "0123456789",
    "+9876543210",
    "0123321789",
    "567890",
    "+234E2"
];

fn compile_time_email(haystack: &str) -> bool {
    Email::is_match(haystack)
}

fn run_time_email(haystack: &str) -> bool {
    EMAIL.is_match(haystack)
}

fn bench_emails(c: &mut Criterion) {
    let mut group = c.benchmark_group("emails");
    for haystack in black_box(EMAILS) {
        group.bench_with_input(
            BenchmarkId::new("ct_regex", haystack),
            haystack,
            |b, haystack| b.iter(|| compile_time_email(haystack))
        );
        group.bench_with_input(
            BenchmarkId::new("regex", haystack),
            haystack,
            |b, haystack| b.iter(|| run_time_email(haystack))
        );
    }
    group.finish();
}

fn compile_time_phone(haystack: &str) -> bool {
    PhoneNum::is_match(haystack)
}

fn run_time_phone(haystack: &str) -> bool {
    PHONE_NUM.is_match(haystack)
}

fn bench_phones(c: &mut Criterion) {
    let mut group = c.benchmark_group("phones");
    for haystack in black_box(PHONE_NUMS) {
        group.bench_with_input(
            BenchmarkId::new("ct_regex", haystack),
            haystack,
            |b, haystack| b.iter(|| compile_time_phone(haystack))
        );
        group.bench_with_input(
            BenchmarkId::new("regex", haystack),
            haystack,
            |b, haystack| b.iter(|| run_time_phone(haystack))
        );
    }
    group.finish();
}
// bench_emails
criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_millis(50))
        .warm_up_time(Duration::from_millis(50));
    targets = bench_phones, bench_emails
);
criterion_main!(benches);