	.text
	.syntax unified
	.eabi_attribute	67, "2.09"	@ Tag_conformance
	.cpu	cortex-m0
	.eabi_attribute	6, 12	@ Tag_CPU_arch
	.eabi_attribute	7, 77	@ Tag_CPU_arch_profile
	.eabi_attribute	8, 0	@ Tag_ARM_ISA_use
	.eabi_attribute	9, 1	@ Tag_THUMB_ISA_use
	.eabi_attribute	34, 0	@ Tag_CPU_unaligned_access
	.eabi_attribute	17, 1	@ Tag_ABI_PCS_GOT_use
	.eabi_attribute	20, 1	@ Tag_ABI_FP_denormal
	.eabi_attribute	21, 0	@ Tag_ABI_FP_exceptions
	.eabi_attribute	23, 3	@ Tag_ABI_FP_number_model
	.eabi_attribute	24, 1	@ Tag_ABI_align_needed
	.eabi_attribute	25, 1	@ Tag_ABI_align_preserved
	.eabi_attribute	38, 1	@ Tag_ABI_FP_16bit_format
	.eabi_attribute	18, 4	@ Tag_ABI_PCS_wchar_t
	.eabi_attribute	26, 2	@ Tag_ABI_enum_size
	.eabi_attribute	14, 0	@ Tag_ABI_PCS_R9_use
	.file	"bubble.c"
	.globl	swap                            @ -- Begin function swap
	.p2align	1
	.type	swap,%function
	.code	16                              @ @swap
	.thumb_func
swap:
	.fnstart
@ %bb.0:
	.pad	#16
	sub	sp, #16
	str	r0, [sp, #12]
	str	r1, [sp, #8]
	str	r2, [sp, #4]
	ldr	r0, [sp, #12]
	ldr	r1, [sp, #8]
	ldrb	r0, [r0, r1]
	mov	r1, sp
	strb	r0, [r1]
	ldr	r1, [sp, #12]
	ldr	r0, [sp, #4]
	ldrb	r0, [r1, r0]
	ldr	r2, [sp, #8]
	strb	r0, [r1, r2]
	ldr	r0, [sp]
	ldr	r1, [sp, #12]
	ldr	r2, [sp, #4]
	strb	r0, [r1, r2]
	add	sp, #16
	bx	lr
.Lfunc_end0:
	.size	swap, .Lfunc_end0-swap
	.cantunwind
	.fnend
                                        @ -- End function
	.globl	bubble_sort                     @ -- Begin function bubble_sort
	.p2align	1
	.type	bubble_sort,%function
	.code	16                              @ @bubble_sort
	.thumb_func
bubble_sort:
	.fnstart
@ %bb.0:
	.save	{r7, lr}
	push	{r7, lr}
	.setfp	r7, sp
	add	r7, sp, #0
	.pad	#16
	sub	sp, #16
	str	r0, [sp, #12]
	str	r1, [sp, #8]
	movs	r0, #0
	str	r0, [sp, #4]
	b	.LBB1_1
.LBB1_1:                                @ =>This Loop Header: Depth=1
                                        @     Child Loop BB1_3 Depth 2
	ldr	r0, [sp, #4]
	ldr	r1, [sp, #8]
	cmp	r0, r1
	bge	.LBB1_10
	b	.LBB1_2
.LBB1_2:                                @   in Loop: Header=BB1_1 Depth=1
	movs	r0, #0
	str	r0, [sp]
	b	.LBB1_3
.LBB1_3:                                @   Parent Loop BB1_1 Depth=1
                                        @ =>  This Inner Loop Header: Depth=2
	ldr	r0, [sp]
	ldr	r2, [sp, #8]
	ldr	r1, [sp, #4]
	mvns	r1, r1
	adds	r1, r1, r2
	cmp	r0, r1
	bge	.LBB1_8
	b	.LBB1_4
.LBB1_4:                                @   in Loop: Header=BB1_3 Depth=2
	ldr	r0, [sp, #12]
	ldr	r2, [sp]
	adds	r1, r0, r2
	ldrb	r0, [r0, r2]
	ldrb	r1, [r1, #1]
	cmp	r0, r1
	ble	.LBB1_6
	b	.LBB1_5
.LBB1_5:                                @   in Loop: Header=BB1_3 Depth=2
	ldr	r0, [sp, #12]
	ldr	r1, [sp]
	adds	r2, r1, #1
	bl	swap
	b	.LBB1_6
.LBB1_6:                                @   in Loop: Header=BB1_3 Depth=2
	b	.LBB1_7
.LBB1_7:                                @   in Loop: Header=BB1_3 Depth=2
	ldr	r0, [sp]
	adds	r0, r0, #1
	str	r0, [sp]
	b	.LBB1_3
.LBB1_8:                                @   in Loop: Header=BB1_1 Depth=1
	b	.LBB1_9
.LBB1_9:                                @   in Loop: Header=BB1_1 Depth=1
	ldr	r0, [sp, #4]
	adds	r0, r0, #1
	str	r0, [sp, #4]
	b	.LBB1_1
.LBB1_10:
	add	sp, #16
	pop	{r7, pc}
.Lfunc_end1:
	.size	bubble_sort, .Lfunc_end1-bubble_sort
	.cantunwind
	.fnend
                                        @ -- End function
	.globl	is_sorted                       @ -- Begin function is_sorted
	.p2align	2
	.type	is_sorted,%function
	.code	16                              @ @is_sorted
	.thumb_func
is_sorted:
	.fnstart
@ %bb.0:
	.save	{r7, lr}
	push	{r7, lr}
	.setfp	r7, sp
	add	r7, sp, #0
	.pad	#16
	sub	sp, #16
	str	r0, [sp, #12]
	str	r1, [sp, #8]
	movs	r0, #0
	str	r0, [sp, #4]
	b	.LBB2_1
.LBB2_1:                                @ =>This Inner Loop Header: Depth=1
	ldr	r0, [sp, #4]
	ldr	r1, [sp, #8]
	subs	r1, r1, #1
	cmp	r0, r1
	bge	.LBB2_6
	b	.LBB2_2
.LBB2_2:                                @   in Loop: Header=BB2_1 Depth=1
	ldr	r0, [sp, #12]
	ldr	r2, [sp, #4]
	adds	r1, r0, r2
	ldrb	r0, [r0, r2]
	ldrb	r1, [r1, #1]
	cmp	r0, r1
	ble	.LBB2_4
	b	.LBB2_3
.LBB2_3:
	ldr	r0, .LCPI2_1
	bl	svc_puts
	movs	r0, #1
	bl	svc_exit
	b	.LBB2_7
.LBB2_4:                                @   in Loop: Header=BB2_1 Depth=1
	b	.LBB2_5
.LBB2_5:                                @   in Loop: Header=BB2_1 Depth=1
	ldr	r0, [sp, #4]
	adds	r0, r0, #1
	str	r0, [sp, #4]
	b	.LBB2_1
.LBB2_6:
	ldr	r0, .LCPI2_0
	bl	svc_puts
	b	.LBB2_7
.LBB2_7:
	add	sp, #16
	pop	{r7, pc}
	.p2align	2
@ %bb.8:
.LCPI2_0:
	.long	.L.str.1
.LCPI2_1:
	.long	.L.str
.Lfunc_end2:
	.size	is_sorted, .Lfunc_end2-is_sorted
	.cantunwind
	.fnend
                                        @ -- End function
	.p2align	1                               @ -- Begin function svc_puts
	.type	svc_puts,%function
	.code	16                              @ @svc_puts
	.thumb_func
svc_puts:
	.fnstart
@ %bb.0:
	.pad	#4
	sub	sp, #4
	str	r0, [sp]
	@APP
	svc	#1
	@NO_APP
	add	sp, #4
	bx	lr
.Lfunc_end3:
	.size	svc_puts, .Lfunc_end3-svc_puts
	.cantunwind
	.fnend
                                        @ -- End function
	.p2align	1                               @ -- Begin function svc_exit
	.type	svc_exit,%function
	.code	16                              @ @svc_exit
	.thumb_func
svc_exit:
	.fnstart
@ %bb.0:
	.pad	#4
	sub	sp, #4
	str	r0, [sp]
	@APP
	svc	#0
	@NO_APP
	add	sp, #4
	bx	lr
.Lfunc_end4:
	.size	svc_exit, .Lfunc_end4-svc_exit
	.cantunwind
	.fnend
                                        @ -- End function
	.globl	main                            @ -- Begin function main
	.p2align	2
	.type	main,%function
	.code	16                              @ @main
	.thumb_func
main:
	.fnstart
@ %bb.0:
	.save	{r7, lr}
	push	{r7, lr}
	.setfp	r7, sp
	add	r7, sp, #0
	.pad	#24
	sub	sp, #24
	ldr	r0, .LCPI5_0
	str	r0, [sp, #20]
	ldr	r0, .LCPI5_1
	str	r0, [sp, #16]
	ldr	r0, .LCPI5_2
	str	r0, [sp, #12]
	ldr	r0, .LCPI5_3
	str	r0, [sp, #8]
	movs	r0, #16
	str	r0, [sp, #4]
	ldr	r1, [sp, #4]
	add	r0, sp, #8
	str	r0, [sp]                        @ 4-byte Spill
	bl	bubble_sort
	ldr	r0, [sp]                        @ 4-byte Reload
	ldr	r1, [sp, #4]
	bl	is_sorted
	ldr	r0, [sp]                        @ 4-byte Reload
	ldrb	r0, [r0]
	bl	svc_exit
	movs	r0, #0
	add	sp, #24
	pop	{r7, pc}
	.p2align	2
@ %bb.1:
.LCPI5_0:
	.long	151520769                       @ 0x9080601
.LCPI5_1:
	.long	3372221188                      @ 0xc9000304
.LCPI5_2:
	.long	1533478151                      @ 0x5b670507
.LCPI5_3:
	.long	319881738                       @ 0x1311020a
.Lfunc_end5:
	.size	main, .Lfunc_end5-main
	.cantunwind
	.fnend
                                        @ -- End function
	.type	.L.str,%object                  @ @.str
	.section	.rodata.str1.1,"aMS",%progbits,1
.L.str:
	.asciz	"Array is not sorted\n"
	.size	.L.str, 21

	.type	.L.str.1,%object                @ @.str.1
.L.str.1:
	.asciz	"Array is sorted\n"
	.size	.L.str.1, 17

	.type	.L__const.main.arr,%object      @ @__const.main.arr
	.section	.rodata.cst16,"aM",%progbits,16
.L__const.main.arr:
	.ascii	"\n\002\021\023\007\005g[\004\003\000\311\001\006\b\t"
	.size	.L__const.main.arr, 16

	.ident	"clang version 19.1.5"
	.section	".note.GNU-stack","",%progbits
	.addrsig
	.addrsig_sym swap
	.addrsig_sym bubble_sort
	.addrsig_sym is_sorted
	.addrsig_sym svc_puts
	.addrsig_sym svc_exit
	.eabi_attribute	30, 6	@ Tag_ABI_optimization_goals
