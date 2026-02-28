use std::fmt::{self, Write};

use regex_syntax::hir::{Capture, Class, ClassBytesRange, ClassUnicodeRange, Hir, HirKind, Literal, Look, Repetition};

use crate::{haystack::HaystackItem, hir::{Groups, util::type_name}, matcher::{Always, Always as A, Beginning, Byte, ByteRange, CaptureGroup, End, Or, QuantifierN, QuantifierNOrMore, QuantifierNToM, QuantifierThen, Scalar, ScalarRange, Then}};

pub trait WriteMatcher {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, caps: &mut Groups) -> fmt::Result;
}

impl WriteMatcher for Hir {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, caps: &mut Groups) -> fmt::Result {
        match self.into_kind() {
            HirKind::Empty              => Empty.write_matcher::<I>(f, caps),
            HirKind::Literal(lit)       => lit.write_matcher::<I>(f, caps),
            HirKind::Class(class)       => class.write_matcher::<I>(f, caps),
            HirKind::Look(look)         => look.write_matcher::<I>(f, caps),
            HirKind::Repetition(rep)    => rep.write_matcher::<I>(f, caps),
            HirKind::Capture(cap)       => cap.write_matcher::<I>(f, caps),
            HirKind::Concat(hirs)       => Concat(hirs).write_matcher::<I>(f, caps),
            HirKind::Alternation(hirs)  => Alternation(hirs).write_matcher::<I>(f, caps),
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
struct Backtrack {
    rep: Repetition,
    then: Vec<Hir>,
}

impl WriteMatcher for u8 {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, _caps: &mut Groups) -> fmt::Result {
        assert_eq!(type_name::<I>(), type_name::<u8>());
        write!(f, "{}<{}u8>", type_name::<Byte<0>>(), self)
    }
}

impl WriteMatcher for &ClassBytesRange {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, caps: &mut Groups) -> fmt::Result {
        assert_eq!(type_name::<I>(), type_name::<u8>());
        if self.start() == self.end() {
            self.start().write_matcher::<I>(f, caps)
        } else {
            write!(f, "{}<{}u8,{}u8>", type_name::<ByteRange<0, 0>>(), self.start(), self.end())
        }
    }
}

impl WriteMatcher for char {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, _caps: &mut Groups) -> fmt::Result {
        assert_eq!(type_name::<I>(), type_name::<char>());
        write!(f, "{}<'{}'>", type_name::<Scalar<'a'>>(), self.escape_unicode())
    }
}

impl WriteMatcher for &ClassUnicodeRange {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, caps: &mut Groups) -> fmt::Result {
        assert_eq!(type_name::<I>(), type_name::<char>(), "{:?}", self);
        if self.start() == self.end() {
            self.start().write_matcher::<I>(f, caps)
        } else {
            write!(f, "{}<'{}','{}'>",
                type_name::<ScalarRange<'a', 'a'>>(),
                self.start().escape_unicode(),
                self.end().escape_unicode()
            )
        }
    }
}

impl WriteMatcher for Empty {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, _caps: &mut Groups) -> fmt::Result {
        write!(f, "{}", type_name::<Always>())
    }
}

impl WriteMatcher for Literal {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, caps: &mut Groups) -> fmt::Result {
        write_chunked::<Then<u8, A, A>, I, _>(
            f, caps,
            I::vec_from_str(
                str::from_utf8(&self.0)
                    .expect("failed to convert bytes to valid unicode")
            )
        )
    }
}

impl WriteMatcher for Class {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, caps: &mut Groups) -> fmt::Result {
        match I::cast_class(self) {
            Class::Unicode(unicode) => write_chunked::<Or<u8, A, A>, I, _>(
                f, caps,
                unicode.ranges().iter().collect()
            ),
            Class::Bytes(bytes) => write_chunked::<Or<u8, A, A>, I, _>(
                f, caps,
                bytes.ranges().iter().collect()
            ),
        }
    }
}

impl WriteMatcher for Look {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, _caps: &mut Groups) -> fmt::Result {
        match self {
            Look::Start => write!(f, "{}", type_name::<Beginning>()),
            Look::End => write!(f, "{}", type_name::<End>()),
            _ => todo!("complex looking"),
        }
    }
}

impl WriteMatcher for Repetition {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, caps: &mut Groups) -> fmt::Result {
        let Repetition { min, max, greedy, sub } = self;
        if !greedy {
            todo!("lazy repetition")
        }

        let required = caps.required;
        if min == 0 {
            caps.required = false;
        }

        match max {
            None => {
                write!(f, "{}<{},", type_name::<QuantifierNOrMore<u8, A, 0>>(), type_name::<I>())?;
                sub.write_matcher::<I>(f, caps)?;
                write!(f, ",{}>", min)?;
            },
            Some(max) if min == max => {
                write!(f, "{}<{},", type_name::<QuantifierN<u8, A, 0>>(), type_name::<I>())?;
                sub.write_matcher::<I>(f, caps)?;
                write!(f, ",{}>", min)?;
            },
            Some(max) => {
                write!(f, "{}<{},", type_name::<QuantifierNToM<u8, A, 0, 0>>(), type_name::<I>())?;
                sub.write_matcher::<I>(f, caps)?;
                write!(f, ",{},{}>", min, max)?;
            },
        }

        if min == 0 {
            caps.required = required;
        }

        Ok(())
    }
}

impl WriteMatcher for Capture {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, caps: &mut Groups) -> fmt::Result {
        caps.insert(self.index, self.name);
        write!(f, "{}<{},", type_name::<CaptureGroup<u8, A, 0>>(), type_name::<I>())?;
        self.sub.write_matcher::<I>(f, caps)?;
        write!(f, ",{}>", self.index)
    }
}

impl WriteMatcher for Alternation {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, caps: &mut Groups) -> fmt::Result {
        let required = caps.required;
        caps.required = false;
        write_chunked::<Or<u8, A, A>, I, _>(f, caps, self.0)?;
        caps.required = required;
        Ok(())
    }
}

impl WriteMatcher for Concat {
    fn write_matcher<I: HaystackItem>(self, f: &mut String, caps: &mut Groups) -> fmt::Result {
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
                (0, _) => backtrack.write_matcher::<I>(f, caps),
                (_, _) => {
                    write!(f, "{}<{},", type_name::<Then<u8, A, A>>(), type_name::<I>())?;
                    concat.write_type_basic::<I>(f, caps)?;
                    write!(f, ",")?;
                    backtrack.write_matcher::<I>(f, caps)?;
                    write!(f, ">")
                },
            }
        } else {
            concat.write_type_basic::<I>(f, caps)
        }
    }
}

impl WriteMatcher for Backtrack {
    fn write_matcher<I: HaystackItem>(mut self, f: &mut String, caps: &mut Groups) -> fmt::Result {
        if self.then.is_empty() {
            return self.rep.write_matcher::<I>(f, caps);
        }

        write!(f, "{}<{},", type_name::<QuantifierThen<u8, A, A>>(), type_name::<I>())?;
        self.rep.write_matcher::<I>(f, caps)?;
        write!(f, ",")?;
        match self.then.len() {
            0 => unreachable!(),
            1 => self.then.pop().unwrap().write_matcher::<I>(f, caps),
            _ => Concat(self.then).write_matcher::<I>(f, caps)
        }?;
        write!(f, ">")
    }
}

impl Concat {
    fn write_type_basic<I: HaystackItem>(self, f: &mut String, caps: &mut Groups) -> fmt::Result {
        write_chunked::<Then<u8, A, A>, I, _>(f, caps, self.0)
    }
}

fn write_chunked<T, I: HaystackItem, W: WriteMatcher>(
    f: &mut String,
    caps: &mut Groups,
    mut items: Vec<W>,
) -> fmt::Result {
    let n = items.len();
    let base = type_name::<T>();
    let item_type = type_name::<I>();

    match n {
        0 => panic!("literal contains no items"),
        1 => items.pop().unwrap().write_matcher::<I>(f, caps),
        2 => {
            let mut iter = items.into_iter();
            write!(f, "{}<{},", base, item_type)?;
            iter.next().unwrap().write_matcher::<I>(f, caps)?;
            write!(f, ",")?;
            iter.next().unwrap().write_matcher::<I>(f, caps)?;
            write!(f, ">")
        }
        3 => {
            let mut iter = items.into_iter();
            write!(f, "{}<{},", base, item_type)?;
            iter.next().unwrap().write_matcher::<I>(f, caps)?;
            write!(f, ",")?;
            write_chunked::<T, I, W>(f, caps, iter.collect())?;
            write!(f, ">")
        }
        4 | 8 | 16 => write_n_items::<T, I, W>(f, caps, items, n),
        _ => {
            // Take largest chunk that fits, combine with remainder
            let chunk_size = if n > 16 { 16 } else if n > 8 { 8 } else { 4 };
            let remainder = items.split_off(chunk_size);
            write!(f, "{}<{},", base, item_type)?;
            write_n_items::<T, I, W>(f, caps, items, chunk_size)?;
            write!(f, ",")?;
            write_chunked::<T, I, W>(f, caps, remainder)?;
            write!(f, ">")
        }
    }
}

fn write_n_items<T, I: HaystackItem, W: WriteMatcher>(
    f: &mut String,
    caps: &mut Groups,
    items: Vec<W>,
    n: usize,
) -> fmt::Result {
    let base = type_name::<T>();
    let item_type = type_name::<I>();

    write!(f, "{}{}<{}", base, n, item_type)?;
    for item in items {
        write!(f, ",")?;
        item.write_matcher::<I>(f, caps)?;
    }
    write!(f, ">")
}