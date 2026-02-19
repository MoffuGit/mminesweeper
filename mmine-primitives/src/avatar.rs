use crate::common::status::use_transition_status;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_node_ref::IntoAnyNodeRef;
use strum_macros::EnumString;

use crate::primitive::RenderElement;
use crate::primitive::RenderFn;

#[derive(Debug, Clone, PartialEq, Default, EnumString)]
pub enum ImageLoadingStatus {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error,
}

#[derive(Debug, Clone, Copy)]
pub struct AvatarState {
    image_status: RwSignal<ImageLoadingStatus>,
}

#[component]
pub fn AvatarRoot(
    #[prop(default = None)] render: Option<RenderFn<AvatarState>>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] image_status: RwSignal<ImageLoadingStatus>,
    children: ChildrenFn,
) -> impl IntoView {
    let state = AvatarState { image_status };
    view! {
        <Provider value=state>
            <RenderElement
                state=state
                node_ref=node_ref
                element=html::span()
                render=render
            >
                {children()}
            </RenderElement>
        </Provider>
    }
}

#[component]
pub fn AvatarImage(
    image_url: MaybeProp<String>,
    #[prop(optional, into)] on_loading_status_change: Option<Callback<ImageLoadingStatus, ()>>,
    #[prop(optional)] node_ref: NodeRef<html::Img>,
) -> impl IntoView {
    let AvatarState { image_status, .. } = use_context::<AvatarState>()
        .expect("AvatarImage expects an AvatarRoot context provider for ImageLoadingStatus");

    Effect::new(move |prev_url: Option<MaybeProp<String>>| {
        if prev_url.is_none() || prev_url.is_some_and(|url| url.get() != image_url.get()) {
            image_status.set(ImageLoadingStatus::Loading);
        }
        image_url
    });

    let img_node = NodeRef::<html::Img>::new();

    Effect::new(move |_| {
        if image_url.get().is_some()
            && let Some(img_element) = img_node.get()
        {
            let current_status = image_status.get_untracked();

            if !matches!(
                current_status,
                ImageLoadingStatus::Loaded | ImageLoadingStatus::Error
            ) && img_element.complete()
            {
                image_status.set(ImageLoadingStatus::Loaded);
            }
        }
    });

    Effect::new(move |_| {
        if let Some(cb) = on_loading_status_change {
            let status = image_status.get();
            cb.run(status);
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            <img
                on:load=move |_| {
                    image_status.set(ImageLoadingStatus::Loaded);
                }
                on:error=move |_| {
                    image_status.set(ImageLoadingStatus::Error);
                }
                class:hidden=true
            />
            <MountedAvatarImage image_url=image_url node_ref={node_ref} {..attrs}/>
        </AttributeInterceptor>
    }
}

#[component]
fn MountedAvatarImage(image_url: MaybeProp<String>, node_ref: NodeRef<html::Img>) -> impl IntoView {
    let AvatarState { image_status, .. } = use_context::<AvatarState>()
        .expect("AvatarImage expects an AvatarRoot context provider for ImageLoadingStatus");

    let is_visible = Signal::derive(move || image_status.get() == ImageLoadingStatus::Loaded);
    let state = use_transition_status(is_visible, node_ref.into_any());

    view! {
        <Show when=move || state.mounted.get()>
            <img
                node_ref=node_ref
                src=image_url
                data-state=move || state.transition_status.get().to_string()
            />
        </Show>

    }
}

#[component]
pub fn AvatarFallback(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] render: Option<RenderFn<RwSignal<ImageLoadingStatus>>>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let status = use_context::<RwSignal<ImageLoadingStatus>>()
        .expect("AvatarFallback expects an AvatarRoot context provider for ImageLoadingStatus");

    let children = StoredValue::new(children);
    let render = StoredValue::new(render);

    view! {
        <Show when=move || matches!(status.get(), ImageLoadingStatus::Loading | ImageLoadingStatus::Error)>
            <RenderElement
                state=status
                node_ref=node_ref
                render=render.get_value()
                element=html::span()
                {..}
                class=class
            >
                {children.get_value().map(|children| children())}
            </RenderElement>
        </Show>
    }
}
