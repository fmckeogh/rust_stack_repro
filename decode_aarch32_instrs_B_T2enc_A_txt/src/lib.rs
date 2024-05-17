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
use execute_aarch32_instrs_B_Op_A_txt::*;
use LastInITBlock::*;
use ConditionPassed::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_B_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm11: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        gs_295957: bool,
        imm11: u16,
    }
    let fn_state = FunctionState {
        imm11,
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
        // D s_2_0: read-var imm11:u11
        let s_2_0: u16 = fn_state.imm11;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 11u16);
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // C s_2_6: cast reint s_2_3 -> u128
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
        // D s_2_12: cast reint s_2_11 -> u12
        let s_2_12: u16 = (s_2_11.value() as u16);
        // C s_2_13: const #32s : i
        let s_2_13: i128 = 32;
        // D s_2_14: cast zx s_2_12 -> bv
        let s_2_14: Bits = Bits::new(s_2_12 as u128, 12u16);
        // D s_2_15: bits-cast sx s_2_14 -> bv length s_2_13
        let s_2_15: Bits = s_2_14.sign_extend(s_2_13);
        // D s_2_16: cast reint s_2_15 -> u32
        let s_2_16: u32 = (s_2_15.value() as u32);
        // D s_2_17: write-var imm32 <= s_2_16
        fn_state.imm32 = s_2_16;
        // C s_2_18: const #() : ()
        let s_2_18: () = ();
        // S s_2_19: call InITBlock(s_2_18)
        let s_2_19: bool = InITBlock(state, tracer, s_2_18);
        // N s_2_20: branch s_2_19 b7 b3
        if s_2_19 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#295957 <= s_3_0
        fn_state.gs_295957 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#295957:u8
        let s_4_0: bool = fn_state.gs_295957;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var imm32:u32
        let s_5_0: u32 = fn_state.imm32;
        // D s_5_1: call execute_aarch32_instrs_B_Op_A_txt(s_5_0)
        let s_5_1: () = execute_aarch32_instrs_B_Op_A_txt(state, tracer, s_5_0);
        // N s_5_2: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call LastInITBlock(s_7_0)
        let s_7_1: bool = LastInITBlock(state, tracer, s_7_0);
        // S s_7_2: not s_7_1
        let s_7_2: bool = !s_7_1;
        // D s_7_3: write-var gs#295957 <= s_7_2
        fn_state.gs_295957 = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
}
