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
use execute_aarch32_instrs_VEXT_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VEXT_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Vn: u8,
    Vd: u8,
    imm4: u8,
    N: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_308867: bool,
        gs_308870: bool,
        gs_308863: bool,
        gs_308866: bool,
        D: bool,
        Vn: u8,
        Vd: u8,
        imm4: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        Vn,
        Vd,
        imm4,
        N,
        Q,
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
        // D s_2_0: read-var Q:u8
        let s_2_0: bool = fn_state.Q;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b12 b3
        if s_2_4 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#308867 <= s_3_0
        fn_state.gs_308867 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#308867:u8
        let s_4_0: bool = fn_state.gs_308867;
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
        // D s_5_0: read-var Q:u8
        let s_5_0: bool = fn_state.Q;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b10 b6
        if s_5_4 {
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
        // D s_6_1: write-var gs#308870 <= s_6_0
        fn_state.gs_308870 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#308870:u8
        let s_7_0: bool = fn_state.gs_308870;
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
        // D s_8_0: read-var Q:u8
        let s_8_0: bool = fn_state.Q;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #1u : u8
        let s_8_2: bool = true;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: read-var imm4:u8
        let s_8_5: u8 = fn_state.imm4;
        // D s_8_6: cast zx s_8_5 -> bv
        let s_8_6: Bits = Bits::new(s_8_5 as u128, 4u16);
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (s_8_6.value() as i128);
        // D s_8_8: cast reint s_8_7 -> i64
        let s_8_8: i64 = (s_8_7 as i64);
        // C s_8_9: const #8s : i
        let s_8_9: i128 = 8;
        // D s_8_10: cast zx s_8_8 -> i
        let s_8_10: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_11: mul s_8_9 s_8_10
        let s_8_11: i128 = ((s_8_9) * (s_8_10));
        // D s_8_12: cast reint s_8_11 -> i64
        let s_8_12: i64 = (s_8_11 as i64);
        // D s_8_13: read-var D:u8
        let s_8_13: bool = fn_state.D;
        // D s_8_14: cast zx s_8_13 -> bv
        let s_8_14: Bits = Bits::new(s_8_13 as u128, 1u16);
        // D s_8_15: read-var Vd:u8
        let s_8_15: u8 = fn_state.Vd;
        // D s_8_16: cast zx s_8_15 -> bv
        let s_8_16: Bits = Bits::new(s_8_15 as u128, 4u16);
        // D s_8_17: cast reint s_8_14 -> u128
        let s_8_17: u128 = (s_8_14.value() as u128);
        // D s_8_18: size-of s_8_14
        let s_8_18: u16 = s_8_14.length();
        // D s_8_19: cast reint s_8_16 -> u128
        let s_8_19: u128 = (s_8_16.value() as u128);
        // D s_8_20: size-of s_8_16
        let s_8_20: u16 = s_8_16.length();
        // D s_8_21: lsl s_8_17 s_8_20
        let s_8_21: u128 = s_8_17 << s_8_20;
        // D s_8_22: or s_8_21 s_8_19
        let s_8_22: u128 = ((s_8_21) | (s_8_19));
        // D s_8_23: add s_8_18 s_8_20
        let s_8_23: u16 = (s_8_18 + s_8_20);
        // D s_8_24: create-bits s_8_22 s_8_23
        let s_8_24: Bits = Bits::new(s_8_22, s_8_23);
        // D s_8_25: cast reint s_8_24 -> u8
        let s_8_25: u8 = (s_8_24.value() as u8);
        // D s_8_26: cast zx s_8_25 -> bv
        let s_8_26: Bits = Bits::new(s_8_25 as u128, 5u16);
        // D s_8_27: cast zx s_8_26 -> i
        let s_8_27: i128 = (s_8_26.value() as i128);
        // D s_8_28: cast reint s_8_27 -> i64
        let s_8_28: i64 = (s_8_27 as i64);
        // D s_8_29: read-var N:u8
        let s_8_29: bool = fn_state.N;
        // D s_8_30: cast zx s_8_29 -> bv
        let s_8_30: Bits = Bits::new(s_8_29 as u128, 1u16);
        // D s_8_31: read-var Vn:u8
        let s_8_31: u8 = fn_state.Vn;
        // D s_8_32: cast zx s_8_31 -> bv
        let s_8_32: Bits = Bits::new(s_8_31 as u128, 4u16);
        // D s_8_33: cast reint s_8_30 -> u128
        let s_8_33: u128 = (s_8_30.value() as u128);
        // D s_8_34: size-of s_8_30
        let s_8_34: u16 = s_8_30.length();
        // D s_8_35: cast reint s_8_32 -> u128
        let s_8_35: u128 = (s_8_32.value() as u128);
        // D s_8_36: size-of s_8_32
        let s_8_36: u16 = s_8_32.length();
        // D s_8_37: lsl s_8_33 s_8_36
        let s_8_37: u128 = s_8_33 << s_8_36;
        // D s_8_38: or s_8_37 s_8_35
        let s_8_38: u128 = ((s_8_37) | (s_8_35));
        // D s_8_39: add s_8_34 s_8_36
        let s_8_39: u16 = (s_8_34 + s_8_36);
        // D s_8_40: create-bits s_8_38 s_8_39
        let s_8_40: Bits = Bits::new(s_8_38, s_8_39);
        // D s_8_41: cast reint s_8_40 -> u8
        let s_8_41: u8 = (s_8_40.value() as u8);
        // D s_8_42: cast zx s_8_41 -> bv
        let s_8_42: Bits = Bits::new(s_8_41 as u128, 5u16);
        // D s_8_43: cast zx s_8_42 -> i
        let s_8_43: i128 = (s_8_42.value() as i128);
        // D s_8_44: cast reint s_8_43 -> i64
        let s_8_44: i64 = (s_8_43 as i64);
        // D s_8_45: read-var M:u8
        let s_8_45: bool = fn_state.M;
        // D s_8_46: cast zx s_8_45 -> bv
        let s_8_46: Bits = Bits::new(s_8_45 as u128, 1u16);
        // D s_8_47: read-var Vm:u8
        let s_8_47: u8 = fn_state.Vm;
        // D s_8_48: cast zx s_8_47 -> bv
        let s_8_48: Bits = Bits::new(s_8_47 as u128, 4u16);
        // D s_8_49: cast reint s_8_46 -> u128
        let s_8_49: u128 = (s_8_46.value() as u128);
        // D s_8_50: size-of s_8_46
        let s_8_50: u16 = s_8_46.length();
        // D s_8_51: cast reint s_8_48 -> u128
        let s_8_51: u128 = (s_8_48.value() as u128);
        // D s_8_52: size-of s_8_48
        let s_8_52: u16 = s_8_48.length();
        // D s_8_53: lsl s_8_49 s_8_52
        let s_8_53: u128 = s_8_49 << s_8_52;
        // D s_8_54: or s_8_53 s_8_51
        let s_8_54: u128 = ((s_8_53) | (s_8_51));
        // D s_8_55: add s_8_50 s_8_52
        let s_8_55: u16 = (s_8_50 + s_8_52);
        // D s_8_56: create-bits s_8_54 s_8_55
        let s_8_56: Bits = Bits::new(s_8_54, s_8_55);
        // D s_8_57: cast reint s_8_56 -> u8
        let s_8_57: u8 = (s_8_56.value() as u8);
        // D s_8_58: cast zx s_8_57 -> bv
        let s_8_58: Bits = Bits::new(s_8_57 as u128, 5u16);
        // D s_8_59: cast zx s_8_58 -> i
        let s_8_59: i128 = (s_8_58.value() as i128);
        // D s_8_60: cast reint s_8_59 -> i64
        let s_8_60: i64 = (s_8_59 as i64);
        // D s_8_61: call execute_aarch32_instrs_VEXT_Op_A_txt(s_8_28, s_8_60, s_8_44, s_8_12, s_8_4)
        let s_8_61: () = execute_aarch32_instrs_VEXT_Op_A_txt(
            state,
            tracer,
            s_8_28,
            s_8_60,
            s_8_44,
            s_8_12,
            s_8_4,
        );
        // N s_8_62: return
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
        // C s_10_0: const #3s : i
        let s_10_0: i128 = 3;
        // D s_10_1: read-var imm4:u8
        let s_10_1: u8 = fn_state.imm4;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 4u16);
        // C s_10_3: const #1u : u64
        let s_10_3: u64 = 1;
        // D s_10_4: bit-extract s_10_2 s_10_0 s_10_3
        let s_10_4: Bits = (Bits::new(
            ((s_10_2) >> (s_10_0)).value(),
            u16::try_from(s_10_3).unwrap(),
        ));
        // D s_10_5: cast reint s_10_4 -> u8
        let s_10_5: bool = ((s_10_4.value()) != 0);
        // C s_10_6: const #0s : i
        let s_10_6: i128 = 0;
        // C s_10_7: const #0u : u64
        let s_10_7: u64 = 0;
        // D s_10_8: cast zx s_10_5 -> u64
        let s_10_8: u64 = (s_10_5 as u64);
        // C s_10_9: const #1u : u64
        let s_10_9: u64 = 1;
        // D s_10_10: and s_10_8 s_10_9
        let s_10_10: u64 = ((s_10_8) & (s_10_9));
        // D s_10_11: cmp-eq s_10_10 s_10_9
        let s_10_11: bool = ((s_10_10) == (s_10_9));
        // D s_10_12: lsl s_10_8 s_10_6
        let s_10_12: u64 = s_10_8 << s_10_6;
        // D s_10_13: or s_10_7 s_10_12
        let s_10_13: u64 = ((s_10_7) | (s_10_12));
        // D s_10_14: cmpl s_10_12
        let s_10_14: u64 = !s_10_12;
        // D s_10_15: and s_10_7 s_10_14
        let s_10_15: u64 = ((s_10_7) & (s_10_14));
        // D s_10_16: select s_10_11 s_10_13 s_10_15
        let s_10_16: u64 = if s_10_11 { s_10_13 } else { s_10_15 };
        // D s_10_17: cast trunc s_10_16 -> u8
        let s_10_17: bool = ((s_10_16) != 0);
        // D s_10_18: cast zx s_10_17 -> bv
        let s_10_18: Bits = Bits::new(s_10_17 as u128, 1u16);
        // C s_10_19: const #1u : u8
        let s_10_19: bool = true;
        // C s_10_20: cast zx s_10_19 -> bv
        let s_10_20: Bits = Bits::new(s_10_19 as u128, 1u16);
        // D s_10_21: cmp-eq s_10_18 s_10_20
        let s_10_21: bool = ((s_10_18) == (s_10_20));
        // D s_10_22: write-var gs#308870 <= s_10_21
        fn_state.gs_308870 = s_10_21;
        // N s_10_23: jump b7
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
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var Vd:u8
        let s_12_1: u8 = fn_state.Vd;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 4u16);
        // C s_12_3: const #1u : u64
        let s_12_3: u64 = 1;
        // D s_12_4: bit-extract s_12_2 s_12_0 s_12_3
        let s_12_4: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_3).unwrap(),
        ));
        // D s_12_5: cast reint s_12_4 -> u8
        let s_12_5: bool = ((s_12_4.value()) != 0);
        // C s_12_6: const #0s : i
        let s_12_6: i128 = 0;
        // C s_12_7: const #0u : u64
        let s_12_7: u64 = 0;
        // D s_12_8: cast zx s_12_5 -> u64
        let s_12_8: u64 = (s_12_5 as u64);
        // C s_12_9: const #1u : u64
        let s_12_9: u64 = 1;
        // D s_12_10: and s_12_8 s_12_9
        let s_12_10: u64 = ((s_12_8) & (s_12_9));
        // D s_12_11: cmp-eq s_12_10 s_12_9
        let s_12_11: bool = ((s_12_10) == (s_12_9));
        // D s_12_12: lsl s_12_8 s_12_6
        let s_12_12: u64 = s_12_8 << s_12_6;
        // D s_12_13: or s_12_7 s_12_12
        let s_12_13: u64 = ((s_12_7) | (s_12_12));
        // D s_12_14: cmpl s_12_12
        let s_12_14: u64 = !s_12_12;
        // D s_12_15: and s_12_7 s_12_14
        let s_12_15: u64 = ((s_12_7) & (s_12_14));
        // D s_12_16: select s_12_11 s_12_13 s_12_15
        let s_12_16: u64 = if s_12_11 { s_12_13 } else { s_12_15 };
        // D s_12_17: cast trunc s_12_16 -> u8
        let s_12_17: bool = ((s_12_16) != 0);
        // D s_12_18: cast zx s_12_17 -> bv
        let s_12_18: Bits = Bits::new(s_12_17 as u128, 1u16);
        // C s_12_19: const #1u : u8
        let s_12_19: bool = true;
        // C s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 1u16);
        // D s_12_21: cmp-eq s_12_18 s_12_20
        let s_12_21: bool = ((s_12_18) == (s_12_20));
        // N s_12_22: branch s_12_21 b18 b13
        if s_12_21 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var Vn:u8
        let s_13_1: u8 = fn_state.Vn;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 4u16);
        // C s_13_3: const #1u : u64
        let s_13_3: u64 = 1;
        // D s_13_4: bit-extract s_13_2 s_13_0 s_13_3
        let s_13_4: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_3).unwrap(),
        ));
        // D s_13_5: cast reint s_13_4 -> u8
        let s_13_5: bool = ((s_13_4.value()) != 0);
        // C s_13_6: const #0s : i
        let s_13_6: i128 = 0;
        // C s_13_7: const #0u : u64
        let s_13_7: u64 = 0;
        // D s_13_8: cast zx s_13_5 -> u64
        let s_13_8: u64 = (s_13_5 as u64);
        // C s_13_9: const #1u : u64
        let s_13_9: u64 = 1;
        // D s_13_10: and s_13_8 s_13_9
        let s_13_10: u64 = ((s_13_8) & (s_13_9));
        // D s_13_11: cmp-eq s_13_10 s_13_9
        let s_13_11: bool = ((s_13_10) == (s_13_9));
        // D s_13_12: lsl s_13_8 s_13_6
        let s_13_12: u64 = s_13_8 << s_13_6;
        // D s_13_13: or s_13_7 s_13_12
        let s_13_13: u64 = ((s_13_7) | (s_13_12));
        // D s_13_14: cmpl s_13_12
        let s_13_14: u64 = !s_13_12;
        // D s_13_15: and s_13_7 s_13_14
        let s_13_15: u64 = ((s_13_7) & (s_13_14));
        // D s_13_16: select s_13_11 s_13_13 s_13_15
        let s_13_16: u64 = if s_13_11 { s_13_13 } else { s_13_15 };
        // D s_13_17: cast trunc s_13_16 -> u8
        let s_13_17: bool = ((s_13_16) != 0);
        // D s_13_18: cast zx s_13_17 -> bv
        let s_13_18: Bits = Bits::new(s_13_17 as u128, 1u16);
        // C s_13_19: const #1u : u8
        let s_13_19: bool = true;
        // C s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 1u16);
        // D s_13_21: cmp-eq s_13_18 s_13_20
        let s_13_21: bool = ((s_13_18) == (s_13_20));
        // D s_13_22: write-var gs#308863 <= s_13_21
        fn_state.gs_308863 = s_13_21;
        // N s_13_23: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#308863:u8
        let s_14_0: bool = fn_state.gs_308863;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0s : i
        let s_15_0: i128 = 0;
        // D s_15_1: read-var Vm:u8
        let s_15_1: u8 = fn_state.Vm;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 4u16);
        // C s_15_3: const #1u : u64
        let s_15_3: u64 = 1;
        // D s_15_4: bit-extract s_15_2 s_15_0 s_15_3
        let s_15_4: Bits = (Bits::new(
            ((s_15_2) >> (s_15_0)).value(),
            u16::try_from(s_15_3).unwrap(),
        ));
        // D s_15_5: cast reint s_15_4 -> u8
        let s_15_5: bool = ((s_15_4.value()) != 0);
        // C s_15_6: const #0s : i
        let s_15_6: i128 = 0;
        // C s_15_7: const #0u : u64
        let s_15_7: u64 = 0;
        // D s_15_8: cast zx s_15_5 -> u64
        let s_15_8: u64 = (s_15_5 as u64);
        // C s_15_9: const #1u : u64
        let s_15_9: u64 = 1;
        // D s_15_10: and s_15_8 s_15_9
        let s_15_10: u64 = ((s_15_8) & (s_15_9));
        // D s_15_11: cmp-eq s_15_10 s_15_9
        let s_15_11: bool = ((s_15_10) == (s_15_9));
        // D s_15_12: lsl s_15_8 s_15_6
        let s_15_12: u64 = s_15_8 << s_15_6;
        // D s_15_13: or s_15_7 s_15_12
        let s_15_13: u64 = ((s_15_7) | (s_15_12));
        // D s_15_14: cmpl s_15_12
        let s_15_14: u64 = !s_15_12;
        // D s_15_15: and s_15_7 s_15_14
        let s_15_15: u64 = ((s_15_7) & (s_15_14));
        // D s_15_16: select s_15_11 s_15_13 s_15_15
        let s_15_16: u64 = if s_15_11 { s_15_13 } else { s_15_15 };
        // D s_15_17: cast trunc s_15_16 -> u8
        let s_15_17: bool = ((s_15_16) != 0);
        // D s_15_18: cast zx s_15_17 -> bv
        let s_15_18: Bits = Bits::new(s_15_17 as u128, 1u16);
        // C s_15_19: const #1u : u8
        let s_15_19: bool = true;
        // C s_15_20: cast zx s_15_19 -> bv
        let s_15_20: Bits = Bits::new(s_15_19 as u128, 1u16);
        // D s_15_21: cmp-eq s_15_18 s_15_20
        let s_15_21: bool = ((s_15_18) == (s_15_20));
        // D s_15_22: write-var gs#308866 <= s_15_21
        fn_state.gs_308866 = s_15_21;
        // N s_15_23: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#308866:u8
        let s_16_0: bool = fn_state.gs_308866;
        // D s_16_1: write-var gs#308867 <= s_16_0
        fn_state.gs_308867 = s_16_0;
        // N s_16_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#308866 <= s_17_0
        fn_state.gs_308866 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#308863 <= s_18_0
        fn_state.gs_308863 = s_18_0;
        // N s_18_2: jump b14
        return block_14(state, tracer, fn_state);
    }
}
