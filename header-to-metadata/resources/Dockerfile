FROM envoyproxy/envoy:v1.18.3

# Copy envoy.yaml & wasm module
COPY ./resources/envoy.yaml /etc/envoy/envoy.yaml
COPY ./build/add_header_filter.wasm /usr/local/bin/add_header_filter.wasm
COPY ./build/metadata_filter.wasm /usr/local/bin/metadata_filter.wasm

# Setup permission
RUN chmod go+r /etc/envoy/envoy.yaml /usr/local/bin/add_header_filter.wasm /usr/local/bin/metadata_filter.wasm

CMD /usr/local/bin/envoy -c /etc/envoy/envoy.yaml