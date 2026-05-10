#![feature(iter_array_chunks)]
#![feature(portable_simd)]
#![feature(substr_range)]

use std::hint::black_box;
use std::{iter, range};
use std::ops::Range;
use std::simd::{Mask, Simd};
use std::simd::cmp::SimdPartialEq;
use std::sync::LazyLock;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use ct_regex::{Regex as _, regex};
use regex::Regex;

regex!(Hay = r"hay");

static HAY: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"hay").unwrap()
);

const NEEDLE: &str = "hay";
const CHUNK_SIZE: usize = 8;

const LITERALS: &[&str] = &[
    "hiheyhahayheh",
    "hiheyhahayhehhiheyhahayhehhiheyhahayhehhiheyhahayhehhiheyhahayheh"
];

fn do_simd_match(haystack: &str) -> Vec<Range<usize>> {
    let haystack = haystack.as_bytes()
        .iter()
        .copied()
        .chain(iter::repeat_n(0, CHUNK_SIZE - 1)) // Pad the end with 0s
        .collect::<Vec<_>>();
    let mut matches = Vec::new();

    let chunks = haystack.iter()
        .copied()
        .array_chunks::<CHUNK_SIZE>()
        .enumerate();

    let zeros = Simd::<u8, CHUNK_SIZE>::splat(0);

    for (chunk_num, chunk) in chunks {
        let mut vector = Simd::from_array(chunk);
        let mut mask = Mask::<i8, CHUNK_SIZE>::splat(true);

        for (byte_num, byte) in NEEDLE.as_bytes().iter().copied().enumerate() {
            let needle_source = [byte; CHUNK_SIZE];
            let needle = Simd::load_select(&needle_source, mask, zeros);
            mask = vector.simd_eq(needle);

            let next_byte = haystack.get(chunk_num * 8 + byte_num)
                .copied()
                .unwrap_or_default();
            vector = vector.shift_elements_left::<1>(next_byte);
        }

        for (index, success) in mask.to_array().into_iter().enumerate() {
            if success {
                let start = chunk_num * 8 + index;
                matches.push(start..(start + CHUNK_SIZE));
            }
        }
    }

    matches
}

fn do_normal_match(haystack: &str) -> Vec<Range<usize>> {
    haystack.matches(NEEDLE).map(|s| { // non-overlapping
        let range::Range { start, end } = haystack.substr_range(s).unwrap();
        Range { start, end }
    }).collect()
}

fn do_ct_regex_match(haystack: &str) -> Vec<Range<usize>> {
    Hay::range_of_all_matches(haystack, true).collect()
}

fn do_regex_match(haystack: &str) -> Vec<Range<usize>> {
    HAY.find_iter(haystack).map(|m| m.range()).collect() // non-overlapping
}

fn bench_literal_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("literal_search");
    for haystack in black_box(LITERALS) {
        group.bench_with_input(
            BenchmarkId::new("simd", haystack),
            haystack,
            |b, haystack| b.iter(|| do_simd_match(haystack))
        );
        group.bench_with_input(
            BenchmarkId::new("normal", haystack),
            haystack,
            |b, haystack| b.iter(|| do_normal_match(haystack))
        );
        group.bench_with_input(
            BenchmarkId::new("ct_regex", haystack),
            haystack,
            |b, haystack| b.iter(|| do_ct_regex_match(haystack))
        );
        group.bench_with_input(
            BenchmarkId::new("regex", haystack),
            haystack,
            |b, haystack| b.iter(|| do_regex_match(haystack))
        );
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_millis(50))
        .warm_up_time(Duration::from_millis(50));
    targets = bench_literal_search
);
criterion_main!(benches);