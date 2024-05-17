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
use execute_aarch32_instrs_VCLT_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VCLT_i_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    size: u8,
    Vd: u8,
    F: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        floating_point: bool,
        m: i64,
        ga_352020: i64,
        esize: i64,
        gs_307574: bool,
        d: i64,
        elements: i64,
        gs_307569: bool,
        gs_307575: bool,
        D: bool,
        size: u8,
        Vd: u8,
        F: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        size,
        Vd,
        F,
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
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b20 b3
        if s_2_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var F:u8
        let s_3_0: bool = fn_state.F;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b19 b4
        if s_3_4 {
            return block_19(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#307569 <= s_4_0
        fn_state.gs_307569 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#307569:u8
        let s_5_0: bool = fn_state.gs_307569;
        // N s_5_1: branch s_5_0 b18 b6
        if s_5_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var Q:u8
        let s_6_0: bool = fn_state.Q;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b14 b7
        if s_6_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#307575 <= s_7_0
        fn_state.gs_307575 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#307575:u8
        let s_8_0: bool = fn_state.gs_307575;
        // N s_8_1: branch s_8_0 b13 b9
        if s_8_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var F:u8
        let s_9_0: bool = fn_state.F;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // C s_9_2: const #1u : u8
        let s_9_2: bool = true;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: write-var floating_point <= s_9_4
        fn_state.floating_point = s_9_4;
        // D s_9_6: read-var size:u8
        let s_9_6: u8 = fn_state.size;
        // D s_9_7: cast zx s_9_6 -> bv
        let s_9_7: Bits = Bits::new(s_9_6 as u128, 2u16);
        // D s_9_8: cast zx s_9_7 -> i
        let s_9_8: i128 = (s_9_7.value() as i128);
        // D s_9_9: cast reint s_9_8 -> i64
        let s_9_9: i64 = (s_9_8 as i64);
        // C s_9_10: const #8s : i64
        let s_9_10: i64 = 8;
        // D s_9_11: lsl s_9_10 s_9_9
        let s_9_11: i64 = s_9_10 << s_9_9;
        // D s_9_12: write-var esize <= s_9_11
        fn_state.esize = s_9_11;
        // C s_9_13: const #64s : i
        let s_9_13: i128 = 64;
        // D s_9_14: read-var esize:i64
        let s_9_14: i64 = fn_state.esize;
        // D s_9_15: cast zx s_9_14 -> i
        let s_9_15: i128 = (i128::try_from(s_9_14).unwrap());
        // D s_9_16: div s_9_13 s_9_15
        let s_9_16: i128 = ((s_9_13) / (s_9_15));
        // D s_9_17: cast reint s_9_16 -> i64
        let s_9_17: i64 = (s_9_16 as i64);
        // D s_9_18: write-var elements <= s_9_17
        fn_state.elements = s_9_17;
        // D s_9_19: read-var D:u8
        let s_9_19: bool = fn_state.D;
        // D s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 1u16);
        // D s_9_21: read-var Vd:u8
        let s_9_21: u8 = fn_state.Vd;
        // D s_9_22: cast zx s_9_21 -> bv
        let s_9_22: Bits = Bits::new(s_9_21 as u128, 4u16);
        // D s_9_23: cast reint s_9_20 -> u128
        let s_9_23: u128 = (s_9_20.value() as u128);
        // D s_9_24: size-of s_9_20
        let s_9_24: u16 = s_9_20.length();
        // D s_9_25: cast reint s_9_22 -> u128
        let s_9_25: u128 = (s_9_22.value() as u128);
        // D s_9_26: size-of s_9_22
        let s_9_26: u16 = s_9_22.length();
        // D s_9_27: lsl s_9_23 s_9_26
        let s_9_27: u128 = s_9_23 << s_9_26;
        // D s_9_28: or s_9_27 s_9_25
        let s_9_28: u128 = ((s_9_27) | (s_9_25));
        // D s_9_29: add s_9_24 s_9_26
        let s_9_29: u16 = (s_9_24 + s_9_26);
        // D s_9_30: create-bits s_9_28 s_9_29
        let s_9_30: Bits = Bits::new(s_9_28, s_9_29);
        // D s_9_31: cast reint s_9_30 -> u8
        let s_9_31: u8 = (s_9_30.value() as u8);
        // D s_9_32: cast zx s_9_31 -> bv
        let s_9_32: Bits = Bits::new(s_9_31 as u128, 5u16);
        // D s_9_33: cast zx s_9_32 -> i
        let s_9_33: i128 = (s_9_32.value() as i128);
        // D s_9_34: cast reint s_9_33 -> i64
        let s_9_34: i64 = (s_9_33 as i64);
        // D s_9_35: write-var d <= s_9_34
        fn_state.d = s_9_34;
        // D s_9_36: read-var M:u8
        let s_9_36: bool = fn_state.M;
        // D s_9_37: cast zx s_9_36 -> bv
        let s_9_37: Bits = Bits::new(s_9_36 as u128, 1u16);
        // D s_9_38: read-var Vm:u8
        let s_9_38: u8 = fn_state.Vm;
        // D s_9_39: cast zx s_9_38 -> bv
        let s_9_39: Bits = Bits::new(s_9_38 as u128, 4u16);
        // D s_9_40: cast reint s_9_37 -> u128
        let s_9_40: u128 = (s_9_37.value() as u128);
        // D s_9_41: size-of s_9_37
        let s_9_41: u16 = s_9_37.length();
        // D s_9_42: cast reint s_9_39 -> u128
        let s_9_42: u128 = (s_9_39.value() as u128);
        // D s_9_43: size-of s_9_39
        let s_9_43: u16 = s_9_39.length();
        // D s_9_44: lsl s_9_40 s_9_43
        let s_9_44: u128 = s_9_40 << s_9_43;
        // D s_9_45: or s_9_44 s_9_42
        let s_9_45: u128 = ((s_9_44) | (s_9_42));
        // D s_9_46: add s_9_41 s_9_43
        let s_9_46: u16 = (s_9_41 + s_9_43);
        // D s_9_47: create-bits s_9_45 s_9_46
        let s_9_47: Bits = Bits::new(s_9_45, s_9_46);
        // D s_9_48: cast reint s_9_47 -> u8
        let s_9_48: u8 = (s_9_47.value() as u8);
        // D s_9_49: cast zx s_9_48 -> bv
        let s_9_49: Bits = Bits::new(s_9_48 as u128, 5u16);
        // D s_9_50: cast zx s_9_49 -> i
        let s_9_50: i128 = (s_9_49.value() as i128);
        // D s_9_51: cast reint s_9_50 -> i64
        let s_9_51: i64 = (s_9_50 as i64);
        // D s_9_52: write-var m <= s_9_51
        fn_state.m = s_9_51;
        // D s_9_53: read-var Q:u8
        let s_9_53: bool = fn_state.Q;
        // D s_9_54: cast zx s_9_53 -> bv
        let s_9_54: Bits = Bits::new(s_9_53 as u128, 1u16);
        // C s_9_55: const #0u : u8
        let s_9_55: bool = false;
        // C s_9_56: cast zx s_9_55 -> bv
        let s_9_56: Bits = Bits::new(s_9_55 as u128, 1u16);
        // D s_9_57: cmp-eq s_9_54 s_9_56
        let s_9_57: bool = ((s_9_54) == (s_9_56));
        // N s_9_58: branch s_9_57 b12 b10
        if s_9_57 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #2s : i64
        let s_10_0: i64 = 2;
        // D s_10_1: write-var ga#352020 <= s_10_0
        fn_state.ga_352020 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#352020:i64
        let s_11_0: i64 = fn_state.ga_352020;
        // D s_11_1: read-var esize:i64
        let s_11_1: i64 = fn_state.esize;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // D s_11_4: read-var elements:i64
        let s_11_4: i64 = fn_state.elements;
        // D s_11_5: cast zx s_11_4 -> i
        let s_11_5: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_6: read-var d:i64
        let s_11_6: i64 = fn_state.d;
        // D s_11_7: read-var floating_point:u8
        let s_11_7: bool = fn_state.floating_point;
        // D s_11_8: read-var m:i64
        let s_11_8: i64 = fn_state.m;
        // D s_11_9: call execute_aarch32_instrs_VCLT_i_Op_A_txt(s_11_6, s_11_5, s_11_3, s_11_7, s_11_8, s_11_0)
        let s_11_9: () = execute_aarch32_instrs_VCLT_i_Op_A_txt(
            state,
            tracer,
            s_11_6,
            s_11_5,
            s_11_3,
            s_11_7,
            s_11_8,
            s_11_0,
        );
        // N s_11_10: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1s : i64
        let s_12_0: i64 = 1;
        // D s_12_1: write-var ga#352020 <= s_12_0
        fn_state.ga_352020 = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
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
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: read-var Vd:u8
        let s_14_1: u8 = fn_state.Vd;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 4u16);
        // C s_14_3: const #1u : u64
        let s_14_3: u64 = 1;
        // D s_14_4: bit-extract s_14_2 s_14_0 s_14_3
        let s_14_4: Bits = (Bits::new(
            ((s_14_2) >> (s_14_0)).value(),
            u16::try_from(s_14_3).unwrap(),
        ));
        // D s_14_5: cast reint s_14_4 -> u8
        let s_14_5: bool = ((s_14_4.value()) != 0);
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // C s_14_7: const #0u : u64
        let s_14_7: u64 = 0;
        // D s_14_8: cast zx s_14_5 -> u64
        let s_14_8: u64 = (s_14_5 as u64);
        // C s_14_9: const #1u : u64
        let s_14_9: u64 = 1;
        // D s_14_10: and s_14_8 s_14_9
        let s_14_10: u64 = ((s_14_8) & (s_14_9));
        // D s_14_11: cmp-eq s_14_10 s_14_9
        let s_14_11: bool = ((s_14_10) == (s_14_9));
        // D s_14_12: lsl s_14_8 s_14_6
        let s_14_12: u64 = s_14_8 << s_14_6;
        // D s_14_13: or s_14_7 s_14_12
        let s_14_13: u64 = ((s_14_7) | (s_14_12));
        // D s_14_14: cmpl s_14_12
        let s_14_14: u64 = !s_14_12;
        // D s_14_15: and s_14_7 s_14_14
        let s_14_15: u64 = ((s_14_7) & (s_14_14));
        // D s_14_16: select s_14_11 s_14_13 s_14_15
        let s_14_16: u64 = if s_14_11 { s_14_13 } else { s_14_15 };
        // D s_14_17: cast trunc s_14_16 -> u8
        let s_14_17: bool = ((s_14_16) != 0);
        // D s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 1u16);
        // C s_14_19: const #1u : u8
        let s_14_19: bool = true;
        // C s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 1u16);
        // D s_14_21: cmp-eq s_14_18 s_14_20
        let s_14_21: bool = ((s_14_18) == (s_14_20));
        // N s_14_22: branch s_14_21 b17 b15
        if s_14_21 {
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
        // D s_15_22: write-var gs#307574 <= s_15_21
        fn_state.gs_307574 = s_15_21;
        // N s_15_23: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#307574:u8
        let s_16_0: bool = fn_state.gs_307574;
        // D s_16_1: write-var gs#307575 <= s_16_0
        fn_state.gs_307575 = s_16_0;
        // N s_16_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#307574 <= s_17_0
        fn_state.gs_307574 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var size:u8
        let s_19_0: u8 = fn_state.size;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 2u16);
        // C s_19_2: const #0u : u8
        let s_19_2: u8 = 0;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 2u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var gs#307569 <= s_19_4
        fn_state.gs_307569 = s_19_4;
        // N s_19_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
    }
}
