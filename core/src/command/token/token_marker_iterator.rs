use super::{Meta, TokenMatch};

pub struct TokenMarkerIterator<'a, 'b> {
    token_match_set: TokenMatchSet<'a>,
    markers: &'b [u8],
    inner: Option<Box<TokenMarkerIterator<'a, 'b>>>,
    cursor: usize,
}

enum TokenMatchSet<'a> {
    Slice(&'a [TokenMatch<'a>]),
    Single(&'a TokenMatch<'a>),
}

impl<'a, 'b> TokenMarkerIterator<'a, 'b> {
    pub fn new(token_match: &'a TokenMatch, markers: &'b [u8]) -> Self {
        Self {
            token_match_set: TokenMatchSet::Single(token_match),
            markers,
            inner: None,
            cursor: 0,
        }
    }
}

impl<'a> TokenMatchSet<'a> {
    fn get(&self, index: usize) -> Option<&'a TokenMatch<'a>> {
        match self {
            TokenMatchSet::Slice(slice) => slice.get(index),
            TokenMatchSet::Single(token_match) if index == 0 => Some(token_match),
            TokenMatchSet::Single(_) => None,
        }
    }
}

impl<'a> TryFrom<&'a Meta<'a>> for TokenMatchSet<'a> {
    type Error = ();

    fn try_from(input: &'a Meta<'a>) -> Result<Self, Self::Error> {
        match input {
            Meta::Sequence(vec) => Ok(TokenMatchSet::Slice(vec.as_slice())),
            Meta::Single(single) => Ok(TokenMatchSet::Single(single.as_ref())),
            _ => Err(()),
        }
    }
}

impl<'a, 'b> Iterator for TokenMarkerIterator<'a, 'b> {
    type Item = &'a TokenMatch<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner) = &mut self.inner {
                if let Some(next) = inner.next() {
                    return Some(next);
                } else {
                    self.inner = None;
                }
            }

            if let Some(token_match) = self.token_match_set.get(self.cursor) {
                if let Ok(token_match_set) = (&token_match.meta).try_into() {
                    self.inner = Some(Box::new(TokenMarkerIterator {
                        token_match_set,
                        markers: self.markers,
                        inner: None,
                        cursor: 0,
                    }));
                }

                self.cursor += 1;

                if token_match
                    .token
                    .marker
                    .map_or(false, |m| self.markers.contains(&m))
                {
                    return Some(token_match);
                }
            } else {
                return None;
            }
        }
    }
}
