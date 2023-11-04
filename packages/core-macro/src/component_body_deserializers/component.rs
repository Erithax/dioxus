use crate::component_body::{ComponentBody, DeserializerArgs};
use crate::component_body_deserializers::inline_props::InlinePropsDeserializerArgs;
use constcat::concat;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::*;

pub(crate) const COMPONENT_ARG_CASE_CHECK_ERROR: &str = concat!(
    "This component does not use PascalCase. \
To ignore this check, pass the \"",
    crate::COMPONENT_ARG_CASE_CHECK_OFF,
    "\" argument, like so: #[component(",
    crate::COMPONENT_ARG_CASE_CHECK_OFF,
    ")]"
);

const INNER_FN_NAME: &str = "__dx_inner_comp";

//static ONCINATOR: Once = Once::new();
// unsafe {
//     ONCINATOR.call_once(|| {
//         println!("foo");
//         VAL = expensive_computations();
//     });
// }

fn get_out_comp_fn(orig_comp_fn: &ItemFn, cx_pat: &Pat) -> ItemFn {
    let inner_comp_ident: Ident = Ident::new(INNER_FN_NAME, orig_comp_fn.sig.ident.span());

    let comp_name = syn::LitStr::new(
        &orig_comp_fn.sig.ident.to_string(),
        orig_comp_fn.sig.ident.span(),
    );

    // let new_stuff: ExprMacro = parse_quote! {
    //     println!("calling component `{}`", #comp_name)
    // };
    let new_stuff: syn::Block = parse_quote! {
        {
            println!("calling component `{}`", #comp_name);
            static COMP_INIT_LOCK: std::sync::Once = std::sync::Once::new();
            COMP_INIT_LOCK.call_once(|| {
                println!("initing component `{}`", #comp_name);
            });
        }
    };

    let inner_comp_name_var = Ident::new(
        dioxus_rsx::INNER_COMP_NAME_VAR,
        proc_macro2::Span::call_site(),
    );

    let mut edited_comp_fn = orig_comp_fn.clone();
    edited_comp_fn.block.stmts.insert(
        0,
        parse_quote! {
            const #inner_comp_name_var: &'static str = #comp_name;
        },
    );
    edited_comp_fn
        .block
        .stmts
        .insert(0, parse_quote! {#new_stuff;});

    let mut visitor = PrependRenderCalls {
        comp_info: dioxus_rsx::CallBodyComponentInfo {
            comp_name: orig_comp_fn.sig.ident.to_string(),
        },
    };
    syn::visit_mut::VisitMut::visit_block_mut(&mut visitor, &mut edited_comp_fn.block);

    // orig_comp_fn.block = parse_quote! {
    //     #new_stuff;
    //     #orig_comp_fn.block
    // };

    let inner_comp_fn = ItemFn {
        sig: Signature {
            ident: inner_comp_ident.clone(),
            ..orig_comp_fn.sig.clone()
        },
        ..edited_comp_fn.clone()
    };

    ItemFn {
        block: parse_quote! {
            {
                #[warn(non_snake_case)]
                #[allow(clippy::inline_always)]
                #[inline(always)]
                #inner_comp_fn
                #inner_comp_ident (#cx_pat)
            }
        },
        ..orig_comp_fn.clone()
    }
}

struct PrependRenderCalls {
    comp_info: dioxus_rsx::CallBodyComponentInfo,
}

impl syn::visit_mut::VisitMut for PrependRenderCalls {
    fn visit_item_mut(&mut self, i: &mut Item) {
        // Do not recurse into items defined inside the function body.
        assert!(true, "nested item {}", quote! {#i}.to_string());
    }

    fn visit_macro_mut(&mut self, mac: &mut Macro) {
        let macro_path = mac
            .path
            .segments
            .iter()
            .fold("".to_string(), |acc, nex| acc + &nex.ident.to_string());

        if macro_path == "render" {
            let t = &mac.tokens;
            let prefix = &self.comp_info;
            mac.tokens = quote! {
                #prefix
                #t
            };
            // TODO: handle recursive render! calls?
        } else {
            // TODO: recurse into other macros?
        }
    }
}

/// The args and deserializing implementation for the [`crate::component`] macro.
#[derive(Clone)]
pub struct ComponentDeserializerArgs {
    pub case_check: bool,
}

/// The output fields and [`ToTokens`] implementation for the [`crate::component`] macro.
#[derive(Clone)]
pub struct ComponentDeserializerOutput {
    pub comp_fn: ItemFn,
    pub props_struct: Option<ItemStruct>,
}

impl ToTokens for ComponentDeserializerOutput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let comp_fn: &ItemFn = &self.comp_fn;
        let props_struct = &self.props_struct;

        tokens.append_all(quote! {
            #props_struct
            #[allow(non_snake_case)]
            #comp_fn
        });
    }
}

impl DeserializerArgs<ComponentDeserializerOutput> for ComponentDeserializerArgs {
    fn to_output(&self, component_body: &ComponentBody) -> Result<ComponentDeserializerOutput> {
        let Signature { ident, .. } = &component_body.item_fn.sig;

        if self.case_check && !is_pascal_case(&ident.to_string()) {
            return Err(Error::new(ident.span(), COMPONENT_ARG_CASE_CHECK_ERROR));
        }

        if component_body.has_extra_args {
            Self::deserialize_with_props(component_body)
        } else {
            Ok(Self::deserialize_no_props(component_body))
        }
    }
}

impl ComponentDeserializerArgs {
    fn deserialize_no_props(component_body: &ComponentBody) -> ComponentDeserializerOutput {
        let ComponentBody {
            item_fn,
            cx_pat_type,
            ..
        } = component_body;
        let cx_pat = &cx_pat_type.pat;

        let comp_fn = get_out_comp_fn(item_fn, cx_pat);

        ComponentDeserializerOutput {
            comp_fn,
            props_struct: None,
        }
    }

    fn deserialize_with_props(
        component_body: &ComponentBody,
    ) -> Result<ComponentDeserializerOutput> {
        let ComponentBody {
            item_fn,
            cx_pat_type,
            ..
        } = component_body;
        let cx_pat = &cx_pat_type.pat;

        let comp_parsed = match parse2::<ComponentBody>(quote!(#item_fn)) {
            Ok(comp_body) => comp_body,
            Err(e) => {
                return Err(Error::new(
                    e.span(),
                    format!(
                        "This is probably a bug in our code, please report it! Error: {}",
                        e
                    ),
                ))
            }
        };

        let inlined_props_output = comp_parsed.deserialize(InlinePropsDeserializerArgs {})?;
        let props_struct = inlined_props_output.props_struct;
        let props_fn = inlined_props_output.comp_fn;

        let comp_fn = get_out_comp_fn(&props_fn, cx_pat);

        Ok(ComponentDeserializerOutput {
            comp_fn,
            props_struct: Some(props_struct),
        })
    }
}

fn is_pascal_case(input: &str) -> bool {
    let mut is_next_lowercase = false;

    for c in input.chars() {
        let is_upper = c.is_ascii_uppercase();

        if (c == '_') || (is_upper && is_next_lowercase) {
            return false;
        }

        is_next_lowercase = is_upper;
    }

    true
}
