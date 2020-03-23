use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{AttrStyle, Error, Item, ItemFn, ItemMod, ItemStruct, Visibility, Meta, Lit};
use quote::quote;

#[ctor::ctor]
fn init() {
    env_logger::init();
}

const SCRIPT_MOD_FN: &str = r#"<script>
if (document.body.classList.contains('mod')) {
    var s = document.currentScript;
    if (s.parentNode.parentNode.parentNode.id != 'main') {
        var n = s.parentNode.parentNode.children[0];
        n.classList.remove('deprecated');
        n.classList.add('unstable');
        n.innerText = 'Unstable';
    }
}
</script>
"#;

const SCRIPT_STRUCT: &str = r#"<div class="stability export-unstable">
<div class="stab unstable">
<span class="emoji">💣</span>
This struct is <strong>unstable</strong>, and requires the feature <code>unstable</code> to be enabled.
</div>
</div>
<script>
(function(){
var l = document.body.classList
if (l.contains('mod') || l.contains('struct')){
  var s = document.currentScript;
  var u = s.previousElementSibling;
  var x = s.parentNode.previousElementSibling;
  x.parentNode.replaceChild(u, x);
}
})()
</script>"#;

const SCRIPT_MOD: &str = r#"<div class="stability export-unstable">
<div class="stab unstable">
<span class="emoji">💣</span>
This module is <strong>unstable</strong>, and requires the feature <code>unstable</code> to be enabled.
</div>
</div>
<script>
(function(){
var l = document.body.classList
if (l.contains('mod') || l.contains('struct')){
  var s = document.currentScript;
  var u = s.previousElementSibling;
  var x = s.parentNode.previousElementSibling;
  x.parentNode.replaceChild(u, x);
}
})()
</script>"#;

const SCRIPT_FN: &str = r#"<div class="stability export-unstable">
<div class="stab unstable">
<span class="emoji">💣</span>
This function is <strong>unstable</strong>, and requires the feature <code>unstable</code> to be enabled.
</div>
</div>
<script>
(function(){
var l = document.body.classList
if (l.contains('fn') || l.contains('struct')){
  var s = document.currentScript;
  var u = s.previousElementSibling;
  var x = s.parentNode.previousElementSibling;
  x.parentNode.replaceChild(u, x);
}
})()
</script>"#;

fn unstable_item_struct(item_struct: ItemStruct) -> Result<TokenStream, syn::Error> {
    match item_struct.vis {
        Visibility::Public(_) => return Err(Error::new(item_struct.span(), "function is already public, cannot be unstable")),
        _ => {}
    };

    let mut pub_item_struct = item_struct.clone();
    pub_item_struct.vis = Visibility::Public(syn::VisPublic { pub_token: syn::token::Pub::default() });

    let mut doc_input: Vec<String> = vec![];

    pub_item_struct.attrs = pub_item_struct.attrs.into_iter().filter_map(|x| {
        log::info!("{:?}", &x);
        if x.style != AttrStyle::Outer {
            return Some(x);
        }

        match x.parse_meta() {
            Ok(meta) => match meta {
                Meta::NameValue(m) if (m.path.get_ident()?.to_string() == "doc") => {
                    match m.lit {
                        Lit::Str(lit_str) => {
                            doc_input.push(lit_str.value().trim().to_string());
                            None
                        }
                        _ => Some(x)
                    }
                }
                _ => Some(x)
            }
            _ => Some(x)
        }
    }).collect();

    if doc_input.len() >= 1 {
        doc_input[0] = format!("{} {}", doc_input[0], SCRIPT_MOD_FN.replace("\n", " "));
    } else {
        doc_input.push(format!("<p>{}</p>", SCRIPT_MOD_FN.replace("\n", " ")));
    }
    let doc_input = doc_input.join("\n");
    log::info!("{}", &doc_input);

    let doc_input = format!("{}\n\n{}", doc_input, SCRIPT_STRUCT);

    Ok(quote! {
        #[cfg(any(feature = "unstable", doc))]
        #[cfg_attr(doc, deprecated(note = "hello"))]
        #[doc = #doc_input]
        #pub_item_struct

        #[cfg(all(not(feature = "unstable"), not(doc)))]
        #item_struct
    })
}

fn unstable_item_mod(item_mod: ItemMod) -> Result<TokenStream, syn::Error> {
    match item_mod.vis {
        Visibility::Public(_) => return Err(Error::new(item_mod.span(), "function is already public, cannot be unstable")),
        _ => {}
    };

    let mut pub_item_mod = item_mod.clone();
    pub_item_mod.vis = Visibility::Public(syn::VisPublic { pub_token: syn::token::Pub::default() });

    let mut doc_input: Vec<String> = vec![];

    pub_item_mod.attrs = pub_item_mod.attrs.into_iter().filter_map(|x| {
        log::info!("{:?}", &x);
        if x.style != AttrStyle::Outer {
            return Some(x);
        }

        match x.parse_meta() {
            Ok(meta) => match meta {
                Meta::NameValue(m) if (m.path.get_ident()?.to_string() == "doc") => {
                    match m.lit {
                        Lit::Str(lit_str) => {
                            doc_input.push(lit_str.value().trim().to_string());
                            None
                        }
                        _ => Some(x)
                    }
                }
                _ => Some(x)
            }
            _ => Some(x)
        }
    }).collect();

    if doc_input.len() >= 1 {
        doc_input[0] = format!("{} {}", doc_input[0], SCRIPT_MOD_FN.replace("\n", " "));
    } else {
        doc_input.push(format!("<p>{}</p>", SCRIPT_MOD_FN.replace("\n", " ")));
    }
    let doc_input = doc_input.join("\n");
    log::info!("{}", &doc_input);

    let doc_input = format!("{}\n\n{}", doc_input, SCRIPT_MOD);

    Ok(quote! {
        #[cfg(any(feature = "unstable", doc))]
        #[cfg_attr(doc, deprecated(note = "hello"))]
        #[doc = #doc_input]
        #pub_item_mod

        #[cfg(all(not(feature = "unstable"), not(doc)))]
        #item_mod
    })
}

fn unstable_item_fn(item_fn: ItemFn) -> Result<TokenStream, syn::Error> {
    match item_fn.vis {
        Visibility::Public(_) => return Err(Error::new(item_fn.span(), "function is already public, cannot be unstable")),
        _ => {}
    };

    let mut pub_item_fn = item_fn.clone();
    pub_item_fn.vis = Visibility::Public(syn::VisPublic { pub_token: syn::token::Pub::default() });

    let mut doc_input: Vec<String> = vec![];

    pub_item_fn.attrs = pub_item_fn.attrs.into_iter().filter_map(|x| {
        log::info!("{:?}", &x);
        if x.style != AttrStyle::Outer {
            return Some(x);
        }

        match x.parse_meta() {
            Ok(meta) => match meta {
                Meta::NameValue(m) if (m.path.get_ident()?.to_string() == "doc") => {
                    match m.lit {
                        Lit::Str(lit_str) => {
                            doc_input.push(lit_str.value().trim().to_string());
                            None
                        }
                        _ => Some(x)
                    }
                }
                _ => Some(x)
            }
            _ => Some(x)
        }
    }).collect();

    if doc_input.len() >= 1 {
        doc_input[0] = format!("{} {}", doc_input[0], SCRIPT_MOD_FN.replace("\n", " "));
    } else {
        doc_input.push(format!("<p>{}</p>", SCRIPT_MOD_FN.replace("\n", " ")));
    }

    let doc_input = doc_input.join("\n");
    log::info!("{}", &doc_input);

    let doc_input = format!("{}\n\n{}", doc_input, SCRIPT_FN);

    Ok(quote! {
        #[cfg(any(feature = "unstable", doc))]
        #[cfg_attr(doc, deprecated(note = "hello"))]
        #[doc = #doc_input]
        #pub_item_fn

        #[cfg(all(not(feature = "unstable"), not(doc)))]
        #item_fn
    })
}

pub fn unstable(_attr: TokenStream, item: TokenStream) -> Result<TokenStream, syn::Error> {
    let item: Item = syn::parse2(item)?;

    match item {
        Item::Fn(item_fn) => unstable_item_fn(item_fn),
        Item::Mod(item_mod) => unstable_item_mod(item_mod),
        Item::Struct(item_struct) => unstable_item_struct(item_struct),
        x => {
            Err(Error::new(x.span(), "not supported in this position"))
        }
    }
}
