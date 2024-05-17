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
use u__id::*;
use ConditionPassed::*;
use execute_aarch32_instrs_VMLA_s_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMLA_s_T2enc_A_txt<T: Tracer>(
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
        m: i128,
        elementsshadow_7534: i64,
        esize: i64,
        gs_312730: bool,
        n: i64,
        index: i128,
        gs_312731: bool,
        gs_312682: bool,
        esizeshadow_7533: i64,
        gs_312732: bool,
        gs_312735: bool,
        gs_312715: bool,
        gs_312728: bool,
        gs_312717: bool,
        gs_312719: bool,
        d: i64,
        add: bool,
        is_unsigned: bool,
        elements: i64,
        gs_312722: bool,
        mshadow_7531: i128,
        gs_312734: bool,
        gs_312729: bool,
        indexshadow_7532: i128,
        gs_312733: bool,
        gs_312725: bool,
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
        // N s_2_5: branch s_2_4 b54 b3
        if s_2_4 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var size:u8
        let s_3_0: u8 = fn_state.size;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #0u : u8
        let s_3_2: u8 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b53 b4
        if s_3_4 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var Vd:u8
        let s_4_1: u8 = fn_state.Vd;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1u : u64
        let s_4_3: u64 = 1;
        // D s_4_4: bit-extract s_4_2 s_4_0 s_4_3
        let s_4_4: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_3).unwrap(),
        ));
        // D s_4_5: cast reint s_4_4 -> u8
        let s_4_5: bool = ((s_4_4.value()) != 0);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // C s_4_7: const #0u : u64
        let s_4_7: u64 = 0;
        // D s_4_8: cast zx s_4_5 -> u64
        let s_4_8: u64 = (s_4_5 as u64);
        // C s_4_9: const #1u : u64
        let s_4_9: u64 = 1;
        // D s_4_10: and s_4_8 s_4_9
        let s_4_10: u64 = ((s_4_8) & (s_4_9));
        // D s_4_11: cmp-eq s_4_10 s_4_9
        let s_4_11: bool = ((s_4_10) == (s_4_9));
        // D s_4_12: lsl s_4_8 s_4_6
        let s_4_12: u64 = s_4_8 << s_4_6;
        // D s_4_13: or s_4_7 s_4_12
        let s_4_13: u64 = ((s_4_7) | (s_4_12));
        // D s_4_14: cmpl s_4_12
        let s_4_14: u64 = !s_4_12;
        // D s_4_15: and s_4_7 s_4_14
        let s_4_15: u64 = ((s_4_7) & (s_4_14));
        // D s_4_16: select s_4_11 s_4_13 s_4_15
        let s_4_16: u64 = if s_4_11 { s_4_13 } else { s_4_15 };
        // D s_4_17: cast trunc s_4_16 -> u8
        let s_4_17: bool = ((s_4_16) != 0);
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // C s_4_19: const #1u : u8
        let s_4_19: bool = true;
        // C s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 1u16);
        // D s_4_21: cmp-eq s_4_18 s_4_20
        let s_4_21: bool = ((s_4_18) == (s_4_20));
        // D s_4_22: write-var gs#312682 <= s_4_21
        fn_state.gs_312682 = s_4_21;
        // N s_4_23: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#312682:u8
        let s_5_0: bool = fn_state.gs_312682;
        // N s_5_1: branch s_5_0 b52 b6
        if s_5_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var U:u8
        let s_6_0: bool = fn_state.U;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: write-var is_unsigned <= s_6_4
        fn_state.is_unsigned = s_6_4;
        // D s_6_6: read-var op:u8
        let s_6_6: bool = fn_state.op;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 1u16);
        // C s_6_8: const #0u : u8
        let s_6_8: bool = false;
        // C s_6_9: cast zx s_6_8 -> bv
        let s_6_9: Bits = Bits::new(s_6_8 as u128, 1u16);
        // D s_6_10: cmp-eq s_6_7 s_6_9
        let s_6_10: bool = ((s_6_7) == (s_6_9));
        // D s_6_11: write-var add <= s_6_10
        fn_state.add = s_6_10;
        // D s_6_12: read-var D:u8
        let s_6_12: bool = fn_state.D;
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 1u16);
        // D s_6_14: read-var Vd:u8
        let s_6_14: u8 = fn_state.Vd;
        // D s_6_15: cast zx s_6_14 -> bv
        let s_6_15: Bits = Bits::new(s_6_14 as u128, 4u16);
        // D s_6_16: cast reint s_6_13 -> u128
        let s_6_16: u128 = (s_6_13.value() as u128);
        // D s_6_17: size-of s_6_13
        let s_6_17: u16 = s_6_13.length();
        // D s_6_18: cast reint s_6_15 -> u128
        let s_6_18: u128 = (s_6_15.value() as u128);
        // D s_6_19: size-of s_6_15
        let s_6_19: u16 = s_6_15.length();
        // D s_6_20: lsl s_6_16 s_6_19
        let s_6_20: u128 = s_6_16 << s_6_19;
        // D s_6_21: or s_6_20 s_6_18
        let s_6_21: u128 = ((s_6_20) | (s_6_18));
        // D s_6_22: add s_6_17 s_6_19
        let s_6_22: u16 = (s_6_17 + s_6_19);
        // D s_6_23: create-bits s_6_21 s_6_22
        let s_6_23: Bits = Bits::new(s_6_21, s_6_22);
        // D s_6_24: cast reint s_6_23 -> u8
        let s_6_24: u8 = (s_6_23.value() as u8);
        // D s_6_25: cast zx s_6_24 -> bv
        let s_6_25: Bits = Bits::new(s_6_24 as u128, 5u16);
        // D s_6_26: cast zx s_6_25 -> i
        let s_6_26: i128 = (s_6_25.value() as i128);
        // D s_6_27: cast reint s_6_26 -> i64
        let s_6_27: i64 = (s_6_26 as i64);
        // D s_6_28: write-var d <= s_6_27
        fn_state.d = s_6_27;
        // D s_6_29: read-var N:u8
        let s_6_29: bool = fn_state.N;
        // D s_6_30: cast zx s_6_29 -> bv
        let s_6_30: Bits = Bits::new(s_6_29 as u128, 1u16);
        // D s_6_31: read-var Vn:u8
        let s_6_31: u8 = fn_state.Vn;
        // D s_6_32: cast zx s_6_31 -> bv
        let s_6_32: Bits = Bits::new(s_6_31 as u128, 4u16);
        // D s_6_33: cast reint s_6_30 -> u128
        let s_6_33: u128 = (s_6_30.value() as u128);
        // D s_6_34: size-of s_6_30
        let s_6_34: u16 = s_6_30.length();
        // D s_6_35: cast reint s_6_32 -> u128
        let s_6_35: u128 = (s_6_32.value() as u128);
        // D s_6_36: size-of s_6_32
        let s_6_36: u16 = s_6_32.length();
        // D s_6_37: lsl s_6_33 s_6_36
        let s_6_37: u128 = s_6_33 << s_6_36;
        // D s_6_38: or s_6_37 s_6_35
        let s_6_38: u128 = ((s_6_37) | (s_6_35));
        // D s_6_39: add s_6_34 s_6_36
        let s_6_39: u16 = (s_6_34 + s_6_36);
        // D s_6_40: create-bits s_6_38 s_6_39
        let s_6_40: Bits = Bits::new(s_6_38, s_6_39);
        // D s_6_41: cast reint s_6_40 -> u8
        let s_6_41: u8 = (s_6_40.value() as u8);
        // D s_6_42: cast zx s_6_41 -> bv
        let s_6_42: Bits = Bits::new(s_6_41 as u128, 5u16);
        // D s_6_43: cast zx s_6_42 -> i
        let s_6_43: i128 = (s_6_42.value() as i128);
        // D s_6_44: cast reint s_6_43 -> i64
        let s_6_44: i64 = (s_6_43 as i64);
        // D s_6_45: write-var n <= s_6_44
        fn_state.n = s_6_44;
        // C s_6_46: const #16s : i64
        let s_6_46: i64 = 16;
        // D s_6_47: write-var esize <= s_6_46
        fn_state.esize = s_6_46;
        // C s_6_48: const #2s : i64
        let s_6_48: i64 = 2;
        // D s_6_49: write-var elements <= s_6_48
        fn_state.elements = s_6_48;
        // D s_6_50: read-var size:u8
        let s_6_50: u8 = fn_state.size;
        // D s_6_51: cast zx s_6_50 -> bv
        let s_6_51: Bits = Bits::new(s_6_50 as u128, 2u16);
        // C s_6_52: const #1u : u8
        let s_6_52: u8 = 1;
        // C s_6_53: cast zx s_6_52 -> bv
        let s_6_53: Bits = Bits::new(s_6_52 as u128, 2u16);
        // D s_6_54: cmp-eq s_6_51 s_6_53
        let s_6_54: bool = ((s_6_51) == (s_6_53));
        // N s_6_55: branch s_6_54 b51 b7
        if s_6_54 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
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
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b50 b9
        if s_8_4 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var m:i
        let s_10_0: i128 = fn_state.m;
        // D s_10_1: write-var mshadow#7531 <= s_10_0
        fn_state.mshadow_7531 = s_10_0;
        // D s_10_2: read-var index:i
        let s_10_2: i128 = fn_state.index;
        // D s_10_3: write-var indexshadow#7532 <= s_10_2
        fn_state.indexshadow_7532 = s_10_2;
        // D s_10_4: read-var esize:i64
        let s_10_4: i64 = fn_state.esize;
        // D s_10_5: write-var esizeshadow#7533 <= s_10_4
        fn_state.esizeshadow_7533 = s_10_4;
        // D s_10_6: read-var elements:i64
        let s_10_6: i64 = fn_state.elements;
        // D s_10_7: write-var elementsshadow#7534 <= s_10_6
        fn_state.elementsshadow_7534 = s_10_6;
        // D s_10_8: read-var n:i64
        let s_10_8: i64 = fn_state.n;
        // D s_10_9: cast zx s_10_8 -> i
        let s_10_9: i128 = (i128::try_from(s_10_8).unwrap());
        // D s_10_10: call __id(s_10_9)
        let s_10_10: i128 = u__id(state, tracer, s_10_9);
        // D s_10_11: cast reint s_10_10 -> i64
        let s_10_11: i64 = (s_10_10 as i64);
        // C s_10_12: const #0s : i
        let s_10_12: i128 = 0;
        // D s_10_13: cast zx s_10_11 -> i
        let s_10_13: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_14: cmp-le s_10_12 s_10_13
        let s_10_14: bool = ((s_10_12) <= (s_10_13));
        // N s_10_15: branch s_10_14 b13 b11
        if s_10_14 {
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
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#312735 <= s_11_0
        fn_state.gs_312735 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#312735:u8
        let s_12_0: bool = fn_state.gs_312735;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var esizeshadow#7533:i64
        let s_12_2: i64 = fn_state.esizeshadow_7533;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // D s_12_5: read-var indexshadow#7532:i
        let s_12_5: i128 = fn_state.indexshadow_7532;
        // D s_12_6: cast reint s_12_5 -> i64
        let s_12_6: i64 = (s_12_5 as i64);
        // D s_12_7: read-var mshadow#7531:i
        let s_12_7: i128 = fn_state.mshadow_7531;
        // D s_12_8: cast reint s_12_7 -> i64
        let s_12_8: i64 = (s_12_7 as i64);
        // C s_12_9: const #1s : i64
        let s_12_9: i64 = 1;
        // D s_12_10: read-var add:u8
        let s_12_10: bool = fn_state.add;
        // D s_12_11: read-var d:i64
        let s_12_11: i64 = fn_state.d;
        // D s_12_12: read-var elementsshadow#7534:i64
        let s_12_12: i64 = fn_state.elementsshadow_7534;
        // C s_12_13: const #0u : u8
        let s_12_13: bool = false;
        // C s_12_14: const #1u : u8
        let s_12_14: bool = true;
        // D s_12_15: read-var n:i64
        let s_12_15: i64 = fn_state.n;
        // D s_12_16: read-var is_unsigned:u8
        let s_12_16: bool = fn_state.is_unsigned;
        // D s_12_17: call execute_aarch32_instrs_VMLA_s_Op_A_txt(s_12_10, s_12_11, s_12_12, s_12_4, s_12_13, s_12_6, s_12_14, s_12_8, s_12_15, s_12_9, s_12_16)
        let s_12_17: () = execute_aarch32_instrs_VMLA_s_Op_A_txt(
            state,
            tracer,
            s_12_10,
            s_12_11,
            s_12_12,
            s_12_4,
            s_12_13,
            s_12_6,
            s_12_14,
            s_12_8,
            s_12_15,
            s_12_9,
            s_12_16,
        );
        // N s_12_18: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var n:i64
        let s_13_0: i64 = fn_state.n;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call __id(s_13_1)
        let s_13_2: i128 = u__id(state, tracer, s_13_1);
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: const #31s : i
        let s_13_4: i128 = 31;
        // D s_13_5: cast zx s_13_3 -> i
        let s_13_5: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_6: cmp-le s_13_5 s_13_4
        let s_13_6: bool = ((s_13_5) <= (s_13_4));
        // N s_13_7: branch s_13_6 b16 b14
        if s_13_6 {
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
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#312734 <= s_14_0
        fn_state.gs_312734 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#312734:u8
        let s_15_0: bool = fn_state.gs_312734;
        // D s_15_1: write-var gs#312735 <= s_15_0
        fn_state.gs_312735 = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var mshadow#7531:i
        let s_16_0: i128 = fn_state.mshadow_7531;
        // D s_16_1: call __id(s_16_0)
        let s_16_1: i128 = u__id(state, tracer, s_16_0);
        // C s_16_2: const #0s : i
        let s_16_2: i128 = 0;
        // D s_16_3: cmp-le s_16_2 s_16_1
        let s_16_3: bool = ((s_16_2) <= (s_16_1));
        // N s_16_4: branch s_16_3 b19 b17
        if s_16_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#312733 <= s_17_0
        fn_state.gs_312733 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#312733:u8
        let s_18_0: bool = fn_state.gs_312733;
        // D s_18_1: write-var gs#312734 <= s_18_0
        fn_state.gs_312734 = s_18_0;
        // N s_18_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var mshadow#7531:i
        let s_19_0: i128 = fn_state.mshadow_7531;
        // D s_19_1: call __id(s_19_0)
        let s_19_1: i128 = u__id(state, tracer, s_19_0);
        // C s_19_2: const #15s : i
        let s_19_2: i128 = 15;
        // D s_19_3: cmp-le s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) <= (s_19_2));
        // N s_19_4: branch s_19_3 b22 b20
        if s_19_3 {
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
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#312732 <= s_20_0
        fn_state.gs_312732 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#312732:u8
        let s_21_0: bool = fn_state.gs_312732;
        // D s_21_1: write-var gs#312733 <= s_21_0
        fn_state.gs_312733 = s_21_0;
        // N s_21_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var indexshadow#7532:i
        let s_22_0: i128 = fn_state.indexshadow_7532;
        // D s_22_1: call __id(s_22_0)
        let s_22_1: i128 = u__id(state, tracer, s_22_0);
        // C s_22_2: const #0s : i
        let s_22_2: i128 = 0;
        // D s_22_3: cmp-eq s_22_1 s_22_2
        let s_22_3: bool = ((s_22_1) == (s_22_2));
        // N s_22_4: branch s_22_3 b49 b23
        if s_22_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var indexshadow#7532:i
        let s_23_0: i128 = fn_state.indexshadow_7532;
        // D s_23_1: call __id(s_23_0)
        let s_23_1: i128 = u__id(state, tracer, s_23_0);
        // C s_23_2: const #1s : i
        let s_23_2: i128 = 1;
        // D s_23_3: cmp-eq s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) == (s_23_2));
        // D s_23_4: write-var gs#312715 <= s_23_3
        fn_state.gs_312715 = s_23_3;
        // N s_23_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#312715:u8
        let s_24_0: bool = fn_state.gs_312715;
        // N s_24_1: branch s_24_0 b48 b25
        if s_24_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var indexshadow#7532:i
        let s_25_0: i128 = fn_state.indexshadow_7532;
        // D s_25_1: call __id(s_25_0)
        let s_25_1: i128 = u__id(state, tracer, s_25_0);
        // C s_25_2: const #2s : i
        let s_25_2: i128 = 2;
        // D s_25_3: cmp-eq s_25_1 s_25_2
        let s_25_3: bool = ((s_25_1) == (s_25_2));
        // D s_25_4: write-var gs#312717 <= s_25_3
        fn_state.gs_312717 = s_25_3;
        // N s_25_5: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#312717:u8
        let s_26_0: bool = fn_state.gs_312717;
        // N s_26_1: branch s_26_0 b47 b27
        if s_26_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var indexshadow#7532:i
        let s_27_0: i128 = fn_state.indexshadow_7532;
        // D s_27_1: call __id(s_27_0)
        let s_27_1: i128 = u__id(state, tracer, s_27_0);
        // C s_27_2: const #3s : i
        let s_27_2: i128 = 3;
        // D s_27_3: cmp-eq s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) == (s_27_2));
        // D s_27_4: write-var gs#312719 <= s_27_3
        fn_state.gs_312719 = s_27_3;
        // N s_27_5: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#312719:u8
        let s_28_0: bool = fn_state.gs_312719;
        // N s_28_1: branch s_28_0 b31 b29
        if s_28_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#312731 <= s_29_0
        fn_state.gs_312731 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#312731:u8
        let s_30_0: bool = fn_state.gs_312731;
        // D s_30_1: write-var gs#312732 <= s_30_0
        fn_state.gs_312732 = s_30_0;
        // N s_30_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var esizeshadow#7533:i64
        let s_31_0: i64 = fn_state.esizeshadow_7533;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: call __id(s_31_1)
        let s_31_2: i128 = u__id(state, tracer, s_31_1);
        // D s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: const #16s : i
        let s_31_4: i128 = 16;
        // D s_31_5: cast zx s_31_3 -> i
        let s_31_5: i128 = (i128::try_from(s_31_3).unwrap());
        // D s_31_6: cmp-eq s_31_5 s_31_4
        let s_31_6: bool = ((s_31_5) == (s_31_4));
        // N s_31_7: branch s_31_6 b46 b32
        if s_31_6 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var esizeshadow#7533:i64
        let s_32_0: i64 = fn_state.esizeshadow_7533;
        // D s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_2: call __id(s_32_1)
        let s_32_2: i128 = u__id(state, tracer, s_32_1);
        // D s_32_3: cast reint s_32_2 -> i64
        let s_32_3: i64 = (s_32_2 as i64);
        // C s_32_4: const #32s : i
        let s_32_4: i128 = 32;
        // D s_32_5: cast zx s_32_3 -> i
        let s_32_5: i128 = (i128::try_from(s_32_3).unwrap());
        // D s_32_6: cmp-eq s_32_5 s_32_4
        let s_32_6: bool = ((s_32_5) == (s_32_4));
        // D s_32_7: write-var gs#312722 <= s_32_6
        fn_state.gs_312722 = s_32_6;
        // N s_32_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#312722:u8
        let s_33_0: bool = fn_state.gs_312722;
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
        // D s_34_1: write-var gs#312730 <= s_34_0
        fn_state.gs_312730 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#312730:u8
        let s_35_0: bool = fn_state.gs_312730;
        // D s_35_1: write-var gs#312731 <= s_35_0
        fn_state.gs_312731 = s_35_0;
        // N s_35_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var elementsshadow#7534:i64
        let s_36_0: i64 = fn_state.elementsshadow_7534;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: call __id(s_36_1)
        let s_36_2: i128 = u__id(state, tracer, s_36_1);
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: const #2s : i
        let s_36_4: i128 = 2;
        // D s_36_5: cast zx s_36_3 -> i
        let s_36_5: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_6: cmp-eq s_36_5 s_36_4
        let s_36_6: bool = ((s_36_5) == (s_36_4));
        // N s_36_7: branch s_36_6 b45 b37
        if s_36_6 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var elementsshadow#7534:i64
        let s_37_0: i64 = fn_state.elementsshadow_7534;
        // D s_37_1: cast zx s_37_0 -> i
        let s_37_1: i128 = (i128::try_from(s_37_0).unwrap());
        // D s_37_2: call __id(s_37_1)
        let s_37_2: i128 = u__id(state, tracer, s_37_1);
        // D s_37_3: cast reint s_37_2 -> i64
        let s_37_3: i64 = (s_37_2 as i64);
        // C s_37_4: const #4s : i
        let s_37_4: i128 = 4;
        // D s_37_5: cast zx s_37_3 -> i
        let s_37_5: i128 = (i128::try_from(s_37_3).unwrap());
        // D s_37_6: cmp-eq s_37_5 s_37_4
        let s_37_6: bool = ((s_37_5) == (s_37_4));
        // D s_37_7: write-var gs#312725 <= s_37_6
        fn_state.gs_312725 = s_37_6;
        // N s_37_8: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#312725:u8
        let s_38_0: bool = fn_state.gs_312725;
        // N s_38_1: branch s_38_0 b41 b39
        if s_38_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#312729 <= s_39_0
        fn_state.gs_312729 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#312729:u8
        let s_40_0: bool = fn_state.gs_312729;
        // D s_40_1: write-var gs#312730 <= s_40_0
        fn_state.gs_312730 = s_40_0;
        // N s_40_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var d:i64
        let s_41_0: i64 = fn_state.d;
        // D s_41_1: cast zx s_41_0 -> i
        let s_41_1: i128 = (i128::try_from(s_41_0).unwrap());
        // D s_41_2: call __id(s_41_1)
        let s_41_2: i128 = u__id(state, tracer, s_41_1);
        // D s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: const #0s : i
        let s_41_4: i128 = 0;
        // D s_41_5: cast zx s_41_3 -> i
        let s_41_5: i128 = (i128::try_from(s_41_3).unwrap());
        // D s_41_6: cmp-le s_41_4 s_41_5
        let s_41_6: bool = ((s_41_4) <= (s_41_5));
        // N s_41_7: branch s_41_6 b44 b42
        if s_41_6 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#312728 <= s_42_0
        fn_state.gs_312728 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#312728:u8
        let s_43_0: bool = fn_state.gs_312728;
        // D s_43_1: write-var gs#312729 <= s_43_0
        fn_state.gs_312729 = s_43_0;
        // N s_43_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var d:i64
        let s_44_0: i64 = fn_state.d;
        // D s_44_1: cast zx s_44_0 -> i
        let s_44_1: i128 = (i128::try_from(s_44_0).unwrap());
        // D s_44_2: call __id(s_44_1)
        let s_44_2: i128 = u__id(state, tracer, s_44_1);
        // D s_44_3: cast reint s_44_2 -> i64
        let s_44_3: i64 = (s_44_2 as i64);
        // C s_44_4: const #31s : i
        let s_44_4: i128 = 31;
        // D s_44_5: cast zx s_44_3 -> i
        let s_44_5: i128 = (i128::try_from(s_44_3).unwrap());
        // D s_44_6: cmp-le s_44_5 s_44_4
        let s_44_6: bool = ((s_44_5) <= (s_44_4));
        // D s_44_7: write-var gs#312728 <= s_44_6
        fn_state.gs_312728 = s_44_6;
        // N s_44_8: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#312725 <= s_45_0
        fn_state.gs_312725 = s_45_0;
        // N s_45_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#312722 <= s_46_0
        fn_state.gs_312722 = s_46_0;
        // N s_46_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#312719 <= s_47_0
        fn_state.gs_312719 = s_47_0;
        // N s_47_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#312717 <= s_48_0
        fn_state.gs_312717 = s_48_0;
        // N s_48_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#312715 <= s_49_0
        fn_state.gs_312715 = s_49_0;
        // N s_49_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #32s : i64
        let s_50_0: i64 = 32;
        // D s_50_1: write-var esize <= s_50_0
        fn_state.esize = s_50_0;
        // C s_50_2: const #2s : i64
        let s_50_2: i64 = 2;
        // D s_50_3: write-var elements <= s_50_2
        fn_state.elements = s_50_2;
        // D s_50_4: read-var Vm:u8
        let s_50_4: u8 = fn_state.Vm;
        // D s_50_5: cast zx s_50_4 -> bv
        let s_50_5: Bits = Bits::new(s_50_4 as u128, 4u16);
        // D s_50_6: cast zx s_50_5 -> i
        let s_50_6: i128 = (s_50_5.value() as i128);
        // D s_50_7: write-var m <= s_50_6
        fn_state.m = s_50_6;
        // D s_50_8: read-var M:u8
        let s_50_8: bool = fn_state.M;
        // D s_50_9: cast zx s_50_8 -> bv
        let s_50_9: Bits = Bits::new(s_50_8 as u128, 1u16);
        // D s_50_10: cast zx s_50_9 -> i
        let s_50_10: i128 = (s_50_9.value() as i128);
        // D s_50_11: write-var index <= s_50_10
        fn_state.index = s_50_10;
        // N s_50_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #16s : i64
        let s_51_0: i64 = 16;
        // D s_51_1: write-var esize <= s_51_0
        fn_state.esize = s_51_0;
        // C s_51_2: const #4s : i64
        let s_51_2: i64 = 4;
        // D s_51_3: write-var elements <= s_51_2
        fn_state.elements = s_51_2;
        // C s_51_4: const #0s : i
        let s_51_4: i128 = 0;
        // D s_51_5: read-var Vm:u8
        let s_51_5: u8 = fn_state.Vm;
        // D s_51_6: cast zx s_51_5 -> bv
        let s_51_6: Bits = Bits::new(s_51_5 as u128, 4u16);
        // C s_51_7: const #1s : i64
        let s_51_7: i64 = 1;
        // C s_51_8: cast zx s_51_7 -> i
        let s_51_8: i128 = (i128::try_from(s_51_7).unwrap());
        // C s_51_9: const #2s : i
        let s_51_9: i128 = 2;
        // C s_51_10: add s_51_9 s_51_8
        let s_51_10: i128 = (s_51_9 + s_51_8);
        // D s_51_11: bit-extract s_51_6 s_51_4 s_51_10
        let s_51_11: Bits = (Bits::new(
            ((s_51_6) >> (s_51_4)).value(),
            u16::try_from(s_51_10).unwrap(),
        ));
        // D s_51_12: cast reint s_51_11 -> u8
        let s_51_12: u8 = (s_51_11.value() as u8);
        // D s_51_13: cast zx s_51_12 -> bv
        let s_51_13: Bits = Bits::new(s_51_12 as u128, 3u16);
        // D s_51_14: cast zx s_51_13 -> i
        let s_51_14: i128 = (s_51_13.value() as i128);
        // D s_51_15: write-var m <= s_51_14
        fn_state.m = s_51_14;
        // C s_51_16: const #3s : i
        let s_51_16: i128 = 3;
        // D s_51_17: read-var Vm:u8
        let s_51_17: u8 = fn_state.Vm;
        // D s_51_18: cast zx s_51_17 -> bv
        let s_51_18: Bits = Bits::new(s_51_17 as u128, 4u16);
        // C s_51_19: const #1u : u64
        let s_51_19: u64 = 1;
        // D s_51_20: bit-extract s_51_18 s_51_16 s_51_19
        let s_51_20: Bits = (Bits::new(
            ((s_51_18) >> (s_51_16)).value(),
            u16::try_from(s_51_19).unwrap(),
        ));
        // D s_51_21: cast reint s_51_20 -> u8
        let s_51_21: bool = ((s_51_20.value()) != 0);
        // C s_51_22: const #0s : i
        let s_51_22: i128 = 0;
        // C s_51_23: const #0u : u64
        let s_51_23: u64 = 0;
        // D s_51_24: cast zx s_51_21 -> u64
        let s_51_24: u64 = (s_51_21 as u64);
        // C s_51_25: const #1u : u64
        let s_51_25: u64 = 1;
        // D s_51_26: and s_51_24 s_51_25
        let s_51_26: u64 = ((s_51_24) & (s_51_25));
        // D s_51_27: cmp-eq s_51_26 s_51_25
        let s_51_27: bool = ((s_51_26) == (s_51_25));
        // D s_51_28: lsl s_51_24 s_51_22
        let s_51_28: u64 = s_51_24 << s_51_22;
        // D s_51_29: or s_51_23 s_51_28
        let s_51_29: u64 = ((s_51_23) | (s_51_28));
        // D s_51_30: cmpl s_51_28
        let s_51_30: u64 = !s_51_28;
        // D s_51_31: and s_51_23 s_51_30
        let s_51_31: u64 = ((s_51_23) & (s_51_30));
        // D s_51_32: select s_51_27 s_51_29 s_51_31
        let s_51_32: u64 = if s_51_27 { s_51_29 } else { s_51_31 };
        // D s_51_33: cast trunc s_51_32 -> u8
        let s_51_33: bool = ((s_51_32) != 0);
        // D s_51_34: read-var M:u8
        let s_51_34: bool = fn_state.M;
        // D s_51_35: cast zx s_51_34 -> bv
        let s_51_35: Bits = Bits::new(s_51_34 as u128, 1u16);
        // D s_51_36: cast zx s_51_33 -> bv
        let s_51_36: Bits = Bits::new(s_51_33 as u128, 1u16);
        // D s_51_37: cast reint s_51_35 -> u128
        let s_51_37: u128 = (s_51_35.value() as u128);
        // D s_51_38: size-of s_51_35
        let s_51_38: u16 = s_51_35.length();
        // D s_51_39: cast reint s_51_36 -> u128
        let s_51_39: u128 = (s_51_36.value() as u128);
        // D s_51_40: size-of s_51_36
        let s_51_40: u16 = s_51_36.length();
        // D s_51_41: lsl s_51_37 s_51_40
        let s_51_41: u128 = s_51_37 << s_51_40;
        // D s_51_42: or s_51_41 s_51_39
        let s_51_42: u128 = ((s_51_41) | (s_51_39));
        // D s_51_43: add s_51_38 s_51_40
        let s_51_43: u16 = (s_51_38 + s_51_40);
        // D s_51_44: create-bits s_51_42 s_51_43
        let s_51_44: Bits = Bits::new(s_51_42, s_51_43);
        // D s_51_45: cast reint s_51_44 -> u8
        let s_51_45: u8 = (s_51_44.value() as u8);
        // D s_51_46: cast zx s_51_45 -> bv
        let s_51_46: Bits = Bits::new(s_51_45 as u128, 2u16);
        // D s_51_47: cast zx s_51_46 -> i
        let s_51_47: i128 = (s_51_46.value() as i128);
        // D s_51_48: write-var index <= s_51_47
        fn_state.index = s_51_47;
        // N s_51_49: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: panic
        panic!("{:?}", ());
        // N s_52_1: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#312682 <= s_53_0
        fn_state.gs_312682 = s_53_0;
        // N s_53_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_54_0: panic
        panic!("{:?}", ());
        // N s_54_1: return
        return;
    }
}
