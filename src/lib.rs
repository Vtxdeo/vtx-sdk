/// SDK 入口模块，用于导出 WIT 接口与工具模块
/// 插件开发者仅需引入 `vtx_sdk::prelude::*` 即可使用全部功能。
pub mod bindings {
    wit_bindgen::generate!({
        world: "plugin",              // 指定 WIT world 名称
        path: "wit",                  // WIT 文件路径
        pub_export_macro: true,      // 允许导出宏（插件接口）
    });
}

// 子模块：数据库操作封装
pub mod db;

// 子模块：HTTP 请求/响应结构与构造器
pub mod http;

// 子模块：用户身份鉴权与上下文工具
pub mod auth;

// SDK 统一入口，聚合导出（给插件调用者使用）
pub mod prelude;

// 显式导出核心类型，供外部引用
pub use bindings::vtx::api::types::Manifest;
pub use bindings::vtx::api::auth_types::UserContext;
pub use bindings::export;
