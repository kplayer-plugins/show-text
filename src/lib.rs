extern crate kplayer_rust_wrap;

use kplayer_rust_wrap::kplayer;

struct ShowText {}
impl ShowText {
    fn new() -> Self {
        ShowText {}
    }
}

impl kplayer::plugin::BasePlugin for ShowText {
    fn get_name(&self) -> String {
        String::from("show-text")
    }
    fn get_args(&mut self, _custom_args: std::collections::HashMap<String, String>) -> Vec<String> {
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from("text=none"));
        args.push(String::from("fontsize=17"));
        args.push(String::from("fontcolor=white"));
        args.push(String::from("fontfile=resource/font.ttf"));
        args.push(String::from("x=0"));
        args.push(String::from("y=0"));

        args
    }
    fn get_allow_custom_args(&self) -> Vec<&'static str> {
        vec!["text", "x", "y", "fontsize", "fontcolor", "fontfile"]
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
    fn validate_user_args(
        &self,
        _args: std::collections::HashMap<String, String>,
    ) -> std::result::Result<bool, &'static str> {
        for (key, value) in _args {
            if key == "fontfile" {
                if !kplayer::util::os::file_exist(&value) {
                    self.print_log(
                        kplayer::util::os::PrintLogLevel::ERROR,
                        format!("font file not eixst: {}", value).as_str(),
                    );
                    return Err("font file not exist");
                }
            }
        }

        Ok(true)
    }
}

kplayer_rust_wrap::export!(ShowText);
