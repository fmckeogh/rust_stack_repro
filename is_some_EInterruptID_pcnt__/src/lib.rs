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
pub fn is_some_EInterruptID_pcnt__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    opt: SumTypef8de2b264306e832,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_76: bool,
        opt: SumTypef8de2b264306e832,
    }
    let fn_state = FunctionState {
        opt,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var opt:enum
        let s_0_0: SumTypef8de2b264306e832 = fn_state.opt;
        // D s_0_1: matches-sum s_0_0 1
        let s_0_1: bool = matches!(s_0_0, SumTypef8de2b264306e832::_1(_));
        // N s_0_2: branch s_0_1 b3 b1
        if s_0_1 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #1u : u8
        let s_1_0: bool = true;
        // D s_1_1: write-var gs#76 <= s_1_0
        fn_state.gs_76 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#76:u8
        let s_2_0: bool = fn_state.gs_76;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var opt:enum
        let s_3_0: SumTypef8de2b264306e832 = fn_state.opt;
        // D s_3_1: matches-sum s_3_0 0
        let s_3_1: bool = matches!(s_3_0, SumTypef8de2b264306e832::_0(_));
        // N s_3_2: branch s_3_1 b5 b4
        if s_3_1 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#76 <= s_4_0
        fn_state.gs_76 = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_5_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
