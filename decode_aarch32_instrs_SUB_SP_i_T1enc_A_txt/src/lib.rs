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
use place_slice::*;
use execute_aarch32_instrs_SUB_SP_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_SUB_SP_i_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm7: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm7: u8,
    }
    let fn_state = FunctionState {
        imm7,
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
        // C s_2_0: const #13s : i64
        let s_2_0: i64 = 13;
        // C s_2_1: const #0u : u8
        let s_2_1: bool = false;
        // C s_2_2: const #32s : i
        let s_2_2: i128 = 32;
        // C s_2_3: const #0s : i
        let s_2_3: i128 = 0;
        // C s_2_4: const #7s : i
        let s_2_4: i128 = 7;
        // C s_2_5: const #2s : i
        let s_2_5: i128 = 2;
        // D s_2_6: read-var imm7:u8
        let s_2_6: u8 = fn_state.imm7;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 7u16);
        // D s_2_8: call place_slice(s_2_2, s_2_7, s_2_3, s_2_4, s_2_5)
        let s_2_8: Bits = place_slice(state, tracer, s_2_2, s_2_7, s_2_3, s_2_4, s_2_5);
        // D s_2_9: cast reint s_2_8 -> u32
        let s_2_9: u32 = (s_2_8.value() as u32);
        // D s_2_10: call execute_aarch32_instrs_SUB_SP_i_Op_A_txt(s_2_0, s_2_9, s_2_1)
        let s_2_10: () = execute_aarch32_instrs_SUB_SP_i_Op_A_txt(
            state,
            tracer,
            s_2_0,
            s_2_9,
            s_2_1,
        );
        // N s_2_11: return
        return;
    }
}
