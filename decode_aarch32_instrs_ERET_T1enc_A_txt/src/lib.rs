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
use LastInITBlock::*;
use ConditionPassed::*;
use execute_aarch32_instrs_ERET_Op_AS_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_ERET_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_323208: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_323210: bool,
        gs_323208: (),
    }
    let fn_state = FunctionState {
        gs_323208,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
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
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call InITBlock(s_2_0)
        let s_2_1: bool = InITBlock(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b7 b3
        if s_2_1 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#323210 <= s_3_0
        fn_state.gs_323210 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#323210:u8
        let s_4_0: bool = fn_state.gs_323210;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call execute_aarch32_instrs_ERET_Op_AS_txt(s_5_0)
        let s_5_1: () = execute_aarch32_instrs_ERET_Op_AS_txt(state, tracer, s_5_0);
        // N s_5_2: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call LastInITBlock(s_7_0)
        let s_7_1: bool = LastInITBlock(state, tracer, s_7_0);
        // S s_7_2: not s_7_1
        let s_7_2: bool = !s_7_1;
        // D s_7_3: write-var gs#323210 <= s_7_2
        fn_state.gs_323210 = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
}
