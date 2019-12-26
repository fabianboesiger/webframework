extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{Data, Field, Fields, Ident, Lit, Meta, NestedMeta};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Meta::NameValue;

#[proc_macro_derive(Form, attributes(form))]
pub fn store_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_store(&ast)
}

fn extract_name_value(nested: &Punctuated<NestedMeta, Comma>, name: &str) -> Option<Lit> {
    for nested_meta in nested.clone() {
        if let NestedMeta::Meta(meta) = nested_meta {
            if let NameValue(name_value) = meta {
                if name_value.path.is_ident(name) {
                    return Some(name_value.lit)
                }
            } 
        }
    }
    None
}

#[derive(Clone)]
struct InputField {
    pub ident: Ident,
    name: String,
    input_type: InputType
}

impl InputField {
    fn parse(field: Field) -> InputField {
        let nested = if let Meta::List(meta_list) = field.attrs
            .into_iter()
            .filter(|attr| attr.path.is_ident("form"))
            .nth(0)
            .unwrap()
            .parse_meta()
            .unwrap()
        { meta_list.nested } else { panic!() };
        
        InputField {
            ident: field.ident.unwrap(),
            name: if let Some(Lit::Str(lit_str)) = extract_name_value(&nested, "name") {
                lit_str.value()
            } else {
                panic!("name not found")
            },
            input_type: InputType::parse(nested)
        }
    }

    fn html(&self) -> String {
        format!("<label for=\"{ident}\">{name}</label><input id=\"{ident}\" name=\"{ident}\" value=\"{placeholder}\"></input>", ident=self.ident, name=self.name, placeholder="{}")
    }
}

#[derive(Clone)]
enum InputType {
    Text(TextInputType)
}

impl InputType {
    fn parse(nested: Punctuated<NestedMeta, Comma>) -> InputType {
        if let Some(Lit::Str(lit_str)) = extract_name_value(&nested, "type") {
            return match lit_str.value().as_str() {
                "text" => InputType::Text(TextInputType::parse(nested)),
                "password" => InputType::Text(TextInputType::parse(nested)),
                "number" => InputType::Text(TextInputType::parse(nested)),
                &_ => panic!("unknown type")
            }
        }
        panic!()
    }
}

#[derive(Clone)]
struct TextInputType {
    min: u32,
    max: u32
}

impl TextInputType {
    fn parse(nested: Punctuated<NestedMeta, Comma>) -> TextInputType {
        let mut default = TextInputType {
            min: 0,
            max: std::u32::MAX
        };
        if let Some(Lit::Int(lit_int)) = extract_name_value(&nested, "min") {
            default.min = u32::from_str_radix(lit_int.base10_digits(), 10).expect("error while parsing minimum");
        }
        if let Some(Lit::Int(lit_int)) = extract_name_value(&nested, "max") {
            default.max = u32::from_str_radix(lit_int.base10_digits(), 10).expect("error while parsing maximum");
        }
        default
    }
}

fn impl_store(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let fields: Vec<InputField> = match &ast.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    fields.named.clone().into_iter().map(|field| InputField::parse(field)).collect()
                },
                Fields::Unnamed(_) => todo!(),
                Fields::Unit => todo!()
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!()
    };
    
    let field_html = fields.clone().into_iter().map(|field| field.html());
    let field_names = fields.into_iter().map(|field| field.ident);

    // generate implementation
    let gen = quote! {
        impl Form for #struct_name {}

        impl Content for #struct_name {
            fn get(&self) -> Vec<u8> {
                let mut input = String::new();
                input.push_str("<form>");
                #(
                    input.push_str(&format!(#field_html, self.#field_names));
                )*
                input.push_str("</form>");
                input.into_bytes()
            }

            fn post(&self) -> Vec<u8> {
                Vec::new()
            }
        }
    };
    gen.into()
}
