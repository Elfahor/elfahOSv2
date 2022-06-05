global start
global multiboot_addr
extern long_mode_start

section .text
bits 32
start:
	; pass multiboot info to rust_main
	mov edi, ebx

	mov esp, stack_top ; setup stack
	; check for everything
	call check_multiboot
	call check_cpuid
	call check_long_mode
	call setup_page_tables
	call enable_paging

	; load the 64bit gdt we have defined in .rodata
	lgdt [gdt64.pointer]
	jmp gdt64.code:long_mode_start
	; print OK, this should not be shown
	mov dword [0xb8000], 0x2f4b2f4f

	hlt
error:
	mov byte [0xb8001], 0x4f
	mov byte [0xb8000], al
	hlt

check_multiboot:
	cmp eax, 0x36d76289
	jne .no_multiboot
	ret
.no_multiboot:
	mov al, "0"
	jmp error

check_cpuid: ; check if the cpuid instruction exists
    ;the ID bit must be flippable
	; copy flags to eax through the stack
	pushfd
	pop eax
	; copy to ecx for comparison later
	mov ecx, eax
	; flip the ID bit (21)
	xor eax, 1 << 21
	; copy back eax to FLAGS
	push eax
	popfd
	; now when copying again the bit should have stayed flipped
	pushfd
	pop eax
	; restore flags before anything
	push ecx
	popfd
	; compare eax and ecx, they should be different
	cmp eax, ecx
	je .no_cpuid
	ret
.no_cpuid:
	mov al, "1"
	jmp error

check_long_mode: ; use cpuid to check for long mode (64bit) support
	; test if cpu info is available
	mov eax, 0x80000000 ; arg for cpuid
	cpuid
	cmp eax, 0x80000001 ; needs to be big enough
	jb .no_long_mode

	; use cpuid to actually check for long mode availability
	mov eax, 0x80000001 ; arg
	cpuid
	test edx, 1 << 29 ; check if LM bit (29) has been set
	jz .no_long_mode
	ret
.no_long_mode:
	mov al, "2"
	jmp error

setup_page_tables:
	; map first PML4 entry to PDP
	mov eax, pdp_table
	or eax, 0b11 ; write present & writable
	mov [pml4_table], eax
	; same but for PDP and PD
	mov eax, pd_table
	or eax, 0b11 ; write present & writable
	mov [pdp_table], eax

	; map each PD entry to a `huge` 2 MiB page
	mov ecx, 0 ; counter
	.map_p2: ; loop
		; map ecx-th PD entry to a page at 2MiB*ecx
		mov eax, 0x200000 ; 2MiB
		mul ecx ; *eax so *2MiB
		or eax, 0b10000011 ; present & writable & huge
		mov [pd_table + ecx * 8], eax ; map ecx-th entry
		; loop 512 times
		inc ecx
		cmp ecx, 512
		jne .map_p2
	ret

enable_paging:
	; load PML4 in cr3 register (special register)
	mov eax, pml4_table
	mov cr3, eax
	; enable Physical Address Extension in cr4
	mov eax, cr4
	or eax, 1 << 5 ; 5th bit
	mov cr4, eax
	; set LM bit in EFER model specific register (msr)
	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 <<8 ; 8th bit
	wrmsr
	; enable paging in cr0
	mov eax, cr0
	or eax, 1 << 31 ; 31th bit
	mov cr0, eax

	ret

section .bss
align 4096
pml4_table:
    resb 4096
pdp_table:
    resb 4096
pd_table:
    resb 4096
stack_bottom:
	resb 4096 * 4
stack_top:

section .rodata ; readonly data
gdt64: ; use Global Descriptor Table to setup 64bit mode
	dq 0
	; set present, descriptor_type, executable and 64bit flags so that:
	; we are a code segment,
	; we are a code or data segment (redundant),
	; we have a valid selector,
	; we wanna be in 64bit (finally!)

	.code: equ $ - gdt64
	dq (1<<43) | (1<<44) | (1<<47) | (1<<53)
.pointer:
	dw $ - gdt64 - 1 ; specify gdt size
	dq gdt64 ; specify gdt address
