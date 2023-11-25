extern crate winresource;

fn main() {
    if let Ok(os) = std::env::var("CARGO_CFG_TAREGET_OS") {
        if os == "windows" {
            let mut res = winresource::WindowsResource::new();
            res.set_icon("src/jemc_circular.png");
            if let Err(e) = res.compile() {
                eprintln!("Failed to compile resources: {e}");
            }
        }
    } else {
        eprintln!("Failed to get OS");
    }
}
