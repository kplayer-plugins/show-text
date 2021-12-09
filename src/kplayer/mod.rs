pub mod plugin;
pub mod util;

static mut author: String = String::new();
static mut filter_name: String = String::new();
static mut media_type: plugin::MediaType = plugin::MediaType::MediaTypeNone;
static mut args: Vec<String> = Vec::new();

pub fn export_plugin(p: impl plugin::BasePlugin) {
    unsafe {
        author = p.get_author();
        filter_name = p.get_filter_name();
        media_type = p.get_media_type();
        args = p.get_args();
    }
}

#[no_mangle]
pub extern "C" fn GetFilterName() -> i32 {
    unsafe { util::string::DynamicString::from(filter_name.as_bytes()).get_index() }
}
