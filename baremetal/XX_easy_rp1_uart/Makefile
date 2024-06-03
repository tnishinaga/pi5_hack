OBJCOPY=cargo objcopy

all: build jtag_kernel.bin

.PHONY: jtag_kernel.bin
build:
	cargo build --release

.PHONY: jtag_kernel.bin

jtag_kernel.bin: build
	$(OBJCOPY) --release --bin jtag_kernel -- -O binary ./target/jtag_kernel.bin