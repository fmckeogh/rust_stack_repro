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
use execute_aarch32_instrs_HVC_Op_AS_txt::*;
use common::*;
pub fn decode_aarch32_instrs_HVC_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    imm12: u16,
    imm4: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
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
        // D s_0_6: read-var cond:u8
        let s_0_6: u8 = fn_state.cond;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 4u16);
        // C s_0_8: const #14u : u8
        let s_0_8: u8 = 14;
        // C s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 4u16);
        // D s_0_10: cmp-ne s_0_7 s_0_9
        let s_0_10: bool = ((s_0_7) != (s_0_9));
        // N s_0_11: branch s_0_10 b2 b1
        if s_0_10 {
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
        // D s_1_0: read-var imm12:u12
        let s_1_0: u16 = fn_state.imm12;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 12u16);
        // D s_1_2: read-var imm4:u8
        let s_1_2: u8 = fn_state.imm4;
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 4u16);
        // D s_1_4: cast reint s_1_1 -> u128
        let s_1_4: u128 = (s_1_1.value() as u128);
        // D s_1_5: size-of s_1_1
        let s_1_5: u16 = s_1_1.length();
        // D s_1_6: cast reint s_1_3 -> u128
        let s_1_6: u128 = (s_1_3.value() as u128);
        // D s_1_7: size-of s_1_3
        let s_1_7: u16 = s_1_3.length();
        // D s_1_8: lsl s_1_4 s_1_7
        let s_1_8: u128 = s_1_4 << s_1_7;
        // D s_1_9: or s_1_8 s_1_6
        let s_1_9: u128 = ((s_1_8) | (s_1_6));
        // D s_1_10: add s_1_5 s_1_7
        let s_1_10: u16 = (s_1_5 + s_1_7);
        // D s_1_11: create-bits s_1_9 s_1_10
        let s_1_11: Bits = Bits::new(s_1_9, s_1_10);
        // D s_1_12: cast reint s_1_11 -> u16
        let s_1_12: u16 = (s_1_11.value() as u16);
        // D s_1_13: call execute_aarch32_instrs_HVC_Op_AS_txt(s_1_12)
        let s_1_13: () = execute_aarch32_instrs_HVC_Op_AS_txt(state, tracer, s_1_12);
        // N s_1_14: return
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
