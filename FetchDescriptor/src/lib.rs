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
pub fn FetchDescriptor<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ee: bool,
    walkaddress: ProductTypece7c66ccb2cab13e,
    walkaccess: ProductType9878976b5bcce9c9,
    fault_in: ProductType1d757adad216cdef,
    N: i64,
    translation_info: ProductTypeb525737120e184b3,
) -> ProductTypeb4cea7287e2eb9d6 {
    #[derive(Default)]
    struct FunctionState {
        ee: bool,
        walkaddress: ProductTypece7c66ccb2cab13e,
        walkaccess: ProductType9878976b5bcce9c9,
        fault_in: ProductType1d757adad216cdef,
        N: i64,
        translation_info: ProductTypeb525737120e184b3,
    }
    let fn_state = FunctionState {
        ee,
        walkaddress,
        walkaccess,
        fault_in,
        N,
        translation_info,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}