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
use u__GIC_InterruptPending::*;
use common::*;
pub fn IRQPending<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_331356: (),
) -> ProductType8b847afc727d5818 {
    #[derive(Default)]
    struct FunctionState {
        gs_331356: (),
    }
    let fn_state = FunctionState {
        gs_331356,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call __GIC_InterruptPending(s_0_0)
        let s_0_1: bool = u__GIC_InterruptPending(state, tracer, s_0_0);
        // C s_0_2: const #0u : u8
        let s_0_2: bool = false;
        // D s_0_3: create-product struct = ["s_0_1", "s_0_2"]
        let s_0_3: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_0_1,
            _1: s_0_2,
        };
        // N s_0_4: return s_0_3
        return s_0_3;
    }
}
