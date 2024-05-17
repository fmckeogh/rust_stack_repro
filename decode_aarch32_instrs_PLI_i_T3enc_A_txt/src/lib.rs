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
pub fn decode_aarch32_instrs_PLI_i_T3enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        U: bool,
        imm12: u16,
    }
    let fn_state = FunctionState {
        U,
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
        // C s_2_0: const #15s : i64
        let s_2_0: i64 = 15;
        // C s_2_1: const #32s : i
        let s_2_1: i128 = 32;
        // D s_2_2: read-var imm12:u12
        let s_2_2: u16 = fn_state.imm12;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 12u16);
        // D s_2_4: bits-cast zx s_2_3 -> bv length s_2_1
        let s_2_4: Bits = s_2_3.zero_extend(s_2_1);
        // D s_2_5: cast reint s_2_4 -> u32
        let s_2_5: u32 = (s_2_4.value() as u32);
        // D s_2_6: read-var U:u8
        let s_2_6: bool = fn_state.U;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // C s_2_8: const #1u : u8
        let s_2_8: bool = true;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // D s_2_11: call execute_aarch32_instrs_PLI_i_Op_A_txt(s_2_10, s_2_5, s_2_0)
        let s_2_11: () = execute_aarch32_instrs_PLI_i_Op_A_txt(
            state,
            tracer,
            s_2_10,
            s_2_5,
            s_2_0,
        );
        // N s_2_12: return
        return;
    }
}
