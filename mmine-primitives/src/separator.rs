#![allow(dead_code)]
use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};

pub use super::common::Orientation;

#[component]
pub fn Separator(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, default = Signal::from(Orientation::Horizontal))] orientation: Signal<Orientation>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let spread = view! {
        <{..}
            class=class
            data-orientation=move || orientation.get().to_string()
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
