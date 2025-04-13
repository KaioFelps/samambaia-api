use std::borrow::Cow;
use std::collections::HashSet;
use std::hash::Hash;

pub trait ChangeSetTrait: Hash + Eq + ToOwned + Clone {}

impl<T: Hash + ToOwned + Clone + Eq> ChangeSetTrait for T {}

#[derive(Clone, Debug)]
pub struct BlankChangeSet<T: ChangeSetTrait> {
    current: HashSet<T>,
}

#[derive(Clone, Debug)]
pub struct FilledChangeSet<T: ChangeSetTrait> {
    current: HashSet<T>,
    new: HashSet<T>,
}

#[derive(Clone, Debug)]
pub enum ChangeSet<T: ChangeSetTrait> {
    Blank(BlankChangeSet<T>),
    Filled(FilledChangeSet<T>),
}

impl<T: ChangeSetTrait> ChangeSet<T> {
    pub fn new(tags: Vec<T>) -> Self {
        Self::Blank(BlankChangeSet::new(tags))
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn has_changes(&self) -> bool {
        match self {
            Self::Blank(_) => false,
            Self::Filled(filled) => filled.differ().has_changes(),
        }
    }

    pub fn get_filled(&self) -> Option<&FilledChangeSet<T>> {
        match self {
            Self::Blank(_) => None,
            Self::Filled(filled) => Some(filled),
        }
    }

    pub fn get_current(&self) -> Vec<&T> {
        match self {
            Self::Blank(changeset) => changeset.get_current(),
            Self::Filled(changeset) => changeset.get_current(),
        }
    }

    pub fn flush(self) -> Self {
        match self {
            ChangeSet::Blank(blank) => blank.into(),
            ChangeSet::Filled(filled) => BlankChangeSet {
                current: filled.new,
            }
            .into(),
        }
    }
}

impl<T: ChangeSetTrait> BlankChangeSet<T> {
    pub fn new(tags: Vec<T>) -> Self {
        Self {
            current: HashSet::from_iter(tags),
        }
    }

    pub fn into_filled(self, other: Vec<T>) -> FilledChangeSet<T> {
        FilledChangeSet {
            current: self.current,
            new: HashSet::from_iter(other),
        }
    }

    pub fn get_current(&self) -> Vec<&T> {
        self.current.iter().collect::<Vec<_>>()
    }
}

impl<T: ChangeSetTrait> FilledChangeSet<T> {
    pub fn differ(&self) -> Changes<'_, T> {
        Changes {
            added: self
                .current
                .iter()
                .filter(|curr_tag| !self.new.contains(curr_tag))
                .collect(),

            removed: self
                .new
                .iter()
                .filter(|new_tag| !self.current.contains(new_tag))
                .collect(),
        }
    }

    pub fn get_current(&self) -> Vec<&T> {
        self.current.iter().collect()
    }

    pub fn get_new(&self) -> Vec<&T> {
        self.new.iter().collect()
    }

    pub fn renew_with(self, r#new: Vec<T>) -> Self {
        Self {
            current: self.current,
            new: HashSet::from_iter(new),
        }
    }
}

impl<T: ChangeSetTrait> Default for ChangeSet<T> {
    fn default() -> Self {
        Self::Blank(BlankChangeSet::new(Vec::new()))
    }
}

impl<T: ChangeSetTrait> Default for BlankChangeSet<T> {
    fn default() -> Self {
        Self {
            current: HashSet::new(),
        }
    }
}

#[derive(Debug)]
pub struct Changes<'a, T: ToOwned> {
    pub removed: Cow<'a, [&'a T]>,
    pub added: Cow<'a, [&'a T]>,
}

impl<T: ToOwned> Changes<'_, T> {
    pub fn has_changes(&self) -> bool {
        !self.removed.is_empty() || !self.added.is_empty()
    }
}

impl<T: ChangeSetTrait> PartialEq for BlankChangeSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current
    }
}

impl<T: ChangeSetTrait> PartialEq for FilledChangeSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current
    }
}

impl<T: ChangeSetTrait> PartialEq for ChangeSet<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Blank(l0), Self::Blank(r0)) => l0 == r0,
            (Self::Filled(l0), Self::Filled(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<T: ChangeSetTrait> Eq for ChangeSet<T> {}
impl<T: ChangeSetTrait> Eq for FilledChangeSet<T> {}
impl<T: ChangeSetTrait> Eq for BlankChangeSet<T> {}

impl<T: ChangeSetTrait> From<BlankChangeSet<T>> for ChangeSet<T> {
    fn from(value: BlankChangeSet<T>) -> Self {
        Self::Blank(value)
    }
}

impl<T: ChangeSetTrait> From<FilledChangeSet<T>> for ChangeSet<T> {
    fn from(value: FilledChangeSet<T>) -> Self {
        Self::Filled(value)
    }
}

impl<T: ChangeSetTrait> From<Vec<T>> for ChangeSet<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Blank(BlankChangeSet::new(value))
    }
}
