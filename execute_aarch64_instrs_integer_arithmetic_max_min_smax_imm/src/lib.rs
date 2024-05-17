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
pub fn execute_aarch64_instrs_integer_arithmetic_max_min_smax_imm<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    imm: i128,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        datasize: i64,
        imm: i128,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        imm,
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
        // D s_0_8: cast sx s_0_7 -> i
        let s_0_8: i128 = {
            let sign_bit = s_0_7.length() - 1;
            let mut result = s_0_7.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_9: cast reint s_0_8 -> i64
        let s_0_9: i64 = (s_0_8 as i64);
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_11: read-var imm:i
        let s_0_11: i128 = fn_state.imm;
        // D s_0_12: cmp-gt s_0_10 s_0_11
        let s_0_12: bool = ((s_0_10) > (s_0_11));
        // D s_0_13: select s_0_12 s_0_10 s_0_11
        let s_0_13: i128 = if s_0_12 { s_0_10 } else { s_0_11 };
        // D s_0_14: cast zx s_0_2 -> i
        let s_0_14: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // C s_0_16: const #1s : i
        let s_0_16: i128 = 1;
        // D s_0_17: cast zx s_0_2 -> i
        let s_0_17: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_18: sub s_0_17 s_0_16
        let s_0_18: i128 = ((s_0_17) - (s_0_16));
        // D s_0_19: cast reint s_0_18 -> i64
        let s_0_19: i64 = (s_0_18 as i64);
        // C s_0_20: const #0s : i
        let s_0_20: i128 = 0;
        // D s_0_21: cast zx s_0_19 -> i
        let s_0_21: i128 = (i128::try_from(s_0_19).unwrap());
        // D s_0_22: call integer_subrange(s_0_13, s_0_21, s_0_20)
        let s_0_22: Bits = integer_subrange(state, tracer, s_0_13, s_0_21, s_0_20);
        // D s_0_23: read-var d:i64
        let s_0_23: i64 = fn_state.d;
        // D s_0_24: cast zx s_0_23 -> i
        let s_0_24: i128 = (i128::try_from(s_0_23).unwrap());
        // D s_0_25: call X_set(s_0_24, s_0_15, s_0_22)
        let s_0_25: () = X_set(state, tracer, s_0_24, s_0_15, s_0_22);
        // N s_0_26: return
        return;
    }
}
