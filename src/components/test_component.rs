use yew::{function_component, Html, html};

#[function_component(TestComponent)]
pub fn test_component() -> Html {
    html!{
        <div>{ "Test" }</div>
    }
}