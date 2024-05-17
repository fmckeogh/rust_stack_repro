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
pub fn StageOA<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ia: u64,
    d128: bool,
    tgx: u32,
    walkstate: ProductType96e7acababe246a1,
) -> ProductTypeda0231e9dc169f81 {
    #[derive(Default)]
    struct FunctionState {
        ia: u64,
        d128: bool,
        tgx: u32,
        walkstate: ProductType96e7acababe246a1,
    }
    let fn_state = FunctionState {
        ia,
        d128,
        tgx,
        walkstate,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
