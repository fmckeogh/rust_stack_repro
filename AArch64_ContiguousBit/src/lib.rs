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
use common::*;
pub fn AArch64_ContiguousBit<T: Tracer>(
    state: &mut State,
    tracer: &T,
    tgx: u32,
    d128: bool,
    level: i128,
    descriptor: Bits,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_17790: bool,
        gs_17803: bool,
        gs_17800: bool,
        gs_17802: bool,
        return_value: bool,
        gs_17794: bool,
        gs_17792: bool,
        tgx: u32,
        d128: bool,
        level: i128,
        descriptor: Bits,
    }
    let fn_state = FunctionState {
        tgx,
        d128,
        level,
        descriptor,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var d128:u8
        let s_0_0: bool = fn_state.d128;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // C s_0_2: const #1u : u8
        let s_0_2: bool = true;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b18 b1
        if s_0_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var tgx:u32
        let s_1_0: u32 = fn_state.tgx;
        // C s_1_1: const #2u : u32
        let s_1_1: u32 = 2;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b17 b2
        if s_1_2 {
            return block_17(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#17790 <= s_2_0
        fn_state.gs_17790 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#17790:u8
        let s_3_0: bool = fn_state.gs_17790;
        // N s_3_1: branch s_3_0 b16 b4
        if s_3_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var tgx:u32
        let s_4_0: u32 = fn_state.tgx;
        // C s_4_1: const #1u : u32
        let s_4_1: u32 = 1;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // N s_4_3: branch s_4_2 b15 b5
        if s_4_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#17792 <= s_5_0
        fn_state.gs_17792 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#17792:u8
        let s_6_0: bool = fn_state.gs_17792;
        // N s_6_1: branch s_6_0 b14 b7
        if s_6_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var tgx:u32
        let s_7_0: u32 = fn_state.tgx;
        // C s_7_1: const #0u : u32
        let s_7_1: u32 = 0;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b13 b8
        if s_7_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#17794 <= s_8_0
        fn_state.gs_17794 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#17794:u8
        let s_9_0: bool = fn_state.gs_17794;
        // N s_9_1: branch s_9_0 b12 b10
        if s_9_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #52s : i
        let s_10_0: i128 = 52;
        // D s_10_1: read-var descriptor:bv
        let s_10_1: Bits = fn_state.descriptor;
        // C s_10_2: const #1u : u64
        let s_10_2: u64 = 1;
        // D s_10_3: bit-extract s_10_1 s_10_0 s_10_2
        let s_10_3: Bits = (Bits::new(
            ((s_10_1) >> (s_10_0)).value(),
            u16::try_from(s_10_2).unwrap(),
        ));
        // D s_10_4: cast reint s_10_3 -> u8
        let s_10_4: bool = ((s_10_3.value()) != 0);
        // C s_10_5: const #0s : i
        let s_10_5: i128 = 0;
        // C s_10_6: const #0u : u64
        let s_10_6: u64 = 0;
        // D s_10_7: cast zx s_10_4 -> u64
        let s_10_7: u64 = (s_10_4 as u64);
        // C s_10_8: const #1u : u64
        let s_10_8: u64 = 1;
        // D s_10_9: and s_10_7 s_10_8
        let s_10_9: u64 = ((s_10_7) & (s_10_8));
        // D s_10_10: cmp-eq s_10_9 s_10_8
        let s_10_10: bool = ((s_10_9) == (s_10_8));
        // D s_10_11: lsl s_10_7 s_10_5
        let s_10_11: u64 = s_10_7 << s_10_5;
        // D s_10_12: or s_10_6 s_10_11
        let s_10_12: u64 = ((s_10_6) | (s_10_11));
        // D s_10_13: cmpl s_10_11
        let s_10_13: u64 = !s_10_11;
        // D s_10_14: and s_10_6 s_10_13
        let s_10_14: u64 = ((s_10_6) & (s_10_13));
        // D s_10_15: select s_10_10 s_10_12 s_10_14
        let s_10_15: u64 = if s_10_10 { s_10_12 } else { s_10_14 };
        // D s_10_16: cast trunc s_10_15 -> u8
        let s_10_16: bool = ((s_10_15) != 0);
        // D s_10_17: write-var return_value <= s_10_16
        fn_state.return_value = s_10_16;
        // N s_10_18: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var return_value:u8
        let s_11_0: bool = fn_state.return_value;
        // N s_11_1: return s_11_0
        return s_11_0;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var level:i
        let s_13_1: i128 = fn_state.level;
        // D s_13_2: cmp-eq s_13_1 s_13_0
        let s_13_2: bool = ((s_13_1) == (s_13_0));
        // D s_13_3: write-var gs#17794 <= s_13_2
        fn_state.gs_17794 = s_13_2;
        // N s_13_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var return_value <= s_14_0
        fn_state.return_value = s_14_0;
        // N s_14_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #1s : i
        let s_15_0: i128 = 1;
        // D s_15_1: read-var level:i
        let s_15_1: i128 = fn_state.level;
        // D s_15_2: cmp-eq s_15_1 s_15_0
        let s_15_2: bool = ((s_15_1) == (s_15_0));
        // D s_15_3: write-var gs#17792 <= s_15_2
        fn_state.gs_17792 = s_15_2;
        // N s_15_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var return_value <= s_16_0
        fn_state.return_value = s_16_0;
        // N s_16_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #1s : i
        let s_17_0: i128 = 1;
        // D s_17_1: read-var level:i
        let s_17_1: i128 = fn_state.level;
        // D s_17_2: cmp-eq s_17_1 s_17_0
        let s_17_2: bool = ((s_17_1) == (s_17_0));
        // D s_17_3: write-var gs#17790 <= s_17_2
        fn_state.gs_17790 = s_17_2;
        // N s_17_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var tgx:u32
        let s_18_0: u32 = fn_state.tgx;
        // C s_18_1: const #2u : u32
        let s_18_1: u32 = 2;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // N s_18_3: branch s_18_2 b29 b19
        if s_18_2 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#17800 <= s_19_0
        fn_state.gs_17800 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var gs#17800:u8
        let s_20_0: bool = fn_state.gs_17800;
        // N s_20_1: branch s_20_0 b28 b21
        if s_20_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var tgx:u32
        let s_21_0: u32 = fn_state.tgx;
        // C s_21_1: const #0u : u32
        let s_21_1: u32 = 0;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // N s_21_3: branch s_21_2 b27 b22
        if s_21_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#17802 <= s_22_0
        fn_state.gs_17802 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var gs#17802:u8
        let s_23_0: bool = fn_state.gs_17802;
        // D s_23_1: write-var gs#17803 <= s_23_0
        fn_state.gs_17803 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var gs#17803:u8
        let s_24_0: bool = fn_state.gs_17803;
        // N s_24_1: branch s_24_0 b26 b25
        if s_24_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var descriptor:bv
        let s_25_0: Bits = fn_state.descriptor;
        // D s_25_1: size-of s_25_0
        let s_25_1: u16 = s_25_0.length();
        // D s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (i128::try_from(s_25_1).unwrap());
        // C s_25_3: const #111s : i
        let s_25_3: i128 = 111;
        // D s_25_4: cmp-lt s_25_3 s_25_2
        let s_25_4: bool = ((s_25_3) < (s_25_2));
        // N s_25_5: assert s_25_4
        let s_25_5: () = assert!(s_25_4);
        // C s_25_6: const #111s : i
        let s_25_6: i128 = 111;
        // D s_25_7: read-var descriptor:bv
        let s_25_7: Bits = fn_state.descriptor;
        // C s_25_8: const #1u : u64
        let s_25_8: u64 = 1;
        // D s_25_9: bit-extract s_25_7 s_25_6 s_25_8
        let s_25_9: Bits = (Bits::new(
            ((s_25_7) >> (s_25_6)).value(),
            u16::try_from(s_25_8).unwrap(),
        ));
        // D s_25_10: cast reint s_25_9 -> u8
        let s_25_10: bool = ((s_25_9.value()) != 0);
        // C s_25_11: const #0s : i
        let s_25_11: i128 = 0;
        // C s_25_12: const #0u : u64
        let s_25_12: u64 = 0;
        // D s_25_13: cast zx s_25_10 -> u64
        let s_25_13: u64 = (s_25_10 as u64);
        // C s_25_14: const #1u : u64
        let s_25_14: u64 = 1;
        // D s_25_15: and s_25_13 s_25_14
        let s_25_15: u64 = ((s_25_13) & (s_25_14));
        // D s_25_16: cmp-eq s_25_15 s_25_14
        let s_25_16: bool = ((s_25_15) == (s_25_14));
        // D s_25_17: lsl s_25_13 s_25_11
        let s_25_17: u64 = s_25_13 << s_25_11;
        // D s_25_18: or s_25_12 s_25_17
        let s_25_18: u64 = ((s_25_12) | (s_25_17));
        // D s_25_19: cmpl s_25_17
        let s_25_19: u64 = !s_25_17;
        // D s_25_20: and s_25_12 s_25_19
        let s_25_20: u64 = ((s_25_12) & (s_25_19));
        // D s_25_21: select s_25_16 s_25_18 s_25_20
        let s_25_21: u64 = if s_25_16 { s_25_18 } else { s_25_20 };
        // D s_25_22: cast trunc s_25_21 -> u8
        let s_25_22: bool = ((s_25_21) != 0);
        // D s_25_23: write-var return_value <= s_25_22
        fn_state.return_value = s_25_22;
        // N s_25_24: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var return_value <= s_26_0
        fn_state.return_value = s_26_0;
        // N s_26_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #0s : i
        let s_27_0: i128 = 0;
        // D s_27_1: read-var level:i
        let s_27_1: i128 = fn_state.level;
        // D s_27_2: cmp-eq s_27_1 s_27_0
        let s_27_2: bool = ((s_27_1) == (s_27_0));
        // D s_27_3: write-var gs#17802 <= s_27_2
        fn_state.gs_17802 = s_27_2;
        // N s_27_4: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#17803 <= s_28_0
        fn_state.gs_17803 = s_28_0;
        // N s_28_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #1s : i
        let s_29_0: i128 = 1;
        // D s_29_1: read-var level:i
        let s_29_1: i128 = fn_state.level;
        // D s_29_2: cmp-eq s_29_1 s_29_0
        let s_29_2: bool = ((s_29_1) == (s_29_0));
        // D s_29_3: write-var gs#17800 <= s_29_2
        fn_state.gs_17800 = s_29_2;
        // N s_29_4: jump b20
        return block_20(state, tracer, fn_state);
    }
}
