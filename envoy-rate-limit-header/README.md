## Rate limit header POC

### About 

In this POC, a dummy local rate limit cache is used to demonstrate the functionality of performing rate limiting using a HTTP filter and shared data. Here we are rate limiting the traffic passed through the proxy for a certain time limit. For example default values are configured as follows. You can override the default by passing the configuration from envoy.yaml.

```sh
ratelimit_limit: 10,
ratelimit_remaining: 10,
ratelimit_reset: Duration::from_secs(30),
```

* ratelimit_limit - The total amount of requests allowed through the proxy during a specified time window.
* ratelimit_remaining - The remaining amount of requests allowed through the proxy during a specified time window.
* ratelimit_reset - The time window rate limiting is performed against. 

For every requests, the following response headers will be added from the response path of the HTTP filter.

```sh
x-3scale-RateLimit-Limit
x-3scale-RateLimit-Remaining
x-3scale-RateLimit-Reset
```

Also when the rate limit is reached for a particular specified time window, the filter will block the request and send a local response to the client with 429 Too many requests.

### Building the filter

Note: For building the filter as a WASM module, the following prerequisites are there. If you want to just test, then there's no need to build the filter. I have already built the filter and included it in the build folder.

* rust (nightly toolchain and the support for WASM compilation target)
    * rustup toolchain install nightly
    * rustup target add wasm32-unknown-unknown
* cargo 
* make

Go to the `envoy-rate-limit-header` root and run the following command to build the filter.

```sh
make build
```

The generated WASM module will be in the build folder.

### Running the setup to test the functionality

To run the setup using `docker-compose` , execute the following command from the project root folder. As mentioned in the previous section, you don't need to build the filter to run the setup since an already built filter is included.

```sh
docker-compose -f envoy-rate-limit-header/docker-compose.yaml up --build
```

### Testing the functionality of rate limiting and rate limit headers

1. Rate limit headers

Send a request using `curl` with verbose. 

```sh
curl localhost:9095/foo -v
```
See the following verbose for the rate limit headers.

```sh

*   Trying 127.0.0.1:9095...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 9095 (#0)
> GET /foo HTTP/1.1
> Host: localhost:9095
> User-Agent: curl/7.68.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< x-powered-by: Express
< content-type: application/json; charset=utf-8
< content-length: 40
< etag: W/"28-QGE1T5zlDpTdS+Wx7VZucGN1toU"
< date: Sun, 23 May 2021 16:59:27 GMT
< x-envoy-upstream-service-time: 1
< x-3scale-ratelimit-limit: 10
< x-3scale-ratelimit-remaining: 8
< x-3scale-ratelimit-reset: 23/05/2021 16:59:45
< server: envoy
< 
* Connection #0 to host localhost left intact
{"message":"Hello from foo service !!!"}% 

```
You can see the 3 headers related to the rate limiting from the above output.

2. Response after rate limiting.

By default only 10 requests are allowed through the proxy within a time window of 30 seconds. See the logs and right after a rate limit reset, send 11 requests continously to get the rate limit response of 429.

```sh
*   Trying 127.0.0.1:9095...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 9095 (#0)
> GET /foo HTTP/1.1
> Host: localhost:9095
> User-Agent: curl/7.68.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 429 Too Many Requests
< ratelimit-remaining: 0
< ratelimit-limit: 10
< ratelimit-reset: 23/05/2021 17:05:15
< x-3scale-ratelimit-limit: 10
< x-3scale-ratelimit-remaining: 0
< x-3scale-ratelimit-reset: 23/05/2021 17:05:15
< date: Sun, 23 May 2021 17:05:01 GMT
< server: envoy
< content-length: 0
< 
* Connection #0 to host localhost left intact

```

