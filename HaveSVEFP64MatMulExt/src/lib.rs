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
pub fn HaveSVEFP64MatMulExt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_22511: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_22511: (),
    }
    let fn_state = FunctionState {
        gs_22511,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #64u : u32
        let s_0_0: u32 = 64;
        // S s_0_1: tail-call IsFeatureImplemented(s_0_0)
        return IsFeatureImplemented(state, tracer, s_0_0);
    }
}
