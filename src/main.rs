use ct_regex::*;

regex!(pub MyPattern = r"^(([a-z]+)|([0-9]+))$");
regex!(MyOtherPattern = r"word\n");

fn main() {
    dbg!(MyPattern::matches(&mut Haystack::from("word")));
    dbg!(MyPattern::matches(&mut Haystack::from("word123")));
    dbg!(MyPattern::matches(&mut Haystack::from("123")));

    dbg!(MyOtherPattern::matches(&mut Haystack::from("word")));
    dbg!(MyOtherPattern::matches(&mut Haystack::from("word\n")));

    dbg!(regex!(r"^[a-z]+$").matches(&mut Haystack::from("word")));
    dbg!(regex!(r"^[a-z]+$").matches(&mut Haystack::from("123")));
    dbg!(regex!(r"word\n").matches(&mut Haystack::from("word\n")));

    dbg!(regex!(r"^start.*end$").matches(&mut Haystack::from("starteend")));
}