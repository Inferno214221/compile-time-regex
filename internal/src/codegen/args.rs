use std::collections::HashSet;
use std::fmt::{self, Display};

use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Token, Visibility};

use crate::codegen::ConfigExt;

pub enum RegexArgType {
    Regex(RegexArgs),
    Anon(AnonRegexArgs),
}

impl Parse for RegexArgType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.fork().parse::<RegexArgs>().is_ok() {
            Ok(RegexArgType::Regex(input.parse()?))
        } else {
            Ok(RegexArgType::Anon(input.parse()?))
        }
    }
}

pub struct RegexArgs {
    pub vis: Visibility,
    pub name: Ident,
    pub pat: LitStr,
    pub flags: Flags,
}

impl Parse for RegexArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis = input.parse()?;
        let name = input.parse()?;
        input.parse::<Token![=]>()?;
        let pat = input.parse()?;
        let flags = input.parse()?;
        Ok(RegexArgs {
            vis,
            name,
            pat,
            flags,
        })
    }
}

pub struct AnonRegexArgs {
    pub pat: LitStr,
    pub flags: Flags,
}

impl Parse for AnonRegexArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(AnonRegexArgs {
            pat: input.parse()?,
            flags: input.parse()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Flags(pub HashSet<char>);

impl Parse for Flags {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let sep = input.parse::<Token![/]>();

        let lit: syn::Result<LitStr> = input.parse();

        let set = lit
            .and_then(|l| sep.map(|_| l))
            .map(|l| l.value())
            .unwrap_or_default()
            .chars()
            .collect();

        Ok(Flags(set))
    }
}

impl Flags {
    pub fn create_config(self) -> ConfigExt {
        let mut config = ConfigExt::default();

        for c in self.0 {
            match c {
                'i' => config.case_insensitive(true),
                'm' => config.multi_line(true),
                's' => config.dot_matches_new_line(true),
                'R' => config.crlf(true),
                'U' => config.swap_greed(true),
                'x' => config.ignore_whitespace(true),
                'c' => config.complex_classes(true),
                'g' => panic!(
                    "the global flag is unsupported by this implementation, please read the docs \
                    on the methods available on the Regex trait"
                ),
                o => panic!("unknown flag provided for regex: {o:?}"),
            };
        }

        config
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut vec: Vec<_> = self.0.iter().collect();
        vec.sort();
        write!(f, "{:?}", &vec[..])
    }
}
