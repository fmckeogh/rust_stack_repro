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
use HaveFP16Ext::*;
use execute_aarch32_instrs_VMOV_h_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMOV_h_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    op: bool,
    Vn: u8,
    Rt: u8,
    N: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        to_arm_register: bool,
        t: i64,
        n: i64,
        cond: u8,
        op: bool,
        Vn: u8,
        Rt: u8,
        N: bool,
    }
    let fn_state = FunctionState {
        cond,
        op,
        Vn,
        Rt,
        N,
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
        // C s_2_6: const #() : ()
        let s_2_6: () = ();
        // S s_2_7: call HaveFP16Ext(s_2_6)
        let s_2_7: bool = HaveFP16Ext(state, tracer, s_2_6);
        // S s_2_8: not s_2_7
        let s_2_8: bool = !s_2_7;
        // N s_2_9: branch s_2_8 b8 b3
        if s_2_8 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var cond:u8
        let s_3_0: u8 = fn_state.cond;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // C s_3_2: const #14u : u8
        let s_3_2: u8 = 14;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // D s_3_4: cmp-ne s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) != (s_3_3));
        // N s_3_5: branch s_3_4 b7 b4
        if s_3_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var op:u8
        let s_4_0: bool = fn_state.op;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: write-var to_arm_register <= s_4_4
        fn_state.to_arm_register = s_4_4;
        // D s_4_6: read-var Rt:u8
        let s_4_6: u8 = fn_state.Rt;
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 4u16);
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (s_4_7.value() as i128);
        // D s_4_9: cast reint s_4_8 -> i64
        let s_4_9: i64 = (s_4_8 as i64);
        // D s_4_10: write-var t <= s_4_9
        fn_state.t = s_4_9;
        // D s_4_11: read-var Vn:u8
        let s_4_11: u8 = fn_state.Vn;
        // D s_4_12: cast zx s_4_11 -> bv
        let s_4_12: Bits = Bits::new(s_4_11 as u128, 4u16);
        // D s_4_13: read-var N:u8
        let s_4_13: bool = fn_state.N;
        // D s_4_14: cast zx s_4_13 -> bv
        let s_4_14: Bits = Bits::new(s_4_13 as u128, 1u16);
        // D s_4_15: cast reint s_4_12 -> u128
        let s_4_15: u128 = (s_4_12.value() as u128);
        // D s_4_16: size-of s_4_12
        let s_4_16: u16 = s_4_12.length();
        // D s_4_17: cast reint s_4_14 -> u128
        let s_4_17: u128 = (s_4_14.value() as u128);
        // D s_4_18: size-of s_4_14
        let s_4_18: u16 = s_4_14.length();
        // D s_4_19: lsl s_4_15 s_4_18
        let s_4_19: u128 = s_4_15 << s_4_18;
        // D s_4_20: or s_4_19 s_4_17
        let s_4_20: u128 = ((s_4_19) | (s_4_17));
        // D s_4_21: add s_4_16 s_4_18
        let s_4_21: u16 = (s_4_16 + s_4_18);
        // D s_4_22: create-bits s_4_20 s_4_21
        let s_4_22: Bits = Bits::new(s_4_20, s_4_21);
        // D s_4_23: cast reint s_4_22 -> u8
        let s_4_23: u8 = (s_4_22.value() as u8);
        // D s_4_24: cast zx s_4_23 -> bv
        let s_4_24: Bits = Bits::new(s_4_23 as u128, 5u16);
        // D s_4_25: cast zx s_4_24 -> i
        let s_4_25: i128 = (s_4_24.value() as i128);
        // D s_4_26: cast reint s_4_25 -> i64
        let s_4_26: i64 = (s_4_25 as i64);
        // D s_4_27: write-var n <= s_4_26
        fn_state.n = s_4_26;
        // C s_4_28: const #15s : i
        let s_4_28: i128 = 15;
        // D s_4_29: read-var t:i64
        let s_4_29: i64 = fn_state.t;
        // D s_4_30: cast zx s_4_29 -> i
        let s_4_30: i128 = (i128::try_from(s_4_29).unwrap());
        // D s_4_31: cmp-eq s_4_30 s_4_28
        let s_4_31: bool = ((s_4_30) == (s_4_28));
        // N s_4_32: branch s_4_31 b6 b5
        if s_4_31 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: read-var t:i64
        let s_5_1: i64 = fn_state.t;
        // D s_5_2: read-var to_arm_register:u8
        let s_5_2: bool = fn_state.to_arm_register;
        // D s_5_3: call execute_aarch32_instrs_VMOV_h_Op_A_txt(s_5_0, s_5_1, s_5_2)
        let s_5_3: () = execute_aarch32_instrs_VMOV_h_Op_A_txt(
            state,
            tracer,
            s_5_0,
            s_5_1,
            s_5_2,
        );
        // N s_5_4: return
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
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
}
