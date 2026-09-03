use std::collections::HashMap;
use std::mem;

use quote::format_ident;
use syn::Ident;

use crate::codegen::{CodegenItem, type_ident};
use crate::matcher::ClassEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Group {
    pub name: Option<Box<str>>,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ExprMetadata<I: CodegenItem> {
    pub name: Ident,
    // Group ids are provided by regex_syntax as a u32, and not returned in the right order, so we
    // store them in a map before checking them and creating a Vec later.
    pub groups: HashMap<u32, Group>,
    pub required: bool,
    pub literals: Vec<Box<[u8]>>,
    // TODO: avoid duplicates
    pub classes: Vec<Box<[ClassEntry<I>]>>,
}

impl<I: CodegenItem> ExprMetadata<I> {
    pub fn new(name: Ident) -> ExprMetadata<I> {
        let mut groups = HashMap::new();
        groups.insert(0, Group {
            name: Some("whole_match".into()),
            required: true,
        });
        ExprMetadata {
            name,
            groups,
            required: true,
            literals: Vec::new(),
            classes: Vec::new(),
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
        create_literal_id(&self.name, index)
    }

    pub fn insert_class(&mut self, mut entry: Box<[ClassEntry<I>]>) -> Ident {
        let index = self.classes.len();
        entry.sort();
        self.classes.push(entry);
        create_class_id::<I>(&self.name, index)
    }
}

impl<I: CodegenItem, J: CodegenItem> PartialEq<ExprMetadata<J>> for ExprMetadata<I> {
    fn eq(&self, other: &ExprMetadata<J>) -> bool {
        self.name == other.name
            && self.groups == other.groups
            && self.literals == other.literals
    }
}

pub(crate) fn create_literal_id(name: &Ident, num: usize) -> Ident {
    format_ident!("{}Literal{}", name, num)
}

pub(crate) fn create_class_id<I: CodegenItem>(name: &Ident, num: usize) -> Ident {
    format_ident!("{}{}Class{}", name, type_ident::<I>(), num)
}