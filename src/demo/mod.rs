use ct_regex_macro::regex;

regex! {
    pub Email = r"([a-z]+)@(?<domain>([a-z]+)(\\.[a-z]+)?)"
}