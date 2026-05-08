use std::env;

pub fn parse_args() -> (String, String) {
    let mut args = env::args();
    let _ = args.next();
    (args.next().expect("missing regex argument"), args.next().expect("missing haystack argument"))
}

pub fn parse_args_many() -> (String, Vec<String>) {
    let mut args = env::args();
    let _ = args.next();
    (args.next().expect("missing regex argument"), args.collect())
}