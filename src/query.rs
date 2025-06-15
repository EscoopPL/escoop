use std::{collections::HashMap, ops::Range};

use codespan_reporting::files::{self, Files};

use crate::Source;

pub(crate) struct __Source {
    pub(crate) name: String,
    pub(crate) contents: String,
    pub(crate) line_starts: Vec<usize>,
}

pub(crate) enum QueryValue {
    Source(__Source),
}

#[derive(Default)]
pub struct Database {
    storage: HashMap<u32, QueryValue>,
    id: u32,
}

impl Database {
    pub(crate) fn get(&self, id: u32) -> Option<&QueryValue> {
        self.storage.get(&id)
    }

    fn id(&mut self) -> u32 {
        let id = self.id;
        self.id += 1;
        id
    }

    pub(crate) fn create(&mut self, value: QueryValue) -> u32 {
        let id = self.id();
        self.storage.insert(id, value);
        id
    }
}

impl<'db> Files<'db> for Database {
    type FileId = Source;

    type Name = &'db str;

    type Source = &'db str;

    fn name(&'db self, id: Self::FileId) -> Result<Self::Name, codespan_reporting::files::Error> {
        Ok(id.name(self))
    }

    fn source(
        &'db self,
        id: Self::FileId,
    ) -> Result<Self::Source, codespan_reporting::files::Error> {
        Ok(id.contents(self))
    }

    fn line_index(&'db self, id: Self::FileId, byte_index: usize) -> Result<usize, files::Error> {
        Ok(id
            .line_starts(self)
            .binary_search(&byte_index)
            .unwrap_or_else(|next_line| next_line - 1))
    }

    fn line_range(
        &self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>, files::Error> {
        let get_line_start = |line_index: usize| -> Result<usize, files::Error> {
            use core::cmp::Ordering;

            match line_index.cmp(&id.line_starts(self).len()) {
                Ordering::Less => Ok(id
                    .line_starts(self)
                    .get(line_index)
                    .cloned()
                    .expect("failed despite previous check")),
                Ordering::Equal => Ok(id.contents(self).len()),
                Ordering::Greater => Err(files::Error::LineTooLarge {
                    given: line_index,
                    max: id.line_starts(self).len() - 1,
                }),
            }
        };

        let line_start = get_line_start(line_index)?;
        let next_line_start = get_line_start(line_index + 1)?;

        Ok(line_start..next_line_start)
    }
}
