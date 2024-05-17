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
use execute_aarch32_instrs_PLI_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_PLI_i_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    Rn: u8,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        U: bool,
        Rn: u8,
        imm12: u16,
    }
    let fn_state = FunctionState {
        U,
        Rn,
        imm12,
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
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #32s : i
        let s_2_4: i128 = 32;
        // D s_2_5: read-var imm12:u12
        let s_2_5: u16 = fn_state.imm12;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 12u16);
        // D s_2_7: bits-cast zx s_2_6 -> bv length s_2_4
        let s_2_7: Bits = s_2_6.zero_extend(s_2_4);
        // D s_2_8: cast reint s_2_7 -> u32
        let s_2_8: u32 = (s_2_7.value() as u32);
        // D s_2_9: read-var U:u8
        let s_2_9: bool = fn_state.U;
        // D s_2_10: cast zx s_2_9 -> bv
        let s_2_10: Bits = Bits::new(s_2_9 as u128, 1u16);
        // C s_2_11: const #1u : u8
        let s_2_11: bool = true;
        // C s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 1u16);
        // D s_2_13: cmp-eq s_2_10 s_2_12
        let s_2_13: bool = ((s_2_10) == (s_2_12));
        // D s_2_14: call execute_aarch32_instrs_PLI_i_Op_A_txt(s_2_13, s_2_8, s_2_3)
        let s_2_14: () = execute_aarch32_instrs_PLI_i_Op_A_txt(
            state,
            tracer,
            s_2_13,
            s_2_8,
            s_2_3,
        );
        // N s_2_15: return
        return;
    }
}
