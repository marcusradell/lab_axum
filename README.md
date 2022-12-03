# lab_axum

A lab to see if it was possible to use normal constructors for DI instead of a web framework DI. With Axum, it was!

## Run

`cargo run`

## Example

`curl localhost:3000/identities/create -d '{"email": "me@example.com"}' -v -H 'Content-Type: application/json'`

Server logs:

```
Identity created! (Fake)
[src/domains/identities/mod.rs:26] data = CreateData {
    _id: "8de458c2-c0c6-471b-8dd1-75afe6e32788",
    _email: "me@example.com",
}
```

Curl output:

```
*   Trying 127.0.0.1:3000...
* Connected to localhost (127.0.0.1) port 3000 (#0)
> POST /identities/create HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/7.84.0
> Accept: */*
> Content-Type: application/json
> Content-Length: 27
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 0
< date: Sat, 03 Dec 2022 16:50:36 GMT
<
* Connection #0 to host localhost left intact
```
