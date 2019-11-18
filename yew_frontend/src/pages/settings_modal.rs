use yew::{Component, ComponentLink, ShouldRender, Html, html, Properties};
use yew::virtual_dom::VNode;
use yewtil::NeqAssign;
use crate::common::{FetchState, fetch_to_state_msg};
use wire::bucket::{Bucket, ChangeBucketFlagsRequest};
use yew_router::unit_state::RouteAgentDispatcher;
use yew_router::agent::RouteRequest;
use yew_router::unit_state::Route;
use crate::AppRoute;
use stdweb::unstable::TryInto;
use wire::bucket_user_relation::{BucketUserPermissions, UserAndPermissions};
use crate::requests::bucket::{SetBucketFlags, GetUsersAndPermissionsInBucket};
use crate::pages::settings_modal::Msg::FetchedUsersPermissions;

pub struct SettingsModal {
    props: Props,
    link: ComponentLink<SettingsModal>,
    settings: Settings,
    users_and_their_settings: FetchState<Vec<UserAndPermissions>>,
    active_tab: SettingsTab
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SettingsTab {
    Bucket,
    Users
}

#[derive(Default, Debug, )]
pub struct Settings {
    is_public: bool,
    is_exclusive: bool,
    is_drawing: bool
}

impl Settings {
    fn from_bucket(bucket: &Bucket) -> Self {
        Self {
            is_public: bucket.public_viewable,
            is_exclusive: bucket.exclusive,
            is_drawing: bucket.drawing_enabled
        }
    }

    fn create_request(&self, bucket: &Bucket) ->  ChangeBucketFlagsRequest {
        let publicly_visible = if self.is_public != bucket.public_viewable {
            Some(self.is_public)
        } else {
            None
        };
        let drawing_enabled = if self.is_drawing != bucket.drawing_enabled {
            Some(self.is_drawing)
        } else {
            None
        };
        let exclusive = if self.is_exclusive != bucket.exclusive {
            Some(self.is_exclusive)
        } else {
            None
        };

        ChangeBucketFlagsRequest {
            publicly_visible,
            drawing_enabled,
            exclusive
        }
    }
}



#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    #[props(required)]
    pub bucket: Bucket,
    #[props(required)]
    pub permissions: BucketUserPermissions
}

pub enum Msg {
    Close,
    TogglePublic,
    ToggleExclusive,
    ToggleDrawing,
    SaveSettings,
    FetchedPutSettings(FetchState<Bucket>),
    FetchedUsersPermissions(FetchState<Vec<UserAndPermissions>>),
    SetTab(SettingsTab)
}

impl Component for SettingsModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let settings = Settings::from_bucket(&props.bucket);
        SettingsModal {
            props,
            link,
            settings,
            users_and_their_settings: Default::default(),
            active_tab: SettingsTab::Bucket
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        let fetch = fetch_to_state_msg(GetUsersAndPermissionsInBucket{bucket_uuid: self.props.bucket.uuid}, FetchedUsersPermissions);
        self.link.send_future(fetch);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => {
                let route = AppRoute::Bucket{ slug: self.props.bucket.bucket_slug.clone()};
                RouteAgentDispatcher::new().send(RouteRequest::ChangeRoute(Route::from(route)));
                false
            }
            Msg::TogglePublic => {
                self.settings.is_public = !self.settings.is_public;
                true
            }
            Msg::ToggleExclusive => {
                self.settings.is_exclusive = !self.settings.is_exclusive;
                true
            }
            Msg::ToggleDrawing => {
                self.settings.is_drawing = !self.settings.is_drawing;
                true
            }
            Msg::SaveSettings => {
                let request = SetBucketFlags{ bucket_uuid: self.props.bucket.uuid, flag_changeset: self.settings.create_request(&self.props.bucket) };
                let fetch = fetch_to_state_msg(request, Msg::FetchedPutSettings);
                self.link.send_future(fetch);
                true
            }
            Msg::FetchedPutSettings(_) => {
                // TODO replace the old settings. -> Use a callback to replace the parent's bucket
                false
            }
            Msg::SetTab(settings_tab) => self.active_tab.neq_assign(settings_tab),
            Msg::FetchedUsersPermissions(permissions) => {self.users_and_their_settings.neq_assign(permissions)}
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.settings = Settings::from_bucket(&props.bucket);
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html<Self> {
        return html! {
            <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-content" style = "height: 70%; overflow-y: hidden; position: inherit;">
                    <div class="card full_height2">
                        <div class="card-header">
                            <div class="card-header-title">
                                {"Settings"}
                            </div>
                        </div>
                        <div class="card-content full_height2 is-paddingless">
                            <div class="panel full_height_scrollable2">

                               {self.tab_panel_and_respective_children()}
                            </div>
                        </div>
                    </div>
                </div>
                <button class="modal-close is-large" aria-label="close" onclick=|_| Msg::Close></button>
            </div>
        }
    }
}


impl SettingsModal {

    fn tab_panel_and_respective_children(&self) -> Html<Self> {
        let tab_links = match self.active_tab {
            SettingsTab::Bucket => html! {
                <>
                    <a class="is-active">{"Bucket"}</a>
                    <a onclick = |_| Msg::SetTab(SettingsTab::Users)>{"Users"}</a>
                </>
            },
            SettingsTab::Users => html! {
                <>
                    <a onclick = |_| Msg::SetTab(SettingsTab::Bucket)>{"Bucket"}</a>
                    <a class="is-active">{"Users"}</a>
                </>
            }
        };

        let tab_content = match self.active_tab {
            SettingsTab::Bucket => self.bucket_permissions(),
            SettingsTab::Users => self.player_permissions()
        };

        html! {
            <>
                <p class="panel-tabs">
                    {tab_links}
                </p>
                {tab_content}
            </>
        }
    }

    fn bucket_permissions(&self) -> Html<Self> {
        html! {
            <>
                <a class="panel-block" onclick=|_| Msg::TogglePublic >
                    <div class="level full_width">
                        <label>{"Public"}</label>
                        <div class = "is-size-7">
                            {"Show the bucket to anyone on the main page."}
                        </div>
                        <div class="level-right">
                            <input
                                id="publicSwitch"
                                type="checkbox"
                                name="publicSwitch"
                                class="switch"
                                checked= self.settings.is_public
                                disabled= !self.props.permissions.set_public_permission
                            />
                            <label for="publicSwitch">{'\u{00A0}'}</label> // Non-breaking space. The switch is targeted to this label.
                        </div>
                    </div>
                </a>
                <a class="panel-block">
                    <div class="level full_width">
                        <label>{"Drawing"}</label>
                        <div class="is-size-7">
                           {"Allows players to draw questions from this bucket."}
                        </div>
                        <div class="level-right">
                            <input
                                id="drawingSwitch"
                                type="checkbox"
                                name="drawingSwitch"
                                class="switch"
                                checked= self.settings.is_drawing
                                disabled= !self.props.permissions.set_drawing_permission
                            />
                            <label for="drawingSwitch">{'\u{00A0}'}</label> // Non-breaking space. The switch is targeted to this label.
                        </div>
                    </div>
                </a>
                <a class="panel-block" onclick=|_| Msg::ToggleExclusive >
                    <div class="level full_width">
                        <label>{"Exclusive"}</label>
                        <div class="is-size-7">
                            {"Prevents anyone from joining"}
                        </div>
                        <div class="level-right">
                            <input
                                id="exclusiveSwitch"
                                type="checkbox"
                                name="exclusiveSwitch"
                                class="switch"
                                checked= self.settings.is_exclusive
                                disabled= !self.props.permissions.set_exclusive_permission
                            />
                            <label for="exclusiveSwitch">{'\u{00A0}'}</label> // Non-breaking space. The switch is targeted to this label.

                        </div>
                    </div>
                </a>
            </>
        }
    }

    fn player_permissions(&self) -> Html<Self> {
        fn render_user_and_permissions(user_permissions: &UserAndPermissions) -> Html<SettingsModal> {
            let user_name = &user_permissions.user.google_name.clone().unwrap_or_else(||user_permissions.user.uuid.to_string());

            let mangle_id_switch = |permission: &str| -> String {
                format!("{}{}Switch", user_permissions.user.uuid, permission)
            };

            html! {
                <div class = "panel-block columns is-marginless">
                    <div class="is-4 column">
                        {user_name}
                    </div>

                    <div class="horiz_flex_wrap columns is-marginless">
                        <div class="is-4 column">
                            <input
                                class="switch"
                                id=mangle_id_switch("public")
                                name=mangle_id_switch("public")
                                type="checkbox"
                                checked = user_permissions.permissions.set_public_permission
                            />

                            <label
                                 for= mangle_id_switch("public")
                            >
                                {"Public"}
                             </label>

                        </div>

                        <div class="is-4 column">
                            <input
                                class="switch"
                                id=mangle_id_switch("drawing")
                                name=mangle_id_switch("drawing")
                                type="checkbox"
                                checked = user_permissions.permissions.set_drawing_permission
                            />
                            <label
                                for=mangle_id_switch("drawing")
                            >
                                {"Draw"}
                            </label>
                        </div>

                        <div class="is-4 column">
                            <input
                                class="switch"
                                id=mangle_id_switch("exclusive")
                                name=mangle_id_switch("exclusive")
                                type="checkbox"
                                checked = user_permissions.permissions.set_exclusive_permission
                            />
                            <label
                                for=mangle_id_switch("exclusive")
                            >
                                {"Exclusive"}
                            </label>
                        </div>

                        <div class="is-4 column">
                            <input
                                class="switch"
                                id=mangle_id_switch("admin")
                                name=mangle_id_switch("admin")
                                type="checkbox"
                                checked = user_permissions.permissions.grant_permissions_permission
                            />
                            <label
                                for=mangle_id_switch("admin")
                            >
                                {"Admin"}
                            </label>
                        </div>

                        <div class="is-4 column">
                            <input
                                class="switch"
                                id=mangle_id_switch("kick")
                                name=mangle_id_switch("kick")
                                type="checkbox"
                                checked = user_permissions.permissions.kick_permission
                            />
                            <label
                                for=mangle_id_switch("kick")
                            >
                                {"Kick"}
                            </label>
                        </div>
                    </div>
                </div>
            }
        }
        match &self.users_and_their_settings {
            FetchState::Success(users_and_permissions) => {
                users_and_permissions.iter().map(render_user_and_permissions).collect()
            },
            FetchState::NotFetching => html!{},
            FetchState::Fetching => html!{
                <div class="panel-block">
                    <progress class="progress is-small is-dark is-radiusless" max="100"></progress>
                </div>
            },
            FetchState::Failed(_) => html!{},
        }
    }
}