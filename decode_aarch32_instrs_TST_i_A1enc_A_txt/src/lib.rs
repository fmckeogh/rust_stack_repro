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
use execute_aarch32_instrs_TST_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_TST_i_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    Rn: u8,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_348752: ProductType4813027798de1e98,
        cond: u8,
        Rn: u8,
        imm12: u16,
    }
    let fn_state = FunctionState {
        cond,
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
        // D s_2_6: read-var Rn:u8
        let s_2_6: u8 = fn_state.Rn;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // C s_2_10: const #16971u : u32
        let s_2_10: u32 = 16971;
        // D s_2_11: read-reg s_2_10:u8
        let s_2_11: bool = {
            let value = state.read_register::<bool>(s_2_10 as isize);
            tracer.read_register(s_2_10 as isize, value);
            value
        };
        // D s_2_12: read-var imm12:u12
        let s_2_12: u16 = fn_state.imm12;
        // D s_2_13: call A32ExpandImm_C(s_2_12, s_2_11)
        let s_2_13: ProductType4813027798de1e98 = A32ExpandImm_C(
            state,
            tracer,
            s_2_12,
            s_2_11,
        );
        // D s_2_14: write-var ga#348752 <= s_2_13
        fn_state.ga_348752 = s_2_13;
        // D s_2_15: read-var ga#348752.0:struct
        let s_2_15: u32 = fn_state.ga_348752._0;
        // D s_2_16: read-var ga#348752.1:struct
        let s_2_16: bool = fn_state.ga_348752._1;
        // D s_2_17: call execute_aarch32_instrs_TST_i_Op_A_txt(s_2_16, s_2_15, s_2_9)
        let s_2_17: () = execute_aarch32_instrs_TST_i_Op_A_txt(
            state,
            tracer,
            s_2_16,
            s_2_15,
            s_2_9,
        );
        // N s_2_18: return
        return;
    }
}
