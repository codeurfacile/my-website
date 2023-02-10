mod components;

use yew::prelude::*;
use components::test_component::TestComponent;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <TestComponent />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}