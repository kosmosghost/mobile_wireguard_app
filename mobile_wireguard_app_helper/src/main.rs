mod app;
use std::{env, io::{stdout, self, Write}};

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let x = match &args[1] as &str {
        "connect" => app::connect_wg(),
        "disconnect" => app::disconnect_wg(),
        "refresh" => app::refresh_wg(),
        _ => panic!(),

    };

    let mut stdout = stdout().lock();
    stdout.write_all(x.as_bytes())?;

    Ok(())
}
