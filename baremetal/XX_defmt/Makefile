OBJCOPY=cargo objcopy

all: jtag_kernel.bin

.PHONY: jtag_kernel.bin

jtag_kernel.bin:
	$(OBJCOPY) --bin jtag_kernel -- -O binary ./target/jtag_kernel.bin