use std::time::Duration;

fn main() {
    let result = ureq::get("http://ns1.dnspod.net:6666")
        .timeout(Duration::from_secs(5))
        .call();

    match result {
        Ok(r) => println!("{}", r.into_string().unwrap()),
        Err(e) => println!("{:?}", e),
    }
}
