.PHONY: all run clean

OUTPUT := target/i586-polnareff-none/debug/polnareff

all:
	@cargo build

run: all
	@qemu-system-i386 -kernel ${OUTPUT} -debugcon stdio

clean:
	@cargo clean

