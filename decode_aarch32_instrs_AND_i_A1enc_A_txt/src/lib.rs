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
use A32ExpandImm_C::*;
use execute_aarch32_instrs_AND_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_AND_i_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    S: bool,
    Rn: u8,
    Rd: u8,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_343061: ProductType4813027798de1e98,
        cond: u8,
        S: bool,
        Rn: u8,
        Rd: u8,
        imm12: u16,
    }
    let fn_state = FunctionState {
        cond,
        S,
        Rn,
        Rd,
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
        // D s_2_6: read-var Rd:u8
        let s_2_6: u8 = fn_state.Rd;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: read-var Rn:u8
        let s_2_10: u8 = fn_state.Rn;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 4u16);
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (s_2_11.value() as i128);
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: read-var S:u8
        let s_2_14: bool = fn_state.S;
        // D s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 1u16);
        // C s_2_16: const #1u : u8
        let s_2_16: bool = true;
        // C s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 1u16);
        // D s_2_18: cmp-eq s_2_15 s_2_17
        let s_2_18: bool = ((s_2_15) == (s_2_17));
        // C s_2_19: const #16971u : u32
        let s_2_19: u32 = 16971;
        // D s_2_20: read-reg s_2_19:u8
        let s_2_20: bool = {
            let value = state.read_register::<bool>(s_2_19 as isize);
            tracer.read_register(s_2_19 as isize, value);
            value
        };
        // D s_2_21: read-var imm12:u12
        let s_2_21: u16 = fn_state.imm12;
        // D s_2_22: call A32ExpandImm_C(s_2_21, s_2_20)
        let s_2_22: ProductType4813027798de1e98 = A32ExpandImm_C(
            state,
            tracer,
            s_2_21,
            s_2_20,
        );
        // D s_2_23: write-var ga#343061 <= s_2_22
        fn_state.ga_343061 = s_2_22;
        // D s_2_24: read-var ga#343061.0:struct
        let s_2_24: u32 = fn_state.ga_343061._0;
        // D s_2_25: read-var ga#343061.1:struct
        let s_2_25: bool = fn_state.ga_343061._1;
        // D s_2_26: call execute_aarch32_instrs_AND_i_Op_A_txt(s_2_25, s_2_9, s_2_24, s_2_13, s_2_18)
        let s_2_26: () = execute_aarch32_instrs_AND_i_Op_A_txt(
            state,
            tracer,
            s_2_25,
            s_2_9,
            s_2_24,
            s_2_13,
            s_2_18,
        );
        // N s_2_27: return
        return;
    }
}