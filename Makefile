arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso
target ?= $(arch)-elfahOS
rust := target/$(target)/debug/libelfah_os.a

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso kernel install

all: $(iso)

clean:
	@rm -r target
	@rm -r build

run: $(iso)
	@echo "Starting QEMU. Log from COM1:"
	@qemu-system-x86_64 -cdrom $(iso) -serial stdio

iso: $(iso)

DRIVE := /dev/null

install : $(iso)
	@sudo dd if=$(iso) of=$(DRIVE)

# make .iso
$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles
	@rm -r build/isofiles

# link .bin
$(kernel): kernel $(assembly_object_files) $(linker_script)
	@ld -n --gc-sections -o $(kernel) -T $(linker_script) \
		$(assembly_object_files) $(rust)

kernel:
	@RUST_TARGET_PATH=$(shell pwd) cargo build --target $(target).json

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@