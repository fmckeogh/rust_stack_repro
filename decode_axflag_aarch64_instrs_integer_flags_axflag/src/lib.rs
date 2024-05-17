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
use HaveFlagFormatExt::*;
use execute_aarch64_instrs_integer_flags_axflag::*;
use common::*;
pub fn decode_axflag_aarch64_instrs_integer_flags_axflag<T: Tracer>(
    state: &mut State,
    tracer: &T,
    CRm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        CRm: u8,
    }
    let fn_state = FunctionState {
        CRm,
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
        // S s_0_1: call HaveFlagFormatExt(s_0_0)
        let s_0_1: bool = HaveFlagFormatExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b2 b1
        if s_0_2 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call execute_aarch64_instrs_integer_flags_axflag(s_1_0)
        let s_1_1: () = execute_aarch64_instrs_integer_flags_axflag(
            state,
            tracer,
            s_1_0,
        );
        // N s_1_2: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}
