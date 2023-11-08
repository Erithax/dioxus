use super::*;

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    Result,
};

pub fn apply_scope_data_attribute_throughout_node(node: &mut BodyNode, s: &str) {
    match node {
        BodyNode::Element(e) => {
            e.attributes.push(ElementAttrNamed {
                el_name: ElementName::Custom(syn::LitStr::new(s, proc_macro2::Span::call_site())),
                attr: ElementAttr::CustomAttrText {
                    name: syn::LitStr::new(
                        &("data-dx-coscos-".to_owned() + s),
                        proc_macro2::Span::call_site(),
                    ),
                    value: IfmtInput {
                        source: None,
                        segments: Vec::new(),
                    },
                },
            });
            apply_scope_data_attribute_throughout(&mut e.children, s);
        }
        BodyNode::Component(_) | BodyNode::Coscos(_) => {}
        BodyNode::IfChain(_) => {
            todo!("propagate coscos attribute to render! calls in if")
        }
        BodyNode::ForLoop(f) => {
            apply_scope_data_attribute_throughout(&mut f.body, s);
        }
        BodyNode::RawExpr(_) => {
            // todo!("coscos can't handle RawExpr right now")
        }
        BodyNode::Text(_) => {}
    }
}

pub fn apply_scope_data_attribute_throughout(roots: &mut Vec<BodyNode>, s: &str) {
    for mut root in roots.iter_mut() {
        apply_scope_data_attribute_throughout_node(&mut root, s);
    }
}

pub fn apply_scope_data_attribute_throughout_slice(roots: &mut [BodyNode], s: &str) {
    for mut root in roots.iter_mut() {
        apply_scope_data_attribute_throughout_node(&mut root, s);
    }
}

/// traverses bodynodes recursively and returns if coscos is used anywhere
pub fn uses_coscos(body_nodes: &[BodyNode]) -> bool {
    fn uses_coscos_down_from(bn: &BodyNode) -> bool {
        match bn {
            BodyNode::Coscos(_) => return true,
            BodyNode::Element(e) => return e.children.iter().any(|bn| uses_coscos_down_from(bn)),
            BodyNode::ForLoop(f) => return f.body.iter().any(|bn| uses_coscos_down_from(bn)),
            BodyNode::IfChain(_) => {
                todo!("support if chain in Callbody.uses_coscos")
            }
            BodyNode::Text(_) | BodyNode::RawExpr(_) => false,
            BodyNode::Component(_) => {
                // TODO: check component children
                false
            }
        }
    }
    return body_nodes.iter().any(|bn| uses_coscos_down_from(bn));
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Coscos {
    pub source_text: String,
    pub brace: syn::token::Brace,
    pub ident: syn::Ident,
    pub coscos_path: Option<(NodeCoscosPath, usize)>,
}

impl Parse for Coscos {
    fn parse(stream: ParseStream) -> Result<Self> {
        let ident: proc_macro2::Ident = stream.parse::<syn::Ident>()?;
        if ident.to_string() != "coscos" {
            return Err(syn::Error::new(
                ident.span(),
                "expected coscos (internal error)",
            ));
        }
        let content: ParseBuffer;
        let brace = syn::braced!(content in stream);

        let body = content.parse::<syn::Lit>()?;

        match body {
            syn::Lit::Str(litstr) => {
                let mut inner = litstr.to_token_stream().to_string();
                inner.remove(0);
                inner.remove(inner.len() - 1);
                return Ok(Self {
                    ident: ident,
                    source_text: inner,
                    brace,
                    coscos_path: None,
                });
            }
            _ => {
                return Err(syn::Error::new(
                    body.span(),
                    "expected string literal in here",
                ));
            }
        }
    }
}

impl ToTokens for Coscos {
    fn to_tokens(&self, _tokens: &mut TokenStream2) {
        // let body = Vec::from([BodyNode::Text(IfmtInput::new_static(&self.source_text))]);

        // tokens.append_all(quote! {
        //     __cx.element(
        //         dioxus_elements::coscos,
        //         __cx.bump().alloc([ #(#body),* ]),
        //         None,
        //     )
        // });
    }
}

#[cfg(feature = "coscos_feature")]
const OUT_FORMAT: &'static rsass::output::Format = &rsass::output::Format {
    style: rsass::output::Style::Compressed,
    precision: 5,
};

impl Coscos {
    #[cfg(feature = "shoobaloo")]
    pub fn coscosate(roots: &mut Vec<BodyNode>) {
        roots.iter_mut().for_each(|root| {
            let _ = Self::rec_coscosate_from(root, &mut Vec::new());
        });
    }

    #[cfg(feature = "shoobaloo")]
    pub fn rec_coscosate_from(
        from: &mut BodyNode,
        curr_element_stack: &mut Vec<String>,
    ) -> Result<()> {
        match from {
            BodyNode::Element(e) => {
                curr_element_stack.push(e.name.to_string());
                e.children.iter_mut().for_each(|child| {
                    Self::rec_coscosate_from(child, curr_element_stack).unwrap();
                });
                curr_element_stack.pop();
                return Ok(());
            }
            BodyNode::IfChain(_) => {
                todo!()
            }
            BodyNode::ForLoop(f) => {
                f.body.iter_mut().for_each(|child| {
                    Self::rec_coscosate_from(child, curr_element_stack).unwrap();
                });
                return Ok(());
            }
            BodyNode::Component(c) => {
                curr_element_stack.push(
                    c.name.segments.iter().fold("".to_string(), |curr, next| {
                        curr + "_" + &next.ident.to_string()
                    }),
                );
                c.children.iter_mut().for_each(|child| {
                    Self::rec_coscosate_from(child, curr_element_stack).unwrap();
                });
                curr_element_stack.pop();
                return Ok(());
            }
            BodyNode::Coscos(coscos) => {
                let wrapping_selector = if curr_element_stack.len() == 0 {
                    "".to_owned()
                } else {
                    let wrapping_selector = curr_element_stack.first().unwrap();
                    curr_element_stack
                        .iter()
                        .skip(1)
                        .fold(wrapping_selector.to_owned(), |curr, next| {
                            curr + " > " + &next.to_string()
                        })
                };
                // let mut wrapping_selector = curr_element_stack
                //     .iter()
                //     .fold("".to_owned(), |curr, next| curr + &next.to_string() + " > ");

                coscos.source_text = wrapping_selector + " {" + &coscos.source_text + "}";
                let scss = rsass::compile_scss(coscos.source_text.as_bytes(), *OUT_FORMAT);
                match scss {
                    Err(e) => {
                        return Err(syn::Error::new(
                            coscos.brace.span.join(),
                            &format!("invalid scss: {}", e),
                        ))
                    }
                    Ok(bytes) => {
                        let s = std::str::from_utf8(&bytes);
                        match s {
                            Err(e) => {
                                return Err(syn::Error::new(
                                    coscos.brace.span.join(),
                                    &format!("could not str::from_utf8(byte_array): {}", e),
                                ))
                            }
                            Ok(out_str) => {
                                coscos.source_text = out_str.to_owned();
                                return Ok(());
                            }
                        }
                    }
                }
            }
            BodyNode::Text(_) => return Ok(()),
            BodyNode::RawExpr(_) => {
                // Coscos does not currently work across nested expressions
                return Ok(());
            }
        }
    }

    #[cfg(feature = "coscos_feature")]
    pub fn gather_coscos(roots: &[BodyNode]) -> Vec<Coscos> {
        let mut res = Vec::new();
        for root in roots.iter() {
            res.append(&mut Coscos::gather_coscos_from(root));
        }
        return res;
    }

    #[cfg(feature = "coscos_feature")]
    pub fn gather_coscos_from(node: &BodyNode) -> Vec<Coscos> {
        match node {
            BodyNode::Text(_) => Vec::new(),
            BodyNode::Component(_) => Vec::new(),
            BodyNode::RawExpr(_) => Vec::new(),
            BodyNode::ForLoop(f) => {
                let mut res = Vec::new();
                for node in f.body.iter() {
                    res.append(&mut Coscos::gather_coscos_from(node));
                }
                res
            }
            BodyNode::IfChain(_) => Vec::new(),
            BodyNode::Coscos(c) => vec![c.clone()],
            BodyNode::Element(e) => {
                let mut res = Vec::new();
                for node in e.children.iter() {
                    res.append(&mut Coscos::gather_coscos_from(node));
                }
                res
            }
        }
    }

    #[cfg(feature = "coscos_feature")]
    pub fn coscosate2_at(node: &mut BodyNode, coscos_path: &NodeCoscosPath, curr_id: &mut usize) {
        match node {
            BodyNode::Text(_) | BodyNode::RawExpr(_) | BodyNode::Component(_) => {}
            BodyNode::Element(e) => {
                e.attributes.push(ElementAttrNamed {
                    el_name: ElementName::Custom(syn::LitStr::new(
                        "coscos-dud",
                        proc_macro2::Span::call_site(),
                    )),
                    attr: ElementAttr::CustomAttrText {
                        name: syn::LitStr::new(
                            &("data-dx-coscos".to_owned()),
                            proc_macro2::Span::call_site(),
                        ),
                        value: IfmtInput {
                            source: None,
                            segments: vec![Segment::Literal(format!(
                                "{}-{}-{curr_id}",
                                coscos_path.comp_scope, coscos_path.rsx_block_scope,
                            ))],
                        },
                    },
                });
                for child in e.children.iter_mut() {
                    *curr_id += 1;
                    Coscos::coscosate2_at(child, coscos_path, curr_id);
                }
            }
            BodyNode::ForLoop(f) => {
                for child in f.body.iter_mut() {
                    Coscos::coscosate2_at(child, coscos_path, curr_id);
                }
            }
            BodyNode::IfChain(_) => {}
            BodyNode::Coscos(c) => {
                assert!(c.coscos_path.is_none(), "internal error: tried to set coscos_path twice on same node. Please report this issue");
                c.coscos_path = Some((coscos_path.clone(), *curr_id));
            }
        }
    }

    pub fn coscosate2(nodes: &mut Vec<BodyNode>, coscos_path: &NodeCoscosPath, curr_id: usize) {
        let mut curr_id = curr_id.clone();
        for node in nodes.iter_mut() {
            Coscos::coscosate2_at(node, coscos_path, &mut curr_id);
        }
    }

    #[cfg(feature = "coscos_feature")]
    pub fn get_scoped_css_text(&self) -> String {
        use lightningcss::printer::PrinterOptions;
        use lightningcss::stylesheet::{ParserOptions, StyleSheet};

        let wrapped_text = format!(
            "*[data-dx-coscos=\"{}-{}-{}\"] {{ {} }}",
            self.coscos_path.as_ref().unwrap().0.comp_scope,
            self.coscos_path.as_ref().unwrap().0.rsx_block_scope,
            self.coscos_path.as_ref().unwrap().1,
            self.source_text,
        );

        let mut opts = ParserOptions::default();
        opts.error_recovery = true;

        let ss = StyleSheet::parse(&wrapped_text, opts).unwrap();

        return ss.to_css(PrinterOptions::default()).unwrap().code;
    }

    #[cfg(feature = "coscos_feature")]
    pub fn collect_rsx_block_css(roots: &Vec<BodyNode>) -> String {
        let mut res = "".to_string();
        for root in roots.iter() {
            res += &Coscos::collect_rsx_block_css_at(root);
        }
        res
    }

    #[cfg(feature = "coscos_feature")]
    pub fn collect_rsx_block_css_at(body_node: &BodyNode) -> String {
        match body_node {
            BodyNode::Coscos(c) => c.get_scoped_css_text(),
            BodyNode::Element(e) => {
                let mut res = "".to_string();
                for child in e.children.iter() {
                    res += &Coscos::collect_rsx_block_css_at(child);
                }
                res
            }
            BodyNode::ForLoop(f) => {
                let mut res = "".to_string();
                for child in f.body.iter() {
                    res += &Coscos::collect_rsx_block_css_at(child);
                }
                res
            }
            BodyNode::IfChain(_) => "".to_string(),
            _ => "".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeCoscosPath {
    pub comp_scope: String,
    pub rsx_block_scope: String,
}
