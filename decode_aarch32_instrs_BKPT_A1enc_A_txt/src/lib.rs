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
pub fn decode_aarch32_instrs_BKPT_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    imm12: u16,
    imm4: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm16: u16,
        cond: u8,
        imm12: u16,
        imm4: u8,
    }
    let fn_state = FunctionState {
        cond,
        imm12,
        imm4,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var cond:u8
        let s_0_0: u8 = fn_state.cond;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 4u16);
        // C s_0_2: const #15u : u8
        let s_0_2: u8 = 15;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 4u16);
        // D s_0_4: cmp-ne s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) != (s_0_3));
        // N s_0_5: assert s_0_4
        let s_0_5: () = assert!(s_0_4);
        // D s_0_6: read-var imm12:u12
        let s_0_6: u16 = fn_state.imm12;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 12u16);
        // D s_0_8: read-var imm4:u8
        let s_0_8: u8 = fn_state.imm4;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 4u16);
        // D s_0_10: cast reint s_0_7 -> u128
        let s_0_10: u128 = (s_0_7.value() as u128);
        // D s_0_11: size-of s_0_7
        let s_0_11: u16 = s_0_7.length();
        // D s_0_12: cast reint s_0_9 -> u128
        let s_0_12: u128 = (s_0_9.value() as u128);
        // D s_0_13: size-of s_0_9
        let s_0_13: u16 = s_0_9.length();
        // D s_0_14: lsl s_0_10 s_0_13
        let s_0_14: u128 = s_0_10 << s_0_13;
        // D s_0_15: or s_0_14 s_0_12
        let s_0_15: u128 = ((s_0_14) | (s_0_12));
        // D s_0_16: add s_0_11 s_0_13
        let s_0_16: u16 = (s_0_11 + s_0_13);
        // D s_0_17: create-bits s_0_15 s_0_16
        let s_0_17: Bits = Bits::new(s_0_15, s_0_16);
        // D s_0_18: cast reint s_0_17 -> u16
        let s_0_18: u16 = (s_0_17.value() as u16);
        // D s_0_19: write-var imm16 <= s_0_18
        fn_state.imm16 = s_0_18;
        // D s_0_20: read-var cond:u8
        let s_0_20: u8 = fn_state.cond;
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 4u16);
        // C s_0_22: const #14u : u8
        let s_0_22: u8 = 14;
        // C s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 4u16);
        // D s_0_24: cmp-ne s_0_21 s_0_23
        let s_0_24: bool = ((s_0_21) != (s_0_23));
        // N s_0_25: branch s_0_24 b2 b1
        if s_0_24 {
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
        // D s_1_0: read-var imm16:u16
        let s_1_0: u16 = fn_state.imm16;
        // D s_1_1: call execute_aarch32_instrs_BKPT_Op_A_txt(s_1_0)
        let s_1_1: () = execute_aarch32_instrs_BKPT_Op_A_txt(state, tracer, s_1_0);
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
