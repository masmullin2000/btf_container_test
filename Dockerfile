FROM almalinux/almalinux:latest as builder
RUN dnf install zlib-devel elfutils-libelf-devel make libbpf-devel clang -y
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && source $HOME/.cargo/env && cargo install libbpf-cargo
WORKDIR /
COPY sal_app .
RUN source $HOME/.cargo/env && cargo libbpf build && cargo libbpf gen && cargo build --release

FROM busybox:glibc
WORKDIR /root/
COPY --from=builder /lib64/libelf.so.1 /lib/
COPY --from=builder /lib64/libz.so.1 /lib/
COPY --from=builder /lib64/libgcc_s.so.1 /lib/
COPY --from=builder /lib64/librt.so.1 /lib/
COPY --from=builder /lib64/libdl.so.2 /lib/

COPY --from=builder /target/release/sal_app /root/my-rkt

RUN ["ls", "-l", "/root/"]
CMD ["/root/my-rkt"]