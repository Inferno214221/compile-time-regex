use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Group {
    pub name: Option<Box<str>>,
    pub required: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Groups {
    pub map: HashMap<u32, Group>,
    pub required: bool,
}

impl Groups {
    pub fn new() -> Groups {
        let mut map = HashMap::new();
        map.insert(0, Group {
            name: Some("whole_match".into()),
            required: true,
        });
        Groups {
            map,
            required: true,
        }
    }

    pub fn insert(&mut self, index: u32, name: Option<Box<str>>) {
        self.map.insert(index, Group {
            name,
            required: self.required,
        });
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
