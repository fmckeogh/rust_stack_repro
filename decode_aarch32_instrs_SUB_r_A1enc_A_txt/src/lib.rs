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
use execute_aarch32_instrs_SUB_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_SUB_r_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    S: bool,
    Rn: u8,
    Rd: u8,
    imm5: u8,
    stype: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_348415: ProductType396b95aa74979079,
        cond: u8,
        S: bool,
        Rn: u8,
        Rd: u8,
        imm5: u8,
        stype: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        cond,
        S,
        Rn,
        Rd,
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
        // C s_2_8: const #13u : u8
        let s_2_8: u8 = 13;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 4u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // N s_2_11: branch s_2_10 b4 b3
        if s_2_10 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rd:u8
        let s_3_0: u8 = fn_state.Rd;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: read-var Rn:u8
        let s_3_4: u8 = fn_state.Rn;
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 4u16);
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (s_3_5.value() as i128);
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: read-var Rm:u8
        let s_3_8: u8 = fn_state.Rm;
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 4u16);
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (s_3_9.value() as i128);
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: read-var S:u8
        let s_3_12: bool = fn_state.S;
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 1u16);
        // C s_3_14: const #1u : u8
        let s_3_14: bool = true;
        // C s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 1u16);
        // D s_3_16: cmp-eq s_3_13 s_3_15
        let s_3_16: bool = ((s_3_13) == (s_3_15));
        // D s_3_17: read-var stype:u8
        let s_3_17: u8 = fn_state.stype;
        // D s_3_18: read-var imm5:u8
        let s_3_18: u8 = fn_state.imm5;
        // D s_3_19: call DecodeImmShift(s_3_17, s_3_18)
        let s_3_19: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_3_17,
            s_3_18,
        );
        // D s_3_20: write-var ga#348415 <= s_3_19
        fn_state.ga_348415 = s_3_19;
        // D s_3_21: read-var ga#348415.0:struct
        let s_3_21: u32 = fn_state.ga_348415._0;
        // D s_3_22: read-var ga#348415.1:struct
        let s_3_22: i128 = fn_state.ga_348415._1;
        // D s_3_23: call execute_aarch32_instrs_SUB_r_Op_A_txt(s_3_3, s_3_11, s_3_7, s_3_16, s_3_22, s_3_21)
        let s_3_23: () = execute_aarch32_instrs_SUB_r_Op_A_txt(
            state,
            tracer,
            s_3_3,
            s_3_11,
            s_3_7,
            s_3_16,
            s_3_22,
            s_3_21,
        );
        // N s_3_24: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
}
