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
pub fn AArch64_ImpDefSysRegRead128<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op0: u8,
    op1: u8,
    CRn: u8,
    CRm: u8,
    op2: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        op0: u8,
        op1: u8,
        CRn: u8,
        CRm: u8,
        op2: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        op0,
        op1,
        CRn,
        CRm,
        op2,
        t,
        t2,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_0_0: panic
        panic!("{:?}", ());
        // N s_0_1: return
        return;
    }
}
