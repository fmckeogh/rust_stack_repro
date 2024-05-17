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
use IsFeatureImplemented::*;
use common::*;
pub fn HaveNoninvasiveDebugAuth<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_2073: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_2073: (),
    }
    let fn_state = FunctionState {
        gs_2073,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #104u : u32
        let s_0_0: u32 = 104;
        // S s_0_1: call IsFeatureImplemented(s_0_0)
        let s_0_1: bool = IsFeatureImplemented(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: return s_0_2
        return s_0_2;
    }
}
