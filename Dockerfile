FROM almalinux/almalinux:latest as builder
RUN dnf install zlib-devel elfutils-libelf-devel make libbpf-devel clang -y
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && source $HOME/.cargo/env && cargo install libbpf-cargo
WORKDIR /
COPY sal_app .
RUN echo $PATH
RUN source $HOME/.cargo/env && cargo libbpf build && cargo libbpf gen && cargo build

FROM almalinux/almalinux:latest
WORKDIR /root/
COPY --from=builder /target/debug/sal_app /root/
RUN ["chmod", "+x", "/root/sal_app"]
RUN ["ls", "-l", "/root/"]
CMD ["/root/sal_app"]