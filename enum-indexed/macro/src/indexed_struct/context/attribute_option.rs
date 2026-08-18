use std::ops::ControlFlow;

use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::{Attribute, Error, Expr, Lit, LitBool, LitStr, Meta, PatLit, Result, Token};

pub(super) trait ExprExt {
    fn require_lit_bool(&self) -> Result<&LitBool>;
    fn require_lit_str(&self) -> Result<&LitStr>;
}

impl ExprExt for Expr {
    fn require_lit_bool(&self) -> Result<&LitBool> {
        let Expr::Lit(PatLit {
            lit: Lit::Bool(lit_bool),
            ..
        }) = self
        else {
            return Err(Error::new(self.span(), "expected literal bool"));
        };

        Ok(lit_bool)
    }

    fn require_lit_str(&self) -> Result<&LitStr> {
        let Expr::Lit(PatLit {
            lit: Lit::Str(lit_str),
            ..
        }) = self
        else {
            return Err(Error::new(self.span(), "expected literal string"));
        };

        Ok(lit_str)
    }
}

pub(super) trait AttrOption<T> {
    fn set(&mut self, value: T, span: Span) -> Result<()>;
}

#[derive(Debug, Clone)]
pub(super) struct OptionalAttrOption<T>(Option<(T, Span)>);

impl<T> OptionalAttrOption<T> {
    pub(super) fn take(self) -> Option<T> {
        let (value, _) = self.0?;
        Some(value)
    }

    pub(super) fn into_option(self) -> Option<(T, Span)> {
        self.0
    }
}

impl<T> AttrOption<T> for OptionalAttrOption<T> {
    fn set(&mut self, value: T, span: Span) -> Result<()> {
        if let Some((_, prev_span)) = self.0 {
            let mut error = Error::new(span, "The attribute option can only be used once");
            error.combine(Error::new(
                prev_span,
                "The attribute option was previously used here",
            ));
            return Err(error);
        }

        self.0 = Some((value, span));
        Ok(())
    }
}

impl<T> Default for OptionalAttrOption<T> {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Debug, Clone)]
pub(super) struct ListAttrOption<T>(Vec<(T, Span)>);

impl<T> ListAttrOption<T> {
    pub(super) fn take(self) -> Vec<T> {
        self.0.into_iter().map(|(value, _)| value).collect()
    }
}

impl<T> AttrOption<T> for ListAttrOption<T> {
    fn set(&mut self, value: T, span: Span) -> Result<()> {
        self.0.push((value, span));
        Ok(())
    }
}

impl<T> Default for ListAttrOption<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

pub(super) struct AttrOptProcessor<'a>(&'a Meta);

impl AttrOptProcessor<'_> {
    pub(super) fn accept<T>(
        &self,
        ident: &str,
        option: &mut impl AttrOption<T>,
        value: impl FnOnce(&Meta) -> Result<T>,
    ) -> ControlFlow<Result<()>, ()> {
        if self.0.path().is_ident(ident) {
            match value(self.0) {
                Ok(value) => {
                    let result = option.set(value, self.0.path().span());
                    ControlFlow::Break(result)
                }
                Err(e) => ControlFlow::Break(Err(e)),
            }
        } else {
            ControlFlow::Continue(())
        }
    }

    #[expect(clippy::unused_self)]
    pub(super) fn reject_others(self) -> ControlFlow<Result<()>> {
        ControlFlow::Continue(())
    }
}

pub(super) fn parse(
    attrs: &[Attribute],
    mut f: impl FnMut(AttrOptProcessor) -> ControlFlow<Result<()>, ()>,
) -> Result<()> {
    for attr in attrs {
        if attr.path().is_ident("indexed_struct") {
            let metas = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

            for meta in metas {
                match f(AttrOptProcessor(&meta)) {
                    ControlFlow::Continue(()) => {
                        return Err(Error::new(
                            meta.path().span(),
                            "Unrecognized attribute option",
                        ));
                    }
                    ControlFlow::Break(result) => result?,
                }
            }
        }
    }

    Ok(())
}

pub(super) fn lit_str_from_name_value_meta(meta: &Meta) -> Result<LitStr> {
    let name_value = meta.require_name_value()?;
    let lit_str = name_value.value.require_lit_str()?;
    Ok(lit_str.clone())
}
