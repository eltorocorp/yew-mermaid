#![recursion_limit = "1024"]

use web_sys::{
    console, wasm_bindgen::JsCast, EventTarget, HtmlInputElement, HtmlTextAreaElement,
};
use yew::{function_component, html, use_state, Callback, Html, InputEvent};
use yew_mermaid::Mermaid;

const CODE: &str = r#"
sequenceDiagram
    Perseus->>Server: Login
    Server->>Perseus: login url with user login state cookie
    Perseus->>Keycloak: Authenticate with Keycloak
    Keycloak->>Perseus: Redirection with code,state,etc
    Perseus->>Server: Exchange code, validate csrf
    Server-->>Server: Record user data
    Server-->>Perseus: User data with session
    Perseus-->>Perseus: App Unlock
    loop Data Requests with Session
        Perseus-->>Server: Catalogs/Schemas/Tables/Query
        Server-->>Keycloak: User Validation
        Keycloak-->>Server: User Validated
        Server-->>Scheduler: Request for Data
        Scheduler-->>Server: Response with Data
        Server-->>Perseus: Data
    end
"#;

pub fn header_view() -> Html {
    let title = "Mermaid.js for Yew";
    html! {
    <header>
        <h1 color="#009688">{title}</h1>
        <a href="https://github.com/GalAster/yew-mermaid.js">{"Fork me!"}</a>
    </header>
    }
}

pub enum Event {
    Input(String),
}

#[function_component(App)]
pub fn app() -> Html {
    let code = use_state(String::default);
    let code_setter = code.setter();

    let oninput = {
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = input {
                code_setter.set(input.value());
            }
        })
    };

    html! {
        <>
            {header_view()}
            <main>
            <textarea
                columns="20"
                placeholder="Input mermaid code"
                value={(*code).clone()}
                {oninput}
            />
            <div>
                <label>
                    {"Mermaid output svg:"}
                </label>
            </div>
            if !code.is_empty() {
                <Mermaid code={(*code).clone()} />
            }
            </main>
        </>
    }
}

pub struct Model {}
//     link: ComponentLink<Self>,
//     storage: StorageService,
//     input: String,
// }

// impl Component for Model {
//     type Message = Event;
//     type Properties = ();

//     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//         let input = {
//             match storage.restore("mermaid") {
//                 Json(Ok(restored)) => restored,
//                 _ => String::from(include_str!("placehold.md")),
//             }
//         };
//         Self { link, storage, input }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
//             Event::Input(s) => {
//                 self.input = s;
//                 self.storage.store("mermaid", Json(&self.input))
//             }
//         }
//         true
//     }

//     fn change(&mut self, _: Self::Properties) -> ShouldRender {
//         false
//     }

//     fn view(&self) -> Html {
//         html! {
//             <>
//                 {header_view()}
//                 <main>
//                 <textarea
//                      placeholder="Input mermaid code"
//                      value=&self.input
//                      oninput=self.link.callback(|input: InputData| Event::Input(input.value))
//                  />
//                 <div><label>{"Mermaid output svg:"}</label></div>
//                 <Mermaid code=&self.input/>
//                 </main>
//             </>
//         }
//     }
// }

fn main() {
    yew::Renderer::<App>::new().render();
}
