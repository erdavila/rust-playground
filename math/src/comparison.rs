use core::any::Any;
use core::iter::{Once, once};

use crate::ComparisonTokens;

#[derive(Debug, PartialEq, Eq)]
pub struct Id {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

#[macro_export]
macro_rules! comparison_id {
    ($name:ident) => {
        $crate::comparison::Id {
            name: stringify!($name),
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}

pub trait AnyEq: Any {
    fn any_eq(&self, other: &dyn AnyEq) -> bool;
}
impl<T> AnyEq for T
where
    T: PartialEq + 'static,
{
    fn any_eq(&self, other: &dyn AnyEq) -> bool {
        let other: &dyn Any = other;
        other
            .downcast_ref::<Self>()
            .is_some_and(|other| self == other)
    }
}

pub enum Token<'a> {
    Id(Id),
    Value(&'a dyn AnyEq),
    End,
}

pub struct TokensBuilder<'a, I>(I)
where
    I: Iterator<Item = Token<'a>>;
impl<'a> TokensBuilder<'a, Once<Token<'a>>> {
    #[must_use]
    pub fn new(id: Id) -> Self {
        TokensBuilder(once(Token::Id(id)))
    }
}
impl<'a, I> TokensBuilder<'a, I>
where
    I: Iterator<Item = Token<'a>>,
{
    pub fn value(self, value: &'a dyn AnyEq) -> TokensBuilder<'a, impl Iterator<Item = Token<'a>>> {
        self.token(Token::Value(value))
    }

    pub fn value_lazy(
        self,
        value_fn: impl FnOnce() -> &'a dyn AnyEq,
    ) -> TokensBuilder<'a, impl Iterator<Item = Token<'a>>> {
        self.token_lazy(|| Token::Value(value_fn()))
    }

    pub fn from_child(
        self,
        child: &'a impl ComparisonTokens,
    ) -> TokensBuilder<'a, impl Iterator<Item = Token<'a>>> {
        self.tokens_lazy(|| child.comparison_tokens())
    }

    pub fn token(self, token: Token<'a>) -> TokensBuilder<'a, impl Iterator<Item = Token<'a>>> {
        self.tokens([token])
    }

    pub fn token_lazy(
        self,
        token_fn: impl FnOnce() -> Token<'a>,
    ) -> TokensBuilder<'a, impl Iterator<Item = Token<'a>>> {
        self.tokens_lazy(|| [token_fn()])
    }

    pub fn tokens(
        self,
        tokens: impl IntoIterator<Item = Token<'a>>,
    ) -> TokensBuilder<'a, impl Iterator<Item = Token<'a>>> {
        self.chain(tokens)
    }

    pub fn tokens_lazy<Tks>(
        self,
        tokens_fn: impl FnOnce() -> Tks,
    ) -> TokensBuilder<'a, impl Iterator<Item = Token<'a>>>
    where
        Tks: IntoIterator<Item = Token<'a>>,
    {
        // Put the `tokens: FnOnce` into an Option so that it can be used in the `FnMut` argument of
        // `flat_map`.
        let mut tokens = Some(tokens_fn);

        let lazy_iter = once(())
            .filter_map(move |()| tokens.take().map(|tokens| tokens()))
            .flatten();

        self.chain(lazy_iter)
    }

    pub fn build(self) -> impl Iterator<Item = Token<'a>> {
        self.token(Token::End).0
    }

    fn chain(
        self,
        tokens: impl IntoIterator<Item = Token<'a>>,
    ) -> TokensBuilder<'a, impl Iterator<Item = Token<'a>>> {
        let tokens = self.0.chain(tokens);
        TokensBuilder(tokens)
    }
}
