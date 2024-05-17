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
use u__id::*;
use ConditionPassed::*;
use execute_aarch32_instrs_VST2_m_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VST2_m_A2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Rn: u8,
    Vd: u8,
    itype: u8,
    size: u8,
    align: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ebytes: i64,
        n: i64,
        gs_321084: bool,
        ga_362101: i128,
        gs_321119: bool,
        gs_321115: bool,
        gs_321106: bool,
        gs_321117: bool,
        gs_321110: bool,
        gs_321108: bool,
        d2: i64,
        gs_321101: bool,
        gs_321114: bool,
        gs_321099: bool,
        gs_321087: bool,
        gs_321120: bool,
        d: i64,
        gs_321090: bool,
        elements: i64,
        register_index: bool,
        gs_321118: bool,
        alignment: i128,
        gs_321097: bool,
        gs_321113: bool,
        wback: bool,
        gs_321112: bool,
        gs_321116: bool,
        D: bool,
        Rn: u8,
        Vd: u8,
        itype: u8,
        size: u8,
        align: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        D,
        Rn,
        Vd,
        itype,
        size,
        align,
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
        // N s_2_5: branch s_2_4 b63 b3
        if s_2_4 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var align:u8
        let s_3_0: u8 = fn_state.align;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #0u : u8
        let s_3_2: u8 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b62 b4
        if s_3_4 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var align:u8
        let s_4_0: u8 = fn_state.align;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #4s : i
        let s_4_4: i128 = 4;
        // D s_4_5: cast zx s_4_3 -> i
        let s_4_5: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_6: lsl s_4_4 s_4_5
        let s_4_6: i128 = s_4_4 << s_4_5;
        // D s_4_7: write-var ga#362101 <= s_4_6
        fn_state.ga_362101 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#362101:i
        let s_5_0: i128 = fn_state.ga_362101;
        // D s_5_1: write-var alignment <= s_5_0
        fn_state.alignment = s_5_0;
        // D s_5_2: read-var size:u8
        let s_5_2: u8 = fn_state.size;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (s_5_3.value() as i128);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // C s_5_6: const #1s : i64
        let s_5_6: i64 = 1;
        // D s_5_7: lsl s_5_6 s_5_5
        let s_5_7: i64 = s_5_6 << s_5_5;
        // D s_5_8: write-var ebytes <= s_5_7
        fn_state.ebytes = s_5_7;
        // C s_5_9: const #8s : i
        let s_5_9: i128 = 8;
        // D s_5_10: read-var ebytes:i64
        let s_5_10: i64 = fn_state.ebytes;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: div s_5_9 s_5_11
        let s_5_12: i128 = ((s_5_9) / (s_5_11));
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: write-var elements <= s_5_13
        fn_state.elements = s_5_13;
        // D s_5_15: read-var D:u8
        let s_5_15: bool = fn_state.D;
        // D s_5_16: cast zx s_5_15 -> bv
        let s_5_16: Bits = Bits::new(s_5_15 as u128, 1u16);
        // D s_5_17: read-var Vd:u8
        let s_5_17: u8 = fn_state.Vd;
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 4u16);
        // D s_5_19: cast reint s_5_16 -> u128
        let s_5_19: u128 = (s_5_16.value() as u128);
        // D s_5_20: size-of s_5_16
        let s_5_20: u16 = s_5_16.length();
        // D s_5_21: cast reint s_5_18 -> u128
        let s_5_21: u128 = (s_5_18.value() as u128);
        // D s_5_22: size-of s_5_18
        let s_5_22: u16 = s_5_18.length();
        // D s_5_23: lsl s_5_19 s_5_22
        let s_5_23: u128 = s_5_19 << s_5_22;
        // D s_5_24: or s_5_23 s_5_21
        let s_5_24: u128 = ((s_5_23) | (s_5_21));
        // D s_5_25: add s_5_20 s_5_22
        let s_5_25: u16 = (s_5_20 + s_5_22);
        // D s_5_26: create-bits s_5_24 s_5_25
        let s_5_26: Bits = Bits::new(s_5_24, s_5_25);
        // D s_5_27: cast reint s_5_26 -> u8
        let s_5_27: u8 = (s_5_26.value() as u8);
        // D s_5_28: cast zx s_5_27 -> bv
        let s_5_28: Bits = Bits::new(s_5_27 as u128, 5u16);
        // D s_5_29: cast zx s_5_28 -> i
        let s_5_29: i128 = (s_5_28.value() as i128);
        // D s_5_30: cast reint s_5_29 -> i64
        let s_5_30: i64 = (s_5_29 as i64);
        // D s_5_31: write-var d <= s_5_30
        fn_state.d = s_5_30;
        // D s_5_32: read-var d:i64
        let s_5_32: i64 = fn_state.d;
        // D s_5_33: cast zx s_5_32 -> i
        let s_5_33: i128 = (i128::try_from(s_5_32).unwrap());
        // C s_5_34: const #2s : i64
        let s_5_34: i64 = 2;
        // C s_5_35: cast zx s_5_34 -> i
        let s_5_35: i128 = (i128::try_from(s_5_34).unwrap());
        // D s_5_36: add s_5_33 s_5_35
        let s_5_36: i128 = (s_5_33 + s_5_35);
        // D s_5_37: cast reint s_5_36 -> i64
        let s_5_37: i64 = (s_5_36 as i64);
        // D s_5_38: write-var d2 <= s_5_37
        fn_state.d2 = s_5_37;
        // D s_5_39: read-var Rn:u8
        let s_5_39: u8 = fn_state.Rn;
        // D s_5_40: cast zx s_5_39 -> bv
        let s_5_40: Bits = Bits::new(s_5_39 as u128, 4u16);
        // D s_5_41: cast zx s_5_40 -> i
        let s_5_41: i128 = (s_5_40.value() as i128);
        // D s_5_42: cast reint s_5_41 -> i64
        let s_5_42: i64 = (s_5_41 as i64);
        // D s_5_43: write-var n <= s_5_42
        fn_state.n = s_5_42;
        // D s_5_44: read-var Rm:u8
        let s_5_44: u8 = fn_state.Rm;
        // D s_5_45: cast zx s_5_44 -> bv
        let s_5_45: Bits = Bits::new(s_5_44 as u128, 4u16);
        // D s_5_46: cast zx s_5_45 -> i
        let s_5_46: i128 = (s_5_45.value() as i128);
        // D s_5_47: cast reint s_5_46 -> i64
        let s_5_47: i64 = (s_5_46 as i64);
        // D s_5_48: write-var m <= s_5_47
        fn_state.m = s_5_47;
        // C s_5_49: const #15s : i
        let s_5_49: i128 = 15;
        // D s_5_50: read-var m:i64
        let s_5_50: i64 = fn_state.m;
        // D s_5_51: cast zx s_5_50 -> i
        let s_5_51: i128 = (i128::try_from(s_5_50).unwrap());
        // D s_5_52: call neq_int(s_5_51, s_5_49)
        let s_5_52: bool = neq_int(state, tracer, s_5_51, s_5_49);
        // D s_5_53: write-var wback <= s_5_52
        fn_state.wback = s_5_52;
        // C s_5_54: const #15s : i
        let s_5_54: i128 = 15;
        // D s_5_55: read-var m:i64
        let s_5_55: i64 = fn_state.m;
        // D s_5_56: cast zx s_5_55 -> i
        let s_5_56: i128 = (i128::try_from(s_5_55).unwrap());
        // D s_5_57: call neq_int(s_5_56, s_5_54)
        let s_5_57: bool = neq_int(state, tracer, s_5_56, s_5_54);
        // N s_5_58: branch s_5_57 b61 b6
        if s_5_57 {
            return block_61(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#321084 <= s_6_0
        fn_state.gs_321084 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#321084:u8
        let s_7_0: bool = fn_state.gs_321084;
        // D s_7_1: write-var register_index <= s_7_0
        fn_state.register_index = s_7_0;
        // C s_7_2: const #15s : i
        let s_7_2: i128 = 15;
        // D s_7_3: read-var n:i64
        let s_7_3: i64 = fn_state.n;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cmp-eq s_7_4 s_7_2
        let s_7_5: bool = ((s_7_4) == (s_7_2));
        // N s_7_6: branch s_7_5 b60 b8
        if s_7_5 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var d2:i64
        let s_8_0: i64 = fn_state.d2;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // C s_8_2: const #2s : i64
        let s_8_2: i64 = 2;
        // C s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: add s_8_1 s_8_3
        let s_8_4: i128 = (s_8_1 + s_8_3);
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // C s_8_6: const #32s : i
        let s_8_6: i128 = 32;
        // D s_8_7: cast zx s_8_5 -> i
        let s_8_7: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_8: cmp-gt s_8_7 s_8_6
        let s_8_8: bool = ((s_8_7) > (s_8_6));
        // D s_8_9: write-var gs#321087 <= s_8_8
        fn_state.gs_321087 = s_8_8;
        // N s_8_10: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#321087:u8
        let s_9_0: bool = fn_state.gs_321087;
        // N s_9_1: branch s_9_0 b59 b10
        if s_9_0 {
            return block_59(state, tracer, fn_state);
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
        // C s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // S s_10_2: call __id(s_10_1)
        let s_10_2: i128 = u__id(state, tracer, s_10_1);
        // S s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #1s : i
        let s_10_4: i128 = 1;
        // S s_10_5: cast zx s_10_3 -> i
        let s_10_5: i128 = (i128::try_from(s_10_3).unwrap());
        // S s_10_6: cmp-eq s_10_5 s_10_4
        let s_10_6: bool = ((s_10_5) == (s_10_4));
        // N s_10_7: branch s_10_6 b58 b11
        if s_10_6 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2s : i64
        let s_11_0: i64 = 2;
        // C s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // S s_11_2: call __id(s_11_1)
        let s_11_2: i128 = u__id(state, tracer, s_11_1);
        // S s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #2s : i
        let s_11_4: i128 = 2;
        // S s_11_5: cast zx s_11_3 -> i
        let s_11_5: i128 = (i128::try_from(s_11_3).unwrap());
        // S s_11_6: cmp-eq s_11_5 s_11_4
        let s_11_6: bool = ((s_11_5) == (s_11_4));
        // D s_11_7: write-var gs#321090 <= s_11_6
        fn_state.gs_321090 = s_11_6;
        // N s_11_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#321090:u8
        let s_12_0: bool = fn_state.gs_321090;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#321120 <= s_13_0
        fn_state.gs_321120 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#321120:u8
        let s_14_0: bool = fn_state.gs_321120;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // D s_14_2: read-var alignment:i
        let s_14_2: i128 = fn_state.alignment;
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // D s_14_4: read-var d2:i64
        let s_14_4: i64 = fn_state.d2;
        // D s_14_5: cast zx s_14_4 -> i
        let s_14_5: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_6: read-var elements:i64
        let s_14_6: i64 = fn_state.elements;
        // D s_14_7: cast zx s_14_6 -> i
        let s_14_7: i128 = (i128::try_from(s_14_6).unwrap());
        // D s_14_8: read-var d:i64
        let s_14_8: i64 = fn_state.d;
        // D s_14_9: read-var ebytes:i64
        let s_14_9: i64 = fn_state.ebytes;
        // D s_14_10: read-var m:i64
        let s_14_10: i64 = fn_state.m;
        // D s_14_11: read-var n:i64
        let s_14_11: i64 = fn_state.n;
        // C s_14_12: const #2s : i64
        let s_14_12: i64 = 2;
        // D s_14_13: read-var register_index:u8
        let s_14_13: bool = fn_state.register_index;
        // D s_14_14: read-var wback:u8
        let s_14_14: bool = fn_state.wback;
        // D s_14_15: call execute_aarch32_instrs_VST2_m_Op_A_txt(s_14_3, s_14_8, s_14_5, s_14_9, s_14_7, s_14_10, s_14_11, s_14_12, s_14_13, s_14_14)
        let s_14_15: () = execute_aarch32_instrs_VST2_m_Op_A_txt(
            state,
            tracer,
            s_14_3,
            s_14_8,
            s_14_5,
            s_14_9,
            s_14_7,
            s_14_10,
            s_14_11,
            s_14_12,
            s_14_13,
            s_14_14,
        );
        // N s_14_16: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var n:i64
        let s_15_0: i64 = fn_state.n;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call __id(s_15_1)
        let s_15_2: i128 = u__id(state, tracer, s_15_1);
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: const #0s : i
        let s_15_4: i128 = 0;
        // D s_15_5: cast zx s_15_3 -> i
        let s_15_5: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_6: cmp-le s_15_4 s_15_5
        let s_15_6: bool = ((s_15_4) <= (s_15_5));
        // N s_15_7: branch s_15_6 b18 b16
        if s_15_6 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#321119 <= s_16_0
        fn_state.gs_321119 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#321119:u8
        let s_17_0: bool = fn_state.gs_321119;
        // D s_17_1: write-var gs#321120 <= s_17_0
        fn_state.gs_321120 = s_17_0;
        // N s_17_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var n:i64
        let s_18_0: i64 = fn_state.n;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: call __id(s_18_1)
        let s_18_2: i128 = u__id(state, tracer, s_18_1);
        // D s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: const #15s : i
        let s_18_4: i128 = 15;
        // D s_18_5: cast zx s_18_3 -> i
        let s_18_5: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_6: cmp-le s_18_5 s_18_4
        let s_18_6: bool = ((s_18_5) <= (s_18_4));
        // N s_18_7: branch s_18_6 b21 b19
        if s_18_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#321118 <= s_19_0
        fn_state.gs_321118 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#321118:u8
        let s_20_0: bool = fn_state.gs_321118;
        // D s_20_1: write-var gs#321119 <= s_20_0
        fn_state.gs_321119 = s_20_0;
        // N s_20_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var m:i64
        let s_21_0: i64 = fn_state.m;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: call __id(s_21_1)
        let s_21_2: i128 = u__id(state, tracer, s_21_1);
        // D s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: const #0s : i
        let s_21_4: i128 = 0;
        // D s_21_5: cast zx s_21_3 -> i
        let s_21_5: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_6: cmp-le s_21_4 s_21_5
        let s_21_6: bool = ((s_21_4) <= (s_21_5));
        // N s_21_7: branch s_21_6 b24 b22
        if s_21_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#321117 <= s_22_0
        fn_state.gs_321117 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#321117:u8
        let s_23_0: bool = fn_state.gs_321117;
        // D s_23_1: write-var gs#321118 <= s_23_0
        fn_state.gs_321118 = s_23_0;
        // N s_23_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var m:i64
        let s_24_0: i64 = fn_state.m;
        // D s_24_1: cast zx s_24_0 -> i
        let s_24_1: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_2: call __id(s_24_1)
        let s_24_2: i128 = u__id(state, tracer, s_24_1);
        // D s_24_3: cast reint s_24_2 -> i64
        let s_24_3: i64 = (s_24_2 as i64);
        // C s_24_4: const #15s : i
        let s_24_4: i128 = 15;
        // D s_24_5: cast zx s_24_3 -> i
        let s_24_5: i128 = (i128::try_from(s_24_3).unwrap());
        // D s_24_6: cmp-le s_24_5 s_24_4
        let s_24_6: bool = ((s_24_5) <= (s_24_4));
        // N s_24_7: branch s_24_6 b27 b25
        if s_24_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#321116 <= s_25_0
        fn_state.gs_321116 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#321116:u8
        let s_26_0: bool = fn_state.gs_321116;
        // D s_26_1: write-var gs#321117 <= s_26_0
        fn_state.gs_321117 = s_26_0;
        // N s_26_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var ebytes:i64
        let s_27_0: i64 = fn_state.ebytes;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: call __id(s_27_1)
        let s_27_2: i128 = u__id(state, tracer, s_27_1);
        // D s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: const #1s : i
        let s_27_4: i128 = 1;
        // D s_27_5: cast zx s_27_3 -> i
        let s_27_5: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_6: cmp-eq s_27_5 s_27_4
        let s_27_6: bool = ((s_27_5) == (s_27_4));
        // N s_27_7: branch s_27_6 b57 b28
        if s_27_6 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var ebytes:i64
        let s_28_0: i64 = fn_state.ebytes;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: call __id(s_28_1)
        let s_28_2: i128 = u__id(state, tracer, s_28_1);
        // D s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: const #2s : i
        let s_28_4: i128 = 2;
        // D s_28_5: cast zx s_28_3 -> i
        let s_28_5: i128 = (i128::try_from(s_28_3).unwrap());
        // D s_28_6: cmp-eq s_28_5 s_28_4
        let s_28_6: bool = ((s_28_5) == (s_28_4));
        // D s_28_7: write-var gs#321097 <= s_28_6
        fn_state.gs_321097 = s_28_6;
        // N s_28_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#321097:u8
        let s_29_0: bool = fn_state.gs_321097;
        // N s_29_1: branch s_29_0 b56 b30
        if s_29_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var ebytes:i64
        let s_30_0: i64 = fn_state.ebytes;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: call __id(s_30_1)
        let s_30_2: i128 = u__id(state, tracer, s_30_1);
        // D s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: const #4s : i
        let s_30_4: i128 = 4;
        // D s_30_5: cast zx s_30_3 -> i
        let s_30_5: i128 = (i128::try_from(s_30_3).unwrap());
        // D s_30_6: cmp-eq s_30_5 s_30_4
        let s_30_6: bool = ((s_30_5) == (s_30_4));
        // D s_30_7: write-var gs#321099 <= s_30_6
        fn_state.gs_321099 = s_30_6;
        // N s_30_8: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#321099:u8
        let s_31_0: bool = fn_state.gs_321099;
        // N s_31_1: branch s_31_0 b55 b32
        if s_31_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var ebytes:i64
        let s_32_0: i64 = fn_state.ebytes;
        // D s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_2: call __id(s_32_1)
        let s_32_2: i128 = u__id(state, tracer, s_32_1);
        // D s_32_3: cast reint s_32_2 -> i64
        let s_32_3: i64 = (s_32_2 as i64);
        // C s_32_4: const #8s : i
        let s_32_4: i128 = 8;
        // D s_32_5: cast zx s_32_3 -> i
        let s_32_5: i128 = (i128::try_from(s_32_3).unwrap());
        // D s_32_6: cmp-eq s_32_5 s_32_4
        let s_32_6: bool = ((s_32_5) == (s_32_4));
        // D s_32_7: write-var gs#321101 <= s_32_6
        fn_state.gs_321101 = s_32_6;
        // N s_32_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#321101:u8
        let s_33_0: bool = fn_state.gs_321101;
        // N s_33_1: branch s_33_0 b36 b34
        if s_33_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#321115 <= s_34_0
        fn_state.gs_321115 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#321115:u8
        let s_35_0: bool = fn_state.gs_321115;
        // D s_35_1: write-var gs#321116 <= s_35_0
        fn_state.gs_321116 = s_35_0;
        // N s_35_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var d:i64
        let s_36_0: i64 = fn_state.d;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: call __id(s_36_1)
        let s_36_2: i128 = u__id(state, tracer, s_36_1);
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: const #0s : i
        let s_36_4: i128 = 0;
        // D s_36_5: cast zx s_36_3 -> i
        let s_36_5: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_6: cmp-le s_36_4 s_36_5
        let s_36_6: bool = ((s_36_4) <= (s_36_5));
        // N s_36_7: branch s_36_6 b39 b37
        if s_36_6 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#321114 <= s_37_0
        fn_state.gs_321114 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#321114:u8
        let s_38_0: bool = fn_state.gs_321114;
        // D s_38_1: write-var gs#321115 <= s_38_0
        fn_state.gs_321115 = s_38_0;
        // N s_38_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var d:i64
        let s_39_0: i64 = fn_state.d;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: call __id(s_39_1)
        let s_39_2: i128 = u__id(state, tracer, s_39_1);
        // D s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: const #31s : i
        let s_39_4: i128 = 31;
        // D s_39_5: cast zx s_39_3 -> i
        let s_39_5: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_6: cmp-le s_39_5 s_39_4
        let s_39_6: bool = ((s_39_5) <= (s_39_4));
        // N s_39_7: branch s_39_6 b42 b40
        if s_39_6 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#321113 <= s_40_0
        fn_state.gs_321113 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#321113:u8
        let s_41_0: bool = fn_state.gs_321113;
        // D s_41_1: write-var gs#321114 <= s_41_0
        fn_state.gs_321114 = s_41_0;
        // N s_41_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var alignment:i
        let s_42_0: i128 = fn_state.alignment;
        // D s_42_1: call __id(s_42_0)
        let s_42_1: i128 = u__id(state, tracer, s_42_0);
        // C s_42_2: const #1s : i
        let s_42_2: i128 = 1;
        // D s_42_3: cmp-eq s_42_1 s_42_2
        let s_42_3: bool = ((s_42_1) == (s_42_2));
        // N s_42_4: branch s_42_3 b54 b43
        if s_42_3 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var alignment:i
        let s_43_0: i128 = fn_state.alignment;
        // D s_43_1: call __id(s_43_0)
        let s_43_1: i128 = u__id(state, tracer, s_43_0);
        // C s_43_2: const #4s : i
        let s_43_2: i128 = 4;
        // D s_43_3: cmp-eq s_43_1 s_43_2
        let s_43_3: bool = ((s_43_1) == (s_43_2));
        // D s_43_4: write-var gs#321106 <= s_43_3
        fn_state.gs_321106 = s_43_3;
        // N s_43_5: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#321106:u8
        let s_44_0: bool = fn_state.gs_321106;
        // N s_44_1: branch s_44_0 b53 b45
        if s_44_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var alignment:i
        let s_45_0: i128 = fn_state.alignment;
        // D s_45_1: call __id(s_45_0)
        let s_45_1: i128 = u__id(state, tracer, s_45_0);
        // C s_45_2: const #8s : i
        let s_45_2: i128 = 8;
        // D s_45_3: cmp-eq s_45_1 s_45_2
        let s_45_3: bool = ((s_45_1) == (s_45_2));
        // D s_45_4: write-var gs#321108 <= s_45_3
        fn_state.gs_321108 = s_45_3;
        // N s_45_5: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#321108:u8
        let s_46_0: bool = fn_state.gs_321108;
        // N s_46_1: branch s_46_0 b52 b47
        if s_46_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var alignment:i
        let s_47_0: i128 = fn_state.alignment;
        // D s_47_1: call __id(s_47_0)
        let s_47_1: i128 = u__id(state, tracer, s_47_0);
        // C s_47_2: const #16s : i
        let s_47_2: i128 = 16;
        // D s_47_3: cmp-eq s_47_1 s_47_2
        let s_47_3: bool = ((s_47_1) == (s_47_2));
        // D s_47_4: write-var gs#321110 <= s_47_3
        fn_state.gs_321110 = s_47_3;
        // N s_47_5: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#321110:u8
        let s_48_0: bool = fn_state.gs_321110;
        // N s_48_1: branch s_48_0 b51 b49
        if s_48_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var alignment:i
        let s_49_0: i128 = fn_state.alignment;
        // D s_49_1: call __id(s_49_0)
        let s_49_1: i128 = u__id(state, tracer, s_49_0);
        // C s_49_2: const #32s : i
        let s_49_2: i128 = 32;
        // D s_49_3: cmp-eq s_49_1 s_49_2
        let s_49_3: bool = ((s_49_1) == (s_49_2));
        // D s_49_4: write-var gs#321112 <= s_49_3
        fn_state.gs_321112 = s_49_3;
        // N s_49_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#321112:u8
        let s_50_0: bool = fn_state.gs_321112;
        // D s_50_1: write-var gs#321113 <= s_50_0
        fn_state.gs_321113 = s_50_0;
        // N s_50_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #1u : u8
        let s_51_0: bool = true;
        // D s_51_1: write-var gs#321112 <= s_51_0
        fn_state.gs_321112 = s_51_0;
        // N s_51_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: write-var gs#321110 <= s_52_0
        fn_state.gs_321110 = s_52_0;
        // N s_52_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#321108 <= s_53_0
        fn_state.gs_321108 = s_53_0;
        // N s_53_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#321106 <= s_54_0
        fn_state.gs_321106 = s_54_0;
        // N s_54_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#321101 <= s_55_0
        fn_state.gs_321101 = s_55_0;
        // N s_55_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#321099 <= s_56_0
        fn_state.gs_321099 = s_56_0;
        // N s_56_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#321097 <= s_57_0
        fn_state.gs_321097 = s_57_0;
        // N s_57_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#321090 <= s_58_0
        fn_state.gs_321090 = s_58_0;
        // N s_58_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: panic
        panic!("{:?}", ());
        // N s_59_1: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#321087 <= s_60_0
        fn_state.gs_321087 = s_60_0;
        // N s_60_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #13s : i
        let s_61_0: i128 = 13;
        // D s_61_1: read-var m:i64
        let s_61_1: i64 = fn_state.m;
        // D s_61_2: cast zx s_61_1 -> i
        let s_61_2: i128 = (i128::try_from(s_61_1).unwrap());
        // D s_61_3: call neq_int(s_61_2, s_61_0)
        let s_61_3: bool = neq_int(state, tracer, s_61_2, s_61_0);
        // D s_61_4: write-var gs#321084 <= s_61_3
        fn_state.gs_321084 = s_61_3;
        // N s_61_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #1s : i
        let s_62_0: i128 = 1;
        // D s_62_1: write-var ga#362101 <= s_62_0
        fn_state.ga_362101 = s_62_0;
        // N s_62_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_63_0: panic
        panic!("{:?}", ());
        // N s_63_1: return
        return;
    }
}
