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
use execute_aarch32_instrs_VMOV_ss_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMOV_ss_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    op: bool,
    Rt2: u8,
    Rt: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_313498: bool,
        m: i64,
        t: i64,
        t2: i64,
        to_arm_registers: bool,
        gs_313496: bool,
        gs_313499: bool,
        cond: u8,
        op: bool,
        Rt2: u8,
        Rt: u8,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        cond,
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
        // D s_2_11: write-var to_arm_registers <= s_2_10
        fn_state.to_arm_registers = s_2_10;
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
        // D s_2_17: read-var Rt2:u8
        let s_2_17: u8 = fn_state.Rt2;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 4u16);
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (s_2_18.value() as i128);
        // D s_2_20: cast reint s_2_19 -> i64
        let s_2_20: i64 = (s_2_19 as i64);
        // D s_2_21: write-var t2 <= s_2_20
        fn_state.t2 = s_2_20;
        // D s_2_22: read-var Vm:u8
        let s_2_22: u8 = fn_state.Vm;
        // D s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 4u16);
        // D s_2_24: read-var M:u8
        let s_2_24: bool = fn_state.M;
        // D s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 1u16);
        // D s_2_26: cast reint s_2_23 -> u128
        let s_2_26: u128 = (s_2_23.value() as u128);
        // D s_2_27: size-of s_2_23
        let s_2_27: u16 = s_2_23.length();
        // D s_2_28: cast reint s_2_25 -> u128
        let s_2_28: u128 = (s_2_25.value() as u128);
        // D s_2_29: size-of s_2_25
        let s_2_29: u16 = s_2_25.length();
        // D s_2_30: lsl s_2_26 s_2_29
        let s_2_30: u128 = s_2_26 << s_2_29;
        // D s_2_31: or s_2_30 s_2_28
        let s_2_31: u128 = ((s_2_30) | (s_2_28));
        // D s_2_32: add s_2_27 s_2_29
        let s_2_32: u16 = (s_2_27 + s_2_29);
        // D s_2_33: create-bits s_2_31 s_2_32
        let s_2_33: Bits = Bits::new(s_2_31, s_2_32);
        // D s_2_34: cast reint s_2_33 -> u8
        let s_2_34: u8 = (s_2_33.value() as u8);
        // D s_2_35: cast zx s_2_34 -> bv
        let s_2_35: Bits = Bits::new(s_2_34 as u128, 5u16);
        // D s_2_36: cast zx s_2_35 -> i
        let s_2_36: i128 = (s_2_35.value() as i128);
        // D s_2_37: cast reint s_2_36 -> i64
        let s_2_37: i64 = (s_2_36 as i64);
        // D s_2_38: write-var m <= s_2_37
        fn_state.m = s_2_37;
        // C s_2_39: const #15s : i
        let s_2_39: i128 = 15;
        // D s_2_40: read-var t:i64
        let s_2_40: i64 = fn_state.t;
        // D s_2_41: cast zx s_2_40 -> i
        let s_2_41: i128 = (i128::try_from(s_2_40).unwrap());
        // D s_2_42: cmp-eq s_2_41 s_2_39
        let s_2_42: bool = ((s_2_41) == (s_2_39));
        // N s_2_43: branch s_2_42 b15 b3
        if s_2_42 {
            return block_15(state, tracer, fn_state);
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
        // D s_3_4: write-var gs#313496 <= s_3_3
        fn_state.gs_313496 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#313496:u8
        let s_4_0: bool = fn_state.gs_313496;
        // N s_4_1: branch s_4_0 b14 b5
        if s_4_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #31s : i
        let s_5_0: i128 = 31;
        // D s_5_1: read-var m:i64
        let s_5_1: i64 = fn_state.m;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // D s_5_4: write-var gs#313498 <= s_5_3
        fn_state.gs_313498 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#313498:u8
        let s_6_0: bool = fn_state.gs_313498;
        // N s_6_1: branch s_6_0 b13 b7
        if s_6_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var to_arm_registers:u8
        let s_7_0: bool = fn_state.to_arm_registers;
        // N s_7_1: branch s_7_0 b12 b8
        if s_7_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#313499 <= s_8_0
        fn_state.gs_313499 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#313499:u8
        let s_9_0: bool = fn_state.gs_313499;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var m:i64
        let s_10_0: i64 = fn_state.m;
        // D s_10_1: read-var t:i64
        let s_10_1: i64 = fn_state.t;
        // D s_10_2: read-var t2:i64
        let s_10_2: i64 = fn_state.t2;
        // D s_10_3: read-var to_arm_registers:u8
        let s_10_3: bool = fn_state.to_arm_registers;
        // D s_10_4: call execute_aarch32_instrs_VMOV_ss_Op_A_txt(s_10_0, s_10_1, s_10_2, s_10_3)
        let s_10_4: () = execute_aarch32_instrs_VMOV_ss_Op_A_txt(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
            s_10_3,
        );
        // N s_10_5: return
        return;
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
        // D s_12_0: read-var t:i64
        let s_12_0: i64 = fn_state.t;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var t2:i64
        let s_12_2: i64 = fn_state.t2;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#313499 <= s_12_4
        fn_state.gs_313499 = s_12_4;
        // N s_12_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#313498 <= s_14_0
        fn_state.gs_313498 = s_14_0;
        // N s_14_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#313496 <= s_15_0
        fn_state.gs_313496 = s_15_0;
        // N s_15_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
