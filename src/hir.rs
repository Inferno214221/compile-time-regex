use std::{any, fmt::{self, Write}};

use regex_syntax::hir::{Capture, Class, ClassBytesRange, Hir, HirKind, Literal, Look, Repetition};

use crate::matcher::{Always, Beginning, Byte, ByteRange, End, Or, QuantifierN, QuantifierNOrMore, QuantifierNToM, Then};

pub trait HirExtension {
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