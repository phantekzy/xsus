
<p align="center">
  <img src="xsusLogo.png" width="500" alt="Xsus Logo">
</p>

## What is Xsus?

Xsus is a tool built for the Rust programming language that allows your computer to talk to websites and servers. Unlike other tools that rely on many external parts, Xsus is built using only the basic parts of Rust. This makes it very small, fast, and easy to understand.

When you use Xsus, you are talking directly to the internet. It doesn't hide what is happening under the hood, which makes it perfect for people who want to learn how the web works or who want to build very clean software.

---

## Table of Contents
* [What is Xsus?](#what-is-xsus)
* [Features and Content](#features-and-content)
* [How it works](#how-it-works)
* [How to use it](#how-to-use-it)
* [Project Goals](#project-goals)
* [Maintainer](#maintainer)
* [License](#license)

---

## Features and Content

| Feature | What it does | Why it matters |
| :--- | :--- | :--- |
| **Direct Connection** | Connects straight to a website's address. | No middleman means fewer errors. |
| **Smart Address Finder** | Finds the IP address of any website name automatically. | You only need the name, like google.com. |
| **Auto-Cleaner** | Removes extra code that websites send to stay fast. | You get only the clean data you need. |
| **Custom Rules** | Lets you add special instructions before a request is sent. | Great for passwords or extra security. |
| **No Extra Weight** | It doesn't use any outside code or libraries. | Very safe and easy to move to any computer. |

---

## How it works

Xsus follows a simple three-step process every time you ask for information:

1. **The Ask:** You give Xsus a website name. Xsus finds where that website lives on the internet.
2. **The Talk:** Xsus opens a direct line to that server and asks for the page you want.
3. **The Clean:** Websites often send data in "chunks" to be fast. Xsus puts these pieces together and removes the technical markers so you just see the final text or HTML.

---

## How to use it

To use Xsus in your project, add it to your `Cargo.toml` file:

```toml
[dependencies]
xsus = "0.1.2"
```

## Example Code

Here is how you can use Xsus to get a page from the internet:

```Rust
use xsus::Xsus;

fn main() {
    // 1. Tell Xsus which website you want to talk to
    let mut client = Xsus::new("[http://httpbin.org](http://httpbin.org)");

    // 2. (Optional) Add a special rule, like a name tag for your request
    client.interceptors.request.push(Box::new(|mut req| {
        req.headers.insert("User-Name".to_string(), "Maini Lotfi");
        req
    }));

    // 3. Ask for a specific page
    match client.get("/get") {
        Ok(response) => {
            println!("Success! Status: {}", response.status);
            println!("Here is the data: {}", response.data);
        },
        Err(error) => {
            println!("Something went wrong: {}", error);
        }
    }
}
```

## Project Goals

Our goal is to evolve Xsus into a complete networking framework while staying light.

* 0.2.0: Create a "Builder Style" for easier coding (e.g., .header().send()).
* 0.2.1: Support sending data (POST/PUT) and body streaming.
* 0.3.0: Add secure "https" support using TLS/SSL via rustls.
* 0.4.0: Speed up multiple requests using connection pooling and Keep Alive.

## maintainer

Maini Lotfi (phantekzy)

## license

This project is licensed under the MIT License.

## Special Thanks

Special thanks to my cat, [Ferchouch](https://github.com/Ferchouch), for not peeing on my Laptop during the development of this engine.

