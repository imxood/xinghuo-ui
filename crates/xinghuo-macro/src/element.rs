use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, spanned::Spanned};
use syn_rsx::{parse, punctuation::Dash, NodeName, NodeType};

pub fn parse_ui_element(tokens: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    let node = parse(tokens)?.remove(0);
    eprintln!("nodes: {:?}", &node);
    Ok(TokenStream::new())
}

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;

// #[proc_macro_derive(Builder)]
// pub fn ui_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let st = syn::parse_macro_input!(input as syn::DeriveInput);
//     match do_expand(&st) {
//         Ok(out) => out.into(),
//         Err(e) => e.to_compile_error().into(),
//     }
// }

fn do_expand(st: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let builder_name_ident = syn::Ident::new(&builder_name_literal, st.span());
    eprintln!("struct_name_literal: {:?}", &struct_name_literal,);
    eprintln!("builder_name_literal: {:?}", &builder_name_literal,);
    eprintln!("builder_name_ident: {:?}", &builder_name_ident,);

    // 模板代码中不可以使用`.`来访问结构体成员，所以要在模板代码外面将标识符放到一个独立的变量中
    let struct_ident = &st.ident;

    let fields = get_fields_from_derive_input(st)?;

    eprintln!("fields len: {:?}", fields.len());

    let builder_struct_fields_def = generate_builder_struct_fields_def(fields)?;

    let builder_struct_factory_init_clauses = generate_builder_struct_factory_init_clauses(fields)?;

    let setter_functions = generate_setter_functions(fields)?;

    let generated_builder_functions = generate_build_function(fields, struct_ident)?;

    Ok(quote! {
        pub struct #builder_name_ident {
            #builder_struct_fields_def
        }

        impl #struct_ident {
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident {
                    #(#builder_struct_factory_init_clauses),*
                }
            }
        }

        impl #builder_name_ident {
            #setter_functions
            #generated_builder_functions
        }
    })
}

fn get_fields_from_derive_input(d: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = d.data
    {
        return Ok(named);
    }
    Err(syn::Error::new_spanned(
        d,
        "Must define on a Struct, not Enum".to_string(),
    ))
}

fn generate_builder_struct_fields_def(fields: &StructFields) -> syn::Result<TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    // 第六关，对types 变量的构建逻辑进行了调整
    let types: Vec<_> = fields
        .iter()
        .map(|f| {
            // 针对是否为`Option`类型字段，产生不同的结果
            if let Some(inner_ty) = get_optional_inner_type(&f.ty) {
                quote!(std::option::Option<#inner_ty>)
            } else {
                let origin_ty = &f.ty;
                quote!(std::option::Option<#origin_ty>)
            }
        })
        .collect();

    let token_stream = quote! {
        // 下面这一行，也做了修改
        #(#idents: #types),*
    };
    Ok(token_stream)
}

fn generate_builder_struct_factory_init_clauses(
    fields: &StructFields,
) -> syn::Result<Vec<TokenStream>> {
    let init_clauses: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            quote! {
                #ident: std::option::Option::None
            }
        })
        .collect();

    Ok(init_clauses)
}

fn generate_setter_functions(fields: &StructFields) -> syn::Result<TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let mut final_tokenstream = TokenStream::new();

    for (ident, type_) in idents.iter().zip(types.iter()) {
        let tokenstream_piece;
        // 第六关，对tokenstream_piece 变量的构建逻辑进行了调整
        if let Some(inner_ty) = get_optional_inner_type(type_) {
            tokenstream_piece = quote! {
                fn #ident(&mut self, #ident: #inner_ty) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            };
        } else {
            tokenstream_piece = quote! {
                fn #ident(&mut self, #ident: #type_) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            };
        }
        final_tokenstream.extend(tokenstream_piece);
    }

    Ok(final_tokenstream)
}

fn generate_build_function(
    fields: &StructFields,
    origin_struct_ident: &syn::Ident,
) -> syn::Result<TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    // 下面这一行是第六关新加的，之前没用到type相关信息，就没写下面这一行
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let mut checker_code_pieces = Vec::new();
    for idx in 0..idents.len() {
        let ident = idents[idx];
        // 第六关修改，只对不是`Option`类型的字段生成校验逻辑
        if get_optional_inner_type(&types[idx]).is_none() {
            checker_code_pieces.push(quote! {
                if self.#ident.is_none() {
                    let err = format!("{} field missing", stringify!(#ident));
                    return std::result::Result::Err(err.into())
                }
            });
        }
    }

    let mut fill_result_clauses = Vec::new();
    for idx in 0..idents.len() {
        let ident = idents[idx];
        // 这里需要区分`Option`类型字段和非`Option`类型字段
        if get_optional_inner_type(&types[idx]).is_none() {
            fill_result_clauses.push(quote! {
                #ident: self.#ident.clone().unwrap()
            });
        } else {
            fill_result_clauses.push(quote! {
                #ident: self.#ident.clone()
            });
        }
    }

    let token_stream = quote! {
        pub fn build(&mut self) -> std::result::Result<#origin_struct_ident, std::boxed::Box<dyn std::error::Error>> {
            #(#checker_code_pieces)*
            let ret = #origin_struct_ident{
                #(#fill_result_clauses),*
            };
            std::result::Result::Ok(ret)
        }
    };
    Ok(token_stream)
}

fn get_optional_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(syn::TypePath { ref path, .. }) = ty {
        // 这里我们取segments的最后一节来判断是不是`Option<T>`，这样如果用户写的是`std:option:Option<T>`我们也能识别出最后的`Option<T>`
        if let Some(seg) = path.segments.last() {
            if seg.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    ref args,
                    ..
                }) = seg.arguments
                {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}
