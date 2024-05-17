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
use neq_int::*;
use execute_aarch32_instrs_LDRSH_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDRSH_i_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rn: u8,
    Rt: u8,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rn: u8,
        Rt: u8,
        imm12: u16,
    }
    let fn_state = FunctionState {
        Rn,
        Rt,
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
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b6 b3
        if s_2_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rt:u8
        let s_3_0: u8 = fn_state.Rt;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // C s_3_2: const #15u : u8
        let s_3_2: u8 = 15;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b5 b4
        if s_3_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Rt:u8
        let s_4_0: u8 = fn_state.Rt;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 4u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #15s : i
        let s_4_4: i128 = 15;
        // D s_4_5: cast zx s_4_3 -> i
        let s_4_5: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_6: call neq_int(s_4_5, s_4_4)
        let s_4_6: bool = neq_int(state, tracer, s_4_5, s_4_4);
        // N s_4_7: assert s_4_6
        let s_4_7: () = assert!(s_4_6);
        // D s_4_8: read-var Rn:u8
        let s_4_8: u8 = fn_state.Rn;
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 4u16);
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (s_4_9.value() as i128);
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // C s_4_12: const #32s : i
        let s_4_12: i128 = 32;
        // D s_4_13: read-var imm12:u12
        let s_4_13: u16 = fn_state.imm12;
        // D s_4_14: cast zx s_4_13 -> bv
        let s_4_14: Bits = Bits::new(s_4_13 as u128, 12u16);
        // D s_4_15: bits-cast zx s_4_14 -> bv length s_4_12
        let s_4_15: Bits = s_4_14.zero_extend(s_4_12);
        // D s_4_16: cast reint s_4_15 -> u32
        let s_4_16: u32 = (s_4_15.value() as u32);
        // C s_4_17: const #1u : u8
        let s_4_17: bool = true;
        // C s_4_18: const #1u : u8
        let s_4_18: bool = true;
        // C s_4_19: const #0u : u8
        let s_4_19: bool = false;
        // D s_4_20: call execute_aarch32_instrs_LDRSH_i_Op_A_txt(s_4_18, s_4_16, s_4_17, s_4_11, s_4_3, s_4_19)
        let s_4_20: () = execute_aarch32_instrs_LDRSH_i_Op_A_txt(
            state,
            tracer,
            s_4_18,
            s_4_16,
            s_4_17,
            s_4_11,
            s_4_3,
            s_4_19,
        );
        // N s_4_21: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
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
}
