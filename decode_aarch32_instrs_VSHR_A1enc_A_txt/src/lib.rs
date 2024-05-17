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
use execute_aarch32_instrs_VSHR_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VSHR_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    imm6: u8,
    Vd: u8,
    L: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        elementsshadow_7813: i64,
        m: i64,
        gs_319119: bool,
        esize: i64,
        ga_360874: i64,
        gs_319118: bool,
        shift_amountshadow_7811: i128,
        ga_360860: u8,
        regs: i64,
        d: i64,
        elements: i64,
        is_unsigned: bool,
        esizeshadow_7812: i64,
        shift_amount: i128,
        gs_319109: bool,
        U: bool,
        D: bool,
        imm6: u8,
        Vd: u8,
        L: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        imm6,
        Vd,
        L,
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
        // D s_2_0: read-var L:u8
        let s_2_0: bool = fn_state.L;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_2: read-var imm6:u8
        let s_2_2: u8 = fn_state.imm6;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 6u16);
        // D s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: u8 = (s_2_11.value() as u8);
        // C s_2_13: const #3s : i
        let s_2_13: i128 = 3;
        // D s_2_14: cast zx s_2_12 -> bv
        let s_2_14: Bits = Bits::new(s_2_12 as u128, 7u16);
        // C s_2_15: const #1s : i64
        let s_2_15: i64 = 1;
        // C s_2_16: cast zx s_2_15 -> i
        let s_2_16: i128 = (i128::try_from(s_2_15).unwrap());
        // C s_2_17: const #3s : i
        let s_2_17: i128 = 3;
        // C s_2_18: add s_2_17 s_2_16
        let s_2_18: i128 = (s_2_17 + s_2_16);
        // D s_2_19: bit-extract s_2_14 s_2_13 s_2_18
        let s_2_19: Bits = (Bits::new(
            ((s_2_14) >> (s_2_13)).value(),
            u16::try_from(s_2_18).unwrap(),
        ));
        // D s_2_20: cast reint s_2_19 -> u8
        let s_2_20: u8 = (s_2_19.value() as u8);
        // D s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 4u16);
        // C s_2_22: const #0u : u8
        let s_2_22: u8 = 0;
        // C s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 4u16);
        // D s_2_24: cmp-eq s_2_21 s_2_23
        let s_2_24: bool = ((s_2_21) == (s_2_23));
        // D s_2_25: not s_2_24
        let s_2_25: bool = !s_2_24;
        // N s_2_26: branch s_2_25 b35 b3
        if s_2_25 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var gs#319109 <= s_3_0
        fn_state.gs_319109 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#319109:u8
        let s_4_0: bool = fn_state.gs_319109;
        // N s_4_1: branch s_4_0 b34 b5
        if s_4_0 {
            return block_34(state, tracer, fn_state);
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
        // C s_5_2: const #1u : u8
        let s_5_2: bool = true;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b30 b6
        if s_5_4 {
            return block_30(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#319119 <= s_6_0
        fn_state.gs_319119 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#319119:u8
        let s_7_0: bool = fn_state.gs_319119;
        // N s_7_1: branch s_7_0 b29 b8
        if s_7_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #8s : i64
        let s_8_0: i64 = 8;
        // D s_8_1: write-var esize <= s_8_0
        fn_state.esize = s_8_0;
        // C s_8_2: const #1s : i64
        let s_8_2: i64 = 1;
        // D s_8_3: write-var elements <= s_8_2
        fn_state.elements = s_8_2;
        // D s_8_4: read-var L:u8
        let s_8_4: bool = fn_state.L;
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // D s_8_6: read-var imm6:u8
        let s_8_6: u8 = fn_state.imm6;
        // D s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 6u16);
        // D s_8_8: cast reint s_8_5 -> u128
        let s_8_8: u128 = (s_8_5.value() as u128);
        // D s_8_9: size-of s_8_5
        let s_8_9: u16 = s_8_5.length();
        // D s_8_10: cast reint s_8_7 -> u128
        let s_8_10: u128 = (s_8_7.value() as u128);
        // D s_8_11: size-of s_8_7
        let s_8_11: u16 = s_8_7.length();
        // D s_8_12: lsl s_8_8 s_8_11
        let s_8_12: u128 = s_8_8 << s_8_11;
        // D s_8_13: or s_8_12 s_8_10
        let s_8_13: u128 = ((s_8_12) | (s_8_10));
        // D s_8_14: add s_8_9 s_8_11
        let s_8_14: u16 = (s_8_9 + s_8_11);
        // D s_8_15: create-bits s_8_13 s_8_14
        let s_8_15: Bits = Bits::new(s_8_13, s_8_14);
        // D s_8_16: cast reint s_8_15 -> u8
        let s_8_16: u8 = (s_8_15.value() as u8);
        // D s_8_17: write-var ga#360860 <= s_8_16
        fn_state.ga_360860 = s_8_16;
        // D s_8_18: read-var ga#360860:u8
        let s_8_18: u8 = fn_state.ga_360860;
        // C s_8_19: const #3s : i
        let s_8_19: i128 = 3;
        // D s_8_20: cast zx s_8_18 -> bv
        let s_8_20: Bits = Bits::new(s_8_18 as u128, 7u16);
        // C s_8_21: const #1s : i64
        let s_8_21: i64 = 1;
        // C s_8_22: cast zx s_8_21 -> i
        let s_8_22: i128 = (i128::try_from(s_8_21).unwrap());
        // C s_8_23: const #3s : i
        let s_8_23: i128 = 3;
        // C s_8_24: add s_8_23 s_8_22
        let s_8_24: i128 = (s_8_23 + s_8_22);
        // D s_8_25: bit-extract s_8_20 s_8_19 s_8_24
        let s_8_25: Bits = (Bits::new(
            ((s_8_20) >> (s_8_19)).value(),
            u16::try_from(s_8_24).unwrap(),
        ));
        // D s_8_26: cast reint s_8_25 -> u8
        let s_8_26: u8 = (s_8_25.value() as u8);
        // D s_8_27: cast zx s_8_26 -> bv
        let s_8_27: Bits = Bits::new(s_8_26 as u128, 4u16);
        // C s_8_28: const #1u : u8
        let s_8_28: u8 = 1;
        // C s_8_29: cast zx s_8_28 -> bv
        let s_8_29: Bits = Bits::new(s_8_28 as u128, 4u16);
        // D s_8_30: cmp-eq s_8_27 s_8_29
        let s_8_30: bool = ((s_8_27) == (s_8_29));
        // D s_8_31: not s_8_30
        let s_8_31: bool = !s_8_30;
        // N s_8_32: branch s_8_31 b22 b9
        if s_8_31 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #8s : i64
        let s_9_0: i64 = 8;
        // D s_9_1: write-var esize <= s_9_0
        fn_state.esize = s_9_0;
        // C s_9_2: const #8s : i64
        let s_9_2: i64 = 8;
        // D s_9_3: write-var elements <= s_9_2
        fn_state.elements = s_9_2;
        // D s_9_4: read-var imm6:u8
        let s_9_4: u8 = fn_state.imm6;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 6u16);
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (s_9_5.value() as i128);
        // D s_9_7: cast reint s_9_6 -> i64
        let s_9_7: i64 = (s_9_6 as i64);
        // C s_9_8: const #16s : i
        let s_9_8: i128 = 16;
        // D s_9_9: cast zx s_9_7 -> i
        let s_9_9: i128 = (i128::try_from(s_9_7).unwrap());
        // D s_9_10: sub s_9_8 s_9_9
        let s_9_10: i128 = ((s_9_8) - (s_9_9));
        // D s_9_11: write-var shift_amount <= s_9_10
        fn_state.shift_amount = s_9_10;
        // N s_9_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var shift_amount:i
        let s_10_0: i128 = fn_state.shift_amount;
        // D s_10_1: write-var shift_amountshadow#7811 <= s_10_0
        fn_state.shift_amountshadow_7811 = s_10_0;
        // D s_10_2: read-var esize:i64
        let s_10_2: i64 = fn_state.esize;
        // D s_10_3: write-var esizeshadow#7812 <= s_10_2
        fn_state.esizeshadow_7812 = s_10_2;
        // D s_10_4: read-var elements:i64
        let s_10_4: i64 = fn_state.elements;
        // D s_10_5: write-var elementsshadow#7813 <= s_10_4
        fn_state.elementsshadow_7813 = s_10_4;
        // D s_10_6: read-var U:u8
        let s_10_6: bool = fn_state.U;
        // D s_10_7: cast zx s_10_6 -> bv
        let s_10_7: Bits = Bits::new(s_10_6 as u128, 1u16);
        // C s_10_8: const #1u : u8
        let s_10_8: bool = true;
        // C s_10_9: cast zx s_10_8 -> bv
        let s_10_9: Bits = Bits::new(s_10_8 as u128, 1u16);
        // D s_10_10: cmp-eq s_10_7 s_10_9
        let s_10_10: bool = ((s_10_7) == (s_10_9));
        // D s_10_11: write-var is_unsigned <= s_10_10
        fn_state.is_unsigned = s_10_10;
        // D s_10_12: read-var D:u8
        let s_10_12: bool = fn_state.D;
        // D s_10_13: cast zx s_10_12 -> bv
        let s_10_13: Bits = Bits::new(s_10_12 as u128, 1u16);
        // D s_10_14: read-var Vd:u8
        let s_10_14: u8 = fn_state.Vd;
        // D s_10_15: cast zx s_10_14 -> bv
        let s_10_15: Bits = Bits::new(s_10_14 as u128, 4u16);
        // D s_10_16: cast reint s_10_13 -> u128
        let s_10_16: u128 = (s_10_13.value() as u128);
        // D s_10_17: size-of s_10_13
        let s_10_17: u16 = s_10_13.length();
        // D s_10_18: cast reint s_10_15 -> u128
        let s_10_18: u128 = (s_10_15.value() as u128);
        // D s_10_19: size-of s_10_15
        let s_10_19: u16 = s_10_15.length();
        // D s_10_20: lsl s_10_16 s_10_19
        let s_10_20: u128 = s_10_16 << s_10_19;
        // D s_10_21: or s_10_20 s_10_18
        let s_10_21: u128 = ((s_10_20) | (s_10_18));
        // D s_10_22: add s_10_17 s_10_19
        let s_10_22: u16 = (s_10_17 + s_10_19);
        // D s_10_23: create-bits s_10_21 s_10_22
        let s_10_23: Bits = Bits::new(s_10_21, s_10_22);
        // D s_10_24: cast reint s_10_23 -> u8
        let s_10_24: u8 = (s_10_23.value() as u8);
        // D s_10_25: cast zx s_10_24 -> bv
        let s_10_25: Bits = Bits::new(s_10_24 as u128, 5u16);
        // D s_10_26: cast zx s_10_25 -> i
        let s_10_26: i128 = (s_10_25.value() as i128);
        // D s_10_27: cast reint s_10_26 -> i64
        let s_10_27: i64 = (s_10_26 as i64);
        // D s_10_28: write-var d <= s_10_27
        fn_state.d = s_10_27;
        // D s_10_29: read-var M:u8
        let s_10_29: bool = fn_state.M;
        // D s_10_30: cast zx s_10_29 -> bv
        let s_10_30: Bits = Bits::new(s_10_29 as u128, 1u16);
        // D s_10_31: read-var Vm:u8
        let s_10_31: u8 = fn_state.Vm;
        // D s_10_32: cast zx s_10_31 -> bv
        let s_10_32: Bits = Bits::new(s_10_31 as u128, 4u16);
        // D s_10_33: cast reint s_10_30 -> u128
        let s_10_33: u128 = (s_10_30.value() as u128);
        // D s_10_34: size-of s_10_30
        let s_10_34: u16 = s_10_30.length();
        // D s_10_35: cast reint s_10_32 -> u128
        let s_10_35: u128 = (s_10_32.value() as u128);
        // D s_10_36: size-of s_10_32
        let s_10_36: u16 = s_10_32.length();
        // D s_10_37: lsl s_10_33 s_10_36
        let s_10_37: u128 = s_10_33 << s_10_36;
        // D s_10_38: or s_10_37 s_10_35
        let s_10_38: u128 = ((s_10_37) | (s_10_35));
        // D s_10_39: add s_10_34 s_10_36
        let s_10_39: u16 = (s_10_34 + s_10_36);
        // D s_10_40: create-bits s_10_38 s_10_39
        let s_10_40: Bits = Bits::new(s_10_38, s_10_39);
        // D s_10_41: cast reint s_10_40 -> u8
        let s_10_41: u8 = (s_10_40.value() as u8);
        // D s_10_42: cast zx s_10_41 -> bv
        let s_10_42: Bits = Bits::new(s_10_41 as u128, 5u16);
        // D s_10_43: cast zx s_10_42 -> i
        let s_10_43: i128 = (s_10_42.value() as i128);
        // D s_10_44: cast reint s_10_43 -> i64
        let s_10_44: i64 = (s_10_43 as i64);
        // D s_10_45: write-var m <= s_10_44
        fn_state.m = s_10_44;
        // D s_10_46: read-var Q:u8
        let s_10_46: bool = fn_state.Q;
        // D s_10_47: cast zx s_10_46 -> bv
        let s_10_47: Bits = Bits::new(s_10_46 as u128, 1u16);
        // C s_10_48: const #0u : u8
        let s_10_48: bool = false;
        // C s_10_49: cast zx s_10_48 -> bv
        let s_10_49: Bits = Bits::new(s_10_48 as u128, 1u16);
        // D s_10_50: cmp-eq s_10_47 s_10_49
        let s_10_50: bool = ((s_10_47) == (s_10_49));
        // N s_10_51: branch s_10_50 b21 b11
        if s_10_50 {
            return block_21(state, tracer, fn_state);
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
        // D s_11_1: write-var ga#360874 <= s_11_0
        fn_state.ga_360874 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#360874:i64
        let s_12_0: i64 = fn_state.ga_360874;
        // D s_12_1: write-var regs <= s_12_0
        fn_state.regs = s_12_0;
        // D s_12_2: read-var esizeshadow#7812:i64
        let s_12_2: i64 = fn_state.esizeshadow_7812;
        // C s_12_3: const #8s : i
        let s_12_3: i128 = 8;
        // D s_12_4: cast zx s_12_2 -> i
        let s_12_4: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_5: cmp-eq s_12_4 s_12_3
        let s_12_5: bool = ((s_12_4) == (s_12_3));
        // D s_12_6: not s_12_5
        let s_12_6: bool = !s_12_5;
        // N s_12_7: branch s_12_6 b14 b13
        if s_12_6 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #8s : i64
        let s_13_0: i64 = 8;
        // D s_13_1: read-var d:i64
        let s_13_1: i64 = fn_state.d;
        // D s_13_2: read-var elementsshadow#7813:i64
        let s_13_2: i64 = fn_state.elementsshadow_7813;
        // D s_13_3: read-var m:i64
        let s_13_3: i64 = fn_state.m;
        // D s_13_4: read-var regs:i64
        let s_13_4: i64 = fn_state.regs;
        // D s_13_5: read-var shift_amountshadow#7811:i
        let s_13_5: i128 = fn_state.shift_amountshadow_7811;
        // D s_13_6: read-var is_unsigned:u8
        let s_13_6: bool = fn_state.is_unsigned;
        // D s_13_7: call execute_aarch32_instrs_VSHR_Op_A_txt(s_13_1, s_13_2, s_13_0, s_13_3, s_13_4, s_13_5, s_13_6)
        let s_13_7: () = execute_aarch32_instrs_VSHR_Op_A_txt(
            state,
            tracer,
            s_13_1,
            s_13_2,
            s_13_0,
            s_13_3,
            s_13_4,
            s_13_5,
            s_13_6,
        );
        // N s_13_8: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var esizeshadow#7812:i64
        let s_14_0: i64 = fn_state.esizeshadow_7812;
        // C s_14_1: const #16s : i
        let s_14_1: i128 = 16;
        // D s_14_2: cast zx s_14_0 -> i
        let s_14_2: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_3: cmp-eq s_14_2 s_14_1
        let s_14_3: bool = ((s_14_2) == (s_14_1));
        // D s_14_4: not s_14_3
        let s_14_4: bool = !s_14_3;
        // N s_14_5: branch s_14_4 b16 b15
        if s_14_4 {
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
        // C s_15_0: const #16s : i64
        let s_15_0: i64 = 16;
        // D s_15_1: read-var d:i64
        let s_15_1: i64 = fn_state.d;
        // D s_15_2: read-var elementsshadow#7813:i64
        let s_15_2: i64 = fn_state.elementsshadow_7813;
        // D s_15_3: read-var m:i64
        let s_15_3: i64 = fn_state.m;
        // D s_15_4: read-var regs:i64
        let s_15_4: i64 = fn_state.regs;
        // D s_15_5: read-var shift_amountshadow#7811:i
        let s_15_5: i128 = fn_state.shift_amountshadow_7811;
        // D s_15_6: read-var is_unsigned:u8
        let s_15_6: bool = fn_state.is_unsigned;
        // D s_15_7: call execute_aarch32_instrs_VSHR_Op_A_txt(s_15_1, s_15_2, s_15_0, s_15_3, s_15_4, s_15_5, s_15_6)
        let s_15_7: () = execute_aarch32_instrs_VSHR_Op_A_txt(
            state,
            tracer,
            s_15_1,
            s_15_2,
            s_15_0,
            s_15_3,
            s_15_4,
            s_15_5,
            s_15_6,
        );
        // N s_15_8: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var esizeshadow#7812:i64
        let s_16_0: i64 = fn_state.esizeshadow_7812;
        // C s_16_1: const #32s : i
        let s_16_1: i128 = 32;
        // D s_16_2: cast zx s_16_0 -> i
        let s_16_2: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_3: cmp-eq s_16_2 s_16_1
        let s_16_3: bool = ((s_16_2) == (s_16_1));
        // D s_16_4: not s_16_3
        let s_16_4: bool = !s_16_3;
        // N s_16_5: branch s_16_4 b18 b17
        if s_16_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #32s : i64
        let s_17_0: i64 = 32;
        // D s_17_1: read-var d:i64
        let s_17_1: i64 = fn_state.d;
        // D s_17_2: read-var elementsshadow#7813:i64
        let s_17_2: i64 = fn_state.elementsshadow_7813;
        // D s_17_3: read-var m:i64
        let s_17_3: i64 = fn_state.m;
        // D s_17_4: read-var regs:i64
        let s_17_4: i64 = fn_state.regs;
        // D s_17_5: read-var shift_amountshadow#7811:i
        let s_17_5: i128 = fn_state.shift_amountshadow_7811;
        // D s_17_6: read-var is_unsigned:u8
        let s_17_6: bool = fn_state.is_unsigned;
        // D s_17_7: call execute_aarch32_instrs_VSHR_Op_A_txt(s_17_1, s_17_2, s_17_0, s_17_3, s_17_4, s_17_5, s_17_6)
        let s_17_7: () = execute_aarch32_instrs_VSHR_Op_A_txt(
            state,
            tracer,
            s_17_1,
            s_17_2,
            s_17_0,
            s_17_3,
            s_17_4,
            s_17_5,
            s_17_6,
        );
        // N s_17_8: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var esizeshadow#7812:i64
        let s_18_0: i64 = fn_state.esizeshadow_7812;
        // C s_18_1: const #64s : i
        let s_18_1: i128 = 64;
        // D s_18_2: cast zx s_18_0 -> i
        let s_18_2: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_3: cmp-eq s_18_2 s_18_1
        let s_18_3: bool = ((s_18_2) == (s_18_1));
        // D s_18_4: not s_18_3
        let s_18_4: bool = !s_18_3;
        // N s_18_5: branch s_18_4 b20 b19
        if s_18_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #64s : i64
        let s_19_0: i64 = 64;
        // D s_19_1: read-var d:i64
        let s_19_1: i64 = fn_state.d;
        // D s_19_2: read-var elementsshadow#7813:i64
        let s_19_2: i64 = fn_state.elementsshadow_7813;
        // D s_19_3: read-var m:i64
        let s_19_3: i64 = fn_state.m;
        // D s_19_4: read-var regs:i64
        let s_19_4: i64 = fn_state.regs;
        // D s_19_5: read-var shift_amountshadow#7811:i
        let s_19_5: i128 = fn_state.shift_amountshadow_7811;
        // D s_19_6: read-var is_unsigned:u8
        let s_19_6: bool = fn_state.is_unsigned;
        // D s_19_7: call execute_aarch32_instrs_VSHR_Op_A_txt(s_19_1, s_19_2, s_19_0, s_19_3, s_19_4, s_19_5, s_19_6)
        let s_19_7: () = execute_aarch32_instrs_VSHR_Op_A_txt(
            state,
            tracer,
            s_19_1,
            s_19_2,
            s_19_0,
            s_19_3,
            s_19_4,
            s_19_5,
            s_19_6,
        );
        // N s_19_8: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // N s_20_1: assert s_20_0
        let s_20_1: () = assert!(s_20_0);
        // N s_20_2: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1s : i64
        let s_21_0: i64 = 1;
        // D s_21_1: write-var ga#360874 <= s_21_0
        fn_state.ga_360874 = s_21_0;
        // N s_21_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var ga#360860:u8
        let s_22_0: u8 = fn_state.ga_360860;
        // C s_22_1: const #4s : i
        let s_22_1: i128 = 4;
        // D s_22_2: cast zx s_22_0 -> bv
        let s_22_2: Bits = Bits::new(s_22_0 as u128, 7u16);
        // C s_22_3: const #1s : i64
        let s_22_3: i64 = 1;
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #2s : i
        let s_22_5: i128 = 2;
        // C s_22_6: add s_22_5 s_22_4
        let s_22_6: i128 = (s_22_5 + s_22_4);
        // D s_22_7: bit-extract s_22_2 s_22_1 s_22_6
        let s_22_7: Bits = (Bits::new(
            ((s_22_2) >> (s_22_1)).value(),
            u16::try_from(s_22_6).unwrap(),
        ));
        // D s_22_8: cast reint s_22_7 -> u8
        let s_22_8: u8 = (s_22_7.value() as u8);
        // D s_22_9: cast zx s_22_8 -> bv
        let s_22_9: Bits = Bits::new(s_22_8 as u128, 3u16);
        // C s_22_10: const #1u : u8
        let s_22_10: u8 = 1;
        // C s_22_11: cast zx s_22_10 -> bv
        let s_22_11: Bits = Bits::new(s_22_10 as u128, 3u16);
        // D s_22_12: cmp-eq s_22_9 s_22_11
        let s_22_12: bool = ((s_22_9) == (s_22_11));
        // D s_22_13: not s_22_12
        let s_22_13: bool = !s_22_12;
        // N s_22_14: branch s_22_13 b24 b23
        if s_22_13 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #16s : i64
        let s_23_0: i64 = 16;
        // D s_23_1: write-var esize <= s_23_0
        fn_state.esize = s_23_0;
        // C s_23_2: const #4s : i64
        let s_23_2: i64 = 4;
        // D s_23_3: write-var elements <= s_23_2
        fn_state.elements = s_23_2;
        // D s_23_4: read-var imm6:u8
        let s_23_4: u8 = fn_state.imm6;
        // D s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 6u16);
        // D s_23_6: cast zx s_23_5 -> i
        let s_23_6: i128 = (s_23_5.value() as i128);
        // D s_23_7: cast reint s_23_6 -> i64
        let s_23_7: i64 = (s_23_6 as i64);
        // C s_23_8: const #32s : i
        let s_23_8: i128 = 32;
        // D s_23_9: cast zx s_23_7 -> i
        let s_23_9: i128 = (i128::try_from(s_23_7).unwrap());
        // D s_23_10: sub s_23_8 s_23_9
        let s_23_10: i128 = ((s_23_8) - (s_23_9));
        // D s_23_11: write-var shift_amount <= s_23_10
        fn_state.shift_amount = s_23_10;
        // N s_23_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var ga#360860:u8
        let s_24_0: u8 = fn_state.ga_360860;
        // C s_24_1: const #5s : i
        let s_24_1: i128 = 5;
        // D s_24_2: cast zx s_24_0 -> bv
        let s_24_2: Bits = Bits::new(s_24_0 as u128, 7u16);
        // C s_24_3: const #1s : i64
        let s_24_3: i64 = 1;
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // C s_24_5: const #1s : i
        let s_24_5: i128 = 1;
        // C s_24_6: add s_24_5 s_24_4
        let s_24_6: i128 = (s_24_5 + s_24_4);
        // D s_24_7: bit-extract s_24_2 s_24_1 s_24_6
        let s_24_7: Bits = (Bits::new(
            ((s_24_2) >> (s_24_1)).value(),
            u16::try_from(s_24_6).unwrap(),
        ));
        // D s_24_8: cast reint s_24_7 -> u8
        let s_24_8: u8 = (s_24_7.value() as u8);
        // D s_24_9: cast zx s_24_8 -> bv
        let s_24_9: Bits = Bits::new(s_24_8 as u128, 2u16);
        // C s_24_10: const #1u : u8
        let s_24_10: u8 = 1;
        // C s_24_11: cast zx s_24_10 -> bv
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 2u16);
        // D s_24_12: cmp-eq s_24_9 s_24_11
        let s_24_12: bool = ((s_24_9) == (s_24_11));
        // D s_24_13: not s_24_12
        let s_24_13: bool = !s_24_12;
        // N s_24_14: branch s_24_13 b26 b25
        if s_24_13 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #32s : i64
        let s_25_0: i64 = 32;
        // D s_25_1: write-var esize <= s_25_0
        fn_state.esize = s_25_0;
        // C s_25_2: const #2s : i64
        let s_25_2: i64 = 2;
        // D s_25_3: write-var elements <= s_25_2
        fn_state.elements = s_25_2;
        // D s_25_4: read-var imm6:u8
        let s_25_4: u8 = fn_state.imm6;
        // D s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 6u16);
        // D s_25_6: cast zx s_25_5 -> i
        let s_25_6: i128 = (s_25_5.value() as i128);
        // D s_25_7: cast reint s_25_6 -> i64
        let s_25_7: i64 = (s_25_6 as i64);
        // C s_25_8: const #64s : i
        let s_25_8: i128 = 64;
        // D s_25_9: cast zx s_25_7 -> i
        let s_25_9: i128 = (i128::try_from(s_25_7).unwrap());
        // D s_25_10: sub s_25_8 s_25_9
        let s_25_10: i128 = ((s_25_8) - (s_25_9));
        // D s_25_11: write-var shift_amount <= s_25_10
        fn_state.shift_amount = s_25_10;
        // N s_25_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var ga#360860:u8
        let s_26_0: u8 = fn_state.ga_360860;
        // C s_26_1: const #6s : i
        let s_26_1: i128 = 6;
        // D s_26_2: cast zx s_26_0 -> bv
        let s_26_2: Bits = Bits::new(s_26_0 as u128, 7u16);
        // C s_26_3: const #1s : i64
        let s_26_3: i64 = 1;
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #0s : i
        let s_26_5: i128 = 0;
        // C s_26_6: add s_26_5 s_26_4
        let s_26_6: i128 = (s_26_5 + s_26_4);
        // D s_26_7: bit-extract s_26_2 s_26_1 s_26_6
        let s_26_7: Bits = (Bits::new(
            ((s_26_2) >> (s_26_1)).value(),
            u16::try_from(s_26_6).unwrap(),
        ));
        // D s_26_8: cast reint s_26_7 -> u8
        let s_26_8: bool = ((s_26_7.value()) != 0);
        // D s_26_9: cast zx s_26_8 -> bv
        let s_26_9: Bits = Bits::new(s_26_8 as u128, 1u16);
        // C s_26_10: const #1u : u8
        let s_26_10: bool = true;
        // C s_26_11: cast zx s_26_10 -> bv
        let s_26_11: Bits = Bits::new(s_26_10 as u128, 1u16);
        // D s_26_12: cmp-eq s_26_9 s_26_11
        let s_26_12: bool = ((s_26_9) == (s_26_11));
        // D s_26_13: not s_26_12
        let s_26_13: bool = !s_26_12;
        // N s_26_14: branch s_26_13 b28 b27
        if s_26_13 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #64s : i64
        let s_27_0: i64 = 64;
        // D s_27_1: write-var esize <= s_27_0
        fn_state.esize = s_27_0;
        // C s_27_2: const #1s : i64
        let s_27_2: i64 = 1;
        // D s_27_3: write-var elements <= s_27_2
        fn_state.elements = s_27_2;
        // D s_27_4: read-var imm6:u8
        let s_27_4: u8 = fn_state.imm6;
        // D s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 6u16);
        // D s_27_6: cast zx s_27_5 -> i
        let s_27_6: i128 = (s_27_5.value() as i128);
        // D s_27_7: cast reint s_27_6 -> i64
        let s_27_7: i64 = (s_27_6 as i64);
        // C s_27_8: const #64s : i
        let s_27_8: i128 = 64;
        // D s_27_9: cast zx s_27_7 -> i
        let s_27_9: i128 = (i128::try_from(s_27_7).unwrap());
        // D s_27_10: sub s_27_8 s_27_9
        let s_27_10: i128 = ((s_27_8) - (s_27_9));
        // D s_27_11: write-var shift_amount <= s_27_10
        fn_state.shift_amount = s_27_10;
        // N s_27_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: panic
        panic!("{:?}", ());
        // N s_29_1: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0s : i
        let s_30_0: i128 = 0;
        // D s_30_1: read-var Vd:u8
        let s_30_1: u8 = fn_state.Vd;
        // D s_30_2: cast zx s_30_1 -> bv
        let s_30_2: Bits = Bits::new(s_30_1 as u128, 4u16);
        // C s_30_3: const #1u : u64
        let s_30_3: u64 = 1;
        // D s_30_4: bit-extract s_30_2 s_30_0 s_30_3
        let s_30_4: Bits = (Bits::new(
            ((s_30_2) >> (s_30_0)).value(),
            u16::try_from(s_30_3).unwrap(),
        ));
        // D s_30_5: cast reint s_30_4 -> u8
        let s_30_5: bool = ((s_30_4.value()) != 0);
        // C s_30_6: const #0s : i
        let s_30_6: i128 = 0;
        // C s_30_7: const #0u : u64
        let s_30_7: u64 = 0;
        // D s_30_8: cast zx s_30_5 -> u64
        let s_30_8: u64 = (s_30_5 as u64);
        // C s_30_9: const #1u : u64
        let s_30_9: u64 = 1;
        // D s_30_10: and s_30_8 s_30_9
        let s_30_10: u64 = ((s_30_8) & (s_30_9));
        // D s_30_11: cmp-eq s_30_10 s_30_9
        let s_30_11: bool = ((s_30_10) == (s_30_9));
        // D s_30_12: lsl s_30_8 s_30_6
        let s_30_12: u64 = s_30_8 << s_30_6;
        // D s_30_13: or s_30_7 s_30_12
        let s_30_13: u64 = ((s_30_7) | (s_30_12));
        // D s_30_14: cmpl s_30_12
        let s_30_14: u64 = !s_30_12;
        // D s_30_15: and s_30_7 s_30_14
        let s_30_15: u64 = ((s_30_7) & (s_30_14));
        // D s_30_16: select s_30_11 s_30_13 s_30_15
        let s_30_16: u64 = if s_30_11 { s_30_13 } else { s_30_15 };
        // D s_30_17: cast trunc s_30_16 -> u8
        let s_30_17: bool = ((s_30_16) != 0);
        // D s_30_18: cast zx s_30_17 -> bv
        let s_30_18: Bits = Bits::new(s_30_17 as u128, 1u16);
        // C s_30_19: const #1u : u8
        let s_30_19: bool = true;
        // C s_30_20: cast zx s_30_19 -> bv
        let s_30_20: Bits = Bits::new(s_30_19 as u128, 1u16);
        // D s_30_21: cmp-eq s_30_18 s_30_20
        let s_30_21: bool = ((s_30_18) == (s_30_20));
        // N s_30_22: branch s_30_21 b33 b31
        if s_30_21 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0s : i
        let s_31_0: i128 = 0;
        // D s_31_1: read-var Vm:u8
        let s_31_1: u8 = fn_state.Vm;
        // D s_31_2: cast zx s_31_1 -> bv
        let s_31_2: Bits = Bits::new(s_31_1 as u128, 4u16);
        // C s_31_3: const #1u : u64
        let s_31_3: u64 = 1;
        // D s_31_4: bit-extract s_31_2 s_31_0 s_31_3
        let s_31_4: Bits = (Bits::new(
            ((s_31_2) >> (s_31_0)).value(),
            u16::try_from(s_31_3).unwrap(),
        ));
        // D s_31_5: cast reint s_31_4 -> u8
        let s_31_5: bool = ((s_31_4.value()) != 0);
        // C s_31_6: const #0s : i
        let s_31_6: i128 = 0;
        // C s_31_7: const #0u : u64
        let s_31_7: u64 = 0;
        // D s_31_8: cast zx s_31_5 -> u64
        let s_31_8: u64 = (s_31_5 as u64);
        // C s_31_9: const #1u : u64
        let s_31_9: u64 = 1;
        // D s_31_10: and s_31_8 s_31_9
        let s_31_10: u64 = ((s_31_8) & (s_31_9));
        // D s_31_11: cmp-eq s_31_10 s_31_9
        let s_31_11: bool = ((s_31_10) == (s_31_9));
        // D s_31_12: lsl s_31_8 s_31_6
        let s_31_12: u64 = s_31_8 << s_31_6;
        // D s_31_13: or s_31_7 s_31_12
        let s_31_13: u64 = ((s_31_7) | (s_31_12));
        // D s_31_14: cmpl s_31_12
        let s_31_14: u64 = !s_31_12;
        // D s_31_15: and s_31_7 s_31_14
        let s_31_15: u64 = ((s_31_7) & (s_31_14));
        // D s_31_16: select s_31_11 s_31_13 s_31_15
        let s_31_16: u64 = if s_31_11 { s_31_13 } else { s_31_15 };
        // D s_31_17: cast trunc s_31_16 -> u8
        let s_31_17: bool = ((s_31_16) != 0);
        // D s_31_18: cast zx s_31_17 -> bv
        let s_31_18: Bits = Bits::new(s_31_17 as u128, 1u16);
        // C s_31_19: const #1u : u8
        let s_31_19: bool = true;
        // C s_31_20: cast zx s_31_19 -> bv
        let s_31_20: Bits = Bits::new(s_31_19 as u128, 1u16);
        // D s_31_21: cmp-eq s_31_18 s_31_20
        let s_31_21: bool = ((s_31_18) == (s_31_20));
        // D s_31_22: write-var gs#319118 <= s_31_21
        fn_state.gs_319118 = s_31_21;
        // N s_31_23: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#319118:u8
        let s_32_0: bool = fn_state.gs_319118;
        // D s_32_1: write-var gs#319119 <= s_32_0
        fn_state.gs_319119 = s_32_0;
        // N s_32_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#319118 <= s_33_0
        fn_state.gs_319118 = s_33_0;
        // N s_33_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: panic
        panic!("{:?}", ());
        // N s_34_1: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#319109 <= s_35_0
        fn_state.gs_319109 = s_35_0;
        // N s_35_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
