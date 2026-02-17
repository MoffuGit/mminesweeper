use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::primitive::{RenderElement, RenderFn};
use crate::toasts::ToastStoreFields;
use crate::toasts::root::ToastRootContext;

#[component]
pub fn ToastTitle(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional, into)] render: Option<RenderFn<()>>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let ToastRootContext { toast, .. } = use_context().expect("should acces to the toast context");
    let spread = view! { <{..} class=class data-type=move || toast._type().get() /> };
    view! {
        <RenderElement state=() render=render node_ref=node_ref element=html::div() {..spread}>
            {move || toast.title().get()}
        </RenderElement>
    }
}
