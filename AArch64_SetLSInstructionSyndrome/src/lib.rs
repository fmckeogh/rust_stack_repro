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
pub fn AArch64_SetLSInstructionSyndrome<T: Tracer>(
    state: &mut State,
    tracer: &T,
    size: i128,
    sign_extend: bool,
    Rt: i128,
    sixty_four: bool,
    acq_rel: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        size: i128,
        sign_extend: bool,
        Rt: i128,
        sixty_four: bool,
        acq_rel: bool,
    }
    let fn_state = FunctionState {
        size,
        sign_extend,
        Rt,
        sixty_four,
        acq_rel,
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
