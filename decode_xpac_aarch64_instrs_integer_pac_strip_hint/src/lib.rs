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
use execute_aarch64_instrs_integer_pac_strip_dp_1src::*;
use common::*;
pub fn decode_xpac_aarch64_instrs_integer_pac_strip_hint<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_174395: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_174395: (),
    }
    let fn_state = FunctionState {
        gs_174395,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #30s : i64
        let s_0_0: i64 = 30;
        // C s_0_1: const #0u : u8
        let s_0_1: bool = false;
        // S s_0_2: call execute_aarch64_instrs_integer_pac_strip_dp_1src(s_0_0, s_0_1)
        let s_0_2: () = execute_aarch64_instrs_integer_pac_strip_dp_1src(
            state,
            tracer,
            s_0_0,
            s_0_1,
        );
        // N s_0_3: return
        return;
    }
}
