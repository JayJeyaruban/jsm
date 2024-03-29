use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, Field, Fields, FieldsNamed, FieldsUnnamed, ItemStruct, Token, Visibility,
};

#[proc_macro_attribute]
pub fn public(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemStruct);
    let span = item.ident.span();
    item.vis = Visibility::Public(Token![pub](span));
    item.fields = match item.fields {
        Fields::Unit => Fields::Unit,
        Fields::Named(fields_named) => {
            let named = fields_named
                .named
                .into_iter()
                .map(|field| public_field(field, span))
                .collect();

            Fields::Named(FieldsNamed {
                named,
                ..fields_named
            })
        }
        Fields::Unnamed(fields_unnamed) => {
            let unnamed = fields_unnamed
                .unnamed
                .into_iter()
                .map(|field| public_field(field, span))
                .collect();

            Fields::Unnamed(FieldsUnnamed {
                unnamed,
                ..fields_unnamed
            })
        }
    };
    TokenStream::from(quote!(#item))
}

fn public_field(field: Field, fallback_span: Span) -> Field {
    Field {
        vis: Visibility::Public(Token![pub](
            field
                .ident
                .as_ref()
                .map(|iden| iden.span())
                .unwrap_or(fallback_span),
        )),
        ..field
    }
}
