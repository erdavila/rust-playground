use proc_macro2::{
    Delimiter, Group, Literal, Punct, Spacing, TokenStream as TokenStream2, TokenTree,
};
use std::fs::File;
use std::io::{self, Write};
use std::ops::Deref;

use syn::punctuated::{Pair, Punctuated};
use syn::{
    Abi, AngleBracketedGenericArguments, Arm, AttrStyle, Attribute, BinOp, Block, Expr, ExprAssign,
    ExprAssignOp, ExprBinary, ExprBlock, ExprCall, ExprCast, ExprIf, ExprLet, ExprLit, ExprLoop,
    ExprMatch, ExprMethodCall, ExprPath, ExprReference, ExprReturn, ExprUnsafe, Field, Fields,
    FieldsUnnamed, FnArg, GenericArgument, GenericMethodArgument, GenericParam, Generics, Ident,
    Item, ItemEnum, ItemFn, ItemMod, ItemStatic, Label, Lifetime, Lit, LitInt, LitStr, Local,
    MethodTurbofish, Pat, PatIdent, PatLit, PatTuple, PatTupleStruct, PatType, PatWild, Path,
    PathArguments, PathSegment, QSelf, ReturnType, Signature, Stmt, Token, Type, TypeParam,
    TypeParamBound, TypePath, TypePtr, TypeReference, TypeTuple, Variadic, Variant, VisPublic,
    Visibility,
};

use crate::indentation::Indentation;

pub fn dump_item(item: &Item, name: &str) {
    let mut w = File::create(name).unwrap();

    item.dump(&mut w, Indentation::new(4)).unwrap();
}

//---------------------------------------------------------------------------------------

macro_rules! impl_dump_for_token {
    ($token:tt) => {
        impl Dump for Token![$token] {
            fn dump(&self, w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
                write!(w, stringify!($token))
            }
        }
    };
}

macro_rules! impl_dump_for_struct {
    ($name:ident, [$($([$attr:ident]:)? $member:ident),*]) => {
        impl Dump for $name {
            fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
                $(
                    _impl_dump_for_struct_member!(self, w, indentation, $($attr)?: $member);
                )*
                Ok(())
            }
        }
    };
}

macro_rules! _impl_dump_for_struct_member {
    ($self:ident, $w:ident, $indentation:ident, spaced: $member:ident) => {
        $self.$member.dump_spaced($w, $indentation)?;
    };
    ($self:ident, $w:ident, $indentation:ident, with_space: $member:ident) => {
        $self.$member.dump_with_space($w, $indentation)?;
    };
    ($self:ident, $w:ident, $indentation:ident, : $member:ident) => {
        $self.$member.dump($w, $indentation)?;
    };
}

macro_rules! impl_dump_for_enum {
    (
        $name:ident
        $(, 0: [$($var0:ident),*])?
        $(, 1: [$($var1:ident),*])?
        $(, 2: [$($var2:ident),*])?
        $(, $($dummy:ident)? _)?
    ) => {
        impl Dump for $name {
            #[allow(unused_variables)]
            fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
                match self {
                    $(
                        $(
                            $name::$var0 => (),
                        )*
                    )?
                    $(
                        $(
                            $name::$var1(x) => x.dump(w, indentation)?,
                        )*
                    )?
                    $(
                        $(
                            $name::$var2(x, y) => {
                                x.dump(w, indentation)?;
                                y.dump(w, indentation)?;
                            },
                        )*
                    )?
                    $(
                        $($dummy)? _ => todo!("at {}:{}: {:?}", file!(), line!(), self),
                    )?
                }
                Ok(())
            }
        }
    };
}

//---------------------------------------------------------------------------------------

trait Dump {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()>;

    fn dump_with_space(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.dump(w, indentation)?;
        write!(w, " ")
    }

    fn dump_ln(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.dump(w, indentation)?;
        writeln!(w)
    }
}

impl<T: Dump, U: Dump> Dump for (T, U) {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.0.dump(w, indentation)?;
        self.1.dump(w, indentation)
    }
}

impl Dump for Abi {
    fn dump(&self, _w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        todo!("at {}:{}", file!(), line!());
    }
}

impl_dump_for_token![+]; // Add
impl_dump_for_token![+=]; // AddEq
impl_dump_for_token![&]; // And
impl_dump_for_struct!(
    AngleBracketedGenericArguments,
    [colon2_token, lt_token, [spaced]: args, gt_token]
);

impl Dump for Arm {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.pat.dump_with_space(w, indentation)?;
        if let Some(guard) = &self.guard {
            guard.0.dump_with_space(w, indentation)?;
            guard.1.dump_with_space(w, indentation)?;
        }
        self.fat_arrow_token.dump_with_space(w, indentation)?;
        self.body.dump(w, indentation)?;
        self.comma.dump(w, indentation)
    }
}

impl_dump_for_token![as];
impl_dump_for_token![async];
impl_dump_for_token![@]; // At

impl Dump for Attribute {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.pound_token.dump(w, indentation)?;
        self.style.dump(w, indentation)?;
        write!(w, "[")?;
        self.path.dump(w, indentation)?;
        self.tokens.dump(w, indentation)?;
        write!(w, "]")
    }
}

impl_dump_for_enum!(AttrStyle, 0: [Outer], _);
impl_dump_for_enum!(BinOp, 1: [AddEq, Lt], _);

impl Dump for Block {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        writeln!(w, "{{")?;
        self.stmts.dump_lines(w, indentation.next())?;
        write!(w, "{indentation}}}")
    }
}

impl<T: Dump> Dump for Box<T> {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.deref().dump(w, indentation)
    }
}

impl_dump_for_token![:]; // Colon
impl_dump_for_token![::]; // Colon2
impl_dump_for_token![,]; // Comma
impl_dump_for_token![const];
impl_dump_for_token![.]; // Dot
impl_dump_for_token![else];
impl_dump_for_token![enum];
impl_dump_for_token![=]; // Eq
impl_dump_for_enum!(Expr, 1: [Assign, AssignOp, Binary, Block, Call, Cast, If, Let, Lit, Loop, Match, MethodCall, Path, Reference, Return, Unsafe], _);
impl_dump_for_struct!(
    ExprAssign,
    [
        [spaced]: attrs,
        [with_space]: left,
        [with_space]: eq_token,
        right
    ]
);
impl_dump_for_struct!(
    ExprAssignOp,
    [[spaced]: attrs, [with_space]: left, [with_space]: op, right]
);
impl_dump_for_struct!(
    ExprBinary,
    [[spaced]: attrs, [with_space]: left, [with_space]: op, right]
);
impl_dump_for_struct!(ExprBlock, [[spaced]: attrs, [with_space]: label, block]);

impl Dump for ExprCall {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.func.dump(w, indentation)?;
        write!(w, "(")?;
        self.args.dump_spaced(w, indentation)?;
        write!(w, ")")
    }
}

impl_dump_for_struct!(
    ExprCast,
    [
        [spaced]: attrs,
        [with_space]: expr,
        [with_space]: as_token,
        ty
    ]
);

impl Dump for ExprIf {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.if_token.dump_with_space(w, indentation)?;
        self.cond.dump_with_space(w, indentation)?;
        self.then_branch.dump(w, indentation)?;
        if let Some(else_branch) = &self.else_branch {
            write!(w, " ")?;
            else_branch.0.dump_with_space(w, indentation)?;
            else_branch.1.dump(w, indentation)?;
        }
        Ok(())
    }
}

impl_dump_for_struct!(
    ExprLet,
    [
        [spaced]: attrs,
        [with_space]: let_token,
        [with_space]: pat,
        [with_space]: eq_token,
        expr
    ]
);
impl_dump_for_struct!(ExprLit, [[spaced]: attrs, lit]);
impl_dump_for_struct!(
    ExprLoop,
    [
        [spaced]: attrs,
        [with_space]: label,
        [with_space]: loop_token,
        body
    ]
);

impl Dump for ExprMatch {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.match_token.dump_with_space(w, indentation)?;
        self.expr.dump(w, indentation)?;
        writeln!(w, " {{")?;
        self.arms.dump_lines(w, indentation.next())?;
        write!(w, "{indentation}}}")
    }
}

impl Dump for ExprMethodCall {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.receiver.dump(w, indentation)?;
        self.dot_token.dump(w, indentation)?;
        self.method.dump(w, indentation)?;
        self.turbofish.dump(w, indentation)?;
        write!(w, "(")?;
        self.args.dump_spaced(w, indentation)?;
        write!(w, ")")
    }
}

impl_dump_for_struct!(ExprPath, [[spaced]: attrs, qself, path]);
impl_dump_for_struct!(
    ExprReference,
    [[spaced]: attrs, and_token, [with_space]: mutability, expr]
);

impl Dump for ExprReturn {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.return_token.dump(w, indentation)?;
        if let Some(expr) = &self.expr {
            write!(w, " ")?;
            expr.dump(w, indentation)?;
        }
        Ok(())
    }
}

impl_dump_for_struct!(
    ExprUnsafe,
    [[spaced]: attrs, [with_space]: unsafe_token, block]
);
impl_dump_for_token![=>]; // FatArrow

impl Dump for Field {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.vis.dump_with_space(w, indentation)?;
        self.ident.dump(w, indentation)?;
        if let Some(colon_token) = self.colon_token {
            colon_token.dump(w, indentation)?;
            write!(w, " ")?;
        }
        self.ty.dump(w, indentation)
    }
}

impl_dump_for_enum!(Fields, 1: [Unnamed], _);

impl Dump for FieldsUnnamed {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        write!(w, "(")?;
        self.unnamed.dump_spaced(w, indentation)?;
        write!(w, ")")
    }
}

impl_dump_for_token![fn];
impl_dump_for_enum!(FnArg, 1: [Typed], _);
impl_dump_for_enum!(GenericArgument, 1: [Type], _);

impl Dump for GenericMethodArgument {
    fn dump(&self, _w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        todo!("at line {}: {:?}", line!(), self)
    }
}

impl_dump_for_enum!(GenericParam, 1: [Type], _);

impl Dump for Generics {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.lt_token.dump(w, indentation)?;
        self.params.dump_spaced(w, indentation)?;
        self.gt_token.dump(w, indentation)?;

        if self.where_clause.is_some() {
            todo!("at {}:{}", file!(), line!());
        }

        Ok(())
    }
}

impl Dump for Group {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        let (open, close): (&str, &str) = match self.delimiter() {
            Delimiter::Parenthesis => ("(", ")"),
            x => todo!("at {}:{}: {:?}", file!(), line!(), x),
        };

        write!(w, "{}", open)?;
        self.stream().dump(w, indentation)?;
        write!(w, "{}", close)
    }
}

impl_dump_for_token![>]; // Gt

impl Dump for Ident {
    fn dump(&self, w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        write!(w, "{self}")
    }
}

impl_dump_for_token![if];
impl_dump_for_enum!(Item, 1:[Enum, Fn, Mod, Static], _);

impl Dump for ItemEnum {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.vis.dump_with_space(w, indentation)?;
        self.enum_token.dump_with_space(w, indentation)?;
        self.ident.dump(w, indentation)?;
        self.generics.dump(w, indentation)?;
        writeln!(w, " {{")?;
        self.variants.dump_lines(w, indentation.next())?;
        write!(w, "{indentation}}}")
    }
}

impl Dump for ItemMod {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.vis.dump_with_space(w, indentation)?;
        self.mod_token.dump_with_space(w, indentation)?;
        self.ident.dump(w, indentation)?;
        if let Some(content) = &self.content {
            writeln!(w, " {{")?;
            content.1.dump_lines(w, indentation.next())?;
            write!(w, "{indentation}}}")?;
        }
        self.semi.dump_ln(w, indentation)
    }
}

impl_dump_for_struct!(ItemFn, [[spaced]: attrs, vis, sig, block]);
impl_dump_for_struct!(
    ItemStatic,
    [
        [spaced]: attrs,
        [with_space]: vis,
        [with_space]: static_token,
        [with_space]: mutability,
        ident,
        [with_space]: colon_token,
        [with_space]: ty,
        [with_space]: eq_token,
        expr,
        semi_token
    ]
);
impl_dump_for_struct!(Label, [name, colon_token]);
impl_dump_for_token![let];

impl Dump for Lifetime {
    fn dump(&self, _w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        todo!("at {}:{}", file!(), line!())
    }
}

impl_dump_for_enum!(Lit, 1: [Int, Str], _);

impl Dump for Literal {
    fn dump(&self, w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        write!(w, "{}", self)
    }
}

impl Dump for LitInt {
    fn dump(&self, w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        write!(w, "{}", self)
    }
}

impl Dump for LitStr {
    fn dump(&self, w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        write!(w, "{}", self.value())
    }
}

impl Dump for Local {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.let_token.dump_with_space(w, indentation)?;
        self.pat.dump(w, indentation)?;
        if let Some(init) = &self.init {
            write!(w, " ")?;
            init.0.dump_with_space(w, indentation)?;
            init.1.dump(w, indentation)?;
        }
        self.semi_token.dump(w, indentation)
    }
}

impl_dump_for_token![loop];
impl_dump_for_token![<]; // Lt
impl_dump_for_token![match];
impl_dump_for_struct!(
    MethodTurbofish,
    [colon2_token, lt_token, [spaced]: args, gt_token]
);
impl_dump_for_token![mod];
impl_dump_for_token![mut];

impl<T: Dump> Dump for Option<T> {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        if let Some(value) = self {
            value.dump(w, indentation)
        } else {
            Ok(())
        }
    }

    fn dump_with_space(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        if let Some(value) = self {
            value.dump_with_space(w, indentation)
        } else {
            Ok(())
        }
    }

    fn dump_ln(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        if let Some(value) = self {
            value.dump_ln(w, indentation)
        } else {
            Ok(())
        }
    }
}

impl_dump_for_enum!(Pat, 1: [Ident, Lit, TupleStruct, Wild], _);
impl_dump_for_struct!(Path, [leading_colon, segments]);
impl_dump_for_enum!(PathArguments, 0: [None], 1: [AngleBracketed], _);
impl_dump_for_struct!(PathSegment, [ident, arguments]);
impl_dump_for_struct!(
    PatIdent,
    [
        [spaced]: attrs,
        [with_space]: by_ref,
        [with_space]: mutability,
        ident,
        subpat
    ]
);
impl_dump_for_struct!(PatLit, [[spaced]: attrs, expr]);

impl Dump for PatTuple {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        write!(w, "(")?;
        self.elems.dump_spaced(w, indentation)?;
        write!(w, ")")
    }
}

impl_dump_for_struct!(PatTupleStruct, [[spaced]: attrs, path, pat]);
impl_dump_for_struct!(
    PatType,
    [[spaced]: attrs, pat, [with_space]: colon_token, ty]
);
impl_dump_for_struct!(PatWild, [[spaced]: attrs, underscore_token]);
impl_dump_for_token![#]; // Pound
impl_dump_for_token![pub];

impl Dump for Punct {
    fn dump(&self, w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        write!(w, "{}", self.as_char())?;
        match self.spacing() {
            Spacing::Alone => write!(w, " ")?,
            Spacing::Joint => (),
        }
        Ok(())
    }
}

impl<T: Dump, P: Dump> Dump for Punctuated<T, P> {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        for pair in self.pairs() {
            match pair {
                Pair::Punctuated(t, p) => {
                    t.dump(w, indentation)?;
                    p.dump(w, indentation)?;
                }
                Pair::End(t) => t.dump(w, indentation)?,
            }
        }
        Ok(())
    }
}

impl Dump for QSelf {
    fn dump(&self, _w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        todo!("at {}:{}", file!(), line!())
    }
}

impl_dump_for_token![->]; // RArrow
impl_dump_for_token![ref];
impl_dump_for_token![return];

impl Dump for ReturnType {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            ReturnType::Type(rarrow, r#type) => {
                write!(w, " ")?;
                rarrow.dump_with_space(w, indentation)?;
                r#type.dump_with_space(w, indentation)
            }
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }
}

impl_dump_for_token![;]; // Semi

impl Dump for Signature {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.constness.dump_with_space(w, indentation)?;
        self.asyncness.dump_with_space(w, indentation)?;
        self.unsafety.dump_with_space(w, indentation)?;
        self.abi.dump_with_space(w, indentation)?;

        self.fn_token.dump_with_space(w, indentation)?;
        self.ident.dump(w, indentation)?;
        self.generics.dump(w, indentation)?;

        write!(w, "(")?;
        self.inputs.dump_spaced(w, indentation)?;
        write!(w, ")")?;

        self.variadic.dump(w, indentation)?;
        self.output.dump(w, indentation)
    }
}

impl_dump_for_token![*]; // Star
impl_dump_for_token![static];
impl_dump_for_enum!(Stmt, 1: [Expr, Item, Local], 2: [Semi]);

impl Dump for TokenStream2 {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        for token in self.clone() {
            token.dump(w, indentation)?;
        }
        Ok(())
    }
}

impl_dump_for_enum!(TokenTree, 1: [Group, Ident, Punct, Literal]);
impl_dump_for_enum!(Type, 1:[Path, Ptr, Reference, Tuple], _);

impl Dump for TypeParam {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.ident.dump(w, indentation)?;
        self.colon_token.dump(w, indentation)?;
        self.bounds.dump_spaced(w, indentation)?;
        if let Some(eq_token) = self.eq_token {
            write!(w, " ")?;
            eq_token.dump(w, indentation)?;
        }
        if let Some(default) = &self.default {
            write!(w, " ")?;
            default.dump(w, indentation)?;
        }
        Ok(())
    }
}

impl Dump for TypeParamBound {
    fn dump(&self, _w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        todo!("at line {}: {:?}", line!(), self)
    }
}

impl_dump_for_struct!(TypePath, [qself, path]);
impl_dump_for_struct!(TypePtr, [star_token, [with_space]: const_token, mutability]);
impl_dump_for_struct!(TypeReference, [and_token, lifetime, mutability, elem]);

impl Dump for TypeTuple {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        write!(w, "(")?;
        self.elems.dump_spaced(w, indentation)?;
        write!(w, ")")
    }
}

impl_dump_for_token![_]; // Underscore
impl_dump_for_token![unsafe];

impl Dump for Variadic {
    fn dump(&self, _w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        todo!("at {}:{}", file!(), line!())
    }
}

impl Dump for Variant {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.ident.dump(w, indentation)?;
        self.fields.dump(w, indentation)?;
        if let Some(discriminant) = &self.discriminant {
            write!(w, " ")?;
            discriminant.0.dump_with_space(w, indentation)?;
            discriminant.1.dump(w, indentation)?;
        }
        Ok(())
    }
}

impl Dump for Visibility {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            Visibility::Inherited => Ok(()),
            Visibility::Public(vis_public) => vis_public.dump(w, indentation),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }

    fn dump_with_space(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            Visibility::Inherited => Ok(()),
            Visibility::Public(vis_public) => vis_public.dump_with_space(w, indentation),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }
}

impl_dump_for_struct!(VisPublic, [pub_token]);

//---------------------------------------------------------------------------------------

trait DumpMulti {
    fn dump_lines(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()>;
    fn dump_spaced(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()>;
}

impl<T: Dump, P: Dump> DumpMulti for Punctuated<T, P> {
    fn dump_lines(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        for pair in self.pairs() {
            write!(w, "{indentation}")?;
            match pair {
                Pair::Punctuated(t, p) => {
                    t.dump(w, indentation)?;
                    p.dump_ln(w, indentation)?;
                }
                Pair::End(t) => t.dump(w, indentation)?,
            }
        }
        Ok(())
    }

    fn dump_spaced(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        for pair in self.pairs() {
            match pair {
                Pair::Punctuated(t, p) => {
                    t.dump(w, indentation)?;
                    p.dump_with_space(w, indentation)?;
                }
                Pair::End(t) => t.dump(w, indentation)?,
            }
        }
        Ok(())
    }
}

impl<T: Dump> DumpMulti for Vec<T> {
    fn dump_lines(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        for x in self {
            write!(w, "{indentation}")?;
            x.dump_ln(w, indentation)?;
        }
        Ok(())
    }

    fn dump_spaced(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        for x in self {
            x.dump_with_space(w, indentation)?;
        }
        Ok(())
    }
}
