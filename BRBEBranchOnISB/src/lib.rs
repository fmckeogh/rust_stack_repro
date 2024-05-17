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
use u__IMPDEF_boolean::*;
use common::*;
pub fn BRBEBranchOnISB<T: Tracer>(state: &mut State, tracer: &T, gs_26374: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_26374: (),
    }
    let fn_state = FunctionState {
        gs_26374,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #"ISB generates Branch records" : str
        let s_0_0: &'static str = "ISB generates Branch records";
        // S s_0_1: tail-call __IMPDEF_boolean(s_0_0)
        return u__IMPDEF_boolean(state, tracer, s_0_0);
    }
}
