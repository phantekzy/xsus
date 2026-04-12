use xsus::Xsus;

fn main() {
    let mut client = Xsus::new("http://httpbin.org/");

    client.interceptors.request.push(Box::new(|mut req| {
        req.headers
            .insert("X-Provider".to_string(), "Xsus-Native".to_string());
        req
    }));

    println!("Sending Xsus Request to example.com...");

    match client.get("/") {
        Ok(res) => {
            println!("Success! Status: {}", res.status);
            println!("Headers: {:?}", res.headers);
            if res.data.len() > 50 {
                println!("Data: {}...", &res.data[..50]);
            } else {
                println!("Data: {}", res.data);
            }
        }
        Err(e) => eprintln!("Xsus Error: {}", e),
    }
}
