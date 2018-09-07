use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

// https://login.eveonline.com/oauth/authorize/?response_type=code&redirect_uri=eveauth-evegate://auth/&client_id=8b2f7dfa7d2f43ec894a50aa5739ea31&scope=publicData&state=A


fn handle_uri(args: &Vec<String>) -> std::io::Result<()>
{
    let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("evegate.log")?;

    for arg in args {
        file.write(arg.as_bytes())?;
        file.write(" ".as_bytes())?;
    }
    file.write("\n".as_bytes())?;
    Ok(())
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    handle_uri(&args)
}
