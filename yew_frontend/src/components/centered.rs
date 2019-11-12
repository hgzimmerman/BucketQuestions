use yew::{html, Html, Component };
use yew_css::{Css, css_file};

thread_local! {
    static CSS: Css = css_file!("../../assets/centered.css"); // TODO, not sure where the assets folder should go.
}


pub fn centered<T: Component>(inner: Html<T>) -> Html<T> {
    CSS.with(|css| -> Html<T> {
        html! {
            <div class=&css["centered"]>
                {inner}
            </div>
        }
    })
}