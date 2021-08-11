use crate::IpVersion::V6;

#[derive(Debug)]
enum IpVersion {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Write(String),
    Read(),
}

#[derive(Debug)]
struct IpData {
    version: IpVersion,
    address: String,
}

impl Message {
    fn send(&self) {
        println!("send message");
    }
}

impl IpData {
    fn new_ip(version: IpVersion, address: String) -> IpData {
        IpData {
            version,
            address,
        }
    }
}

fn main() {
    let v = IpVersion::V4;
    println!("{:?}", v);

    let ip = IpData::new_ip(IpVersion::V4, String::from("127.0.0.1"));
    println!("{:#?}", ip);

    let local = IpAddr::V4(127, 0, 0, 1);
    let ip6 = IpAddr::V6(String::from("::1"));
    println!("{:?} {:?}", local, ip6);
    println!("---------------------------------------");

    let m = Message::Write(String::from("hello"));
    m.send();

    let some_num: Option<i32>= Some(5);
    let n = Some(6);
    println!("{:?} {:?}", some_num, n);

}
