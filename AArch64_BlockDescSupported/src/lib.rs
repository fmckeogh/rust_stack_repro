#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use AArch64_PAMax::*;
use common::*;
pub fn AArch64_BlockDescSupported<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d128: bool,
    ds: bool,
    tgx: u32,
    level: i128,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_17560: bool,
        gs_17563: bool,
        gs_17546: bool,
        gs_17550: bool,
        gs_17553: bool,
        return_value: bool,
        gs_17556: bool,
        gs_17545: bool,
        gs_17554: bool,
        gs_17548: bool,
        gs_17561: bool,
        d128: bool,
        ds: bool,
        tgx: u32,
        level: i128,
    }
    let fn_state = FunctionState {
        d128,
        ds,
        tgx,
        level,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // D s_0_1: read-var tgx:u32
        let s_0_1: u32 = fn_state.tgx;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b15 b1
        if s_0_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0s : i
        let s_1_0: i128 = 0;
        // D s_1_1: read-var level:i
        let s_1_1: i128 = fn_state.level;
        // D s_1_2: cmp-eq s_1_1 s_1_0
        let s_1_2: bool = ((s_1_1) == (s_1_0));
        // N s_1_3: branch s_1_2 b11 b2
        if s_1_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#17546 <= s_2_0
        fn_state.gs_17546 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#17546:u8
        let s_3_0: bool = fn_state.gs_17546;
        // N s_3_1: branch s_3_0 b10 b4
        if s_3_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #1s : i
        let s_4_0: i128 = 1;
        // D s_4_1: read-var level:i
        let s_4_1: i128 = fn_state.level;
        // D s_4_2: cmp-eq s_4_1 s_4_0
        let s_4_2: bool = ((s_4_1) == (s_4_0));
        // D s_4_3: write-var gs#17548 <= s_4_2
        fn_state.gs_17548 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#17548:u8
        let s_5_0: bool = fn_state.gs_17548;
        // N s_5_1: branch s_5_0 b9 b6
        if s_5_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #2s : i
        let s_6_0: i128 = 2;
        // D s_6_1: read-var level:i
        let s_6_1: i128 = fn_state.level;
        // D s_6_2: cmp-eq s_6_1 s_6_0
        let s_6_2: bool = ((s_6_1) == (s_6_0));
        // D s_6_3: write-var gs#17550 <= s_6_2
        fn_state.gs_17550 = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#17550:u8
        let s_7_0: bool = fn_state.gs_17550;
        // D s_7_1: write-var return_value <= s_7_0
        fn_state.return_value = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var return_value:u8
        let s_8_0: bool = fn_state.return_value;
        // N s_8_1: return s_8_0
        return s_8_0;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#17550 <= s_9_0
        fn_state.gs_17550 = s_9_0;
        // N s_9_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#17548 <= s_10_0
        fn_state.gs_17548 = s_10_0;
        // N s_10_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var ds:u8
        let s_11_0: bool = fn_state.ds;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b14 b12
        if s_11_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var d128:u8
        let s_12_0: bool = fn_state.d128;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#17545 <= s_12_4
        fn_state.gs_17545 = s_12_4;
        // N s_12_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var gs#17545:u8
        let s_13_0: bool = fn_state.gs_17545;
        // D s_13_1: write-var gs#17546 <= s_13_0
        fn_state.gs_17546 = s_13_0;
        // N s_13_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#17545 <= s_14_0
        fn_state.gs_17545 = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #1u : u32
        let s_15_0: u32 = 1;
        // D s_15_1: read-var tgx:u32
        let s_15_1: u32 = fn_state.tgx;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // N s_15_4: branch s_15_3 b26 b16
        if s_15_3 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #1s : i
        let s_16_0: i128 = 1;
        // D s_16_1: read-var level:i
        let s_16_1: i128 = fn_state.level;
        // D s_16_2: cmp-eq s_16_1 s_16_0
        let s_16_2: bool = ((s_16_1) == (s_16_0));
        // N s_16_3: branch s_16_2 b22 b17
        if s_16_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#17554 <= s_17_0
        fn_state.gs_17554 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#17554:u8
        let s_18_0: bool = fn_state.gs_17554;
        // N s_18_1: branch s_18_0 b21 b19
        if s_18_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #2s : i
        let s_19_0: i128 = 2;
        // D s_19_1: read-var level:i
        let s_19_1: i128 = fn_state.level;
        // D s_19_2: cmp-eq s_19_1 s_19_0
        let s_19_2: bool = ((s_19_1) == (s_19_0));
        // D s_19_3: write-var gs#17556 <= s_19_2
        fn_state.gs_17556 = s_19_2;
        // N s_19_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var gs#17556:u8
        let s_20_0: bool = fn_state.gs_17556;
        // D s_20_1: write-var return_value <= s_20_0
        fn_state.return_value = s_20_0;
        // N s_20_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#17556 <= s_21_0
        fn_state.gs_17556 = s_21_0;
        // N s_21_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var ds:u8
        let s_22_0: bool = fn_state.ds;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #1u : u8
        let s_22_2: bool = true;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // N s_22_5: branch s_22_4 b25 b23
        if s_22_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var d128:u8
        let s_23_0: bool = fn_state.d128;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // C s_23_2: const #1u : u8
        let s_23_2: bool = true;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: write-var gs#17553 <= s_23_4
        fn_state.gs_17553 = s_23_4;
        // N s_23_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var gs#17553:u8
        let s_24_0: bool = fn_state.gs_17553;
        // D s_24_1: write-var gs#17554 <= s_24_0
        fn_state.gs_17554 = s_24_0;
        // N s_24_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#17553 <= s_25_0
        fn_state.gs_17553 = s_25_0;
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #2u : u32
        let s_26_0: u32 = 2;
        // D s_26_1: read-var tgx:u32
        let s_26_1: u32 = fn_state.tgx;
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // D s_26_3: not s_26_2
        let s_26_3: bool = !s_26_2;
        // N s_26_4: branch s_26_3 b37 b27
        if s_26_3 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #1s : i
        let s_27_0: i128 = 1;
        // D s_27_1: read-var level:i
        let s_27_1: i128 = fn_state.level;
        // D s_27_2: cmp-eq s_27_1 s_27_0
        let s_27_2: bool = ((s_27_1) == (s_27_0));
        // N s_27_3: branch s_27_2 b33 b28
        if s_27_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#17561 <= s_28_0
        fn_state.gs_17561 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_29_0: read-var gs#17561:u8
        let s_29_0: bool = fn_state.gs_17561;
        // N s_29_1: branch s_29_0 b32 b30
        if s_29_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_30_0: const #2s : i
        let s_30_0: i128 = 2;
        // D s_30_1: read-var level:i
        let s_30_1: i128 = fn_state.level;
        // D s_30_2: cmp-eq s_30_1 s_30_0
        let s_30_2: bool = ((s_30_1) == (s_30_0));
        // D s_30_3: write-var gs#17563 <= s_30_2
        fn_state.gs_17563 = s_30_2;
        // N s_30_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_31_0: read-var gs#17563:u8
        let s_31_0: bool = fn_state.gs_17563;
        // D s_31_1: write-var return_value <= s_31_0
        fn_state.return_value = s_31_0;
        // N s_31_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#17563 <= s_32_0
        fn_state.gs_17563 = s_32_0;
        // N s_32_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_33_0: read-var d128:u8
        let s_33_0: bool = fn_state.d128;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // N s_33_5: branch s_33_4 b36 b34
        if s_33_4 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call AArch64_PAMax(s_34_0)
        let s_34_1: i64 = AArch64_PAMax(state, tracer, s_34_0);
        // C s_34_2: const #52s : i
        let s_34_2: i128 = 52;
        // S s_34_3: cast zx s_34_1 -> i
        let s_34_3: i128 = (i128::try_from(s_34_1).unwrap());
        // S s_34_4: cmp-ge s_34_3 s_34_2
        let s_34_4: bool = ((s_34_3) >= (s_34_2));
        // D s_34_5: write-var gs#17560 <= s_34_4
        fn_state.gs_17560 = s_34_4;
        // N s_34_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_35_0: read-var gs#17560:u8
        let s_35_0: bool = fn_state.gs_17560;
        // D s_35_1: write-var gs#17561 <= s_35_0
        fn_state.gs_17561 = s_35_0;
        // N s_35_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#17560 <= s_36_0
        fn_state.gs_17560 = s_36_0;
        // N s_36_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var return_value <= s_37_0
        fn_state.return_value = s_37_0;
        // N s_37_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
