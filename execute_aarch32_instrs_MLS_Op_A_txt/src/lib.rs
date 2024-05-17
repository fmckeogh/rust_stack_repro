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
pub fn execute_aarch32_instrs_MLS_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    a: i64,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        a: i64,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        a,
        d,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
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
        // D s_0_6: read-var m:i64
        let s_0_6: i64 = fn_state.m;
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_8: call R_read(s_0_7)
        let s_0_8: u32 = R_read(state, tracer, s_0_7);
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 32u16);
        // D s_0_10: cast sx s_0_9 -> i
        let s_0_10: i128 = {
            let sign_bit = s_0_9.length() - 1;
            let mut result = s_0_9.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // D s_0_12: read-var a:i64
        let s_0_12: i64 = fn_state.a;
        // D s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (i128::try_from(s_0_12).unwrap());
        // D s_0_14: call R_read(s_0_13)
        let s_0_14: u32 = R_read(state, tracer, s_0_13);
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 32u16);
        // D s_0_16: cast sx s_0_15 -> i
        let s_0_16: i128 = {
            let sign_bit = s_0_15.length() - 1;
            let mut result = s_0_15.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_17: cast reint s_0_16 -> i64
        let s_0_17: i64 = (s_0_16 as i64);
        // D s_0_18: cast zx s_0_5 -> i
        let s_0_18: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_19: cast zx s_0_11 -> i
        let s_0_19: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_20: mul s_0_18 s_0_19
        let s_0_20: i128 = ((s_0_18) * (s_0_19));
        // D s_0_21: cast reint s_0_20 -> i64
        let s_0_21: i64 = (s_0_20 as i64);
        // D s_0_22: cast zx s_0_17 -> i
        let s_0_22: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_23: cast zx s_0_21 -> i
        let s_0_23: i128 = (i128::try_from(s_0_21).unwrap());
        // D s_0_24: sub s_0_22 s_0_23
        let s_0_24: i128 = ((s_0_22) - (s_0_23));
        // D s_0_25: cast reint s_0_24 -> i64
        let s_0_25: i64 = (s_0_24 as i64);
        // C s_0_26: const #31s : i
        let s_0_26: i128 = 31;
        // C s_0_27: const #0s : i
        let s_0_27: i128 = 0;
        // D s_0_28: cast zx s_0_25 -> i
        let s_0_28: i128 = (i128::try_from(s_0_25).unwrap());
        // D s_0_29: call integer_subrange(s_0_28, s_0_26, s_0_27)
        let s_0_29: Bits = integer_subrange(state, tracer, s_0_28, s_0_26, s_0_27);
        // D s_0_30: cast reint s_0_29 -> u32
        let s_0_30: u32 = (s_0_29.value() as u32);
        // D s_0_31: read-var d:i64
        let s_0_31: i64 = fn_state.d;
        // D s_0_32: cast zx s_0_31 -> i
        let s_0_32: i128 = (i128::try_from(s_0_31).unwrap());
        // D s_0_33: call R_set(s_0_32, s_0_30)
        let s_0_33: () = R_set(state, tracer, s_0_32, s_0_30);
        // N s_0_34: return
        return;
    }
}
