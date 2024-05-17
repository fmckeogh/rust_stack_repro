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
use execute_aarch32_instrs_LDRSH_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDRSH_r_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rm: u8,
    Rn: u8,
    Rt: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rm: u8,
        Rn: u8,
        Rt: u8,
    }
    let fn_state = FunctionState {
        Rm,
        Rn,
        Rt,
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
        // D s_2_4: read-var Rn:u8
        let s_2_4: u8 = fn_state.Rn;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 3u16);
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (s_2_5.value() as i128);
        // D s_2_7: cast reint s_2_6 -> i64
        let s_2_7: i64 = (s_2_6 as i64);
        // D s_2_8: read-var Rm:u8
        let s_2_8: u8 = fn_state.Rm;
        // D s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 3u16);
        // D s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (s_2_9.value() as i128);
        // D s_2_11: cast reint s_2_10 -> i64
        let s_2_11: i64 = (s_2_10 as i64);
        // C s_2_12: const #1u : u8
        let s_2_12: bool = true;
        // C s_2_13: const #1u : u8
        let s_2_13: bool = true;
        // C s_2_14: const #0u : u8
        let s_2_14: bool = false;
        // C s_2_15: const #0s : i
        let s_2_15: i128 = 0;
        // C s_2_16: const #0u : u32
        let s_2_16: u32 = 0;
        // D s_2_17: create-product struct = ["s_2_16", "s_2_15"]
        let s_2_17: ProductType396b95aa74979079 = ProductType396b95aa74979079 {
            _0: s_2_16,
            _1: s_2_15,
        };
        // D s_2_18: extract-field s_2_17.0
        let s_2_18: u32 = s_2_17._0;
        // C s_2_19: const #0u : u32
        let s_2_19: u32 = 0;
        // D s_2_20: create-product struct = ["s_2_19", "s_2_15"]
        let s_2_20: ProductType396b95aa74979079 = ProductType396b95aa74979079 {
            _0: s_2_19,
            _1: s_2_15,
        };
        // D s_2_21: extract-field s_2_20.1
        let s_2_21: i128 = s_2_20._1;
        // D s_2_22: cast reint s_2_21 -> i64
        let s_2_22: i64 = (s_2_21 as i64);
        // D s_2_23: cast zx s_2_22 -> i
        let s_2_23: i128 = (i128::try_from(s_2_22).unwrap());
        // D s_2_24: call execute_aarch32_instrs_LDRSH_r_Op_A_txt(s_2_13, s_2_12, s_2_11, s_2_7, s_2_23, s_2_18, s_2_3, s_2_14)
        let s_2_24: () = execute_aarch32_instrs_LDRSH_r_Op_A_txt(
            state,
            tracer,
            s_2_13,
            s_2_12,
            s_2_11,
            s_2_7,
            s_2_23,
            s_2_18,
            s_2_3,
            s_2_14,
        );
        // N s_2_25: return
        return;
    }
}
