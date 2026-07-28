use std::{collections::HashMap, mem};

use quote::format_ident;
use syn::Ident;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub name: Option<Box<str>>,
    pub required: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExprMetadata {
    // Group ids are provided by regex_syntax as a u32, and not returned in the right order, so we
    // store them in a map before checking them and creating a Vec later.
    pub groups: HashMap<u32, Group>,
    pub required: bool,
    pub literals: Vec<Box<[u8]>>,
}

impl ExprMetadata {
    pub fn new() -> ExprMetadata {
        let mut groups = HashMap::new();
        groups.insert(0, Group {
            name: Some("whole_match".into()),
            required: true,
        });
        ExprMetadata {
            groups,
            required: true,
            literals: Vec::new(),
        }
    }

    pub fn insert_group(&mut self, index: u32, name: Option<Box<str>>) {
        self.groups.insert(index, Group {
            name,
            required: self.required,
        });
    }

    pub fn take_groups(&mut self) -> Vec<Group> {
        let mut items: Vec<_> = mem::take(&mut self.groups).into_iter().collect();
        items.sort_by_key(|(i, _)| *i);

        if items
            .iter()
            .enumerate()
            .any(|(num, (index, _))| num != *index as usize)
        {
            panic!("missing a capture group");
        }

        items.into_iter().map(|(_, item)| item).collect()
    }

    pub fn insert_literal(&mut self, literal: Box<[u8]>) -> Ident {
        let index = self.literals.len();
        self.literals.push(literal);
        create_literal_id(index)
    }
}

pub fn create_literal_id(num: usize) -> Ident {
    format_ident!("__regex_Literal{}", num)
}
