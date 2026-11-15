use std::collections::BTreeMap;
use std::mem;

use quote::format_ident;
use syn::Ident;

use crate::codegen::{CodegenItem};
use crate::matcher::ClassEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Group {
    pub name: Option<Box<str>>,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ExprMetadata<I: CodegenItem> {
    pub name: Ident,
    // Group ids are provided by regex_syntax in a strange order, so we store them by index in a Vec
    // and checking them later.
    pub groups: Vec<Option<Group>>,
    pub required: bool,
    pub literals: BTreeMap<Box<[u8]>, Ident>,
    pub classes: BTreeMap<Box<[ClassEntry<I>]>, Ident>,
}

impl<I: CodegenItem> ExprMetadata<I> {
    pub fn new(name: Ident) -> ExprMetadata<I> {
        let groups = vec![
            Some(Group {
                name: Some("whole_match".into()),
                required: true,
            })
        ];
        ExprMetadata {
            name,
            groups,
            required: true,
            literals: BTreeMap::new(),
            classes: BTreeMap::new(),
        }
    }

    pub fn insert_group(&mut self, index: u32, name: Option<Box<str>>) {
        let index = index as usize;
        // Pad the vec with Nones first if required.
        while self.groups.len() <= index {
            self.groups.push(None);
        }

        let group = Some(Group {
            name,
            required: self.required,
        });

        if self.groups.len() == index {
            self.groups.push(group);
        } else {
            self.groups[index] = group;
        }
    }

    pub fn take_groups(&mut self) -> Vec<Group> {
        let items: Vec<_> = mem::take(&mut self.groups)
            .into_iter()
            .collect();

        // If we have any gaps in the group list, panic.
        if items.iter().any(Option::is_none) {
            panic!("missing a capture group");
        }

        items.into_iter()
            .flatten()
            .collect()
    }

    pub fn insert_literal(&mut self, literal: Box<[u8]>) -> Ident {
        let index = self.literals.len();
        self.literals.entry(literal)
            .or_insert_with(|| create_literal_id(&self.name, index))
            .clone()
    }

    pub fn insert_class(&mut self, entry: Box<[ClassEntry<I>]>) -> Ident {
        let index = self.classes.len();

        // Entries are already sorted by regex_syntax.

        self.classes.entry(entry)
            .or_insert_with(|| create_class_id::<I>(&self.name, index))
            .clone()
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
    format_ident!("{}{}Class{}", name, I::ident_fragment(), num)
}