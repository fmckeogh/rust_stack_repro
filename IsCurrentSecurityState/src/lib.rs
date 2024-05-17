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
use CurrentSecurityState::*;
use common::*;
pub fn IsCurrentSecurityState<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ss: u32,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ss: u32,
    }
    let fn_state = FunctionState {
        ss,
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
        // S s_0_1: call CurrentSecurityState(s_0_0)
        let s_0_1: u32 = CurrentSecurityState(state, tracer, s_0_0);
        // D s_0_2: read-var ss:u32
        let s_0_2: u32 = fn_state.ss;
        // D s_0_3: cmp-eq s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) == (s_0_2));
        // N s_0_4: return s_0_3
        return s_0_3;
    }
}
