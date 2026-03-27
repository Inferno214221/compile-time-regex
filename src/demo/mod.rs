use ct_regex_macro::regex;

regex! {
    pub Email = r"(\w+)@(?<domain>(\w+)(\\.\w+)?)"
}