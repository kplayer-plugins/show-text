mod kplayer;

// #![no_std]

struct ShowText {}

impl kplayer::plugin::BasePlugin for ShowText {
    fn get_args(&self) -> std::vec::Vec<std::string::String> {
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from("text=none"));
        args.push(String::from("fontsize=17"));
        args.push(String::from("fontcolor=white"));
        args.push(String::from("fontfile=font.ttf"));
        args.push(String::from("x=0"));
        args.push(String::from("y=0"));

        args
    }
    fn get_author(&self) -> std::string::String {
        String::from("kplayer")
    }
    fn get_filter_name(&self) -> std::string::String {
        String::from("drawtext")
    }
    fn get_media_type(&self) -> kplayer::plugin::MediaType {
        kplayer::plugin::MediaType::MediaTypeVideo
    }
    fn validate_args(&self, args: &Vec<String>) -> std::result::Result<bool, &'static str> {
        Ok(true)
    }
}

#[no_mangle]
pub extern "C" fn Initialization() -> i32 {
    kplayer::export_plugin(Box::new(ShowText {}));
    0
}

// #[cfg(not(test))]
// #[panic_handler]
// fn handle_panic(_: &core::panic::PanicInfo) -> ! {
//     unreachable!()
// }
