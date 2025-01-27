use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::record_model::RecordModel;

use crate::component::breadcrumb::Breadcrumb;
use crate::component::loading_box::LoadingBoxComponent;
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::handler::content_handler::ContentHandler;
use crate::page::administrator::AdministratorRouteModel;
use crate::APP_STATE;

#[component]
pub fn Content() -> Element {
    let app_state = APP_STATE.peek();
    let i18 = use_i18();

    let mut administrator_route = use_context::<Signal<AdministratorRouteModel>>();

    let active_content_api = app_state.active_content_api.signal();
    let mut active_content = app_state.active_content.signal();

    let content_future = use_resource(move || async move {
        let active_api = active_content_api();
        APP_STATE
            .peek()
            .api
            .get_content_list(&active_api.slug)
            .await
    });

    rsx! {
         match &*content_future.read() {
             Some(Ok(response)) => rsx! {
                 section { class: "flex grow flex-row",
                     div { class: "flex grow flex-col items-center gap-3 p-2 body-scroll",
                         div { class: "self-start",
                            Breadcrumb { title:
                                if active_content_api().slug.is_empty() {
                                    translate!(i18, "messages.singles")
                                } else {
                                    active_content_api().title.clone()
                                }
                            }
                         }
                         table { class: "table w-full",
                             thead {
                                 tr {
                                     th { class: "w-6" }
                                     th { { translate!(i18, "messages.slug") } }
                                     th { { translate!(i18, "messages.title") } }
                                 }
                             }
                             tbody {
                                 for item in response.iter() {
                                     {
                                         let m_item = RecordModel{ slug: item.slug.clone(), title: item.title.clone() };

                                         rsx! {
                                             tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                                 onclick: move |event| {
                                                     event.stop_propagation();
                                                     active_content.set(m_item.clone());
                                                     administrator_route.set(AdministratorRouteModel::ContentEditor);
                                                 },
                                                 td {
                                                     if !item.published {
                                                         Icon { class: "text-warning",
                                                             width: 16,
                                                             height: 16,
                                                             fill: "currentColor",
                                                             icon: dioxus_free_icons::icons::md_action_icons::MdVisibilityOff
                                                         }
                                                     }
                                                 }
                                                 td { { item.slug.clone() } }
                                                 td { { item.title.clone() } }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
                 if !active_content_api().slug.is_empty() {
                    button {
                        class: "absolute right-4 bottom-4 btn btn-circle btn-neutral",
                        onclick: move |_| {
                            active_content.set(RecordModel::default());
                            administrator_route.set(AdministratorRouteModel::ContentEditor);
                        },
                        Icon {
                            width: 26,
                            height: 26,
                            icon: dioxus_free_icons::icons::md_content_icons::MdAdd
                        }
                    }
                }
             },
             Some(Err(e)) => rsx! {
                div { class: "grid w-full place-items-center body-scroll",
                    ReloadingBoxComponent { message: e.message(), resource: content_future }
                }
             },
             None =>  rsx! {
                div { class: "grid w-full place-items-center body-scroll",
                    LoadingBoxComponent {}
                }
             },
         }
    }
}
