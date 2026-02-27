use std::fmt::{self, Write};

use regex_syntax::hir::{Capture, Class, ClassBytesRange, ClassUnicodeRange, Hir, HirKind, Literal, Look, Repetition};

use crate::{haystack::HaystackItem, hir::util::type_name, matcher::{Always, Beginning, Byte, ByteRange, End, Or, QuantifierN, QuantifierNOrMore, QuantifierNToM, QuantifierThen, Scalar, ScalarRange, Then}};

use Always as A;

pub trait WriteMatcher {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result;
}

impl WriteMatcher for Hir {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        match self.into_kind() {
            HirKind::Empty              => Empty.write_matcher::<I>(f),
            HirKind::Literal(lit)       => lit.write_matcher::<I>(f),
            HirKind::Class(class)       => class.write_matcher::<I>(f),
            HirKind::Look(look)         => look.write_matcher::<I>(f),
            HirKind::Repetition(rep)    => rep.write_matcher::<I>(f),
            HirKind::Capture(cap)       => cap.write_matcher::<I>(f),
            HirKind::Concat(hirs)       => Concat(hirs).write_matcher::<I>(f),
            HirKind::Alternation(hirs)  => Alternation(hirs).write_matcher::<I>(f),
        }
    }
}

#[derive(Debug)]
struct Empty;

#[derive(Debug)]
struct Concat(pub Vec<Hir>);

#[derive(Debug)]
struct Alternation(pub Vec<Hir>);

#[derive(Debug)]
pub struct Backtrack {
    rep: Repetition,
    then: Vec<Hir>,
}

impl WriteMatcher for u8 {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        assert_eq!(type_name::<I>(), type_name::<u8>());
        write!(f, "{}<{}u8>", type_name::<Byte<0>>(), self)
    }
}

impl WriteMatcher for &ClassBytesRange {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        assert_eq!(type_name::<I>(), type_name::<u8>());
        write!(f, "{}<{}u8,{}u8>", type_name::<ByteRange<0, 0>>(), self.start(), self.end())
    }
}

impl WriteMatcher for char {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        assert_eq!(type_name::<I>(), type_name::<char>());
        write!(f, "{}<'{}'>", type_name::<Scalar<'a'>>(), self.escape_unicode())
    }
}

impl WriteMatcher for &ClassUnicodeRange {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        assert_eq!(type_name::<I>(), type_name::<char>(), "{:?}", self);
        write!(f, "{}<'{}','{}'>",
            type_name::<ScalarRange<'a', 'a'>>(),
            self.start().escape_unicode(),
            self.end().escape_unicode()
        )
    }
}

impl WriteMatcher for Empty {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        write!(f, "{}", type_name::<Always>())
    }
}

impl WriteMatcher for Literal {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        write_chunked::<Then<u8, A, A>, I, _>(
            f,
            I::vec_from_str(
                str::from_utf8(&self.0)
                    .expect("failed to convert bytes to valid unicode")
            )
        )
    }
}

impl WriteMatcher for Class {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        match I::cast_class(self) {
            Class::Unicode(unicode) => write_chunked::<Or<u8, A, A>, I, _>(
                f,
                unicode.ranges().iter().collect()
            ),
            Class::Bytes(bytes) => write_chunked::<Or<u8, A, A>, I, _>(
                f,
                bytes.ranges().iter().collect()
            ),
        }
    }
}

impl WriteMatcher for Look {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        match self {
            Look::Start => write!(f, "{}", type_name::<Beginning>()),
            Look::End => write!(f, "{}", type_name::<End>()),
            _ => todo!("complex looking"),
        }
    }
}

impl WriteMatcher for Repetition {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        let Repetition { min, max, greedy, sub } = self;
        if !greedy {
            todo!("lazy repetition")
        }
        match max {
            None => {
                write!(f, "{}<{},", type_name::<QuantifierNOrMore<u8, A, 0>>(), type_name::<I>())?;
                sub.write_matcher::<I>(f)?;
                write!(f, ",{}>", min)
            },
            Some(max) if min == max => {
                write!(f, "{}<{},", type_name::<QuantifierN<u8, A, 0>>(), type_name::<I>())?;
                sub.write_matcher::<I>(f)?;
                write!(f, ",{}>", min)
            },
            Some(max) => {
                write!(f, "{}<{},", type_name::<QuantifierNToM<u8, A, 0, 0>>(), type_name::<I>())?;
                sub.write_matcher::<I>(f)?;
                write!(f, ",{},{}>", min, max)
            },
        }
    }
}

impl WriteMatcher for Capture {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        // TODO: Actually handle captures
        self.sub.write_matcher::<I>(f)
    }
}

impl WriteMatcher for Alternation {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        write_chunked::<Or<u8, A, A>, I, _>(f, self.0)
    }
}

impl WriteMatcher for Concat {
    fn write_matcher<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        let mut iter = self.0.into_iter();
        let mut rep_item = None;
        let concat = Concat(
            iter.by_ref()
                .take_while(|i| if let HirKind::Repetition(rep) = i.kind() {
                    rep_item = Some(rep.clone());
                    false
                } else {
                    true
                })
                .collect()
        );
        if let Some(rep) = rep_item {
            let backtrack = Backtrack {
                rep,
                then: iter.collect(),
            };

            match (concat.0.len(), backtrack.then.len()) {
                (0, 0) => unreachable!(),
                (0, _) => backtrack.write_matcher::<I>(f),
                (_, _) => {
                    write!(f, "{}<{},", type_name::<Then<u8, A, A>>(), type_name::<I>())?;
                    concat.write_type_basic::<I>(f)?;
                    write!(f, ",")?;
                    backtrack.write_matcher::<I>(f)?;
                    write!(f, ">")
                },
            }
        } else {
            concat.write_type_basic::<I>(f)
        }
    }
}

impl WriteMatcher for Backtrack {
    fn write_matcher<I: HaystackItem>(mut self, f: &mut String) -> fmt::Result {
        if self.then.is_empty() {
            return self.rep.write_matcher::<I>(f);
        }

        write!(f, "{}<{},", type_name::<QuantifierThen<u8, A, A>>(), type_name::<I>())?;
        self.rep.write_matcher::<I>(f)?;
        write!(f, ",")?;
        match self.then.len() {
            0 => unreachable!(),
            1 => self.then.pop().unwrap().write_matcher::<I>(f),
            _ => Concat(self.then).write_matcher::<I>(f)
        }?;
        write!(f, ">")
    }
}

impl Concat {
    fn write_type_basic<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        write_chunked::<Then<u8, A, A>, I, _>(f, self.0)
    }
}

fn write_chunked<T, I: HaystackItem, W: WriteMatcher>(
    f: &mut String,
    mut items: Vec<W>,
) -> fmt::Result {
    let n = items.len();
    let base = type_name::<T>();
    let item_type = type_name::<I>();

    match n {
        0 => panic!("literal contains no items"),
        1 => items.pop().unwrap().write_matcher::<I>(f),
        2 => {
            let mut iter = items.into_iter();
            write!(f, "{}<{},", base, item_type)?;
            iter.next().unwrap().write_matcher::<I>(f)?;
            write!(f, ",")?;
            iter.next().unwrap().write_matcher::<I>(f)?;
            write!(f, ">")
        }
        3 => {
            let mut iter = items.into_iter();
            write!(f, "{}<{},", base, item_type)?;
            iter.next().unwrap().write_matcher::<I>(f)?;
            write!(f, ",")?;
            write_chunked::<T, I, W>(f, iter.collect())?;
            write!(f, ">")
        }
        4 | 8 | 16 => write_n_items::<T, I, W>(f, items, n),
        _ => {
            // Take largest chunk that fits, combine with remainder
            let chunk_size = if n > 16 { 16 } else if n > 8 { 8 } else { 4 };
            let remainder = items.split_off(chunk_size);
            write!(f, "{}<{},", base, item_type)?;
            write_n_items::<T, I, W>(f, items, chunk_size)?;
            write!(f, ",")?;
            write_chunked::<T, I, W>(f, remainder)?;
            write!(f, ">")
        }
    }
}

fn write_n_items<T, I: HaystackItem, W: WriteMatcher>(
    f: &mut String,
    items: Vec<W>,
    n: usize,
) -> fmt::Result {
    let base = type_name::<T>();
    let item_type = type_name::<I>();

    write!(f, "{}{}<{}", base, n, item_type)?;
    for item in items {
        write!(f, ",")?;
        item.write_matcher::<I>(f)?;
    }
    write!(f, ">")
}