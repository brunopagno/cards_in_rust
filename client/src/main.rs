use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
           <span>{ "This is the app component thing" }</span>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
