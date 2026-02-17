use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};

#[derive(Clone)]
pub struct GroupContext {
    label: RwSignal<Option<String>>,
}

#[component]
pub fn MenuGroup(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let label: RwSignal<Option<String>> = RwSignal::new(None);
    let children = StoredValue::new(children);
    view! {
        <Provider value=GroupContext { label }>
            <RenderElement
                state=()
                render=render
                node_ref=node_ref
                element=html::div()
            >
                {children.get_value().map(|children| children())}
            </RenderElement>
        </Provider>
    }
}
