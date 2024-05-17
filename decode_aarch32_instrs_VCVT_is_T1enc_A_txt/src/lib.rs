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
use execute_aarch32_instrs_VCVT_is_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_VCVT_is_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    size: u8,
    Vd: u8,
    op: u8,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        to_integer: bool,
        esize: i64,
        gs_308058: bool,
        elementsshadow_7411: i64,
        gs_308057: bool,
        d: i64,
        ga_352403: i64,
        is_unsigned: bool,
        elements: i64,
        gs_308061: bool,
        gs_308059: bool,
        esizeshadow_7410: i64,
        D: bool,
        size: u8,
        Vd: u8,
        op: u8,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        size,
        Vd,
        op,
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
        // N s_2_5: branch s_2_4 b25 b3
        if s_2_4 {
            return block_25(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#308058 <= s_3_0
        fn_state.gs_308058 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#308058:u8
        let s_4_0: bool = fn_state.gs_308058;
        // N s_4_1: branch s_4_0 b24 b5
        if s_4_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var size:u8
        let s_5_0: u8 = fn_state.size;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #0u : u8
        let s_5_2: u8 = 0;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b23 b6
        if s_5_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var size:u8
        let s_6_0: u8 = fn_state.size;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #3u : u8
        let s_6_2: u8 = 3;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: write-var gs#308059 <= s_6_4
        fn_state.gs_308059 = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#308059:u8
        let s_7_0: bool = fn_state.gs_308059;
        // N s_7_1: branch s_7_0 b22 b8
        if s_7_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var size:u8
        let s_8_0: u8 = fn_state.size;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #1u : u8
        let s_8_2: u8 = 1;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b21 b9
        if s_8_4 {
            return block_21(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#308061 <= s_9_0
        fn_state.gs_308061 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#308061:u8
        let s_10_0: bool = fn_state.gs_308061;
        // N s_10_1: branch s_10_0 b20 b11
        if s_10_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1s : i
        let s_11_0: i128 = 1;
        // D s_11_1: read-var op:u8
        let s_11_1: u8 = fn_state.op;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 2u16);
        // C s_11_3: const #1u : u64
        let s_11_3: u64 = 1;
        // D s_11_4: bit-extract s_11_2 s_11_0 s_11_3
        let s_11_4: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_3).unwrap(),
        ));
        // D s_11_5: cast reint s_11_4 -> u8
        let s_11_5: bool = ((s_11_4.value()) != 0);
        // C s_11_6: const #0s : i
        let s_11_6: i128 = 0;
        // C s_11_7: const #0u : u64
        let s_11_7: u64 = 0;
        // D s_11_8: cast zx s_11_5 -> u64
        let s_11_8: u64 = (s_11_5 as u64);
        // C s_11_9: const #1u : u64
        let s_11_9: u64 = 1;
        // D s_11_10: and s_11_8 s_11_9
        let s_11_10: u64 = ((s_11_8) & (s_11_9));
        // D s_11_11: cmp-eq s_11_10 s_11_9
        let s_11_11: bool = ((s_11_10) == (s_11_9));
        // D s_11_12: lsl s_11_8 s_11_6
        let s_11_12: u64 = s_11_8 << s_11_6;
        // D s_11_13: or s_11_7 s_11_12
        let s_11_13: u64 = ((s_11_7) | (s_11_12));
        // D s_11_14: cmpl s_11_12
        let s_11_14: u64 = !s_11_12;
        // D s_11_15: and s_11_7 s_11_14
        let s_11_15: u64 = ((s_11_7) & (s_11_14));
        // D s_11_16: select s_11_11 s_11_13 s_11_15
        let s_11_16: u64 = if s_11_11 { s_11_13 } else { s_11_15 };
        // D s_11_17: cast trunc s_11_16 -> u8
        let s_11_17: bool = ((s_11_16) != 0);
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 1u16);
        // C s_11_19: const #1u : u8
        let s_11_19: bool = true;
        // C s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 1u16);
        // D s_11_21: cmp-eq s_11_18 s_11_20
        let s_11_21: bool = ((s_11_18) == (s_11_20));
        // D s_11_22: write-var to_integer <= s_11_21
        fn_state.to_integer = s_11_21;
        // C s_11_23: const #0s : i
        let s_11_23: i128 = 0;
        // D s_11_24: read-var op:u8
        let s_11_24: u8 = fn_state.op;
        // D s_11_25: cast zx s_11_24 -> bv
        let s_11_25: Bits = Bits::new(s_11_24 as u128, 2u16);
        // C s_11_26: const #1u : u64
        let s_11_26: u64 = 1;
        // D s_11_27: bit-extract s_11_25 s_11_23 s_11_26
        let s_11_27: Bits = (Bits::new(
            ((s_11_25) >> (s_11_23)).value(),
            u16::try_from(s_11_26).unwrap(),
        ));
        // D s_11_28: cast reint s_11_27 -> u8
        let s_11_28: bool = ((s_11_27.value()) != 0);
        // C s_11_29: const #0s : i
        let s_11_29: i128 = 0;
        // C s_11_30: const #0u : u64
        let s_11_30: u64 = 0;
        // D s_11_31: cast zx s_11_28 -> u64
        let s_11_31: u64 = (s_11_28 as u64);
        // C s_11_32: const #1u : u64
        let s_11_32: u64 = 1;
        // D s_11_33: and s_11_31 s_11_32
        let s_11_33: u64 = ((s_11_31) & (s_11_32));
        // D s_11_34: cmp-eq s_11_33 s_11_32
        let s_11_34: bool = ((s_11_33) == (s_11_32));
        // D s_11_35: lsl s_11_31 s_11_29
        let s_11_35: u64 = s_11_31 << s_11_29;
        // D s_11_36: or s_11_30 s_11_35
        let s_11_36: u64 = ((s_11_30) | (s_11_35));
        // D s_11_37: cmpl s_11_35
        let s_11_37: u64 = !s_11_35;
        // D s_11_38: and s_11_30 s_11_37
        let s_11_38: u64 = ((s_11_30) & (s_11_37));
        // D s_11_39: select s_11_34 s_11_36 s_11_38
        let s_11_39: u64 = if s_11_34 { s_11_36 } else { s_11_38 };
        // D s_11_40: cast trunc s_11_39 -> u8
        let s_11_40: bool = ((s_11_39) != 0);
        // D s_11_41: cast zx s_11_40 -> bv
        let s_11_41: Bits = Bits::new(s_11_40 as u128, 1u16);
        // C s_11_42: const #1u : u8
        let s_11_42: bool = true;
        // C s_11_43: cast zx s_11_42 -> bv
        let s_11_43: Bits = Bits::new(s_11_42 as u128, 1u16);
        // D s_11_44: cmp-eq s_11_41 s_11_43
        let s_11_44: bool = ((s_11_41) == (s_11_43));
        // D s_11_45: write-var is_unsigned <= s_11_44
        fn_state.is_unsigned = s_11_44;
        // C s_11_46: const #16s : i64
        let s_11_46: i64 = 16;
        // D s_11_47: write-var esize <= s_11_46
        fn_state.esize = s_11_46;
        // C s_11_48: const #2s : i64
        let s_11_48: i64 = 2;
        // D s_11_49: write-var elements <= s_11_48
        fn_state.elements = s_11_48;
        // D s_11_50: read-var size:u8
        let s_11_50: u8 = fn_state.size;
        // D s_11_51: cast zx s_11_50 -> bv
        let s_11_51: Bits = Bits::new(s_11_50 as u128, 2u16);
        // C s_11_52: const #1u : u8
        let s_11_52: u8 = 1;
        // C s_11_53: cast zx s_11_52 -> bv
        let s_11_53: Bits = Bits::new(s_11_52 as u128, 2u16);
        // D s_11_54: cmp-eq s_11_51 s_11_53
        let s_11_54: bool = ((s_11_51) == (s_11_53));
        // D s_11_55: not s_11_54
        let s_11_55: bool = !s_11_54;
        // N s_11_56: branch s_11_55 b17 b12
        if s_11_55 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #16s : i64
        let s_12_0: i64 = 16;
        // D s_12_1: write-var esize <= s_12_0
        fn_state.esize = s_12_0;
        // C s_12_2: const #4s : i64
        let s_12_2: i64 = 4;
        // D s_12_3: write-var elements <= s_12_2
        fn_state.elements = s_12_2;
        // N s_12_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var esize:i64
        let s_13_0: i64 = fn_state.esize;
        // D s_13_1: write-var esizeshadow#7410 <= s_13_0
        fn_state.esizeshadow_7410 = s_13_0;
        // D s_13_2: read-var elements:i64
        let s_13_2: i64 = fn_state.elements;
        // D s_13_3: write-var elementsshadow#7411 <= s_13_2
        fn_state.elementsshadow_7411 = s_13_2;
        // D s_13_4: read-var D:u8
        let s_13_4: bool = fn_state.D;
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 1u16);
        // D s_13_6: read-var Vd:u8
        let s_13_6: u8 = fn_state.Vd;
        // D s_13_7: cast zx s_13_6 -> bv
        let s_13_7: Bits = Bits::new(s_13_6 as u128, 4u16);
        // D s_13_8: cast reint s_13_5 -> u128
        let s_13_8: u128 = (s_13_5.value() as u128);
        // D s_13_9: size-of s_13_5
        let s_13_9: u16 = s_13_5.length();
        // D s_13_10: cast reint s_13_7 -> u128
        let s_13_10: u128 = (s_13_7.value() as u128);
        // D s_13_11: size-of s_13_7
        let s_13_11: u16 = s_13_7.length();
        // D s_13_12: lsl s_13_8 s_13_11
        let s_13_12: u128 = s_13_8 << s_13_11;
        // D s_13_13: or s_13_12 s_13_10
        let s_13_13: u128 = ((s_13_12) | (s_13_10));
        // D s_13_14: add s_13_9 s_13_11
        let s_13_14: u16 = (s_13_9 + s_13_11);
        // D s_13_15: create-bits s_13_13 s_13_14
        let s_13_15: Bits = Bits::new(s_13_13, s_13_14);
        // D s_13_16: cast reint s_13_15 -> u8
        let s_13_16: u8 = (s_13_15.value() as u8);
        // D s_13_17: cast zx s_13_16 -> bv
        let s_13_17: Bits = Bits::new(s_13_16 as u128, 5u16);
        // D s_13_18: cast zx s_13_17 -> i
        let s_13_18: i128 = (s_13_17.value() as i128);
        // D s_13_19: cast reint s_13_18 -> i64
        let s_13_19: i64 = (s_13_18 as i64);
        // D s_13_20: write-var d <= s_13_19
        fn_state.d = s_13_19;
        // D s_13_21: read-var M:u8
        let s_13_21: bool = fn_state.M;
        // D s_13_22: cast zx s_13_21 -> bv
        let s_13_22: Bits = Bits::new(s_13_21 as u128, 1u16);
        // D s_13_23: read-var Vm:u8
        let s_13_23: u8 = fn_state.Vm;
        // D s_13_24: cast zx s_13_23 -> bv
        let s_13_24: Bits = Bits::new(s_13_23 as u128, 4u16);
        // D s_13_25: cast reint s_13_22 -> u128
        let s_13_25: u128 = (s_13_22.value() as u128);
        // D s_13_26: size-of s_13_22
        let s_13_26: u16 = s_13_22.length();
        // D s_13_27: cast reint s_13_24 -> u128
        let s_13_27: u128 = (s_13_24.value() as u128);
        // D s_13_28: size-of s_13_24
        let s_13_28: u16 = s_13_24.length();
        // D s_13_29: lsl s_13_25 s_13_28
        let s_13_29: u128 = s_13_25 << s_13_28;
        // D s_13_30: or s_13_29 s_13_27
        let s_13_30: u128 = ((s_13_29) | (s_13_27));
        // D s_13_31: add s_13_26 s_13_28
        let s_13_31: u16 = (s_13_26 + s_13_28);
        // D s_13_32: create-bits s_13_30 s_13_31
        let s_13_32: Bits = Bits::new(s_13_30, s_13_31);
        // D s_13_33: cast reint s_13_32 -> u8
        let s_13_33: u8 = (s_13_32.value() as u8);
        // D s_13_34: cast zx s_13_33 -> bv
        let s_13_34: Bits = Bits::new(s_13_33 as u128, 5u16);
        // D s_13_35: cast zx s_13_34 -> i
        let s_13_35: i128 = (s_13_34.value() as i128);
        // D s_13_36: cast reint s_13_35 -> i64
        let s_13_36: i64 = (s_13_35 as i64);
        // D s_13_37: write-var m <= s_13_36
        fn_state.m = s_13_36;
        // D s_13_38: read-var Q:u8
        let s_13_38: bool = fn_state.Q;
        // D s_13_39: cast zx s_13_38 -> bv
        let s_13_39: Bits = Bits::new(s_13_38 as u128, 1u16);
        // C s_13_40: const #0u : u8
        let s_13_40: bool = false;
        // C s_13_41: cast zx s_13_40 -> bv
        let s_13_41: Bits = Bits::new(s_13_40 as u128, 1u16);
        // D s_13_42: cmp-eq s_13_39 s_13_41
        let s_13_42: bool = ((s_13_39) == (s_13_41));
        // N s_13_43: branch s_13_42 b16 b14
        if s_13_42 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #2s : i64
        let s_14_0: i64 = 2;
        // D s_14_1: write-var ga#352403 <= s_14_0
        fn_state.ga_352403 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ga#352403:i64
        let s_15_0: i64 = fn_state.ga_352403;
        // D s_15_1: read-var esizeshadow#7410:i64
        let s_15_1: i64 = fn_state.esizeshadow_7410;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // D s_15_4: read-var d:i64
        let s_15_4: i64 = fn_state.d;
        // D s_15_5: read-var elementsshadow#7411:i64
        let s_15_5: i64 = fn_state.elementsshadow_7411;
        // D s_15_6: read-var m:i64
        let s_15_6: i64 = fn_state.m;
        // D s_15_7: read-var to_integer:u8
        let s_15_7: bool = fn_state.to_integer;
        // D s_15_8: read-var is_unsigned:u8
        let s_15_8: bool = fn_state.is_unsigned;
        // D s_15_9: call execute_aarch32_instrs_VCVT_is_Op_A_txt(s_15_4, s_15_5, s_15_3, s_15_6, s_15_0, s_15_7, s_15_8)
        let s_15_9: () = execute_aarch32_instrs_VCVT_is_Op_A_txt(
            state,
            tracer,
            s_15_4,
            s_15_5,
            s_15_3,
            s_15_6,
            s_15_0,
            s_15_7,
            s_15_8,
        );
        // N s_15_10: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1s : i64
        let s_16_0: i64 = 1;
        // D s_16_1: write-var ga#352403 <= s_16_0
        fn_state.ga_352403 = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var size:u8
        let s_17_0: u8 = fn_state.size;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: not s_17_4
        let s_17_5: bool = !s_17_4;
        // N s_17_6: branch s_17_5 b19 b18
        if s_17_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #32s : i64
        let s_18_0: i64 = 32;
        // D s_18_1: write-var esize <= s_18_0
        fn_state.esize = s_18_0;
        // C s_18_2: const #2s : i64
        let s_18_2: i64 = 2;
        // D s_18_3: write-var elements <= s_18_2
        fn_state.elements = s_18_2;
        // N s_18_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b13
        return block_13(state, tracer, fn_state);
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
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call InITBlock(s_21_0)
        let s_21_1: bool = InITBlock(state, tracer, s_21_0);
        // D s_21_2: write-var gs#308061 <= s_21_1
        fn_state.gs_308061 = s_21_1;
        // N s_21_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#308059 <= s_23_0
        fn_state.gs_308059 = s_23_0;
        // N s_23_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0s : i
        let s_25_0: i128 = 0;
        // D s_25_1: read-var Vd:u8
        let s_25_1: u8 = fn_state.Vd;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 4u16);
        // C s_25_3: const #1u : u64
        let s_25_3: u64 = 1;
        // D s_25_4: bit-extract s_25_2 s_25_0 s_25_3
        let s_25_4: Bits = (Bits::new(
            ((s_25_2) >> (s_25_0)).value(),
            u16::try_from(s_25_3).unwrap(),
        ));
        // D s_25_5: cast reint s_25_4 -> u8
        let s_25_5: bool = ((s_25_4.value()) != 0);
        // C s_25_6: const #0s : i
        let s_25_6: i128 = 0;
        // C s_25_7: const #0u : u64
        let s_25_7: u64 = 0;
        // D s_25_8: cast zx s_25_5 -> u64
        let s_25_8: u64 = (s_25_5 as u64);
        // C s_25_9: const #1u : u64
        let s_25_9: u64 = 1;
        // D s_25_10: and s_25_8 s_25_9
        let s_25_10: u64 = ((s_25_8) & (s_25_9));
        // D s_25_11: cmp-eq s_25_10 s_25_9
        let s_25_11: bool = ((s_25_10) == (s_25_9));
        // D s_25_12: lsl s_25_8 s_25_6
        let s_25_12: u64 = s_25_8 << s_25_6;
        // D s_25_13: or s_25_7 s_25_12
        let s_25_13: u64 = ((s_25_7) | (s_25_12));
        // D s_25_14: cmpl s_25_12
        let s_25_14: u64 = !s_25_12;
        // D s_25_15: and s_25_7 s_25_14
        let s_25_15: u64 = ((s_25_7) & (s_25_14));
        // D s_25_16: select s_25_11 s_25_13 s_25_15
        let s_25_16: u64 = if s_25_11 { s_25_13 } else { s_25_15 };
        // D s_25_17: cast trunc s_25_16 -> u8
        let s_25_17: bool = ((s_25_16) != 0);
        // D s_25_18: cast zx s_25_17 -> bv
        let s_25_18: Bits = Bits::new(s_25_17 as u128, 1u16);
        // C s_25_19: const #1u : u8
        let s_25_19: bool = true;
        // C s_25_20: cast zx s_25_19 -> bv
        let s_25_20: Bits = Bits::new(s_25_19 as u128, 1u16);
        // D s_25_21: cmp-eq s_25_18 s_25_20
        let s_25_21: bool = ((s_25_18) == (s_25_20));
        // N s_25_22: branch s_25_21 b28 b26
        if s_25_21 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0s : i
        let s_26_0: i128 = 0;
        // D s_26_1: read-var Vm:u8
        let s_26_1: u8 = fn_state.Vm;
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 4u16);
        // C s_26_3: const #1u : u64
        let s_26_3: u64 = 1;
        // D s_26_4: bit-extract s_26_2 s_26_0 s_26_3
        let s_26_4: Bits = (Bits::new(
            ((s_26_2) >> (s_26_0)).value(),
            u16::try_from(s_26_3).unwrap(),
        ));
        // D s_26_5: cast reint s_26_4 -> u8
        let s_26_5: bool = ((s_26_4.value()) != 0);
        // C s_26_6: const #0s : i
        let s_26_6: i128 = 0;
        // C s_26_7: const #0u : u64
        let s_26_7: u64 = 0;
        // D s_26_8: cast zx s_26_5 -> u64
        let s_26_8: u64 = (s_26_5 as u64);
        // C s_26_9: const #1u : u64
        let s_26_9: u64 = 1;
        // D s_26_10: and s_26_8 s_26_9
        let s_26_10: u64 = ((s_26_8) & (s_26_9));
        // D s_26_11: cmp-eq s_26_10 s_26_9
        let s_26_11: bool = ((s_26_10) == (s_26_9));
        // D s_26_12: lsl s_26_8 s_26_6
        let s_26_12: u64 = s_26_8 << s_26_6;
        // D s_26_13: or s_26_7 s_26_12
        let s_26_13: u64 = ((s_26_7) | (s_26_12));
        // D s_26_14: cmpl s_26_12
        let s_26_14: u64 = !s_26_12;
        // D s_26_15: and s_26_7 s_26_14
        let s_26_15: u64 = ((s_26_7) & (s_26_14));
        // D s_26_16: select s_26_11 s_26_13 s_26_15
        let s_26_16: u64 = if s_26_11 { s_26_13 } else { s_26_15 };
        // D s_26_17: cast trunc s_26_16 -> u8
        let s_26_17: bool = ((s_26_16) != 0);
        // D s_26_18: cast zx s_26_17 -> bv
        let s_26_18: Bits = Bits::new(s_26_17 as u128, 1u16);
        // C s_26_19: const #1u : u8
        let s_26_19: bool = true;
        // C s_26_20: cast zx s_26_19 -> bv
        let s_26_20: Bits = Bits::new(s_26_19 as u128, 1u16);
        // D s_26_21: cmp-eq s_26_18 s_26_20
        let s_26_21: bool = ((s_26_18) == (s_26_20));
        // D s_26_22: write-var gs#308057 <= s_26_21
        fn_state.gs_308057 = s_26_21;
        // N s_26_23: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#308057:u8
        let s_27_0: bool = fn_state.gs_308057;
        // D s_27_1: write-var gs#308058 <= s_27_0
        fn_state.gs_308058 = s_27_0;
        // N s_27_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#308057 <= s_28_0
        fn_state.gs_308057 = s_28_0;
        // N s_28_2: jump b27
        return block_27(state, tracer, fn_state);
    }
}
