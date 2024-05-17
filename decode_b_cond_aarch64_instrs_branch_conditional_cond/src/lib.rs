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
use execute_aarch64_instrs_branch_conditional_cond::*;
use common::*;
pub fn decode_b_cond_aarch64_instrs_branch_conditional_cond<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    imm19: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        cond: u8,
        imm19: u32,
    }
    let fn_state = FunctionState {
        cond,
        imm19,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var imm19:u19
        let s_0_0: u32 = fn_state.imm19;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 19u16);
        // C s_0_2: const #0u : u8
        let s_0_2: u8 = 0;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // D s_0_4: cast reint s_0_1 -> u128
        let s_0_4: u128 = (s_0_1.value() as u128);
        // D s_0_5: size-of s_0_1
        let s_0_5: u16 = s_0_1.length();
        // C s_0_6: cast reint s_0_3 -> u128
        let s_0_6: u128 = (s_0_3.value() as u128);
        // D s_0_7: size-of s_0_3
        let s_0_7: u16 = s_0_3.length();
        // D s_0_8: lsl s_0_4 s_0_7
        let s_0_8: u128 = s_0_4 << s_0_7;
        // D s_0_9: or s_0_8 s_0_6
        let s_0_9: u128 = ((s_0_8) | (s_0_6));
        // D s_0_10: add s_0_5 s_0_7
        let s_0_10: u16 = (s_0_5 + s_0_7);
        // D s_0_11: create-bits s_0_9 s_0_10
        let s_0_11: Bits = Bits::new(s_0_9, s_0_10);
        // D s_0_12: cast reint s_0_11 -> u21
        let s_0_12: u32 = (s_0_11.value() as u32);
        // C s_0_13: const #64s : i
        let s_0_13: i128 = 64;
        // D s_0_14: cast zx s_0_12 -> bv
        let s_0_14: Bits = Bits::new(s_0_12 as u128, 21u16);
        // D s_0_15: bits-cast sx s_0_14 -> bv length s_0_13
        let s_0_15: Bits = s_0_14.sign_extend(s_0_13);
        // D s_0_16: cast reint s_0_15 -> u64
        let s_0_16: u64 = (s_0_15.value() as u64);
        // D s_0_17: read-var cond:u8
        let s_0_17: u8 = fn_state.cond;
        // D s_0_18: call execute_aarch64_instrs_branch_conditional_cond(s_0_17, s_0_16)
        let s_0_18: () = execute_aarch64_instrs_branch_conditional_cond(
            state,
            tracer,
            s_0_17,
            s_0_16,
        );
        // N s_0_19: return
        return;
    }
}
