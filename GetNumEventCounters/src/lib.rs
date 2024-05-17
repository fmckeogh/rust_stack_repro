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
use u__IMPDEF_integer::*;
use common::*;
pub fn GetNumEventCounters<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_1888: (),
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        gs_1888: (),
    }
    let fn_state = FunctionState {
        gs_1888,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #"Number of event counters" : str
        let s_0_0: &'static str = "Number of event counters";
        // S s_0_1: tail-call __IMPDEF_integer(s_0_0)
        return u__IMPDEF_integer(state, tracer, s_0_0);
    }
}
