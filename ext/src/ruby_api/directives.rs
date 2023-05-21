use crate::ruby_api::Directive;
use bluejay_core::{AsIter, Directives as CoreDirectives};
use magnus::{gc, Error, RArray};
use std::ops::Not;

#[derive(Debug)]
pub struct Directives {
    directives: Vec<Directive>,
    rarray: RArray,
}

impl CoreDirectives<true> for Directives {
    type Directive = Directive;
}

impl AsIter for Directives {
    type Item = Directive;
    type Iterator<'a> = std::slice::Iter<'a, Directive>;

    fn iter(&self) -> Self::Iterator<'_> {
        self.directives.iter()
    }
}

impl From<&Directives> for RArray {
    fn from(value: &Directives) -> Self {
        value.rarray
    }
}

impl Directives {
    pub(crate) fn mark(&self) {
        self.directives.iter().for_each(Directive::mark);
        gc::mark(self.rarray);
    }

    pub(crate) fn to_option(&self) -> Option<&Self> {
        self.directives.is_empty().not().then_some(self)
    }
}

impl TryFrom<Option<RArray>> for Directives {
    type Error = Error;

    fn try_from(value: Option<RArray>) -> Result<Self, Self::Error> {
        value.unwrap_or(RArray::new()).try_into()
    }
}

impl TryFrom<RArray> for Directives {
    type Error = Error;

    fn try_from(rarray: RArray) -> Result<Self, Self::Error> {
        rarray.freeze();
        let directives: Result<Vec<Directive>, Error> = unsafe { rarray.as_slice() }
            .iter()
            .map(|val| -> Result<Directive, Error> { val.try_convert() })
            .collect();
        directives.map(|directives| Self { directives, rarray })
    }
}
