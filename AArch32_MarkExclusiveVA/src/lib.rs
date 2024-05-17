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
pub fn AArch32_MarkExclusiveVA<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    processorid: i128,
    size: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u32,
        processorid: i128,
        size: i128,
    }
    let fn_state = FunctionState {
        address,
        processorid,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_0_0: return
        return;
    }
}
