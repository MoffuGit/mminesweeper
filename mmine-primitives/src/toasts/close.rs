use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};
use crate::toasts::ToastStoreFields;
use crate::toasts::root::ToastRootContext;

#[component]
pub fn ToastClose(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let ToastRootContext { toast, remove } =
        use_context().expect("should acces to the toast context");
    let spread = view! { <{..} class=class data-type=move || toast._type().get() on:click=move |_| { remove.run(()) } /> };
    view! {
        <RenderElement state=() render=render node_ref=node_ref element=html::button() {..spread}>
            {children.get_value().map(|children| children())}
        </RenderElement>
    }
}
