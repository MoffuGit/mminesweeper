use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};

#[component]
pub fn GroupLabel(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let spread = view! {
        <{..}
            class=class
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
