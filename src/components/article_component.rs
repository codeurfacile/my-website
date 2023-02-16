use chrono::NaiveDate;
use yew::{function_component, html, use_state, Callback, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct ArticleBlock {
    pub title: String,
    pub text: Html,
    pub date: NaiveDate,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ArticleElement {
    pub article_block: ArticleBlock,
}

#[function_component(ArticleComponent)]
pub fn breaking_block_component(article_element: &ArticleElement) -> Html {
    let component = article_element.clone();
    let article_show_handle = use_state(|| false);
    let article_show = *article_show_handle;
    let handle_click = Callback::from(move |_| article_show_handle.set(true));
    html! {
        <>
            <div class="articleWrapper">
                <article>
                    <h2 class="articleTitle">{ component.article_block.title }</h2>
                    <p class="articleDate"><i>{"Écrit le "}{component.article_block.date.format("%d/%m/%Y")}</i></p>
                    if article_show {
                        <p class="articleTextAnim">
                            { component.article_block.text }
                        </p>
                    } else {
                        <p onclick={handle_click} class="articleText"><i>{"Click to expand..."}</i></p>
                    }
                </article>
            </div>
        </>
    }
}
