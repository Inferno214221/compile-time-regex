use ct_regex::*;

regex!(pub MyPattern = r"^(([a-z]+)|([0-9]+))$");
regex!(MyOtherPattern = r"^word$");
regex!(PhoneNum = r"(0|(?<country_code>\+[0-9]+))(?<number>[0-9]{9})");

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

    dbg!(regex!(r"bc*").slice_matching("abcccd"));

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
    dbg!(regex!("$").slice_all_matching("aaa", true));
}