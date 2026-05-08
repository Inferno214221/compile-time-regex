use ct_regex_bench::parse_args;

use regex::Regex;

const NEEDLE: &str = r"needle";
const ALPHA: &str = r"[a-zA-Z]+";
const PHONE_NUM: &str = r"(0|(?<country_code>\+[0-9]+))(?<number>[0-9]{9})";
const EMAIL: &str = r"([[:word:]]+)@(?<domain>([[:word:]]+)(\.[[:word:]]+))";

fn main() {
    let (expression, haystack) = parse_args();

    let success = match expression.as_str() {
        "needle"   => Regex::new(NEEDLE).unwrap().is_match(&haystack),
        "alpha"    => Regex::new(ALPHA).unwrap().is_match(&haystack),
        "phonenum" => Regex::new(PHONE_NUM).unwrap().is_match(&haystack),
        "email"    => Regex::new(EMAIL).unwrap().is_match(&haystack),
        _ => panic!("unknown expression"),
    };

    if success {
        println!("success")
    } else {
        println!("fail")
    }
}