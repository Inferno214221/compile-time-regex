use ct_regex::*;

regex!(pub MyPattern = r"^(([a-z]+)|([0-9]+))$");
regex!(MyOtherPattern = r"word\n");

fn main() {
    dbg!(MyPattern::is_match(&mut Haystack::from("word")));
    dbg!(MyPattern::is_match(&mut Haystack::from("word123")));
    dbg!(MyPattern::is_match(&mut Haystack::from("123")));

    dbg!(MyOtherPattern::is_match(&mut Haystack::from("word")));
    dbg!(MyOtherPattern::is_match(&mut Haystack::from("word\n")));

    dbg!(regex!(r"^[a-z]+$").is_match(&mut Haystack::from("word")));
    dbg!(regex!(r"^[a-z]+$").is_match(&mut Haystack::from("123")));
    dbg!(regex!(r"word\n").is_match(&mut Haystack::from("word\n")));

    dbg!(regex!(r"^start.*end$").is_match(&mut Haystack::from("startfend")));
    dbg!(regex!(r"^start.*end$").is_match(&mut Haystack::from("starteend")));
    dbg!(regex!(r"[a-z][A-Z][0-9]\d\w\s").is_match(&mut Haystack::from("sS01_ ")));
    dbg!(regex!(r"a|b|c").is_match(&mut Haystack::from("c")));
    dbg!(regex!(r"^(a|b)+c$").is_match(&mut Haystack::from("abc")));

    dbg!(regex!(r"bc*").find_match(&mut Haystack::from("abcccd")));
}