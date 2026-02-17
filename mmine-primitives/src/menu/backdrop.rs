use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::menu::MenuProviderContext;
use crate::primitive::{RenderElement, RenderFn};

#[component]
pub fn MenuBackDrop(
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let MenuProviderContext {
        open,
        modal,
        dismissible,
        transition_status,
        ..
    } = use_context().expect("should acces the menu context");

    let children = StoredValue::new(children);

    let on_click_handler = move |_| {
        if dismissible {
            open.set(false);
        }
    };

    let spread = view! {
        <{..}
            class=class
            data-state=move || transition_status.transition_status.get().to_string()
            data-modal=move || modal.to_string()
            on:click=on_click_handler
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
