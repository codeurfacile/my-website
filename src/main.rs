mod components;

use yew::prelude::*;

use components::article_component::ArticleComponent;
use components::breaking_block_component::BreakingBlockComponent;
use components::helpers::codeur_facile_description_block;
use components::helpers::{
    facebook_link, github_link, introduction_rust_article, twitter_link, youtube_link,
};
use components::social_media_block_component::SocialMediaBlockComponent;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1></h1>
            <div id="breakingBlock">
                <BreakingBlockComponent breaking_block = { codeur_facile_description_block() } />
            </div>
            <div id="blog">
                <h1>{"Articles du blog: "}</h1>
                <div id="blogArticles">
                    <ArticleComponent article_block = { introduction_rust_article() } />
                </div>
            </div>
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
