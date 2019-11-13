use yew::{html, Html, Component };
use yew_css::{Css, css_file};


thread_local! {
    static CSS: Css = css_file!("../../assets/full_height.css"); // TODO, not sure where the assets folder should go.
}


pub fn full_height<T: Component>(inner: Html<T>) -> Html<T> {
    CSS.with(|css| -> Html<T> {
        html! {
            <div class=&css["full_height"]>
                {inner}
            </div>
        }
    })
}

pub fn full_height_scrollable<T: Component>(inner: Html<T>) -> Html<T> {
    CSS.with(|css| -> Html<T> {
        html! {
            <div class=&css["full_height_scrollable"]>
                {inner}
            </div>
        }
    })
}
