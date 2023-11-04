use dioxus::prelude::*;

fn main() {
    unsafe { coscos::GLOBAL_STYLES.push(("BONK".to_string(), "woosh")) };
    dioxus_desktop::launch(app);
}

#[component]
fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    let bonk = "bonk";

    render! {
        div { class: "{bonk}" }
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
        CommanderComp { id: 0 }
        CommanderComp { id: 1 }
    }
}

#[component]
fn CommanderComp(cx: Scope, id: usize) -> Element {
    let switch: &UseState<bool> = use_state(cx, || true);

    assert!(__COMP_NAME == __COMP_NAME);

    render! {
        div {
            onclick: move |_| {
                switch.set(!switch.get());
            },
            class: "switch-{switch}",
            "Oo {id} oO"
            div {
                coscos { "color: purple;" }
                "Nested stuff"
            }
            for i in 0..3 {
                div { "ye" }
            }
            if true {
                rsx!{div {"inside then block"}}
            }
            if false {
                rsx!{div {"should not see"}}
            } else {
                rsx!{div {"inside else block"}}
            }
            coscos {
                "
                font-weight: 900;
                &.switch-true {
                   background: blue;
                }
                &.switch-false {
                   background: red;
                }
            "
            }
        }
    }
}

/*
#[derive(Props, PartialEq)]
#[allow(non_camel_case_types)]
pub struct GeneralCompProps {
    id: usize,
}
#[allow(non_snake_case)]
pub fn ExploreExpanded<'a>(cx: Scope<'a, GeneralCompProps>) -> Element {
    #[warn(non_snake_case)]
    #[inline(always)]
    fn __dx_inner_comp<'a>(cx: Scope<'a, GeneralCompProps>) -> Element {
        const __COMP_NAME: &'static str = "ExploreExpanded";
        let GeneralCompProps { id } = &cx.props;
        {
            // let switch: &UseState<bool> = use_state(cx, || true);
            let mut count = use_state(cx, || 0);
            Some({
                let __cx: &Scoped<'_, GeneralCompProps> = cx;
                static TEMPLATE: ::dioxus::core::Template = ::dioxus::core::Template {
                    name: ":0:0:",
                    roots: &[
                        ::dioxus::core::TemplateNode::Element {
                            tag: dioxus_elements::h1::TAG_NAME,
                            namespace: dioxus_elements::h1::NAME_SPACE,
                            attrs: &[::dioxus::core::TemplateAttribute::Static {
                                name: ::dioxus::rsx::comp_time_concat!(
                                    "data-dx-coscos-",
                                    __COMP_NAME
                                ),
                                value: "",
                                namespace: None,
                            }],
                            children: &[::dioxus::core::TemplateNode::DynamicText { id: 0usize }],
                        },
                        ::dioxus::core::TemplateNode::Element {
                            tag: dioxus_elements::button::TAG_NAME,
                            namespace: dioxus_elements::button::NAME_SPACE,
                            attrs: &[::dioxus::core::TemplateAttribute::Dynamic { id: 0usize }],
                            children: &[::dioxus::core::TemplateNode::Text { text: "Up high!" }],
                        },
                        ::dioxus::core::TemplateNode::Element {
                            tag: dioxus_elements::button::TAG_NAME,
                            namespace: dioxus_elements::button::NAME_SPACE,
                            attrs: &[::dioxus::core::TemplateAttribute::Dynamic { id: 1usize }],
                            children: &[::dioxus::core::TemplateNode::Text { text: "Down low!" }],
                        },
                        ::dioxus::core::TemplateNode::Dynamic { id: 1usize },
                        ::dioxus::core::TemplateNode::Dynamic { id: 2usize },
                    ],
                    node_paths: &[&[0u8, 0u8], &[3u8], &[4u8]],
                    attr_paths: &[&[1u8], &[2u8]],
                };
                ::dioxus::core::VNode {
                    parent: None,
                    key: None,
                    template: std::cell::Cell::new(TEMPLATE),
                    root_ids: dioxus::core::exports::bumpalo::collections::Vec::with_capacity_in(
                        5usize,
                        __cx.bump(),
                    )
                    .into(),
                    dynamic_nodes: __cx.bump().alloc([
                        __cx.text_node(format_args!("High-Five counter: {count:}", count = count)),
                        __cx.component(
                            CommanderComp,
                            fc_to_builder(CommanderComp).id(0).build(),
                            "CommanderComp",
                        ),
                        __cx.component(
                            CommanderComp,
                            fc_to_builder(CommanderComp).id(1).build(),
                            "CommanderComp",
                        ),
                    ]),
                    dynamic_attrs: __cx.bump().alloc([
                        dioxus_elements::events::onclick(__cx, move |_| count += 1),
                        dioxus_elements::events::onclick(__cx, move |_| count -= 1),
                    ]),
                }
                // static TEMPLATE: ::dioxus::core::Template = ::dioxus::core::Template {
                //     name: ":0:0:",
                //     roots: &[
                //         ::dioxus::core::TemplateNode::Element {
                //             tag: dioxus_elements::div::TAG_NAME,
                //             namespace: dioxus_elements::div::NAME_SPACE,
                //             attrs: &[
                //                 ::dioxus::core::TemplateAttribute::Dynamic { id: 0usize },
                //                 ::dioxus::core::TemplateAttribute::Dynamic { id: 1usize },
                //             ],
                //             children: &[::dioxus::core::TemplateNode::DynamicText { id: 0usize }],
                //         },
                //         ::dioxus::core::TemplateNode::Element {
                //             tag: dioxus_elements::div::TAG_NAME,
                //             namespace: dioxus_elements::div::NAME_SPACE,
                //             attrs: &[],
                //             children: &[::dioxus::core::TemplateNode::Text { text: "BONK" }],
                //         },
                //         ::dioxus::core::TemplateNode::Element {
                //             tag: dioxus_elements::coscos::TAG_NAME,
                //             namespace: dioxus_elements::coscos::NAME_SPACE,
                //             attrs: &[],
                //             children: &[::dioxus::core::TemplateNode::Text { text: "BONK" }],
                //         },
                //     ],
                //     node_paths: &[&[0u8, 0u8]],
                //     attr_paths: &[&[0u8], &[0u8]],
                // };
                // ::dioxus::core::VNode {
                //     parent: None,
                //     key: None,
                //     template: std::cell::Cell::new(TEMPLATE),
                //     root_ids: dioxus::core::exports::bumpalo::collections::Vec::with_capacity_in(
                //         3usize,
                //         __cx.bump(),
                //     )
                //     .into(),
                //     dynamic_nodes: __cx
                //         .bump()
                //         .alloc([__cx.text_node(format_args!("Oo {} oO", id))]),
                //     dynamic_attrs: __cx.bump().alloc([
                //         dioxus_elements::events::onclick(__cx, move |_| {
                //             switch.set(!switch.get());
                //         }),
                //         __cx.attr(
                //             dioxus_elements::div::class.0,
                //             format_args!("switch-{}", switch),
                //             dioxus_elements::div::class.1,
                //             dioxus_elements::div::class.2,
                //         ),
                //     ]),
                // }
            })
        }
    }
    __dx_inner_comp(cx)
}
*/

// pub fn formatter_dud_fun() -> &[Element] {
//     let switch = use_state();
//     let id = 55;
//     return &[Some({
//         let __cx = cx;
//         static TEMPLATE: ::dioxus::core::Template =  ::dioxus::core::Template {
//             name:":0:0:",
//             roots: &[
//                 ::dioxus::core::TemplateNode::Element {
//                     tag:dioxus_elements::div::TAG_NAME,
//                     namespace:dioxus_elements::div::NAME_SPACE,
//                     attrs: &[
//                         ::dioxus::core::TemplateAttribute::Dynamic {
//                             id:0usize
//                         }, ::dioxus::core::TemplateAttribute::Dynamic {
//                             id:1usize
//                         }, ::dioxus::core::TemplateAttribute::Static {
//                             name:"",
//                             namespace:None,
//                             value:"",
//                         }
//                     ],
//                 children: &[
//                     ::dioxus::core::TemplateNode::DynamicText {
//                         id:0usize
//                     }, ::dioxus::core::TemplateNode::Element {
//                         tag:dioxus_elements::div::TAG_NAME,
//                         namespace:dioxus_elements::div::NAME_SPACE,
//                         attrs: &[::dioxus::core::TemplateAttribute::Static {
//                             name:"",
//                             namespace:None,
//                             value:"",
//                         }
//                         ],
//                         children: &[
//                             ::dioxus::core::TemplateNode::Element {
//                             tag:"style",
//                                 namespace:None,
//                                 attrs: &[],
//                                 children: &[::dioxus::core::TemplateNode::Text {
//                                     text:"div>div{color:red}\n"
//                                 }],
//                             }, ::dioxus::core::TemplateNode::Text {
//                                 text:"Nested stuff"
//                             }
//                         ],
//                     }, ::dioxus::core::TemplateNode::Element {
//                         tag:"style",
//                         namespace:None,
//                         attrs: &[],
//                         children: &[
//                             ::dioxus::core::TemplateNode::Text {
//                                 text:"div{font-weight:900}div.switch-true{background:blue}div.switch-false{background:red}\n"
//                             }
//                         ],
//                     }
//                 ],
//             }],
//             node_paths: &[&[0u8,0u8]],
//             attr_paths: &[&[0u8], &[0u8]],
//         };
//         ::dioxus::core::VNode {
//             parent: None,
//             key: None,
//             template: std::cell::Cell::new(TEMPLATE),
//             root_ids: dioxus::core::exports::bumpalo::collections::Vec::with_capacity_in(
//                 1usize,
//                 __cx.bump(),
//             )
//             .into(),
//             dynamic_nodes: __cx
//                 .bump()
//                 .alloc([__cx.text_node(format_args!("Oo {} oO", id))]),
//             dynamic_attrs: __cx.bump().alloc([
//                 dioxus_elements::events::onclick(__cx, move |_| {
//                     switch.set(!switch.get());
//                 }),
//                 __cx.attr(
//                     dioxus_elements::div::class.0,
//                     format_args!("switch-{}", switch),
//                     dioxus_elements::div::class.1,
//                     dioxus_elements::div::class.2,
//                 ),
//             ]),
//         }
//     })];
// }
