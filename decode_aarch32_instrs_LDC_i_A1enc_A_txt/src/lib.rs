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
use neq_int::*;
use execute_aarch32_instrs_LDC_i_Op_A_txt::*;
use ConditionPassed::*;
use place_slice::*;
use common::*;
pub fn decode_aarch32_instrs_LDC_i_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    P: bool,
    U: bool,
    W: bool,
    Rn: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_296732: bool,
        gs_296731: bool,
        cond: u8,
        P: bool,
        U: bool,
        W: bool,
        Rn: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        cond,
        P,
        U,
        W,
        Rn,
        imm8,
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
        // C s_2_8: const #15u : u8
        let s_2_8: u8 = 15;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 4u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // N s_2_11: branch s_2_10 b12 b3
        if s_2_10 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var P:u8
        let s_3_0: bool = fn_state.P;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b11 b4
        if s_3_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#296731 <= s_4_0
        fn_state.gs_296731 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#296731:u8
        let s_5_0: bool = fn_state.gs_296731;
        // N s_5_1: branch s_5_0 b10 b6
        if s_5_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#296732 <= s_6_0
        fn_state.gs_296732 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#296732:u8
        let s_7_0: bool = fn_state.gs_296732;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var Rn:u8
        let s_8_0: u8 = fn_state.Rn;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 4u16);
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (s_8_1.value() as i128);
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // C s_8_4: const #15s : i
        let s_8_4: i128 = 15;
        // D s_8_5: cast zx s_8_3 -> i
        let s_8_5: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_6: call neq_int(s_8_5, s_8_4)
        let s_8_6: bool = neq_int(state, tracer, s_8_5, s_8_4);
        // N s_8_7: assert s_8_6
        let s_8_7: () = assert!(s_8_6);
        // C s_8_8: const #14s : i64
        let s_8_8: i64 = 14;
        // C s_8_9: const #32s : i
        let s_8_9: i128 = 32;
        // C s_8_10: const #0s : i
        let s_8_10: i128 = 0;
        // C s_8_11: const #8s : i
        let s_8_11: i128 = 8;
        // C s_8_12: const #2s : i
        let s_8_12: i128 = 2;
        // D s_8_13: read-var imm8:u8
        let s_8_13: u8 = fn_state.imm8;
        // D s_8_14: cast zx s_8_13 -> bv
        let s_8_14: Bits = Bits::new(s_8_13 as u128, 8u16);
        // D s_8_15: call place_slice(s_8_9, s_8_14, s_8_10, s_8_11, s_8_12)
        let s_8_15: Bits = place_slice(
            state,
            tracer,
            s_8_9,
            s_8_14,
            s_8_10,
            s_8_11,
            s_8_12,
        );
        // D s_8_16: cast reint s_8_15 -> u32
        let s_8_16: u32 = (s_8_15.value() as u32);
        // D s_8_17: read-var P:u8
        let s_8_17: bool = fn_state.P;
        // D s_8_18: cast zx s_8_17 -> bv
        let s_8_18: Bits = Bits::new(s_8_17 as u128, 1u16);
        // C s_8_19: const #1u : u8
        let s_8_19: bool = true;
        // C s_8_20: cast zx s_8_19 -> bv
        let s_8_20: Bits = Bits::new(s_8_19 as u128, 1u16);
        // D s_8_21: cmp-eq s_8_18 s_8_20
        let s_8_21: bool = ((s_8_18) == (s_8_20));
        // D s_8_22: read-var U:u8
        let s_8_22: bool = fn_state.U;
        // D s_8_23: cast zx s_8_22 -> bv
        let s_8_23: Bits = Bits::new(s_8_22 as u128, 1u16);
        // C s_8_24: const #1u : u8
        let s_8_24: bool = true;
        // C s_8_25: cast zx s_8_24 -> bv
        let s_8_25: Bits = Bits::new(s_8_24 as u128, 1u16);
        // D s_8_26: cmp-eq s_8_23 s_8_25
        let s_8_26: bool = ((s_8_23) == (s_8_25));
        // D s_8_27: read-var W:u8
        let s_8_27: bool = fn_state.W;
        // D s_8_28: cast zx s_8_27 -> bv
        let s_8_28: Bits = Bits::new(s_8_27 as u128, 1u16);
        // C s_8_29: const #1u : u8
        let s_8_29: bool = true;
        // C s_8_30: cast zx s_8_29 -> bv
        let s_8_30: Bits = Bits::new(s_8_29 as u128, 1u16);
        // D s_8_31: cmp-eq s_8_28 s_8_30
        let s_8_31: bool = ((s_8_28) == (s_8_30));
        // D s_8_32: call execute_aarch32_instrs_LDC_i_Op_A_txt(s_8_26, s_8_8, s_8_16, s_8_21, s_8_3, s_8_31)
        let s_8_32: () = execute_aarch32_instrs_LDC_i_Op_A_txt(
            state,
            tracer,
            s_8_26,
            s_8_8,
            s_8_16,
            s_8_21,
            s_8_3,
            s_8_31,
        );
        // N s_8_33: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var W:u8
        let s_10_0: bool = fn_state.W;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #0u : u8
        let s_10_2: bool = false;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: write-var gs#296732 <= s_10_4
        fn_state.gs_296732 = s_10_4;
        // N s_10_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var U:u8
        let s_11_0: bool = fn_state.U;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #0u : u8
        let s_11_2: bool = false;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: write-var gs#296731 <= s_11_4
        fn_state.gs_296731 = s_11_4;
        // N s_11_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
}
