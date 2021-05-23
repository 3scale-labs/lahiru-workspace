<!-- PROJECT LOGO -->
<br />
<p align="center">
  <h3 align="center">3scale envoy proxy authorization cache</h3>

  <p align="center">
   🛰 POCs to demonstrate and verify features required for implementing an in-proxy authorization cache🛰️
    <br />
    <a href="#"><strong>Explore the docs »</strong></a>
    <br />
  </p>
</p>

<!-- ABOUT THE PROJECT -->
## About
Several POCs to demonstrate and verify features required for implementing an in-proxy local cache. 

<!-- GETTING STARTED -->
## List of POCs

**Note: Please refer to the individual POC documentation for more information regarding each of the POCs.**

* [Rate limit header POC](envoy-rate-limit-header/README.md)
    * Handling rate limit logic inside a HTTP filter with the help of shared data (cache) and adding rate limit headers from the response path.
* [Set dynamic metadata POC](header-to-metadata/README.md)
    * Currently setting up dynamic metadata is not support in proxy-wasm-rust-sdk. ([#81](https://github.com/proxy-wasm/proxy-wasm-rust-sdk/issues/81)). As a workaround headers can be added from the request path and then those headers can be converted to dynamic metadata using envoy [header to metadata filter](https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/http/header_to_metadata/v3/header_to_metadata.proto.html).
* [In proxy cache implementation using shared data, singleton service and HTTP filter](https://github.com/NomadXD/envoy-authorization-cache-filter)
    * This is done as a part of the GSoC proposal and it uses shared data to store the cache. And it uses singleton service to flush the cache to a remote service and update the local cache using the response of that request. Authorization and rate limiting is performed from the HTTP filter per each HTTP call using the cache stored in the shared data. This POC is in my personal github account and can be accessed via the following link. [envoy-authorization-cache-filter](https://github.com/NomadXD/envoy-authorization-cache-filter)

