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
use common::*;
pub fn decode_aarch32_instrs_VCVT_is_A1enc_A_txt<T: Tracer>(
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
        gs_308019: bool,
        gs_308017: bool,
        d: i64,
        ga_352377: i64,
        is_unsigned: bool,
        elements: i64,
        esizeshadow_7408: i64,
        gs_308018: bool,
        elementsshadow_7409: i64,
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#308018 <= s_3_0
        fn_state.gs_308018 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#308018:u8
        let s_4_0: bool = fn_state.gs_308018;
        // N s_4_1: branch s_4_0 b19 b5
        if s_4_0 {
            return block_19(state, tracer, fn_state);
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
        // N s_5_5: branch s_5_4 b18 b6
        if s_5_4 {
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
        // D s_6_5: write-var gs#308019 <= s_6_4
        fn_state.gs_308019 = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#308019:u8
        let s_7_0: bool = fn_state.gs_308019;
        // N s_7_1: branch s_7_0 b17 b8
        if s_7_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1s : i
        let s_8_0: i128 = 1;
        // D s_8_1: read-var op:u8
        let s_8_1: u8 = fn_state.op;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 2u16);
        // C s_8_3: const #1u : u64
        let s_8_3: u64 = 1;
        // D s_8_4: bit-extract s_8_2 s_8_0 s_8_3
        let s_8_4: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_3).unwrap(),
        ));
        // D s_8_5: cast reint s_8_4 -> u8
        let s_8_5: bool = ((s_8_4.value()) != 0);
        // C s_8_6: const #0s : i
        let s_8_6: i128 = 0;
        // C s_8_7: const #0u : u64
        let s_8_7: u64 = 0;
        // D s_8_8: cast zx s_8_5 -> u64
        let s_8_8: u64 = (s_8_5 as u64);
        // C s_8_9: const #1u : u64
        let s_8_9: u64 = 1;
        // D s_8_10: and s_8_8 s_8_9
        let s_8_10: u64 = ((s_8_8) & (s_8_9));
        // D s_8_11: cmp-eq s_8_10 s_8_9
        let s_8_11: bool = ((s_8_10) == (s_8_9));
        // D s_8_12: lsl s_8_8 s_8_6
        let s_8_12: u64 = s_8_8 << s_8_6;
        // D s_8_13: or s_8_7 s_8_12
        let s_8_13: u64 = ((s_8_7) | (s_8_12));
        // D s_8_14: cmpl s_8_12
        let s_8_14: u64 = !s_8_12;
        // D s_8_15: and s_8_7 s_8_14
        let s_8_15: u64 = ((s_8_7) & (s_8_14));
        // D s_8_16: select s_8_11 s_8_13 s_8_15
        let s_8_16: u64 = if s_8_11 { s_8_13 } else { s_8_15 };
        // D s_8_17: cast trunc s_8_16 -> u8
        let s_8_17: bool = ((s_8_16) != 0);
        // D s_8_18: cast zx s_8_17 -> bv
        let s_8_18: Bits = Bits::new(s_8_17 as u128, 1u16);
        // C s_8_19: const #1u : u8
        let s_8_19: bool = true;
        // C s_8_20: cast zx s_8_19 -> bv
        let s_8_20: Bits = Bits::new(s_8_19 as u128, 1u16);
        // D s_8_21: cmp-eq s_8_18 s_8_20
        let s_8_21: bool = ((s_8_18) == (s_8_20));
        // D s_8_22: write-var to_integer <= s_8_21
        fn_state.to_integer = s_8_21;
        // C s_8_23: const #0s : i
        let s_8_23: i128 = 0;
        // D s_8_24: read-var op:u8
        let s_8_24: u8 = fn_state.op;
        // D s_8_25: cast zx s_8_24 -> bv
        let s_8_25: Bits = Bits::new(s_8_24 as u128, 2u16);
        // C s_8_26: const #1u : u64
        let s_8_26: u64 = 1;
        // D s_8_27: bit-extract s_8_25 s_8_23 s_8_26
        let s_8_27: Bits = (Bits::new(
            ((s_8_25) >> (s_8_23)).value(),
            u16::try_from(s_8_26).unwrap(),
        ));
        // D s_8_28: cast reint s_8_27 -> u8
        let s_8_28: bool = ((s_8_27.value()) != 0);
        // C s_8_29: const #0s : i
        let s_8_29: i128 = 0;
        // C s_8_30: const #0u : u64
        let s_8_30: u64 = 0;
        // D s_8_31: cast zx s_8_28 -> u64
        let s_8_31: u64 = (s_8_28 as u64);
        // C s_8_32: const #1u : u64
        let s_8_32: u64 = 1;
        // D s_8_33: and s_8_31 s_8_32
        let s_8_33: u64 = ((s_8_31) & (s_8_32));
        // D s_8_34: cmp-eq s_8_33 s_8_32
        let s_8_34: bool = ((s_8_33) == (s_8_32));
        // D s_8_35: lsl s_8_31 s_8_29
        let s_8_35: u64 = s_8_31 << s_8_29;
        // D s_8_36: or s_8_30 s_8_35
        let s_8_36: u64 = ((s_8_30) | (s_8_35));
        // D s_8_37: cmpl s_8_35
        let s_8_37: u64 = !s_8_35;
        // D s_8_38: and s_8_30 s_8_37
        let s_8_38: u64 = ((s_8_30) & (s_8_37));
        // D s_8_39: select s_8_34 s_8_36 s_8_38
        let s_8_39: u64 = if s_8_34 { s_8_36 } else { s_8_38 };
        // D s_8_40: cast trunc s_8_39 -> u8
        let s_8_40: bool = ((s_8_39) != 0);
        // D s_8_41: cast zx s_8_40 -> bv
        let s_8_41: Bits = Bits::new(s_8_40 as u128, 1u16);
        // C s_8_42: const #1u : u8
        let s_8_42: bool = true;
        // C s_8_43: cast zx s_8_42 -> bv
        let s_8_43: Bits = Bits::new(s_8_42 as u128, 1u16);
        // D s_8_44: cmp-eq s_8_41 s_8_43
        let s_8_44: bool = ((s_8_41) == (s_8_43));
        // D s_8_45: write-var is_unsigned <= s_8_44
        fn_state.is_unsigned = s_8_44;
        // C s_8_46: const #16s : i64
        let s_8_46: i64 = 16;
        // D s_8_47: write-var esize <= s_8_46
        fn_state.esize = s_8_46;
        // C s_8_48: const #2s : i64
        let s_8_48: i64 = 2;
        // D s_8_49: write-var elements <= s_8_48
        fn_state.elements = s_8_48;
        // D s_8_50: read-var size:u8
        let s_8_50: u8 = fn_state.size;
        // D s_8_51: cast zx s_8_50 -> bv
        let s_8_51: Bits = Bits::new(s_8_50 as u128, 2u16);
        // C s_8_52: const #1u : u8
        let s_8_52: u8 = 1;
        // C s_8_53: cast zx s_8_52 -> bv
        let s_8_53: Bits = Bits::new(s_8_52 as u128, 2u16);
        // D s_8_54: cmp-eq s_8_51 s_8_53
        let s_8_54: bool = ((s_8_51) == (s_8_53));
        // D s_8_55: not s_8_54
        let s_8_55: bool = !s_8_54;
        // N s_8_56: branch s_8_55 b14 b9
        if s_8_55 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #16s : i64
        let s_9_0: i64 = 16;
        // D s_9_1: write-var esize <= s_9_0
        fn_state.esize = s_9_0;
        // C s_9_2: const #4s : i64
        let s_9_2: i64 = 4;
        // D s_9_3: write-var elements <= s_9_2
        fn_state.elements = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esize:i64
        let s_10_0: i64 = fn_state.esize;
        // D s_10_1: write-var esizeshadow#7408 <= s_10_0
        fn_state.esizeshadow_7408 = s_10_0;
        // D s_10_2: read-var elements:i64
        let s_10_2: i64 = fn_state.elements;
        // D s_10_3: write-var elementsshadow#7409 <= s_10_2
        fn_state.elementsshadow_7409 = s_10_2;
        // D s_10_4: read-var D:u8
        let s_10_4: bool = fn_state.D;
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: read-var Vd:u8
        let s_10_6: u8 = fn_state.Vd;
        // D s_10_7: cast zx s_10_6 -> bv
        let s_10_7: Bits = Bits::new(s_10_6 as u128, 4u16);
        // D s_10_8: cast reint s_10_5 -> u128
        let s_10_8: u128 = (s_10_5.value() as u128);
        // D s_10_9: size-of s_10_5
        let s_10_9: u16 = s_10_5.length();
        // D s_10_10: cast reint s_10_7 -> u128
        let s_10_10: u128 = (s_10_7.value() as u128);
        // D s_10_11: size-of s_10_7
        let s_10_11: u16 = s_10_7.length();
        // D s_10_12: lsl s_10_8 s_10_11
        let s_10_12: u128 = s_10_8 << s_10_11;
        // D s_10_13: or s_10_12 s_10_10
        let s_10_13: u128 = ((s_10_12) | (s_10_10));
        // D s_10_14: add s_10_9 s_10_11
        let s_10_14: u16 = (s_10_9 + s_10_11);
        // D s_10_15: create-bits s_10_13 s_10_14
        let s_10_15: Bits = Bits::new(s_10_13, s_10_14);
        // D s_10_16: cast reint s_10_15 -> u8
        let s_10_16: u8 = (s_10_15.value() as u8);
        // D s_10_17: cast zx s_10_16 -> bv
        let s_10_17: Bits = Bits::new(s_10_16 as u128, 5u16);
        // D s_10_18: cast zx s_10_17 -> i
        let s_10_18: i128 = (s_10_17.value() as i128);
        // D s_10_19: cast reint s_10_18 -> i64
        let s_10_19: i64 = (s_10_18 as i64);
        // D s_10_20: write-var d <= s_10_19
        fn_state.d = s_10_19;
        // D s_10_21: read-var M:u8
        let s_10_21: bool = fn_state.M;
        // D s_10_22: cast zx s_10_21 -> bv
        let s_10_22: Bits = Bits::new(s_10_21 as u128, 1u16);
        // D s_10_23: read-var Vm:u8
        let s_10_23: u8 = fn_state.Vm;
        // D s_10_24: cast zx s_10_23 -> bv
        let s_10_24: Bits = Bits::new(s_10_23 as u128, 4u16);
        // D s_10_25: cast reint s_10_22 -> u128
        let s_10_25: u128 = (s_10_22.value() as u128);
        // D s_10_26: size-of s_10_22
        let s_10_26: u16 = s_10_22.length();
        // D s_10_27: cast reint s_10_24 -> u128
        let s_10_27: u128 = (s_10_24.value() as u128);
        // D s_10_28: size-of s_10_24
        let s_10_28: u16 = s_10_24.length();
        // D s_10_29: lsl s_10_25 s_10_28
        let s_10_29: u128 = s_10_25 << s_10_28;
        // D s_10_30: or s_10_29 s_10_27
        let s_10_30: u128 = ((s_10_29) | (s_10_27));
        // D s_10_31: add s_10_26 s_10_28
        let s_10_31: u16 = (s_10_26 + s_10_28);
        // D s_10_32: create-bits s_10_30 s_10_31
        let s_10_32: Bits = Bits::new(s_10_30, s_10_31);
        // D s_10_33: cast reint s_10_32 -> u8
        let s_10_33: u8 = (s_10_32.value() as u8);
        // D s_10_34: cast zx s_10_33 -> bv
        let s_10_34: Bits = Bits::new(s_10_33 as u128, 5u16);
        // D s_10_35: cast zx s_10_34 -> i
        let s_10_35: i128 = (s_10_34.value() as i128);
        // D s_10_36: cast reint s_10_35 -> i64
        let s_10_36: i64 = (s_10_35 as i64);
        // D s_10_37: write-var m <= s_10_36
        fn_state.m = s_10_36;
        // D s_10_38: read-var Q:u8
        let s_10_38: bool = fn_state.Q;
        // D s_10_39: cast zx s_10_38 -> bv
        let s_10_39: Bits = Bits::new(s_10_38 as u128, 1u16);
        // C s_10_40: const #0u : u8
        let s_10_40: bool = false;
        // C s_10_41: cast zx s_10_40 -> bv
        let s_10_41: Bits = Bits::new(s_10_40 as u128, 1u16);
        // D s_10_42: cmp-eq s_10_39 s_10_41
        let s_10_42: bool = ((s_10_39) == (s_10_41));
        // N s_10_43: branch s_10_42 b13 b11
        if s_10_42 {
            return block_13(state, tracer, fn_state);
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
        // D s_11_1: write-var ga#352377 <= s_11_0
        fn_state.ga_352377 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#352377:i64
        let s_12_0: i64 = fn_state.ga_352377;
        // D s_12_1: read-var esizeshadow#7408:i64
        let s_12_1: i64 = fn_state.esizeshadow_7408;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: read-var d:i64
        let s_12_4: i64 = fn_state.d;
        // D s_12_5: read-var elementsshadow#7409:i64
        let s_12_5: i64 = fn_state.elementsshadow_7409;
        // D s_12_6: read-var m:i64
        let s_12_6: i64 = fn_state.m;
        // D s_12_7: read-var to_integer:u8
        let s_12_7: bool = fn_state.to_integer;
        // D s_12_8: read-var is_unsigned:u8
        let s_12_8: bool = fn_state.is_unsigned;
        // D s_12_9: call execute_aarch32_instrs_VCVT_is_Op_A_txt(s_12_4, s_12_5, s_12_3, s_12_6, s_12_0, s_12_7, s_12_8)
        let s_12_9: () = execute_aarch32_instrs_VCVT_is_Op_A_txt(
            state,
            tracer,
            s_12_4,
            s_12_5,
            s_12_3,
            s_12_6,
            s_12_0,
            s_12_7,
            s_12_8,
        );
        // N s_12_10: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1s : i64
        let s_13_0: i64 = 1;
        // D s_13_1: write-var ga#352377 <= s_13_0
        fn_state.ga_352377 = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var size:u8
        let s_14_0: u8 = fn_state.size;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 2u16);
        // C s_14_2: const #2u : u8
        let s_14_2: u8 = 2;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 2u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: not s_14_4
        let s_14_5: bool = !s_14_4;
        // N s_14_6: branch s_14_5 b16 b15
        if s_14_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #32s : i64
        let s_15_0: i64 = 32;
        // D s_15_1: write-var esize <= s_15_0
        fn_state.esize = s_15_0;
        // C s_15_2: const #2s : i64
        let s_15_2: i64 = 2;
        // D s_15_3: write-var elements <= s_15_2
        fn_state.elements = s_15_2;
        // N s_15_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#308019 <= s_18_0
        fn_state.gs_308019 = s_18_0;
        // N s_18_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0s : i
        let s_20_0: i128 = 0;
        // D s_20_1: read-var Vd:u8
        let s_20_1: u8 = fn_state.Vd;
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
        // N s_20_22: branch s_20_21 b23 b21
        if s_20_21 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0s : i
        let s_21_0: i128 = 0;
        // D s_21_1: read-var Vm:u8
        let s_21_1: u8 = fn_state.Vm;
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 4u16);
        // C s_21_3: const #1u : u64
        let s_21_3: u64 = 1;
        // D s_21_4: bit-extract s_21_2 s_21_0 s_21_3
        let s_21_4: Bits = (Bits::new(
            ((s_21_2) >> (s_21_0)).value(),
            u16::try_from(s_21_3).unwrap(),
        ));
        // D s_21_5: cast reint s_21_4 -> u8
        let s_21_5: bool = ((s_21_4.value()) != 0);
        // C s_21_6: const #0s : i
        let s_21_6: i128 = 0;
        // C s_21_7: const #0u : u64
        let s_21_7: u64 = 0;
        // D s_21_8: cast zx s_21_5 -> u64
        let s_21_8: u64 = (s_21_5 as u64);
        // C s_21_9: const #1u : u64
        let s_21_9: u64 = 1;
        // D s_21_10: and s_21_8 s_21_9
        let s_21_10: u64 = ((s_21_8) & (s_21_9));
        // D s_21_11: cmp-eq s_21_10 s_21_9
        let s_21_11: bool = ((s_21_10) == (s_21_9));
        // D s_21_12: lsl s_21_8 s_21_6
        let s_21_12: u64 = s_21_8 << s_21_6;
        // D s_21_13: or s_21_7 s_21_12
        let s_21_13: u64 = ((s_21_7) | (s_21_12));
        // D s_21_14: cmpl s_21_12
        let s_21_14: u64 = !s_21_12;
        // D s_21_15: and s_21_7 s_21_14
        let s_21_15: u64 = ((s_21_7) & (s_21_14));
        // D s_21_16: select s_21_11 s_21_13 s_21_15
        let s_21_16: u64 = if s_21_11 { s_21_13 } else { s_21_15 };
        // D s_21_17: cast trunc s_21_16 -> u8
        let s_21_17: bool = ((s_21_16) != 0);
        // D s_21_18: cast zx s_21_17 -> bv
        let s_21_18: Bits = Bits::new(s_21_17 as u128, 1u16);
        // C s_21_19: const #1u : u8
        let s_21_19: bool = true;
        // C s_21_20: cast zx s_21_19 -> bv
        let s_21_20: Bits = Bits::new(s_21_19 as u128, 1u16);
        // D s_21_21: cmp-eq s_21_18 s_21_20
        let s_21_21: bool = ((s_21_18) == (s_21_20));
        // D s_21_22: write-var gs#308017 <= s_21_21
        fn_state.gs_308017 = s_21_21;
        // N s_21_23: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#308017:u8
        let s_22_0: bool = fn_state.gs_308017;
        // D s_22_1: write-var gs#308018 <= s_22_0
        fn_state.gs_308018 = s_22_0;
        // N s_22_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#308017 <= s_23_0
        fn_state.gs_308017 = s_23_0;
        // N s_23_2: jump b22
        return block_22(state, tracer, fn_state);
    }
}
