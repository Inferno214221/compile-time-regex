use std::{any, fmt::{self, Write}, iter::Peekable, marker::PhantomData, str::Bytes};

use regex_automata::util::syntax::{self, Config};
use regex_syntax::hir::{Capture, Class, ClassBytesRange, Hir, HirKind, Literal, Look, Repetition};

fn main() {
    type E = Or::<Byte<0u8>, Or<Byte<1u8>, Byte<2u8>>>;
    dbg!(E::default());
    type F = Then::<Byte<b'e'>, Then<Byte<b'a'>, Byte<b's'>>>;
    dbg!(F::matches(&mut Haystack::new("eas")));
    dbg!(F::matches(&mut Haystack::new("nope")));
    dbg!(F::matches(&mut Haystack::new("ea")));
    // We've matched the whole thing if there is only one remaining byte
    dbg!(F::matches(&mut Haystack::new("easp")));
    // If a Matcher invokes another matcher but accepts a fail, it needs to rollback
    type G = Or<Then<Byte<b'e'>, Byte<b'a'>>, Byte<b's'>>;
    dbg!(G::matches(&mut Haystack::new("ea")));
    type H = Then<Then<Byte<b'e'>, Optional<Byte<b'a'>>>, Byte<b's'>>;
    dbg!(H::matches(&mut Haystack::new("es")));

    let mut a = Haystack::new("A");
    let mut b = a.clone();
    b.progress();
    assert_ne!(a.byte(), b.byte());

    let config = Config::new().unicode(false);

    eprintln!("{}", syntax::parse_with(r"(word|other) tail", &config).unwrap().into_type_expr());
    eprintln!("{}", syntax::parse_with(r"^(([a-z]+)|([0-9]+))$", &config).unwrap().into_type_expr());

    use crate as regex;
    type I = regex::Then<regex::Or<regex::Then<regex::Byte<119u8>,regex::Then<regex::Byte<111u8>,regex::Then<regex::Byte<114u8>,regex::Byte<100u8>>>>,regex::Then<regex::Byte<111u8>,regex::Then<regex::Byte<116u8>,regex::Then<regex::Byte<104u8>,regex::Then<regex::Byte<101u8>,regex::Byte<114u8>>>>>>,regex::Then<regex::Byte<32u8>,regex::Then<regex::Byte<116u8>,regex::Then<regex::Byte<97u8>,regex::Then<regex::Byte<105u8>,regex::Byte<108u8>>>>>>;
    dbg!(I::matches(&mut Haystack::new("word")));
    dbg!(I::matches(&mut Haystack::new("other")));
    dbg!(I::matches(&mut Haystack::new("word tail")));
    dbg!(I::matches(&mut Haystack::new("other tail")));

    type J = regex::Then<regex::Beginning,regex::Then<regex::Or<regex::QuantifierNOrMore<regex::ByteRange<97u8,122u8>,1>,regex::QuantifierNOrMore<regex::ByteRange<48u8,57u8>,1>>,regex::End>>;
    dbg!(J::matches(&mut Haystack::new("word")));
    dbg!(J::matches(&mut Haystack::new("word123")));
    dbg!(J::matches(&mut Haystack::new("123")));
}

#[derive(Debug, Clone)]
struct Haystack<'a> {
    iter: Peekable<Bytes<'a>>,
    start: bool,
}

impl<'a> Haystack<'a> {
    pub fn new(value: &'a str) -> Haystack<'a> {
        Haystack {
            iter: value.bytes().peekable(),
            start: true,
        }
    }

    pub fn byte(&mut self) -> Option<u8> {
        self.iter.peek().copied()
    }

    // Progression is only completed by elements which explicitly check the byte and succeed.
    pub fn progress(&mut self) {
        self.iter.next();
        self.start = false;
    }

    pub fn is_start(&mut self) -> bool {
        self.start
    }

    pub fn is_end(&mut self) -> bool {
        // TODO: Check that there is no other way of getting a None
        self.byte().is_none()
    }
}

trait Matcher {
    fn matches(hay: &mut Haystack) -> bool;
}

#[derive(Debug, Default)]
struct Byte<const N: u8>;

impl<const N: u8> Matcher for Byte<N> {
    fn matches(hay: &mut Haystack) -> bool {
        if hay.byte() == Some(N) {
            hay.progress();
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
struct ByteRange<const A: u8, const B: u8>;

impl<const A: u8, const B: u8> Matcher for ByteRange<A, B> {
    fn matches(hay: &mut Haystack) -> bool {
        if let Some(byte) = hay.byte() && A <= byte && byte <= B {
            hay.progress();
            true
        } else {
            false
        }
    }
}

type ClassDigit = ByteRange<b'0', b'9'>;
type ClassWord = Or<Or<ByteRange<b'A', b'Z'>, ByteRange<b'a', b'z'>>, Or<ClassDigit, Byte<b'_'>>>;

#[derive(Debug, Default)]
struct Or<A: Matcher, B: Matcher>(PhantomData<A>, PhantomData<B>);

impl<A: Matcher, B: Matcher> Matcher for Or<A, B> {
    fn matches(hay: &mut Haystack) -> bool {
        let rollback = hay.clone();
        if A::matches(hay) {
            true
        } else {
            *hay = rollback;
            B::matches(hay)
        }
    }
}

#[derive(Debug, Default)]
struct Then<A: Matcher, B: Matcher>(PhantomData<A>, PhantomData<B>);

impl<A: Matcher, B: Matcher> Matcher for Then<A, B> {
    fn matches(hay: &mut Haystack) -> bool {
        if A::matches(hay) {
            B::matches(hay)
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
struct AnyNumber<A: Matcher>(PhantomData<A>);

impl<A: Matcher> Matcher for AnyNumber<A> {
    fn matches(hay: &mut Haystack) -> bool {
        while A::matches(hay) {}
        true
    }
}

#[derive(Debug, Default)]
struct OneOrMore<A: Matcher>(PhantomData<A>);

impl<A: Matcher> Matcher for OneOrMore<A> {
    fn matches(hay: &mut Haystack) -> bool {
        let mut matched = false;
        while A::matches(hay) {
            matched = true;
        }
        matched
    }
}

#[derive(Debug, Default)]
struct QuantifierN<A: Matcher, const N: usize>(PhantomData<A>);

impl<A: Matcher, const N: usize> Matcher for QuantifierN<A, N> {
    fn matches(hay: &mut Haystack) -> bool {
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
        }
        matches == N
    }
}

#[derive(Debug, Default)]
struct QuantifierNOrMore<A: Matcher, const N: usize>(PhantomData<A>);

impl<A: Matcher, const N: usize> Matcher for QuantifierNOrMore<A, N> {
    fn matches(hay: &mut Haystack) -> bool {
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
        }
        matches >= N
    }
}

#[derive(Debug, Default)]
struct QuantifierNToM<A: Matcher, const N: usize, const M: usize>(PhantomData<A>);

impl<A: Matcher, const N: usize, const M: usize> Matcher for QuantifierNToM<A, N, M> {
    fn matches(hay: &mut Haystack) -> bool {
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
        }
        N <= matches && matches <= M
    }
}

#[derive(Debug, Default)]
struct Optional<A: Matcher>(PhantomData<A>);

impl<A: Matcher> Matcher for Optional<A> {
    fn matches(hay: &mut Haystack) -> bool {
        A::matches(hay);
        true
    }
}

#[derive(Debug, Default)]
struct Beginning;

impl Matcher for Beginning {
    fn matches(hay: &mut Haystack) -> bool {
        hay.is_start()
    }
}

#[derive(Debug, Default)]
struct End;

impl Matcher for End {
    fn matches(hay: &mut Haystack) -> bool {
        hay.is_end()
    }
}

#[derive(Debug, Default)]
struct Always;

impl Matcher for Always {
    fn matches(_: &mut Haystack) -> bool {
        true
    }
}

trait HirExtension {
    fn into_type_expr(self) -> String;
}

impl HirExtension for Hir {
    fn into_type_expr(self) -> String {
        let mut string = String::new();
        self.write_type_expr(&mut string).unwrap();
        string
    }
}

trait WriteTypeExpr {
    fn write_type_expr(self, f: &mut String) -> fmt::Result;
}

impl WriteTypeExpr for Hir {
    fn write_type_expr(self, f: &mut String) -> fmt::Result {
        match self.into_kind() {
            HirKind::Empty              => write_empty(f),
            HirKind::Literal(lit)       => write_literal(f, lit),
            HirKind::Class(class)       => write_class(f, class),
            HirKind::Look(look)         => write_look(f, look),
            HirKind::Repetition(rep)    => write_repetition(f, rep),
            HirKind::Capture(cap)       => write_capture(f, cap),
            HirKind::Concat(hirs)       => write_concat(f, hirs),
            HirKind::Alternation(hirs)  => write_alternation(f, hirs),
        }
    }
}

impl WriteTypeExpr for u8 {
    fn write_type_expr(self, f: &mut String) -> fmt::Result {
        write!(f, "{}<{}u8>", type_name::<Byte<0>>(), self)
    }
}

impl WriteTypeExpr for &ClassBytesRange {
    fn write_type_expr(self, f: &mut String) -> fmt::Result {
        write!(f, "{}<{}u8,{}u8>", type_name::<ByteRange<0, 0>>(), self.start(), self.end())
    }
}

fn write_nested_pairs<T>(
    f: &mut String,
    into_iter: impl IntoIterator<
        IntoIter = impl ExactSizeIterator<
            Item = impl WriteTypeExpr
        >
    >
) -> fmt::Result {
    let iter = into_iter.into_iter();
    let last = iter.len() - 1;
    for (i, expr) in iter.into_iter().enumerate() {
        if i == last {
            expr.write_type_expr(f)?;
        } else {
            write!(f, "{}<", type_name::<T>())?;
            expr.write_type_expr(f)?;
            write!(f, ",")?;
        }
    }
    for _ in 0..last {
        write!(f, ">")?;
    }
    Ok(())
}

fn write_empty(f: &mut String) -> fmt::Result {
    write!(f, "{}", type_name::<Always>())
}

fn write_literal(f: &mut String, lit: Literal) -> fmt::Result {
    write_nested_pairs::<Then<Always, Always>>(f, lit.0)
}

fn write_class(f: &mut String, class: Class) -> fmt::Result {
    match class {
        Class::Unicode(unicode) => todo!("unicode classes {:?}", unicode),
        Class::Bytes(bytes) => write_nested_pairs::<Or<Always, Always>>(f, bytes.ranges()),
    }
}

fn write_look(f: &mut String, look: Look) -> fmt::Result {
    match look {
        Look::Start => write!(f, "{}", type_name::<Beginning>()),
        Look::End => write!(f, "{}", type_name::<End>()),
        _ => todo!("complex looking"),
    }
}

fn write_repetition(f: &mut String, rep: Repetition) -> fmt::Result {
    let Repetition { min, max, greedy, sub } = rep;
    if !greedy {
        todo!("lazy repetition")
    }
    match max {
        None => {
            write!(f, "{}<", type_name::<QuantifierNOrMore<Always, 0>>())?;
            sub.write_type_expr(f)?;
            write!(f, ",{}>", min)
        },
        Some(max) if min == max => {
            write!(f, "{}<", type_name::<QuantifierN<Always, 0>>())?;
            sub.write_type_expr(f)?;
            write!(f, ",{}>", min)
        },
        Some(max) => {
            write!(f, "{}<", type_name::<QuantifierNToM<Always, 0, 0>>())?;
            sub.write_type_expr(f)?;
            write!(f, ",{},{}>", min, max)
        },
    }
}

fn write_capture(f: &mut String, cap: Capture) -> fmt::Result {
    // TODO: Actually handle captures
    cap.sub.write_type_expr(f)
}

fn write_concat(f: &mut String, hirs: Vec<Hir>) -> fmt::Result {
    write_nested_pairs::<Then<Always, Always>>(f, hirs)
}

fn write_alternation(f: &mut String, hirs: Vec<Hir>) -> fmt::Result {
    write_nested_pairs::<Or<Always, Always>>(f, hirs)
}

fn type_name<T>() -> &'static str {
    any::type_name::<T>().split('<').next().unwrap()
}
// TODO: Lazy, Sets?, Negated Sets, Groups