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
use Have56BitVAExt::*;
use Have52BitVAExt::*;
use common::*;
pub fn DebugAddrTop<T: Tracer>(state: &mut State, tracer: &T, gs_16006: ()) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        return_value: i128,
        gs_16006: (),
    }
    let fn_state = FunctionState {
        gs_16006,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call Have56BitVAExt(s_0_0)
        let s_0_1: bool = Have56BitVAExt(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b5 b1
        if s_0_1 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call Have52BitVAExt(s_1_0)
        let s_1_1: bool = Have52BitVAExt(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b4 b2
        if s_1_1 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_2_0: const #48s : i
        let s_2_0: i128 = 48;
        // D s_2_1: write-var return_value <= s_2_0
        fn_state.return_value = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_3_0: read-var return_value:i
        let s_3_0: i128 = fn_state.return_value;
        // N s_3_1: return s_3_0
        return s_3_0;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_4_0: const #52s : i
        let s_4_0: i128 = 52;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_5_0: const #55s : i
        let s_5_0: i128 = 55;
        // D s_5_1: write-var return_value <= s_5_0
        fn_state.return_value = s_5_0;
        // N s_5_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
