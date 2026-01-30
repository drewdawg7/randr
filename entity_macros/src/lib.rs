use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{
    parse_macro_input, parse::{Parse, ParseStream},
    Ident, Token, Expr, Type, Visibility, braced, parenthesized,
};

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

struct SpriteConfig {
    default_sheet: Expr,
}

impl Parse for SpriteConfig {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let key: Ident = content.parse()?;
        if key != "default_sheet" {
            return Err(syn::Error::new(key.span(), "expected `default_sheet`"));
        }
        content.parse::<Token![:]>()?;
        let default_sheet: Expr = content.parse()?;
        Ok(SpriteConfig { default_sheet })
    }
}

struct VariantSprite {
    name: Expr,
    sheet: Option<Expr>,
}

impl Parse for VariantSprite {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Expr = input.parse()?;
        let sheet = if input.peek(Token![in]) {
            input.parse::<Token![in]>()?;
            Some(input.parse::<Expr>()?)
        } else {
            None
        };
        Ok(VariantSprite { name, sheet })
    }
}

struct VariantDef {
    name: Ident,
    fields: Vec<FieldInit>,
    sprite: Option<VariantSprite>,
}

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

        let mut fields = Vec::new();
        let mut sprite = None;

        while !content.is_empty() {
            if content.peek(Token![@]) {
                content.parse::<Token![@]>()?;
                let attr_name: Ident = content.parse()?;
                if attr_name != "sprite" {
                    return Err(syn::Error::new(attr_name.span(), "expected `sprite`"));
                }
                content.parse::<Token![:]>()?;
                sprite = Some(content.parse::<VariantSprite>()?);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            } else {
                fields.push(content.parse::<FieldInit>()?);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }
        }

        Ok(VariantDef { name, fields, sprite })
    }
}

struct DefineEntityInput {
    spec_name: Ident,
    fields: Vec<FieldDef>,
    id_name: Ident,
    sprite_config: Option<SpriteConfig>,
    variants: Vec<VariantDef>,
}

impl Parse for DefineEntityInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

        input.parse::<Ident>()?; // "id"
        let id_name: Ident = input.parse()?;
        input.parse::<Token![;]>()?;

        let sprite_config = if input.peek(Ident) {
            let keyword: Ident = input.parse()?;
            if keyword == "sprites" {
                let config = input.parse::<SpriteConfig>()?;
                input.parse::<Token![;]>()?;
                Some(config)
            } else if keyword == "variants" {
                let variants_content;
                braced!(variants_content in input);
                let mut variants = Vec::new();
                while !variants_content.is_empty() {
                    variants.push(variants_content.parse::<VariantDef>()?);
                }
                return Ok(DefineEntityInput {
                    spec_name,
                    fields,
                    id_name,
                    sprite_config: None,
                    variants,
                });
            } else {
                return Err(syn::Error::new(keyword.span(), "expected `sprites` or `variants`"));
            }
        } else {
            None
        };

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
            sprite_config,
            variants,
        })
    }
}

#[proc_macro]
pub fn define_entity(input: TokenStream) -> TokenStream {
    let DefineEntityInput {
        spec_name,
        fields,
        id_name,
        sprite_config,
        variants,
    } = parse_macro_input!(input as DefineEntityInput);

    let field_defs = fields.iter().map(|f| {
        let vis = &f.vis;
        let name = &f.name;
        let ty = &f.ty;
        quote! { #vis #name: #ty }
    });

    let variant_names: Vec<_> = variants.iter().map(|v| &v.name).collect();

    let static_specs = variants.iter().map(|v| {
        let static_name = format_ident!("{}_SPEC", to_screaming_snake_case(&v.name.to_string()));
        let field_inits = v.fields.iter().map(|f| {
            let name = &f.name;
            let value = &f.value;
            quote! { #name: #value }
        });
        quote! {
            static #static_name: std::sync::LazyLock<#spec_name> = std::sync::LazyLock::new(|| #spec_name {
                #(#field_inits),*
            });
        }
    });

    let spec_arms = variants.iter().map(|v| {
        let variant = &v.name;
        let static_name = format_ident!("{}_SPEC", to_screaming_snake_case(&v.name.to_string()));
        quote! {
            #id_name::#variant => &*#static_name
        }
    });

    let all_variants = variants.iter().map(|v| {
        let variant = &v.name;
        quote! { #id_name::#variant }
    });

    let sprite_methods = if let Some(config) = &sprite_config {
        let default_sheet = &config.default_sheet;

        let sheet_arms = variants.iter().map(|v| {
            let variant = &v.name;
            if let Some(sprite) = &v.sprite {
                if let Some(sheet) = &sprite.sheet {
                    quote! { #id_name::#variant => #sheet }
                } else {
                    quote! { #id_name::#variant => #default_sheet }
                }
            } else {
                quote! { #id_name::#variant => #default_sheet }
            }
        });

        let name_arms = variants.iter().map(|v| {
            let variant = &v.name;
            if let Some(sprite) = &v.sprite {
                let name = &sprite.name;
                quote! { #id_name::#variant => #name }
            } else {
                let snake = to_snake_case(&v.name.to_string());
                quote! { #id_name::#variant => #snake }
            }
        });

        quote! {
            pub fn sprite_sheet_key(&self) -> crate::assets::SpriteSheetKey {
                match self {
                    #(#sheet_arms),*
                }
            }

            pub fn sprite_name(&self) -> &'static str {
                match self {
                    #(#name_arms),*
                }
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #[derive(Debug, Clone)]
        pub struct #spec_name {
            #(#field_defs),*
        }

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub enum #id_name {
            #(#variant_names),*
        }

        #(#static_specs)*

        impl #id_name {
            pub fn spec(&self) -> &'static #spec_name {
                match self {
                    #(#spec_arms),*
                }
            }

            pub const ALL: &'static [#id_name] = &[
                #(#all_variants),*
            ];

            #sprite_methods
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

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
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

    #[test]
    fn test_snake_case() {
        assert_eq!(to_snake_case("Sword"), "sword");
        assert_eq!(to_snake_case("BronzeSword"), "bronze_sword");
        assert_eq!(to_snake_case("BasicHPPotion"), "basic_h_p_potion");
    }
}
