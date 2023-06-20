use bluejay_core::AsIter;
use bluejay_parser::ast::executable::{Field, Selection, SelectionSet};
use itertools::Either;
use std::rc::Rc;

#[derive(PartialEq, Eq, Hash)]
pub(crate) enum SelectionSetProvider<'a> {
    SelectionSet(&'a SelectionSet<'a>),
    Fields(Rc<Vec<&'a Field<'a>>>),
}

impl<'a> SelectionSetProvider<'a> {
    pub(crate) fn selection_set(&self) -> impl Iterator<Item = &'a Selection<'a>> + '_ {
        match self {
            Self::SelectionSet(ss) => Either::Left(ss.iter()),
            Self::Fields(fields) => Either::Right(
                fields
                    .iter()
                    .copied()
                    .filter_map(|field| {
                        field
                            .selection_set()
                            .map(|selection_set| selection_set.iter())
                    })
                    .flatten(),
            ),
        }
    }
}

impl<'a> From<Rc<Vec<&'a Field<'a>>>> for SelectionSetProvider<'a> {
    fn from(value: Rc<Vec<&'a Field<'a>>>) -> Self {
        Self::Fields(value)
    }
}
