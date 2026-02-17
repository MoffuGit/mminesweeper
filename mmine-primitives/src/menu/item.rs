use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::menu::MenuProviderContext;
use crate::primitive::{RenderElement, RenderFn};

#[component]
pub fn MenuItem(
    #[prop(optional)] close_on_click: bool,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let MenuProviderContext { open, .. } = use_context().expect("should acces the menu context");
    let children = StoredValue::new(children);
    let spread = view! {
        <{..}
            class=class
            on:click=move |_| {
                if close_on_click {
                    open.set(false)
                }
            }
        />
    };
    view! {
        <RenderElement
            state=()
            render=render
            node_ref=node_ref
            element=html::div()
            {..spread}
        >
            {children.get_value().map(|children| children())}
        </RenderElement>
    }
}
