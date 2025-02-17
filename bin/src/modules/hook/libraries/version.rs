use rhai::plugin::{
    export_module, Dynamic, FnAccess, FnNamespace, Module, NativeCallContext, PluginFunction,
    RhaiResult, TypeId,
};

#[export_module]
pub mod version_functions {
    use hemtt_version::Version;

    #[rhai_fn(global, pure)]
    pub fn to_string(version: &mut Version) -> String {
        version.to_string()
    }

    #[rhai_fn(global, pure)]
    pub fn to_string_short(version: &mut Version) -> String {
        format!(
            "{}.{}.{}",
            version.major(),
            version.minor(),
            version.patch()
        )
    }

    #[rhai_fn(global, pure)]
    pub fn major(version: &mut Version) -> i64 {
        i64::from(version.major())
    }

    #[rhai_fn(global, pure)]
    pub fn minor(version: &mut Version) -> i64 {
        i64::from(version.minor())
    }

    #[rhai_fn(global, pure)]
    pub fn patch(version: &mut Version) -> i64 {
        i64::from(version.patch())
    }

    #[rhai_fn(global, pure)]
    pub fn build(version: &mut Version) -> Option<i64> {
        version.build().map(i64::from)
    }
}
