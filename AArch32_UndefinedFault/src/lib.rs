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
use AArch64_UndefinedFault::*;
use AArch32_TakeUndefInstrException::*;
use AArch32_GeneralExceptionsToAArch64::*;
use common::*;
pub fn AArch32_UndefinedFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_31842: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_31842: (),
    }
    let fn_state = FunctionState {
        gs_31842,
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
        // S s_0_1: call AArch32_GeneralExceptionsToAArch64(s_0_0)
        let s_0_1: bool = AArch32_GeneralExceptionsToAArch64(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b3 b1
        if s_0_1 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call AArch32_TakeUndefInstrException(s_2_0)
        let s_2_1: () = AArch32_TakeUndefInstrException(state, tracer, s_2_0);
        // N s_2_2: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call AArch64_UndefinedFault(s_3_0)
        let s_3_1: () = AArch64_UndefinedFault(state, tracer, s_3_0);
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
