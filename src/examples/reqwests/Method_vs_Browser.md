# Correspondence Between What You See in Browser, Dev Tools, and Methods inside Reqwest

## URL

    https://www.rust-lang.org

## Browser

    Provides the web page

## Dev Tools

![GET_RustLangOrg](https://user-images.githubusercontent.com/66894106/235514674-fc56e361-42e8-4a8f-8d16-1a93176b41e6.png)

## Reqwest Methods:

> printing Response itself gives
>
> `Response { url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("www.rust-lang.org")), port: None, path: "/", query: None, fragment: None }, status: 200, headers: {"content-type": "text/html; charset=utf-8", "content-length": "19657", "connection": "keep-alive", "server": "Rocket", "x-frame-options": "SAMEORIGIN", "permissions-policy": "interest-cohort=()", "x-content-type-options": "nosniff", "x-xss-protection": "1; mode=block", "strict-transport-security": "max-age=63072000", "referrer-policy": "no-referrer, strict-origin-when-cross-origin", "content-security-policy": "default-src 'self'; frame-ancestors 'self'; img-src 'self' avatars.githubusercontent.com; frame-src 'self' player.vimeo.com", "date": "Mon, 01 May 2023 19:23:09 GMT", "via": "1.1 vegur, 1.1 869fd0f96fdb3c4fb055832b019f2d1e.cloudfront.net (CloudFront)", "vary": "Accept-Encoding", "x-cache": "Miss from cloudfront", "x-amz-cf-pop": "ATL59-P2", "x-amz-cf-id": "i7Bw7YBuLWgl3-ilgbFw2LG3nh36QgiLrZzK9cUC3JX2PAs0aPsgxg=="} }`
>
> Containing info such as:
>
> 1. They're using Rust's Rocket crate to run their site
> 1. Responds with HTML type
> 1. All of this is in a Rust struct, Response.

> `.text()` gives HTML as a well formatted String

> `.status()` gives 200 in a reqwest::ResponseCode struct

> `.url()` gives the URL struct from above:
>
> `Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("www.rust-lang.org")), port: None, path: "/", query: None, fragment: None }`

> `.headers()` gives the headers field from above:
>
> `{"content-type": "text/html; charset=utf-8", "content-length": "19657", "connection": "keep-alive", "server": "Rocket", "x-frame-options": "SAMEORIGIN", "permissions-policy": "interest-cohort=()", "x-content-type-options": "nosniff", "x-xss-protection": "1; mode=block", "strict-transport-security": "max-age=63072000", "referrer-policy": "no-referrer, strict-origin-when-cross-origin", "content-security-policy": "default-src 'self'; frame-ancestors 'self'; img-src 'self' avatars.githubusercontent.com; frame-src 'self' player.vimeo.com", "date": "Mon, 01 May 2023 19:32:11 GMT", "via": "1.1 vegur, 1.1 a52298b9a4983302c93091e39cb79af2.cloudfront.net (CloudFront)", "vary": "Accept-Encoding", "x-cache": "Miss from cloudfront", "x-amz-cf-pop": "ATL59-P2", "x-amz-cf-id": "YfonUr55twzEJ1As-Ig0HI1R0wBjdIecx5t4-75gxKMhqgt1WO2ayg=="}`

> The response is not JSON, so calling .json gives an error.

---

## URL

    https://www.boredapi.com/api/activity

## Browser

    Your browser will display the JSON response. Firefox will cook up a bit of HTML to get ahead of dev tools, but fundamentally the only thing received was a JSON string. Here the formatting has been prettied:

> { <br/>
> "activity":"Learn how the internet works",<br/>
> "type":"education",<br/>
> "participants":1,<br/>
> "price":0,<br/>
> "link":"",<br/>
> "key":"9414706",<br/>
> "accessibility":0.1<br/>
> }

## Dev Tools

![GET_Bored_API](https://user-images.githubusercontent.com/66894106/235520101-edf95730-bd0a-4eb8-aa6d-daa1ee14a292.png)

    Notice the response type is now "application/json"

## Reqwest Methods:

> Printing the Response struct directly (using debug print {:?}) gives you
>
> `Response { url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("www.boredapi.com")), port: None, path: "/api/activity", query: None, fragment: None }, status: 200, headers: {"server": "Cowboy", "connection": "keep-alive", "x-powered-by": "Express", "access-control-allow-origin": "*", "access-control-allow-headers": "Origin, X-Requested-With, Content-Type, Accept", "content-type": "application/json; charset=utf-8", "content-length": "143", "etag": "W/\"8f-s9F4x7F+QchQpBgcYQ1CQITfX+k\"", "date": "Mon, 01 May 2023 19:57:48 GMT", "via": "1.1 vegur"} }`

> `.json` requires you to have serde as well, and the JSON feature enabled for Reqwest. You can then call `.json::<MyStruct>().await` to attempt to take the response's JSON (assuming it responds in JSON) and deserialize it into the named struct. It's important that you name the struct type you have prepared to catch the JSON (here "MyStruct"), which need to at least be filled in completely by the JSON. If you do not, you will get the error:
>
> `type inside "async fn" body must be known in this context
cannot infer type for type parameter "T" declared on the method "json"`

> If you declare the struct with a field that doesn't match the JSON (u64 when the JSON gives the number in a string), you'll get:
>
> `thread 'main' panicked at 'called `Result::unwrap()`on an`Err` value: reqwest::Error { kind: Decode, source: Error("invalid type: string \"4101229\", expected u64", line: 1, column: 145) }', src\examples\reqwests\get.rs:50:51`

> `.text` gives the JSON in a String with the special characters escaped (rarely what you want from a JSON API)

---

## URL

    https://datausa.io/api/data?drilldowns=Nation&measures=Population

## Browser

    API responds with JSON instead of a webpage. See this other URL for response JSON example:

> [DataUSA.io/about/api](https://datausa.io/about/api)

## Dev Tools

![GET_DataUSA](https://user-images.githubusercontent.com/66894106/235530040-4d583a4e-c0a9-42c1-97c1-301203fff47b.png)

## Reqwest Methods:

---

## URL

## Browser

## Dev Tools

## Reqwest Methods:

---

## URL

## Browser

## Dev Tools

## Reqwest Methods:
