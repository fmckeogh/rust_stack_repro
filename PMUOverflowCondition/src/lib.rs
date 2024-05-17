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
pub fn PMUOverflowCondition<T: Tracer>(
    state: &mut State,
    tracer: &T,
    check_e: bool,
    check_cnten: bool,
    check_inten: bool,
    include_hi_name: bool,
    include_lo_name: bool,
    exclude_cyc: bool,
    exclude_sync: bool,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        check_e: bool,
        check_cnten: bool,
        check_inten: bool,
        include_hi_name: bool,
        include_lo_name: bool,
        exclude_cyc: bool,
        exclude_sync: bool,
    }
    let fn_state = FunctionState {
        check_e,
        check_cnten,
        check_inten,
        include_hi_name,
        include_lo_name,
        exclude_cyc,
        exclude_sync,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
