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
use execute_aarch32_instrs_SVC_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_SVC_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm8: u8,
    }
    let fn_state = FunctionState {
        imm8,
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
        // C s_2_0: const #32s : i
        let s_2_0: i128 = 32;
        // D s_2_1: read-var imm8:u8
        let s_2_1: u8 = fn_state.imm8;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 8u16);
        // D s_2_3: bits-cast zx s_2_2 -> bv length s_2_0
        let s_2_3: Bits = s_2_2.zero_extend(s_2_0);
        // D s_2_4: cast reint s_2_3 -> u32
        let s_2_4: u32 = (s_2_3.value() as u32);
        // D s_2_5: call execute_aarch32_instrs_SVC_Op_A_txt(s_2_4)
        let s_2_5: () = execute_aarch32_instrs_SVC_Op_A_txt(state, tracer, s_2_4);
        // N s_2_6: return
        return;
    }
}
