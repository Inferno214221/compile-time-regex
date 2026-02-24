use regex_automata::util::syntax::{self, Config};

use regex::{self, haystack::Haystack, hir::HirExtension, regex::Regex};

use regex_macro::regex;

regex!(MyPattern = r"^(([a-z]+)|([0-9]+))$");
regex!(MyOtherPattern = r"word\n");

fn main() {
    // let mut a = Haystack::new("A");
    // let mut b = a.clone();
    // b.progress();
    // assert_ne!(a.item(), b.item());

    let config = Config::new().unicode(false);

    eprintln!("{}", syntax::parse_with(r"(word|other) tail", &config).unwrap().into_type_expr::<u8>());
    eprintln!("{}", syntax::parse_with(r"^(([a-z]+)|([0-9]+))$", &config).unwrap().into_type_expr::<u8>());

    // type I = regex::matcher::Then<regex::matcher::Or<regex::matcher::Then<regex::matcher::Byte<119u8>,regex::matcher::Then<regex::matcher::Byte<111u8>,regex::matcher::Then<regex::matcher::Byte<114u8>,regex::matcher::Byte<100u8>>>>,regex::matcher::Then<regex::matcher::Byte<111u8>,regex::matcher::Then<regex::matcher::Byte<116u8>,regex::matcher::Then<regex::matcher::Byte<104u8>,regex::matcher::Then<regex::matcher::Byte<101u8>,regex::matcher::Byte<114u8>>>>>>,regex::matcher::Then<regex::matcher::Byte<32u8>,regex::matcher::Then<regex::matcher::Byte<116u8>,regex::matcher::Then<regex::matcher::Byte<97u8>,regex::matcher::Then<regex::matcher::Byte<105u8>,regex::matcher::Byte<108u8>>>>>>;
    // dbg!(I::matches(&mut Haystack::new("word")));
    // dbg!(I::matches(&mut Haystack::new("other")));
    // dbg!(I::matches(&mut Haystack::new("word tail")));
    // dbg!(I::matches(&mut Haystack::new("other tail")));

    // type J = regex::matcher::Then<regex::matcher::Beginning,regex::matcher::Then<regex::matcher::Or<regex::matcher::QuantifierNOrMore<regex::matcher::ByteRange<97u8,122u8>,1>,regex::matcher::QuantifierNOrMore<regex::matcher::ByteRange<48u8,57u8>,1>>,regex::matcher::End>>;
    // dbg!(J::matches(&mut Haystack::new("word")));
    // dbg!(J::matches(&mut Haystack::new("word123")));
    // dbg!(J::matches(&mut Haystack::new("123")));

    dbg!(MyPattern::matches(&mut Haystack::<char>::new("word")));
    dbg!(MyPattern::matches(&mut Haystack::<char>::new("word123")));
    dbg!(MyPattern::matches(&mut Haystack::<char>::new("123")));

    dbg!(MyOtherPattern::matches(&mut Haystack::<char>::new("word")));
    dbg!(MyOtherPattern::matches(&mut Haystack::<char>::new("word\n")));
}