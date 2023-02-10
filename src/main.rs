mod components;

use yew::prelude::*;

use components::breaking_block_component::BreakingBlockComponent;
use components::helpers::codeur_facile_description_block;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BreakingBlockComponent breaking_block = { codeur_facile_description_block() } />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
