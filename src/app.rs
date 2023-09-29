use std::time::Duration;
use leptos::{leptos_dom::helpers::debounce, *};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{to_value, from_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct GenUrlArgs<'a> {
    url: &'a str,
    id: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct CpyUrlArgs<'a> {
    url: &'a str,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (visible, set_visible) = create_signal(cx, false);
    let (url, set_url) = create_signal(cx, "https://devimp.lexikos.com".to_string());
    let (id, set_id) = create_signal(cx, "".to_string());
    let (display_url, set_display_url) = create_signal(cx, "".to_string());
    let (display_code, set_display_code) = create_signal::<Option<String>>(cx, None);

    let gen_url = create_action(cx, move |_| {
        async move {
            if id.get().is_empty() {
                set_display_url.set("".to_owned());
                set_display_code.set(None);
                return;
            }

            let args = to_value(&GenUrlArgs {
                url: &url.get(),
                id: &id.get(),
            }).unwrap();

            let res = invoke("hash", args).await;

            let values = from_value::<Vec<String>>(res).unwrap();

            if let Some(t) = values.get(0) {
                set_display_url.set(t.to_owned());
            }

            if let Some(t) = values.get(1) {
                set_display_code.set(Some(t.to_owned()));
            } else {
                set_display_code.set(None);
            }
        }
    });

    let mut debounce_gen_url = debounce(cx, Duration::from_millis(150), move |_| gen_url.dispatch(()));

    let copy_url = create_action(cx, move |_| {
        async move {
            set_visible.set(true);
            invoke("cpy", to_value(&CpyUrlArgs{ url: &display_url.get() }).unwrap()).await;
            set_timeout(move || set_visible.set(false), Duration::from_millis(500))
        }
    });

    view! { cx,
        <main>
            <AnimatedShow
               when=visible
               show_class="fade-in-1000"
               hide_class="fade-out-1000"
               hide_delay=Duration::from_millis(1000)
            >
               <div class="absolute left-1/2 top-0 -translate-x-1/2 text-white py-2 px-4 rounded-lg bg-green-500">
                   "复制成功"
               </div>
            </AnimatedShow>
            <div class="flex items-center mb-2">
                <select
                    class="bg-gray-100 border-none text-gray-900 text-sm rounded-lg focus:ring-blue-500 block mr-2 p-2.5"
                    on:change=move |ev| {
                        set_url.set(event_target_value(&ev));
                        gen_url.dispatch(());
                    }
                >
                    <option value="https://devimp.lexikos.com">"DEV"</option>
                    <option value="https://testimp.lexikos.com">"TEST"</option>
                    <option value="https://test2imp.lexikos.com">"TEST2"</option>
                    <option value="https://preimp.leedarson.com">"PRE"</option>
                    <option value="https://imp.leedarson.com">"PRO"</option>
                </select>
                <input
                    class="flex-grow bg-gray-100 border-none text-gray-900 text-sm rounded-lg focus:ring-blue-500 block mr-2 p-2.5"
                    placeholder="输入locationId,仅支持数字"
                    on:input=move |ev| {
                        set_id.set(event_target_value(&ev));
                        debounce_gen_url(());
                    }
                />
            </div>
            {
                move || match display_url.get().is_empty() {
                    true => view!{cx, <div></div>},
                    false => view!{cx, <div class="flex items-center">
                            <input class="flex-grow text-sm bg-gray-100 rounded-lg border-none mr-2 p-2.5" value=display_url.get() readonly />
                            <button
                                class="border-none text-center text-sm bg-sky-500 hover:bg-sky-600 text-white cursor-pointer rounded-lg px-4 py-2.5"
                                on:click=move |_| copy_url.dispatch(())
                            >
                                复制
                            </button>
                        </div>
                    },
                }
            }
            {
                move || match display_code.get() {
                    None => view!{cx, <div></div>},
                    Some(data) => view!{cx, <div><img src=data class="w-80 h-80" /></div>},
                }
            }
        </main>
    }
}
