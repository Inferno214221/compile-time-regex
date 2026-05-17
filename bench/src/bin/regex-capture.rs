use std::sync::LazyLock;

use ct_regex_bench::parse_args_many;

use regex::Regex;

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
        match expression.as_str() {
            "phonenum" => println!(
                "{:?}",
                PHONE_NUM.captures(&haystack)
                    .and_then(|c| Some((c.get(2)?.as_str(), c.get(3)?.as_str())))
            ),
            "email" => println!(
                "{:?}",
                EMAIL.captures(&haystack)
                    .and_then(|c| Some(c.get(2)?.as_str()))
            ),
            _ => panic!("unknown expression"),
        }
    }
}