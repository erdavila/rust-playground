use std::fs::File;
use std::io::{self, Write};
use std::ops::Deref;

use syn::punctuated::{Pair, Punctuated};
use syn::{
    Abi, AngleBracketedGenericArguments, Arm, Attribute, BinOp, Block, Expr, ExprAssign,
    ExprAssignOp, ExprBinary, ExprBlock, ExprCall, ExprCast, ExprIf, ExprLet, ExprLit, ExprLoop,
    ExprMatch, ExprMethodCall, ExprPath, ExprReference, ExprReturn, ExprUnsafe, Field, Fields,
    FieldsUnnamed, FnArg, GenericArgument, GenericMethodArgument, GenericParam, Generics, Ident,
    Item, ItemEnum, ItemFn, ItemMod, ItemStatic, Label, Lifetime, Lit, LitInt, LitStr, Local,
    MethodTurbofish, Pat, PatIdent, PatLit, PatTuple, PatTupleStruct, PatType, PatWild, Path,
    PathArguments, PathSegment, QSelf, ReturnType, Signature, Stmt, Token, Type, TypeParam,
    TypeParamBound, TypePath, TypePtr, TypeReference, TypeTuple, Variadic, Variant, VisPublic,
    Visibility,
};

use super::indentation::Indentation;

pub fn dump_item_fn(item_fn: &ItemFn, name: &str) {
    let mut w = File::create(name).unwrap();

    item_fn.dump(&mut w, Indentation::new(4)).unwrap();
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
        todo!("at line {}", line!());
    }
}

impl_dump_for_token![+]; // Add
impl_dump_for_token![+=]; // AddEq
impl_dump_for_token![&]; // And

impl Dump for AngleBracketedGenericArguments {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.colon2_token.dump(w, indentation)?;
        self.lt_token.dump(w, indentation)?;
        self.args.dump_spaced(w, indentation)?;
        self.gt_token.dump(w, indentation)
    }
}

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
    fn dump(&self, _w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        todo!("at line {}", line!());
    }
}

impl Dump for BinOp {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            BinOp::AddEq(add_eq) => add_eq.dump(w, indentation),
            BinOp::Lt(lt) => lt.dump(w, indentation),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }
}

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

impl Dump for Expr {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            Expr::Assign(expr_assign) => expr_assign.dump(w, indentation),
            Expr::AssignOp(expr_assign_op) => expr_assign_op.dump(w, indentation),
            Expr::Binary(expr_binary) => expr_binary.dump(w, indentation),
            Expr::Block(expr_block) => expr_block.dump(w, indentation),
            Expr::Call(expr_call) => expr_call.dump(w, indentation),
            Expr::Cast(expr_cast) => expr_cast.dump(w, indentation),
            Expr::If(expr_if) => expr_if.dump(w, indentation),
            Expr::Let(expr_let) => expr_let.dump(w, indentation),
            Expr::Lit(expr_lit) => expr_lit.dump(w, indentation),
            Expr::Loop(expr_loop) => expr_loop.dump(w, indentation),
            Expr::Match(expr_match) => expr_match.dump(w, indentation),
            Expr::MethodCall(expr_method_call) => expr_method_call.dump(w, indentation),
            Expr::Path(expr_path) => expr_path.dump(w, indentation),
            Expr::Reference(expr_reference) => expr_reference.dump(w, indentation),
            Expr::Return(expr_return) => expr_return.dump(w, indentation),
            Expr::Unsafe(expr_unsafe) => expr_unsafe.dump(w, indentation),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }
}

impl Dump for ExprAssign {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.left.dump_with_space(w, indentation)?;
        self.eq_token.dump_with_space(w, indentation)?;
        self.right.dump(w, indentation)
    }
}

impl Dump for ExprAssignOp {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.left.dump_with_space(w, indentation)?;
        self.op.dump_with_space(w, indentation)?;
        self.right.dump(w, indentation)
    }
}

impl Dump for ExprBinary {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.left.dump_with_space(w, indentation)?;
        self.op.dump_with_space(w, indentation)?;
        self.right.dump(w, indentation)
    }
}

impl Dump for ExprBlock {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.label.dump_with_space(w, indentation)?;
        self.block.dump(w, indentation)
    }
}

impl Dump for ExprCall {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.func.dump(w, indentation)?;
        write!(w, "(")?;
        self.args.dump_spaced(w, indentation)?;
        write!(w, ")")
    }
}

impl Dump for ExprCast {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.expr.dump_with_space(w, indentation)?;
        self.as_token.dump_with_space(w, indentation)?;
        self.ty.dump(w, indentation)
    }
}

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

impl Dump for ExprLet {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.let_token.dump_with_space(w, indentation)?;
        self.pat.dump_with_space(w, indentation)?;
        self.eq_token.dump_with_space(w, indentation)?;
        self.expr.dump(w, indentation)
    }
}

impl Dump for ExprLit {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.lit.dump(w, indentation)
    }
}

impl Dump for ExprLoop {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.label.dump_with_space(w, indentation)?;
        self.loop_token.dump_with_space(w, indentation)?;
        self.body.dump(w, indentation)
    }
}

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

impl Dump for ExprPath {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.qself.dump(w, indentation)?;
        self.path.dump(w, indentation)
    }
}

impl Dump for ExprReference {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.and_token.dump(w, indentation)?;
        self.mutability.dump_with_space(w, indentation)?;
        self.expr.dump(w, indentation)
    }
}

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

impl Dump for ExprUnsafe {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.unsafe_token.dump_with_space(w, indentation)?;
        self.block.dump(w, indentation)
    }
}

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

impl Dump for Fields {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            Fields::Unnamed(fields_unnamed) => fields_unnamed.dump(w, indentation),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }
}

impl Dump for FieldsUnnamed {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        write!(w, "(")?;
        self.unnamed.dump_spaced(w, indentation)?;
        write!(w, ")")
    }
}

impl_dump_for_token![fn];

impl Dump for FnArg {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            FnArg::Receiver(_) => todo!("at line {}", line!()),
            FnArg::Typed(pat_type) => pat_type.dump(w, indentation),
        }
    }
}

impl Dump for GenericArgument {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            GenericArgument::Type(r#type) => r#type.dump(w, indentation),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }
}

impl Dump for GenericMethodArgument {
    fn dump(&self, _w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        todo!("at line {}: {:?}", line!(), self)
    }
}

impl Dump for GenericParam {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            GenericParam::Type(type_param) => type_param.dump(w, indentation),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }
}

impl Dump for Generics {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.lt_token.dump(w, indentation)?;
        self.params.dump_spaced(w, indentation)?;
        self.gt_token.dump(w, indentation)?;

        if self.where_clause.is_some() {
            todo!("at line {}", line!());
        }

        Ok(())
    }
}

impl_dump_for_token![>]; // Gt

impl Dump for Ident {
    fn dump(&self, w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        write!(w, "{self}")
    }
}

impl_dump_for_token![if];

impl Dump for Item {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            Item::Enum(item_enum) => item_enum.dump(w, indentation),
            Item::Fn(item_fn) => item_fn.dump(w, indentation),
            Item::Mod(item_mod) => item_mod.dump(w, indentation),
            Item::Static(item_static) => item_static.dump(w, indentation),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }
}

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

impl Dump for ItemFn {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.vis.dump(w, indentation)?;
        self.sig.dump(w, indentation)?;
        self.block.dump(w, indentation)
    }
}

impl Dump for ItemStatic {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.vis.dump_with_space(w, indentation)?;
        self.static_token.dump_with_space(w, indentation)?;
        self.mutability.dump_with_space(w, indentation)?;
        self.ident.dump(w, indentation)?;
        self.colon_token.dump_with_space(w, indentation)?;
        self.ty.dump_with_space(w, indentation)?;
        self.eq_token.dump_with_space(w, indentation)?;
        self.expr.dump(w, indentation)?;
        self.semi_token.dump(w, indentation)
    }
}

impl Dump for Label {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.name.dump(w, indentation)?;
        self.colon_token.dump(w, indentation)
    }
}

impl_dump_for_token![let];

impl Dump for Lifetime {
    fn dump(&self, _w: &mut impl Write, _indentation: Indentation) -> io::Result<()> {
        todo!("at line {}", line!())
    }
}

impl Dump for Lit {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            Lit::Int(lit_int) => lit_int.dump(w, indentation),
            Lit::Str(lit_str) => lit_str.dump(w, indentation),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
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

impl Dump for MethodTurbofish {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.colon2_token.dump(w, indentation)?;
        self.lt_token.dump(w, indentation)?;
        self.args.dump_spaced(w, indentation)?;
        self.gt_token.dump(w, indentation)
    }
}

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

impl Dump for Pat {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            Pat::Ident(pat_ident) => pat_ident.dump(w, indentation),
            Pat::Lit(pat_lit) => pat_lit.dump(w, indentation),
            Pat::TupleStruct(pat_tuple_struct) => pat_tuple_struct.dump(w, indentation),
            Pat::Wild(pat_wild) => pat_wild.dump(w, indentation),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }
}

impl Dump for Path {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.leading_colon.dump(w, indentation)?;
        self.segments.dump(w, indentation)
    }
}

impl Dump for PathArguments {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            PathArguments::None => Ok(()),
            PathArguments::AngleBracketed(args) => args.dump(w, indentation),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }
}

impl Dump for PathSegment {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.ident.dump(w, indentation)?;
        self.arguments.dump(w, indentation)
    }
}

impl Dump for PatIdent {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.by_ref.dump_with_space(w, indentation)?;
        self.mutability.dump_with_space(w, indentation)?;
        self.ident.dump(w, indentation)?;
        self.subpat.dump(w, indentation)
    }
}

impl Dump for PatLit {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.expr.dump(w, indentation)
    }
}

impl Dump for PatTuple {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        write!(w, "(")?;
        self.elems.dump_spaced(w, indentation)?;
        write!(w, ")")
    }
}

impl Dump for PatTupleStruct {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.path.dump(w, indentation)?;
        self.pat.dump(w, indentation)
    }
}

impl Dump for PatType {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.pat.dump(w, indentation)?;
        self.colon_token.dump_with_space(w, indentation)?;
        self.ty.dump(w, indentation)
    }
}

impl Dump for PatWild {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.attrs.dump_spaced(w, indentation)?;
        self.underscore_token.dump(w, indentation)
    }
}

impl_dump_for_token![pub];

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
        todo!("at line {}", line!())
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

impl Dump for Stmt {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            Stmt::Expr(expr) => expr.dump(w, indentation),
            Stmt::Item(item) => item.dump(w, indentation),
            Stmt::Local(local) => local.dump(w, indentation),
            Stmt::Semi(expr, semi) => {
                expr.dump(w, indentation)?;
                semi.dump(w, indentation)
            }
        }
    }
}

impl Dump for Type {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        match self {
            Type::Path(type_path) => type_path.dump(w, indentation),
            Type::Ptr(type_ptr) => type_ptr.dump(w, indentation),
            Type::Reference(type_reference) => type_reference.dump(w, indentation),
            Type::Tuple(type_tuple) => type_tuple.dump(w, indentation),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }
}

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

impl Dump for TypePath {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.qself.dump(w, indentation)?;
        self.path.dump(w, indentation)
    }
}

impl Dump for TypePtr {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.star_token.dump(w, indentation)?;
        self.const_token.dump_with_space(w, indentation)?;
        self.mutability.dump(w, indentation)?;
        self.elem.dump(w, indentation)
    }
}

impl Dump for TypeReference {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.and_token.dump(w, indentation)?;
        self.lifetime.dump(w, indentation)?;
        self.mutability.dump(w, indentation)?;
        self.elem.dump(w, indentation)
    }
}

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
        todo!("at line {}", line!())
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

impl Dump for VisPublic {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<()> {
        self.pub_token.dump(w, indentation)
    }
}

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
