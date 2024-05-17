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
use execute_aarch32_instrs_SVC_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_SVC_A1enc_A_txt<T: Tracer>(
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
        // C s_2_6: const #32s : i
        let s_2_6: i128 = 32;
        // D s_2_7: read-var imm24:u24
        let s_2_7: u32 = fn_state.imm24;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 24u16);
        // D s_2_9: bits-cast zx s_2_8 -> bv length s_2_6
        let s_2_9: Bits = s_2_8.zero_extend(s_2_6);
        // D s_2_10: cast reint s_2_9 -> u32
        let s_2_10: u32 = (s_2_9.value() as u32);
        // D s_2_11: call execute_aarch32_instrs_SVC_Op_A_txt(s_2_10)
        let s_2_11: () = execute_aarch32_instrs_SVC_Op_A_txt(state, tracer, s_2_10);
        // N s_2_12: return
        return;
    }
}
