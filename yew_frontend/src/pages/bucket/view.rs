use crate::pages::bucket::{BucketPage, Msg};
use yewtil::fetch::FetchState;
use yew::{Html, html};
use crate::pages::settings_modal::SettingsModal;
use crate::pages::bucket::control::join::JoinAction;
use crate::pages::bucket::control::answer::AnswerAction;

enum SettingsJoin {
    Settings,
    JoinBucket
}

impl BucketPage {

    pub fn view_impl(&self) -> Html<Self> {
        html! {
            <>
                {self.modal()}
                <div class= "has-background-primary full_height_scrollable">
                    <div class = "full_width">
                        <div class = "columns is-centered no_margin">
                            <div class="column is-two-thirds-tablet is-half-desktop is-centered">
                                {self.render_title()}
                                {self.render_q_and_a_card()}
                                {self.render_new_question_card()}
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }

    // TODO It may not be wise to rely on a 403/401 error to indicate that a certain button should exist.
    // TODO if it is indeed a 404, that is probably better.
    /// Show settings if the user is capable of editing any of them.
    /// Or, if the permissions can't be gotten, show the option to join the bucket.
    ///
    /// None represents that neither should be rendered
    fn show_settings_or_join_button(&self) -> Option<SettingsJoin> {
        match &self.permissions {
            FetchState::Success(permissions) => {
                if permissions.grant_permissions_permission
                    || permissions.kick_permission
                    || permissions.set_exclusive_permission
                    || permissions.set_drawing_permission
                    || permissions.set_public_permission {
                    Some(SettingsJoin::Settings)
                } else {
                    None
                }
            }
            // TODO proide a more narrow error here.
            FetchState::Failed(_) => {
                Some(SettingsJoin::JoinBucket)
            }
            _ => None
        }
    }

    fn modal(&self) -> Html<Self> {
        if let (FetchState::Success(bucket), FetchState::Success(permissions) )= (&self.bucket, &self.permissions) {
            if self.props.is_settings_open {
                return html!{
                    <SettingsModal bucket= bucket.clone() permissions = permissions.clone()/>
                }
            } else {
                return html!{}
            }
        } else {
            return html!{}
        }
    }

    fn render_title(&self) -> Html<Self> {
        let settings_modal_link_or_join_button = match self.show_settings_or_join_button() {
            Some(SettingsJoin::Settings) => html! {
                <a
                    onclick=|_| Msg::ShowSettingsModal
                    href="#" class="card-header-icon" aria-label="bucket settings"
                >
                    <span class="icon has-text-dark">
                        <i class="fas fa-cog" aria-hidden="true"></i>
                    </span>
                </a>
            },
            Some(SettingsJoin::JoinBucket) => html!{
                <div class="full_height">
                    <button
                        class="button"
                        onclick=|_| Msg::Joining(JoinAction::Join)
                    >
                        {"Join"}
                    </button>
                </div>
            },
            None => html!{}
        } ;

        let num_questions_in_bucket = if let FetchState::Success(count) = self.questions_in_bucket_count {
            html! {
                <span class= "" style = "padding-top: .75rem; padding-bottom: .75rem; padding-right: .25rem">
                    {format!("Q: {}", count)}
                </span>
            }
        } else {
            html!{}
        };

        let content = match &self.bucket {
            FetchState::Success(bucket) => html !{
                html!{
                    <>
                        <span class="card-header-title">
                            {&bucket.bucket_name}
                        </span>
                        {num_questions_in_bucket}
                        {settings_modal_link_or_join_button}
                    </>
                }
            },
            _ => html!{
                <>
                    <span class="card-header-title">
                        <progress class="progress is-small is-dark is-radiusless" max="100"></progress>
                        {crate::NBS}
                    </span>
                    {settings_modal_link_or_join_button}
                </>
            }
        };

        html! {

            <div class="card column_margin">
                <header class="card-header">
                    {content}
                </header>
            </div>
        }
    }

    fn render_q_and_a_card(&self) -> Html<Self> {
        let content = match &self.active_question {
            FetchState::Fetching => html! {
                <div class="card-footer">
                    <button
                        class = "button is-success card-footer-item is-radiusless"
                        disabled = true
                    >
                        {"Get A Random Question"}
                    </button>
                </div>
            },
            FetchState::NotFetching => html! {

                <div class="card-footer">
                    <button
                        class = "button is-success card-footer-item is-radiusless"
                        onclick=|_| Msg::GetARandomQuestion
                    >
                        {"Get A Random Question"}
                    </button>
                </div>
            },
            FetchState::Success(Some(question)) => html! {
                <>
                    <div class="card-content">
                        <div class="is-size-4">
                            <p>{&question.question_text}</p>
                        </div>
                        <br />


                        <div class="level">
                            <button class="button is-info" onclick = |_| Msg::PutQuestionBackInBucket>
                                {"Put Back"}
                            </button>
                            <button class="button is-warning" onclick = |_| Msg::DiscardQuestion>
                                {"Discard"}
                            </button>
                        </div>

                        <textarea
                            class = "textarea is-medium"
                            rows=6
                            value=&self.new_answer.new_answer_string
                            oninput=|e| Msg::Answer(AnswerAction::UpdateNewAnswer(e.value))
                            placeholder="Answer"
                        />
                    </div>

                    <div class="card-footer">
                        <button
                            class= "button is-success card-footer-item is-radiusless"
                            onclick= |_| Msg::Answer(AnswerAction::SubmitNewAnswer)
                            disabled=self.new_answer.new_answer_string.is_empty()
                        >
                            {"Answer"}
                        </button>
                    </div>

                </>
            },
            FetchState::Success(None) => html! {
                html! {
                    <>
                        <div class="card-content">
                            {"No questions in this bucket. Try adding some!"}
                        </div>
                        <div class="card-footer">
                            <button
                                class= "button is-success card-footer-item is-radiusless"
                                onclick=|_| Msg::GetARandomQuestion
                            >
                                {"Get A Random Question"}
                            </button>

                        </div>
                    </>
                }
            },
            FetchState::Failed(_) => html! {
                {"Something went wrong :("}
            }
        };

        let title = match &self.active_question {
            FetchState::Success(_) => "Answer Question",
            _ => "Draw Question From Bucket"
        };

        html!{
            <div class="card column_margin">
                <header class="card-header">
                    <p class="card-header-title">
                        {title}
                    </p>
                </header>
                {content}
            </div>
        }
    }

    fn render_new_question_card(&self) -> Html<Self> {
        let textarea: Html<Self> = match &self.new_question_create {
            FetchState::Success(_)
            | FetchState::NotFetching => html! {
                <textarea
                    class = "textarea is-medium"
                    rows=6
                    value=&self.new_question
                    oninput=|e| Msg::UpdateNewQuestion(e.value)
                    placeholder="New Question"
                />
            },
            FetchState::Fetching => html! {
                <textarea
                    class = "textarea is-medium is-loading"
                    rows=6
                    value=&self.new_question
                    oninput=|e| Msg::UpdateNewQuestion(e.value)
                    placeholder="New Question"
                />
            },
            FetchState::Failed(_) => html! {
                <textarea
                    class = "textarea is-medium is-danger"
                    rows=6
                    value=&self.new_question
                    oninput=|e| Msg::UpdateNewQuestion(e.value)
                    placeholder="New Question"
                />
            }
        };

        html! {
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">
                        {"New Question"}
                    </p>
                </header>
                <div class="card-content">
                    {textarea}
                </div>
                <div class="card-footer">
                    <button
                        class= "button is-success card-footer-item is-radiusless"
                        onclick=|_| Msg::SubmitNewQuestion
                        disabled=self.new_question.is_empty()
                    >
                         {"Submit New Question"}
                    </button>
                </div>
            </div>
        }
    }
}