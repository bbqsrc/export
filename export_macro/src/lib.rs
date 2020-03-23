use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{Error, Item, ItemFn, Visibility};
use quote::quote;

// Todo: implement an internal variant

fn unstable_item_fn(item_fn: ItemFn) -> Result<TokenStream, syn::Error> {
    match item_fn.vis {
        Visibility::Public(_) => return Err(Error::new(item_fn.span(), "function is already public, cannot be unstable")),
        _ => {}
    };

    let mut pub_item_fn = item_fn.clone();
    pub_item_fn.vis = Visibility::Public(syn::VisPublic { pub_token: syn::token::Pub::default() });

    Ok(quote! {
        #[cfg(any(feature = "unstable", doc))]
        #[cfg_attr(doc, deprecated(note = "hello"))]
        /// <script>
        /// if (document.body.classList.contains("mod")) {
        ///   var s = document.currentScript
        ///   var n = s.parentNode.previousElementSibling
        ///   n.classList.remove("deprecated")
        ///   n.classList.add("unstable")
        ///   n.innerText = "Unstable"
        ///   s.parentNode.insertBefore(document.createTextNode("This is some text"), s)
        /// }
        /// </script>
        /// This is some text
        /// <div class="stability export-unstable">
        /// <div class="stab unstable">
        /// <details>
        /// <summary>
        /// <span class="emoji">🔬</span>
        /// This function is <strong>unstable</strong>, and requires the feature <code>unstable</code> to be enabled.
        /// </summary></details></div></div>
        /// <script>
        /// (function(){
        /// if (document.body.classList.contains("fn")) {
        ///   var s = document.currentScript
        ///   var u = s.previousElementSibling
        ///   var x = s.parentNode.previousElementSibling
        ///   console.log(u, x)
        ///   x.parentNode.replaceChild(u, x)
        /// }
        /// })()
        /// </script>
        #pub_item_fn

        #[cfg(all(not(feature = "unstable"), not(doc)))]
        #item_fn
    })
}

pub fn unstable(_attr: TokenStream, item: TokenStream) -> Result<TokenStream, syn::Error> {
    let item: Item = syn::parse2(item)?;

    match item {
        Item::Fn(item_fn) => unstable_item_fn(item_fn),
        x => {
            Err(Error::new(x.span(), "not supported in this position"))
        }
    }
}
