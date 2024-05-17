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
pub fn PACSub<T: Tracer>(state: &mut State, tracer: &T, Tinput: u64) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        Toutput: u64,
        i: i64,
        ga_10575: u8,
        Tinput: u64,
    }
    let fn_state = FunctionState {
        Tinput,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #0s : i64
        let s_0_0: i64 = 0;
        // D s_0_1: write-var i <= s_0_0
        fn_state.i = s_0_0;
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // C s_1_1: const #15s : i64
        let s_1_1: i64 = 15;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b36 b2
        if s_1_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_2_0: const #4s : i
        let s_2_0: i128 = 4;
        // D s_2_1: read-var i:i64
        let s_2_1: i64 = fn_state.i;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: mul s_2_0 s_2_2
        let s_2_3: i128 = ((s_2_0) * (s_2_2));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // C s_2_5: const #4s : i
        let s_2_5: i128 = 4;
        // D s_2_6: read-var Tinput:u64
        let s_2_6: u64 = fn_state.Tinput;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 64u16);
        // D s_2_8: cast zx s_2_4 -> i
        let s_2_8: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_9: bit-extract s_2_7 s_2_8 s_2_5
        let s_2_9: Bits = (Bits::new(
            ((s_2_7) >> (s_2_8)).value(),
            u16::try_from(s_2_5).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u8
        let s_2_10: u8 = (s_2_9.value() as u8);
        // D s_2_11: write-var ga#10575 <= s_2_10
        fn_state.ga_10575 = s_2_10;
        // D s_2_12: read-var ga#10575:u8
        let s_2_12: u8 = fn_state.ga_10575;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 4u16);
        // C s_2_14: const #0u : u8
        let s_2_14: u8 = 0;
        // C s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 4u16);
        // D s_2_16: cmp-eq s_2_13 s_2_15
        let s_2_16: bool = ((s_2_13) == (s_2_15));
        // D s_2_17: not s_2_16
        let s_2_17: bool = !s_2_16;
        // N s_2_18: branch s_2_17 b5 b3
        if s_2_17 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_3_0: const #4s : i
        let s_3_0: i128 = 4;
        // D s_3_1: read-var i:i64
        let s_3_1: i64 = fn_state.i;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // C s_3_5: const #3s : i
        let s_3_5: i128 = 3;
        // D s_3_6: cast zx s_3_4 -> i
        let s_3_6: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_7: add s_3_6 s_3_5
        let s_3_7: i128 = (s_3_6 + s_3_5);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // C s_3_9: const #4s : i
        let s_3_9: i128 = 4;
        // D s_3_10: read-var i:i64
        let s_3_10: i64 = fn_state.i;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: mul s_3_9 s_3_11
        let s_3_12: i128 = ((s_3_9) * (s_3_11));
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: read-var Toutput:u64
        let s_3_14: u64 = fn_state.Toutput;
        // D s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 64u16);
        // D s_3_16: cast zx s_3_8 -> i
        let s_3_16: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_17: cast zx s_3_13 -> i
        let s_3_17: i128 = (i128::try_from(s_3_13).unwrap());
        // C s_3_18: const #11u : u8
        let s_3_18: u8 = 11;
        // C s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 4u16);
        // D s_3_20: sub s_3_16 s_3_17
        let s_3_20: i128 = ((s_3_16) - (s_3_17));
        // C s_3_21: const #1u : u64
        let s_3_21: u64 = 1;
        // C s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 64u16);
        // D s_3_23: lsl s_3_22 s_3_20
        let s_3_23: Bits = s_3_22 << s_3_20;
        // D s_3_24: sub s_3_23 s_3_22
        let s_3_24: Bits = ((s_3_23) - (s_3_22));
        // D s_3_25: and s_3_19 s_3_24
        let s_3_25: Bits = ((s_3_19) & (s_3_24));
        // D s_3_26: lsl s_3_25 s_3_17
        let s_3_26: Bits = s_3_25 << s_3_17;
        // D s_3_27: lsl s_3_24 s_3_17
        let s_3_27: Bits = s_3_24 << s_3_17;
        // D s_3_28: cmpl s_3_27
        let s_3_28: Bits = !s_3_27;
        // D s_3_29: and s_3_15 s_3_28
        let s_3_29: Bits = ((s_3_15) & (s_3_28));
        // D s_3_30: or s_3_29 s_3_26
        let s_3_30: Bits = ((s_3_29) | (s_3_26));
        // D s_3_31: cast reint s_3_30 -> u64
        let s_3_31: u64 = (s_3_30.value() as u64);
        // D s_3_32: write-var Toutput <= s_3_31
        fn_state.Toutput = s_3_31;
        // N s_3_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #1s : i64
        let s_4_1: i64 = 1;
        // D s_4_2: add s_4_0 s_4_1
        let s_4_2: i64 = (s_4_0 + s_4_1);
        // D s_4_3: write-var i <= s_4_2
        fn_state.i = s_4_2;
        // N s_4_4: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_5_0: read-var ga#10575:u8
        let s_5_0: u8 = fn_state.ga_10575;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // C s_5_2: const #1u : u8
        let s_5_2: u8 = 1;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
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
    ) -> u64 {
        // C s_6_0: const #4s : i
        let s_6_0: i128 = 4;
        // D s_6_1: read-var i:i64
        let s_6_1: i64 = fn_state.i;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: mul s_6_0 s_6_2
        let s_6_3: i128 = ((s_6_0) * (s_6_2));
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // C s_6_5: const #3s : i
        let s_6_5: i128 = 3;
        // D s_6_6: cast zx s_6_4 -> i
        let s_6_6: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_7: add s_6_6 s_6_5
        let s_6_7: i128 = (s_6_6 + s_6_5);
        // D s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // C s_6_9: const #4s : i
        let s_6_9: i128 = 4;
        // D s_6_10: read-var i:i64
        let s_6_10: i64 = fn_state.i;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: mul s_6_9 s_6_11
        let s_6_12: i128 = ((s_6_9) * (s_6_11));
        // D s_6_13: cast reint s_6_12 -> i64
        let s_6_13: i64 = (s_6_12 as i64);
        // D s_6_14: read-var Toutput:u64
        let s_6_14: u64 = fn_state.Toutput;
        // D s_6_15: cast zx s_6_14 -> bv
        let s_6_15: Bits = Bits::new(s_6_14 as u128, 64u16);
        // D s_6_16: cast zx s_6_8 -> i
        let s_6_16: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_17: cast zx s_6_13 -> i
        let s_6_17: i128 = (i128::try_from(s_6_13).unwrap());
        // C s_6_18: const #6u : u8
        let s_6_18: u8 = 6;
        // C s_6_19: cast zx s_6_18 -> bv
        let s_6_19: Bits = Bits::new(s_6_18 as u128, 4u16);
        // D s_6_20: sub s_6_16 s_6_17
        let s_6_20: i128 = ((s_6_16) - (s_6_17));
        // C s_6_21: const #1u : u64
        let s_6_21: u64 = 1;
        // C s_6_22: cast zx s_6_21 -> bv
        let s_6_22: Bits = Bits::new(s_6_21 as u128, 64u16);
        // D s_6_23: lsl s_6_22 s_6_20
        let s_6_23: Bits = s_6_22 << s_6_20;
        // D s_6_24: sub s_6_23 s_6_22
        let s_6_24: Bits = ((s_6_23) - (s_6_22));
        // D s_6_25: and s_6_19 s_6_24
        let s_6_25: Bits = ((s_6_19) & (s_6_24));
        // D s_6_26: lsl s_6_25 s_6_17
        let s_6_26: Bits = s_6_25 << s_6_17;
        // D s_6_27: lsl s_6_24 s_6_17
        let s_6_27: Bits = s_6_24 << s_6_17;
        // D s_6_28: cmpl s_6_27
        let s_6_28: Bits = !s_6_27;
        // D s_6_29: and s_6_15 s_6_28
        let s_6_29: Bits = ((s_6_15) & (s_6_28));
        // D s_6_30: or s_6_29 s_6_26
        let s_6_30: Bits = ((s_6_29) | (s_6_26));
        // D s_6_31: cast reint s_6_30 -> u64
        let s_6_31: u64 = (s_6_30.value() as u64);
        // D s_6_32: write-var Toutput <= s_6_31
        fn_state.Toutput = s_6_31;
        // N s_6_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var ga#10575:u8
        let s_7_0: u8 = fn_state.ga_10575;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 4u16);
        // C s_7_2: const #2u : u8
        let s_7_2: u8 = 2;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 4u16);
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
    ) -> u64 {
        // C s_8_0: const #4s : i
        let s_8_0: i128 = 4;
        // D s_8_1: read-var i:i64
        let s_8_1: i64 = fn_state.i;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: mul s_8_0 s_8_2
        let s_8_3: i128 = ((s_8_0) * (s_8_2));
        // D s_8_4: cast reint s_8_3 -> i64
        let s_8_4: i64 = (s_8_3 as i64);
        // C s_8_5: const #3s : i
        let s_8_5: i128 = 3;
        // D s_8_6: cast zx s_8_4 -> i
        let s_8_6: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_7: add s_8_6 s_8_5
        let s_8_7: i128 = (s_8_6 + s_8_5);
        // D s_8_8: cast reint s_8_7 -> i64
        let s_8_8: i64 = (s_8_7 as i64);
        // C s_8_9: const #4s : i
        let s_8_9: i128 = 4;
        // D s_8_10: read-var i:i64
        let s_8_10: i64 = fn_state.i;
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: mul s_8_9 s_8_11
        let s_8_12: i128 = ((s_8_9) * (s_8_11));
        // D s_8_13: cast reint s_8_12 -> i64
        let s_8_13: i64 = (s_8_12 as i64);
        // D s_8_14: read-var Toutput:u64
        let s_8_14: u64 = fn_state.Toutput;
        // D s_8_15: cast zx s_8_14 -> bv
        let s_8_15: Bits = Bits::new(s_8_14 as u128, 64u16);
        // D s_8_16: cast zx s_8_8 -> i
        let s_8_16: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_17: cast zx s_8_13 -> i
        let s_8_17: i128 = (i128::try_from(s_8_13).unwrap());
        // C s_8_18: const #8u : u8
        let s_8_18: u8 = 8;
        // C s_8_19: cast zx s_8_18 -> bv
        let s_8_19: Bits = Bits::new(s_8_18 as u128, 4u16);
        // D s_8_20: sub s_8_16 s_8_17
        let s_8_20: i128 = ((s_8_16) - (s_8_17));
        // C s_8_21: const #1u : u64
        let s_8_21: u64 = 1;
        // C s_8_22: cast zx s_8_21 -> bv
        let s_8_22: Bits = Bits::new(s_8_21 as u128, 64u16);
        // D s_8_23: lsl s_8_22 s_8_20
        let s_8_23: Bits = s_8_22 << s_8_20;
        // D s_8_24: sub s_8_23 s_8_22
        let s_8_24: Bits = ((s_8_23) - (s_8_22));
        // D s_8_25: and s_8_19 s_8_24
        let s_8_25: Bits = ((s_8_19) & (s_8_24));
        // D s_8_26: lsl s_8_25 s_8_17
        let s_8_26: Bits = s_8_25 << s_8_17;
        // D s_8_27: lsl s_8_24 s_8_17
        let s_8_27: Bits = s_8_24 << s_8_17;
        // D s_8_28: cmpl s_8_27
        let s_8_28: Bits = !s_8_27;
        // D s_8_29: and s_8_15 s_8_28
        let s_8_29: Bits = ((s_8_15) & (s_8_28));
        // D s_8_30: or s_8_29 s_8_26
        let s_8_30: Bits = ((s_8_29) | (s_8_26));
        // D s_8_31: cast reint s_8_30 -> u64
        let s_8_31: u64 = (s_8_30.value() as u64);
        // D s_8_32: write-var Toutput <= s_8_31
        fn_state.Toutput = s_8_31;
        // N s_8_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_9_0: read-var ga#10575:u8
        let s_9_0: u8 = fn_state.ga_10575;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 4u16);
        // C s_9_2: const #3u : u8
        let s_9_2: u8 = 3;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 4u16);
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
    ) -> u64 {
        // C s_10_0: const #4s : i
        let s_10_0: i128 = 4;
        // D s_10_1: read-var i:i64
        let s_10_1: i64 = fn_state.i;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: mul s_10_0 s_10_2
        let s_10_3: i128 = ((s_10_0) * (s_10_2));
        // D s_10_4: cast reint s_10_3 -> i64
        let s_10_4: i64 = (s_10_3 as i64);
        // C s_10_5: const #3s : i
        let s_10_5: i128 = 3;
        // D s_10_6: cast zx s_10_4 -> i
        let s_10_6: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_7: add s_10_6 s_10_5
        let s_10_7: i128 = (s_10_6 + s_10_5);
        // D s_10_8: cast reint s_10_7 -> i64
        let s_10_8: i64 = (s_10_7 as i64);
        // C s_10_9: const #4s : i
        let s_10_9: i128 = 4;
        // D s_10_10: read-var i:i64
        let s_10_10: i64 = fn_state.i;
        // D s_10_11: cast zx s_10_10 -> i
        let s_10_11: i128 = (i128::try_from(s_10_10).unwrap());
        // D s_10_12: mul s_10_9 s_10_11
        let s_10_12: i128 = ((s_10_9) * (s_10_11));
        // D s_10_13: cast reint s_10_12 -> i64
        let s_10_13: i64 = (s_10_12 as i64);
        // D s_10_14: read-var Toutput:u64
        let s_10_14: u64 = fn_state.Toutput;
        // D s_10_15: cast zx s_10_14 -> bv
        let s_10_15: Bits = Bits::new(s_10_14 as u128, 64u16);
        // D s_10_16: cast zx s_10_8 -> i
        let s_10_16: i128 = (i128::try_from(s_10_8).unwrap());
        // D s_10_17: cast zx s_10_13 -> i
        let s_10_17: i128 = (i128::try_from(s_10_13).unwrap());
        // C s_10_18: const #15u : u8
        let s_10_18: u8 = 15;
        // C s_10_19: cast zx s_10_18 -> bv
        let s_10_19: Bits = Bits::new(s_10_18 as u128, 4u16);
        // D s_10_20: sub s_10_16 s_10_17
        let s_10_20: i128 = ((s_10_16) - (s_10_17));
        // C s_10_21: const #1u : u64
        let s_10_21: u64 = 1;
        // C s_10_22: cast zx s_10_21 -> bv
        let s_10_22: Bits = Bits::new(s_10_21 as u128, 64u16);
        // D s_10_23: lsl s_10_22 s_10_20
        let s_10_23: Bits = s_10_22 << s_10_20;
        // D s_10_24: sub s_10_23 s_10_22
        let s_10_24: Bits = ((s_10_23) - (s_10_22));
        // D s_10_25: and s_10_19 s_10_24
        let s_10_25: Bits = ((s_10_19) & (s_10_24));
        // D s_10_26: lsl s_10_25 s_10_17
        let s_10_26: Bits = s_10_25 << s_10_17;
        // D s_10_27: lsl s_10_24 s_10_17
        let s_10_27: Bits = s_10_24 << s_10_17;
        // D s_10_28: cmpl s_10_27
        let s_10_28: Bits = !s_10_27;
        // D s_10_29: and s_10_15 s_10_28
        let s_10_29: Bits = ((s_10_15) & (s_10_28));
        // D s_10_30: or s_10_29 s_10_26
        let s_10_30: Bits = ((s_10_29) | (s_10_26));
        // D s_10_31: cast reint s_10_30 -> u64
        let s_10_31: u64 = (s_10_30.value() as u64);
        // D s_10_32: write-var Toutput <= s_10_31
        fn_state.Toutput = s_10_31;
        // N s_10_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_11_0: read-var ga#10575:u8
        let s_11_0: u8 = fn_state.ga_10575;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 4u16);
        // C s_11_2: const #4u : u8
        let s_11_2: u8 = 4;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 4u16);
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
    ) -> u64 {
        // C s_12_0: const #4s : i
        let s_12_0: i128 = 4;
        // D s_12_1: read-var i:i64
        let s_12_1: i64 = fn_state.i;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: mul s_12_0 s_12_2
        let s_12_3: i128 = ((s_12_0) * (s_12_2));
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // C s_12_5: const #3s : i
        let s_12_5: i128 = 3;
        // D s_12_6: cast zx s_12_4 -> i
        let s_12_6: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_7: add s_12_6 s_12_5
        let s_12_7: i128 = (s_12_6 + s_12_5);
        // D s_12_8: cast reint s_12_7 -> i64
        let s_12_8: i64 = (s_12_7 as i64);
        // C s_12_9: const #4s : i
        let s_12_9: i128 = 4;
        // D s_12_10: read-var i:i64
        let s_12_10: i64 = fn_state.i;
        // D s_12_11: cast zx s_12_10 -> i
        let s_12_11: i128 = (i128::try_from(s_12_10).unwrap());
        // D s_12_12: mul s_12_9 s_12_11
        let s_12_12: i128 = ((s_12_9) * (s_12_11));
        // D s_12_13: cast reint s_12_12 -> i64
        let s_12_13: i64 = (s_12_12 as i64);
        // D s_12_14: read-var Toutput:u64
        let s_12_14: u64 = fn_state.Toutput;
        // D s_12_15: cast zx s_12_14 -> bv
        let s_12_15: Bits = Bits::new(s_12_14 as u128, 64u16);
        // D s_12_16: cast zx s_12_8 -> i
        let s_12_16: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_17: cast zx s_12_13 -> i
        let s_12_17: i128 = (i128::try_from(s_12_13).unwrap());
        // C s_12_18: const #12u : u8
        let s_12_18: u8 = 12;
        // C s_12_19: cast zx s_12_18 -> bv
        let s_12_19: Bits = Bits::new(s_12_18 as u128, 4u16);
        // D s_12_20: sub s_12_16 s_12_17
        let s_12_20: i128 = ((s_12_16) - (s_12_17));
        // C s_12_21: const #1u : u64
        let s_12_21: u64 = 1;
        // C s_12_22: cast zx s_12_21 -> bv
        let s_12_22: Bits = Bits::new(s_12_21 as u128, 64u16);
        // D s_12_23: lsl s_12_22 s_12_20
        let s_12_23: Bits = s_12_22 << s_12_20;
        // D s_12_24: sub s_12_23 s_12_22
        let s_12_24: Bits = ((s_12_23) - (s_12_22));
        // D s_12_25: and s_12_19 s_12_24
        let s_12_25: Bits = ((s_12_19) & (s_12_24));
        // D s_12_26: lsl s_12_25 s_12_17
        let s_12_26: Bits = s_12_25 << s_12_17;
        // D s_12_27: lsl s_12_24 s_12_17
        let s_12_27: Bits = s_12_24 << s_12_17;
        // D s_12_28: cmpl s_12_27
        let s_12_28: Bits = !s_12_27;
        // D s_12_29: and s_12_15 s_12_28
        let s_12_29: Bits = ((s_12_15) & (s_12_28));
        // D s_12_30: or s_12_29 s_12_26
        let s_12_30: Bits = ((s_12_29) | (s_12_26));
        // D s_12_31: cast reint s_12_30 -> u64
        let s_12_31: u64 = (s_12_30.value() as u64);
        // D s_12_32: write-var Toutput <= s_12_31
        fn_state.Toutput = s_12_31;
        // N s_12_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_13_0: read-var ga#10575:u8
        let s_13_0: u8 = fn_state.ga_10575;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 4u16);
        // C s_13_2: const #5u : u8
        let s_13_2: u8 = 5;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 4u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b15 b14
        if s_13_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_14_0: const #4s : i
        let s_14_0: i128 = 4;
        // D s_14_1: read-var i:i64
        let s_14_1: i64 = fn_state.i;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: mul s_14_0 s_14_2
        let s_14_3: i128 = ((s_14_0) * (s_14_2));
        // D s_14_4: cast reint s_14_3 -> i64
        let s_14_4: i64 = (s_14_3 as i64);
        // C s_14_5: const #3s : i
        let s_14_5: i128 = 3;
        // D s_14_6: cast zx s_14_4 -> i
        let s_14_6: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_7: add s_14_6 s_14_5
        let s_14_7: i128 = (s_14_6 + s_14_5);
        // D s_14_8: cast reint s_14_7 -> i64
        let s_14_8: i64 = (s_14_7 as i64);
        // C s_14_9: const #4s : i
        let s_14_9: i128 = 4;
        // D s_14_10: read-var i:i64
        let s_14_10: i64 = fn_state.i;
        // D s_14_11: cast zx s_14_10 -> i
        let s_14_11: i128 = (i128::try_from(s_14_10).unwrap());
        // D s_14_12: mul s_14_9 s_14_11
        let s_14_12: i128 = ((s_14_9) * (s_14_11));
        // D s_14_13: cast reint s_14_12 -> i64
        let s_14_13: i64 = (s_14_12 as i64);
        // D s_14_14: read-var Toutput:u64
        let s_14_14: u64 = fn_state.Toutput;
        // D s_14_15: cast zx s_14_14 -> bv
        let s_14_15: Bits = Bits::new(s_14_14 as u128, 64u16);
        // D s_14_16: cast zx s_14_8 -> i
        let s_14_16: i128 = (i128::try_from(s_14_8).unwrap());
        // D s_14_17: cast zx s_14_13 -> i
        let s_14_17: i128 = (i128::try_from(s_14_13).unwrap());
        // C s_14_18: const #0u : u8
        let s_14_18: u8 = 0;
        // C s_14_19: cast zx s_14_18 -> bv
        let s_14_19: Bits = Bits::new(s_14_18 as u128, 4u16);
        // D s_14_20: sub s_14_16 s_14_17
        let s_14_20: i128 = ((s_14_16) - (s_14_17));
        // C s_14_21: const #1u : u64
        let s_14_21: u64 = 1;
        // C s_14_22: cast zx s_14_21 -> bv
        let s_14_22: Bits = Bits::new(s_14_21 as u128, 64u16);
        // D s_14_23: lsl s_14_22 s_14_20
        let s_14_23: Bits = s_14_22 << s_14_20;
        // D s_14_24: sub s_14_23 s_14_22
        let s_14_24: Bits = ((s_14_23) - (s_14_22));
        // D s_14_25: and s_14_19 s_14_24
        let s_14_25: Bits = ((s_14_19) & (s_14_24));
        // D s_14_26: lsl s_14_25 s_14_17
        let s_14_26: Bits = s_14_25 << s_14_17;
        // D s_14_27: lsl s_14_24 s_14_17
        let s_14_27: Bits = s_14_24 << s_14_17;
        // D s_14_28: cmpl s_14_27
        let s_14_28: Bits = !s_14_27;
        // D s_14_29: and s_14_15 s_14_28
        let s_14_29: Bits = ((s_14_15) & (s_14_28));
        // D s_14_30: or s_14_29 s_14_26
        let s_14_30: Bits = ((s_14_29) | (s_14_26));
        // D s_14_31: cast reint s_14_30 -> u64
        let s_14_31: u64 = (s_14_30.value() as u64);
        // D s_14_32: write-var Toutput <= s_14_31
        fn_state.Toutput = s_14_31;
        // N s_14_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_15_0: read-var ga#10575:u8
        let s_15_0: u8 = fn_state.ga_10575;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 4u16);
        // C s_15_2: const #6u : u8
        let s_15_2: u8 = 6;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 4u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: not s_15_4
        let s_15_5: bool = !s_15_4;
        // N s_15_6: branch s_15_5 b17 b16
        if s_15_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_16_0: const #4s : i
        let s_16_0: i128 = 4;
        // D s_16_1: read-var i:i64
        let s_16_1: i64 = fn_state.i;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: mul s_16_0 s_16_2
        let s_16_3: i128 = ((s_16_0) * (s_16_2));
        // D s_16_4: cast reint s_16_3 -> i64
        let s_16_4: i64 = (s_16_3 as i64);
        // C s_16_5: const #3s : i
        let s_16_5: i128 = 3;
        // D s_16_6: cast zx s_16_4 -> i
        let s_16_6: i128 = (i128::try_from(s_16_4).unwrap());
        // D s_16_7: add s_16_6 s_16_5
        let s_16_7: i128 = (s_16_6 + s_16_5);
        // D s_16_8: cast reint s_16_7 -> i64
        let s_16_8: i64 = (s_16_7 as i64);
        // C s_16_9: const #4s : i
        let s_16_9: i128 = 4;
        // D s_16_10: read-var i:i64
        let s_16_10: i64 = fn_state.i;
        // D s_16_11: cast zx s_16_10 -> i
        let s_16_11: i128 = (i128::try_from(s_16_10).unwrap());
        // D s_16_12: mul s_16_9 s_16_11
        let s_16_12: i128 = ((s_16_9) * (s_16_11));
        // D s_16_13: cast reint s_16_12 -> i64
        let s_16_13: i64 = (s_16_12 as i64);
        // D s_16_14: read-var Toutput:u64
        let s_16_14: u64 = fn_state.Toutput;
        // D s_16_15: cast zx s_16_14 -> bv
        let s_16_15: Bits = Bits::new(s_16_14 as u128, 64u16);
        // D s_16_16: cast zx s_16_8 -> i
        let s_16_16: i128 = (i128::try_from(s_16_8).unwrap());
        // D s_16_17: cast zx s_16_13 -> i
        let s_16_17: i128 = (i128::try_from(s_16_13).unwrap());
        // C s_16_18: const #9u : u8
        let s_16_18: u8 = 9;
        // C s_16_19: cast zx s_16_18 -> bv
        let s_16_19: Bits = Bits::new(s_16_18 as u128, 4u16);
        // D s_16_20: sub s_16_16 s_16_17
        let s_16_20: i128 = ((s_16_16) - (s_16_17));
        // C s_16_21: const #1u : u64
        let s_16_21: u64 = 1;
        // C s_16_22: cast zx s_16_21 -> bv
        let s_16_22: Bits = Bits::new(s_16_21 as u128, 64u16);
        // D s_16_23: lsl s_16_22 s_16_20
        let s_16_23: Bits = s_16_22 << s_16_20;
        // D s_16_24: sub s_16_23 s_16_22
        let s_16_24: Bits = ((s_16_23) - (s_16_22));
        // D s_16_25: and s_16_19 s_16_24
        let s_16_25: Bits = ((s_16_19) & (s_16_24));
        // D s_16_26: lsl s_16_25 s_16_17
        let s_16_26: Bits = s_16_25 << s_16_17;
        // D s_16_27: lsl s_16_24 s_16_17
        let s_16_27: Bits = s_16_24 << s_16_17;
        // D s_16_28: cmpl s_16_27
        let s_16_28: Bits = !s_16_27;
        // D s_16_29: and s_16_15 s_16_28
        let s_16_29: Bits = ((s_16_15) & (s_16_28));
        // D s_16_30: or s_16_29 s_16_26
        let s_16_30: Bits = ((s_16_29) | (s_16_26));
        // D s_16_31: cast reint s_16_30 -> u64
        let s_16_31: u64 = (s_16_30.value() as u64);
        // D s_16_32: write-var Toutput <= s_16_31
        fn_state.Toutput = s_16_31;
        // N s_16_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_17_0: read-var ga#10575:u8
        let s_17_0: u8 = fn_state.ga_10575;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 4u16);
        // C s_17_2: const #7u : u8
        let s_17_2: u8 = 7;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 4u16);
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
    ) -> u64 {
        // C s_18_0: const #4s : i
        let s_18_0: i128 = 4;
        // D s_18_1: read-var i:i64
        let s_18_1: i64 = fn_state.i;
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // D s_18_3: mul s_18_0 s_18_2
        let s_18_3: i128 = ((s_18_0) * (s_18_2));
        // D s_18_4: cast reint s_18_3 -> i64
        let s_18_4: i64 = (s_18_3 as i64);
        // C s_18_5: const #3s : i
        let s_18_5: i128 = 3;
        // D s_18_6: cast zx s_18_4 -> i
        let s_18_6: i128 = (i128::try_from(s_18_4).unwrap());
        // D s_18_7: add s_18_6 s_18_5
        let s_18_7: i128 = (s_18_6 + s_18_5);
        // D s_18_8: cast reint s_18_7 -> i64
        let s_18_8: i64 = (s_18_7 as i64);
        // C s_18_9: const #4s : i
        let s_18_9: i128 = 4;
        // D s_18_10: read-var i:i64
        let s_18_10: i64 = fn_state.i;
        // D s_18_11: cast zx s_18_10 -> i
        let s_18_11: i128 = (i128::try_from(s_18_10).unwrap());
        // D s_18_12: mul s_18_9 s_18_11
        let s_18_12: i128 = ((s_18_9) * (s_18_11));
        // D s_18_13: cast reint s_18_12 -> i64
        let s_18_13: i64 = (s_18_12 as i64);
        // D s_18_14: read-var Toutput:u64
        let s_18_14: u64 = fn_state.Toutput;
        // D s_18_15: cast zx s_18_14 -> bv
        let s_18_15: Bits = Bits::new(s_18_14 as u128, 64u16);
        // D s_18_16: cast zx s_18_8 -> i
        let s_18_16: i128 = (i128::try_from(s_18_8).unwrap());
        // D s_18_17: cast zx s_18_13 -> i
        let s_18_17: i128 = (i128::try_from(s_18_13).unwrap());
        // C s_18_18: const #14u : u8
        let s_18_18: u8 = 14;
        // C s_18_19: cast zx s_18_18 -> bv
        let s_18_19: Bits = Bits::new(s_18_18 as u128, 4u16);
        // D s_18_20: sub s_18_16 s_18_17
        let s_18_20: i128 = ((s_18_16) - (s_18_17));
        // C s_18_21: const #1u : u64
        let s_18_21: u64 = 1;
        // C s_18_22: cast zx s_18_21 -> bv
        let s_18_22: Bits = Bits::new(s_18_21 as u128, 64u16);
        // D s_18_23: lsl s_18_22 s_18_20
        let s_18_23: Bits = s_18_22 << s_18_20;
        // D s_18_24: sub s_18_23 s_18_22
        let s_18_24: Bits = ((s_18_23) - (s_18_22));
        // D s_18_25: and s_18_19 s_18_24
        let s_18_25: Bits = ((s_18_19) & (s_18_24));
        // D s_18_26: lsl s_18_25 s_18_17
        let s_18_26: Bits = s_18_25 << s_18_17;
        // D s_18_27: lsl s_18_24 s_18_17
        let s_18_27: Bits = s_18_24 << s_18_17;
        // D s_18_28: cmpl s_18_27
        let s_18_28: Bits = !s_18_27;
        // D s_18_29: and s_18_15 s_18_28
        let s_18_29: Bits = ((s_18_15) & (s_18_28));
        // D s_18_30: or s_18_29 s_18_26
        let s_18_30: Bits = ((s_18_29) | (s_18_26));
        // D s_18_31: cast reint s_18_30 -> u64
        let s_18_31: u64 = (s_18_30.value() as u64);
        // D s_18_32: write-var Toutput <= s_18_31
        fn_state.Toutput = s_18_31;
        // N s_18_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_19_0: read-var ga#10575:u8
        let s_19_0: u8 = fn_state.ga_10575;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 4u16);
        // C s_19_2: const #8u : u8
        let s_19_2: u8 = 8;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 4u16);
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
    ) -> u64 {
        // C s_20_0: const #4s : i
        let s_20_0: i128 = 4;
        // D s_20_1: read-var i:i64
        let s_20_1: i64 = fn_state.i;
        // D s_20_2: cast zx s_20_1 -> i
        let s_20_2: i128 = (i128::try_from(s_20_1).unwrap());
        // D s_20_3: mul s_20_0 s_20_2
        let s_20_3: i128 = ((s_20_0) * (s_20_2));
        // D s_20_4: cast reint s_20_3 -> i64
        let s_20_4: i64 = (s_20_3 as i64);
        // C s_20_5: const #3s : i
        let s_20_5: i128 = 3;
        // D s_20_6: cast zx s_20_4 -> i
        let s_20_6: i128 = (i128::try_from(s_20_4).unwrap());
        // D s_20_7: add s_20_6 s_20_5
        let s_20_7: i128 = (s_20_6 + s_20_5);
        // D s_20_8: cast reint s_20_7 -> i64
        let s_20_8: i64 = (s_20_7 as i64);
        // C s_20_9: const #4s : i
        let s_20_9: i128 = 4;
        // D s_20_10: read-var i:i64
        let s_20_10: i64 = fn_state.i;
        // D s_20_11: cast zx s_20_10 -> i
        let s_20_11: i128 = (i128::try_from(s_20_10).unwrap());
        // D s_20_12: mul s_20_9 s_20_11
        let s_20_12: i128 = ((s_20_9) * (s_20_11));
        // D s_20_13: cast reint s_20_12 -> i64
        let s_20_13: i64 = (s_20_12 as i64);
        // D s_20_14: read-var Toutput:u64
        let s_20_14: u64 = fn_state.Toutput;
        // D s_20_15: cast zx s_20_14 -> bv
        let s_20_15: Bits = Bits::new(s_20_14 as u128, 64u16);
        // D s_20_16: cast zx s_20_8 -> i
        let s_20_16: i128 = (i128::try_from(s_20_8).unwrap());
        // D s_20_17: cast zx s_20_13 -> i
        let s_20_17: i128 = (i128::try_from(s_20_13).unwrap());
        // C s_20_18: const #3u : u8
        let s_20_18: u8 = 3;
        // C s_20_19: cast zx s_20_18 -> bv
        let s_20_19: Bits = Bits::new(s_20_18 as u128, 4u16);
        // D s_20_20: sub s_20_16 s_20_17
        let s_20_20: i128 = ((s_20_16) - (s_20_17));
        // C s_20_21: const #1u : u64
        let s_20_21: u64 = 1;
        // C s_20_22: cast zx s_20_21 -> bv
        let s_20_22: Bits = Bits::new(s_20_21 as u128, 64u16);
        // D s_20_23: lsl s_20_22 s_20_20
        let s_20_23: Bits = s_20_22 << s_20_20;
        // D s_20_24: sub s_20_23 s_20_22
        let s_20_24: Bits = ((s_20_23) - (s_20_22));
        // D s_20_25: and s_20_19 s_20_24
        let s_20_25: Bits = ((s_20_19) & (s_20_24));
        // D s_20_26: lsl s_20_25 s_20_17
        let s_20_26: Bits = s_20_25 << s_20_17;
        // D s_20_27: lsl s_20_24 s_20_17
        let s_20_27: Bits = s_20_24 << s_20_17;
        // D s_20_28: cmpl s_20_27
        let s_20_28: Bits = !s_20_27;
        // D s_20_29: and s_20_15 s_20_28
        let s_20_29: Bits = ((s_20_15) & (s_20_28));
        // D s_20_30: or s_20_29 s_20_26
        let s_20_30: Bits = ((s_20_29) | (s_20_26));
        // D s_20_31: cast reint s_20_30 -> u64
        let s_20_31: u64 = (s_20_30.value() as u64);
        // D s_20_32: write-var Toutput <= s_20_31
        fn_state.Toutput = s_20_31;
        // N s_20_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_21_0: read-var ga#10575:u8
        let s_21_0: u8 = fn_state.ga_10575;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 4u16);
        // C s_21_2: const #9u : u8
        let s_21_2: u8 = 9;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 4u16);
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
    ) -> u64 {
        // C s_22_0: const #4s : i
        let s_22_0: i128 = 4;
        // D s_22_1: read-var i:i64
        let s_22_1: i64 = fn_state.i;
        // D s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (i128::try_from(s_22_1).unwrap());
        // D s_22_3: mul s_22_0 s_22_2
        let s_22_3: i128 = ((s_22_0) * (s_22_2));
        // D s_22_4: cast reint s_22_3 -> i64
        let s_22_4: i64 = (s_22_3 as i64);
        // C s_22_5: const #3s : i
        let s_22_5: i128 = 3;
        // D s_22_6: cast zx s_22_4 -> i
        let s_22_6: i128 = (i128::try_from(s_22_4).unwrap());
        // D s_22_7: add s_22_6 s_22_5
        let s_22_7: i128 = (s_22_6 + s_22_5);
        // D s_22_8: cast reint s_22_7 -> i64
        let s_22_8: i64 = (s_22_7 as i64);
        // C s_22_9: const #4s : i
        let s_22_9: i128 = 4;
        // D s_22_10: read-var i:i64
        let s_22_10: i64 = fn_state.i;
        // D s_22_11: cast zx s_22_10 -> i
        let s_22_11: i128 = (i128::try_from(s_22_10).unwrap());
        // D s_22_12: mul s_22_9 s_22_11
        let s_22_12: i128 = ((s_22_9) * (s_22_11));
        // D s_22_13: cast reint s_22_12 -> i64
        let s_22_13: i64 = (s_22_12 as i64);
        // D s_22_14: read-var Toutput:u64
        let s_22_14: u64 = fn_state.Toutput;
        // D s_22_15: cast zx s_22_14 -> bv
        let s_22_15: Bits = Bits::new(s_22_14 as u128, 64u16);
        // D s_22_16: cast zx s_22_8 -> i
        let s_22_16: i128 = (i128::try_from(s_22_8).unwrap());
        // D s_22_17: cast zx s_22_13 -> i
        let s_22_17: i128 = (i128::try_from(s_22_13).unwrap());
        // C s_22_18: const #7u : u8
        let s_22_18: u8 = 7;
        // C s_22_19: cast zx s_22_18 -> bv
        let s_22_19: Bits = Bits::new(s_22_18 as u128, 4u16);
        // D s_22_20: sub s_22_16 s_22_17
        let s_22_20: i128 = ((s_22_16) - (s_22_17));
        // C s_22_21: const #1u : u64
        let s_22_21: u64 = 1;
        // C s_22_22: cast zx s_22_21 -> bv
        let s_22_22: Bits = Bits::new(s_22_21 as u128, 64u16);
        // D s_22_23: lsl s_22_22 s_22_20
        let s_22_23: Bits = s_22_22 << s_22_20;
        // D s_22_24: sub s_22_23 s_22_22
        let s_22_24: Bits = ((s_22_23) - (s_22_22));
        // D s_22_25: and s_22_19 s_22_24
        let s_22_25: Bits = ((s_22_19) & (s_22_24));
        // D s_22_26: lsl s_22_25 s_22_17
        let s_22_26: Bits = s_22_25 << s_22_17;
        // D s_22_27: lsl s_22_24 s_22_17
        let s_22_27: Bits = s_22_24 << s_22_17;
        // D s_22_28: cmpl s_22_27
        let s_22_28: Bits = !s_22_27;
        // D s_22_29: and s_22_15 s_22_28
        let s_22_29: Bits = ((s_22_15) & (s_22_28));
        // D s_22_30: or s_22_29 s_22_26
        let s_22_30: Bits = ((s_22_29) | (s_22_26));
        // D s_22_31: cast reint s_22_30 -> u64
        let s_22_31: u64 = (s_22_30.value() as u64);
        // D s_22_32: write-var Toutput <= s_22_31
        fn_state.Toutput = s_22_31;
        // N s_22_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_23_0: read-var ga#10575:u8
        let s_23_0: u8 = fn_state.ga_10575;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 4u16);
        // C s_23_2: const #10u : u8
        let s_23_2: u8 = 10;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 4u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: not s_23_4
        let s_23_5: bool = !s_23_4;
        // N s_23_6: branch s_23_5 b25 b24
        if s_23_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_24_0: const #4s : i
        let s_24_0: i128 = 4;
        // D s_24_1: read-var i:i64
        let s_24_1: i64 = fn_state.i;
        // D s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (i128::try_from(s_24_1).unwrap());
        // D s_24_3: mul s_24_0 s_24_2
        let s_24_3: i128 = ((s_24_0) * (s_24_2));
        // D s_24_4: cast reint s_24_3 -> i64
        let s_24_4: i64 = (s_24_3 as i64);
        // C s_24_5: const #3s : i
        let s_24_5: i128 = 3;
        // D s_24_6: cast zx s_24_4 -> i
        let s_24_6: i128 = (i128::try_from(s_24_4).unwrap());
        // D s_24_7: add s_24_6 s_24_5
        let s_24_7: i128 = (s_24_6 + s_24_5);
        // D s_24_8: cast reint s_24_7 -> i64
        let s_24_8: i64 = (s_24_7 as i64);
        // C s_24_9: const #4s : i
        let s_24_9: i128 = 4;
        // D s_24_10: read-var i:i64
        let s_24_10: i64 = fn_state.i;
        // D s_24_11: cast zx s_24_10 -> i
        let s_24_11: i128 = (i128::try_from(s_24_10).unwrap());
        // D s_24_12: mul s_24_9 s_24_11
        let s_24_12: i128 = ((s_24_9) * (s_24_11));
        // D s_24_13: cast reint s_24_12 -> i64
        let s_24_13: i64 = (s_24_12 as i64);
        // D s_24_14: read-var Toutput:u64
        let s_24_14: u64 = fn_state.Toutput;
        // D s_24_15: cast zx s_24_14 -> bv
        let s_24_15: Bits = Bits::new(s_24_14 as u128, 64u16);
        // D s_24_16: cast zx s_24_8 -> i
        let s_24_16: i128 = (i128::try_from(s_24_8).unwrap());
        // D s_24_17: cast zx s_24_13 -> i
        let s_24_17: i128 = (i128::try_from(s_24_13).unwrap());
        // C s_24_18: const #4u : u8
        let s_24_18: u8 = 4;
        // C s_24_19: cast zx s_24_18 -> bv
        let s_24_19: Bits = Bits::new(s_24_18 as u128, 4u16);
        // D s_24_20: sub s_24_16 s_24_17
        let s_24_20: i128 = ((s_24_16) - (s_24_17));
        // C s_24_21: const #1u : u64
        let s_24_21: u64 = 1;
        // C s_24_22: cast zx s_24_21 -> bv
        let s_24_22: Bits = Bits::new(s_24_21 as u128, 64u16);
        // D s_24_23: lsl s_24_22 s_24_20
        let s_24_23: Bits = s_24_22 << s_24_20;
        // D s_24_24: sub s_24_23 s_24_22
        let s_24_24: Bits = ((s_24_23) - (s_24_22));
        // D s_24_25: and s_24_19 s_24_24
        let s_24_25: Bits = ((s_24_19) & (s_24_24));
        // D s_24_26: lsl s_24_25 s_24_17
        let s_24_26: Bits = s_24_25 << s_24_17;
        // D s_24_27: lsl s_24_24 s_24_17
        let s_24_27: Bits = s_24_24 << s_24_17;
        // D s_24_28: cmpl s_24_27
        let s_24_28: Bits = !s_24_27;
        // D s_24_29: and s_24_15 s_24_28
        let s_24_29: Bits = ((s_24_15) & (s_24_28));
        // D s_24_30: or s_24_29 s_24_26
        let s_24_30: Bits = ((s_24_29) | (s_24_26));
        // D s_24_31: cast reint s_24_30 -> u64
        let s_24_31: u64 = (s_24_30.value() as u64);
        // D s_24_32: write-var Toutput <= s_24_31
        fn_state.Toutput = s_24_31;
        // N s_24_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_25_0: read-var ga#10575:u8
        let s_25_0: u8 = fn_state.ga_10575;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 4u16);
        // C s_25_2: const #11u : u8
        let s_25_2: u8 = 11;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 4u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: not s_25_4
        let s_25_5: bool = !s_25_4;
        // N s_25_6: branch s_25_5 b27 b26
        if s_25_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_26_0: const #4s : i
        let s_26_0: i128 = 4;
        // D s_26_1: read-var i:i64
        let s_26_1: i64 = fn_state.i;
        // D s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (i128::try_from(s_26_1).unwrap());
        // D s_26_3: mul s_26_0 s_26_2
        let s_26_3: i128 = ((s_26_0) * (s_26_2));
        // D s_26_4: cast reint s_26_3 -> i64
        let s_26_4: i64 = (s_26_3 as i64);
        // C s_26_5: const #3s : i
        let s_26_5: i128 = 3;
        // D s_26_6: cast zx s_26_4 -> i
        let s_26_6: i128 = (i128::try_from(s_26_4).unwrap());
        // D s_26_7: add s_26_6 s_26_5
        let s_26_7: i128 = (s_26_6 + s_26_5);
        // D s_26_8: cast reint s_26_7 -> i64
        let s_26_8: i64 = (s_26_7 as i64);
        // C s_26_9: const #4s : i
        let s_26_9: i128 = 4;
        // D s_26_10: read-var i:i64
        let s_26_10: i64 = fn_state.i;
        // D s_26_11: cast zx s_26_10 -> i
        let s_26_11: i128 = (i128::try_from(s_26_10).unwrap());
        // D s_26_12: mul s_26_9 s_26_11
        let s_26_12: i128 = ((s_26_9) * (s_26_11));
        // D s_26_13: cast reint s_26_12 -> i64
        let s_26_13: i64 = (s_26_12 as i64);
        // D s_26_14: read-var Toutput:u64
        let s_26_14: u64 = fn_state.Toutput;
        // D s_26_15: cast zx s_26_14 -> bv
        let s_26_15: Bits = Bits::new(s_26_14 as u128, 64u16);
        // D s_26_16: cast zx s_26_8 -> i
        let s_26_16: i128 = (i128::try_from(s_26_8).unwrap());
        // D s_26_17: cast zx s_26_13 -> i
        let s_26_17: i128 = (i128::try_from(s_26_13).unwrap());
        // C s_26_18: const #5u : u8
        let s_26_18: u8 = 5;
        // C s_26_19: cast zx s_26_18 -> bv
        let s_26_19: Bits = Bits::new(s_26_18 as u128, 4u16);
        // D s_26_20: sub s_26_16 s_26_17
        let s_26_20: i128 = ((s_26_16) - (s_26_17));
        // C s_26_21: const #1u : u64
        let s_26_21: u64 = 1;
        // C s_26_22: cast zx s_26_21 -> bv
        let s_26_22: Bits = Bits::new(s_26_21 as u128, 64u16);
        // D s_26_23: lsl s_26_22 s_26_20
        let s_26_23: Bits = s_26_22 << s_26_20;
        // D s_26_24: sub s_26_23 s_26_22
        let s_26_24: Bits = ((s_26_23) - (s_26_22));
        // D s_26_25: and s_26_19 s_26_24
        let s_26_25: Bits = ((s_26_19) & (s_26_24));
        // D s_26_26: lsl s_26_25 s_26_17
        let s_26_26: Bits = s_26_25 << s_26_17;
        // D s_26_27: lsl s_26_24 s_26_17
        let s_26_27: Bits = s_26_24 << s_26_17;
        // D s_26_28: cmpl s_26_27
        let s_26_28: Bits = !s_26_27;
        // D s_26_29: and s_26_15 s_26_28
        let s_26_29: Bits = ((s_26_15) & (s_26_28));
        // D s_26_30: or s_26_29 s_26_26
        let s_26_30: Bits = ((s_26_29) | (s_26_26));
        // D s_26_31: cast reint s_26_30 -> u64
        let s_26_31: u64 = (s_26_30.value() as u64);
        // D s_26_32: write-var Toutput <= s_26_31
        fn_state.Toutput = s_26_31;
        // N s_26_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_27_0: read-var ga#10575:u8
        let s_27_0: u8 = fn_state.ga_10575;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 4u16);
        // C s_27_2: const #12u : u8
        let s_27_2: u8 = 12;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 4u16);
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
    ) -> u64 {
        // C s_28_0: const #4s : i
        let s_28_0: i128 = 4;
        // D s_28_1: read-var i:i64
        let s_28_1: i64 = fn_state.i;
        // D s_28_2: cast zx s_28_1 -> i
        let s_28_2: i128 = (i128::try_from(s_28_1).unwrap());
        // D s_28_3: mul s_28_0 s_28_2
        let s_28_3: i128 = ((s_28_0) * (s_28_2));
        // D s_28_4: cast reint s_28_3 -> i64
        let s_28_4: i64 = (s_28_3 as i64);
        // C s_28_5: const #3s : i
        let s_28_5: i128 = 3;
        // D s_28_6: cast zx s_28_4 -> i
        let s_28_6: i128 = (i128::try_from(s_28_4).unwrap());
        // D s_28_7: add s_28_6 s_28_5
        let s_28_7: i128 = (s_28_6 + s_28_5);
        // D s_28_8: cast reint s_28_7 -> i64
        let s_28_8: i64 = (s_28_7 as i64);
        // C s_28_9: const #4s : i
        let s_28_9: i128 = 4;
        // D s_28_10: read-var i:i64
        let s_28_10: i64 = fn_state.i;
        // D s_28_11: cast zx s_28_10 -> i
        let s_28_11: i128 = (i128::try_from(s_28_10).unwrap());
        // D s_28_12: mul s_28_9 s_28_11
        let s_28_12: i128 = ((s_28_9) * (s_28_11));
        // D s_28_13: cast reint s_28_12 -> i64
        let s_28_13: i64 = (s_28_12 as i64);
        // D s_28_14: read-var Toutput:u64
        let s_28_14: u64 = fn_state.Toutput;
        // D s_28_15: cast zx s_28_14 -> bv
        let s_28_15: Bits = Bits::new(s_28_14 as u128, 64u16);
        // D s_28_16: cast zx s_28_8 -> i
        let s_28_16: i128 = (i128::try_from(s_28_8).unwrap());
        // D s_28_17: cast zx s_28_13 -> i
        let s_28_17: i128 = (i128::try_from(s_28_13).unwrap());
        // C s_28_18: const #13u : u8
        let s_28_18: u8 = 13;
        // C s_28_19: cast zx s_28_18 -> bv
        let s_28_19: Bits = Bits::new(s_28_18 as u128, 4u16);
        // D s_28_20: sub s_28_16 s_28_17
        let s_28_20: i128 = ((s_28_16) - (s_28_17));
        // C s_28_21: const #1u : u64
        let s_28_21: u64 = 1;
        // C s_28_22: cast zx s_28_21 -> bv
        let s_28_22: Bits = Bits::new(s_28_21 as u128, 64u16);
        // D s_28_23: lsl s_28_22 s_28_20
        let s_28_23: Bits = s_28_22 << s_28_20;
        // D s_28_24: sub s_28_23 s_28_22
        let s_28_24: Bits = ((s_28_23) - (s_28_22));
        // D s_28_25: and s_28_19 s_28_24
        let s_28_25: Bits = ((s_28_19) & (s_28_24));
        // D s_28_26: lsl s_28_25 s_28_17
        let s_28_26: Bits = s_28_25 << s_28_17;
        // D s_28_27: lsl s_28_24 s_28_17
        let s_28_27: Bits = s_28_24 << s_28_17;
        // D s_28_28: cmpl s_28_27
        let s_28_28: Bits = !s_28_27;
        // D s_28_29: and s_28_15 s_28_28
        let s_28_29: Bits = ((s_28_15) & (s_28_28));
        // D s_28_30: or s_28_29 s_28_26
        let s_28_30: Bits = ((s_28_29) | (s_28_26));
        // D s_28_31: cast reint s_28_30 -> u64
        let s_28_31: u64 = (s_28_30.value() as u64);
        // D s_28_32: write-var Toutput <= s_28_31
        fn_state.Toutput = s_28_31;
        // N s_28_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_29_0: read-var ga#10575:u8
        let s_29_0: u8 = fn_state.ga_10575;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 4u16);
        // C s_29_2: const #13u : u8
        let s_29_2: u8 = 13;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 4u16);
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
    ) -> u64 {
        // C s_30_0: const #4s : i
        let s_30_0: i128 = 4;
        // D s_30_1: read-var i:i64
        let s_30_1: i64 = fn_state.i;
        // D s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (i128::try_from(s_30_1).unwrap());
        // D s_30_3: mul s_30_0 s_30_2
        let s_30_3: i128 = ((s_30_0) * (s_30_2));
        // D s_30_4: cast reint s_30_3 -> i64
        let s_30_4: i64 = (s_30_3 as i64);
        // C s_30_5: const #3s : i
        let s_30_5: i128 = 3;
        // D s_30_6: cast zx s_30_4 -> i
        let s_30_6: i128 = (i128::try_from(s_30_4).unwrap());
        // D s_30_7: add s_30_6 s_30_5
        let s_30_7: i128 = (s_30_6 + s_30_5);
        // D s_30_8: cast reint s_30_7 -> i64
        let s_30_8: i64 = (s_30_7 as i64);
        // C s_30_9: const #4s : i
        let s_30_9: i128 = 4;
        // D s_30_10: read-var i:i64
        let s_30_10: i64 = fn_state.i;
        // D s_30_11: cast zx s_30_10 -> i
        let s_30_11: i128 = (i128::try_from(s_30_10).unwrap());
        // D s_30_12: mul s_30_9 s_30_11
        let s_30_12: i128 = ((s_30_9) * (s_30_11));
        // D s_30_13: cast reint s_30_12 -> i64
        let s_30_13: i64 = (s_30_12 as i64);
        // D s_30_14: read-var Toutput:u64
        let s_30_14: u64 = fn_state.Toutput;
        // D s_30_15: cast zx s_30_14 -> bv
        let s_30_15: Bits = Bits::new(s_30_14 as u128, 64u16);
        // D s_30_16: cast zx s_30_8 -> i
        let s_30_16: i128 = (i128::try_from(s_30_8).unwrap());
        // D s_30_17: cast zx s_30_13 -> i
        let s_30_17: i128 = (i128::try_from(s_30_13).unwrap());
        // C s_30_18: const #2u : u8
        let s_30_18: u8 = 2;
        // C s_30_19: cast zx s_30_18 -> bv
        let s_30_19: Bits = Bits::new(s_30_18 as u128, 4u16);
        // D s_30_20: sub s_30_16 s_30_17
        let s_30_20: i128 = ((s_30_16) - (s_30_17));
        // C s_30_21: const #1u : u64
        let s_30_21: u64 = 1;
        // C s_30_22: cast zx s_30_21 -> bv
        let s_30_22: Bits = Bits::new(s_30_21 as u128, 64u16);
        // D s_30_23: lsl s_30_22 s_30_20
        let s_30_23: Bits = s_30_22 << s_30_20;
        // D s_30_24: sub s_30_23 s_30_22
        let s_30_24: Bits = ((s_30_23) - (s_30_22));
        // D s_30_25: and s_30_19 s_30_24
        let s_30_25: Bits = ((s_30_19) & (s_30_24));
        // D s_30_26: lsl s_30_25 s_30_17
        let s_30_26: Bits = s_30_25 << s_30_17;
        // D s_30_27: lsl s_30_24 s_30_17
        let s_30_27: Bits = s_30_24 << s_30_17;
        // D s_30_28: cmpl s_30_27
        let s_30_28: Bits = !s_30_27;
        // D s_30_29: and s_30_15 s_30_28
        let s_30_29: Bits = ((s_30_15) & (s_30_28));
        // D s_30_30: or s_30_29 s_30_26
        let s_30_30: Bits = ((s_30_29) | (s_30_26));
        // D s_30_31: cast reint s_30_30 -> u64
        let s_30_31: u64 = (s_30_30.value() as u64);
        // D s_30_32: write-var Toutput <= s_30_31
        fn_state.Toutput = s_30_31;
        // N s_30_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_31_0: read-var ga#10575:u8
        let s_31_0: u8 = fn_state.ga_10575;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 4u16);
        // C s_31_2: const #14u : u8
        let s_31_2: u8 = 14;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 4u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: not s_31_4
        let s_31_5: bool = !s_31_4;
        // N s_31_6: branch s_31_5 b33 b32
        if s_31_5 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_32_0: const #4s : i
        let s_32_0: i128 = 4;
        // D s_32_1: read-var i:i64
        let s_32_1: i64 = fn_state.i;
        // D s_32_2: cast zx s_32_1 -> i
        let s_32_2: i128 = (i128::try_from(s_32_1).unwrap());
        // D s_32_3: mul s_32_0 s_32_2
        let s_32_3: i128 = ((s_32_0) * (s_32_2));
        // D s_32_4: cast reint s_32_3 -> i64
        let s_32_4: i64 = (s_32_3 as i64);
        // C s_32_5: const #3s : i
        let s_32_5: i128 = 3;
        // D s_32_6: cast zx s_32_4 -> i
        let s_32_6: i128 = (i128::try_from(s_32_4).unwrap());
        // D s_32_7: add s_32_6 s_32_5
        let s_32_7: i128 = (s_32_6 + s_32_5);
        // D s_32_8: cast reint s_32_7 -> i64
        let s_32_8: i64 = (s_32_7 as i64);
        // C s_32_9: const #4s : i
        let s_32_9: i128 = 4;
        // D s_32_10: read-var i:i64
        let s_32_10: i64 = fn_state.i;
        // D s_32_11: cast zx s_32_10 -> i
        let s_32_11: i128 = (i128::try_from(s_32_10).unwrap());
        // D s_32_12: mul s_32_9 s_32_11
        let s_32_12: i128 = ((s_32_9) * (s_32_11));
        // D s_32_13: cast reint s_32_12 -> i64
        let s_32_13: i64 = (s_32_12 as i64);
        // D s_32_14: read-var Toutput:u64
        let s_32_14: u64 = fn_state.Toutput;
        // D s_32_15: cast zx s_32_14 -> bv
        let s_32_15: Bits = Bits::new(s_32_14 as u128, 64u16);
        // D s_32_16: cast zx s_32_8 -> i
        let s_32_16: i128 = (i128::try_from(s_32_8).unwrap());
        // D s_32_17: cast zx s_32_13 -> i
        let s_32_17: i128 = (i128::try_from(s_32_13).unwrap());
        // C s_32_18: const #1u : u8
        let s_32_18: u8 = 1;
        // C s_32_19: cast zx s_32_18 -> bv
        let s_32_19: Bits = Bits::new(s_32_18 as u128, 4u16);
        // D s_32_20: sub s_32_16 s_32_17
        let s_32_20: i128 = ((s_32_16) - (s_32_17));
        // C s_32_21: const #1u : u64
        let s_32_21: u64 = 1;
        // C s_32_22: cast zx s_32_21 -> bv
        let s_32_22: Bits = Bits::new(s_32_21 as u128, 64u16);
        // D s_32_23: lsl s_32_22 s_32_20
        let s_32_23: Bits = s_32_22 << s_32_20;
        // D s_32_24: sub s_32_23 s_32_22
        let s_32_24: Bits = ((s_32_23) - (s_32_22));
        // D s_32_25: and s_32_19 s_32_24
        let s_32_25: Bits = ((s_32_19) & (s_32_24));
        // D s_32_26: lsl s_32_25 s_32_17
        let s_32_26: Bits = s_32_25 << s_32_17;
        // D s_32_27: lsl s_32_24 s_32_17
        let s_32_27: Bits = s_32_24 << s_32_17;
        // D s_32_28: cmpl s_32_27
        let s_32_28: Bits = !s_32_27;
        // D s_32_29: and s_32_15 s_32_28
        let s_32_29: Bits = ((s_32_15) & (s_32_28));
        // D s_32_30: or s_32_29 s_32_26
        let s_32_30: Bits = ((s_32_29) | (s_32_26));
        // D s_32_31: cast reint s_32_30 -> u64
        let s_32_31: u64 = (s_32_30.value() as u64);
        // D s_32_32: write-var Toutput <= s_32_31
        fn_state.Toutput = s_32_31;
        // N s_32_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_33_0: read-var ga#10575:u8
        let s_33_0: u8 = fn_state.ga_10575;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 4u16);
        // C s_33_2: const #15u : u8
        let s_33_2: u8 = 15;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 4u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: not s_33_4
        let s_33_5: bool = !s_33_4;
        // N s_33_6: branch s_33_5 b35 b34
        if s_33_5 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_34_0: const #4s : i
        let s_34_0: i128 = 4;
        // D s_34_1: read-var i:i64
        let s_34_1: i64 = fn_state.i;
        // D s_34_2: cast zx s_34_1 -> i
        let s_34_2: i128 = (i128::try_from(s_34_1).unwrap());
        // D s_34_3: mul s_34_0 s_34_2
        let s_34_3: i128 = ((s_34_0) * (s_34_2));
        // D s_34_4: cast reint s_34_3 -> i64
        let s_34_4: i64 = (s_34_3 as i64);
        // C s_34_5: const #3s : i
        let s_34_5: i128 = 3;
        // D s_34_6: cast zx s_34_4 -> i
        let s_34_6: i128 = (i128::try_from(s_34_4).unwrap());
        // D s_34_7: add s_34_6 s_34_5
        let s_34_7: i128 = (s_34_6 + s_34_5);
        // D s_34_8: cast reint s_34_7 -> i64
        let s_34_8: i64 = (s_34_7 as i64);
        // C s_34_9: const #4s : i
        let s_34_9: i128 = 4;
        // D s_34_10: read-var i:i64
        let s_34_10: i64 = fn_state.i;
        // D s_34_11: cast zx s_34_10 -> i
        let s_34_11: i128 = (i128::try_from(s_34_10).unwrap());
        // D s_34_12: mul s_34_9 s_34_11
        let s_34_12: i128 = ((s_34_9) * (s_34_11));
        // D s_34_13: cast reint s_34_12 -> i64
        let s_34_13: i64 = (s_34_12 as i64);
        // D s_34_14: read-var Toutput:u64
        let s_34_14: u64 = fn_state.Toutput;
        // D s_34_15: cast zx s_34_14 -> bv
        let s_34_15: Bits = Bits::new(s_34_14 as u128, 64u16);
        // D s_34_16: cast zx s_34_8 -> i
        let s_34_16: i128 = (i128::try_from(s_34_8).unwrap());
        // D s_34_17: cast zx s_34_13 -> i
        let s_34_17: i128 = (i128::try_from(s_34_13).unwrap());
        // C s_34_18: const #10u : u8
        let s_34_18: u8 = 10;
        // C s_34_19: cast zx s_34_18 -> bv
        let s_34_19: Bits = Bits::new(s_34_18 as u128, 4u16);
        // D s_34_20: sub s_34_16 s_34_17
        let s_34_20: i128 = ((s_34_16) - (s_34_17));
        // C s_34_21: const #1u : u64
        let s_34_21: u64 = 1;
        // C s_34_22: cast zx s_34_21 -> bv
        let s_34_22: Bits = Bits::new(s_34_21 as u128, 64u16);
        // D s_34_23: lsl s_34_22 s_34_20
        let s_34_23: Bits = s_34_22 << s_34_20;
        // D s_34_24: sub s_34_23 s_34_22
        let s_34_24: Bits = ((s_34_23) - (s_34_22));
        // D s_34_25: and s_34_19 s_34_24
        let s_34_25: Bits = ((s_34_19) & (s_34_24));
        // D s_34_26: lsl s_34_25 s_34_17
        let s_34_26: Bits = s_34_25 << s_34_17;
        // D s_34_27: lsl s_34_24 s_34_17
        let s_34_27: Bits = s_34_24 << s_34_17;
        // D s_34_28: cmpl s_34_27
        let s_34_28: Bits = !s_34_27;
        // D s_34_29: and s_34_15 s_34_28
        let s_34_29: Bits = ((s_34_15) & (s_34_28));
        // D s_34_30: or s_34_29 s_34_26
        let s_34_30: Bits = ((s_34_29) | (s_34_26));
        // D s_34_31: cast reint s_34_30 -> u64
        let s_34_31: u64 = (s_34_30.value() as u64);
        // D s_34_32: write-var Toutput <= s_34_31
        fn_state.Toutput = s_34_31;
        // N s_34_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_35_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_36_0: read-var Toutput:u64
        let s_36_0: u64 = fn_state.Toutput;
        // N s_36_1: return s_36_0
        return s_36_0;
    }
}
