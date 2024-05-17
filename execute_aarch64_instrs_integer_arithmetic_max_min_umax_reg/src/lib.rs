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
use X_set::*;
use X_read::*;
use integer_subrange::*;
use common::*;
pub fn execute_aarch64_instrs_integer_arithmetic_max_min_umax_reg<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        datasize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
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
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_4: cast reint s_0_3 -> i64
        let s_0_4: i64 = (s_0_3 as i64);
        // D s_0_5: read-var n:i64
        let s_0_5: i64 = fn_state.n;
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: call X_read(s_0_6, s_0_4)
        let s_0_7: Bits = X_read(state, tracer, s_0_6, s_0_4);
        // D s_0_8: cast zx s_0_2 -> i
        let s_0_8: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_9: cast reint s_0_8 -> i64
        let s_0_9: i64 = (s_0_8 as i64);
        // D s_0_10: read-var m:i64
        let s_0_10: i64 = fn_state.m;
        // D s_0_11: cast zx s_0_10 -> i
        let s_0_11: i128 = (i128::try_from(s_0_10).unwrap());
        // D s_0_12: call X_read(s_0_11, s_0_9)
        let s_0_12: Bits = X_read(state, tracer, s_0_11, s_0_9);
        // D s_0_13: cast zx s_0_7 -> i
        let s_0_13: i128 = (s_0_7.value() as i128);
        // D s_0_14: cast zx s_0_12 -> i
        let s_0_14: i128 = (s_0_12.value() as i128);
        // D s_0_15: cmp-gt s_0_13 s_0_14
        let s_0_15: bool = ((s_0_13) > (s_0_14));
        // D s_0_16: select s_0_15 s_0_13 s_0_14
        let s_0_16: i128 = if s_0_15 { s_0_13 } else { s_0_14 };
        // D s_0_17: cast zx s_0_2 -> i
        let s_0_17: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_18: cast reint s_0_17 -> i64
        let s_0_18: i64 = (s_0_17 as i64);
        // C s_0_19: const #1s : i
        let s_0_19: i128 = 1;
        // D s_0_20: cast zx s_0_2 -> i
        let s_0_20: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_21: sub s_0_20 s_0_19
        let s_0_21: i128 = ((s_0_20) - (s_0_19));
        // D s_0_22: cast reint s_0_21 -> i64
        let s_0_22: i64 = (s_0_21 as i64);
        // C s_0_23: const #0s : i
        let s_0_23: i128 = 0;
        // D s_0_24: cast zx s_0_22 -> i
        let s_0_24: i128 = (i128::try_from(s_0_22).unwrap());
        // D s_0_25: call integer_subrange(s_0_16, s_0_24, s_0_23)
        let s_0_25: Bits = integer_subrange(state, tracer, s_0_16, s_0_24, s_0_23);
        // D s_0_26: read-var d:i64
        let s_0_26: i64 = fn_state.d;
        // D s_0_27: cast zx s_0_26 -> i
        let s_0_27: i128 = (i128::try_from(s_0_26).unwrap());
        // D s_0_28: call X_set(s_0_27, s_0_18, s_0_25)
        let s_0_28: () = X_set(state, tracer, s_0_27, s_0_18, s_0_25);
        // N s_0_29: return
        return;
    }
}
