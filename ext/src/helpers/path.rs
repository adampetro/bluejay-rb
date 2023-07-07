use std::rc::Rc;

#[derive(Clone)]
pub enum PathElement<'a> {
    Key(&'a str),
    Index(usize),
}

impl<'a> From<&'a str> for PathElement<'a> {
    fn from(s: &'a str) -> Self {
        Self::Key(s)
    }
}

impl From<usize> for PathElement<'_> {
    fn from(i: usize) -> Self {
        Self::Index(i)
    }
}

impl std::fmt::Display for PathElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathElement::Key(s) => write!(f, "{}", s),
            PathElement::Index(i) => write!(f, "{}", i),
        }
    }
}

struct PathInner<'a> {
    element: PathElement<'a>,
    parent: Option<Rc<Self>>,
}

impl<'a> PathInner<'a> {
    fn new(element: impl Into<PathElement<'a>>) -> Self {
        Self {
            element: element.into(),
            parent: None,
        }
    }

    fn append(rc_self: Rc<Self>, element: impl Into<PathElement<'a>>) -> Rc<Self> {
        Rc::new(Self {
            element: element.into(),
            parent: Some(rc_self.clone()),
        })
    }
}

#[derive(Clone, Default)]
pub struct Path<'a>(Option<Rc<PathInner<'a>>>);

impl<'a> Path<'a> {
    pub fn new(element: impl Into<PathElement<'a>>) -> Self {
        Self(Some(Rc::new(PathInner::new(element))))
    }

    pub fn append(&self, element: impl Into<PathElement<'a>>) -> Self {
        match &self.0 {
            Some(inner) => Self(Some(PathInner::append(inner.clone(), element))),
            None => Self(Some(Rc::new(PathInner::new(element)))),
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.clone().map(|s| s.to_string()).collect()
    }
}

impl<'a> Iterator for Path<'a> {
    type Item = PathElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(inner) = self.0.take() {
            self.0 = inner.parent.clone();
            Some(inner.element.clone())
        } else {
            None
        }
    }
}
