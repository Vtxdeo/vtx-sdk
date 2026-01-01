use crate::error::{VtxError, VtxResult};
use crate::events::PluginEvent;
use crate::http::{Request, Response, ResponseBuilder};
use crate::{Manifest, UserContext};

/// 更低样板的插件入口 Trait。
///
/// `wit-bindgen` 生成的 `bindings::Guest` 要求实现所有导出函数；
/// 此 Trait 为常见方法提供默认实现，并允许在 `handle`/`handle_event` 中直接返回 `VtxResult`。
pub trait VtxPlugin {
    fn handle(_req: Request) -> VtxResult<Response> {
        Ok(ResponseBuilder::not_found())
    }

    fn handle_event(_event: PluginEvent) -> VtxResult<()> {
        Ok(())
    }

    fn get_migrations() -> Vec<String> {
        Vec::new()
    }

    fn get_manifest() -> Manifest;

    fn get_resources() -> Vec<String> {
        Vec::new()
    }

    /// 默认返回 401，表示该插件不处理鉴权（不会阻断责任链中的其它插件）。
    fn authenticate(_headers: &[(String, String)]) -> VtxResult<UserContext> {
        Err(VtxError::AuthDenied(401))
    }
}

/// 将实现了 `VtxPlugin` 的类型导出为 WIT `world plugin` 的 Guest。
///
/// - `export_plugin!(MyPlugin)`：生成一个默认的 guest 适配类型并导出。
/// - `export_plugin!(MyPlugin => MyGuest)`：自定义适配类型名（同一 crate 多次调用时避免命名冲突）。
#[macro_export]
macro_rules! export_plugin {
    ($plugin:ty) => {
        $crate::export_plugin!($plugin => __VtxSdkGuest);
    };
    ($plugin:ty => $guest:ident) => {
        struct $guest;

        impl $crate::bindings::Guest for $guest {
            fn handle(req: $crate::http::Request) -> $crate::http::Response {
                match <$plugin as $crate::plugin::VtxPlugin>::handle(req) {
                    Ok(resp) => resp,
                    Err(err) => $crate::http::ResponseBuilder::error(err),
                }
            }

            fn handle_event(event: $crate::events::PluginEvent) -> Result<(), String> {
                match <$plugin as $crate::plugin::VtxPlugin>::handle_event(event) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(err.to_string()),
                }
            }

            fn get_migrations() -> Vec<String> {
                <$plugin as $crate::plugin::VtxPlugin>::get_migrations()
            }

            fn get_manifest() -> $crate::Manifest {
                <$plugin as $crate::plugin::VtxPlugin>::get_manifest()
            }

            fn get_resources() -> Vec<String> {
                <$plugin as $crate::plugin::VtxPlugin>::get_resources()
            }

            fn authenticate(
                headers: Vec<(String, String)>,
            ) -> Result<$crate::UserContext, u16> {
                use $crate::auth::IntoAuthResult as _;
                <$plugin as $crate::plugin::VtxPlugin>::authenticate(&headers).into_auth_result()
            }
        }

        $crate::export!($guest);
    };
}
