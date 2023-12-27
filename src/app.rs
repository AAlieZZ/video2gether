use serde::Deserialize;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize, Debug)]
struct Video {
    path: String
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let videos = use_state(|| Vec::new());
    {
        let videos = videos.clone();
        use_effect_with_deps(move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("https://www.ilqy1314.xyz/vidapi/")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos.into_iter().map(|v| html_nested!{<option value={v.path.clone()}>{v.path.split("/").last()}</option>}).collect());
            });
            || ()
        }, ());
    }

    let greet_msg = use_state(|| Vec::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let new_msg = String::from("https://www.ilqy1314.xyz/") + &name;
                    greet_msg.set(vec![html_nested!{<source src={ new_msg } type="video/mp4" />}]);
                });

                || {}
            },
            name2,
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <div class="row">
                <video width="62%" controls=true>
                    {for greet_msg.iter().map(|v| v.clone())}
                    <script src="https://2gether.video/release/extension.website.user.js"></script>
                </video> 
            </div>

            <form class="row" onsubmit={greet}>
                <select id="ciphers" ref={greet_input_ref}>
                    {for videos.iter().map(|v| v.clone())}
                </select>
                <button type="submit">{"Play"}</button>
            </form>
        </main>
    }
}
