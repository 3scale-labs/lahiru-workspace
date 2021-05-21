FROM envoyproxy/envoy:v1.18.3

# Copy envoy.yaml & wasm module
COPY ./resources/envoy.yaml /etc/envoy/envoy.yaml
COPY ./build/envoy_rate_limit_header.wasm /usr/local/bin/envoy_rate_limit_header.wasm

# Setup permission
RUN chmod go+r /etc/envoy/envoy.yaml /usr/local/bin/envoy_rate_limit_header.wasm /usr/local/bin/envoy_rate_limit_header.wasm

CMD /usr/local/bin/envoy -c /etc/envoy/envoy.yaml