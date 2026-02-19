use leptos::{
    html::{ElementType, HtmlElement},
    prelude::*,
    tachys::html::node_ref::NodeRefContainer,
    wasm_bindgen::JsCast,
};
use leptos_node_ref::{AnyNodeRef, any_node_ref};
use leptos_typed_fallback_show::TypedFallbackShow;
use std::sync::Arc;

use leptos::either::Either;

#[component]
pub fn Primitive<E, C>(
    element: fn() -> HtmlElement<E, (), ()>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> impl IntoView
where
    E: ElementType + 'static,
    C: IntoView + 'static,
    View<C>: RenderHtml,
    HtmlElement<E, (), ()>: ElementChild<View<C>>,
    <HtmlElement<E, (), ()> as ElementChild<View<C>>>::Output: IntoView,
    <E as ElementType>::Output: JsCast,
    AnyNodeRef: NodeRefContainer<E>,
{
    let children = StoredValue::new(children.into_inner());

    view! {
        <TypedFallbackShow
            when=move || as_child.get().unwrap_or_default()
            fallback=move || {
                element().child(children.with_value(|children| children())).add_any_attr(any_node_ref(node_ref))
            }
        >
            {children.with_value(|children| children()).add_any_attr(any_node_ref(node_ref))}
        </TypedFallbackShow>
    }
}

#[component]
pub fn RenderElement<S, E>(
    element: HtmlElement<E, (), ()>,
    state: S,
    #[prop(into)] render: Option<RenderFn<S>>,
    children: ChildrenFn,
    node_ref: AnyNodeRef,
) -> impl IntoView
where
    E: ElementType,
    E: leptos::html::ElementWithChildren,
    S: Clone + Copy + 'static,
{
    view! {
        {
            if let Some(render) = render {
                Either::Left(render.run(node_ref, children, state))
            } else {
                Either::Right(element.child(children()).node_ref(node_ref))
            }
        }
    }
}

#[derive(Clone)]
pub struct RenderFn<S: Clone + Copy + 'static>(
    Arc<dyn Fn(AnyNodeRef, ChildrenFn, S) -> AnyView + Send + Sync + 'static>,
);

impl<S> Default for RenderFn<S>
where
    S: Clone + Copy + 'static + Default,
{
    fn default() -> Self {
        Self(Arc::new(|_, _, _| ().into_any()))
    }
}

impl<F, C, S> From<F> for RenderFn<S>
where
    F: Fn(AnyNodeRef, ChildrenFn, S) -> C + Send + Sync + 'static,
    C: RenderHtml + Send + 'static,
    S: Clone + Copy,
{
    fn from(value: F) -> Self {
        Self(Arc::new(move |node_ref, children, state| {
            value(node_ref, children, state).into_any()
        }))
    }
}

impl<S> RenderFn<S>
where
    S: Clone + Copy + 'static,
{
    pub fn run(&self, node_ref: AnyNodeRef, children: ChildrenFn, state: S) -> AnyView {
        (self.0)(node_ref, children, state)
    }
}
