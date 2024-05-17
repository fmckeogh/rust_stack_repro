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
use execute_aarch32_instrs_VMOV_s_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMOV_s_A1enc_A_txt<T: Tracer>(
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
        // D s_2_6: read-var op:u8
        let s_2_6: bool = fn_state.op;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // C s_2_8: const #1u : u8
        let s_2_8: bool = true;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // D s_2_11: write-var to_arm_register <= s_2_10
        fn_state.to_arm_register = s_2_10;
        // D s_2_12: read-var Rt:u8
        let s_2_12: u8 = fn_state.Rt;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 4u16);
        // D s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (s_2_13.value() as i128);
        // D s_2_15: cast reint s_2_14 -> i64
        let s_2_15: i64 = (s_2_14 as i64);
        // D s_2_16: write-var t <= s_2_15
        fn_state.t = s_2_15;
        // D s_2_17: read-var Vn:u8
        let s_2_17: u8 = fn_state.Vn;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 4u16);
        // D s_2_19: read-var N:u8
        let s_2_19: bool = fn_state.N;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_21: cast reint s_2_18 -> u128
        let s_2_21: u128 = (s_2_18.value() as u128);
        // D s_2_22: size-of s_2_18
        let s_2_22: u16 = s_2_18.length();
        // D s_2_23: cast reint s_2_20 -> u128
        let s_2_23: u128 = (s_2_20.value() as u128);
        // D s_2_24: size-of s_2_20
        let s_2_24: u16 = s_2_20.length();
        // D s_2_25: lsl s_2_21 s_2_24
        let s_2_25: u128 = s_2_21 << s_2_24;
        // D s_2_26: or s_2_25 s_2_23
        let s_2_26: u128 = ((s_2_25) | (s_2_23));
        // D s_2_27: add s_2_22 s_2_24
        let s_2_27: u16 = (s_2_22 + s_2_24);
        // D s_2_28: create-bits s_2_26 s_2_27
        let s_2_28: Bits = Bits::new(s_2_26, s_2_27);
        // D s_2_29: cast reint s_2_28 -> u8
        let s_2_29: u8 = (s_2_28.value() as u8);
        // D s_2_30: cast zx s_2_29 -> bv
        let s_2_30: Bits = Bits::new(s_2_29 as u128, 5u16);
        // D s_2_31: cast zx s_2_30 -> i
        let s_2_31: i128 = (s_2_30.value() as i128);
        // D s_2_32: cast reint s_2_31 -> i64
        let s_2_32: i64 = (s_2_31 as i64);
        // D s_2_33: write-var n <= s_2_32
        fn_state.n = s_2_32;
        // C s_2_34: const #15s : i
        let s_2_34: i128 = 15;
        // D s_2_35: read-var t:i64
        let s_2_35: i64 = fn_state.t;
        // D s_2_36: cast zx s_2_35 -> i
        let s_2_36: i128 = (i128::try_from(s_2_35).unwrap());
        // D s_2_37: cmp-eq s_2_36 s_2_34
        let s_2_37: bool = ((s_2_36) == (s_2_34));
        // N s_2_38: branch s_2_37 b4 b3
        if s_2_37 {
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
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: read-var t:i64
        let s_3_1: i64 = fn_state.t;
        // D s_3_2: read-var to_arm_register:u8
        let s_3_2: bool = fn_state.to_arm_register;
        // D s_3_3: call execute_aarch32_instrs_VMOV_s_Op_A_txt(s_3_0, s_3_1, s_3_2)
        let s_3_3: () = execute_aarch32_instrs_VMOV_s_Op_A_txt(
            state,
            tracer,
            s_3_0,
            s_3_1,
            s_3_2,
        );
        // N s_3_4: return
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
