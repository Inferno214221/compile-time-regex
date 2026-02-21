use std::{any, fmt::{self, Write}};

use regex_syntax::hir::{Capture, Class, ClassBytesRange, Hir, HirKind, Literal, Look, Repetition};

use crate::matcher::{Always, Beginning, Byte, ByteRange, End, Or, QuantifierN, QuantifierNOrMore, QuantifierNToM, Then};

fn type_name<T>() -> &'static str {
    any::type_name::<T>().split('<').next().unwrap()
}

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
            HirKind::Empty              => Empty.write_type_expr(f),
            HirKind::Literal(lit)       => lit.write_type_expr(f),
            HirKind::Class(class)       => class.write_type_expr(f),
            HirKind::Look(look)         => look.write_type_expr(f),
            HirKind::Repetition(rep)    => rep.write_type_expr(f),
            HirKind::Capture(cap)       => cap.write_type_expr(f),
            HirKind::Concat(hirs)       => Concat(hirs).write_type_expr(f),
            HirKind::Alternation(hirs)  => Alternation(hirs).write_type_expr(f),
        }
    }
}

struct Empty;
struct Concat(pub Vec<Hir>);
struct Alternation(pub Vec<Hir>);

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

impl WriteTypeExpr for Empty {
    fn write_type_expr(self, f: &mut String) -> fmt::Result {
        write!(f, "{}", type_name::<Always>())
    }
}

impl WriteTypeExpr for Literal {
    fn write_type_expr(self, f: &mut String) -> fmt::Result {
        write_nested_pairs::<Then<Always, Always>>(f, self.0)
    }
}

impl WriteTypeExpr for Class {
    fn write_type_expr(self, f: &mut String) -> fmt::Result {
        match self {
            Class::Unicode(unicode) => todo!("unicode classes {:?}", unicode),
            Class::Bytes(bytes) => write_nested_pairs::<Or<Always, Always>>(f, bytes.ranges()),
        }
    }
}

impl WriteTypeExpr for Look {
    fn write_type_expr(self, f: &mut String) -> fmt::Result {
        match self {
            Look::Start => write!(f, "{}", type_name::<Beginning>()),
            Look::End => write!(f, "{}", type_name::<End>()),
            _ => todo!("complex looking"),
        }
    }
}

impl WriteTypeExpr for Repetition {
    fn write_type_expr(self, f: &mut String) -> fmt::Result {
        let Repetition { min, max, greedy, sub } = self;
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
}

impl WriteTypeExpr for Capture {
    fn write_type_expr(self, f: &mut String) -> fmt::Result {
        // TODO: Actually handle captures
        self.sub.write_type_expr(f)
    }
}

impl WriteTypeExpr for Concat {
    fn write_type_expr(self, f: &mut String) -> fmt::Result {
        write_nested_pairs::<Then<Always, Always>>(f, self.0)
    }
}

impl WriteTypeExpr for Alternation {
    fn write_type_expr(self, f: &mut String) -> fmt::Result {
        write_nested_pairs::<Or<Always, Always>>(f, self.0)
    }
}