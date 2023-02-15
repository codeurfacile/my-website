use yew::{function_component, html, use_state, Callback, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct BreakingBlock {
    pub id: usize,
    pub text: String,
    pub text_destroyed: String,
    pub texture_url: String,
    pub broken_texture_url: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct BreakingBlockElement {
    pub breaking_block: BreakingBlock,
}

#[function_component(BreakingBlockComponent)]
pub fn breaking_block_component(breaking_block_element: &BreakingBlockElement) -> Html {
    let block = breaking_block_element.clone();
    let block_class_handle = use_state(|| "first_text");
    let block_class = block_class_handle.clone();
    let block_text_handle = use_state(|| block.breaking_block.text);
    let block_text = (*block_text_handle).clone();
    let block_image_handle = use_state(|| block.breaking_block.texture_url);
    let block_image = (*block_image_handle).clone();
    let handle_over = Callback::from(move |_| {
        block_text_handle.set(block.breaking_block.text_destroyed.to_string());
        block_image_handle.set(block.breaking_block.broken_texture_url.to_string());
        block_class_handle.set("second_text");
    });

    html! {
        <>
            <div>
                <div style="display: flex; justify-content: center"><img src={block_image} onmouseover={handle_over} style="width: 400px; display: inline-block"/></div>
                <div style="text-align: center" class={block_class.to_string()}>{ block_text }</div>
            </div>
        </>
    }
}
