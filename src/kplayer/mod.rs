pub mod plugin;
pub mod util;

extern "C" {
    fn GetValidateArgIterator() -> i32;
}

static mut AUTHOR: String = String::new();
static mut FILTER_NAME: String = String::new();
static mut MEDIA_TYPE: plugin::MediaType = plugin::MediaType::MediaTypeNone;
static mut ARGS: Vec<String> = Vec::new();
static mut ARGS_INDEX: usize = 0;

static mut b: Vec<Box<plugin::BasePlugin>> = Vec::new();

fn Get(p: impl plugin::BasePlugin) -> String {
    p.get_filter_name()
}

pub fn export_plugin(p: Box<plugin::BasePlugin>) {
    unsafe {
        b.push(p);
        AUTHOR = p.get_author();
        FILTER_NAME = p.get_filter_name();
        MEDIA_TYPE = p.get_media_type();
        ARGS = p.get_args();
    }
}

#[no_mangle]
pub extern "C" fn GetFilterName() -> i32 {
    unsafe { util::string::DynamicString::from(FILTER_NAME.as_bytes()).get_index() }
}

#[no_mangle]
pub extern "C" fn GetAuthor() -> i32 {
    unsafe { util::string::DynamicString::from(AUTHOR.as_bytes()).get_index() }
}

#[no_mangle]
pub extern "C" fn GetMediaType() -> i32 {
    unsafe { MEDIA_TYPE as i32 }
}

#[no_mangle]
pub extern "C" fn GetArgIterator() -> i32 {
    unsafe {
        if ARGS_INDEX >= ARGS.len() {
            return 0;
        }

        let iterator = util::string::DynamicString::from(ARGS[ARGS_INDEX].as_bytes()).get_index();
        ARGS_INDEX = ARGS_INDEX + 1;

        iterator
    }
}

#[no_mangle]
pub extern "C" fn ValidateArgs() -> i32 {
    GetValidateArgIterator()
}
