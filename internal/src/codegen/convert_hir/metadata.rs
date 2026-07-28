use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub name: Option<Box<str>>,
    pub required: bool,
}

#[derive(Debug, Default, Clone)]
pub struct ExprMetadata {
    // Group ids are provided by regex_syntax as a u32, and not returned in the right order, so we
    // store them in a map before checking them and creating a Vec later.
    pub groups: HashMap<u32, Group>,
    pub required: bool,
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
        }
    }

    pub fn insert_group(&mut self, index: u32, name: Option<Box<str>>) {
        self.groups.insert(index, Group {
            name,
            required: self.required,
        });
    }

    pub fn take_groups(self) -> Vec<Group> {
        let mut items: Vec<_> = self.groups.into_iter().collect();
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
}
