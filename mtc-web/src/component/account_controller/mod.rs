use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use tracing::error;

use mtc_model::auth_model::AuthModelTrait;

use crate::APP_STATE;
use crate::handler::auth_handler::AuthHandler;
use crate::router::Route::{AdministratorPage, DashboardPage};

#[component]
pub fn AccountControllerComponent() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = APP_STATE.peek().auth.signal();
    let i18 = use_i18();

    let sign_out = |_| {
        spawn(async move {
            let mut auth_state = APP_STATE.peek().auth.signal();

            match APP_STATE.peek().api.sign_out().await {
                Ok(auth_model) => { auth_state.set(auth_model) }
                Err(e) => error!("SignOut: {}", e.message())
            }
        });
    };

    rsx! {
        if !auth_state().is_auth() {
            Link { class: "btn btn-ghost join-item",
                to: DashboardPage {},
                Icon {
                    width: 22,
                    height: 22,
                    icon: dioxus_free_icons::icons::md_action_icons::MdLogin
                }
            }
        } else {
            div { class: "dropdown dropdown-end dropdown-hover join-item",
                div { tabindex: "0", role: "button", class: "btn btn-ghost join-item",
                    Icon {
                        width: 28,
                        height: 28,
                        icon: dioxus_free_icons::icons::md_social_icons::MdPersonOutline
                    }
                }
                ul { tabindex: "0", class: "w-52 rounded border p-2 shadow-md dropdown-content z-[1] menu bg-base-100 input-bordered",
                    "onclick": "document.activeElement.blur()",
                    li { Link { to: DashboardPage {},
                        Icon {
                                width: 18,
                                height: 18,
                                icon: dioxus_free_icons::icons::md_action_icons::MdSettings
                        }
                        { translate!(i18, "messages.settings") } }
                    }
                    if auth_state().is_admin() {
                        li { Link { to: AdministratorPage {},
                            Icon {
                                width: 18,
                                height: 18,
                                icon: dioxus_free_icons::icons::md_action_icons::MdAdminPanelSettings
                            }
                            { translate!(i18, "messages.administrator") }
                        } }
                    }
                    div { class: "divider my-0" }
                    li {
                        a {
                            onclick: sign_out,
                            Icon {
                                width: 18,
                                height: 18,
                                icon: dioxus_free_icons::icons::md_action_icons::MdLogout
                            }
                            { translate!(i18, "messages.sign_out") }
                        }
                    }
                }
            }
        }
    }
}