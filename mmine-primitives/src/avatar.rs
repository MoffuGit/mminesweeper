use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
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

#[component]
pub fn AvatarRoot(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] render: Option<RenderFn<RwSignal<ImageLoadingStatus>>>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let status = RwSignal::new(ImageLoadingStatus::Idle);
    let spread = view! {
        <{..}
            class=class
        />
    };

    let children = StoredValue::new(children);
    view! {
        <Provider value=status>
            <RenderElement
                state=status
                node_ref=node_ref
                render=render
                element=html::span()
                {..spread}
            >
                {children.get_value().map(|children| children())}
            </RenderElement>
        </Provider>
    }
}

#[component]
pub fn AvatarImage(
    image_url: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] on_loading_status_change: Option<Callback<ImageLoadingStatus, ()>>,
) -> impl IntoView {
    let status = use_context::<RwSignal<ImageLoadingStatus>>()
        .expect("AvatarImage expects an AvatarRoot context provider for ImageLoadingStatus");

    let img_ref: NodeRef<html::Img> = NodeRef::new();

    Effect::new(move |prev_url: Option<MaybeProp<String>>| {
        if prev_url.is_none() || prev_url.is_some_and(|url| url.get() != image_url.get()) {
            status.set(ImageLoadingStatus::Loading);
        }
        image_url
    });

    Effect::new(move |_| {
        if image_url.get().is_some()
            && let Some(img_element) = img_ref.get()
        {
            let current_status = status.get_untracked();

            if !matches!(
                current_status,
                ImageLoadingStatus::Loaded | ImageLoadingStatus::Error
            ) && img_element.complete()
            {
                status.set(ImageLoadingStatus::Loaded);
            }
        }
    });

    Effect::new(move |_| {
        if let Some(cb) = on_loading_status_change {
            let status = status.get();
            cb.run(status);
        }
    });

    view! {
        <img
            node_ref=img_ref
            src=image_url
            class=class
            on:load=move |_| {
                status.set(ImageLoadingStatus::Loaded);
            }
            on:error=move |_| {
                status.set(ImageLoadingStatus::Error);
            }
            class:hidden=move || {
                let current_status = status.get();
                !matches!(current_status, ImageLoadingStatus::Loaded)
            }
        />
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
