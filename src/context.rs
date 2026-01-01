use crate::bindings::vtx::api::{auth_types::CurrentUser, context};

pub type CurrentUserInfo = CurrentUser;

/// 获取当前请求上下文中的用户（若存在）。
pub fn current_user() -> Option<CurrentUser> {
    context::get_current_user()
}

pub trait CurrentUserExt {
    fn is_in_group(&self, group: &str) -> bool;
}

impl CurrentUserExt for CurrentUser {
    fn is_in_group(&self, group: &str) -> bool {
        self.groups.iter().any(|g| g == group)
    }
}

