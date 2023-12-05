pub use mermaid_wasmbind::MermaidOptions;
use web_sys::{wasm_bindgen::UnwrapThrowExt, Element, HtmlElement};
use yew::{prelude::*, Component, Html};

#[derive(Properties, Clone, PartialEq, Default)]
pub struct MermaidProperties {
    pub code: String,
    #[prop_or(true)]
    pub arrow_marker_absolute: bool,
    #[prop_or(String::from("default"))]
    pub theme: String,
}

#[function_component(Mermaid)]
pub fn mermaid(props: &MermaidProperties) -> Html {
    let node = use_node_ref();

    {
        let node = node.clone();
        let mut opts = MermaidOptions::default();
        opts.set_theme(&props.theme);
        let code = props.code.clone();

        use_effect_with((), move |_| {
            let elem: Element = node.cast().expect_throw("failed to get mermaid element");
            let x = opts.render(&elem, &code);
            elem.set_inner_html(&x);
        });
    }

    html! {
        <div ref={node}></div>
    }
}

// #[derive(Default)]
// pub struct Mermaid {
//     pub props: MermaidProperties,
// }

// impl Component for Mermaid {
//     type Message = ();
//     type Properties = MermaidProperties;

//     fn create(ctx: &Context<Mermaid>) -> Self {
//         Self::default()
//     }

//     fn update(&mut self, ctx: &yew::Context<Mermaid>, _: Self::Message) -> bool {
//         false
//     }

//     fn changed(&mut self, ctx: &yew::Context<Mermaid>, props: &MermaidProperties) -> bool {
//         match &self.props == props {
//             true => false,
//             false => {
//                 self.props = props.clone();
//                 true
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Mermaid>) -> Html {
//         let drawer = self.create_drawer();
//         let document = gloo_utils::document();
//         let t = document.create_element("div").unwrap();
//         t.set_inner_html(&drawer.render(&t, &self.props.code));
//         Html::VRef(t.first_child().unwrap().into())
//     }
// }

// impl Mermaid {
//     pub fn create_drawer(&self) -> MermaidOptions {
//         let mut render = MermaidOptions::default();
//         render.set_theme(&self.props.theme);
//         return render;
//     }
// }
