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
use IsAsyncAbort::*;
use common::*;
pub fn IsAsyncAbort__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault: ProductType1d757adad216cdef,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var fault.16:struct
        let s_0_0: u32 = fn_state.fault._16;
        // D s_0_1: tail-call IsAsyncAbort(s_0_0)
        return IsAsyncAbort(state, tracer, s_0_0);
    }
}
