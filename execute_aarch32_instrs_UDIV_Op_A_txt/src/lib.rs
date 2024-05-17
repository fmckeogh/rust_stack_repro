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
use R_read::*;
use R_set::*;
use RoundTowardsZero::*;
use integer_subrange::*;
use common::*;
pub fn execute_aarch32_instrs_UDIV_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: i128,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
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
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (s_0_3.value() as i128);
        // D s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // D s_0_7: cast zx s_0_5 -> i
        let s_0_7: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_8: cmp-eq s_0_7 s_0_6
        let s_0_8: bool = ((s_0_7) == (s_0_6));
        // N s_0_9: branch s_0_8 b3 b1
        if s_0_8 {
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
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 32u16);
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (s_1_3.value() as i128);
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: cast reint s_1_6 -> f64
        let s_1_7: f64 = (s_1_6 as f64);
        // D s_1_8: cast trunc s_1_7 -> f32
        let s_1_8: f32 = (s_1_7 as f32);
        // D s_1_9: read-var m:i64
        let s_1_9: i64 = fn_state.m;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: call R_read(s_1_10)
        let s_1_11: u32 = R_read(state, tracer, s_1_10);
        // D s_1_12: cast zx s_1_11 -> bv
        let s_1_12: Bits = Bits::new(s_1_11 as u128, 32u16);
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (s_1_12.value() as i128);
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> f64
        let s_1_16: f64 = (s_1_15 as f64);
        // D s_1_17: cast trunc s_1_16 -> f32
        let s_1_17: f32 = (s_1_16 as f32);
        // D s_1_18: div s_1_8 s_1_17
        let s_1_18: f32 = ((s_1_8) / (s_1_17));
        // D s_1_19: call RoundTowardsZero(s_1_18)
        let s_1_19: i128 = RoundTowardsZero(state, tracer, s_1_18);
        // D s_1_20: write-var result <= s_1_19
        fn_state.result = s_1_19;
        // N s_1_21: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var result:i
        let s_2_0: i128 = fn_state.result;
        // C s_2_1: const #31s : i
        let s_2_1: i128 = 31;
        // C s_2_2: const #0s : i
        let s_2_2: i128 = 0;
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
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: write-var result <= s_3_0
        fn_state.result = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
