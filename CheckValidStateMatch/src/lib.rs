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
pub fn CheckValidStateMatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ssc_in: u8,
    ssce_in: bool,
    hmc_in: bool,
    pxc_in: u8,
    isbreakpnt: bool,
) -> ProductTypeba129578e5d1bd1b {
    #[derive(Default)]
    struct FunctionState {
        ssc_in: u8,
        ssce_in: bool,
        hmc_in: bool,
        pxc_in: u8,
        isbreakpnt: bool,
    }
    let fn_state = FunctionState {
        ssc_in,
        ssce_in,
        hmc_in,
        pxc_in,
        isbreakpnt,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeba129578e5d1bd1b {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
