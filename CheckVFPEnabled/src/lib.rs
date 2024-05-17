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
use AArch32_CheckAdvSIMDOrFPEnabled::*;
use common::*;
pub fn CheckVFPEnabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    include_fpexc_check_name: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        include_fpexc_check_name: bool,
    }
    let fn_state = FunctionState {
        include_fpexc_check_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // D s_0_1: read-var include_fpexc_check_name:u8
        let s_0_1: bool = fn_state.include_fpexc_check_name;
        // D s_0_2: call AArch32_CheckAdvSIMDOrFPEnabled(s_0_1, s_0_0)
        let s_0_2: () = AArch32_CheckAdvSIMDOrFPEnabled(state, tracer, s_0_1, s_0_0);
        // N s_0_3: return
        return;
    }
}
