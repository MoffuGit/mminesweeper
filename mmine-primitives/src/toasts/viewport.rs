use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};
use super::ToastContext;

#[component]
pub fn ToastViewport(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let ToastContext {
        hovering, view_ref, ..
    } = use_context().expect("should acces to the toast context");

    let handle_mouse_enter = move |_| {
        hovering.set(true);
    };

    let handle_mouse_leave = move |_| {
        hovering.set(false);
    };

    let state = Memo::new(move |_| hovering.get());

    let node_ref = if node_ref.get().is_some() { node_ref } else { AnyNodeRef::from(view_ref) };
    let spread = view! {
        <{..}
            class=class
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
            data-expanded=move || state.get().to_string()
        />
    };
    view! {
        <RenderElement state=() render=render node_ref=node_ref element=html::div() {..spread}>
            {children.get_value().map(|children| children())}
        </RenderElement>
    }
}
