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
use DecodeImmShift::*;
use execute_aarch32_instrs_TEQ_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_TEQ_r_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    Rn: u8,
    imm5: u8,
    stype: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_348720: ProductType396b95aa74979079,
        cond: u8,
        Rn: u8,
        imm5: u8,
        stype: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        cond,
        Rn,
        imm5,
        stype,
        Rm,
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
        // D s_2_10: read-var Rm:u8
        let s_2_10: u8 = fn_state.Rm;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 4u16);
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (s_2_11.value() as i128);
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: read-var stype:u8
        let s_2_14: u8 = fn_state.stype;
        // D s_2_15: read-var imm5:u8
        let s_2_15: u8 = fn_state.imm5;
        // D s_2_16: call DecodeImmShift(s_2_14, s_2_15)
        let s_2_16: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_2_14,
            s_2_15,
        );
        // D s_2_17: write-var ga#348720 <= s_2_16
        fn_state.ga_348720 = s_2_16;
        // D s_2_18: read-var ga#348720.0:struct
        let s_2_18: u32 = fn_state.ga_348720._0;
        // D s_2_19: read-var ga#348720.1:struct
        let s_2_19: i128 = fn_state.ga_348720._1;
        // D s_2_20: call execute_aarch32_instrs_TEQ_r_Op_A_txt(s_2_13, s_2_9, s_2_19, s_2_18)
        let s_2_20: () = execute_aarch32_instrs_TEQ_r_Op_A_txt(
            state,
            tracer,
            s_2_13,
            s_2_9,
            s_2_19,
            s_2_18,
        );
        // N s_2_21: return
        return;
    }
}
