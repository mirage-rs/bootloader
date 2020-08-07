.extern main
.type main, %function

.section .text.crt0, "ax", %progbits
.arm
.global _start
.type _start, %function
_start:
    // Check if it is necessary to relocate.
    ldr r0, =__payload_start__
    adr r1, _start
    cmp r0, r1
    beq start

    // Copy the relocator to upper IRAM.
    adr r2, relocator
    adr r4, relocator_end
    sub r4, r4, r2
    ldr r3, =__relocator__

    copy_relocator:
    ldmia r2!, {r5}
    stmia r3!, {r5}
    subs r4, #4
    bne copy_relocator

    // Use the relocator to copy the payload to the right place.
    ldr r2, =__payload_end__
    sub r2, r2, r0
    ldr r4, =__relocator__
    bx r4

    // Now that the payload is relocated, jump to the actual entrypoint.
    ldr r3, =start
    bx r3

//
// Copies the payload data to the start of the execution stack.
// This is safe, since payload data is aligned to a 16-byte boundary.
//
// Arguments:
//   r0 - The destination address.
//   r1 - The source address.
//   r2 - Amount of bytes to copy.
//
// Destroys r0-r7.
//
.align 4
relocator:
    // Relocate the payload.
    ldmia r1!, {r4-r7}
    stmia r0!, {r4-r7}
    subs r2, #0x10
    bne relocator
relocator_end:

//
// Entrypoint to the actual bootloader code.
//
// Sets sp, destroys r0-r2.
//
start:
    // Set the stack pointer.
    ldr sp, =__stack_top__

    // Jump to Rust entrypoint.
    bl main

    // Halt execution in an infinite loop.
    b .
