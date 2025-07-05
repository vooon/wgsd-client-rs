use argh::FromArgs;

#[derive(FromArgs)]
/// WireGuard Service Discovery client
struct Cli {
    #[argh(option, short = 'd')]
    /// network interface name
    device: String,

    #[argh(option, short = 's')]
    /// DNS server to use, IP:PORT
    dns: String,

    #[argh(option, short = 'n')]
    /// base DNS Zone to lookup
    zone: String,

    #[argh(switch)]
    /// debug logging
    debug: bool,
}

fn main() {
    let cli: Cli = argh::from_env();

    println!("Hello, world! {}", cli.device);
}
