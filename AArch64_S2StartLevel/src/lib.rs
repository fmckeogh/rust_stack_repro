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
use fdiv_int::*;
use TGxGranuleBits::*;
use AArch64_IASize::*;
use common::*;
pub fn AArch64_S2StartLevel<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeb05ce25a107f0c5e,
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        ga_14141: u32,
        ga_14149: u8,
        ga_14146: i128,
        ga_14151: i128,
        ga_14148: i128,
        ga_14150: i128,
        return_value: i128,
        ga_14147: u8,
        ga_14144: u8,
        walkparams: ProductTypeb05ce25a107f0c5e,
    }
    let fn_state = FunctionState {
        walkparams,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_0_0: read-var walkparams.2:struct
        let s_0_0: bool = fn_state.walkparams._2;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // C s_0_2: const #1u : u8
        let s_0_2: bool = true;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b33 b1
        if s_0_4 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_1_0: read-var walkparams.26:struct
        let s_1_0: u32 = fn_state.walkparams._26;
        // D s_1_1: write-var ga#14141 <= s_1_0
        fn_state.ga_14141 = s_1_0;
        // C s_1_2: const #0u : u32
        let s_1_2: u32 = 0;
        // D s_1_3: read-var ga#14141:u32
        let s_1_3: u32 = fn_state.ga_14141;
        // D s_1_4: cmp-eq s_1_2 s_1_3
        let s_1_4: bool = ((s_1_2) == (s_1_3));
        // D s_1_5: not s_1_4
        let s_1_5: bool = !s_1_4;
        // N s_1_6: branch s_1_5 b14 b2
        if s_1_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_2_0: read-var walkparams.23:struct
        let s_2_0: bool = fn_state.walkparams._23;
        // D s_2_1: read-var walkparams.22:struct
        let s_2_1: u8 = fn_state.walkparams._22;
        // D s_2_2: cast zx s_2_0 -> bv
        let s_2_2: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_3: cast zx s_2_1 -> bv
        let s_2_3: Bits = Bits::new(s_2_1 as u128, 2u16);
        // D s_2_4: cast reint s_2_2 -> u128
        let s_2_4: u128 = (s_2_2.value() as u128);
        // D s_2_5: size-of s_2_2
        let s_2_5: u16 = s_2_2.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: u8 = (s_2_11.value() as u8);
        // D s_2_13: write-var ga#14144 <= s_2_12
        fn_state.ga_14144 = s_2_12;
        // D s_2_14: read-var ga#14144:u8
        let s_2_14: u8 = fn_state.ga_14144;
        // D s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 3u16);
        // C s_2_16: const #0u : u8
        let s_2_16: u8 = 0;
        // C s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 3u16);
        // D s_2_18: cmp-eq s_2_15 s_2_17
        let s_2_18: bool = ((s_2_15) == (s_2_17));
        // D s_2_19: not s_2_18
        let s_2_19: bool = !s_2_18;
        // N s_2_20: branch s_2_19 b5 b3
        if s_2_19 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: write-var return_value <= s_3_0
        fn_state.return_value = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_4_0: read-var return_value:i
        let s_4_0: i128 = fn_state.return_value;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_5_0: read-var ga#14144:u8
        let s_5_0: u8 = fn_state.ga_14144;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 3u16);
        // C s_5_2: const #1u : u8
        let s_5_2: u8 = 1;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 3u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_6_0: const #1s : i
        let s_6_0: i128 = 1;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_7_0: read-var ga#14144:u8
        let s_7_0: u8 = fn_state.ga_14144;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 3u16);
        // C s_7_2: const #2u : u8
        let s_7_2: u8 = 2;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 3u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_8_0: const #0s : i
        let s_8_0: i128 = 0;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_9_0: read-var ga#14144:u8
        let s_9_0: u8 = fn_state.ga_14144;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 3u16);
        // C s_9_2: const #3u : u8
        let s_9_2: u8 = 3;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 3u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: not s_9_4
        let s_9_5: bool = !s_9_4;
        // N s_9_6: branch s_9_5 b11 b10
        if s_9_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_10_0: const #3s : i
        let s_10_0: i128 = 3;
        // D s_10_1: write-var return_value <= s_10_0
        fn_state.return_value = s_10_0;
        // N s_10_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_11_0: read-var ga#14144:u8
        let s_11_0: u8 = fn_state.ga_14144;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 3u16);
        // C s_11_2: const #4u : u8
        let s_11_2: u8 = 4;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 3u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: not s_11_4
        let s_11_5: bool = !s_11_4;
        // N s_11_6: branch s_11_5 b13 b12
        if s_11_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_12_0: const #1s : i
        let s_12_0: i128 = 1;
        // C s_12_1: neg s_12_0
        let s_12_1: i128 = -s_12_0;
        // C s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // C s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: write-var return_value <= s_12_3
        fn_state.return_value = s_12_3;
        // N s_12_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_13_0: read-var ga#14146:i
        let s_13_0: i128 = fn_state.ga_14146;
        // D s_13_1: write-var return_value <= s_13_0
        fn_state.return_value = s_13_0;
        // N s_13_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_14_0: const #1u : u32
        let s_14_0: u32 = 1;
        // D s_14_1: read-var ga#14141:u32
        let s_14_1: u32 = fn_state.ga_14141;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // D s_14_3: not s_14_2
        let s_14_3: bool = !s_14_2;
        // N s_14_4: branch s_14_3 b24 b15
        if s_14_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_15_0: read-var walkparams.22:struct
        let s_15_0: u8 = fn_state.walkparams._22;
        // D s_15_1: write-var ga#14147 <= s_15_0
        fn_state.ga_14147 = s_15_0;
        // D s_15_2: read-var ga#14147:u8
        let s_15_2: u8 = fn_state.ga_14147;
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 2u16);
        // C s_15_4: const #0u : u8
        let s_15_4: u8 = 0;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 2u16);
        // D s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // D s_15_7: not s_15_6
        let s_15_7: bool = !s_15_6;
        // N s_15_8: branch s_15_7 b17 b16
        if s_15_7 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_16_0: const #3s : i
        let s_16_0: i128 = 3;
        // D s_16_1: write-var return_value <= s_16_0
        fn_state.return_value = s_16_0;
        // N s_16_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_17_0: read-var ga#14147:u8
        let s_17_0: u8 = fn_state.ga_14147;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #1u : u8
        let s_17_2: u8 = 1;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: not s_17_4
        let s_17_5: bool = !s_17_4;
        // N s_17_6: branch s_17_5 b19 b18
        if s_17_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_18_0: const #2s : i
        let s_18_0: i128 = 2;
        // D s_18_1: write-var return_value <= s_18_0
        fn_state.return_value = s_18_0;
        // N s_18_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_19_0: read-var ga#14147:u8
        let s_19_0: u8 = fn_state.ga_14147;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 2u16);
        // C s_19_2: const #2u : u8
        let s_19_2: u8 = 2;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 2u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: not s_19_4
        let s_19_5: bool = !s_19_4;
        // N s_19_6: branch s_19_5 b21 b20
        if s_19_5 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_20_0: const #1s : i
        let s_20_0: i128 = 1;
        // D s_20_1: write-var return_value <= s_20_0
        fn_state.return_value = s_20_0;
        // N s_20_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_21_0: read-var ga#14147:u8
        let s_21_0: u8 = fn_state.ga_14147;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 2u16);
        // C s_21_2: const #3u : u8
        let s_21_2: u8 = 3;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 2u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: not s_21_4
        let s_21_5: bool = !s_21_4;
        // N s_21_6: branch s_21_5 b23 b22
        if s_21_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_22_0: const #0s : i
        let s_22_0: i128 = 0;
        // D s_22_1: write-var return_value <= s_22_0
        fn_state.return_value = s_22_0;
        // N s_22_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_23_0: read-var ga#14148:i
        let s_23_0: i128 = fn_state.ga_14148;
        // D s_23_1: write-var return_value <= s_23_0
        fn_state.return_value = s_23_0;
        // N s_23_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_24_0: const #2u : u32
        let s_24_0: u32 = 2;
        // D s_24_1: read-var ga#14141:u32
        let s_24_1: u32 = fn_state.ga_14141;
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
        // D s_24_3: not s_24_2
        let s_24_3: bool = !s_24_2;
        // N s_24_4: branch s_24_3 b32 b25
        if s_24_3 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_25_0: read-var walkparams.22:struct
        let s_25_0: u8 = fn_state.walkparams._22;
        // D s_25_1: write-var ga#14149 <= s_25_0
        fn_state.ga_14149 = s_25_0;
        // D s_25_2: read-var ga#14149:u8
        let s_25_2: u8 = fn_state.ga_14149;
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 2u16);
        // C s_25_4: const #0u : u8
        let s_25_4: u8 = 0;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 2u16);
        // D s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // D s_25_7: not s_25_6
        let s_25_7: bool = !s_25_6;
        // N s_25_8: branch s_25_7 b27 b26
        if s_25_7 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_26_0: const #3s : i
        let s_26_0: i128 = 3;
        // D s_26_1: write-var return_value <= s_26_0
        fn_state.return_value = s_26_0;
        // N s_26_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_27_0: read-var ga#14149:u8
        let s_27_0: u8 = fn_state.ga_14149;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 2u16);
        // C s_27_2: const #1u : u8
        let s_27_2: u8 = 1;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 2u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: not s_27_4
        let s_27_5: bool = !s_27_4;
        // N s_27_6: branch s_27_5 b29 b28
        if s_27_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_28_0: const #2s : i
        let s_28_0: i128 = 2;
        // D s_28_1: write-var return_value <= s_28_0
        fn_state.return_value = s_28_0;
        // N s_28_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_29_0: read-var ga#14149:u8
        let s_29_0: u8 = fn_state.ga_14149;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 2u16);
        // C s_29_2: const #2u : u8
        let s_29_2: u8 = 2;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 2u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: not s_29_4
        let s_29_5: bool = !s_29_4;
        // N s_29_6: branch s_29_5 b31 b30
        if s_29_5 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_30_0: const #1s : i
        let s_30_0: i128 = 1;
        // D s_30_1: write-var return_value <= s_30_0
        fn_state.return_value = s_30_0;
        // N s_30_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_31_0: read-var ga#14150:i
        let s_31_0: i128 = fn_state.ga_14150;
        // D s_31_1: write-var return_value <= s_31_0
        fn_state.return_value = s_31_0;
        // N s_31_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_32_0: read-var ga#14151:i
        let s_32_0: i128 = fn_state.ga_14151;
        // D s_32_1: write-var return_value <= s_32_0
        fn_state.return_value = s_32_0;
        // N s_32_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_33_0: read-var walkparams.29:struct
        let s_33_0: u8 = fn_state.walkparams._29;
        // D s_33_1: call AArch64_IASize(s_33_0)
        let s_33_1: i128 = AArch64_IASize(state, tracer, s_33_0);
        // D s_33_2: read-var walkparams.26:struct
        let s_33_2: u32 = fn_state.walkparams._26;
        // D s_33_3: call TGxGranuleBits(s_33_2)
        let s_33_3: i128 = TGxGranuleBits(state, tracer, s_33_2);
        // C s_33_4: const #4s : i64
        let s_33_4: i64 = 4;
        // C s_33_5: cast zx s_33_4 -> i
        let s_33_5: i128 = (i128::try_from(s_33_4).unwrap());
        // D s_33_6: sub s_33_3 s_33_5
        let s_33_6: i128 = ((s_33_3) - (s_33_5));
        // C s_33_7: const #1s : i
        let s_33_7: i128 = 1;
        // D s_33_8: sub s_33_1 s_33_7
        let s_33_8: i128 = ((s_33_1) - (s_33_7));
        // D s_33_9: sub s_33_8 s_33_3
        let s_33_9: i128 = ((s_33_8) - (s_33_3));
        // D s_33_10: call fdiv_int(s_33_9, s_33_6)
        let s_33_10: i128 = fdiv_int(state, tracer, s_33_9, s_33_6);
        // C s_33_11: const #800u : u32
        let s_33_11: u32 = 800;
        // D s_33_12: read-reg s_33_11:i64
        let s_33_12: i64 = {
            let value = state.read_register::<i64>(s_33_11 as isize);
            tracer.read_register(s_33_11 as isize, value);
            value
        };
        // D s_33_13: cast zx s_33_12 -> i
        let s_33_13: i128 = (i128::try_from(s_33_12).unwrap());
        // D s_33_14: sub s_33_13 s_33_10
        let s_33_14: i128 = ((s_33_13) - (s_33_10));
        // D s_33_15: read-var walkparams.21:struct
        let s_33_15: u8 = fn_state.walkparams._21;
        // D s_33_16: cast zx s_33_15 -> bv
        let s_33_16: Bits = Bits::new(s_33_15 as u128, 2u16);
        // D s_33_17: cast zx s_33_16 -> i
        let s_33_17: i128 = (s_33_16.value() as i128);
        // D s_33_18: cast reint s_33_17 -> i64
        let s_33_18: i64 = (s_33_17 as i64);
        // D s_33_19: cast zx s_33_18 -> i
        let s_33_19: i128 = (i128::try_from(s_33_18).unwrap());
        // D s_33_20: add s_33_14 s_33_19
        let s_33_20: i128 = (s_33_14 + s_33_19);
        // D s_33_21: write-var return_value <= s_33_20
        fn_state.return_value = s_33_20;
        // N s_33_22: jump b4
        return block_4(state, tracer, fn_state);
    }
}
