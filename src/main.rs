mod components;

use yew::prelude::*;

use components::breaking_block_component::BreakingBlockComponent;
use components::helpers::codeur_facile_description_block;
use components::helpers::{facebook_link, github_link, twitter_link, youtube_link};
use components::social_media_block_component::SocialMediaBlockComponent;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BreakingBlockComponent breaking_block = { codeur_facile_description_block() } />
            <footer><div>
                <SocialMediaBlockComponent social_media_block = { youtube_link() } />
                <SocialMediaBlockComponent social_media_block = { github_link() } />
                <SocialMediaBlockComponent social_media_block = { twitter_link() } />
                <SocialMediaBlockComponent social_media_block = { facebook_link() } />
            </div></footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
