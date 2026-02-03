use std::fmt::{Result as FmtResult, Write};

use crate::{
    CairoWriteSlice, Expr, ExprPath, from_typed_syntax_node, syntax_enum, syntax_option,
    syntax_type, typed_syntax_node_to_string_without_trivia, vec_from_element_list,
};

syntax_enum! {
    GenericParam {
        Type(String),
        Const(ConstGenericParam),
        ImplNamed(ImplNamedGenericParam),
        ImplAnonymous(ImplAnonymousGenericParam),
        NegativeImpl(ExprPath),
    }
}

syntax_type! {
    ConstGenericParam[GenericParamConst]{
        name: String,
        ty: Expr,
    }
}

syntax_type! {
    ImplNamedGenericParam[GenericParamImplNamed]{
        name: String,
        trait_path: ExprPath,
        type_constrains: Option<Vec<AssociatedItemConstraint>>,
    }
}

syntax_type! {
    AssociatedItemConstraint{
        item: String,
        value: Expr,
    }
}

syntax_type! {
   ImplAnonymousGenericParam[GenericParamImplAnonymous]{
        trait_path: ExprPath,
        type_constrains: Option<Vec<AssociatedItemConstraint>>,
    }
}

from_typed_syntax_node!(GenericParamNegativeImpl.trait_path, ExprPath);

typed_syntax_node_to_string_without_trivia!(GenericParamType.name);

syntax_option! {OptionAssociatedItemConstraints{AssociatedItemConstraints: Vec<AssociatedItemConstraint>}}
syntax_option! {OptionWrappedGenericParamList{WrappedGenericParamList: Vec<GenericParam>}}
vec_from_element_list! {AssociatedItemConstraints.associated_item_constraints,
AssociatedItemConstraint}

vec_from_element_list! {GenericParamList, GenericParam}
vec_from_element_list! {WrappedGenericParamList.generic_params, GenericParam}

pub trait GenericParamsTrait {
    fn generic_params(&self) -> &Option<Vec<GenericParam>>;
    fn generic_types(&self) -> Option<Vec<&str>> {
        if let Some(params) = self.generic_params() {
            params
                .iter()
                .filter_map(|p| match p {
                    GenericParam::Type(name) => Some(name.as_str()),
                    _ => None,
                })
                .collect::<Vec<&str>>()
                .into()
        } else {
            None
        }
    }
    fn generic_types_string(&self) -> String {
        let mut buf = String::new();
        self.cwrite_generic_types(&mut buf).unwrap();
        buf
    }
    fn cwrite_generic_types<W: Write>(&self, buf: &mut W) -> FmtResult {
        if let Some(generics) = self.generic_types() {
            generics.cwrite_csv_angled(buf)?;
        };
        Ok(())
    }
    fn cwrite_generic_types_call<W: Write>(&self, buf: &mut W) -> FmtResult {
        if let Some(generics) = self.generic_types() {
            buf.write_str("::")?;
            generics.cwrite_csv_angled(buf)?;
        };
        Ok(())
    }
}

impl GenericParamsTrait for Option<Vec<GenericParam>> {
    fn generic_params(&self) -> &Option<Vec<GenericParam>> {
        self
    }
}
