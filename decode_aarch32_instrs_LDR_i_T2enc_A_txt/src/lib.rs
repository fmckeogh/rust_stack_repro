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
use execute_aarch32_instrs_LDR_i_OpT_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDR_i_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rt: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        Rt,
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
        // D s_2_0: read-var Rt:u8
        let s_2_0: u8 = fn_state.Rt;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 3u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #13s : i64
        let s_2_4: i64 = 13;
        // C s_2_5: const #32s : i
        let s_2_5: i128 = 32;
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #8s : i
        let s_2_7: i128 = 8;
        // C s_2_8: const #2s : i
        let s_2_8: i128 = 2;
        // D s_2_9: read-var imm8:u8
        let s_2_9: u8 = fn_state.imm8;
        // D s_2_10: cast zx s_2_9 -> bv
        let s_2_10: Bits = Bits::new(s_2_9 as u128, 8u16);
        // D s_2_11: call place_slice(s_2_5, s_2_10, s_2_6, s_2_7, s_2_8)
        let s_2_11: Bits = place_slice(
            state,
            tracer,
            s_2_5,
            s_2_10,
            s_2_6,
            s_2_7,
            s_2_8,
        );
        // D s_2_12: cast reint s_2_11 -> u32
        let s_2_12: u32 = (s_2_11.value() as u32);
        // C s_2_13: const #1u : u8
        let s_2_13: bool = true;
        // C s_2_14: const #1u : u8
        let s_2_14: bool = true;
        // C s_2_15: const #0u : u8
        let s_2_15: bool = false;
        // D s_2_16: call execute_aarch32_instrs_LDR_i_OpT_A_txt(s_2_14, s_2_12, s_2_13, s_2_4, s_2_3, s_2_15)
        let s_2_16: () = execute_aarch32_instrs_LDR_i_OpT_A_txt(
            state,
            tracer,
            s_2_14,
            s_2_12,
            s_2_13,
            s_2_4,
            s_2_3,
            s_2_15,
        );
        // N s_2_17: return
        return;
    }
}