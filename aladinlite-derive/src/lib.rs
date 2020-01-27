#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

#[proc_macro_derive(Shaderize, attributes(VertexShader, FragmentShader))]
pub fn derive_shader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_shaderize(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_shaderize(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let attrs = &ast.attrs;
    let vertex_shader_attr = attrs
        .iter()
        .filter(|attr| attr.value.name() == "VertexShader")
        .next()
        .unwrap_or_else(|| panic!(
            "Struct is missing #[VertexShader = ?] attribute"
        ));

    let vertex_shader_value = match &vertex_shader_attr.value {
        syn::MetaItem::NameValue(_, syn::Lit::Str(ref s, _)) => s,
        _ => panic!("Struct VertexShader attribute value must be a string literal")
    };

    let fragment_shader_attr = &ast.attrs
        .iter()
        .filter(|attr| attr.value.name() == "FragmentShader")
        .next()
        .unwrap_or_else(|| panic!(
            "Struct is missing #[FragmentShader = ?] attribute"
        ));

    let fragment_shader_value = match &fragment_shader_attr.value {
        syn::MetaItem::NameValue(_, syn::Lit::Str(s, _)) => s,
        _ => panic!("Struct FragmentShader attribute value must be a string literal")
    };

    quote! {
        impl Shaderize for #name {
            fn name() -> &'static str {
                stringify!(#name)
            }

            fn vertex_shader_content() -> String {
                (#vertex_shader_value).to_string()
            }
            fn fragment_shader_content() -> String {
                (#fragment_shader_value).to_string()
            }
        }
    }
}

