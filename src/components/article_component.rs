use chrono::NaiveDate;
use yew::{function_component, html, Html, Properties};

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
pub fn article_component(article_element: &ArticleElement) -> Html {
    let component = article_element.clone();
    html! {
        <>
            <div class="articleWrapper">
                <article>
                    <h2 class="articleTitle">{ component.article_block.title }</h2>
                    <p class="articleDate"><i>{"Écrit le "}{component.article_block.date.format("%d/%m/%Y")}</i></p>
                    <p class="articleText"><i>{"Click to visualize..."}</i></p>
                </article>
            </div>
        </>
    }
}

#[function_component(ArticleComponentDetails)]
pub fn article_component_details(article_element: &ArticleElement) -> Html {
    let component = article_element.clone();
    html! {
        <>
            <div class="articleWrapper">
                <article>
                    <h2 class="articleTitle">{ component.article_block.title }</h2>
                    <p class="articleDate"><i>{"Écrit le "}{component.article_block.date.format("%d/%m/%Y")}</i></p>
                    <p class="articleTextAnim">
                        { component.article_block.text }
                    </p>
                </article>
            </div>
        </>
    }
}
