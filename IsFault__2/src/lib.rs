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
pub fn IsFault__2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    retstatus: ProductTypef8c3639b88223255,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        retstatus: ProductTypef8c3639b88223255,
    }
    let fn_state = FunctionState {
        retstatus,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var retstatus.2:struct
        let s_0_0: u32 = fn_state.retstatus._2;
        // C s_0_1: const #0u : u32
        let s_0_1: u32 = 0;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: return s_0_2
        return s_0_2;
    }
}