use std::sync::LazyLock;

use ct_regex_bench::parse_args_many;

use regex::Regex;

static NEEDLE: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"needle").unwrap()
);
static ALPHA: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"[a-zA-Z]+").unwrap()
);
static PHONE_NUM: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"(0|(?<country_code>\+[0-9]+))(?<number>[0-9]{9})").unwrap()
);
static EMAIL: LazyLock<Regex> = LazyLock::new(
    // Interestingly, regex really doesn't like it if you make this    |
    // last group optional. ct_regex doesn't care though.              V
    || Regex::new(r"([[:word:]]+)@(?<domain>([[:word:]]+)(\.[[:word:]]+))").unwrap()
);

fn main() {
    let (expression, haystacks) = parse_args_many();

    for haystack in haystacks {
        let success = match expression.as_str() {
            "needle"   => NEEDLE.is_match(&haystack),
            "alpha"    => ALPHA.is_match(&haystack),
            "phonenum" => PHONE_NUM.is_match(&haystack),
            "email"    => EMAIL.is_match(&haystack),
            _ => panic!("unknown expression"),
        };

        if success {
            println!("success")
        } else {
            println!("fail")
        }
    }
}