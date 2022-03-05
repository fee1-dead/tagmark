use serde::Serialize;
use wasm_bindgen::prelude::*;

use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace = ["browser", "tabs"])]
    async fn query(obj: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TabsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_discardable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie_store_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_window: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discarded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    highlighted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    muted: Option<bool>,
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let val = query(JsValue::from_serde(&TabsQuery::default()).unwrap())
        .await
        .unwrap();
        yew::start_app::<Model>();
    log!("Hello World! {val:?}");
}

use js_sys::Promise;
use std::time::Duration;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

pub async fn sleep(duration: Duration) {
    JsFuture::from(Promise::new(&mut |yes, _| {
        window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &yes,
                duration.as_millis() as i32,
            )
            .unwrap();
    }))
    .await
    .unwrap();
}
