use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, spanned::Spanned};
use syn_rsx::{parse, punctuation::Dash, NodeName, NodeType};

pub fn parse_ui_view(tokens: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    let node = parse(tokens)?.remove(0);
    eprintln!("nodes: {:?}", &node);
    let item = UiItem::try_from(node)?;
    Ok(quote!({#item .build()}))
}

struct UiTag {
    name: syn::ExprPath,
    attributes: Vec<UiAttr>,
    children: Vec<UiItem>,
}

impl UiTag {
    fn validate_name(name: syn_rsx::NodeName) -> syn::Result<syn::ExprPath> {
        match name {
            NodeName::Path(mut expr_path) => {
                mangle_expr_path(&mut expr_path);
                Ok(expr_path)
            }
            NodeName::Dash(punctuated) => {
                let ident = dashes_to_underscores(punctuated);
                let mut segments = Punctuated::new();
                segments.push(ident.into());
                let path = syn::Path {
                    leading_colon: None,
                    segments,
                };

                Ok(syn::ExprPath {
                    attrs: vec![],
                    qself: None,
                    path,
                })
            }
            NodeName::Colon(punctuated) => Err(syn::Error::new(
                punctuated.span(),
                "Colon tag name syntax isn't supported",
            )),
            NodeName::Block(block) => Err(syn::Error::new(
                block.span(),
                "Block expression as a tag name isn't supported",
            )),
        }
    }
}

impl TryFrom<syn_rsx::Node> for UiTag {
    type Error = syn::Error;

    fn try_from(mut node: syn_rsx::Node) -> syn::Result<Self> {
        match node.node_type {
            NodeType::Element => Ok({
                eprintln!("node.name: {:?}", &node.name);
                Self {
                name: UiTag::validate_name(node.name.unwrap())?,
                attributes: node
                    .attributes
                    .drain(..)
                    .map(UiAttr::try_from)
                    .collect::<syn::Result<Vec<_>>>()?,
                children: node
                    .children
                    .drain(..)
                    .map(UiItem::try_from)
                    .collect::<syn::Result<Vec<_>>>()?,
            }}),
            NodeType::Attribute
            | NodeType::Text
            | NodeType::Block
            | NodeType::Comment
            | NodeType::Doctype
            // TODO(#232) implement
            | NodeType::Fragment => Err(Self::node_convert_error(&node)),
        }
    }
}

impl ToTokens for UiTag {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let UiTag {
            name,
            attributes,
            children,
        } = self;

        let mut contents = quote!( #name() );

        for attr in attributes {
            attr.to_tokens(&mut contents);
        }

        for child in children {
            match child {
                UiItem::None => (),
                nonempty_child => quote!(.child(#nonempty_child)).to_tokens(&mut contents),
            }
        }

        quote!( #contents ).to_tokens(tokens);
    }
}

struct UiAttr {
    name: syn::Ident,
    value: Option<syn::Expr>,
}

impl TryFrom<syn_rsx::Node> for UiAttr {
    type Error = syn::Error;

    fn try_from(node: syn_rsx::Node) -> syn::Result<Self> {
        eprintln!("node attr: {:?}", &node);
        match node.node_type {
            NodeType::Element
            | NodeType::Text
            | NodeType::Block
            | NodeType::Comment
            | NodeType::Doctype
            | NodeType::Fragment => Err(Self::node_convert_error(&node)),
            NodeType::Attribute => Ok(UiAttr {
                name: UiAttr::validate_name(node.name.unwrap())?,
                value: node.value,
            }),
        }
    }
}

impl UiAttr {
    fn validate_name(name: syn_rsx::NodeName) -> syn::Result<syn::Ident> {
        use syn::{punctuated::Pair, PathSegment};

        let invalid_error = |span| syn::Error::new(span, "Invalid name for an attribute");

        match name {
            NodeName::Path(syn::ExprPath {
                attrs,
                qself: None,
                path:
                    syn::Path {
                        leading_colon: None,
                        mut segments,
                    },
            }) if attrs.is_empty() && segments.len() == 1 => {
                let pair = segments.pop();
                match pair {
                    Some(Pair::End(PathSegment {
                        mut ident,
                        arguments,
                    })) if arguments.is_empty() => {
                        mangle_ident(&mut ident);
                        Ok(ident)
                    }
                    // TODO improve error handling, see `https://github.com/stoically/syn-rsx/issues/12`
                    _ => Err(invalid_error(segments.span())),
                }
            }
            NodeName::Dash(punctuated) => Ok(dashes_to_underscores(punctuated)),
            NodeName::Colon(punctuated) => Err(syn::Error::new(
                punctuated.span(),
                "Colon attribute name syntax isn't supported",
            )),
            name => Err(invalid_error(name.span())),
        }
    }
}

impl ToTokens for UiAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, value } = self;
        match value {
            Some(value) => tokens.extend(quote!(.#name(#value))),
            None => tokens.extend(quote!(.#name(#name))),
        };
    }
}

struct UiExpr {
    expr: syn::Expr,
}

impl TryFrom<syn_rsx::Node> for UiExpr {
    type Error = syn::Error;

    fn try_from(node: syn_rsx::Node) -> syn::Result<Self> {
        match node.node_type {
            NodeType::Element
            | NodeType::Attribute
            | NodeType::Comment
            | NodeType::Doctype
            | NodeType::Fragment => Err(Self::node_convert_error(&node)),
            NodeType::Text | NodeType::Block => Ok(UiExpr {
                expr: node.value.unwrap(),
            }),
        }
    }
}

impl ToTokens for UiExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { expr } = self;
        quote!(#[allow(unused_braces)] #expr).to_tokens(tokens);
    }
}

enum UiItem {
    Tag(UiTag),
    Expr(UiExpr),
    None,
}

impl TryFrom<syn_rsx::Node> for UiItem {
    type Error = syn::Error;

    fn try_from(node: syn_rsx::Node) -> syn::Result<Self> {
        match node.node_type {
            NodeType::Element => UiTag::try_from(node).map(UiItem::Tag),
            NodeType::Attribute | NodeType::Fragment => Err(Self::node_convert_error(&node)),
            NodeType::Text | NodeType::Block => UiExpr::try_from(node).map(UiItem::Expr),
            NodeType::Comment | NodeType::Doctype => Ok(UiItem::None),
        }
    }
}

impl ToTokens for UiItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            UiItem::Tag(tag) => tag.to_tokens(tokens),
            UiItem::Expr(expr) => expr.to_tokens(tokens),
            UiItem::None => (),
        }
    }
}

trait NodeConvertError {
    fn node_convert_error(node: &syn_rsx::Node) -> syn::Error {
        syn::Error::new(
            node_span(node),
            format_args!(
                "Cannot convert {} to {}",
                node.node_type,
                std::any::type_name::<Self>(),
            ),
        )
    }
}

impl<T> NodeConvertError for T where T: TryFrom<syn_rsx::Node> {}

fn mangle_expr_path(name: &mut syn::ExprPath) {
    for segment in name.path.segments.iter_mut() {
        mangle_ident(&mut segment.ident);
    }
}

fn mangle_ident(ident: &mut syn::Ident) {
    let name = ident.to_string();
    match name.as_str() {
        "async" | "for" | "loop" | "type" => *ident = syn::Ident::new(&(name + "_"), ident.span()),
        _ => (),
    }
}

fn node_span(node: &syn_rsx::Node) -> Span {
    // TODO get the span for the whole node, see `https://github.com/stoically/syn-rsx/issues/14`
    // Prioritize name's span then value's span then call site's span.
    node.name_span()
        .or_else(|| node.value.as_ref().map(|value| value.span()))
        .unwrap_or_else(Span::call_site)
}

fn dashes_to_underscores(punctuated: Punctuated<Ident, Dash>) -> syn::Ident {
    let mut words = punctuated.iter();
    let mut ident_name = words
        .next()
        .expect("There must be at least one ident in a punctuated list")
        .to_string();

    for w in words {
        ident_name.push('_');
        ident_name.push_str(&w.to_string());
    }

    if punctuated.trailing_punct() {
        ident_name.push('_');
    }

    syn::Ident::new(&ident_name, punctuated.span())
}
