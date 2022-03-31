.PHONY: build
build:
	cargo bootimage

.PHONY: run
run:
	qemu-system-x86_64 \
		-drive format=raw,file=./target/x86_64-acceolar_kernel/debug/bootimage-acceolar.bin \
		-nographic

.PHONY: clean
clean:
	cargo clean
