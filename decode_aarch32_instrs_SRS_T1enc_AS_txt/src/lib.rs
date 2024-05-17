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
use ConditionPassed::*;
use execute_aarch32_instrs_SRS_OpT_AS_txt::*;
use common::*;
pub fn decode_aarch32_instrs_SRS_T1enc_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    W: bool,
    mode: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        W: bool,
        mode: u8,
    }
    let fn_state = FunctionState {
        W,
        mode,
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
        // D s_2_0: read-var W:u8
        let s_2_0: bool = fn_state.W;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // C s_2_5: const #0u : u8
        let s_2_5: bool = false;
        // C s_2_6: const #0u : u8
        let s_2_6: bool = false;
        // D s_2_7: read-var mode:u8
        let s_2_7: u8 = fn_state.mode;
        // D s_2_8: call execute_aarch32_instrs_SRS_OpT_AS_txt(s_2_5, s_2_7, s_2_4, s_2_6)
        let s_2_8: () = execute_aarch32_instrs_SRS_OpT_AS_txt(
            state,
            tracer,
            s_2_5,
            s_2_7,
            s_2_4,
            s_2_6,
        );
        // N s_2_9: return
        return;
    }
}
