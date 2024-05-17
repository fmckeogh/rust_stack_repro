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
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_HVC_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm4: u8,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm16: u16,
        imm4: u8,
        imm12: u16,
    }
    let fn_state = FunctionState {
        imm4,
        imm12,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var imm4:u8
        let s_0_0: u8 = fn_state.imm4;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 4u16);
        // D s_0_2: read-var imm12:u12
        let s_0_2: u16 = fn_state.imm12;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 12u16);
        // D s_0_4: cast reint s_0_1 -> u128
        let s_0_4: u128 = (s_0_1.value() as u128);
        // D s_0_5: size-of s_0_1
        let s_0_5: u16 = s_0_1.length();
        // D s_0_6: cast reint s_0_3 -> u128
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
        // D s_0_12: cast reint s_0_11 -> u16
        let s_0_12: u16 = (s_0_11.value() as u16);
        // D s_0_13: write-var imm16 <= s_0_12
        fn_state.imm16 = s_0_12;
        // C s_0_14: const #() : ()
        let s_0_14: () = ();
        // S s_0_15: call InITBlock(s_0_14)
        let s_0_15: bool = InITBlock(state, tracer, s_0_14);
        // N s_0_16: branch s_0_15 b2 b1
        if s_0_15 {
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
        // D s_1_1: call execute_aarch32_instrs_HVC_Op_AS_txt(s_1_0)
        let s_1_1: () = execute_aarch32_instrs_HVC_Op_AS_txt(state, tracer, s_1_0);
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
