use std::{iter::Peekable, marker::PhantomData, str::Bytes};

fn main() {
    type E = Or::<Byte<0u8>, Or<Byte<1u8>, Byte<2u8>>>;
    dbg!(E::default());
    type F = Then::<Byte<b'e'>, Then<Byte<b'a'>, Byte<b's'>>>;
    dbg!(F::matches(&mut Haystack::new("eas")));
    dbg!(F::matches(&mut Haystack::new("nope")));
    dbg!(F::matches(&mut Haystack::new("ea")));
    // We've matched the whole thing if there is only one remaining byte
    dbg!(F::matches(&mut Haystack::new("easp")));
    // FIXME: /ea|s/ can't match "s" atm because it has progressed the haystack
    // If a Matcher invokes another matcher but accepts a fail, it needs to rollback
    type G = Or<Then<Byte<b'e'>, Byte<b'a'>>, Byte<b's'>>;
    dbg!(G::matches(&mut Haystack::new("ea")));
    type H = Then<Then<Byte<b'e'>, Optional<Byte<b'a'>>>, Byte<b's'>>;
    dbg!(H::matches(&mut Haystack::new("es")));

    let mut a = Haystack::new("A");
    let mut b = a.clone();
    b.progress();
    assert_ne!(a.byte(), b.byte());
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
            dbg!(B::matches(hay))
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
struct QuantifierN<A: Matcher, const N: u8>(PhantomData<A>);

impl<A: Matcher, const N: u8> Matcher for QuantifierN<A, N> {
    fn matches(hay: &mut Haystack) -> bool {
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
        }
        matches == N
    }
}

#[derive(Debug, Default)]
struct QuantifierNOrMore<A: Matcher, const N: u8>(PhantomData<A>);

impl<A: Matcher, const N: u8> Matcher for QuantifierNOrMore<A, N> {
    fn matches(hay: &mut Haystack) -> bool {
        let mut matches = 0;
        while A::matches(hay) {
            matches += 1;
        }
        matches >= N
    }
}

#[derive(Debug, Default)]
struct QuantifierNToM<A: Matcher, const N: u8, const M: u8>(PhantomData<A>);

impl<A: Matcher, const N: u8, const M: u8> Matcher for QuantifierNToM<A, N, M> {
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

// TODO: Lazy, Sets?, Negated Sets, Groups