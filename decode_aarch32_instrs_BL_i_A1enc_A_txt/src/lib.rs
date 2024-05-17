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
use execute_aarch32_instrs_BL_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_BL_i_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    imm24: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        cond: u8,
        imm24: u32,
    }
    let fn_state = FunctionState {
        cond,
        imm24,
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
        // D s_2_0: read-var cond:u8
        let s_2_0: u8 = fn_state.cond;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // D s_2_6: read-var imm24:u24
        let s_2_6: u32 = fn_state.imm24;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 24u16);
        // C s_2_8: const #0u : u8
        let s_2_8: u8 = 0;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 2u16);
        // D s_2_10: cast reint s_2_7 -> u128
        let s_2_10: u128 = (s_2_7.value() as u128);
        // D s_2_11: size-of s_2_7
        let s_2_11: u16 = s_2_7.length();
        // C s_2_12: cast reint s_2_9 -> u128
        let s_2_12: u128 = (s_2_9.value() as u128);
        // D s_2_13: size-of s_2_9
        let s_2_13: u16 = s_2_9.length();
        // D s_2_14: lsl s_2_10 s_2_13
        let s_2_14: u128 = s_2_10 << s_2_13;
        // D s_2_15: or s_2_14 s_2_12
        let s_2_15: u128 = ((s_2_14) | (s_2_12));
        // D s_2_16: add s_2_11 s_2_13
        let s_2_16: u16 = (s_2_11 + s_2_13);
        // D s_2_17: create-bits s_2_15 s_2_16
        let s_2_17: Bits = Bits::new(s_2_15, s_2_16);
        // D s_2_18: cast reint s_2_17 -> u26
        let s_2_18: u32 = (s_2_17.value() as u32);
        // C s_2_19: const #32s : i
        let s_2_19: i128 = 32;
        // D s_2_20: cast zx s_2_18 -> bv
        let s_2_20: Bits = Bits::new(s_2_18 as u128, 26u16);
        // D s_2_21: bits-cast sx s_2_20 -> bv length s_2_19
        let s_2_21: Bits = s_2_20.sign_extend(s_2_19);
        // D s_2_22: cast reint s_2_21 -> u32
        let s_2_22: u32 = (s_2_21.value() as u32);
        // C s_2_23: const #1u : u32
        let s_2_23: u32 = 1;
        // D s_2_24: call execute_aarch32_instrs_BL_i_Op_A_txt(s_2_22, s_2_23)
        let s_2_24: () = execute_aarch32_instrs_BL_i_Op_A_txt(
            state,
            tracer,
            s_2_22,
            s_2_23,
        );
        // N s_2_25: return
        return;
    }
}
