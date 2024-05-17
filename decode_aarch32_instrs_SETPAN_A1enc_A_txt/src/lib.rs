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
use HavePANExt::*;
use execute_aarch32_instrs_SETPAN_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_SETPAN_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm1: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm1: bool,
    }
    let fn_state = FunctionState {
        imm1,
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
        // S s_0_1: call HavePANExt(s_0_0)
        let s_0_1: bool = HavePANExt(state, tracer, s_0_0);
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
        // D s_1_0: read-var imm1:u8
        let s_1_0: bool = fn_state.imm1;
        // D s_1_1: call execute_aarch32_instrs_SETPAN_Op_A_txt(s_1_0)
        let s_1_1: () = execute_aarch32_instrs_SETPAN_Op_A_txt(state, tracer, s_1_0);
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
