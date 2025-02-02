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
	.globl	assert_is_sorted                @ -- Begin function assert_is_sorted
	.p2align	2
	.type	assert_is_sorted,%function
	.code	16                              @ @assert_is_sorted
	.thumb_func
assert_is_sorted:
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
	.size	assert_is_sorted, .Lfunc_end2-assert_is_sorted
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
	.save	{r4, r6, r7, lr}
	push	{r4, r6, r7, lr}
	.setfp	r7, sp, #8
	add	r7, sp, #8
	.pad	#508
	sub	sp, #508
	.pad	#508
	sub	sp, #508
	ldr	r1, .LCPI5_0
	movs	r0, #125
	lsls	r2, r0, #3
	str	r2, [sp, #4]                    @ 4-byte Spill
	add	r0, sp, #16
	str	r0, [sp, #8]                    @ 4-byte Spill
	bl	__aeabi_memcpy
	ldr	r1, [sp, #4]                    @ 4-byte Reload
                                        @ kill: def $r2 killed $r0
	ldr	r0, [sp, #8]                    @ 4-byte Reload
	str	r1, [sp, #12]
	ldr	r1, [sp, #12]
	bl	bubble_sort
	ldr	r0, [sp, #8]                    @ 4-byte Reload
	ldr	r1, [sp, #12]
	bl	assert_is_sorted
	ldr	r0, [sp, #8]                    @ 4-byte Reload
	ldrb	r0, [r0]
	bl	svc_exit
	movs	r0, #0
	subs	r6, r7, #7
	subs	r6, #1
	mov	sp, r6
	pop	{r4, r6, r7, pc}
	.p2align	2
@ %bb.1:
.LCPI5_0:
	.long	.L__const.main.arr
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
	.section	.rodata,"a",%progbits
.L__const.main.arr:
	.ascii	"gE\316\247h\324`\363\0200\031U\003nLT\321r'.\225\\8Go\271\033\207\3340\341:\260\022\000{4P0l\331\016h\r\273\013\031_\177\265\333{\233\304>\"L\375\201[\333\306\326\317\266\243\364\351Dc#\232\304\004\210W\2330C,\220\002p\261\351\217p.\315\231J\030\227\030\362\362\205\231\372\317\021\341\bi\274Y\006\207\2160b\263B\347\356\331H\\w\362\2360\233\26796Y\201+\245\231\274\311j\341\206\357\274\006g;V\375\205\337\224\362/\356\324a\360>z\363K\367]\371\227\265\f\266\347\r>\276S\217\302\245\215\005\333\361H.K\206\224\341<<\336\263~T\277h\311\031*\2772\025$M\237ze\244\334\255\217\013\013\331\000\247\206\245\352Nug\307\223Z\206\261[\260f]Hv\013Y**\276\364\210\227\025\205\276L;q\216u$\006\277\336\320DK\371\031\372\2117?A\360\247\013\250\340\\{c[7q\276\304\001\371\237\347\022\272\021e\341\216\214\253>\254U\301\200\256\306\335\344xp\003\037\326Vu\373H\244\230\373\252\276\366\315u\254\0167\376?]\276\234\353>]\256\201\022\b\\L\b\t\f\020}e\204\361\035\242Aa\372\321\210\246\365\236lx\336I\036\275\241,-|0\373#u\314\304\310\341\316`\343d\226%\023\262\313Iu\233\224\206\265M\020&\356\356+b\306\036\262\313\3716\023\224\207L\341\330\313d\\\035P\242\35080\272\376\205K\231\204G\341\337\375x\241C\267$\342\270;\200\313\236\247\374\271\025\255r)\006\307\351\350u`\234D\220\022SG\035#|\247\330\301w\223\263\362# S\202\245P!\341\222\204X\3616\004\002N\376\220+\321\261\324\264 \224/\370\0228\264\342\242\333#\3039-\350L\204F\\\031R\016\242\177\201\216\344\250=\346\312CH\203\312\372\315\2561\023&\202\271\221+-\367}\177\227]q~(\352\025 \372\315I0\206\301%\2420\240\306\372\335\361\247\354\230\304#]&\273\230x8\341\346F\3665\022\002\f-\203E)\317\244\315\231>\213g\200\\2\177\003\\\304\342\3738\347&o\013<\327\331\231\030\225\300p\204\272\224\261\371^/\"\335\257\242\334\254DW\275s\257\276\276\351\004NRN\023\232\210\231\214\233\251tCO\340w\210\"'H\233\033\303\026`NG\0257\006\234\364h\020*|\023\2332,c5Y\234\2628\330\251\241\235\221\3512\226\355\030\232\306\207\263\274fT\246\374wP*\347\212\263\f'\026\f\276\262\032m\363\351\227\342\235\242t\344\210EW9)\275\216\235\261\347\214;\313jr\266\311R/h\317l\236\307e\325\203\341^\322\r\"\355\371I'\034c\265\004\336bg\246^\333a_\202\371\304V\223f\320\244\260\215L\223\256\3006\020\224\276>\220%\260\344\002\035e\232\265 e#\243p\336p\013G\376|\225\307\350W\327\247\215\007\264rZqt\306f\216%8*u\031n\024\321\322 R\307\351\270\350h\035\313\230\312\251\265\320cK\352\364z\206\365\266\305\305\210K\226\305W\211\372\370J%\345Q\3473\032SN\267\354*\326\366RH\215\t\344\354\313)Y\353\361\275\244\222g\030\371\305\340\2258\020_\252\346t\222\306\233wX|3\005\r\230\261^\026%\025DH\224\260\210\253HmO`!\f9{\316\211EtO\2133\244$\251\362\034Y\3727M\246\230\357\005WX&\256L{\207\2310\346\245\311\366?!\263P!\211!\202~L\216,\024\274\017\246K\372\001\336\234|\200P\366Wc\2707\t\263\224QL=\244rk\363\n\362\251\253a\263\264H"
	.size	.L__const.main.arr, 1000

	.ident	"clang version 19.1.5"
	.section	".note.GNU-stack","",%progbits
	.addrsig
	.addrsig_sym swap
	.addrsig_sym bubble_sort
	.addrsig_sym assert_is_sorted
	.addrsig_sym svc_puts
	.addrsig_sym svc_exit
	.eabi_attribute	30, 6	@ Tag_ABI_optimization_goals
