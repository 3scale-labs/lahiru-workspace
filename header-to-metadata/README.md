## Header to metadata POC

### About

Currently setting up dynamic metadata is not support in proxy-wasm-rust-sdk. ([#81](https://github.com/proxy-wasm/proxy-wasm-rust-sdk/issues/81)). As a workaround headers can be added from the request path and then those headers can be converted to dynamic metadata using envoy [header to metadata filter](https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/http/header_to_metadata/v3/header_to_metadata.proto.html).

If we want to use some kind of metadata just in the request path only, then we can get those information from headers. But if we need some kind of metadata in the response path , then request headers are not available. In that case we have to use this approach to set dynamic metadata.

In this POC envoy filter chain is as follows. 

DOWNSTREAM -> [add-header-filter] -> [header-to-metadata filter] -> [metadata-logging filter] -> [envoy router filter] -> UPSTREAM

* add-header-filter will add the following 2 headers to the request headers. (key, value)
    * "x-3scale-service-key", "123456789"
    * "x-3scale-application-key", "987654321"

* header-to-metadata filter will add those 2 headers as dynamic metadata.

```sh
 - name: envoy.filters.http.header_to_metadata
   typed_config:
    "@type": type.googleapis.com/envoy.extensions.filters.http.header_to_metadata.v3.Config
    request_rules:
    - header: x-3scale-service-key
        on_header_present:
        metadata_namespace: 3scale
        key: service_key
        type: STRING
        on_header_missing:
        metadata_namespace: 3scale
        key: service_key
        value: '123456'
        type: STRING
        remove: false
    - header: x-3scale-application-key
        on_header_present:
        metadata_namespace: 3scale
        key: application_key
        type: STRING
        on_header_missing:
        metadata_namespace: 3scale
        key: application_key
        value: '123456'
        type: STRING
        remove: false
```

* metadata-logging filter

This is just a simple filter that logs the dynamic metadata added from the previous header to metdata filter.

### Building the filter

Note: For building the filter as a WASM module, the following prerequisites are there. If you want to just test, then there's no need to build the filter. I have already built the filter and included it in the build folder.

* rust (nightly toolchain and the support for WASM compilation target)
    * rustup toolchain install nightly
    * rustup target add wasm32-unknown-unknown
* cargo 
* make

Go to the `header-to-metadata` root and run the following command to build the filter.

```sh
make build
```

The generated WASM module will be in the build folder.

### Running the setup to test the functionality

To run the setup using `docker-compose` , execute the following command from the project root folder. As mentioned in the previous section, you don't need to build the filter to run the setup since an already built filter is included.

```sh
docker-compose -f header-to-metadata/docker-compose.yaml up --build
```

### Testing the dynamic metadata functionality

In order to test for the functionality of adding dynamic metadata , send a request and see the logs.

```sh
curl localhost:9095/foo -v
```

In the logs, the following two lines will be there to indicate that dynamic metadata has been added successfully.

```sh
envoy_1            | [2021-05-23 17:19:53.963][36][info][wasm] [source/extensions/common/wasm/context.cc:1218] wasm log htm htm_root htm_vm: #2 -> x-3scale-service-key: 123456789
envoy_1            | [2021-05-23 17:19:53.963][36][info][wasm] [source/extensions/common/wasm/context.cc:1218] wasm log htm htm_root htm_vm: #2 -> x-3scale-application-key: 987654321
```