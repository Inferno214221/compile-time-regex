use ct_regex_bench::parse_args_many;

use ct_regex::{Regex, regex};

regex!(Needle = r"needle");
regex!(Alpha = r"[a-zA-Z]+");
regex!(PhoneNum = r"(0|(?<country_code>\+[0-9]+))(?<number>[0-9]{9})");
regex!(Email = r"([[:word:]]+)@(?<domain>([[:word:]]+)(\.[[:word:]]+))");

fn main() {
    let (expression, haystacks) = parse_args_many();

    for haystack in haystacks {
        let success = match expression.as_str() {
            "needle"   => Needle::contains_match(&haystack),
            "alpha"    => Alpha::contains_match(&haystack),
            "phonenum" => PhoneNum::contains_match(&haystack),
            "email"    => Email::contains_match(&haystack),
            _ => panic!("unknown expression"),
        };

        if success {
            println!("success")
        } else {
            println!("fail")
        }
    }
}