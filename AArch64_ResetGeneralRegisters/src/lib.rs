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
use u__UNKNOWN_bits::*;
use common::*;
pub fn AArch64_ResetGeneralRegisters<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_15538: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        i: i64,
        gs_15538: (),
    }
    let fn_state = FunctionState {
        gs_15538,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
    ) -> () {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // C s_1_1: const #30s : i64
        let s_1_1: i64 = 30;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b3 b2
        if s_1_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // C s_2_1: const #64s : i64
        let s_2_1: i64 = 64;
        // C s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // S s_2_3: call __UNKNOWN_bits(s_2_2)
        let s_2_3: Bits = u__UNKNOWN_bits(state, tracer, s_2_2);
        // S s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: read-var i:i64
        let s_2_5: i64 = fn_state.i;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // S s_2_7: cast zx s_2_4 -> bv
        let s_2_7: Bits = Bits::new(s_2_4 as u128, 64u16);
        // D s_2_8: call X_set(s_2_6, s_2_0, s_2_7)
        let s_2_8: () = X_set(state, tracer, s_2_6, s_2_0, s_2_7);
        // D s_2_9: read-var i:i64
        let s_2_9: i64 = fn_state.i;
        // C s_2_10: const #1s : i64
        let s_2_10: i64 = 1;
        // D s_2_11: add s_2_9 s_2_10
        let s_2_11: i64 = (s_2_9 + s_2_10);
        // D s_2_12: write-var i <= s_2_11
        fn_state.i = s_2_11;
        // N s_2_13: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
}
