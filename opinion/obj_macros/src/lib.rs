use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Lifetime, parse_macro_input};

/// A procedural macro to implement `From<Self>` for a target enum named `Obj<'a>`.
///
/// Assumes:
/// 1. The target enum is named `Obj`.
/// 2. The variant name is the struct's name with "Obj" stripped from the end.
#[proc_macro_attribute]
pub fn Obj(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. Parse the item the macro is applied to (the struct definition)
    let input = parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &input.ident; // e.g., RectObj or TextObj

    // 2. Define the target enum and variant names based on convention
    let target_enum = Ident::new("Obj", struct_name.span());
    let struct_name_str = struct_name.to_string();

    // Logic: Strip the "Obj" suffix to get the variant name.
    let variant_name_str = if struct_name_str.ends_with("Obj") {
        &struct_name_str[0..(struct_name_str.len() - 3)] // Slice off the last 3 chars ("Obj")
    } else {
        // Fallback: If it doesn't end in "Obj", use the full name or panic/error
        // For simplicity here, we'll use the full name, but in real code, you'd error.
        &struct_name_str
    };

    let variant_name = Ident::new(variant_name_str, struct_name.span()); // e.g., Rect

    // 3. Define the necessary components
    let lifetime_a = Lifetime::new("'a", struct_name.span());
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Determine if the struct itself has a lifetime parameter, like TextObj<'a>
    let has_lifetime_param = input
        .generics
        .params
        .iter()
        .any(|p| matches!(p, syn::GenericParam::Lifetime(_)));

    // 4. Generate the impl block
    let impl_block = if has_lifetime_param {
        // Case: Struct has a lifetime (e.g., TextObj<'a>)
        // Source type is StructName<'a>, Target type is Obj<'a>
        quote! {
            impl #impl_generics From<#struct_name #ty_generics> for #target_enum <#lifetime_a> #where_clause {
                fn from(s: #struct_name #ty_generics) -> Self {
                    #target_enum::#variant_name(s)
                }
            }
        }
    } else {
        // Case: Struct does not have a lifetime (e.g., RectObj)
        // Source type is StructName, Target type is Obj<'a>
        quote! {
            impl <#lifetime_a> From<#struct_name #ty_generics> for #target_enum <#lifetime_a> #where_clause {
                fn from(s: #struct_name #ty_generics) -> Self {
                    #target_enum::#variant_name(s)
                }
            }
        }
    };

    // 5. Combine the original struct definition with the new impl block
    let expanded = quote! {
        #input

        #impl_block
    };

    TokenStream::from(expanded)
}
