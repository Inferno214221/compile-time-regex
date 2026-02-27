use std::ops::Range;

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

pub struct Capture<'a> {
    pub range: Range<usize>,
    pub content: &'a str,
}

impl<'a> Capture<'a> {
    pub fn content(&self) -> &str {
        self.content
    }

    pub fn range(&self) -> &Range<usize> {
        &self.range
    }
}

// TODO: When getting a Capture in hir, increment a const generic
// Push to a Vec of Captures, and track whether the current HirKind is required, leading to optional
// capture groups. 
regex!(Email = r"(?<name>[a-z]+)@(?<domain>[a-z]+\.(?<tld>[a-z]+))");

pub struct EmailCaptures<'a>(pub Capture<'a>, pub Capture<'a>, pub Capture<'a>);

impl<'a> EmailCaptures<'a> {
    pub fn name(&'a self) -> &'a str {
        self.0.content
    }

    pub fn cap_name(&'a self) -> &'a Capture<'a> {
        &self.0
    }

    pub fn domain(&'a self) -> &'a str {
        self.1.content
    }

    pub fn cap_domain(&'a self) -> &'a Capture<'a> {
        &self.1
    }

    pub fn tld(&'a self) -> &'a str {
        self.2.content
    }

    pub fn cap_tld(&'a self) -> &'a Capture<'a> {
        &self.2
    }
}

regex!(PhoneNum = r"(0|(?<country_code>\d+))(?<number>\d{9})");

pub struct PhoneNumCaptures<'a>(pub Option<Capture<'a>>, pub Capture<'a>);

impl<'a> PhoneNumCaptures<'a> {
    pub fn country_code(&'a self) -> Option<&'a str> {
        self.0.as_ref().map(Capture::content)
    }

    pub fn cap_country_code(&'a self) -> Option<&'a Capture<'a>> {
        self.0.as_ref()
    }

    pub fn number(&'a self) -> &'a str {
        self.1.content
    }

    pub fn cap_number(&'a self) -> &'a Capture<'a> {
        &self.1
    }
}