mod kplayer;

// #![no_std]

struct ShowText {}

impl kplayer::plugin::BasePlugin for ShowText {
    fn get_args(&self) -> std::vec::Vec<std::string::String> {
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from("text=hello wasm"));
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
}

#[no_mangle]
pub extern "C" fn Initialization() -> i32 {
    let show_text = ShowText {};
    kplayer::export_plugin(show_text);
    0
}

// #[cfg(not(test))]
// #[panic_handler]
// fn handle_panic(_: &core::panic::PanicInfo) -> ! {
//     unreachable!()
// }
