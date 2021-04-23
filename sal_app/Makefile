.PHONY: all
all: vmlinux bpf app

.PHONY: bpf
bpf:
	cargo libbpf build
	cargo libbpf gen

.PHONY: app
app:
	cargo build

.PHONY: vmlinux
vmlinux:
	bpftool btf dump file /sys/kernel/btf/vmlinux format c > src/bpf/vmlinux.h

.PHONY: run
run: all
	sudo ./target/debug/sal_app

.PHONY: clean
clean:
	-rm src/bpf/*.rs
	-rm src/bpf/vmlinux.h
	cargo clean
