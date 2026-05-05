use ct_regex::*;

regex!(pub MyPattern = r"^(([a-z]+)|([0-9]+))$" / "i");
regex!(MyOtherPattern = r"^word$");
regex!(PhoneNum = r"(0|(?<country_code>\+[0-9]+))(?<number>[0-9]{9})");

regex!(Re = r"bc*");

fn main() {
    dbg!(MyPattern::is_match("word"));
    dbg!(MyPattern::is_match("word123"));
    dbg!(MyPattern::is_match("123"));

    dbg!(MyOtherPattern::is_match("word"));
    dbg!(MyOtherPattern::is_match("word\n"));

    dbg!(regex!(r"[a-z]+").is_match("word"));
    dbg!(regex!(r"[a-z]+").is_match("123"));
    dbg!(regex!(r"word\n").is_match("word\n"));

    dbg!(regex!(r"start.*end").is_match("startfend"));
    dbg!(regex!(r"start.*end").is_match("starteend"));
    dbg!(regex!(r"[a-z][A-Z][0-9]\d\w\s").is_match("sS01_ "));
    dbg!(regex!(r"a|b|c").is_match("c"));
    dbg!(regex!(r"(a|b)+c").is_match("abc"));

    dbg!(regex!(r"bc*").slice_match("abcccd"));
    dbg!(regex!(r"bc+?").slice_match("abcccd"));

    dbg!(PhoneNum);

    let hay = "0123456789";
    dbg!(PhoneNum::is_match(hay));
    let caps = PhoneNum::do_capture(hay).unwrap();
    dbg!(caps.whole_match());
    dbg!(caps.country_code());
    dbg!(caps.number());
    dbg!(caps.cap_0());
    dbg!(caps.cap_1());
    dbg!(caps.cap_2());
    dbg!(caps.cap_3());

    let hay = "+61123456789";
    dbg!(PhoneNum::is_match(hay));
    let caps = PhoneNum::do_capture(hay).unwrap();
    dbg!(caps.whole_match());
    dbg!(caps.country_code());
    dbg!(caps.number());
    dbg!(caps.cap_0());
    dbg!(caps.cap_1());
    dbg!(caps.cap_2());
    dbg!(caps.cap_3());

    // FIXME: not matching. Do I need a final check after hay returns none?
    // I don't think there is a single other pattern that can fail like that.
    dbg!(regex!("$").slice_all_matches("aaa", true));

    dbg!(regex!(r"[^s]*").slice_match("sbcccs"));

    let mut hay = String::from("sbcccs");
    dbg!(Re::slice_match(hay.as_str()));
    dbg!(Re::replace(&mut hay, "ucces"), &hay);

    let mut hay = String::from("a b b b c");
    dbg!(Re::replace_all(&mut hay, "de"));
    dbg!(hay);

    let mut hay = String::from("a b b b c");
    let mut it = "123".chars().map(String::from);
    dbg!(Re::replace_all_using(&mut hay, || it.next().unwrap()));
    dbg!(hay);

    let mut hay = String::from("+61123456789");
    dbg!(PhoneNum::replace_captured(&mut hay, do_the_thing));
    dbg!(hay);

    let mut hay = String::from("a bc bcc bccc d");
    dbg!(Re::replace_all_captured(&mut hay, do_the_other_thing));
    dbg!(hay);

    dbg!(regex!(r"e\w").find_all_captures("aeeced", true));
    dbg!(regex!(r"e\w").find_all_captures("aeeced", false));
    dbg!(regex!(r"e\w").count_matches("aeeced", true));
    dbg!(regex!(r"e\w").count_matches("aeeced", false));
}

fn do_the_thing<'a>(value: PhoneNumCapture<'a, &'a str>) -> String {
    format!("0{}", value.number())
}

fn do_the_other_thing<'a>(value: ReCapture<'a, &'a str>) -> String {
    format!("{}", value.cap_0().len())
}
