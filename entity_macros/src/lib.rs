use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{
    parse_macro_input, parse::{Parse, ParseStream},
    Ident, Token, Expr, Type, Visibility, braced,
    punctuated::Punctuated,
};

// Note: This macro generates Lazy statics (not const) because StatSheet uses HashMap.
// When StatSheet becomes const-friendly, we can switch to const statics.

/// Parsed field definition: `pub name: Type`
struct FieldDef {
    vis: Visibility,
    name: Ident,
    ty: Type,
}

impl Parse for FieldDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis: Visibility = input.parse()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(FieldDef { vis, name, ty })
    }
}

/// Parsed variant definition: `VariantName { field: value, ... }`
struct VariantDef {
    name: Ident,
    fields: Punctuated<FieldInit, Token![,]>,
}

/// Field initialization: `field: value`
struct FieldInit {
    name: Ident,
    value: Expr,
}

impl Parse for FieldInit {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let value: Expr = input.parse()?;
        Ok(FieldInit { name, value })
    }
}

impl Parse for VariantDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let fields = content.parse_terminated(FieldInit::parse, Token![,])?;
        Ok(VariantDef { name, fields })
    }
}

/// Main input for define_entity! macro
///
/// ```ignore
/// define_entity! {
///     spec ItemSpec {
///         pub name: &'static str,
///         pub gold_value: i32,
///     }
///
///     id ItemId;
///
///     variants {
///         Sword { name: "Sword", gold_value: 15 }
///         Dagger { name: "Dagger", gold_value: 10 }
///     }
/// }
/// ```
struct DefineEntityInput {
    spec_name: Ident,
    fields: Vec<FieldDef>,
    id_name: Ident,
    variants: Vec<VariantDef>,
}

impl Parse for DefineEntityInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse: spec SpecName { fields... }
        input.parse::<Ident>()?; // "spec"
        let spec_name: Ident = input.parse()?;

        let fields_content;
        braced!(fields_content in input);
        let mut fields = Vec::new();
        while !fields_content.is_empty() {
            fields.push(fields_content.parse::<FieldDef>()?);
            if fields_content.peek(Token![,]) {
                fields_content.parse::<Token![,]>()?;
            }
        }

        // Parse: id IdName;
        input.parse::<Ident>()?; // "id"
        let id_name: Ident = input.parse()?;
        input.parse::<Token![;]>()?;

        // Parse: variants { ... }
        input.parse::<Ident>()?; // "variants"
        let variants_content;
        braced!(variants_content in input);
        let mut variants = Vec::new();
        while !variants_content.is_empty() {
            variants.push(variants_content.parse::<VariantDef>()?);
        }

        Ok(DefineEntityInput {
            spec_name,
            fields,
            id_name,
            variants,
        })
    }
}

/// Defines an entity with its spec struct and ID enum.
///
/// Generates:
/// - The spec struct with all fields
/// - The ID enum with all variants
/// - `ID::spec(&self) -> &'static Spec` method
/// - `ID::ALL` constant array of all variants
/// - Static const specs for each variant
///
/// # Example
///
/// ```ignore
/// define_entity! {
///     spec ItemSpec {
///         pub name: &'static str,
///         pub gold_value: i32,
///     }
///
///     id ItemId;
///
///     variants {
///         Sword { name: "Sword", gold_value: 15 }
///         Dagger { name: "Dagger", gold_value: 10 }
///     }
/// }
/// ```
#[proc_macro]
pub fn define_entity(input: TokenStream) -> TokenStream {
    let DefineEntityInput {
        spec_name,
        fields,
        id_name,
        variants,
    } = parse_macro_input!(input as DefineEntityInput);

    // Generate field definitions for the spec struct
    let field_defs = fields.iter().map(|f| {
        let vis = &f.vis;
        let name = &f.name;
        let ty = &f.ty;
        quote! { #vis #name: #ty }
    });

    // Generate variant names for the enum
    let variant_names: Vec<_> = variants.iter().map(|v| &v.name).collect();

    // Generate static Lazy specs (not const, because StatSheet uses HashMap)
    let static_specs = variants.iter().map(|v| {
        let static_name = format_ident!("{}_SPEC", to_screaming_snake_case(&v.name.to_string()));
        let field_inits = v.fields.iter().map(|f| {
            let name = &f.name;
            let value = &f.value;
            quote! { #name: #value }
        });
        quote! {
            static #static_name: once_cell::sync::Lazy<#spec_name> = once_cell::sync::Lazy::new(|| #spec_name {
                #(#field_inits),*
            });
        }
    });

    // Generate match arms for spec()
    let spec_arms = variants.iter().map(|v| {
        let variant = &v.name;
        let static_name = format_ident!("{}_SPEC", to_screaming_snake_case(&v.name.to_string()));
        quote! {
            #id_name::#variant => &*#static_name
        }
    });

    // Generate ALL slice
    let all_variants = variants.iter().map(|v| {
        let variant = &v.name;
        quote! { #id_name::#variant }
    });

    let expanded = quote! {
        /// Spec struct defining static entity properties
        #[derive(Debug, Clone)]
        pub struct #spec_name {
            #(#field_defs),*
        }

        /// ID enum for this entity type
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub enum #id_name {
            #(#variant_names),*
        }

        // Static const specs
        #(#static_specs)*

        impl #id_name {
            /// Get the static spec for this ID
            pub fn spec(&self) -> &'static #spec_name {
                match self {
                    #(#spec_arms),*
                }
            }

            /// All variant IDs
            pub const ALL: &'static [#id_name] = &[
                #(#all_variants),*
            ];
        }
    };

    TokenStream::from(expanded)
}

/// Similar to define_entity but for static data without spawning
///
/// # Example
///
/// ```ignore
/// define_data! {
///     data RecipeSpec {
///         pub name: &'static str,
///         pub output: ItemId,
///     }
///
///     id RecipeId;
///
///     entries {
///         BronzeIngot { name: "Bronze Ingot", output: ItemId::BronzeIngot }
///     }
/// }
/// ```
#[proc_macro]
pub fn define_data(input: TokenStream) -> TokenStream {
    // For now, define_data is identical to define_entity
    // In the future we may add different functionality
    define_entity(input)
}

/// Convert PascalCase to SCREAMING_SNAKE_CASE
fn to_screaming_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_uppercase());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screaming_snake_case() {
        assert_eq!(to_screaming_snake_case("Sword"), "SWORD");
        assert_eq!(to_screaming_snake_case("BronzeSword"), "BRONZE_SWORD");
        assert_eq!(to_screaming_snake_case("BasicHPPotion"), "BASIC_H_P_POTION");
    }
}
