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
use execute_aarch32_instrs_MRS_br_Op_AS_txt::*;
use common::*;
pub fn decode_aarch32_instrs_MRS_br_T1enc_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    R: bool,
    M1: u8,
    Rd: u8,
    M: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        read_spsr: bool,
        d: i64,
        R: bool,
        M1: u8,
        Rd: u8,
        M: bool,
    }
    let fn_state = FunctionState {
        R,
        M1,
        Rd,
        M,
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
        // D s_2_0: read-var Rd:u8
        let s_2_0: u8 = fn_state.Rd;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var d <= s_2_3
        fn_state.d = s_2_3;
        // D s_2_5: read-var R:u8
        let s_2_5: bool = fn_state.R;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 1u16);
        // C s_2_7: const #1u : u8
        let s_2_7: bool = true;
        // C s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 1u16);
        // D s_2_9: cmp-eq s_2_6 s_2_8
        let s_2_9: bool = ((s_2_6) == (s_2_8));
        // D s_2_10: write-var read_spsr <= s_2_9
        fn_state.read_spsr = s_2_9;
        // C s_2_11: const #15s : i
        let s_2_11: i128 = 15;
        // D s_2_12: read-var d:i64
        let s_2_12: i64 = fn_state.d;
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (i128::try_from(s_2_12).unwrap());
        // D s_2_14: cmp-eq s_2_13 s_2_11
        let s_2_14: bool = ((s_2_13) == (s_2_11));
        // N s_2_15: branch s_2_14 b4 b3
        if s_2_14 {
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
        // D s_3_0: read-var M:u8
        let s_3_0: bool = fn_state.M;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // D s_3_2: read-var M1:u8
        let s_3_2: u8 = fn_state.M1;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // D s_3_4: cast reint s_3_1 -> u128
        let s_3_4: u128 = (s_3_1.value() as u128);
        // D s_3_5: size-of s_3_1
        let s_3_5: u16 = s_3_1.length();
        // D s_3_6: cast reint s_3_3 -> u128
        let s_3_6: u128 = (s_3_3.value() as u128);
        // D s_3_7: size-of s_3_3
        let s_3_7: u16 = s_3_3.length();
        // D s_3_8: lsl s_3_4 s_3_7
        let s_3_8: u128 = s_3_4 << s_3_7;
        // D s_3_9: or s_3_8 s_3_6
        let s_3_9: u128 = ((s_3_8) | (s_3_6));
        // D s_3_10: add s_3_5 s_3_7
        let s_3_10: u16 = (s_3_5 + s_3_7);
        // D s_3_11: create-bits s_3_9 s_3_10
        let s_3_11: Bits = Bits::new(s_3_9, s_3_10);
        // D s_3_12: cast reint s_3_11 -> u8
        let s_3_12: u8 = (s_3_11.value() as u8);
        // D s_3_13: read-var d:i64
        let s_3_13: i64 = fn_state.d;
        // D s_3_14: read-var read_spsr:u8
        let s_3_14: bool = fn_state.read_spsr;
        // D s_3_15: call execute_aarch32_instrs_MRS_br_Op_AS_txt(s_3_12, s_3_13, s_3_14)
        let s_3_15: () = execute_aarch32_instrs_MRS_br_Op_AS_txt(
            state,
            tracer,
            s_3_12,
            s_3_13,
            s_3_14,
        );
        // N s_3_16: return
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
