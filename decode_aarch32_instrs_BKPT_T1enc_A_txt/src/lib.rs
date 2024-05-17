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
use execute_aarch32_instrs_BKPT_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_BKPT_T1enc_A_txt<T: Tracer>(
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
        // C s_0_0: const #16s : i
        let s_0_0: i128 = 16;
        // D s_0_1: read-var imm8:u8
        let s_0_1: u8 = fn_state.imm8;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 8u16);
        // D s_0_3: bits-cast zx s_0_2 -> bv length s_0_0
        let s_0_3: Bits = s_0_2.zero_extend(s_0_0);
        // D s_0_4: cast reint s_0_3 -> u16
        let s_0_4: u16 = (s_0_3.value() as u16);
        // D s_0_5: call execute_aarch32_instrs_BKPT_Op_A_txt(s_0_4)
        let s_0_5: () = execute_aarch32_instrs_BKPT_Op_A_txt(state, tracer, s_0_4);
        // N s_0_6: return
        return;
    }
}
