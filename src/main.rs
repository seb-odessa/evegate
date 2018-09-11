#![feature(core_intrinsics)]
#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate log;
extern crate simplelog;
extern crate rocket;
extern crate reqwest;

use std::env;

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            extern crate core;
            unsafe { core::intrinsics::type_name::<T>() }
        }
        let name = type_name_of(f);
        &name[6..name.len() - 4]
    }}
    // println!("{} (in {} [{}:{}:{}])", function!(), module_path!(), file!(), line!(), column!());
}


fn configure_logger()
{
    use simplelog::*;
    use std::fs::File;

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("evegate_info.log").unwrap()),
        ]
    ).unwrap();
}


// https://login.eveonline.com/oauth/authorize/?response_type=code&redirect_uri=eveauth-evegate://auth/&client_id=8b2f7dfa7d2f43ec894a50aa5739ea31&scope=publicData&state=A

fn transform_uri(uri: &String) -> Option<String>
{
    static HANDLED_URL: &str = "eveauth-evegate://";
    info!("{} <= {:?}", function!(), &uri);
    let result = if uri.starts_with(HANDLED_URL) {
        let rocket = rocket::ignite();
        let config = rocket.config();
        let rocket_host = format!("http://localhost:{}/", config.port);
        Some(uri.replace(HANDLED_URL, &rocket_host))
    } else {
        None
    };
    info!("{} => {:?}", function!(), &result);
    result
}

fn handle_args(args: &Vec<String>) -> std::io::Result<()>
{
    info!("{} <= {:?}", function!(), &args);
    let cmd = args.join(" ");
    info!("{} .. {:?}", function!(), &cmd);    
    if let Some(url) = transform_uri(&args[1]) // @todo impl out of bounds check
    {
        let mut result = reqwest::get(&url);
        info!("{} .. response = {:?}", function!(), &result);
    }
    let result = Ok(());
    info!("{} => {:?}", function!(), &result);
    result
}


mod proxy
{
use std;
use rocket;

#[derive(FromForm, Debug)]
struct Auth {
    code: String,
    state: String
}

#[get("/auth?<arg>")]
fn auth(arg: Auth) -> String {
    info!("{} <= {:?}", function!(), &arg);
    let result = format!("Authorization Code: {} Authorization State {}", arg.code, arg.state);
    info!("{} => {:?}", function!(), &result);
    result
}

pub fn launch(args: &Vec<String>) -> std::io::Result<()>{
    info!("{} <= {:?}", function!(), &args);
    rocket::ignite()
        .mount("/", routes![auth])
        .launch();
    
    let result = Ok(());
    info!("{} => {:?}", function!(), &result);
    result
}

}




fn main() -> std::io::Result<()> {
    configure_logger();
    let args: Vec<String> = env::args().collect();

    info!("{} <= {:?}", function!(), &args);
    let result = if args.len() > 1
    {
        handle_args(&args)
    }
    else
    {
        proxy::launch(&args)
    };

    info!("{} => {:?}", function!(), &result);
    result
}
