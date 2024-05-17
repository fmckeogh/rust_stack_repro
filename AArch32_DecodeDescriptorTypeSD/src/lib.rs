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
pub fn AArch32_DecodeDescriptorTypeSD<T: Tracer>(
    state: &mut State,
    tracer: &T,
    descriptor: u32,
    level: i128,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_28573: bool,
        gs_28548: bool,
        gs_28554: bool,
        return_value: u32,
        gs_28568: bool,
        gs_28564: bool,
        gs_28560: bool,
        descriptor: u32,
        level: i128,
    }
    let fn_state = FunctionState {
        descriptor,
        level,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #1s : i
        let s_0_0: i128 = 1;
        // D s_0_1: read-var level:i
        let s_0_1: i128 = fn_state.level;
        // D s_0_2: cmp-eq s_0_1 s_0_0
        let s_0_2: bool = ((s_0_1) == (s_0_0));
        // N s_0_3: branch s_0_2 b29 b1
        if s_0_2 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#28548 <= s_1_0
        fn_state.gs_28548 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var gs#28548:u8
        let s_2_0: bool = fn_state.gs_28548;
        // N s_2_1: branch s_2_0 b28 b3
        if s_2_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_3_0: const #1s : i
        let s_3_0: i128 = 1;
        // D s_3_1: read-var level:i
        let s_3_1: i128 = fn_state.level;
        // D s_3_2: cmp-eq s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) == (s_3_0));
        // N s_3_3: branch s_3_2 b27 b4
        if s_3_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#28554 <= s_4_0
        fn_state.gs_28554 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_5_0: read-var gs#28554:u8
        let s_5_0: bool = fn_state.gs_28554;
        // N s_5_1: branch s_5_0 b26 b6
        if s_5_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_6_0: const #1s : i
        let s_6_0: i128 = 1;
        // D s_6_1: read-var level:i
        let s_6_1: i128 = fn_state.level;
        // D s_6_2: cmp-eq s_6_1 s_6_0
        let s_6_2: bool = ((s_6_1) == (s_6_0));
        // N s_6_3: branch s_6_2 b25 b7
        if s_6_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#28560 <= s_7_0
        fn_state.gs_28560 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_8_0: read-var gs#28560:u8
        let s_8_0: bool = fn_state.gs_28560;
        // N s_8_1: branch s_8_0 b24 b9
        if s_8_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_9_0: const #2s : i
        let s_9_0: i128 = 2;
        // D s_9_1: read-var level:i
        let s_9_1: i128 = fn_state.level;
        // D s_9_2: cmp-eq s_9_1 s_9_0
        let s_9_2: bool = ((s_9_1) == (s_9_0));
        // N s_9_3: branch s_9_2 b23 b10
        if s_9_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#28564 <= s_10_0
        fn_state.gs_28564 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_11_0: read-var gs#28564:u8
        let s_11_0: bool = fn_state.gs_28564;
        // N s_11_1: branch s_11_0 b22 b12
        if s_11_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_12_0: const #2s : i
        let s_12_0: i128 = 2;
        // D s_12_1: read-var level:i
        let s_12_1: i128 = fn_state.level;
        // D s_12_2: cmp-eq s_12_1 s_12_0
        let s_12_2: bool = ((s_12_1) == (s_12_0));
        // N s_12_3: branch s_12_2 b18 b13
        if s_12_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#28573 <= s_13_0
        fn_state.gs_28573 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_14_0: read-var gs#28573:u8
        let s_14_0: bool = fn_state.gs_28573;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_15_0: const #1u : u32
        let s_15_0: u32 = 1;
        // D s_15_1: write-var return_value <= s_15_0
        fn_state.return_value = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_16_0: read-var return_value:u32
        let s_16_0: u32 = fn_state.return_value;
        // N s_16_1: return s_16_0
        return s_16_0;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_17_0: const #5u : u32
        let s_17_0: u32 = 5;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_18_0: const #0s : i
        let s_18_0: i128 = 0;
        // D s_18_1: read-var descriptor:u32
        let s_18_1: u32 = fn_state.descriptor;
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 32u16);
        // C s_18_3: const #1s : i64
        let s_18_3: i64 = 1;
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // C s_18_5: const #1s : i
        let s_18_5: i128 = 1;
        // C s_18_6: add s_18_5 s_18_4
        let s_18_6: i128 = (s_18_5 + s_18_4);
        // D s_18_7: bit-extract s_18_2 s_18_0 s_18_6
        let s_18_7: Bits = (Bits::new(
            ((s_18_2) >> (s_18_0)).value(),
            u16::try_from(s_18_6).unwrap(),
        ));
        // D s_18_8: cast reint s_18_7 -> u8
        let s_18_8: u8 = (s_18_7.value() as u8);
        // C s_18_9: const #1s : i
        let s_18_9: i128 = 1;
        // D s_18_10: cast zx s_18_8 -> bv
        let s_18_10: Bits = Bits::new(s_18_8 as u128, 2u16);
        // C s_18_11: const #1s : i64
        let s_18_11: i64 = 1;
        // C s_18_12: cast zx s_18_11 -> i
        let s_18_12: i128 = (i128::try_from(s_18_11).unwrap());
        // C s_18_13: const #0s : i
        let s_18_13: i128 = 0;
        // C s_18_14: add s_18_13 s_18_12
        let s_18_14: i128 = (s_18_13 + s_18_12);
        // D s_18_15: bit-extract s_18_10 s_18_9 s_18_14
        let s_18_15: Bits = (Bits::new(
            ((s_18_10) >> (s_18_9)).value(),
            u16::try_from(s_18_14).unwrap(),
        ));
        // D s_18_16: cast reint s_18_15 -> u8
        let s_18_16: bool = ((s_18_15.value()) != 0);
        // D s_18_17: cast zx s_18_16 -> bv
        let s_18_17: Bits = Bits::new(s_18_16 as u128, 1u16);
        // C s_18_18: const #1u : u8
        let s_18_18: bool = true;
        // C s_18_19: cast zx s_18_18 -> bv
        let s_18_19: Bits = Bits::new(s_18_18 as u128, 1u16);
        // D s_18_20: cmp-eq s_18_17 s_18_19
        let s_18_20: bool = ((s_18_17) == (s_18_19));
        // D s_18_21: not s_18_20
        let s_18_21: bool = !s_18_20;
        // N s_18_22: branch s_18_21 b21 b19
        if s_18_21 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#28568 <= s_19_0
        fn_state.gs_28568 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_20_0: read-var gs#28568:u8
        let s_20_0: bool = fn_state.gs_28568;
        // D s_20_1: write-var gs#28573 <= s_20_0
        fn_state.gs_28573 = s_20_0;
        // N s_20_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#28568 <= s_21_0
        fn_state.gs_28568 = s_21_0;
        // N s_21_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_22_0: const #4u : u32
        let s_22_0: u32 = 4;
        // D s_22_1: write-var return_value <= s_22_0
        fn_state.return_value = s_22_0;
        // N s_22_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_23_0: const #0s : i
        let s_23_0: i128 = 0;
        // D s_23_1: read-var descriptor:u32
        let s_23_1: u32 = fn_state.descriptor;
        // D s_23_2: cast zx s_23_1 -> bv
        let s_23_2: Bits = Bits::new(s_23_1 as u128, 32u16);
        // C s_23_3: const #1s : i64
        let s_23_3: i64 = 1;
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #1s : i
        let s_23_5: i128 = 1;
        // C s_23_6: add s_23_5 s_23_4
        let s_23_6: i128 = (s_23_5 + s_23_4);
        // D s_23_7: bit-extract s_23_2 s_23_0 s_23_6
        let s_23_7: Bits = (Bits::new(
            ((s_23_2) >> (s_23_0)).value(),
            u16::try_from(s_23_6).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: u8 = (s_23_7.value() as u8);
        // D s_23_9: cast zx s_23_8 -> bv
        let s_23_9: Bits = Bits::new(s_23_8 as u128, 2u16);
        // C s_23_10: const #1u : u8
        let s_23_10: u8 = 1;
        // C s_23_11: cast zx s_23_10 -> bv
        let s_23_11: Bits = Bits::new(s_23_10 as u128, 2u16);
        // D s_23_12: cmp-eq s_23_9 s_23_11
        let s_23_12: bool = ((s_23_9) == (s_23_11));
        // D s_23_13: write-var gs#28564 <= s_23_12
        fn_state.gs_28564 = s_23_12;
        // N s_23_14: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_24_0: const #2u : u32
        let s_24_0: u32 = 2;
        // D s_24_1: write-var return_value <= s_24_0
        fn_state.return_value = s_24_0;
        // N s_24_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_25_0: const #18s : i
        let s_25_0: i128 = 18;
        // D s_25_1: read-var descriptor:u32
        let s_25_1: u32 = fn_state.descriptor;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 32u16);
        // C s_25_3: const #1u : u64
        let s_25_3: u64 = 1;
        // D s_25_4: bit-extract s_25_2 s_25_0 s_25_3
        let s_25_4: Bits = (Bits::new(
            ((s_25_2) >> (s_25_0)).value(),
            u16::try_from(s_25_3).unwrap(),
        ));
        // D s_25_5: cast reint s_25_4 -> u8
        let s_25_5: bool = ((s_25_4.value()) != 0);
        // C s_25_6: const #0s : i
        let s_25_6: i128 = 0;
        // C s_25_7: const #0u : u64
        let s_25_7: u64 = 0;
        // D s_25_8: cast zx s_25_5 -> u64
        let s_25_8: u64 = (s_25_5 as u64);
        // C s_25_9: const #1u : u64
        let s_25_9: u64 = 1;
        // D s_25_10: and s_25_8 s_25_9
        let s_25_10: u64 = ((s_25_8) & (s_25_9));
        // D s_25_11: cmp-eq s_25_10 s_25_9
        let s_25_11: bool = ((s_25_10) == (s_25_9));
        // D s_25_12: lsl s_25_8 s_25_6
        let s_25_12: u64 = s_25_8 << s_25_6;
        // D s_25_13: or s_25_7 s_25_12
        let s_25_13: u64 = ((s_25_7) | (s_25_12));
        // D s_25_14: cmpl s_25_12
        let s_25_14: u64 = !s_25_12;
        // D s_25_15: and s_25_7 s_25_14
        let s_25_15: u64 = ((s_25_7) & (s_25_14));
        // D s_25_16: select s_25_11 s_25_13 s_25_15
        let s_25_16: u64 = if s_25_11 { s_25_13 } else { s_25_15 };
        // D s_25_17: cast trunc s_25_16 -> u8
        let s_25_17: bool = ((s_25_16) != 0);
        // C s_25_18: const #1s : i
        let s_25_18: i128 = 1;
        // D s_25_19: read-var descriptor:u32
        let s_25_19: u32 = fn_state.descriptor;
        // D s_25_20: cast zx s_25_19 -> bv
        let s_25_20: Bits = Bits::new(s_25_19 as u128, 32u16);
        // C s_25_21: const #1u : u64
        let s_25_21: u64 = 1;
        // D s_25_22: bit-extract s_25_20 s_25_18 s_25_21
        let s_25_22: Bits = (Bits::new(
            ((s_25_20) >> (s_25_18)).value(),
            u16::try_from(s_25_21).unwrap(),
        ));
        // D s_25_23: cast reint s_25_22 -> u8
        let s_25_23: bool = ((s_25_22.value()) != 0);
        // C s_25_24: const #0s : i
        let s_25_24: i128 = 0;
        // C s_25_25: const #0u : u64
        let s_25_25: u64 = 0;
        // D s_25_26: cast zx s_25_23 -> u64
        let s_25_26: u64 = (s_25_23 as u64);
        // C s_25_27: const #1u : u64
        let s_25_27: u64 = 1;
        // D s_25_28: and s_25_26 s_25_27
        let s_25_28: u64 = ((s_25_26) & (s_25_27));
        // D s_25_29: cmp-eq s_25_28 s_25_27
        let s_25_29: bool = ((s_25_28) == (s_25_27));
        // D s_25_30: lsl s_25_26 s_25_24
        let s_25_30: u64 = s_25_26 << s_25_24;
        // D s_25_31: or s_25_25 s_25_30
        let s_25_31: u64 = ((s_25_25) | (s_25_30));
        // D s_25_32: cmpl s_25_30
        let s_25_32: u64 = !s_25_30;
        // D s_25_33: and s_25_25 s_25_32
        let s_25_33: u64 = ((s_25_25) & (s_25_32));
        // D s_25_34: select s_25_29 s_25_31 s_25_33
        let s_25_34: u64 = if s_25_29 { s_25_31 } else { s_25_33 };
        // D s_25_35: cast trunc s_25_34 -> u8
        let s_25_35: bool = ((s_25_34) != 0);
        // D s_25_36: cast zx s_25_17 -> bv
        let s_25_36: Bits = Bits::new(s_25_17 as u128, 1u16);
        // D s_25_37: cast zx s_25_35 -> bv
        let s_25_37: Bits = Bits::new(s_25_35 as u128, 1u16);
        // D s_25_38: cast reint s_25_36 -> u128
        let s_25_38: u128 = (s_25_36.value() as u128);
        // D s_25_39: size-of s_25_36
        let s_25_39: u16 = s_25_36.length();
        // D s_25_40: cast reint s_25_37 -> u128
        let s_25_40: u128 = (s_25_37.value() as u128);
        // D s_25_41: size-of s_25_37
        let s_25_41: u16 = s_25_37.length();
        // D s_25_42: lsl s_25_38 s_25_41
        let s_25_42: u128 = s_25_38 << s_25_41;
        // D s_25_43: or s_25_42 s_25_40
        let s_25_43: u128 = ((s_25_42) | (s_25_40));
        // D s_25_44: add s_25_39 s_25_41
        let s_25_44: u16 = (s_25_39 + s_25_41);
        // D s_25_45: create-bits s_25_43 s_25_44
        let s_25_45: Bits = Bits::new(s_25_43, s_25_44);
        // D s_25_46: cast reint s_25_45 -> u8
        let s_25_46: u8 = (s_25_45.value() as u8);
        // D s_25_47: cast zx s_25_46 -> bv
        let s_25_47: Bits = Bits::new(s_25_46 as u128, 2u16);
        // C s_25_48: const #3u : u8
        let s_25_48: u8 = 3;
        // C s_25_49: cast zx s_25_48 -> bv
        let s_25_49: Bits = Bits::new(s_25_48 as u128, 2u16);
        // D s_25_50: cmp-eq s_25_47 s_25_49
        let s_25_50: bool = ((s_25_47) == (s_25_49));
        // D s_25_51: write-var gs#28560 <= s_25_50
        fn_state.gs_28560 = s_25_50;
        // N s_25_52: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_26_0: const #3u : u32
        let s_26_0: u32 = 3;
        // D s_26_1: write-var return_value <= s_26_0
        fn_state.return_value = s_26_0;
        // N s_26_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_27_0: const #18s : i
        let s_27_0: i128 = 18;
        // D s_27_1: read-var descriptor:u32
        let s_27_1: u32 = fn_state.descriptor;
        // D s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 32u16);
        // C s_27_3: const #1u : u64
        let s_27_3: u64 = 1;
        // D s_27_4: bit-extract s_27_2 s_27_0 s_27_3
        let s_27_4: Bits = (Bits::new(
            ((s_27_2) >> (s_27_0)).value(),
            u16::try_from(s_27_3).unwrap(),
        ));
        // D s_27_5: cast reint s_27_4 -> u8
        let s_27_5: bool = ((s_27_4.value()) != 0);
        // C s_27_6: const #0s : i
        let s_27_6: i128 = 0;
        // C s_27_7: const #0u : u64
        let s_27_7: u64 = 0;
        // D s_27_8: cast zx s_27_5 -> u64
        let s_27_8: u64 = (s_27_5 as u64);
        // C s_27_9: const #1u : u64
        let s_27_9: u64 = 1;
        // D s_27_10: and s_27_8 s_27_9
        let s_27_10: u64 = ((s_27_8) & (s_27_9));
        // D s_27_11: cmp-eq s_27_10 s_27_9
        let s_27_11: bool = ((s_27_10) == (s_27_9));
        // D s_27_12: lsl s_27_8 s_27_6
        let s_27_12: u64 = s_27_8 << s_27_6;
        // D s_27_13: or s_27_7 s_27_12
        let s_27_13: u64 = ((s_27_7) | (s_27_12));
        // D s_27_14: cmpl s_27_12
        let s_27_14: u64 = !s_27_12;
        // D s_27_15: and s_27_7 s_27_14
        let s_27_15: u64 = ((s_27_7) & (s_27_14));
        // D s_27_16: select s_27_11 s_27_13 s_27_15
        let s_27_16: u64 = if s_27_11 { s_27_13 } else { s_27_15 };
        // D s_27_17: cast trunc s_27_16 -> u8
        let s_27_17: bool = ((s_27_16) != 0);
        // C s_27_18: const #1s : i
        let s_27_18: i128 = 1;
        // D s_27_19: read-var descriptor:u32
        let s_27_19: u32 = fn_state.descriptor;
        // D s_27_20: cast zx s_27_19 -> bv
        let s_27_20: Bits = Bits::new(s_27_19 as u128, 32u16);
        // C s_27_21: const #1u : u64
        let s_27_21: u64 = 1;
        // D s_27_22: bit-extract s_27_20 s_27_18 s_27_21
        let s_27_22: Bits = (Bits::new(
            ((s_27_20) >> (s_27_18)).value(),
            u16::try_from(s_27_21).unwrap(),
        ));
        // D s_27_23: cast reint s_27_22 -> u8
        let s_27_23: bool = ((s_27_22.value()) != 0);
        // C s_27_24: const #0s : i
        let s_27_24: i128 = 0;
        // C s_27_25: const #0u : u64
        let s_27_25: u64 = 0;
        // D s_27_26: cast zx s_27_23 -> u64
        let s_27_26: u64 = (s_27_23 as u64);
        // C s_27_27: const #1u : u64
        let s_27_27: u64 = 1;
        // D s_27_28: and s_27_26 s_27_27
        let s_27_28: u64 = ((s_27_26) & (s_27_27));
        // D s_27_29: cmp-eq s_27_28 s_27_27
        let s_27_29: bool = ((s_27_28) == (s_27_27));
        // D s_27_30: lsl s_27_26 s_27_24
        let s_27_30: u64 = s_27_26 << s_27_24;
        // D s_27_31: or s_27_25 s_27_30
        let s_27_31: u64 = ((s_27_25) | (s_27_30));
        // D s_27_32: cmpl s_27_30
        let s_27_32: u64 = !s_27_30;
        // D s_27_33: and s_27_25 s_27_32
        let s_27_33: u64 = ((s_27_25) & (s_27_32));
        // D s_27_34: select s_27_29 s_27_31 s_27_33
        let s_27_34: u64 = if s_27_29 { s_27_31 } else { s_27_33 };
        // D s_27_35: cast trunc s_27_34 -> u8
        let s_27_35: bool = ((s_27_34) != 0);
        // D s_27_36: cast zx s_27_17 -> bv
        let s_27_36: Bits = Bits::new(s_27_17 as u128, 1u16);
        // D s_27_37: cast zx s_27_35 -> bv
        let s_27_37: Bits = Bits::new(s_27_35 as u128, 1u16);
        // D s_27_38: cast reint s_27_36 -> u128
        let s_27_38: u128 = (s_27_36.value() as u128);
        // D s_27_39: size-of s_27_36
        let s_27_39: u16 = s_27_36.length();
        // D s_27_40: cast reint s_27_37 -> u128
        let s_27_40: u128 = (s_27_37.value() as u128);
        // D s_27_41: size-of s_27_37
        let s_27_41: u16 = s_27_37.length();
        // D s_27_42: lsl s_27_38 s_27_41
        let s_27_42: u128 = s_27_38 << s_27_41;
        // D s_27_43: or s_27_42 s_27_40
        let s_27_43: u128 = ((s_27_42) | (s_27_40));
        // D s_27_44: add s_27_39 s_27_41
        let s_27_44: u16 = (s_27_39 + s_27_41);
        // D s_27_45: create-bits s_27_43 s_27_44
        let s_27_45: Bits = Bits::new(s_27_43, s_27_44);
        // D s_27_46: cast reint s_27_45 -> u8
        let s_27_46: u8 = (s_27_45.value() as u8);
        // D s_27_47: cast zx s_27_46 -> bv
        let s_27_47: Bits = Bits::new(s_27_46 as u128, 2u16);
        // C s_27_48: const #1u : u8
        let s_27_48: u8 = 1;
        // C s_27_49: cast zx s_27_48 -> bv
        let s_27_49: Bits = Bits::new(s_27_48 as u128, 2u16);
        // D s_27_50: cmp-eq s_27_47 s_27_49
        let s_27_50: bool = ((s_27_47) == (s_27_49));
        // D s_27_51: write-var gs#28554 <= s_27_50
        fn_state.gs_28554 = s_27_50;
        // N s_27_52: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_28_0: const #0u : u32
        let s_28_0: u32 = 0;
        // D s_28_1: write-var return_value <= s_28_0
        fn_state.return_value = s_28_0;
        // N s_28_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_29_0: const #0s : i
        let s_29_0: i128 = 0;
        // D s_29_1: read-var descriptor:u32
        let s_29_1: u32 = fn_state.descriptor;
        // D s_29_2: cast zx s_29_1 -> bv
        let s_29_2: Bits = Bits::new(s_29_1 as u128, 32u16);
        // C s_29_3: const #1s : i64
        let s_29_3: i64 = 1;
        // C s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // C s_29_5: const #1s : i
        let s_29_5: i128 = 1;
        // C s_29_6: add s_29_5 s_29_4
        let s_29_6: i128 = (s_29_5 + s_29_4);
        // D s_29_7: bit-extract s_29_2 s_29_0 s_29_6
        let s_29_7: Bits = (Bits::new(
            ((s_29_2) >> (s_29_0)).value(),
            u16::try_from(s_29_6).unwrap(),
        ));
        // D s_29_8: cast reint s_29_7 -> u8
        let s_29_8: u8 = (s_29_7.value() as u8);
        // D s_29_9: cast zx s_29_8 -> bv
        let s_29_9: Bits = Bits::new(s_29_8 as u128, 2u16);
        // C s_29_10: const #1u : u8
        let s_29_10: u8 = 1;
        // C s_29_11: cast zx s_29_10 -> bv
        let s_29_11: Bits = Bits::new(s_29_10 as u128, 2u16);
        // D s_29_12: cmp-eq s_29_9 s_29_11
        let s_29_12: bool = ((s_29_9) == (s_29_11));
        // D s_29_13: write-var gs#28548 <= s_29_12
        fn_state.gs_28548 = s_29_12;
        // N s_29_14: jump b2
        return block_2(state, tracer, fn_state);
    }
}
