#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn as_v4(addr: String) -> IpAddr{
        IpAddr::V4(addr)
    }
    fn as_v6(addr: String) -> IpAddr{
        IpAddr::V6(addr)
    }
}
fn main() {
    let ip1 = IpAddr::as_v4(String::from("my.home.domain"));
    let ip2 = IpAddr::as_v6(String::from("::1"));

    println!("ip1 is {:#?}", ip1);
    println!("ip2 is {:#?}", ip2);
}
