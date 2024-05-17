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
pub fn decode_aarch32_instrs_BL_i_A2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    H: bool,
    imm24: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        H: bool,
        imm24: u32,
    }
    let fn_state = FunctionState {
        H,
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
        // D s_2_0: read-var imm24:u24
        let s_2_0: u32 = fn_state.imm24;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 24u16);
        // D s_2_2: read-var H:u8
        let s_2_2: bool = fn_state.H;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u25
        let s_2_12: u32 = (s_2_11.value() as u32);
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 25u16);
        // C s_2_14: const #0u : u8
        let s_2_14: bool = false;
        // C s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 1u16);
        // D s_2_16: cast reint s_2_13 -> u128
        let s_2_16: u128 = (s_2_13.value() as u128);
        // D s_2_17: size-of s_2_13
        let s_2_17: u16 = s_2_13.length();
        // C s_2_18: cast reint s_2_15 -> u128
        let s_2_18: u128 = (s_2_15.value() as u128);
        // D s_2_19: size-of s_2_15
        let s_2_19: u16 = s_2_15.length();
        // D s_2_20: lsl s_2_16 s_2_19
        let s_2_20: u128 = s_2_16 << s_2_19;
        // D s_2_21: or s_2_20 s_2_18
        let s_2_21: u128 = ((s_2_20) | (s_2_18));
        // D s_2_22: add s_2_17 s_2_19
        let s_2_22: u16 = (s_2_17 + s_2_19);
        // D s_2_23: create-bits s_2_21 s_2_22
        let s_2_23: Bits = Bits::new(s_2_21, s_2_22);
        // D s_2_24: cast reint s_2_23 -> u26
        let s_2_24: u32 = (s_2_23.value() as u32);
        // C s_2_25: const #32s : i
        let s_2_25: i128 = 32;
        // D s_2_26: cast zx s_2_24 -> bv
        let s_2_26: Bits = Bits::new(s_2_24 as u128, 26u16);
        // D s_2_27: bits-cast sx s_2_26 -> bv length s_2_25
        let s_2_27: Bits = s_2_26.sign_extend(s_2_25);
        // D s_2_28: cast reint s_2_27 -> u32
        let s_2_28: u32 = (s_2_27.value() as u32);
        // C s_2_29: const #2u : u32
        let s_2_29: u32 = 2;
        // D s_2_30: call execute_aarch32_instrs_BL_i_Op_A_txt(s_2_28, s_2_29)
        let s_2_30: () = execute_aarch32_instrs_BL_i_Op_A_txt(
            state,
            tracer,
            s_2_28,
            s_2_29,
        );
        // N s_2_31: return
        return;
    }
}
