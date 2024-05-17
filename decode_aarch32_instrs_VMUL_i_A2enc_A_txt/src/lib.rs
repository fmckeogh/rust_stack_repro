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
use HaveBit128PMULLExt::*;
use execute_aarch32_instrs_VMUL_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMUL_i_A2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    size: u8,
    Vn: u8,
    Vd: u8,
    op: bool,
    N: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_7585: i64,
        polynomial: bool,
        esize: i64,
        is_unsigned: bool,
        elements: i128,
        gs_313757: bool,
        elementsshadow_7586: i128,
        U: bool,
        D: bool,
        size: u8,
        Vn: u8,
        Vd: u8,
        op: bool,
        N: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        size,
        Vn,
        Vd,
        op,
        N,
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
        // N s_2_5: branch s_2_4 b19 b3
        if s_2_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var U:u8
        let s_3_0: bool = fn_state.U;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: write-var is_unsigned <= s_3_4
        fn_state.is_unsigned = s_3_4;
        // D s_3_6: read-var op:u8
        let s_3_6: bool = fn_state.op;
        // D s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 1u16);
        // C s_3_8: const #1u : u8
        let s_3_8: bool = true;
        // C s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 1u16);
        // D s_3_10: cmp-eq s_3_7 s_3_9
        let s_3_10: bool = ((s_3_7) == (s_3_9));
        // D s_3_11: write-var polynomial <= s_3_10
        fn_state.polynomial = s_3_10;
        // D s_3_12: read-var size:u8
        let s_3_12: u8 = fn_state.size;
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 2u16);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (s_3_13.value() as i128);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // C s_3_16: const #8s : i64
        let s_3_16: i64 = 8;
        // D s_3_17: lsl s_3_16 s_3_15
        let s_3_17: i64 = s_3_16 << s_3_15;
        // D s_3_18: write-var esize <= s_3_17
        fn_state.esize = s_3_17;
        // C s_3_19: const #64s : i
        let s_3_19: i128 = 64;
        // D s_3_20: read-var esize:i64
        let s_3_20: i64 = fn_state.esize;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: div s_3_19 s_3_21
        let s_3_22: i128 = ((s_3_19) / (s_3_21));
        // D s_3_23: write-var elements <= s_3_22
        fn_state.elements = s_3_22;
        // D s_3_24: read-var polynomial:u8
        let s_3_24: bool = fn_state.polynomial;
        // N s_3_25: branch s_3_24 b8 b4
        if s_3_24 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esize:i64
        let s_5_0: i64 = fn_state.esize;
        // D s_5_1: write-var esizeshadow#7585 <= s_5_0
        fn_state.esizeshadow_7585 = s_5_0;
        // D s_5_2: read-var elements:i
        let s_5_2: i128 = fn_state.elements;
        // D s_5_3: write-var elementsshadow#7586 <= s_5_2
        fn_state.elementsshadow_7586 = s_5_2;
        // C s_5_4: const #0s : i
        let s_5_4: i128 = 0;
        // D s_5_5: read-var Vd:u8
        let s_5_5: u8 = fn_state.Vd;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 4u16);
        // C s_5_7: const #1u : u64
        let s_5_7: u64 = 1;
        // D s_5_8: bit-extract s_5_6 s_5_4 s_5_7
        let s_5_8: Bits = (Bits::new(
            ((s_5_6) >> (s_5_4)).value(),
            u16::try_from(s_5_7).unwrap(),
        ));
        // D s_5_9: cast reint s_5_8 -> u8
        let s_5_9: bool = ((s_5_8.value()) != 0);
        // C s_5_10: const #0s : i
        let s_5_10: i128 = 0;
        // C s_5_11: const #0u : u64
        let s_5_11: u64 = 0;
        // D s_5_12: cast zx s_5_9 -> u64
        let s_5_12: u64 = (s_5_9 as u64);
        // C s_5_13: const #1u : u64
        let s_5_13: u64 = 1;
        // D s_5_14: and s_5_12 s_5_13
        let s_5_14: u64 = ((s_5_12) & (s_5_13));
        // D s_5_15: cmp-eq s_5_14 s_5_13
        let s_5_15: bool = ((s_5_14) == (s_5_13));
        // D s_5_16: lsl s_5_12 s_5_10
        let s_5_16: u64 = s_5_12 << s_5_10;
        // D s_5_17: or s_5_11 s_5_16
        let s_5_17: u64 = ((s_5_11) | (s_5_16));
        // D s_5_18: cmpl s_5_16
        let s_5_18: u64 = !s_5_16;
        // D s_5_19: and s_5_11 s_5_18
        let s_5_19: u64 = ((s_5_11) & (s_5_18));
        // D s_5_20: select s_5_15 s_5_17 s_5_19
        let s_5_20: u64 = if s_5_15 { s_5_17 } else { s_5_19 };
        // D s_5_21: cast trunc s_5_20 -> u8
        let s_5_21: bool = ((s_5_20) != 0);
        // D s_5_22: cast zx s_5_21 -> bv
        let s_5_22: Bits = Bits::new(s_5_21 as u128, 1u16);
        // C s_5_23: const #1u : u8
        let s_5_23: bool = true;
        // C s_5_24: cast zx s_5_23 -> bv
        let s_5_24: Bits = Bits::new(s_5_23 as u128, 1u16);
        // D s_5_25: cmp-eq s_5_22 s_5_24
        let s_5_25: bool = ((s_5_22) == (s_5_24));
        // N s_5_26: branch s_5_25 b7 b6
        if s_5_25 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var D:u8
        let s_6_0: bool = fn_state.D;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // D s_6_2: read-var Vd:u8
        let s_6_2: u8 = fn_state.Vd;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 4u16);
        // D s_6_4: cast reint s_6_1 -> u128
        let s_6_4: u128 = (s_6_1.value() as u128);
        // D s_6_5: size-of s_6_1
        let s_6_5: u16 = s_6_1.length();
        // D s_6_6: cast reint s_6_3 -> u128
        let s_6_6: u128 = (s_6_3.value() as u128);
        // D s_6_7: size-of s_6_3
        let s_6_7: u16 = s_6_3.length();
        // D s_6_8: lsl s_6_4 s_6_7
        let s_6_8: u128 = s_6_4 << s_6_7;
        // D s_6_9: or s_6_8 s_6_6
        let s_6_9: u128 = ((s_6_8) | (s_6_6));
        // D s_6_10: add s_6_5 s_6_7
        let s_6_10: u16 = (s_6_5 + s_6_7);
        // D s_6_11: create-bits s_6_9 s_6_10
        let s_6_11: Bits = Bits::new(s_6_9, s_6_10);
        // D s_6_12: cast reint s_6_11 -> u8
        let s_6_12: u8 = (s_6_11.value() as u8);
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 5u16);
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (s_6_13.value() as i128);
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: read-var N:u8
        let s_6_16: bool = fn_state.N;
        // D s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 1u16);
        // D s_6_18: read-var Vn:u8
        let s_6_18: u8 = fn_state.Vn;
        // D s_6_19: cast zx s_6_18 -> bv
        let s_6_19: Bits = Bits::new(s_6_18 as u128, 4u16);
        // D s_6_20: cast reint s_6_17 -> u128
        let s_6_20: u128 = (s_6_17.value() as u128);
        // D s_6_21: size-of s_6_17
        let s_6_21: u16 = s_6_17.length();
        // D s_6_22: cast reint s_6_19 -> u128
        let s_6_22: u128 = (s_6_19.value() as u128);
        // D s_6_23: size-of s_6_19
        let s_6_23: u16 = s_6_19.length();
        // D s_6_24: lsl s_6_20 s_6_23
        let s_6_24: u128 = s_6_20 << s_6_23;
        // D s_6_25: or s_6_24 s_6_22
        let s_6_25: u128 = ((s_6_24) | (s_6_22));
        // D s_6_26: add s_6_21 s_6_23
        let s_6_26: u16 = (s_6_21 + s_6_23);
        // D s_6_27: create-bits s_6_25 s_6_26
        let s_6_27: Bits = Bits::new(s_6_25, s_6_26);
        // D s_6_28: cast reint s_6_27 -> u8
        let s_6_28: u8 = (s_6_27.value() as u8);
        // D s_6_29: cast zx s_6_28 -> bv
        let s_6_29: Bits = Bits::new(s_6_28 as u128, 5u16);
        // D s_6_30: cast zx s_6_29 -> i
        let s_6_30: i128 = (s_6_29.value() as i128);
        // D s_6_31: cast reint s_6_30 -> i64
        let s_6_31: i64 = (s_6_30 as i64);
        // D s_6_32: read-var M:u8
        let s_6_32: bool = fn_state.M;
        // D s_6_33: cast zx s_6_32 -> bv
        let s_6_33: Bits = Bits::new(s_6_32 as u128, 1u16);
        // D s_6_34: read-var Vm:u8
        let s_6_34: u8 = fn_state.Vm;
        // D s_6_35: cast zx s_6_34 -> bv
        let s_6_35: Bits = Bits::new(s_6_34 as u128, 4u16);
        // D s_6_36: cast reint s_6_33 -> u128
        let s_6_36: u128 = (s_6_33.value() as u128);
        // D s_6_37: size-of s_6_33
        let s_6_37: u16 = s_6_33.length();
        // D s_6_38: cast reint s_6_35 -> u128
        let s_6_38: u128 = (s_6_35.value() as u128);
        // D s_6_39: size-of s_6_35
        let s_6_39: u16 = s_6_35.length();
        // D s_6_40: lsl s_6_36 s_6_39
        let s_6_40: u128 = s_6_36 << s_6_39;
        // D s_6_41: or s_6_40 s_6_38
        let s_6_41: u128 = ((s_6_40) | (s_6_38));
        // D s_6_42: add s_6_37 s_6_39
        let s_6_42: u16 = (s_6_37 + s_6_39);
        // D s_6_43: create-bits s_6_41 s_6_42
        let s_6_43: Bits = Bits::new(s_6_41, s_6_42);
        // D s_6_44: cast reint s_6_43 -> u8
        let s_6_44: u8 = (s_6_43.value() as u8);
        // D s_6_45: cast zx s_6_44 -> bv
        let s_6_45: Bits = Bits::new(s_6_44 as u128, 5u16);
        // D s_6_46: cast zx s_6_45 -> i
        let s_6_46: i128 = (s_6_45.value() as i128);
        // D s_6_47: cast reint s_6_46 -> i64
        let s_6_47: i64 = (s_6_46 as i64);
        // D s_6_48: read-var esizeshadow#7585:i64
        let s_6_48: i64 = fn_state.esizeshadow_7585;
        // D s_6_49: cast zx s_6_48 -> i
        let s_6_49: i128 = (i128::try_from(s_6_48).unwrap());
        // D s_6_50: cast reint s_6_49 -> i64
        let s_6_50: i64 = (s_6_49 as i64);
        // C s_6_51: const #1s : i64
        let s_6_51: i64 = 1;
        // D s_6_52: read-var elementsshadow#7586:i
        let s_6_52: i128 = fn_state.elementsshadow_7586;
        // C s_6_53: const #1u : u8
        let s_6_53: bool = true;
        // D s_6_54: read-var polynomial:u8
        let s_6_54: bool = fn_state.polynomial;
        // D s_6_55: read-var is_unsigned:u8
        let s_6_55: bool = fn_state.is_unsigned;
        // D s_6_56: call execute_aarch32_instrs_VMUL_i_Op_A_txt(s_6_15, s_6_52, s_6_50, s_6_53, s_6_47, s_6_31, s_6_54, s_6_51, s_6_55)
        let s_6_56: () = execute_aarch32_instrs_VMUL_i_Op_A_txt(
            state,
            tracer,
            s_6_15,
            s_6_52,
            s_6_50,
            s_6_53,
            s_6_47,
            s_6_31,
            s_6_54,
            s_6_51,
            s_6_55,
        );
        // N s_6_57: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var U:u8
        let s_8_0: bool = fn_state.U;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #1u : u8
        let s_8_2: bool = true;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b18 b9
        if s_8_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var size:u8
        let s_9_0: u8 = fn_state.size;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // C s_9_2: const #1u : u8
        let s_9_2: u8 = 1;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 2u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: write-var gs#313757 <= s_9_4
        fn_state.gs_313757 = s_9_4;
        // N s_9_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#313757:u8
        let s_10_0: bool = fn_state.gs_313757;
        // N s_10_1: branch s_10_0 b17 b11
        if s_10_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var size:u8
        let s_11_0: u8 = fn_state.size;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // C s_11_2: const #2u : u8
        let s_11_2: u8 = 2;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 2u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b14 b12
        if s_11_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaveBit128PMULLExt(s_14_0)
        let s_14_1: bool = HaveBit128PMULLExt(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // N s_14_3: branch s_14_2 b16 b15
        if s_14_2 {
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
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // D s_15_1: write-var esize <= s_15_0
        fn_state.esize = s_15_0;
        // C s_15_2: const #1s : i
        let s_15_2: i128 = 1;
        // D s_15_3: write-var elements <= s_15_2
        fn_state.elements = s_15_2;
        // N s_15_4: jump b13
        return block_13(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#313757 <= s_18_0
        fn_state.gs_313757 = s_18_0;
        // N s_18_2: jump b10
        return block_10(state, tracer, fn_state);
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
}
