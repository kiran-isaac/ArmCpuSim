.text
    adcs r7, r1
    adcs r3, r5

    // addimm t1
    adds r1, r5, 7
    adds r7, r2, 2

    // addimm t2
    adds r0, #100
    adds r4, #255

    ands r1, r0
    cmn r2, r2
    mvns r0, r6
    muls r5, r1

    BL 122456
    BL #-100000