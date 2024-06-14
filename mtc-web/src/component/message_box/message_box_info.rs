use dioxus::prelude::*;

pub fn MessageBoxInfo(message: String) -> Element {
    rsx! {
        div { role: "alert", class: "flex flex-row p-4 gap-2 rounded border border-info text-info",
            svg {
                "fill": "none",
                "xmlns": "http://www.w3.org/2000/svg",
                "viewBox": "0 0 24 24",
                class: "stroke-current shrink-0 w-6 h-6",
                path {
                    "stroke-linejoin": "round",
                    "d": "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                    "stroke-linecap": "round",
                    "stroke-width": "2"
                }
            }
            span { { message } }
        }
    }
}