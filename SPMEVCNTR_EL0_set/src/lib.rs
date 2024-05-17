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
pub fn SPMEVCNTR_EL0_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    s: i128,
    n: i128,
    value_name: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        s: i128,
        n: i128,
        value_name: u64,
    }
    let fn_state = FunctionState {
        s,
        n,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_0_0: return
        return;
    }
}
