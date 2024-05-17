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
use AArch32_TakeHypTrapException__1::*;
use AArch32_SystemAccessTrapSyndrome::*;
use ThisInstr::*;
use common::*;
pub fn AArch32_TakeHypTrapException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ec: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ec: i128,
    }
    let fn_state = FunctionState {
        ec,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ThisInstr(s_0_0)
        let s_0_1: u32 = ThisInstr(state, tracer, s_0_0);
        // D s_0_2: read-var ec:i
        let s_0_2: i128 = fn_state.ec;
        // D s_0_3: call AArch32_SystemAccessTrapSyndrome(s_0_1, s_0_2)
        let s_0_3: ProductTypeb7f99f96751e17c4 = AArch32_SystemAccessTrapSyndrome(
            state,
            tracer,
            s_0_1,
            s_0_2,
        );
        // D s_0_4: call AArch32_TakeHypTrapException__1(s_0_3)
        let s_0_4: () = AArch32_TakeHypTrapException__1(state, tracer, s_0_3);
        // N s_0_5: return
        return;
    }
}
