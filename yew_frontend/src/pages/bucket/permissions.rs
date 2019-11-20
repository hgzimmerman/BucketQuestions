use wire::bucket_user_relation::BucketUserPermissions;
use yewtil::fetch::{FetchState, fetch_to_state_msg};
use crate::pages::bucket::{BucketLink, Msg};
use yew::ShouldRender;
use uuid::Uuid;
use yewtil::NeqAssign;
use crate::requests::bucket::GetPermissionsForUser;

pub enum PermissionsAction {
    Get,
    Fetched(FetchState<BucketUserPermissions>),
}

#[derive(Debug, Default)]
pub struct PermissionsState {
    pub permissions: FetchState<BucketUserPermissions>
}


pub enum SettingsJoin {
    Settings,
    JoinBucket
}

impl PermissionsState {

    // TODO It may not be wise to rely on a 403/401 error to indicate that a certain button should exist.
    // TODO if it is indeed a 404, that is probably better.
    /// Deterimne if a settings button should be shown if the user is capable of editing any of them.
    /// Or, if the permissions can't be gotten, show the option to join the bucket.
    ///
    /// None represents that neither should be rendered
    pub fn show_settings_or_join_button(&self) -> Option<SettingsJoin> {
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


    pub fn update(&mut self, action: PermissionsAction, link: &mut BucketLink, bucket_uuid: Option<Uuid>) -> ShouldRender {
        match action {
            PermissionsAction::Get => self.get_user_permissions(link, bucket_uuid),
            PermissionsAction::Fetched(permissions) => self.permissions.neq_assign(permissions)
        }
    }

    fn get_user_permissions(&mut self, link: &mut BucketLink, bucket_uuid: Option<Uuid>) -> ShouldRender {
        log::info!("getting permissions");
        if let Some(bucket_uuid) = bucket_uuid {
            self.permissions.set_fetching();
            let request = GetPermissionsForUser{ bucket_uuid};
            link.send_future(fetch_to_state_msg(request, |resp| Msg::Permissions(PermissionsAction::Fetched(resp))));
            true
        } else {
            log::warn!("Did not have bucket to use in fetching permissions.");
            false
        }
    }
}
