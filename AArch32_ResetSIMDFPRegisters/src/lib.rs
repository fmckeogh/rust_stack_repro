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
use u__UNKNOWN_bits::*;
use Q_set::*;
use common::*;
pub fn AArch32_ResetSIMDFPRegisters<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_31321: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        i: i64,
        gs_31321: (),
    }
    let fn_state = FunctionState {
        gs_31321,
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
        // C s_1_1: const #15s : i64
        let s_1_1: i64 = 15;
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
        // C s_2_0: const #128s : i64
        let s_2_0: i64 = 128;
        // C s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // S s_2_2: call __UNKNOWN_bits(s_2_1)
        let s_2_2: Bits = u__UNKNOWN_bits(state, tracer, s_2_1);
        // S s_2_3: cast reint s_2_2 -> u128
        let s_2_3: u128 = (s_2_2.value() as u128);
        // D s_2_4: read-var i:i64
        let s_2_4: i64 = fn_state.i;
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: call Q_set(s_2_5, s_2_3)
        let s_2_6: () = Q_set(state, tracer, s_2_5, s_2_3);
        // D s_2_7: read-var i:i64
        let s_2_7: i64 = fn_state.i;
        // C s_2_8: const #1s : i64
        let s_2_8: i64 = 1;
        // D s_2_9: add s_2_7 s_2_8
        let s_2_9: i64 = (s_2_7 + s_2_8);
        // D s_2_10: write-var i <= s_2_9
        fn_state.i = s_2_9;
        // N s_2_11: jump b1
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
