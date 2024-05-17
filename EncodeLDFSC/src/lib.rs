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
use Have56BitPAExt::*;
use HaveMTE2Ext::*;
use Have52BitIPAAndPASpaceExt::*;
use integer_subrange::*;
use UsingAArch32::*;
use HaveRASExt::*;
use Unreachable::*;
use common::*;
pub fn EncodeLDFSC<T: Tracer>(
    state: &mut State,
    tracer: &T,
    statuscode: u32,
    level: i128,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        gs_8676: bool,
        gs_8665: bool,
        gs_8688: bool,
        gs_8690: bool,
        gs_8701: bool,
        gs_8667: bool,
        gs_8718: bool,
        gs_8655: bool,
        gs_8656: bool,
        gs_8678: bool,
        gs_8654: bool,
        gs_8720: bool,
        gs_8689: bool,
        gs_8645: bool,
        return_value: u8,
        gs_8702: bool,
        gs_8643: bool,
        gs_8719: bool,
        gs_8677: bool,
        gs_8700: bool,
        gs_8666: bool,
        result: u8,
        gs_8644: bool,
        statuscode: u32,
        level: i128,
    }
    let fn_state = FunctionState {
        statuscode,
        level,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #2s : i
        let s_0_0: i128 = 2;
        // C s_0_1: neg s_0_0
        let s_0_1: i128 = -s_0_0;
        // C s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // C s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_4: read-var level:i
        let s_0_4: i128 = fn_state.level;
        // D s_0_5: cmp-eq s_0_4 s_0_3
        let s_0_5: bool = ((s_0_4) == (s_0_3));
        // N s_0_6: branch s_0_5 b118 b1
        if s_0_5 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #1s : i
        let s_1_0: i128 = 1;
        // C s_1_1: neg s_1_0
        let s_1_1: i128 = -s_1_0;
        // C s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // C s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var level:i
        let s_1_4: i128 = fn_state.level;
        // D s_1_5: cmp-eq s_1_4 s_1_3
        let s_1_5: bool = ((s_1_4) == (s_1_3));
        // N s_1_6: branch s_1_5 b106 b2
        if s_1_5 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_2_0: const #7u : u32
        let s_2_0: u32 = 7;
        // D s_2_1: read-var statuscode:u32
        let s_2_1: u32 = fn_state.statuscode;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b15 b3
        if s_2_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_3_0: const #1s : i
        let s_3_0: i128 = 1;
        // C s_3_1: const #0s : i
        let s_3_1: i128 = 0;
        // D s_3_2: read-var level:i
        let s_3_2: i128 = fn_state.level;
        // D s_3_3: call integer_subrange(s_3_2, s_3_0, s_3_1)
        let s_3_3: Bits = integer_subrange(state, tracer, s_3_2, s_3_0, s_3_1);
        // D s_3_4: cast reint s_3_3 -> u8
        let s_3_4: u8 = (s_3_3.value() as u8);
        // C s_3_5: const #0u : u8
        let s_3_5: u8 = 0;
        // C s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 4u16);
        // D s_3_7: cast zx s_3_4 -> bv
        let s_3_7: Bits = Bits::new(s_3_4 as u128, 2u16);
        // C s_3_8: cast reint s_3_6 -> u128
        let s_3_8: u128 = (s_3_6.value() as u128);
        // D s_3_9: size-of s_3_6
        let s_3_9: u16 = s_3_6.length();
        // D s_3_10: cast reint s_3_7 -> u128
        let s_3_10: u128 = (s_3_7.value() as u128);
        // D s_3_11: size-of s_3_7
        let s_3_11: u16 = s_3_7.length();
        // D s_3_12: lsl s_3_8 s_3_11
        let s_3_12: u128 = s_3_8 << s_3_11;
        // D s_3_13: or s_3_12 s_3_10
        let s_3_13: u128 = ((s_3_12) | (s_3_10));
        // D s_3_14: add s_3_9 s_3_11
        let s_3_14: u16 = (s_3_9 + s_3_11);
        // D s_3_15: create-bits s_3_13 s_3_14
        let s_3_15: Bits = Bits::new(s_3_13, s_3_14);
        // D s_3_16: cast reint s_3_15 -> u8
        let s_3_16: u8 = (s_3_15.value() as u8);
        // D s_3_17: write-var result <= s_3_16
        fn_state.result = s_3_16;
        // C s_3_18: const #0s : i
        let s_3_18: i128 = 0;
        // D s_3_19: read-var level:i
        let s_3_19: i128 = fn_state.level;
        // D s_3_20: cmp-eq s_3_19 s_3_18
        let s_3_20: bool = ((s_3_19) == (s_3_18));
        // N s_3_21: branch s_3_20 b14 b4
        if s_3_20 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_4_0: const #1s : i
        let s_4_0: i128 = 1;
        // D s_4_1: read-var level:i
        let s_4_1: i128 = fn_state.level;
        // D s_4_2: cmp-eq s_4_1 s_4_0
        let s_4_2: bool = ((s_4_1) == (s_4_0));
        // N s_4_3: branch s_4_2 b13 b5
        if s_4_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var level:i
        let s_5_1: i128 = fn_state.level;
        // D s_5_2: cmp-eq s_5_1 s_5_0
        let s_5_2: bool = ((s_5_1) == (s_5_0));
        // N s_5_3: branch s_5_2 b12 b6
        if s_5_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_6_0: const #3s : i
        let s_6_0: i128 = 3;
        // D s_6_1: read-var level:i
        let s_6_1: i128 = fn_state.level;
        // D s_6_2: cmp-eq s_6_1 s_6_0
        let s_6_2: bool = ((s_6_1) == (s_6_0));
        // D s_6_3: write-var gs#8643 <= s_6_2
        fn_state.gs_8643 = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_7_0: read-var gs#8643:u8
        let s_7_0: bool = fn_state.gs_8643;
        // D s_7_1: write-var gs#8644 <= s_7_0
        fn_state.gs_8644 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_8_0: read-var gs#8644:u8
        let s_8_0: bool = fn_state.gs_8644;
        // D s_8_1: write-var gs#8645 <= s_8_0
        fn_state.gs_8645 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_9_0: read-var gs#8645:u8
        let s_9_0: bool = fn_state.gs_8645;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_10_0: read-var result:u8
        let s_10_0: u8 = fn_state.result;
        // D s_10_1: write-var return_value <= s_10_0
        fn_state.return_value = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_11_0: read-var return_value:u8
        let s_11_0: u8 = fn_state.return_value;
        // N s_11_1: return s_11_0
        return s_11_0;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#8643 <= s_12_0
        fn_state.gs_8643 = s_12_0;
        // N s_12_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#8644 <= s_13_0
        fn_state.gs_8644 = s_13_0;
        // N s_13_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#8645 <= s_14_0
        fn_state.gs_8645 = s_14_0;
        // N s_14_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_15_0: const #1u : u32
        let s_15_0: u32 = 1;
        // D s_15_1: read-var statuscode:u32
        let s_15_1: u32 = fn_state.statuscode;
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
    ) -> u8 {
        // C s_16_0: const #1s : i
        let s_16_0: i128 = 1;
        // C s_16_1: const #0s : i
        let s_16_1: i128 = 0;
        // D s_16_2: read-var level:i
        let s_16_2: i128 = fn_state.level;
        // D s_16_3: call integer_subrange(s_16_2, s_16_0, s_16_1)
        let s_16_3: Bits = integer_subrange(state, tracer, s_16_2, s_16_0, s_16_1);
        // D s_16_4: cast reint s_16_3 -> u8
        let s_16_4: u8 = (s_16_3.value() as u8);
        // C s_16_5: const #2u : u8
        let s_16_5: u8 = 2;
        // C s_16_6: cast zx s_16_5 -> bv
        let s_16_6: Bits = Bits::new(s_16_5 as u128, 4u16);
        // D s_16_7: cast zx s_16_4 -> bv
        let s_16_7: Bits = Bits::new(s_16_4 as u128, 2u16);
        // C s_16_8: cast reint s_16_6 -> u128
        let s_16_8: u128 = (s_16_6.value() as u128);
        // D s_16_9: size-of s_16_6
        let s_16_9: u16 = s_16_6.length();
        // D s_16_10: cast reint s_16_7 -> u128
        let s_16_10: u128 = (s_16_7.value() as u128);
        // D s_16_11: size-of s_16_7
        let s_16_11: u16 = s_16_7.length();
        // D s_16_12: lsl s_16_8 s_16_11
        let s_16_12: u128 = s_16_8 << s_16_11;
        // D s_16_13: or s_16_12 s_16_10
        let s_16_13: u128 = ((s_16_12) | (s_16_10));
        // D s_16_14: add s_16_9 s_16_11
        let s_16_14: u16 = (s_16_9 + s_16_11);
        // D s_16_15: create-bits s_16_13 s_16_14
        let s_16_15: Bits = Bits::new(s_16_13, s_16_14);
        // D s_16_16: cast reint s_16_15 -> u8
        let s_16_16: u8 = (s_16_15.value() as u8);
        // D s_16_17: write-var result <= s_16_16
        fn_state.result = s_16_16;
        // C s_16_18: const #0s : i
        let s_16_18: i128 = 0;
        // D s_16_19: read-var level:i
        let s_16_19: i128 = fn_state.level;
        // D s_16_20: cmp-eq s_16_19 s_16_18
        let s_16_20: bool = ((s_16_19) == (s_16_18));
        // N s_16_21: branch s_16_20 b25 b17
        if s_16_20 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_17_0: const #1s : i
        let s_17_0: i128 = 1;
        // D s_17_1: read-var level:i
        let s_17_1: i128 = fn_state.level;
        // D s_17_2: cmp-eq s_17_1 s_17_0
        let s_17_2: bool = ((s_17_1) == (s_17_0));
        // N s_17_3: branch s_17_2 b24 b18
        if s_17_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_18_0: const #2s : i
        let s_18_0: i128 = 2;
        // D s_18_1: read-var level:i
        let s_18_1: i128 = fn_state.level;
        // D s_18_2: cmp-eq s_18_1 s_18_0
        let s_18_2: bool = ((s_18_1) == (s_18_0));
        // N s_18_3: branch s_18_2 b23 b19
        if s_18_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_19_0: const #3s : i
        let s_19_0: i128 = 3;
        // D s_19_1: read-var level:i
        let s_19_1: i128 = fn_state.level;
        // D s_19_2: cmp-eq s_19_1 s_19_0
        let s_19_2: bool = ((s_19_1) == (s_19_0));
        // D s_19_3: write-var gs#8654 <= s_19_2
        fn_state.gs_8654 = s_19_2;
        // N s_19_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_20_0: read-var gs#8654:u8
        let s_20_0: bool = fn_state.gs_8654;
        // D s_20_1: write-var gs#8655 <= s_20_0
        fn_state.gs_8655 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_21_0: read-var gs#8655:u8
        let s_21_0: bool = fn_state.gs_8655;
        // D s_21_1: write-var gs#8656 <= s_21_0
        fn_state.gs_8656 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_22_0: read-var gs#8656:u8
        let s_22_0: bool = fn_state.gs_8656;
        // N s_22_1: assert s_22_0
        let s_22_1: () = assert!(s_22_0);
        // N s_22_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#8654 <= s_23_0
        fn_state.gs_8654 = s_23_0;
        // N s_23_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#8655 <= s_24_0
        fn_state.gs_8655 = s_24_0;
        // N s_24_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#8656 <= s_25_0
        fn_state.gs_8656 = s_25_0;
        // N s_25_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_26_0: const #5u : u32
        let s_26_0: u32 = 5;
        // D s_26_1: read-var statuscode:u32
        let s_26_1: u32 = fn_state.statuscode;
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
    ) -> u8 {
        // C s_27_0: const #1s : i
        let s_27_0: i128 = 1;
        // C s_27_1: const #0s : i
        let s_27_1: i128 = 0;
        // D s_27_2: read-var level:i
        let s_27_2: i128 = fn_state.level;
        // D s_27_3: call integer_subrange(s_27_2, s_27_0, s_27_1)
        let s_27_3: Bits = integer_subrange(state, tracer, s_27_2, s_27_0, s_27_1);
        // D s_27_4: cast reint s_27_3 -> u8
        let s_27_4: u8 = (s_27_3.value() as u8);
        // C s_27_5: const #3u : u8
        let s_27_5: u8 = 3;
        // C s_27_6: cast zx s_27_5 -> bv
        let s_27_6: Bits = Bits::new(s_27_5 as u128, 4u16);
        // D s_27_7: cast zx s_27_4 -> bv
        let s_27_7: Bits = Bits::new(s_27_4 as u128, 2u16);
        // C s_27_8: cast reint s_27_6 -> u128
        let s_27_8: u128 = (s_27_6.value() as u128);
        // D s_27_9: size-of s_27_6
        let s_27_9: u16 = s_27_6.length();
        // D s_27_10: cast reint s_27_7 -> u128
        let s_27_10: u128 = (s_27_7.value() as u128);
        // D s_27_11: size-of s_27_7
        let s_27_11: u16 = s_27_7.length();
        // D s_27_12: lsl s_27_8 s_27_11
        let s_27_12: u128 = s_27_8 << s_27_11;
        // D s_27_13: or s_27_12 s_27_10
        let s_27_13: u128 = ((s_27_12) | (s_27_10));
        // D s_27_14: add s_27_9 s_27_11
        let s_27_14: u16 = (s_27_9 + s_27_11);
        // D s_27_15: create-bits s_27_13 s_27_14
        let s_27_15: Bits = Bits::new(s_27_13, s_27_14);
        // D s_27_16: cast reint s_27_15 -> u8
        let s_27_16: u8 = (s_27_15.value() as u8);
        // D s_27_17: write-var result <= s_27_16
        fn_state.result = s_27_16;
        // C s_27_18: const #0s : i
        let s_27_18: i128 = 0;
        // D s_27_19: read-var level:i
        let s_27_19: i128 = fn_state.level;
        // D s_27_20: cmp-eq s_27_19 s_27_18
        let s_27_20: bool = ((s_27_19) == (s_27_18));
        // N s_27_21: branch s_27_20 b36 b28
        if s_27_20 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_28_0: const #1s : i
        let s_28_0: i128 = 1;
        // D s_28_1: read-var level:i
        let s_28_1: i128 = fn_state.level;
        // D s_28_2: cmp-eq s_28_1 s_28_0
        let s_28_2: bool = ((s_28_1) == (s_28_0));
        // N s_28_3: branch s_28_2 b35 b29
        if s_28_2 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_29_0: const #2s : i
        let s_29_0: i128 = 2;
        // D s_29_1: read-var level:i
        let s_29_1: i128 = fn_state.level;
        // D s_29_2: cmp-eq s_29_1 s_29_0
        let s_29_2: bool = ((s_29_1) == (s_29_0));
        // N s_29_3: branch s_29_2 b34 b30
        if s_29_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_30_0: const #3s : i
        let s_30_0: i128 = 3;
        // D s_30_1: read-var level:i
        let s_30_1: i128 = fn_state.level;
        // D s_30_2: cmp-eq s_30_1 s_30_0
        let s_30_2: bool = ((s_30_1) == (s_30_0));
        // D s_30_3: write-var gs#8665 <= s_30_2
        fn_state.gs_8665 = s_30_2;
        // N s_30_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_31_0: read-var gs#8665:u8
        let s_31_0: bool = fn_state.gs_8665;
        // D s_31_1: write-var gs#8666 <= s_31_0
        fn_state.gs_8666 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_32_0: read-var gs#8666:u8
        let s_32_0: bool = fn_state.gs_8666;
        // D s_32_1: write-var gs#8667 <= s_32_0
        fn_state.gs_8667 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_33_0: read-var gs#8667:u8
        let s_33_0: bool = fn_state.gs_8667;
        // N s_33_1: assert s_33_0
        let s_33_1: () = assert!(s_33_0);
        // N s_33_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#8665 <= s_34_0
        fn_state.gs_8665 = s_34_0;
        // N s_34_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#8666 <= s_35_0
        fn_state.gs_8666 = s_35_0;
        // N s_35_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#8667 <= s_36_0
        fn_state.gs_8667 = s_36_0;
        // N s_36_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_37_0: const #6u : u32
        let s_37_0: u32 = 6;
        // D s_37_1: read-var statuscode:u32
        let s_37_1: u32 = fn_state.statuscode;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // D s_37_3: not s_37_2
        let s_37_3: bool = !s_37_2;
        // N s_37_4: branch s_37_3 b48 b38
        if s_37_3 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_38_0: const #1s : i
        let s_38_0: i128 = 1;
        // C s_38_1: const #0s : i
        let s_38_1: i128 = 0;
        // D s_38_2: read-var level:i
        let s_38_2: i128 = fn_state.level;
        // D s_38_3: call integer_subrange(s_38_2, s_38_0, s_38_1)
        let s_38_3: Bits = integer_subrange(state, tracer, s_38_2, s_38_0, s_38_1);
        // D s_38_4: cast reint s_38_3 -> u8
        let s_38_4: u8 = (s_38_3.value() as u8);
        // C s_38_5: const #1u : u8
        let s_38_5: u8 = 1;
        // C s_38_6: cast zx s_38_5 -> bv
        let s_38_6: Bits = Bits::new(s_38_5 as u128, 4u16);
        // D s_38_7: cast zx s_38_4 -> bv
        let s_38_7: Bits = Bits::new(s_38_4 as u128, 2u16);
        // C s_38_8: cast reint s_38_6 -> u128
        let s_38_8: u128 = (s_38_6.value() as u128);
        // D s_38_9: size-of s_38_6
        let s_38_9: u16 = s_38_6.length();
        // D s_38_10: cast reint s_38_7 -> u128
        let s_38_10: u128 = (s_38_7.value() as u128);
        // D s_38_11: size-of s_38_7
        let s_38_11: u16 = s_38_7.length();
        // D s_38_12: lsl s_38_8 s_38_11
        let s_38_12: u128 = s_38_8 << s_38_11;
        // D s_38_13: or s_38_12 s_38_10
        let s_38_13: u128 = ((s_38_12) | (s_38_10));
        // D s_38_14: add s_38_9 s_38_11
        let s_38_14: u16 = (s_38_9 + s_38_11);
        // D s_38_15: create-bits s_38_13 s_38_14
        let s_38_15: Bits = Bits::new(s_38_13, s_38_14);
        // D s_38_16: cast reint s_38_15 -> u8
        let s_38_16: u8 = (s_38_15.value() as u8);
        // D s_38_17: write-var result <= s_38_16
        fn_state.result = s_38_16;
        // C s_38_18: const #0s : i
        let s_38_18: i128 = 0;
        // D s_38_19: read-var level:i
        let s_38_19: i128 = fn_state.level;
        // D s_38_20: cmp-eq s_38_19 s_38_18
        let s_38_20: bool = ((s_38_19) == (s_38_18));
        // N s_38_21: branch s_38_20 b47 b39
        if s_38_20 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_39_0: const #1s : i
        let s_39_0: i128 = 1;
        // D s_39_1: read-var level:i
        let s_39_1: i128 = fn_state.level;
        // D s_39_2: cmp-eq s_39_1 s_39_0
        let s_39_2: bool = ((s_39_1) == (s_39_0));
        // N s_39_3: branch s_39_2 b46 b40
        if s_39_2 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_40_0: const #2s : i
        let s_40_0: i128 = 2;
        // D s_40_1: read-var level:i
        let s_40_1: i128 = fn_state.level;
        // D s_40_2: cmp-eq s_40_1 s_40_0
        let s_40_2: bool = ((s_40_1) == (s_40_0));
        // N s_40_3: branch s_40_2 b45 b41
        if s_40_2 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_41_0: const #3s : i
        let s_41_0: i128 = 3;
        // D s_41_1: read-var level:i
        let s_41_1: i128 = fn_state.level;
        // D s_41_2: cmp-eq s_41_1 s_41_0
        let s_41_2: bool = ((s_41_1) == (s_41_0));
        // D s_41_3: write-var gs#8676 <= s_41_2
        fn_state.gs_8676 = s_41_2;
        // N s_41_4: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_42_0: read-var gs#8676:u8
        let s_42_0: bool = fn_state.gs_8676;
        // D s_42_1: write-var gs#8677 <= s_42_0
        fn_state.gs_8677 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_43_0: read-var gs#8677:u8
        let s_43_0: bool = fn_state.gs_8677;
        // D s_43_1: write-var gs#8678 <= s_43_0
        fn_state.gs_8678 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_44_0: read-var gs#8678:u8
        let s_44_0: bool = fn_state.gs_8678;
        // N s_44_1: assert s_44_0
        let s_44_1: () = assert!(s_44_0);
        // N s_44_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#8676 <= s_45_0
        fn_state.gs_8676 = s_45_0;
        // N s_45_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#8677 <= s_46_0
        fn_state.gs_8677 = s_46_0;
        // N s_46_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#8678 <= s_47_0
        fn_state.gs_8678 = s_47_0;
        // N s_47_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_48_0: const #8u : u32
        let s_48_0: u32 = 8;
        // D s_48_1: read-var statuscode:u32
        let s_48_1: u32 = fn_state.statuscode;
        // D s_48_2: cmp-eq s_48_0 s_48_1
        let s_48_2: bool = ((s_48_0) == (s_48_1));
        // D s_48_3: not s_48_2
        let s_48_3: bool = !s_48_2;
        // N s_48_4: branch s_48_3 b50 b49
        if s_48_3 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_49_0: const #16u : u8
        let s_49_0: u8 = 16;
        // D s_49_1: write-var result <= s_49_0
        fn_state.result = s_49_0;
        // N s_49_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_50_0: const #9u : u32
        let s_50_0: u32 = 9;
        // D s_50_1: read-var statuscode:u32
        let s_50_1: u32 = fn_state.statuscode;
        // D s_50_2: cmp-eq s_50_0 s_50_1
        let s_50_2: bool = ((s_50_0) == (s_50_1));
        // D s_50_3: not s_50_2
        let s_50_3: bool = !s_50_2;
        // N s_50_4: branch s_50_3 b61 b51
        if s_50_3 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_51_0: const #1s : i
        let s_51_0: i128 = 1;
        // C s_51_1: const #0s : i
        let s_51_1: i128 = 0;
        // D s_51_2: read-var level:i
        let s_51_2: i128 = fn_state.level;
        // D s_51_3: call integer_subrange(s_51_2, s_51_0, s_51_1)
        let s_51_3: Bits = integer_subrange(state, tracer, s_51_2, s_51_0, s_51_1);
        // D s_51_4: cast reint s_51_3 -> u8
        let s_51_4: u8 = (s_51_3.value() as u8);
        // C s_51_5: const #5u : u8
        let s_51_5: u8 = 5;
        // C s_51_6: cast zx s_51_5 -> bv
        let s_51_6: Bits = Bits::new(s_51_5 as u128, 4u16);
        // D s_51_7: cast zx s_51_4 -> bv
        let s_51_7: Bits = Bits::new(s_51_4 as u128, 2u16);
        // C s_51_8: cast reint s_51_6 -> u128
        let s_51_8: u128 = (s_51_6.value() as u128);
        // D s_51_9: size-of s_51_6
        let s_51_9: u16 = s_51_6.length();
        // D s_51_10: cast reint s_51_7 -> u128
        let s_51_10: u128 = (s_51_7.value() as u128);
        // D s_51_11: size-of s_51_7
        let s_51_11: u16 = s_51_7.length();
        // D s_51_12: lsl s_51_8 s_51_11
        let s_51_12: u128 = s_51_8 << s_51_11;
        // D s_51_13: or s_51_12 s_51_10
        let s_51_13: u128 = ((s_51_12) | (s_51_10));
        // D s_51_14: add s_51_9 s_51_11
        let s_51_14: u16 = (s_51_9 + s_51_11);
        // D s_51_15: create-bits s_51_13 s_51_14
        let s_51_15: Bits = Bits::new(s_51_13, s_51_14);
        // D s_51_16: cast reint s_51_15 -> u8
        let s_51_16: u8 = (s_51_15.value() as u8);
        // D s_51_17: write-var result <= s_51_16
        fn_state.result = s_51_16;
        // C s_51_18: const #0s : i
        let s_51_18: i128 = 0;
        // D s_51_19: read-var level:i
        let s_51_19: i128 = fn_state.level;
        // D s_51_20: cmp-eq s_51_19 s_51_18
        let s_51_20: bool = ((s_51_19) == (s_51_18));
        // N s_51_21: branch s_51_20 b60 b52
        if s_51_20 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_52_0: const #1s : i
        let s_52_0: i128 = 1;
        // D s_52_1: read-var level:i
        let s_52_1: i128 = fn_state.level;
        // D s_52_2: cmp-eq s_52_1 s_52_0
        let s_52_2: bool = ((s_52_1) == (s_52_0));
        // N s_52_3: branch s_52_2 b59 b53
        if s_52_2 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_53_0: const #2s : i
        let s_53_0: i128 = 2;
        // D s_53_1: read-var level:i
        let s_53_1: i128 = fn_state.level;
        // D s_53_2: cmp-eq s_53_1 s_53_0
        let s_53_2: bool = ((s_53_1) == (s_53_0));
        // N s_53_3: branch s_53_2 b58 b54
        if s_53_2 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_54_0: const #3s : i
        let s_54_0: i128 = 3;
        // D s_54_1: read-var level:i
        let s_54_1: i128 = fn_state.level;
        // D s_54_2: cmp-eq s_54_1 s_54_0
        let s_54_2: bool = ((s_54_1) == (s_54_0));
        // D s_54_3: write-var gs#8688 <= s_54_2
        fn_state.gs_8688 = s_54_2;
        // N s_54_4: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_55_0: read-var gs#8688:u8
        let s_55_0: bool = fn_state.gs_8688;
        // D s_55_1: write-var gs#8689 <= s_55_0
        fn_state.gs_8689 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_56_0: read-var gs#8689:u8
        let s_56_0: bool = fn_state.gs_8689;
        // D s_56_1: write-var gs#8690 <= s_56_0
        fn_state.gs_8690 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_57_0: read-var gs#8690:u8
        let s_57_0: bool = fn_state.gs_8690;
        // N s_57_1: assert s_57_0
        let s_57_1: () = assert!(s_57_0);
        // N s_57_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#8688 <= s_58_0
        fn_state.gs_8688 = s_58_0;
        // N s_58_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_59_0: const #1u : u8
        let s_59_0: bool = true;
        // D s_59_1: write-var gs#8689 <= s_59_0
        fn_state.gs_8689 = s_59_0;
        // N s_59_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#8690 <= s_60_0
        fn_state.gs_8690 = s_60_0;
        // N s_60_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_61_0: const #10u : u32
        let s_61_0: u32 = 10;
        // D s_61_1: read-var statuscode:u32
        let s_61_1: u32 = fn_state.statuscode;
        // D s_61_2: cmp-eq s_61_0 s_61_1
        let s_61_2: bool = ((s_61_0) == (s_61_1));
        // D s_61_3: not s_61_2
        let s_61_3: bool = !s_61_2;
        // N s_61_4: branch s_61_3 b63 b62
        if s_61_3 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_62_0: const #24u : u8
        let s_62_0: u8 = 24;
        // D s_62_1: write-var result <= s_62_0
        fn_state.result = s_62_0;
        // N s_62_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_63_0: const #11u : u32
        let s_63_0: u32 = 11;
        // D s_63_1: read-var statuscode:u32
        let s_63_1: u32 = fn_state.statuscode;
        // D s_63_2: cmp-eq s_63_0 s_63_1
        let s_63_2: bool = ((s_63_0) == (s_63_1));
        // D s_63_3: not s_63_2
        let s_63_3: bool = !s_63_2;
        // N s_63_4: branch s_63_3 b74 b64
        if s_63_3 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_64_0: const #1s : i
        let s_64_0: i128 = 1;
        // C s_64_1: const #0s : i
        let s_64_1: i128 = 0;
        // D s_64_2: read-var level:i
        let s_64_2: i128 = fn_state.level;
        // D s_64_3: call integer_subrange(s_64_2, s_64_0, s_64_1)
        let s_64_3: Bits = integer_subrange(state, tracer, s_64_2, s_64_0, s_64_1);
        // D s_64_4: cast reint s_64_3 -> u8
        let s_64_4: u8 = (s_64_3.value() as u8);
        // C s_64_5: const #7u : u8
        let s_64_5: u8 = 7;
        // C s_64_6: cast zx s_64_5 -> bv
        let s_64_6: Bits = Bits::new(s_64_5 as u128, 4u16);
        // D s_64_7: cast zx s_64_4 -> bv
        let s_64_7: Bits = Bits::new(s_64_4 as u128, 2u16);
        // C s_64_8: cast reint s_64_6 -> u128
        let s_64_8: u128 = (s_64_6.value() as u128);
        // D s_64_9: size-of s_64_6
        let s_64_9: u16 = s_64_6.length();
        // D s_64_10: cast reint s_64_7 -> u128
        let s_64_10: u128 = (s_64_7.value() as u128);
        // D s_64_11: size-of s_64_7
        let s_64_11: u16 = s_64_7.length();
        // D s_64_12: lsl s_64_8 s_64_11
        let s_64_12: u128 = s_64_8 << s_64_11;
        // D s_64_13: or s_64_12 s_64_10
        let s_64_13: u128 = ((s_64_12) | (s_64_10));
        // D s_64_14: add s_64_9 s_64_11
        let s_64_14: u16 = (s_64_9 + s_64_11);
        // D s_64_15: create-bits s_64_13 s_64_14
        let s_64_15: Bits = Bits::new(s_64_13, s_64_14);
        // D s_64_16: cast reint s_64_15 -> u8
        let s_64_16: u8 = (s_64_15.value() as u8);
        // D s_64_17: write-var result <= s_64_16
        fn_state.result = s_64_16;
        // C s_64_18: const #0s : i
        let s_64_18: i128 = 0;
        // D s_64_19: read-var level:i
        let s_64_19: i128 = fn_state.level;
        // D s_64_20: cmp-eq s_64_19 s_64_18
        let s_64_20: bool = ((s_64_19) == (s_64_18));
        // N s_64_21: branch s_64_20 b73 b65
        if s_64_20 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_65_0: const #1s : i
        let s_65_0: i128 = 1;
        // D s_65_1: read-var level:i
        let s_65_1: i128 = fn_state.level;
        // D s_65_2: cmp-eq s_65_1 s_65_0
        let s_65_2: bool = ((s_65_1) == (s_65_0));
        // N s_65_3: branch s_65_2 b72 b66
        if s_65_2 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_66_0: const #2s : i
        let s_66_0: i128 = 2;
        // D s_66_1: read-var level:i
        let s_66_1: i128 = fn_state.level;
        // D s_66_2: cmp-eq s_66_1 s_66_0
        let s_66_2: bool = ((s_66_1) == (s_66_0));
        // N s_66_3: branch s_66_2 b71 b67
        if s_66_2 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_67_0: const #3s : i
        let s_67_0: i128 = 3;
        // D s_67_1: read-var level:i
        let s_67_1: i128 = fn_state.level;
        // D s_67_2: cmp-eq s_67_1 s_67_0
        let s_67_2: bool = ((s_67_1) == (s_67_0));
        // D s_67_3: write-var gs#8700 <= s_67_2
        fn_state.gs_8700 = s_67_2;
        // N s_67_4: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_68_0: read-var gs#8700:u8
        let s_68_0: bool = fn_state.gs_8700;
        // D s_68_1: write-var gs#8701 <= s_68_0
        fn_state.gs_8701 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_69_0: read-var gs#8701:u8
        let s_69_0: bool = fn_state.gs_8701;
        // D s_69_1: write-var gs#8702 <= s_69_0
        fn_state.gs_8702 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_70_0: read-var gs#8702:u8
        let s_70_0: bool = fn_state.gs_8702;
        // N s_70_1: assert s_70_0
        let s_70_1: () = assert!(s_70_0);
        // N s_70_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#8700 <= s_71_0
        fn_state.gs_8700 = s_71_0;
        // N s_71_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#8701 <= s_72_0
        fn_state.gs_8701 = s_72_0;
        // N s_72_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var gs#8702 <= s_73_0
        fn_state.gs_8702 = s_73_0;
        // N s_73_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_74_0: const #14u : u32
        let s_74_0: u32 = 14;
        // D s_74_1: read-var statuscode:u32
        let s_74_1: u32 = fn_state.statuscode;
        // D s_74_2: cmp-eq s_74_0 s_74_1
        let s_74_2: bool = ((s_74_0) == (s_74_1));
        // D s_74_3: not s_74_2
        let s_74_3: bool = !s_74_2;
        // N s_74_4: branch s_74_3 b76 b75
        if s_74_3 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_75_0: const #25u : u8
        let s_75_0: u8 = 25;
        // D s_75_1: write-var result <= s_75_0
        fn_state.result = s_75_0;
        // N s_75_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_76_0: const #15u : u32
        let s_76_0: u32 = 15;
        // D s_76_1: read-var statuscode:u32
        let s_76_1: u32 = fn_state.statuscode;
        // D s_76_2: cmp-eq s_76_0 s_76_1
        let s_76_2: bool = ((s_76_0) == (s_76_1));
        // D s_76_3: not s_76_2
        let s_76_3: bool = !s_76_2;
        // N s_76_4: branch s_76_3 b78 b77
        if s_76_3 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_77_0: const #17u : u8
        let s_77_0: u8 = 17;
        // D s_77_1: write-var result <= s_77_0
        fn_state.result = s_77_0;
        // C s_77_2: const #() : ()
        let s_77_2: () = ();
        // S s_77_3: call UsingAArch32(s_77_2)
        let s_77_3: bool = UsingAArch32(state, tracer, s_77_2);
        // N s_77_4: assert s_77_3
        let s_77_4: () = assert!(s_77_3);
        // N s_77_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_78_0: const #16u : u32
        let s_78_0: u32 = 16;
        // D s_78_1: read-var statuscode:u32
        let s_78_1: u32 = fn_state.statuscode;
        // D s_78_2: cmp-eq s_78_0 s_78_1
        let s_78_2: bool = ((s_78_0) == (s_78_1));
        // D s_78_3: not s_78_2
        let s_78_3: bool = !s_78_2;
        // N s_78_4: branch s_78_3 b80 b79
        if s_78_3 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_79_0: const #17u : u8
        let s_79_0: u8 = 17;
        // D s_79_1: write-var result <= s_79_0
        fn_state.result = s_79_0;
        // C s_79_2: const #() : ()
        let s_79_2: () = ();
        // S s_79_3: call HaveMTE2Ext(s_79_2)
        let s_79_3: bool = HaveMTE2Ext(state, tracer, s_79_2);
        // N s_79_4: assert s_79_3
        let s_79_4: () = assert!(s_79_3);
        // N s_79_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_80_0: const #2u : u32
        let s_80_0: u32 = 2;
        // D s_80_1: read-var statuscode:u32
        let s_80_1: u32 = fn_state.statuscode;
        // D s_80_2: cmp-eq s_80_0 s_80_1
        let s_80_2: bool = ((s_80_0) == (s_80_1));
        // D s_80_3: not s_80_2
        let s_80_3: bool = !s_80_2;
        // N s_80_4: branch s_80_3 b82 b81
        if s_80_3 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_81_0: const #33u : u8
        let s_81_0: u8 = 33;
        // D s_81_1: write-var result <= s_81_0
        fn_state.result = s_81_0;
        // N s_81_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_82_0: const #17u : u32
        let s_82_0: u32 = 17;
        // D s_82_1: read-var statuscode:u32
        let s_82_1: u32 = fn_state.statuscode;
        // D s_82_2: cmp-eq s_82_0 s_82_1
        let s_82_2: bool = ((s_82_0) == (s_82_1));
        // D s_82_3: not s_82_2
        let s_82_3: bool = !s_82_2;
        // N s_82_4: branch s_82_3 b84 b83
        if s_82_3 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_83_0: const #34u : u8
        let s_83_0: u8 = 34;
        // D s_83_1: write-var result <= s_83_0
        fn_state.result = s_83_0;
        // N s_83_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_84_0: const #12u : u32
        let s_84_0: u32 = 12;
        // D s_84_1: read-var statuscode:u32
        let s_84_1: u32 = fn_state.statuscode;
        // D s_84_2: cmp-eq s_84_0 s_84_1
        let s_84_2: bool = ((s_84_0) == (s_84_1));
        // D s_84_3: not s_84_2
        let s_84_3: bool = !s_84_2;
        // N s_84_4: branch s_84_3 b95 b85
        if s_84_3 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_85_0: const #1s : i
        let s_85_0: i128 = 1;
        // C s_85_1: const #0s : i
        let s_85_1: i128 = 0;
        // D s_85_2: read-var level:i
        let s_85_2: i128 = fn_state.level;
        // D s_85_3: call integer_subrange(s_85_2, s_85_0, s_85_1)
        let s_85_3: Bits = integer_subrange(state, tracer, s_85_2, s_85_0, s_85_1);
        // D s_85_4: cast reint s_85_3 -> u8
        let s_85_4: u8 = (s_85_3.value() as u8);
        // C s_85_5: const #9u : u8
        let s_85_5: u8 = 9;
        // C s_85_6: cast zx s_85_5 -> bv
        let s_85_6: Bits = Bits::new(s_85_5 as u128, 4u16);
        // D s_85_7: cast zx s_85_4 -> bv
        let s_85_7: Bits = Bits::new(s_85_4 as u128, 2u16);
        // C s_85_8: cast reint s_85_6 -> u128
        let s_85_8: u128 = (s_85_6.value() as u128);
        // D s_85_9: size-of s_85_6
        let s_85_9: u16 = s_85_6.length();
        // D s_85_10: cast reint s_85_7 -> u128
        let s_85_10: u128 = (s_85_7.value() as u128);
        // D s_85_11: size-of s_85_7
        let s_85_11: u16 = s_85_7.length();
        // D s_85_12: lsl s_85_8 s_85_11
        let s_85_12: u128 = s_85_8 << s_85_11;
        // D s_85_13: or s_85_12 s_85_10
        let s_85_13: u128 = ((s_85_12) | (s_85_10));
        // D s_85_14: add s_85_9 s_85_11
        let s_85_14: u16 = (s_85_9 + s_85_11);
        // D s_85_15: create-bits s_85_13 s_85_14
        let s_85_15: Bits = Bits::new(s_85_13, s_85_14);
        // D s_85_16: cast reint s_85_15 -> u8
        let s_85_16: u8 = (s_85_15.value() as u8);
        // D s_85_17: write-var result <= s_85_16
        fn_state.result = s_85_16;
        // C s_85_18: const #0s : i
        let s_85_18: i128 = 0;
        // D s_85_19: read-var level:i
        let s_85_19: i128 = fn_state.level;
        // D s_85_20: cmp-eq s_85_19 s_85_18
        let s_85_20: bool = ((s_85_19) == (s_85_18));
        // N s_85_21: branch s_85_20 b94 b86
        if s_85_20 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_86_0: const #1s : i
        let s_86_0: i128 = 1;
        // D s_86_1: read-var level:i
        let s_86_1: i128 = fn_state.level;
        // D s_86_2: cmp-eq s_86_1 s_86_0
        let s_86_2: bool = ((s_86_1) == (s_86_0));
        // N s_86_3: branch s_86_2 b93 b87
        if s_86_2 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_87_0: const #2s : i
        let s_87_0: i128 = 2;
        // D s_87_1: read-var level:i
        let s_87_1: i128 = fn_state.level;
        // D s_87_2: cmp-eq s_87_1 s_87_0
        let s_87_2: bool = ((s_87_1) == (s_87_0));
        // N s_87_3: branch s_87_2 b92 b88
        if s_87_2 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_88_0: const #3s : i
        let s_88_0: i128 = 3;
        // D s_88_1: read-var level:i
        let s_88_1: i128 = fn_state.level;
        // D s_88_2: cmp-eq s_88_1 s_88_0
        let s_88_2: bool = ((s_88_1) == (s_88_0));
        // D s_88_3: write-var gs#8718 <= s_88_2
        fn_state.gs_8718 = s_88_2;
        // N s_88_4: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_89_0: read-var gs#8718:u8
        let s_89_0: bool = fn_state.gs_8718;
        // D s_89_1: write-var gs#8719 <= s_89_0
        fn_state.gs_8719 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_90_0: read-var gs#8719:u8
        let s_90_0: bool = fn_state.gs_8719;
        // D s_90_1: write-var gs#8720 <= s_90_0
        fn_state.gs_8720 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_91_0: read-var gs#8720:u8
        let s_91_0: bool = fn_state.gs_8720;
        // N s_91_1: assert s_91_0
        let s_91_1: () = assert!(s_91_0);
        // N s_91_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_92_0: const #1u : u8
        let s_92_0: bool = true;
        // D s_92_1: write-var gs#8718 <= s_92_0
        fn_state.gs_8718 = s_92_0;
        // N s_92_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_93_0: const #1u : u8
        let s_93_0: bool = true;
        // D s_93_1: write-var gs#8719 <= s_93_0
        fn_state.gs_8719 = s_93_0;
        // N s_93_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_94_0: const #1u : u8
        let s_94_0: bool = true;
        // D s_94_1: write-var gs#8720 <= s_94_0
        fn_state.gs_8720 = s_94_0;
        // N s_94_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_95_0: const #13u : u32
        let s_95_0: u32 = 13;
        // D s_95_1: read-var statuscode:u32
        let s_95_1: u32 = fn_state.statuscode;
        // D s_95_2: cmp-eq s_95_0 s_95_1
        let s_95_2: bool = ((s_95_0) == (s_95_1));
        // D s_95_3: not s_95_2
        let s_95_3: bool = !s_95_2;
        // N s_95_4: branch s_95_3 b97 b96
        if s_95_3 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_96_0: const #40u : u8
        let s_96_0: u8 = 40;
        // D s_96_1: write-var result <= s_96_0
        fn_state.result = s_96_0;
        // N s_96_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_97_0: const #18u : u32
        let s_97_0: u32 = 18;
        // D s_97_1: read-var statuscode:u32
        let s_97_1: u32 = fn_state.statuscode;
        // D s_97_2: cmp-eq s_97_0 s_97_1
        let s_97_2: bool = ((s_97_0) == (s_97_1));
        // D s_97_3: not s_97_2
        let s_97_3: bool = !s_97_2;
        // N s_97_4: branch s_97_3 b99 b98
        if s_97_3 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_98_0: const #48u : u8
        let s_98_0: u8 = 48;
        // D s_98_1: write-var result <= s_98_0
        fn_state.result = s_98_0;
        // N s_98_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_99_0: const #20u : u32
        let s_99_0: u32 = 20;
        // D s_99_1: read-var statuscode:u32
        let s_99_1: u32 = fn_state.statuscode;
        // D s_99_2: cmp-eq s_99_0 s_99_1
        let s_99_2: bool = ((s_99_0) == (s_99_1));
        // D s_99_3: not s_99_2
        let s_99_3: bool = !s_99_2;
        // N s_99_4: branch s_99_3 b101 b100
        if s_99_3 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_100_0: const #49u : u8
        let s_100_0: u8 = 49;
        // D s_100_1: write-var result <= s_100_0
        fn_state.result = s_100_0;
        // N s_100_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_101_0: const #21u : u32
        let s_101_0: u32 = 21;
        // D s_101_1: read-var statuscode:u32
        let s_101_1: u32 = fn_state.statuscode;
        // D s_101_2: cmp-eq s_101_0 s_101_1
        let s_101_2: bool = ((s_101_0) == (s_101_1));
        // D s_101_3: not s_101_2
        let s_101_3: bool = !s_101_2;
        // N s_101_4: branch s_101_3 b103 b102
        if s_101_3 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_102_0: const #52u : u8
        let s_102_0: u8 = 52;
        // D s_102_1: write-var result <= s_102_0
        fn_state.result = s_102_0;
        // N s_102_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_103_0: const #22u : u32
        let s_103_0: u32 = 22;
        // D s_103_1: read-var statuscode:u32
        let s_103_1: u32 = fn_state.statuscode;
        // D s_103_2: cmp-eq s_103_0 s_103_1
        let s_103_2: bool = ((s_103_0) == (s_103_1));
        // D s_103_3: not s_103_2
        let s_103_3: bool = !s_103_2;
        // N s_103_4: branch s_103_3 b105 b104
        if s_103_3 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_104_0: const #53u : u8
        let s_104_0: u8 = 53;
        // D s_104_1: write-var result <= s_104_0
        fn_state.result = s_104_0;
        // N s_104_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_105_0: const #() : ()
        let s_105_0: () = ();
        // S s_105_1: call Unreachable(s_105_0)
        let s_105_1: () = Unreachable(state, tracer, s_105_0);
        // N s_105_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_106_0: const #() : ()
        let s_106_0: () = ();
        // S s_106_1: call Have52BitIPAAndPASpaceExt(s_106_0)
        let s_106_1: bool = Have52BitIPAAndPASpaceExt(state, tracer, s_106_0);
        // N s_106_2: assert s_106_1
        let s_106_2: () = assert!(s_106_1);
        // C s_106_3: const #7u : u32
        let s_106_3: u32 = 7;
        // D s_106_4: read-var statuscode:u32
        let s_106_4: u32 = fn_state.statuscode;
        // D s_106_5: cmp-eq s_106_3 s_106_4
        let s_106_5: bool = ((s_106_3) == (s_106_4));
        // D s_106_6: not s_106_5
        let s_106_6: bool = !s_106_5;
        // N s_106_7: branch s_106_6 b109 b107
        if s_106_6 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_107_0: const #41u : u8
        let s_107_0: u8 = 41;
        // D s_107_1: write-var result <= s_107_0
        fn_state.result = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_108_0: read-var result:u8
        let s_108_0: u8 = fn_state.result;
        // D s_108_1: write-var return_value <= s_108_0
        fn_state.return_value = s_108_0;
        // N s_108_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_109_0: const #6u : u32
        let s_109_0: u32 = 6;
        // D s_109_1: read-var statuscode:u32
        let s_109_1: u32 = fn_state.statuscode;
        // D s_109_2: cmp-eq s_109_0 s_109_1
        let s_109_2: bool = ((s_109_0) == (s_109_1));
        // D s_109_3: not s_109_2
        let s_109_3: bool = !s_109_2;
        // N s_109_4: branch s_109_3 b111 b110
        if s_109_3 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_110_0: const #43u : u8
        let s_110_0: u8 = 43;
        // D s_110_1: write-var result <= s_110_0
        fn_state.result = s_110_0;
        // N s_110_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_111_0: const #9u : u32
        let s_111_0: u32 = 9;
        // D s_111_1: read-var statuscode:u32
        let s_111_1: u32 = fn_state.statuscode;
        // D s_111_2: cmp-eq s_111_0 s_111_1
        let s_111_2: bool = ((s_111_0) == (s_111_1));
        // D s_111_3: not s_111_2
        let s_111_3: bool = !s_111_2;
        // N s_111_4: branch s_111_3 b113 b112
        if s_111_3 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_112_0: const #19u : u8
        let s_112_0: u8 = 19;
        // D s_112_1: write-var result <= s_112_0
        fn_state.result = s_112_0;
        // N s_112_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_113_0: const #11u : u32
        let s_113_0: u32 = 11;
        // D s_113_1: read-var statuscode:u32
        let s_113_1: u32 = fn_state.statuscode;
        // D s_113_2: cmp-eq s_113_0 s_113_1
        let s_113_2: bool = ((s_113_0) == (s_113_1));
        // D s_113_3: not s_113_2
        let s_113_3: bool = !s_113_2;
        // N s_113_4: branch s_113_3 b115 b114
        if s_113_3 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_114_0: const #27u : u8
        let s_114_0: u8 = 27;
        // D s_114_1: write-var result <= s_114_0
        fn_state.result = s_114_0;
        // C s_114_2: const #() : ()
        let s_114_2: () = ();
        // S s_114_3: call HaveRASExt(s_114_2)
        let s_114_3: bool = HaveRASExt(state, tracer, s_114_2);
        // S s_114_4: not s_114_3
        let s_114_4: bool = !s_114_3;
        // N s_114_5: assert s_114_4
        let s_114_5: () = assert!(s_114_4);
        // N s_114_6: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_115_0: const #12u : u32
        let s_115_0: u32 = 12;
        // D s_115_1: read-var statuscode:u32
        let s_115_1: u32 = fn_state.statuscode;
        // D s_115_2: cmp-eq s_115_0 s_115_1
        let s_115_2: bool = ((s_115_0) == (s_115_1));
        // D s_115_3: not s_115_2
        let s_115_3: bool = !s_115_2;
        // N s_115_4: branch s_115_3 b117 b116
        if s_115_3 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_116_0: const #35u : u8
        let s_116_0: u8 = 35;
        // D s_116_1: write-var result <= s_116_0
        fn_state.result = s_116_0;
        // N s_116_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_117_0: const #() : ()
        let s_117_0: () = ();
        // S s_117_1: call Unreachable(s_117_0)
        let s_117_1: () = Unreachable(state, tracer, s_117_0);
        // N s_117_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call Have56BitPAExt(s_118_0)
        let s_118_1: bool = Have56BitPAExt(state, tracer, s_118_0);
        // N s_118_2: assert s_118_1
        let s_118_2: () = assert!(s_118_1);
        // C s_118_3: const #7u : u32
        let s_118_3: u32 = 7;
        // D s_118_4: read-var statuscode:u32
        let s_118_4: u32 = fn_state.statuscode;
        // D s_118_5: cmp-eq s_118_3 s_118_4
        let s_118_5: bool = ((s_118_3) == (s_118_4));
        // D s_118_6: not s_118_5
        let s_118_6: bool = !s_118_5;
        // N s_118_7: branch s_118_6 b121 b119
        if s_118_6 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_119_0: const #44u : u8
        let s_119_0: u8 = 44;
        // D s_119_1: write-var result <= s_119_0
        fn_state.result = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_120_0: read-var result:u8
        let s_120_0: u8 = fn_state.result;
        // D s_120_1: write-var return_value <= s_120_0
        fn_state.return_value = s_120_0;
        // N s_120_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_121_0: const #6u : u32
        let s_121_0: u32 = 6;
        // D s_121_1: read-var statuscode:u32
        let s_121_1: u32 = fn_state.statuscode;
        // D s_121_2: cmp-eq s_121_0 s_121_1
        let s_121_2: bool = ((s_121_0) == (s_121_1));
        // D s_121_3: not s_121_2
        let s_121_3: bool = !s_121_2;
        // N s_121_4: branch s_121_3 b123 b122
        if s_121_3 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_122_0: const #42u : u8
        let s_122_0: u8 = 42;
        // D s_122_1: write-var result <= s_122_0
        fn_state.result = s_122_0;
        // N s_122_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_123_0: const #9u : u32
        let s_123_0: u32 = 9;
        // D s_123_1: read-var statuscode:u32
        let s_123_1: u32 = fn_state.statuscode;
        // D s_123_2: cmp-eq s_123_0 s_123_1
        let s_123_2: bool = ((s_123_0) == (s_123_1));
        // D s_123_3: not s_123_2
        let s_123_3: bool = !s_123_2;
        // N s_123_4: branch s_123_3 b125 b124
        if s_123_3 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_124_0: const #18u : u8
        let s_124_0: u8 = 18;
        // D s_124_1: write-var result <= s_124_0
        fn_state.result = s_124_0;
        // N s_124_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_125_0: const #11u : u32
        let s_125_0: u32 = 11;
        // D s_125_1: read-var statuscode:u32
        let s_125_1: u32 = fn_state.statuscode;
        // D s_125_2: cmp-eq s_125_0 s_125_1
        let s_125_2: bool = ((s_125_0) == (s_125_1));
        // D s_125_3: not s_125_2
        let s_125_3: bool = !s_125_2;
        // N s_125_4: branch s_125_3 b127 b126
        if s_125_3 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_126_0: const #26u : u8
        let s_126_0: u8 = 26;
        // D s_126_1: write-var result <= s_126_0
        fn_state.result = s_126_0;
        // C s_126_2: const #() : ()
        let s_126_2: () = ();
        // S s_126_3: call HaveRASExt(s_126_2)
        let s_126_3: bool = HaveRASExt(state, tracer, s_126_2);
        // S s_126_4: not s_126_3
        let s_126_4: bool = !s_126_3;
        // N s_126_5: assert s_126_4
        let s_126_5: () = assert!(s_126_4);
        // N s_126_6: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_127_0: const #12u : u32
        let s_127_0: u32 = 12;
        // D s_127_1: read-var statuscode:u32
        let s_127_1: u32 = fn_state.statuscode;
        // D s_127_2: cmp-eq s_127_0 s_127_1
        let s_127_2: bool = ((s_127_0) == (s_127_1));
        // D s_127_3: not s_127_2
        let s_127_3: bool = !s_127_2;
        // N s_127_4: branch s_127_3 b129 b128
        if s_127_3 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_128_0: const #34u : u8
        let s_128_0: u8 = 34;
        // D s_128_1: write-var result <= s_128_0
        fn_state.result = s_128_0;
        // N s_128_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_129_0: const #() : ()
        let s_129_0: () = ();
        // S s_129_1: call Unreachable(s_129_0)
        let s_129_1: () = Unreachable(state, tracer, s_129_0);
        // N s_129_2: jump b120
        return block_120(state, tracer, fn_state);
    }
}
