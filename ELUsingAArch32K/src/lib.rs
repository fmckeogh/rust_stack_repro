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
use IsSecureBelowEL3::*;
use ELStateUsingAArch32K::*;
use common::*;
pub fn ELUsingAArch32K<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
) -> ProductType8b847afc727d5818 {
    #[derive(Default)]
    struct FunctionState {
        el: u8,
    }
    let fn_state = FunctionState {
        el,
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
        // S s_0_1: call IsSecureBelowEL3(s_0_0)
        let s_0_1: bool = IsSecureBelowEL3(state, tracer, s_0_0);
        // D s_0_2: read-var el:u8
        let s_0_2: u8 = fn_state.el;
        // D s_0_3: tail-call ELStateUsingAArch32K(s_0_2, s_0_1)
        return ELStateUsingAArch32K(state, tracer, s_0_2, s_0_1);
    }
}
