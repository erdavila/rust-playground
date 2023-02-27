use std::fs::File;
use std::io::{self, Write};
use std::ops::Deref;

use syn::punctuated::Punctuated;
use syn::token::{Brace, Bracket, Paren};
use syn::{
    Abi, AngleBracketedGenericArguments, Arm, AttrStyle, Attribute, BinOp, Block, Expr, ExprAssign,
    ExprBinary, ExprBlock, ExprCall, ExprCast, ExprIf, ExprLit, ExprLoop, ExprMatch,
    ExprMethodCall, ExprPath, ExprReference, ExprReturn, Field, Fields, FieldsUnnamed, FnArg,
    GenericArgument, GenericMethodArgument, GenericParam, Generics, Ident, Item, ItemEnum, ItemFn,
    ItemMod, Label, Lifetime, Lit, LitInt, LitStr, Local, MethodTurbofish, Pat, PatIdent, PatLit,
    PatTuple, PatTupleStruct, PatType, PatWild, Path, PathArguments, PathSegment, QSelf,
    ReturnType, Signature, Stmt, Token, Type, TypeParam, TypeParamBound, TypePath, TypePtr,
    TypeReference, TypeTuple, Variadic, Variant, VisPublic, Visibility, WhereClause,
    WherePredicate,
};

use super::indentation::Indentation;

const PRINT_USELESS_TOKENS: bool = false;

pub fn dump_item_fn(item_fn: &ItemFn, name: &str) {
    let mut w = File::create(name).unwrap();

    item_fn
        .to_value()
        .dump(&mut w, Indentation::new(2))
        .unwrap();
    writeln!(w).unwrap();
}

trait Dump {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<DumpResult>;
}

enum Value {
    Singleton(String),
    Named(Named),
    Set(Set),
    List(List),
}
impl Value {
    fn singleton<S: ToString>(s: S) -> Value {
        Value::Singleton(s.to_string())
    }

    fn named<S: ToString>(self, name: S) -> Value {
        Value::Named(Named(name.to_string(), Box::new(self)))
    }

    fn set(members: Vec<Value>) -> Value {
        Value::Set(Set(members))
    }

    fn list(values: Vec<Value>) -> Value {
        Value::List(List(values))
    }

    fn labeled<S: ToString>(self, label: S) -> Value {
        self.named(label.to_string() + ":")
    }

    fn in_set(self) -> Value {
        Self::set(vec![self])
    }

    fn r#struct<S: ToString>(name: S, members: Vec<Value>) -> Value {
        Self::set(members).named(name)
    }
}
impl Dump for Value {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<DumpResult> {
        match self {
            Self::Singleton(singleton) => {
                write!(w, "{singleton}")?;
                Ok(DumpResult {
                    end_delimiter_in_single_line: false,
                })
            }
            Self::Named(named) => named.dump(w, indentation),
            Self::Set(set) => set.dump(w, indentation),
            Self::List(list) => list.dump(w, indentation),
        }
    }
}

struct DumpResult {
    end_delimiter_in_single_line: bool,
}

trait ToValue {
    fn to_value(&self) -> Value;

    fn to_value_labeled<S: ToString>(&self, label: S) -> Value {
        self.to_value().labeled(label)
    }

    fn to_value_in_set_named<S: ToString>(&self, name: S) -> Value {
        self.to_value().in_set().named(name)
    }
}
impl ToValue for usize {
    fn to_value(&self) -> Value {
        Value::singleton(self)
    }
}
impl<T: ToValue> ToValue for Box<T> {
    fn to_value(&self) -> Value {
        self.deref().to_value()
    }
}
impl<T: ToValue> ToValue for Option<T> {
    fn to_value(&self) -> Value {
        match self {
            Some(x) => x.to_value_in_set_named("Some"),
            None => Value::singleton("None"),
        }
    }
}
impl<T: ToValue> ToValue for Vec<T> {
    fn to_value(&self) -> Value {
        Value::list(self.iter().map(|x| x.to_value()).collect())
    }
}
impl<A: ToValue, B: ToValue> ToValue for (A, B) {
    fn to_value(&self) -> Value {
        Value::set(vec![self.0.to_value(), self.1.to_value()])
    }
}

struct Named(String, Box<Value>);
impl Dump for Named {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<DumpResult> {
        write!(w, "{} ", self.0)?;
        self.1.dump(w, indentation)
    }
}

struct Set(Vec<Value>);
impl Dump for Set {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<DumpResult> {
        dump_sequence(&self.0, w, Delimiter::CurlyBraces, indentation)
    }
}

struct List(Vec<Value>);
impl Dump for List {
    fn dump(&self, w: &mut impl Write, indentation: Indentation) -> io::Result<DumpResult> {
        dump_sequence(&self.0, w, Delimiter::SquareBrackets, indentation)
    }
}

fn dump_sequence(
    values: &Vec<Value>,
    w: &mut impl Write,
    delimiter: Delimiter,
    indentation: Indentation,
) -> io::Result<DumpResult> {
    let next_indentation = indentation.next();

    write!(w, "{}", delimiter.open())?;
    let end_delimiter_in_single_line = match values.len() {
        0 => false,
        1 => {
            write!(w, " ")?;
            let print_result = values[0].dump(w, indentation)?;
            if !print_result.end_delimiter_in_single_line {
                write!(w, " ")?;
            }
            print_result.end_delimiter_in_single_line
        }
        _ => {
            writeln!(w)?;
            for value in values {
                write!(w, "{next_indentation}")?;
                value.dump(w, next_indentation)?;
                writeln!(w, ",")?;
            }
            write!(w, "{indentation}")?;
            true
        }
    };
    write!(w, "{}", delimiter.close())?;

    Ok(DumpResult {
        end_delimiter_in_single_line,
    })
}

enum Delimiter {
    CurlyBraces,
    SquareBrackets,
}
impl Delimiter {
    fn open(&self) -> char {
        match self {
            Self::CurlyBraces => '{',
            Self::SquareBrackets => '[',
        }
    }

    fn close(&self) -> char {
        match self {
            Self::CurlyBraces => '}',
            Self::SquareBrackets => ']',
        }
    }
}

//-------------------------------------------------------------------------------------
macro_rules! to_value_struct {
    ($name:ident, [ $($([$attr:ident]:)? $member:ident),* ]) => {
        impl ToValue for $name {
            fn to_value(&self) -> Value {
                let mut members = Vec::new();
                $(
                    if _struct_member_condition!($($attr)?: $member, self ) {
                        members.push(self.$member.to_value_labeled(stringify!($member)));
                    }
                )*

                Value::r#struct(stringify!($name), members)
            }
        }
    };
}

macro_rules! _struct_member_condition {
    (if_any: $member:ident, $self:ident) => {
        !$self.$member.is_empty()
    };
    (if_some: $member:ident, $self:ident) => {
        $self.$member.is_some()
    };
    (useless_token: $member:ident, $self:ident) => {
        PRINT_USELESS_TOKENS
    };
    (: $member:ident, $self:ident) => {
        true
    };
}

macro_rules! to_value_token_singleton {
    [$token:tt] => {
        impl ToValue for Token![$token] {
            fn to_value(&self) -> Value {
                Value::singleton(quoted(stringify!($token)))
            }
        }
    };
}

macro_rules! to_value_string_singleton {
    ($name:ident, $str:literal) => {
        impl ToValue for $name {
            fn to_value(&self) -> Value {
                Value::singleton(quoted($str))
            }
        }
    };
}

macro_rules! to_value_to_string_in_set {
    ($name:ident) => {
        impl ToValue for $name {
            fn to_value(&self) -> Value {
                Value::singleton(quoted(&self.to_string()))
                    .in_set()
                    .named(stringify!($name))
            }
        }
    };
}

macro_rules! to_value_enum {
    (
        $name:ident
        $(, 0: [ $($var0:ident),* ] )?
        $(, 1: [ $($var1:ident),* ] )?
        $(, 2: [ $($var2:ident),* ] )?
        $(, _ $($dummy:item)?)?
    ) => {
        impl ToValue for $name {
            fn to_value(&self) -> Value {
                match self {
                    $(
                        $(
                            $name::$var0 => Value::singleton(_variant_string!($name, $var0)),
                        )*
                    )?
                    $(
                        $(
                            $name::$var1(x) => x.to_value_in_set_named(_variant_string!($name, $var1)),
                        )*
                    )?
                    $(
                        $(
                            $name::$var2(x, y) => Value::set(vec![x.to_value(), y.to_value()]).named(_variant_string!($name, $var2)),
                        )*
                    )?
                    $(
                        _ $($dummy)? => todo!("at line {}: {:?}", line!(), self),
                    )?
                }
            }
        }

    };
}

macro_rules! _variant_string {
    ($enum:ident, $var:ident) => {
        concat!(stringify!($enum), "::", stringify!($var))
    };
}

//-------------------------------------------------------------------------------------
to_value_token_singleton![&]; // And
to_value_token_singleton![as];
to_value_token_singleton![async];
to_value_token_singleton![@]; // At
to_value_string_singleton!(Brace, "{...}");
to_value_string_singleton!(Bracket, "[...]");
to_value_token_singleton![:]; // Colon
to_value_token_singleton![::]; // Colon2
to_value_token_singleton![,]; // Comma
to_value_token_singleton![const];
to_value_token_singleton![.]; // Dot
to_value_token_singleton![...]; // Dot3
to_value_token_singleton![else];
to_value_token_singleton![enum];
to_value_token_singleton![=]; // Eq
to_value_token_singleton![extern];
to_value_token_singleton![=>]; // FatArrow
to_value_token_singleton![fn];
to_value_token_singleton![>]; // Gt
to_value_token_singleton![if];
to_value_token_singleton![let];
to_value_token_singleton![loop];
to_value_token_singleton![<]; // Lt
to_value_token_singleton![match];
to_value_token_singleton![mod];
to_value_token_singleton![mut];
to_value_string_singleton!(Paren, "(...)");
to_value_token_singleton![#]; // Pound
to_value_token_singleton![pub];
to_value_token_singleton![->]; // RArrow
to_value_token_singleton![return];
to_value_token_singleton![ref];
to_value_token_singleton![;]; // Semi
to_value_token_singleton![*]; // Star
to_value_token_singleton![_]; // Underscore
to_value_token_singleton![unsafe];
to_value_token_singleton![where];
//-------------------------------------------------------------------------------------
to_value_struct!(Abi, [[useless_token]: extern_token, [if_some]: name]);
to_value_struct!(
    AngleBracketedGenericArguments,
    [
        [useless_token]: colon2_token,
        [useless_token]: lt_token,
        args,
        [useless_token]: gt_token
    ]
);
to_value_struct!(
    Arm,
    [
        [if_any]: attrs,
        pat,
        [if_some]: guard,
        [useless_token]: fat_arrow_token,
        body,
        [if_some]: comma
    ]
);
to_value_struct!(
    Attribute,
    [
        [useless_token]: pound_token,
        style,
        [useless_token]: bracket_token,
        path
    ]
);
to_value_enum!(AttrStyle, _);

impl ToValue for BinOp {
    fn to_value(&self) -> Value {
        match self {
            BinOp::Lt(_) => Value::singleton("Lt"),
            _ => todo!("at line {}: {:?}", line!(), self),
        }
    }
}

to_value_struct!(Block, [[useless_token]: brace_token, stmts]);
to_value_enum!(
    Expr,
    1: [Assign, Binary, Block, Call, Cast, If, Lit, Loop, Match, MethodCall, Path, Reference, Return],
    _
);
to_value_struct!(
    ExprAssign,
    [[if_any]: attrs, left, [useless_token]: eq_token, right]
);
to_value_struct!(ExprBinary, [[if_any]: attrs, left, op, right]);
to_value_struct!(ExprBlock, [[if_any]: attrs, [if_some]: label, block]);
to_value_struct!(
    ExprCall,
    [[if_any]: attrs, func, [useless_token]: paren_token, args]
);
to_value_struct!(
    ExprCast,
    [[if_any]: attrs, expr, [useless_token]: as_token, ty]
);
to_value_struct!(
    ExprIf,
    [
        [if_any]: attrs,
        [useless_token]: if_token,
        cond,
        then_branch,
        [if_some]: else_branch
    ]
);
to_value_struct!(ExprLit, [[if_any]: attrs, lit]);
to_value_struct!(
    ExprLoop,
    [
        [if_any]: attrs,
        [if_some]: label,
        [useless_token]: loop_token,
        body
    ]
);
to_value_struct!(
    ExprMatch,
    [
        [if_any]: attrs,
        [useless_token]: match_token,
        expr,
        [useless_token]: brace_token,
        [if_any]: arms
    ]
);
to_value_struct!(
    ExprMethodCall,
    [
        [if_any]: attrs,
        receiver,
        [useless_token]: dot_token,
        method,
        [if_some]: turbofish,
        [useless_token]: paren_token,
        args
    ]
);
to_value_struct!(ExprPath, [[if_any]: attrs, [if_some]: qself, path]);
to_value_struct!(
    ExprReference,
    [
        [if_any]: attrs,
        [useless_token]: and_token,
        [if_some]: mutability,
        expr
    ]
);
to_value_struct!(
    ExprReturn,
    [[if_any]: attrs, [useless_token]: return_token, expr]
);
to_value_struct!(
    Field,
    [
        [if_any]: attrs,
        vis,
        [if_some]: ident,
        [if_some]: colon_token,
        ty
    ]
);
to_value_enum!(Fields, 1: [Unnamed], _);
to_value_struct!(FieldsUnnamed, [[useless_token]: paren_token, unnamed]);
to_value_enum!(FnArg, 1: [Typed], _);
to_value_struct!(
    Generics,
    [
        [useless_token]: lt_token,
        params,
        [useless_token]: gt_token,
        [if_some]: where_clause
    ]
);
to_value_enum!(GenericArgument, 1: [Type], _);
to_value_enum!(GenericMethodArgument, _);
to_value_enum!(GenericParam, 1: [Type], _);
to_value_to_string_in_set!(Ident);
to_value_enum!(Item, 1: [Enum, Fn, Mod], _);
to_value_struct!(
    ItemEnum,
    [
        [if_any]: attrs,
        vis,
        [useless_token]: enum_token,
        ident,
        generics,
        [useless_token]: brace_token,
        variants
    ]
);
to_value_struct!(ItemFn, [[if_any]: attrs, vis, sig, block]);
to_value_struct!(
    ItemMod,
    [
        [if_any]: attrs,
        vis,
        [useless_token]: mod_token,
        ident,
        [if_some]: content,
        [if_some]: semi
    ]
);
to_value_struct!(Label, [name, [useless_token]: colon_token]);
to_value_struct!(Lifetime, [/* apostrophe,  */ ident]);
to_value_enum!(Lit, 1: [ Int, Str], _);
to_value_to_string_in_set!(LitInt);

impl ToValue for LitStr {
    fn to_value(&self) -> Value {
        Value::singleton(quoted(&self.value()))
            .in_set()
            .named("LitStr")
    }
}

to_value_struct!(
    Local,
    [
        [if_any]: attrs,
        [useless_token]: let_token,
        pat,
        [if_some]: init,
        [useless_token]: semi_token
    ]
);
to_value_struct!(
    MethodTurbofish,
    [
        [useless_token]: colon2_token,
        [useless_token]: lt_token,
        args,
        [useless_token]: gt_token
    ]
);
to_value_enum!(Pat, 1: [Ident, Lit, TupleStruct , Wild], _);
to_value_struct!(Path, [[if_some]: leading_colon, segments]);
to_value_enum!(PathArguments, 0: [None], 1: [AngleBracketed], _);
to_value_struct!(PathSegment, [ident, arguments]);
to_value_struct!(
    PatIdent,
    [
        [if_any]: attrs,
        [if_some]: by_ref,
        mutability,
        ident,
        [if_some]: subpat
    ]
);
to_value_struct!(PatLit, [[if_any]: attrs, expr]);
to_value_struct!(
    PatTuple,
    [[if_any]: attrs, [useless_token]: paren_token, elems]
);
to_value_struct!(PatTupleStruct, [[if_any]: attrs, path, pat]);
to_value_struct!(
    PatType,
    [[if_any]: attrs, pat, [useless_token]: colon_token, ty]
);
to_value_struct!(
    PatWild,
    [[if_any]: attrs, [useless_token]: underscore_token]
);

impl<T: ToValue, P> ToValue for Punctuated<T, P> {
    fn to_value(&self) -> Value {
        Value::r#struct("Punctuated", self.iter().map(|x| x.to_value()).collect())
    }
}

to_value_struct!(
    QSelf,
    [
        [useless_token]: lt_token,
        ty,
        position,
        [useless_token]: as_token,
        [useless_token]: gt_token
    ]
);
to_value_enum!(ReturnType, 2: [Type], _);
to_value_struct!(
    Signature,
    [
        [if_some]: constness,
        [if_some]: asyncness,
        [if_some]: unsafety,
        [if_some]: abi,
        [useless_token]: fn_token,
        ident,
        generics,
        [useless_token]: paren_token,
        inputs,
        [if_some]: variadic,
        output
    ]
);
to_value_enum!(Stmt, 1: [Expr , Item, Local], 2: [Semi]);
to_value_enum!(Type, 1: [Ptr, Path, Reference, Tuple], _);
to_value_struct!(
    TypeParam,
    [
        [if_any]: attrs,
        ident,
        [useless_token]: colon_token,
        bounds,
        [if_some]: eq_token,
        [if_some]: default
    ]
);
to_value_enum!(TypeParamBound, _);
to_value_struct!(TypePath, [[if_some]: qself, path]);
to_value_struct!(
    TypePtr,
    [
        [useless_token]: star_token,
        [if_some]: const_token,
        [if_some]: mutability,
        elem
    ]
);
to_value_struct!(
    TypeReference,
    [
        [useless_token]: and_token,
        [if_some]: lifetime,
        [if_some]: mutability,
        elem
    ]
);
to_value_struct!(TypeTuple, [[useless_token]: paren_token, elems]);
to_value_struct!(Variadic, [[if_any]: attrs, [useless_token]: dots]);
to_value_struct!(
    Variant,
    [[if_any]: attrs, ident, fields, [if_some]: discriminant]
);
to_value_enum!(Visibility, 0: [Inherited], 1: [Public], _);
to_value_struct!(VisPublic, [[useless_token]: pub_token]);
to_value_struct!(WhereClause, [[useless_token]: where_token, predicates]);
to_value_enum!(WherePredicate, _);
//-------------------------------------------------------------------------------------

fn quoted(arg: &str) -> String {
    format!("\"{}\"", arg)
}
