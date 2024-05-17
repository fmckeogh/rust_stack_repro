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
use execute_aarch32_instrs_VNEG_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_VNEG_T1enc_A_txt<T: Tracer>(
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
        ga_357401: i64,
        esize: i64,
        gs_314415: bool,
        gs_314414: bool,
        gs_314413: bool,
        d: i64,
        gs_314420: bool,
        gs_314421: bool,
        elements: i64,
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
        // N s_2_5: branch s_2_4 b28 b3
        if s_2_4 {
            return block_28(state, tracer, fn_state);
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
        // N s_3_5: branch s_3_4 b27 b4
        if s_3_4 {
            return block_27(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#314413 <= s_4_0
        fn_state.gs_314413 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#314413:u8
        let s_5_0: bool = fn_state.gs_314413;
        // N s_5_1: branch s_5_0 b26 b6
        if s_5_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var F:u8
        let s_6_0: bool = fn_state.F;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b25 b7
        if s_6_4 {
            return block_25(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#314414 <= s_7_0
        fn_state.gs_314414 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#314414:u8
        let s_8_0: bool = fn_state.gs_314414;
        // N s_8_1: branch s_8_0 b24 b9
        if s_8_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#314415 <= s_9_0
        fn_state.gs_314415 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#314415:u8
        let s_10_0: bool = fn_state.gs_314415;
        // N s_10_1: branch s_10_0 b23 b11
        if s_10_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var Q:u8
        let s_11_0: bool = fn_state.Q;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b19 b12
        if s_11_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#314421 <= s_12_0
        fn_state.gs_314421 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#314421:u8
        let s_13_0: bool = fn_state.gs_314421;
        // N s_13_1: branch s_13_0 b18 b14
        if s_13_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var F:u8
        let s_14_0: bool = fn_state.F;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: write-var floating_point <= s_14_4
        fn_state.floating_point = s_14_4;
        // D s_14_6: read-var size:u8
        let s_14_6: u8 = fn_state.size;
        // D s_14_7: cast zx s_14_6 -> bv
        let s_14_7: Bits = Bits::new(s_14_6 as u128, 2u16);
        // D s_14_8: cast zx s_14_7 -> i
        let s_14_8: i128 = (s_14_7.value() as i128);
        // D s_14_9: cast reint s_14_8 -> i64
        let s_14_9: i64 = (s_14_8 as i64);
        // C s_14_10: const #8s : i64
        let s_14_10: i64 = 8;
        // D s_14_11: lsl s_14_10 s_14_9
        let s_14_11: i64 = s_14_10 << s_14_9;
        // D s_14_12: write-var esize <= s_14_11
        fn_state.esize = s_14_11;
        // C s_14_13: const #64s : i
        let s_14_13: i128 = 64;
        // D s_14_14: read-var esize:i64
        let s_14_14: i64 = fn_state.esize;
        // D s_14_15: cast zx s_14_14 -> i
        let s_14_15: i128 = (i128::try_from(s_14_14).unwrap());
        // D s_14_16: div s_14_13 s_14_15
        let s_14_16: i128 = ((s_14_13) / (s_14_15));
        // D s_14_17: cast reint s_14_16 -> i64
        let s_14_17: i64 = (s_14_16 as i64);
        // D s_14_18: write-var elements <= s_14_17
        fn_state.elements = s_14_17;
        // D s_14_19: read-var D:u8
        let s_14_19: bool = fn_state.D;
        // D s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 1u16);
        // D s_14_21: read-var Vd:u8
        let s_14_21: u8 = fn_state.Vd;
        // D s_14_22: cast zx s_14_21 -> bv
        let s_14_22: Bits = Bits::new(s_14_21 as u128, 4u16);
        // D s_14_23: cast reint s_14_20 -> u128
        let s_14_23: u128 = (s_14_20.value() as u128);
        // D s_14_24: size-of s_14_20
        let s_14_24: u16 = s_14_20.length();
        // D s_14_25: cast reint s_14_22 -> u128
        let s_14_25: u128 = (s_14_22.value() as u128);
        // D s_14_26: size-of s_14_22
        let s_14_26: u16 = s_14_22.length();
        // D s_14_27: lsl s_14_23 s_14_26
        let s_14_27: u128 = s_14_23 << s_14_26;
        // D s_14_28: or s_14_27 s_14_25
        let s_14_28: u128 = ((s_14_27) | (s_14_25));
        // D s_14_29: add s_14_24 s_14_26
        let s_14_29: u16 = (s_14_24 + s_14_26);
        // D s_14_30: create-bits s_14_28 s_14_29
        let s_14_30: Bits = Bits::new(s_14_28, s_14_29);
        // D s_14_31: cast reint s_14_30 -> u8
        let s_14_31: u8 = (s_14_30.value() as u8);
        // D s_14_32: cast zx s_14_31 -> bv
        let s_14_32: Bits = Bits::new(s_14_31 as u128, 5u16);
        // D s_14_33: cast zx s_14_32 -> i
        let s_14_33: i128 = (s_14_32.value() as i128);
        // D s_14_34: cast reint s_14_33 -> i64
        let s_14_34: i64 = (s_14_33 as i64);
        // D s_14_35: write-var d <= s_14_34
        fn_state.d = s_14_34;
        // D s_14_36: read-var M:u8
        let s_14_36: bool = fn_state.M;
        // D s_14_37: cast zx s_14_36 -> bv
        let s_14_37: Bits = Bits::new(s_14_36 as u128, 1u16);
        // D s_14_38: read-var Vm:u8
        let s_14_38: u8 = fn_state.Vm;
        // D s_14_39: cast zx s_14_38 -> bv
        let s_14_39: Bits = Bits::new(s_14_38 as u128, 4u16);
        // D s_14_40: cast reint s_14_37 -> u128
        let s_14_40: u128 = (s_14_37.value() as u128);
        // D s_14_41: size-of s_14_37
        let s_14_41: u16 = s_14_37.length();
        // D s_14_42: cast reint s_14_39 -> u128
        let s_14_42: u128 = (s_14_39.value() as u128);
        // D s_14_43: size-of s_14_39
        let s_14_43: u16 = s_14_39.length();
        // D s_14_44: lsl s_14_40 s_14_43
        let s_14_44: u128 = s_14_40 << s_14_43;
        // D s_14_45: or s_14_44 s_14_42
        let s_14_45: u128 = ((s_14_44) | (s_14_42));
        // D s_14_46: add s_14_41 s_14_43
        let s_14_46: u16 = (s_14_41 + s_14_43);
        // D s_14_47: create-bits s_14_45 s_14_46
        let s_14_47: Bits = Bits::new(s_14_45, s_14_46);
        // D s_14_48: cast reint s_14_47 -> u8
        let s_14_48: u8 = (s_14_47.value() as u8);
        // D s_14_49: cast zx s_14_48 -> bv
        let s_14_49: Bits = Bits::new(s_14_48 as u128, 5u16);
        // D s_14_50: cast zx s_14_49 -> i
        let s_14_50: i128 = (s_14_49.value() as i128);
        // D s_14_51: cast reint s_14_50 -> i64
        let s_14_51: i64 = (s_14_50 as i64);
        // D s_14_52: write-var m <= s_14_51
        fn_state.m = s_14_51;
        // D s_14_53: read-var Q:u8
        let s_14_53: bool = fn_state.Q;
        // D s_14_54: cast zx s_14_53 -> bv
        let s_14_54: Bits = Bits::new(s_14_53 as u128, 1u16);
        // C s_14_55: const #0u : u8
        let s_14_55: bool = false;
        // C s_14_56: cast zx s_14_55 -> bv
        let s_14_56: Bits = Bits::new(s_14_55 as u128, 1u16);
        // D s_14_57: cmp-eq s_14_54 s_14_56
        let s_14_57: bool = ((s_14_54) == (s_14_56));
        // N s_14_58: branch s_14_57 b17 b15
        if s_14_57 {
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
        // C s_15_0: const #2s : i64
        let s_15_0: i64 = 2;
        // D s_15_1: write-var ga#357401 <= s_15_0
        fn_state.ga_357401 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var ga#357401:i64
        let s_16_0: i64 = fn_state.ga_357401;
        // D s_16_1: read-var esize:i64
        let s_16_1: i64 = fn_state.esize;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // D s_16_4: read-var elements:i64
        let s_16_4: i64 = fn_state.elements;
        // D s_16_5: cast zx s_16_4 -> i
        let s_16_5: i128 = (i128::try_from(s_16_4).unwrap());
        // D s_16_6: cast zx s_16_0 -> i
        let s_16_6: i128 = (i128::try_from(s_16_0).unwrap());
        // C s_16_7: const #1u : u8
        let s_16_7: bool = true;
        // D s_16_8: read-var d:i64
        let s_16_8: i64 = fn_state.d;
        // D s_16_9: read-var floating_point:u8
        let s_16_9: bool = fn_state.floating_point;
        // D s_16_10: read-var m:i64
        let s_16_10: i64 = fn_state.m;
        // D s_16_11: call execute_aarch32_instrs_VNEG_Op_A_txt(s_16_7, s_16_8, s_16_5, s_16_3, s_16_9, s_16_10, s_16_6)
        let s_16_11: () = execute_aarch32_instrs_VNEG_Op_A_txt(
            state,
            tracer,
            s_16_7,
            s_16_8,
            s_16_5,
            s_16_3,
            s_16_9,
            s_16_10,
            s_16_6,
        );
        // N s_16_12: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1s : i64
        let s_17_0: i64 = 1;
        // D s_17_1: write-var ga#357401 <= s_17_0
        fn_state.ga_357401 = s_17_0;
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
        // C s_19_0: const #0s : i
        let s_19_0: i128 = 0;
        // D s_19_1: read-var Vd:u8
        let s_19_1: u8 = fn_state.Vd;
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 4u16);
        // C s_19_3: const #1u : u64
        let s_19_3: u64 = 1;
        // D s_19_4: bit-extract s_19_2 s_19_0 s_19_3
        let s_19_4: Bits = (Bits::new(
            ((s_19_2) >> (s_19_0)).value(),
            u16::try_from(s_19_3).unwrap(),
        ));
        // D s_19_5: cast reint s_19_4 -> u8
        let s_19_5: bool = ((s_19_4.value()) != 0);
        // C s_19_6: const #0s : i
        let s_19_6: i128 = 0;
        // C s_19_7: const #0u : u64
        let s_19_7: u64 = 0;
        // D s_19_8: cast zx s_19_5 -> u64
        let s_19_8: u64 = (s_19_5 as u64);
        // C s_19_9: const #1u : u64
        let s_19_9: u64 = 1;
        // D s_19_10: and s_19_8 s_19_9
        let s_19_10: u64 = ((s_19_8) & (s_19_9));
        // D s_19_11: cmp-eq s_19_10 s_19_9
        let s_19_11: bool = ((s_19_10) == (s_19_9));
        // D s_19_12: lsl s_19_8 s_19_6
        let s_19_12: u64 = s_19_8 << s_19_6;
        // D s_19_13: or s_19_7 s_19_12
        let s_19_13: u64 = ((s_19_7) | (s_19_12));
        // D s_19_14: cmpl s_19_12
        let s_19_14: u64 = !s_19_12;
        // D s_19_15: and s_19_7 s_19_14
        let s_19_15: u64 = ((s_19_7) & (s_19_14));
        // D s_19_16: select s_19_11 s_19_13 s_19_15
        let s_19_16: u64 = if s_19_11 { s_19_13 } else { s_19_15 };
        // D s_19_17: cast trunc s_19_16 -> u8
        let s_19_17: bool = ((s_19_16) != 0);
        // D s_19_18: cast zx s_19_17 -> bv
        let s_19_18: Bits = Bits::new(s_19_17 as u128, 1u16);
        // C s_19_19: const #1u : u8
        let s_19_19: bool = true;
        // C s_19_20: cast zx s_19_19 -> bv
        let s_19_20: Bits = Bits::new(s_19_19 as u128, 1u16);
        // D s_19_21: cmp-eq s_19_18 s_19_20
        let s_19_21: bool = ((s_19_18) == (s_19_20));
        // N s_19_22: branch s_19_21 b22 b20
        if s_19_21 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0s : i
        let s_20_0: i128 = 0;
        // D s_20_1: read-var Vm:u8
        let s_20_1: u8 = fn_state.Vm;
        // D s_20_2: cast zx s_20_1 -> bv
        let s_20_2: Bits = Bits::new(s_20_1 as u128, 4u16);
        // C s_20_3: const #1u : u64
        let s_20_3: u64 = 1;
        // D s_20_4: bit-extract s_20_2 s_20_0 s_20_3
        let s_20_4: Bits = (Bits::new(
            ((s_20_2) >> (s_20_0)).value(),
            u16::try_from(s_20_3).unwrap(),
        ));
        // D s_20_5: cast reint s_20_4 -> u8
        let s_20_5: bool = ((s_20_4.value()) != 0);
        // C s_20_6: const #0s : i
        let s_20_6: i128 = 0;
        // C s_20_7: const #0u : u64
        let s_20_7: u64 = 0;
        // D s_20_8: cast zx s_20_5 -> u64
        let s_20_8: u64 = (s_20_5 as u64);
        // C s_20_9: const #1u : u64
        let s_20_9: u64 = 1;
        // D s_20_10: and s_20_8 s_20_9
        let s_20_10: u64 = ((s_20_8) & (s_20_9));
        // D s_20_11: cmp-eq s_20_10 s_20_9
        let s_20_11: bool = ((s_20_10) == (s_20_9));
        // D s_20_12: lsl s_20_8 s_20_6
        let s_20_12: u64 = s_20_8 << s_20_6;
        // D s_20_13: or s_20_7 s_20_12
        let s_20_13: u64 = ((s_20_7) | (s_20_12));
        // D s_20_14: cmpl s_20_12
        let s_20_14: u64 = !s_20_12;
        // D s_20_15: and s_20_7 s_20_14
        let s_20_15: u64 = ((s_20_7) & (s_20_14));
        // D s_20_16: select s_20_11 s_20_13 s_20_15
        let s_20_16: u64 = if s_20_11 { s_20_13 } else { s_20_15 };
        // D s_20_17: cast trunc s_20_16 -> u8
        let s_20_17: bool = ((s_20_16) != 0);
        // D s_20_18: cast zx s_20_17 -> bv
        let s_20_18: Bits = Bits::new(s_20_17 as u128, 1u16);
        // C s_20_19: const #1u : u8
        let s_20_19: bool = true;
        // C s_20_20: cast zx s_20_19 -> bv
        let s_20_20: Bits = Bits::new(s_20_19 as u128, 1u16);
        // D s_20_21: cmp-eq s_20_18 s_20_20
        let s_20_21: bool = ((s_20_18) == (s_20_20));
        // D s_20_22: write-var gs#314420 <= s_20_21
        fn_state.gs_314420 = s_20_21;
        // N s_20_23: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#314420:u8
        let s_21_0: bool = fn_state.gs_314420;
        // D s_21_1: write-var gs#314421 <= s_21_0
        fn_state.gs_314421 = s_21_0;
        // N s_21_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#314420 <= s_22_0
        fn_state.gs_314420 = s_22_0;
        // N s_22_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call InITBlock(s_24_0)
        let s_24_1: bool = InITBlock(state, tracer, s_24_0);
        // D s_24_2: write-var gs#314415 <= s_24_1
        fn_state.gs_314415 = s_24_1;
        // N s_24_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var size:u8
        let s_25_0: u8 = fn_state.size;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 2u16);
        // C s_25_2: const #1u : u8
        let s_25_2: u8 = 1;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 2u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#314414 <= s_25_4
        fn_state.gs_314414 = s_25_4;
        // N s_25_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var size:u8
        let s_27_0: u8 = fn_state.size;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 2u16);
        // C s_27_2: const #0u : u8
        let s_27_2: u8 = 0;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 2u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#314413 <= s_27_4
        fn_state.gs_314413 = s_27_4;
        // N s_27_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
}
