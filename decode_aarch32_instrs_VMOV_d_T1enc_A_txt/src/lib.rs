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
use execute_aarch32_instrs_VMOV_d_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMOV_d_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op: bool,
    Rt2: u8,
    Rt: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        t: i64,
        t2: i64,
        to_arm_registers: bool,
        gs_312820: bool,
        gs_312819: bool,
        op: bool,
        Rt2: u8,
        Rt: u8,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        op,
        Rt2,
        Rt,
        M,
        Vm,
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
        // D s_2_0: read-var op:u8
        let s_2_0: bool = fn_state.op;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // D s_2_5: write-var to_arm_registers <= s_2_4
        fn_state.to_arm_registers = s_2_4;
        // D s_2_6: read-var Rt:u8
        let s_2_6: u8 = fn_state.Rt;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var t <= s_2_9
        fn_state.t = s_2_9;
        // D s_2_11: read-var Rt2:u8
        let s_2_11: u8 = fn_state.Rt2;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 4u16);
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (s_2_12.value() as i128);
        // D s_2_14: cast reint s_2_13 -> i64
        let s_2_14: i64 = (s_2_13 as i64);
        // D s_2_15: write-var t2 <= s_2_14
        fn_state.t2 = s_2_14;
        // D s_2_16: read-var M:u8
        let s_2_16: bool = fn_state.M;
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 1u16);
        // D s_2_18: read-var Vm:u8
        let s_2_18: u8 = fn_state.Vm;
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 4u16);
        // D s_2_20: cast reint s_2_17 -> u128
        let s_2_20: u128 = (s_2_17.value() as u128);
        // D s_2_21: size-of s_2_17
        let s_2_21: u16 = s_2_17.length();
        // D s_2_22: cast reint s_2_19 -> u128
        let s_2_22: u128 = (s_2_19.value() as u128);
        // D s_2_23: size-of s_2_19
        let s_2_23: u16 = s_2_19.length();
        // D s_2_24: lsl s_2_20 s_2_23
        let s_2_24: u128 = s_2_20 << s_2_23;
        // D s_2_25: or s_2_24 s_2_22
        let s_2_25: u128 = ((s_2_24) | (s_2_22));
        // D s_2_26: add s_2_21 s_2_23
        let s_2_26: u16 = (s_2_21 + s_2_23);
        // D s_2_27: create-bits s_2_25 s_2_26
        let s_2_27: Bits = Bits::new(s_2_25, s_2_26);
        // D s_2_28: cast reint s_2_27 -> u8
        let s_2_28: u8 = (s_2_27.value() as u8);
        // D s_2_29: cast zx s_2_28 -> bv
        let s_2_29: Bits = Bits::new(s_2_28 as u128, 5u16);
        // D s_2_30: cast zx s_2_29 -> i
        let s_2_30: i128 = (s_2_29.value() as i128);
        // D s_2_31: cast reint s_2_30 -> i64
        let s_2_31: i64 = (s_2_30 as i64);
        // D s_2_32: write-var m <= s_2_31
        fn_state.m = s_2_31;
        // C s_2_33: const #15s : i
        let s_2_33: i128 = 15;
        // D s_2_34: read-var t:i64
        let s_2_34: i64 = fn_state.t;
        // D s_2_35: cast zx s_2_34 -> i
        let s_2_35: i128 = (i128::try_from(s_2_34).unwrap());
        // D s_2_36: cmp-eq s_2_35 s_2_33
        let s_2_36: bool = ((s_2_35) == (s_2_33));
        // N s_2_37: branch s_2_36 b12 b3
        if s_2_36 {
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
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var t2:i64
        let s_3_1: i64 = fn_state.t2;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // D s_3_4: write-var gs#312819 <= s_3_3
        fn_state.gs_312819 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#312819:u8
        let s_4_0: bool = fn_state.gs_312819;
        // N s_4_1: branch s_4_0 b11 b5
        if s_4_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var to_arm_registers:u8
        let s_5_0: bool = fn_state.to_arm_registers;
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
        // D s_6_1: write-var gs#312820 <= s_6_0
        fn_state.gs_312820 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#312820:u8
        let s_7_0: bool = fn_state.gs_312820;
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
        // D s_8_0: read-var m:i64
        let s_8_0: i64 = fn_state.m;
        // D s_8_1: read-var t:i64
        let s_8_1: i64 = fn_state.t;
        // D s_8_2: read-var t2:i64
        let s_8_2: i64 = fn_state.t2;
        // D s_8_3: read-var to_arm_registers:u8
        let s_8_3: bool = fn_state.to_arm_registers;
        // D s_8_4: call execute_aarch32_instrs_VMOV_d_Op_A_txt(s_8_0, s_8_1, s_8_2, s_8_3)
        let s_8_4: () = execute_aarch32_instrs_VMOV_d_Op_A_txt(
            state,
            tracer,
            s_8_0,
            s_8_1,
            s_8_2,
            s_8_3,
        );
        // N s_8_5: return
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
        // D s_10_0: read-var t:i64
        let s_10_0: i64 = fn_state.t;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var t2:i64
        let s_10_2: i64 = fn_state.t2;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: write-var gs#312820 <= s_10_4
        fn_state.gs_312820 = s_10_4;
        // N s_10_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#312819 <= s_12_0
        fn_state.gs_312819 = s_12_0;
        // N s_12_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
