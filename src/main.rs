use ct_regex_internal::{haystack::Haystack, regex::{AnonRegex, Regex}};

use ct_regex_macro::{anon_regex, regex};

regex!(pub MyPattern = r"^(([a-z]+)|([0-9]+))$");
regex!(MyOtherPattern = r"word\n");

fn main() {
    dbg!(MyPattern::matches(&mut Haystack::from("word")));
    dbg!(MyPattern::matches(&mut Haystack::from("word123")));
    dbg!(MyPattern::matches(&mut Haystack::from("123")));

    dbg!(MyOtherPattern::matches(&mut Haystack::from("word")));
    dbg!(MyOtherPattern::matches(&mut Haystack::from("word\n")));

    dbg!(anon_regex!(r"^[a-z]+$").matches(&mut Haystack::from("word")));
    dbg!(anon_regex!(r"^[a-z]+$").matches(&mut Haystack::from("123")));
    dbg!(anon_regex!(r"word\n").matches(&mut Haystack::from("word\n")));
}

// TODO: Test each matcher individually