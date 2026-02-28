use ct_regex_macro::regex;

regex! {
    pub Email = r"([a-z]+)@([a-z]+)(\\.[a-z]+)?"
}