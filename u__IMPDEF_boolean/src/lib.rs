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
use ImpDefBool::*;
use common::*;
pub fn u__IMPDEF_boolean<T: Tracer>(
    state: &mut State,
    tracer: &T,
    x: &'static str,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        x: &'static str,
    }
    let fn_state = FunctionState {
        x,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var x:str
        let s_0_0: &'static str = fn_state.x;
        // D s_0_1: tail-call ImpDefBool(s_0_0)
        return ImpDefBool(state, tracer, s_0_0);
    }
}
