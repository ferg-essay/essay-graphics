
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(PageLabel, attributes(component))]
pub fn derive_page_label(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    TokenStream::from(quote! {
        impl essay_graphics::ui::layout::PageLabel for #name {
            fn box_clone(&self) -> Box<dyn essay_graphics::ui::layout::PageLabel> {
                Box::new(Clone::clone(self))
            }
        }

        impl AsRef<dyn essay_graphics::ui::layout::PageLabel> for #name {
            fn as_ref(&self) -> &dyn essay_graphics::ui::layout::PageLabel {
                self
            }
        }
    })
}
