use std::{collections::HashMap, ops::Range};

#[derive(Debug, Clone)]
pub struct Group {
    pub name: Option<Box<str>>,
    pub required: bool,
    // *..0 indicates "not only digits"
    pub digits: Range<usize>,
}

#[derive(Debug, Default, Clone)]
pub struct Groups {
    pub map: HashMap<u32, Group>,
    pub required: bool,
    pub only_digits: bool,
}

impl Groups {
    pub fn new() -> Groups {
        let mut map = HashMap::new();
        map.insert(0, Group {
            name: Some("whole_match".into()),
            required: true,
            // FIXME: Incorrect default, I should probably add whole match somewhere else.
            digits: 0..0,
        });
        Groups {
            map,
            required: true,
            only_digits: true,
        }
    }

    pub fn insert(&mut self, index: u32, name: Option<Box<str>>, digits: Range<usize>) {
        self.map.insert(index, Group {
            name,
            required: self.required,
            digits,
        });
    }

    pub fn set_digits(&mut self, index: u32, digits: Range<usize>) {
        self.map.get_mut(&index)
            .expect("error setting digits of missing capture group")
            .digits = digits;
    }

    pub fn into_vec(self) -> Vec<Group> {
        let mut items: Vec<_> = self.map.into_iter().collect();
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
