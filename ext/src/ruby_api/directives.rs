use crate::ruby_api::Directive;
use bluejay_core::Directives as CoreDirectives;
use magnus::{gc, Error, RArray};

#[derive(Debug)]
pub struct Directives {
    directives: Vec<Directive>,
    rarray: RArray,
}

impl CoreDirectives<true> for Directives {
    type Directive = Directive;
}

impl AsRef<[Directive]> for Directives {
    fn as_ref(&self) -> &[Directive] {
        &self.directives
    }
}

impl From<&Directives> for RArray {
    fn from(value: &Directives) -> Self {
        value.rarray
    }
}

impl Directives {
    pub(crate) fn empty() -> Self {
        let rarray = RArray::new();
        rarray.freeze();
        Self {
            directives: Vec::new(),
            rarray,
        }
    }

    pub(crate) fn mark(&self) {
        self.directives.iter().for_each(Directive::mark);
        gc::mark(self.rarray);
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
