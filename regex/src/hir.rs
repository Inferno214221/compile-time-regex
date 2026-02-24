use std::{any, fmt::{self, Write}};

use regex_syntax::hir::{Capture, Class, ClassBytesRange, ClassUnicodeRange, Hir, HirKind, Literal, Look, Repetition};

use crate::{haystack::HaystackItem, matcher::{Always, Beginning, Byte, ByteRange, End, Or, QuantifierN, QuantifierNOrMore, QuantifierNToM, Scalar, ScalarRange, Then}};

pub fn type_name<T>() -> &'static str {
    any::type_name::<T>().split('<').next().unwrap()
}

pub trait HirExtension {
    fn into_type_expr<I: HaystackItem>(self) -> String;
}

impl HirExtension for Hir {
    fn into_type_expr<I: HaystackItem>(self) -> String {
        let mut string = String::new();
        self.write_type_expr::<I>(&mut string).unwrap();
        string
    }
}

trait WriteTypeExpr {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result;
}

impl WriteTypeExpr for Hir {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        match self.into_kind() {
            HirKind::Empty              => Empty.write_type_expr::<I>(f),
            HirKind::Literal(lit)       => lit.write_type_expr::<I>(f),
            HirKind::Class(class)       => class.write_type_expr::<I>(f),
            HirKind::Look(look)         => look.write_type_expr::<I>(f),
            HirKind::Repetition(rep)    => rep.write_type_expr::<I>(f),
            HirKind::Capture(cap)       => cap.write_type_expr::<I>(f),
            HirKind::Concat(hirs)       => Concat(hirs).write_type_expr::<I>(f),
            HirKind::Alternation(hirs)  => Alternation(hirs).write_type_expr::<I>(f),
        }
    }
}

struct Empty;
struct Concat(pub Vec<Hir>);
struct Alternation(pub Vec<Hir>);

impl WriteTypeExpr for u8 {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        assert_eq!(type_name::<I>(), type_name::<u8>());
        write!(f, "{}<{}u8>", type_name::<Byte<0>>(), self)
    }
}

impl WriteTypeExpr for &ClassBytesRange {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        assert_eq!(type_name::<I>(), type_name::<u8>());
        write!(f, "{}<{}u8,{}u8>", type_name::<ByteRange<0, 0>>(), self.start(), self.end())
    }
}

impl WriteTypeExpr for char {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        assert_eq!(type_name::<I>(), type_name::<char>());
        write!(f, "{}<'{}'>", type_name::<Scalar<'a'>>(), self.escape_unicode())
    }
}

impl WriteTypeExpr for &ClassUnicodeRange {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        assert_eq!(type_name::<I>(), type_name::<char>());
        write!(f, "{}<'{}','{}'>",
            type_name::<ScalarRange<'a', 'a'>>(),
            self.start().escape_unicode(),
            self.end().escape_unicode()
        )
    }
}

fn write_nested_pairs<T, I: HaystackItem>(
    f: &mut String,
    into_iter: impl IntoIterator<
        IntoIter = impl ExactSizeIterator<
            Item = impl WriteTypeExpr
        >
    >
) -> fmt::Result {
    let iter = into_iter.into_iter();
    let last = iter.len() - 1;
    for (i, expr) in iter.enumerate() {
        if i == last {
            expr.write_type_expr::<I>(f)?;
        } else {
            write!(f, "{}<{},", type_name::<T>(), type_name::<I>())?;
            expr.write_type_expr::<I>(f)?;
            write!(f, ",")?;
        }
    }
    for _ in 0..last {
        write!(f, ">")?;
    }
    Ok(())
}

impl WriteTypeExpr for Empty {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        write!(f, "{}", type_name::<Always>())
    }
}

impl WriteTypeExpr for Literal {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        write_nested_pairs::<Then<u8, Always, Always>, I>(f, self.0)
    }
}

impl WriteTypeExpr for Class {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        match self {
            Class::Unicode(unicode) => write_nested_pairs::<Or<u8, Always, Always>, I>(f, unicode.ranges()),
            Class::Bytes(bytes) => write_nested_pairs::<Or<u8, Always, Always>, I>(f, bytes.ranges()),
        }
    }
}

impl WriteTypeExpr for Look {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        match self {
            Look::Start => write!(f, "{}", type_name::<Beginning>()),
            Look::End => write!(f, "{}", type_name::<End>()),
            _ => todo!("complex looking"),
        }
    }
}

impl WriteTypeExpr for Repetition {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        let Repetition { min, max, greedy, sub } = self;
        if !greedy {
            todo!("lazy repetition")
        }
        match max {
            None => {
                write!(f, "{}<{},", type_name::<QuantifierNOrMore<u8, Always, 0>>(), type_name::<I>())?;
                sub.write_type_expr::<I>(f)?;
                write!(f, ",{}>", min)
            },
            Some(max) if min == max => {
                write!(f, "{}<{},", type_name::<QuantifierN<u8, Always, 0>>(), type_name::<I>())?;
                sub.write_type_expr::<I>(f)?;
                write!(f, ",{}>", min)
            },
            Some(max) => {
                write!(f, "{}<{},", type_name::<QuantifierNToM<u8, Always, 0, 0>>(), type_name::<I>())?;
                sub.write_type_expr::<I>(f)?;
                write!(f, ",{},{}>", min, max)
            },
        }
    }
}

impl WriteTypeExpr for Capture {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        // TODO: Actually handle captures
        self.sub.write_type_expr::<I>(f)
    }
}

impl WriteTypeExpr for Concat {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        write_nested_pairs::<Then<u8, Always, Always>, I>(f, self.0)
    }
}

impl WriteTypeExpr for Alternation {
    fn write_type_expr<I: HaystackItem>(self, f: &mut String) -> fmt::Result {
        write_nested_pairs::<Or<u8, Always, Always>, I>(f, self.0)
    }
}