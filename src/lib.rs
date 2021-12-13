extern crate kplayer_rust_wrap;

use kplayer_rust_wrap::kplayer;

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
    fn validate_args(&self, _args: &Vec<String>) -> std::result::Result<bool, &'static str> {
        for str in _args {
            let sp: Vec<&str> = str.split('=').collect();
            if sp.len() < 2 {
                return Err("args format error");
            }

            // validte font file exist
            if sp[0] == "fontfile" {
                if !kplayer::util::os::file_exist(sp[1].to_string()) {
                    return Err("font file not exist");
                }
            }
        }

        Ok(true)
    }
}

kplayer_rust_wrap::export!(ShowText);
