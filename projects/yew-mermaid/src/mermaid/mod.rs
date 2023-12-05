pub use mermaid_wasmbind::MermaidOptions;
use web_sys::{wasm_bindgen::UnwrapThrowExt, Element};
use yew::{prelude::*, Html};

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
