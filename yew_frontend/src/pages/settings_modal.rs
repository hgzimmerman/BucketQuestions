use yew::{Component, ComponentLink, ShouldRender, Html, html, Properties};
use yew::virtual_dom::VNode;
use yewtil::NeqAssign;
use crate::common::FetchState;
use wire::bucket::Bucket;
use yew_router::unit_state::RouteAgentDispatcher;
use yew_router::agent::RouteRequest;
use yew_router::unit_state::Route;
use crate::AppRoute;
use stdweb::unstable::TryInto;

pub struct SettingsModal {
    props: Props,
    link: ComponentLink<SettingsModal>,
    settings: Settings
}

#[derive(Default, Debug, )]
pub struct Settings {
    is_public: LoadingBool,
    is_exclusive: LoadingBool
}

#[derive(Debug)]
pub enum LoadingBool {
    True,
    False,
    LoadingTrue,
    LoadingFalse
}

impl Default for LoadingBool {
    fn default() -> Self {
        LoadingBool::LoadingFalse
    }
}

impl LoadingBool {
    pub fn eval(&self) -> bool {
        match self {
            LoadingBool::True| LoadingBool::LoadingTrue => true,
            LoadingBool::False | LoadingBool::LoadingFalse => false
        }
    }

    pub fn is_loading(&self) -> bool {
        match self {
            LoadingBool::LoadingFalse | LoadingBool::LoadingTrue => true,
            _ => false
        }
    }

    pub fn negate_if_not_loading(&self) -> Self {
        match self {
            LoadingBool::True => LoadingBool::False,
            LoadingBool::False => LoadingBool::True,
            LoadingBool::LoadingTrue => LoadingBool::LoadingTrue,
            LoadingBool::LoadingFalse => LoadingBool::LoadingFalse
        }
    }
}



#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub is_open: bool,
    pub bucket: FetchState<Bucket>,
}

pub enum Msg {
    Close,
    TogglePublic,
    ToggleExclusive
}

impl Component for SettingsModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SettingsModal {
            props,
            link,
            settings: Default::default()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => {
                let route = match &self.props.bucket {
                    FetchState::Success(bucket) => AppRoute::Bucket{ slug: bucket.bucket_slug.clone() },
                    _ => {
                        log::warn!("Bucket wasn't fetched, settings don't know where to go");
                        AppRoute::Index
                    }
                };
                RouteAgentDispatcher::new().send(RouteRequest::ChangeRoute(Route::from(route)));
                false
            }
            Msg::TogglePublic => {
                self.settings.is_public = self.settings.is_public.negate_if_not_loading();
                true
            }
            Msg::ToggleExclusive => {
                self.settings.is_exclusive = self.settings.is_exclusive.negate_if_not_loading();
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html<Self> {
        if self.props.is_open {
            return html! {
                <div class="modal is-active">
                    <div class="modal-background"></div>
                    <div class="modal-content">
                   // <!-- Any other Bulma elements you want -->
                        <div class="has-background-white">
                            <div class="panel">
                                <p class="panel-heading">
                                    {"Settings"}
                                </p>
                                <a class="panel-block">
                                    <div class="level full_width">
                                        <label>{"Pubilc"}</label>
                                        <div class="level-right">
                                            <input
                                                id="publicSwitch"
                                                type="checkbox"
                                                name="publicSwitch"
                                                class="switch"
                                                checked=self.settings.is_public.eval()
                                                disabled=self.settings.is_public.is_loading()
                                            />
                                            <label for="publicSwitch">{'\u{00A0}'}</label> // Non-breaking space. The switch is targeted to this label.
                                        </div>
                                    </div>
                                </a>
                                <a class="panel-block">
                                    {"Exclusive"}
                                </a>
                            </div>
                        </div>
                    </div>
                    <button class="modal-close is-large" aria-label="close" onclick=|_| Msg::Close></button>
                </div>
            }
        } else {
            return html!{}
        }
    }
}