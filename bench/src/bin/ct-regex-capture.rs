use ct_regex_bench::parse_args_many;

use ct_regex::{Regex, regex};

regex!(PhoneNum = r"(0|(?<country_code>\+[0-9]+))(?<number>[0-9]{9})");
regex!(Email = r"([[:word:]]+)@(?<domain>([[:word:]]+)(\.[[:word:]]+))");

fn main() {
    let (expression, haystacks) = parse_args_many();

    for haystack in haystacks {
        match expression.as_str() {
            "phonenum" => println!(
                "{:?}",
                PhoneNum::find_capture(&haystack)
                    .and_then(|c| Some((c.country_code()?, c.number())))
            ),
            "email" => println!(
                "{:?}",
                Email::find_capture(&haystack)
                    .map(|c| c.domain())
            ),
            _ => panic!("unknown expression"),
        }
    }
}