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
use is_some_EInterruptID_pcnt__::*;
use common::*;
pub fn u__GIC_InterruptPending<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_331301: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_331301: (),
    }
    let fn_state = FunctionState {
        gs_331301,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #20608u : u32
        let s_0_0: u32 = 20608;
        // D s_0_1: read-reg s_0_0:enum
        let s_0_1: SumTypef8de2b264306e832 = {
            let value = state.read_register::<SumTypef8de2b264306e832>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: tail-call is_some<EInterruptID%>(s_0_1)
        return is_some_EInterruptID_pcnt__(state, tracer, s_0_1);
    }
}
