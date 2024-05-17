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
use u__UNKNOWN_boolean::*;
use common::*;
pub fn AArch64_CheckNVCondsIfCurrentEL<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op0: i128,
    op1: i128,
    crn: i128,
    crm: i128,
    op2: i128,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        op0: i128,
        op1: i128,
        crn: i128,
        crm: i128,
        op2: i128,
    }
    let fn_state = FunctionState {
        op0,
        op1,
        crn,
        crm,
        op2,
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
        // S s_0_1: tail-call __UNKNOWN_boolean(s_0_0)
        return u__UNKNOWN_boolean(state, tracer, s_0_0);
    }
}
