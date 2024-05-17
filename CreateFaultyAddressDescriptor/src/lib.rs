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
pub fn CreateFaultyAddressDescriptor<T: Tracer>(
    state: &mut State,
    tracer: &T,
    va: u64,
    fault: ProductType1d757adad216cdef,
) -> ProductTypece7c66ccb2cab13e {
    #[derive(Default)]
    struct FunctionState {
        addrdesc: ProductTypece7c66ccb2cab13e,
        va: u64,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        va,
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_0_0: read-var va:u64
        let s_0_0: u64 = fn_state.va;
        // D s_0_1: write-var addrdesc.7 <= s_0_0
        fn_state.addrdesc._7 = s_0_0;
        // D s_0_2: read-var fault:struct
        let s_0_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_0_3: write-var addrdesc.0 <= s_0_2
        fn_state.addrdesc._0 = s_0_2;
        // D s_0_4: read-var addrdesc:struct
        let s_0_4: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // N s_0_5: return s_0_4
        return s_0_4;
    }
}
