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
use R_set::*;
use integer_subrange::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_SMMLS_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    a: i64,
    d: i64,
    m: i64,
    n: i64,
    round: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: i128,
        a: i64,
        d: i64,
        m: i64,
        n: i64,
        round: bool,
    }
    let fn_state = FunctionState {
        a,
        d,
        m,
        n,
        round,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var a:i64
        let s_0_0: i64 = fn_state.a;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: cast sx s_0_3 -> i
        let s_0_4: i128 = {
            let sign_bit = s_0_3.length() - 1;
            let mut result = s_0_3.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // C s_0_6: const #32s : i
        let s_0_6: i128 = 32;
        // D s_0_7: cast zx s_0_5 -> i
        let s_0_7: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_8: lsl s_0_7 s_0_6
        let s_0_8: i128 = s_0_7 << s_0_6;
        // D s_0_9: read-var n:i64
        let s_0_9: i64 = fn_state.n;
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_11: call R_read(s_0_10)
        let s_0_11: u32 = R_read(state, tracer, s_0_10);
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 32u16);
        // D s_0_13: cast sx s_0_12 -> i
        let s_0_13: i128 = {
            let sign_bit = s_0_12.length() - 1;
            let mut result = s_0_12.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_14: cast reint s_0_13 -> i64
        let s_0_14: i64 = (s_0_13 as i64);
        // D s_0_15: read-var m:i64
        let s_0_15: i64 = fn_state.m;
        // D s_0_16: cast zx s_0_15 -> i
        let s_0_16: i128 = (i128::try_from(s_0_15).unwrap());
        // D s_0_17: call R_read(s_0_16)
        let s_0_17: u32 = R_read(state, tracer, s_0_16);
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 32u16);
        // D s_0_19: cast sx s_0_18 -> i
        let s_0_19: i128 = {
            let sign_bit = s_0_18.length() - 1;
            let mut result = s_0_18.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_20: cast reint s_0_19 -> i64
        let s_0_20: i64 = (s_0_19 as i64);
        // D s_0_21: cast zx s_0_14 -> i
        let s_0_21: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_22: cast zx s_0_20 -> i
        let s_0_22: i128 = (i128::try_from(s_0_20).unwrap());
        // D s_0_23: mul s_0_21 s_0_22
        let s_0_23: i128 = ((s_0_21) * (s_0_22));
        // D s_0_24: cast reint s_0_23 -> i64
        let s_0_24: i64 = (s_0_23 as i64);
        // D s_0_25: cast zx s_0_24 -> i
        let s_0_25: i128 = (i128::try_from(s_0_24).unwrap());
        // D s_0_26: sub s_0_8 s_0_25
        let s_0_26: i128 = ((s_0_8) - (s_0_25));
        // D s_0_27: write-var result <= s_0_26
        fn_state.result = s_0_26;
        // D s_0_28: read-var round:u8
        let s_0_28: bool = fn_state.round;
        // N s_0_29: branch s_0_28 b3 b1
        if s_0_28 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var result:i
        let s_2_0: i128 = fn_state.result;
        // C s_2_1: const #63s : i
        let s_2_1: i128 = 63;
        // C s_2_2: const #32s : i
        let s_2_2: i128 = 32;
        // D s_2_3: call integer_subrange(s_2_0, s_2_1, s_2_2)
        let s_2_3: Bits = integer_subrange(state, tracer, s_2_0, s_2_1, s_2_2);
        // D s_2_4: cast reint s_2_3 -> u32
        let s_2_4: u32 = (s_2_3.value() as u32);
        // D s_2_5: read-var d:i64
        let s_2_5: i64 = fn_state.d;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: call R_set(s_2_6, s_2_4)
        let s_2_7: () = R_set(state, tracer, s_2_6, s_2_4);
        // N s_2_8: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #2147483648u : u32
        let s_3_0: u32 = 2147483648;
        // C s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 32u16);
        // C s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // C s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-var result:i
        let s_3_5: i128 = fn_state.result;
        // D s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: write-var result <= s_3_6
        fn_state.result = s_3_6;
        // N s_3_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
