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
use execute_aarch32_instrs_VRINTA_asimd_Op_A_txt::*;
use FPDecodeRM::*;
use common::*;
pub fn decode_aarch32_instrs_VRINTA_asimd_A1enc_A_txt<T: Tracer>(
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
        elementsshadow_7937: i64,
        ga_365876: i64,
        gs_325416: bool,
        esize: i64,
        gs_325417: bool,
        esizeshadow_7936: i64,
        gs_325415: bool,
        d: i64,
        elements: i64,
        rounding: u32,
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
        // C s_0_0: const #2s : i
        let s_0_0: i128 = 2;
        // D s_0_1: read-var op:u8
        let s_0_1: u8 = fn_state.op;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 3u16);
        // C s_0_3: const #1u : u64
        let s_0_3: u64 = 1;
        // D s_0_4: bit-extract s_0_2 s_0_0 s_0_3
        let s_0_4: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_3).unwrap(),
        ));
        // D s_0_5: cast reint s_0_4 -> u8
        let s_0_5: bool = ((s_0_4.value()) != 0);
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // C s_0_7: const #0u : u64
        let s_0_7: u64 = 0;
        // D s_0_8: cast zx s_0_5 -> u64
        let s_0_8: u64 = (s_0_5 as u64);
        // C s_0_9: const #1u : u64
        let s_0_9: u64 = 1;
        // D s_0_10: and s_0_8 s_0_9
        let s_0_10: u64 = ((s_0_8) & (s_0_9));
        // D s_0_11: cmp-eq s_0_10 s_0_9
        let s_0_11: bool = ((s_0_10) == (s_0_9));
        // D s_0_12: lsl s_0_8 s_0_6
        let s_0_12: u64 = s_0_8 << s_0_6;
        // D s_0_13: or s_0_7 s_0_12
        let s_0_13: u64 = ((s_0_7) | (s_0_12));
        // D s_0_14: cmpl s_0_12
        let s_0_14: u64 = !s_0_12;
        // D s_0_15: and s_0_7 s_0_14
        let s_0_15: u64 = ((s_0_7) & (s_0_14));
        // D s_0_16: select s_0_11 s_0_13 s_0_15
        let s_0_16: u64 = if s_0_11 { s_0_13 } else { s_0_15 };
        // D s_0_17: cast trunc s_0_16 -> u8
        let s_0_17: bool = ((s_0_16) != 0);
        // C s_0_18: const #0s : i
        let s_0_18: i128 = 0;
        // D s_0_19: read-var op:u8
        let s_0_19: u8 = fn_state.op;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 3u16);
        // C s_0_21: const #1u : u64
        let s_0_21: u64 = 1;
        // D s_0_22: bit-extract s_0_20 s_0_18 s_0_21
        let s_0_22: Bits = (Bits::new(
            ((s_0_20) >> (s_0_18)).value(),
            u16::try_from(s_0_21).unwrap(),
        ));
        // D s_0_23: cast reint s_0_22 -> u8
        let s_0_23: bool = ((s_0_22.value()) != 0);
        // C s_0_24: const #0s : i
        let s_0_24: i128 = 0;
        // C s_0_25: const #0u : u64
        let s_0_25: u64 = 0;
        // D s_0_26: cast zx s_0_23 -> u64
        let s_0_26: u64 = (s_0_23 as u64);
        // C s_0_27: const #1u : u64
        let s_0_27: u64 = 1;
        // D s_0_28: and s_0_26 s_0_27
        let s_0_28: u64 = ((s_0_26) & (s_0_27));
        // D s_0_29: cmp-eq s_0_28 s_0_27
        let s_0_29: bool = ((s_0_28) == (s_0_27));
        // D s_0_30: lsl s_0_26 s_0_24
        let s_0_30: u64 = s_0_26 << s_0_24;
        // D s_0_31: or s_0_25 s_0_30
        let s_0_31: u64 = ((s_0_25) | (s_0_30));
        // D s_0_32: cmpl s_0_30
        let s_0_32: u64 = !s_0_30;
        // D s_0_33: and s_0_25 s_0_32
        let s_0_33: u64 = ((s_0_25) & (s_0_32));
        // D s_0_34: select s_0_29 s_0_31 s_0_33
        let s_0_34: u64 = if s_0_29 { s_0_31 } else { s_0_33 };
        // D s_0_35: cast trunc s_0_34 -> u8
        let s_0_35: bool = ((s_0_34) != 0);
        // D s_0_36: cast zx s_0_17 -> bv
        let s_0_36: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_37: cast zx s_0_35 -> bv
        let s_0_37: Bits = Bits::new(s_0_35 as u128, 1u16);
        // D s_0_38: cmp-ne s_0_36 s_0_37
        let s_0_38: bool = ((s_0_36) != (s_0_37));
        // N s_0_39: branch s_0_38 b23 b1
        if s_0_38 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Q:u8
        let s_1_0: bool = fn_state.Q;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b19 b2
        if s_1_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#325416 <= s_2_0
        fn_state.gs_325416 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#325416:u8
        let s_3_0: bool = fn_state.gs_325416;
        // N s_3_1: branch s_3_0 b18 b4
        if s_3_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var size:u8
        let s_4_0: u8 = fn_state.size;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #0u : u8
        let s_4_2: u8 = 0;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b17 b5
        if s_4_4 {
            return block_17(state, tracer, fn_state);
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
        // C s_5_2: const #3u : u8
        let s_5_2: u8 = 3;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: write-var gs#325417 <= s_5_4
        fn_state.gs_325417 = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#325417:u8
        let s_6_0: bool = fn_state.gs_325417;
        // N s_6_1: branch s_6_0 b16 b7
        if s_6_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i
        let s_7_0: i128 = 2;
        // D s_7_1: read-var op:u8
        let s_7_1: u8 = fn_state.op;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 3u16);
        // C s_7_3: const #1u : u64
        let s_7_3: u64 = 1;
        // D s_7_4: bit-extract s_7_2 s_7_0 s_7_3
        let s_7_4: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_3).unwrap(),
        ));
        // D s_7_5: cast reint s_7_4 -> u8
        let s_7_5: bool = ((s_7_4.value()) != 0);
        // C s_7_6: const #0s : i
        let s_7_6: i128 = 0;
        // C s_7_7: const #0u : u64
        let s_7_7: u64 = 0;
        // D s_7_8: cast zx s_7_5 -> u64
        let s_7_8: u64 = (s_7_5 as u64);
        // C s_7_9: const #1u : u64
        let s_7_9: u64 = 1;
        // D s_7_10: and s_7_8 s_7_9
        let s_7_10: u64 = ((s_7_8) & (s_7_9));
        // D s_7_11: cmp-eq s_7_10 s_7_9
        let s_7_11: bool = ((s_7_10) == (s_7_9));
        // D s_7_12: lsl s_7_8 s_7_6
        let s_7_12: u64 = s_7_8 << s_7_6;
        // D s_7_13: or s_7_7 s_7_12
        let s_7_13: u64 = ((s_7_7) | (s_7_12));
        // D s_7_14: cmpl s_7_12
        let s_7_14: u64 = !s_7_12;
        // D s_7_15: and s_7_7 s_7_14
        let s_7_15: u64 = ((s_7_7) & (s_7_14));
        // D s_7_16: select s_7_11 s_7_13 s_7_15
        let s_7_16: u64 = if s_7_11 { s_7_13 } else { s_7_15 };
        // D s_7_17: cast trunc s_7_16 -> u8
        let s_7_17: bool = ((s_7_16) != 0);
        // C s_7_18: const #1s : i
        let s_7_18: i128 = 1;
        // D s_7_19: read-var op:u8
        let s_7_19: u8 = fn_state.op;
        // D s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 3u16);
        // C s_7_21: const #1u : u64
        let s_7_21: u64 = 1;
        // D s_7_22: bit-extract s_7_20 s_7_18 s_7_21
        let s_7_22: Bits = (Bits::new(
            ((s_7_20) >> (s_7_18)).value(),
            u16::try_from(s_7_21).unwrap(),
        ));
        // D s_7_23: cast reint s_7_22 -> u8
        let s_7_23: bool = ((s_7_22.value()) != 0);
        // C s_7_24: const #0s : i
        let s_7_24: i128 = 0;
        // C s_7_25: const #0u : u64
        let s_7_25: u64 = 0;
        // D s_7_26: cast zx s_7_23 -> u64
        let s_7_26: u64 = (s_7_23 as u64);
        // C s_7_27: const #1u : u64
        let s_7_27: u64 = 1;
        // D s_7_28: and s_7_26 s_7_27
        let s_7_28: u64 = ((s_7_26) & (s_7_27));
        // D s_7_29: cmp-eq s_7_28 s_7_27
        let s_7_29: bool = ((s_7_28) == (s_7_27));
        // D s_7_30: lsl s_7_26 s_7_24
        let s_7_30: u64 = s_7_26 << s_7_24;
        // D s_7_31: or s_7_25 s_7_30
        let s_7_31: u64 = ((s_7_25) | (s_7_30));
        // D s_7_32: cmpl s_7_30
        let s_7_32: u64 = !s_7_30;
        // D s_7_33: and s_7_25 s_7_32
        let s_7_33: u64 = ((s_7_25) & (s_7_32));
        // D s_7_34: select s_7_29 s_7_31 s_7_33
        let s_7_34: u64 = if s_7_29 { s_7_31 } else { s_7_33 };
        // D s_7_35: cast trunc s_7_34 -> u8
        let s_7_35: bool = ((s_7_34) != 0);
        // D s_7_36: cast zx s_7_35 -> bv
        let s_7_36: Bits = Bits::new(s_7_35 as u128, 1u16);
        // D s_7_37: not s_7_36
        let s_7_37: Bits = !s_7_36;
        // D s_7_38: cast reint s_7_37 -> u8
        let s_7_38: bool = ((s_7_37.value()) != 0);
        // D s_7_39: cast zx s_7_17 -> bv
        let s_7_39: Bits = Bits::new(s_7_17 as u128, 1u16);
        // D s_7_40: cast zx s_7_38 -> bv
        let s_7_40: Bits = Bits::new(s_7_38 as u128, 1u16);
        // D s_7_41: cast reint s_7_39 -> u128
        let s_7_41: u128 = (s_7_39.value() as u128);
        // D s_7_42: size-of s_7_39
        let s_7_42: u16 = s_7_39.length();
        // D s_7_43: cast reint s_7_40 -> u128
        let s_7_43: u128 = (s_7_40.value() as u128);
        // D s_7_44: size-of s_7_40
        let s_7_44: u16 = s_7_40.length();
        // D s_7_45: lsl s_7_41 s_7_44
        let s_7_45: u128 = s_7_41 << s_7_44;
        // D s_7_46: or s_7_45 s_7_43
        let s_7_46: u128 = ((s_7_45) | (s_7_43));
        // D s_7_47: add s_7_42 s_7_44
        let s_7_47: u16 = (s_7_42 + s_7_44);
        // D s_7_48: create-bits s_7_46 s_7_47
        let s_7_48: Bits = Bits::new(s_7_46, s_7_47);
        // D s_7_49: cast reint s_7_48 -> u8
        let s_7_49: u8 = (s_7_48.value() as u8);
        // D s_7_50: call FPDecodeRM(s_7_49)
        let s_7_50: u32 = FPDecodeRM(state, tracer, s_7_49);
        // D s_7_51: write-var rounding <= s_7_50
        fn_state.rounding = s_7_50;
        // C s_7_52: const #16s : i64
        let s_7_52: i64 = 16;
        // D s_7_53: write-var esize <= s_7_52
        fn_state.esize = s_7_52;
        // C s_7_54: const #2s : i64
        let s_7_54: i64 = 2;
        // D s_7_55: write-var elements <= s_7_54
        fn_state.elements = s_7_54;
        // D s_7_56: read-var size:u8
        let s_7_56: u8 = fn_state.size;
        // D s_7_57: cast zx s_7_56 -> bv
        let s_7_57: Bits = Bits::new(s_7_56 as u128, 2u16);
        // C s_7_58: const #1u : u8
        let s_7_58: u8 = 1;
        // C s_7_59: cast zx s_7_58 -> bv
        let s_7_59: Bits = Bits::new(s_7_58 as u128, 2u16);
        // D s_7_60: cmp-eq s_7_57 s_7_59
        let s_7_60: bool = ((s_7_57) == (s_7_59));
        // D s_7_61: not s_7_60
        let s_7_61: bool = !s_7_60;
        // N s_7_62: branch s_7_61 b13 b8
        if s_7_61 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #16s : i64
        let s_8_0: i64 = 16;
        // D s_8_1: write-var esize <= s_8_0
        fn_state.esize = s_8_0;
        // C s_8_2: const #4s : i64
        let s_8_2: i64 = 4;
        // D s_8_3: write-var elements <= s_8_2
        fn_state.elements = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esize:i64
        let s_9_0: i64 = fn_state.esize;
        // D s_9_1: write-var esizeshadow#7936 <= s_9_0
        fn_state.esizeshadow_7936 = s_9_0;
        // D s_9_2: read-var elements:i64
        let s_9_2: i64 = fn_state.elements;
        // D s_9_3: write-var elementsshadow#7937 <= s_9_2
        fn_state.elementsshadow_7937 = s_9_2;
        // D s_9_4: read-var D:u8
        let s_9_4: bool = fn_state.D;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: read-var Vd:u8
        let s_9_6: u8 = fn_state.Vd;
        // D s_9_7: cast zx s_9_6 -> bv
        let s_9_7: Bits = Bits::new(s_9_6 as u128, 4u16);
        // D s_9_8: cast reint s_9_5 -> u128
        let s_9_8: u128 = (s_9_5.value() as u128);
        // D s_9_9: size-of s_9_5
        let s_9_9: u16 = s_9_5.length();
        // D s_9_10: cast reint s_9_7 -> u128
        let s_9_10: u128 = (s_9_7.value() as u128);
        // D s_9_11: size-of s_9_7
        let s_9_11: u16 = s_9_7.length();
        // D s_9_12: lsl s_9_8 s_9_11
        let s_9_12: u128 = s_9_8 << s_9_11;
        // D s_9_13: or s_9_12 s_9_10
        let s_9_13: u128 = ((s_9_12) | (s_9_10));
        // D s_9_14: add s_9_9 s_9_11
        let s_9_14: u16 = (s_9_9 + s_9_11);
        // D s_9_15: create-bits s_9_13 s_9_14
        let s_9_15: Bits = Bits::new(s_9_13, s_9_14);
        // D s_9_16: cast reint s_9_15 -> u8
        let s_9_16: u8 = (s_9_15.value() as u8);
        // D s_9_17: cast zx s_9_16 -> bv
        let s_9_17: Bits = Bits::new(s_9_16 as u128, 5u16);
        // D s_9_18: cast zx s_9_17 -> i
        let s_9_18: i128 = (s_9_17.value() as i128);
        // D s_9_19: cast reint s_9_18 -> i64
        let s_9_19: i64 = (s_9_18 as i64);
        // D s_9_20: write-var d <= s_9_19
        fn_state.d = s_9_19;
        // D s_9_21: read-var M:u8
        let s_9_21: bool = fn_state.M;
        // D s_9_22: cast zx s_9_21 -> bv
        let s_9_22: Bits = Bits::new(s_9_21 as u128, 1u16);
        // D s_9_23: read-var Vm:u8
        let s_9_23: u8 = fn_state.Vm;
        // D s_9_24: cast zx s_9_23 -> bv
        let s_9_24: Bits = Bits::new(s_9_23 as u128, 4u16);
        // D s_9_25: cast reint s_9_22 -> u128
        let s_9_25: u128 = (s_9_22.value() as u128);
        // D s_9_26: size-of s_9_22
        let s_9_26: u16 = s_9_22.length();
        // D s_9_27: cast reint s_9_24 -> u128
        let s_9_27: u128 = (s_9_24.value() as u128);
        // D s_9_28: size-of s_9_24
        let s_9_28: u16 = s_9_24.length();
        // D s_9_29: lsl s_9_25 s_9_28
        let s_9_29: u128 = s_9_25 << s_9_28;
        // D s_9_30: or s_9_29 s_9_27
        let s_9_30: u128 = ((s_9_29) | (s_9_27));
        // D s_9_31: add s_9_26 s_9_28
        let s_9_31: u16 = (s_9_26 + s_9_28);
        // D s_9_32: create-bits s_9_30 s_9_31
        let s_9_32: Bits = Bits::new(s_9_30, s_9_31);
        // D s_9_33: cast reint s_9_32 -> u8
        let s_9_33: u8 = (s_9_32.value() as u8);
        // D s_9_34: cast zx s_9_33 -> bv
        let s_9_34: Bits = Bits::new(s_9_33 as u128, 5u16);
        // D s_9_35: cast zx s_9_34 -> i
        let s_9_35: i128 = (s_9_34.value() as i128);
        // D s_9_36: cast reint s_9_35 -> i64
        let s_9_36: i64 = (s_9_35 as i64);
        // D s_9_37: write-var m <= s_9_36
        fn_state.m = s_9_36;
        // D s_9_38: read-var Q:u8
        let s_9_38: bool = fn_state.Q;
        // D s_9_39: cast zx s_9_38 -> bv
        let s_9_39: Bits = Bits::new(s_9_38 as u128, 1u16);
        // C s_9_40: const #0u : u8
        let s_9_40: bool = false;
        // C s_9_41: cast zx s_9_40 -> bv
        let s_9_41: Bits = Bits::new(s_9_40 as u128, 1u16);
        // D s_9_42: cmp-eq s_9_39 s_9_41
        let s_9_42: bool = ((s_9_39) == (s_9_41));
        // N s_9_43: branch s_9_42 b12 b10
        if s_9_42 {
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
        // D s_10_1: write-var ga#365876 <= s_10_0
        fn_state.ga_365876 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#365876:i64
        let s_11_0: i64 = fn_state.ga_365876;
        // D s_11_1: read-var esizeshadow#7936:i64
        let s_11_1: i64 = fn_state.esizeshadow_7936;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // D s_11_4: read-var d:i64
        let s_11_4: i64 = fn_state.d;
        // D s_11_5: read-var elementsshadow#7937:i64
        let s_11_5: i64 = fn_state.elementsshadow_7937;
        // C s_11_6: const #0u : u8
        let s_11_6: bool = false;
        // D s_11_7: read-var m:i64
        let s_11_7: i64 = fn_state.m;
        // D s_11_8: read-var rounding:u32
        let s_11_8: u32 = fn_state.rounding;
        // D s_11_9: call execute_aarch32_instrs_VRINTA_asimd_Op_A_txt(s_11_4, s_11_5, s_11_3, s_11_6, s_11_7, s_11_0, s_11_8)
        let s_11_9: () = execute_aarch32_instrs_VRINTA_asimd_Op_A_txt(
            state,
            tracer,
            s_11_4,
            s_11_5,
            s_11_3,
            s_11_6,
            s_11_7,
            s_11_0,
            s_11_8,
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
        // D s_12_1: write-var ga#365876 <= s_12_0
        fn_state.ga_365876 = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var size:u8
        let s_13_0: u8 = fn_state.size;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b15 b14
        if s_13_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #32s : i64
        let s_14_0: i64 = 32;
        // D s_14_1: write-var esize <= s_14_0
        fn_state.esize = s_14_0;
        // C s_14_2: const #2s : i64
        let s_14_2: i64 = 2;
        // D s_14_3: write-var elements <= s_14_2
        fn_state.elements = s_14_2;
        // N s_14_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#325417 <= s_17_0
        fn_state.gs_325417 = s_17_0;
        // N s_17_2: jump b6
        return block_6(state, tracer, fn_state);
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
        // D s_20_22: write-var gs#325415 <= s_20_21
        fn_state.gs_325415 = s_20_21;
        // N s_20_23: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#325415:u8
        let s_21_0: bool = fn_state.gs_325415;
        // D s_21_1: write-var gs#325416 <= s_21_0
        fn_state.gs_325416 = s_21_0;
        // N s_21_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#325415 <= s_22_0
        fn_state.gs_325415 = s_22_0;
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
}
