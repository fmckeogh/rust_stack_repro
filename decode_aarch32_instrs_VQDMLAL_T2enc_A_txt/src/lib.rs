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
use execute_aarch32_instrs_VQDMLAL_Op_A_txt::*;
use ConditionPassed::*;
use u__id::*;
use common::*;
pub fn decode_aarch32_instrs_VQDMLAL_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
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
        gs_315527: bool,
        gs_315483: bool,
        esize: i64,
        esizeshadow_7656: i64,
        gs_315526: bool,
        n: i64,
        index: i128,
        gs_315524: bool,
        gs_315519: bool,
        indexshadow_7655: i128,
        mshadow_7654: i128,
        gs_315517: bool,
        elementsshadow_7657: i128,
        gs_315525: bool,
        d: i64,
        add: bool,
        elements: i128,
        gs_315515: bool,
        gs_315523: bool,
        gs_315522: bool,
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
        // N s_2_5: branch s_2_4 b42 b3
        if s_2_4 {
            return block_42(state, tracer, fn_state);
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
        // N s_3_5: branch s_3_4 b41 b4
        if s_3_4 {
            return block_41(state, tracer, fn_state);
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
        // D s_4_22: write-var gs#315483 <= s_4_21
        fn_state.gs_315483 = s_4_21;
        // N s_4_23: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#315483:u8
        let s_5_0: bool = fn_state.gs_315483;
        // N s_5_1: branch s_5_0 b40 b6
        if s_5_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var op:u8
        let s_6_0: bool = fn_state.op;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #0u : u8
        let s_6_2: bool = false;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: write-var add <= s_6_4
        fn_state.add = s_6_4;
        // D s_6_6: read-var D:u8
        let s_6_6: bool = fn_state.D;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 1u16);
        // D s_6_8: read-var Vd:u8
        let s_6_8: u8 = fn_state.Vd;
        // D s_6_9: cast zx s_6_8 -> bv
        let s_6_9: Bits = Bits::new(s_6_8 as u128, 4u16);
        // D s_6_10: cast reint s_6_7 -> u128
        let s_6_10: u128 = (s_6_7.value() as u128);
        // D s_6_11: size-of s_6_7
        let s_6_11: u16 = s_6_7.length();
        // D s_6_12: cast reint s_6_9 -> u128
        let s_6_12: u128 = (s_6_9.value() as u128);
        // D s_6_13: size-of s_6_9
        let s_6_13: u16 = s_6_9.length();
        // D s_6_14: lsl s_6_10 s_6_13
        let s_6_14: u128 = s_6_10 << s_6_13;
        // D s_6_15: or s_6_14 s_6_12
        let s_6_15: u128 = ((s_6_14) | (s_6_12));
        // D s_6_16: add s_6_11 s_6_13
        let s_6_16: u16 = (s_6_11 + s_6_13);
        // D s_6_17: create-bits s_6_15 s_6_16
        let s_6_17: Bits = Bits::new(s_6_15, s_6_16);
        // D s_6_18: cast reint s_6_17 -> u8
        let s_6_18: u8 = (s_6_17.value() as u8);
        // D s_6_19: cast zx s_6_18 -> bv
        let s_6_19: Bits = Bits::new(s_6_18 as u128, 5u16);
        // D s_6_20: cast zx s_6_19 -> i
        let s_6_20: i128 = (s_6_19.value() as i128);
        // D s_6_21: cast reint s_6_20 -> i64
        let s_6_21: i64 = (s_6_20 as i64);
        // D s_6_22: write-var d <= s_6_21
        fn_state.d = s_6_21;
        // D s_6_23: read-var N:u8
        let s_6_23: bool = fn_state.N;
        // D s_6_24: cast zx s_6_23 -> bv
        let s_6_24: Bits = Bits::new(s_6_23 as u128, 1u16);
        // D s_6_25: read-var Vn:u8
        let s_6_25: u8 = fn_state.Vn;
        // D s_6_26: cast zx s_6_25 -> bv
        let s_6_26: Bits = Bits::new(s_6_25 as u128, 4u16);
        // D s_6_27: cast reint s_6_24 -> u128
        let s_6_27: u128 = (s_6_24.value() as u128);
        // D s_6_28: size-of s_6_24
        let s_6_28: u16 = s_6_24.length();
        // D s_6_29: cast reint s_6_26 -> u128
        let s_6_29: u128 = (s_6_26.value() as u128);
        // D s_6_30: size-of s_6_26
        let s_6_30: u16 = s_6_26.length();
        // D s_6_31: lsl s_6_27 s_6_30
        let s_6_31: u128 = s_6_27 << s_6_30;
        // D s_6_32: or s_6_31 s_6_29
        let s_6_32: u128 = ((s_6_31) | (s_6_29));
        // D s_6_33: add s_6_28 s_6_30
        let s_6_33: u16 = (s_6_28 + s_6_30);
        // D s_6_34: create-bits s_6_32 s_6_33
        let s_6_34: Bits = Bits::new(s_6_32, s_6_33);
        // D s_6_35: cast reint s_6_34 -> u8
        let s_6_35: u8 = (s_6_34.value() as u8);
        // D s_6_36: cast zx s_6_35 -> bv
        let s_6_36: Bits = Bits::new(s_6_35 as u128, 5u16);
        // D s_6_37: cast zx s_6_36 -> i
        let s_6_37: i128 = (s_6_36.value() as i128);
        // D s_6_38: cast reint s_6_37 -> i64
        let s_6_38: i64 = (s_6_37 as i64);
        // D s_6_39: write-var n <= s_6_38
        fn_state.n = s_6_38;
        // C s_6_40: const #8s : i64
        let s_6_40: i64 = 8;
        // D s_6_41: write-var esize <= s_6_40
        fn_state.esize = s_6_40;
        // D s_6_42: read-var size:u8
        let s_6_42: u8 = fn_state.size;
        // D s_6_43: cast zx s_6_42 -> bv
        let s_6_43: Bits = Bits::new(s_6_42 as u128, 2u16);
        // C s_6_44: const #1u : u8
        let s_6_44: u8 = 1;
        // C s_6_45: cast zx s_6_44 -> bv
        let s_6_45: Bits = Bits::new(s_6_44 as u128, 2u16);
        // D s_6_46: cmp-eq s_6_43 s_6_45
        let s_6_46: bool = ((s_6_43) == (s_6_45));
        // N s_6_47: branch s_6_46 b39 b7
        if s_6_46 {
            return block_39(state, tracer, fn_state);
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
        // N s_8_5: branch s_8_4 b38 b9
        if s_8_4 {
            return block_38(state, tracer, fn_state);
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
        // D s_10_1: write-var mshadow#7654 <= s_10_0
        fn_state.mshadow_7654 = s_10_0;
        // D s_10_2: read-var index:i
        let s_10_2: i128 = fn_state.index;
        // D s_10_3: write-var indexshadow#7655 <= s_10_2
        fn_state.indexshadow_7655 = s_10_2;
        // D s_10_4: read-var esize:i64
        let s_10_4: i64 = fn_state.esize;
        // D s_10_5: write-var esizeshadow#7656 <= s_10_4
        fn_state.esizeshadow_7656 = s_10_4;
        // D s_10_6: read-var elements:i
        let s_10_6: i128 = fn_state.elements;
        // D s_10_7: write-var elementsshadow#7657 <= s_10_6
        fn_state.elementsshadow_7657 = s_10_6;
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
        // D s_11_1: write-var gs#315527 <= s_11_0
        fn_state.gs_315527 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#315527:u8
        let s_12_0: bool = fn_state.gs_315527;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var esizeshadow#7656:i64
        let s_12_2: i64 = fn_state.esizeshadow_7656;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // D s_12_5: read-var mshadow#7654:i
        let s_12_5: i128 = fn_state.mshadow_7654;
        // D s_12_6: cast reint s_12_5 -> i64
        let s_12_6: i64 = (s_12_5 as i64);
        // D s_12_7: read-var add:u8
        let s_12_7: bool = fn_state.add;
        // D s_12_8: read-var d:i64
        let s_12_8: i64 = fn_state.d;
        // D s_12_9: read-var elementsshadow#7657:i
        let s_12_9: i128 = fn_state.elementsshadow_7657;
        // D s_12_10: read-var indexshadow#7655:i
        let s_12_10: i128 = fn_state.indexshadow_7655;
        // D s_12_11: read-var n:i64
        let s_12_11: i64 = fn_state.n;
        // C s_12_12: const #1u : u8
        let s_12_12: bool = true;
        // D s_12_13: call execute_aarch32_instrs_VQDMLAL_Op_A_txt(s_12_7, s_12_8, s_12_9, s_12_4, s_12_10, s_12_6, s_12_11, s_12_12)
        let s_12_13: () = execute_aarch32_instrs_VQDMLAL_Op_A_txt(
            state,
            tracer,
            s_12_7,
            s_12_8,
            s_12_9,
            s_12_4,
            s_12_10,
            s_12_6,
            s_12_11,
            s_12_12,
        );
        // N s_12_14: return
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
        // D s_14_1: write-var gs#315526 <= s_14_0
        fn_state.gs_315526 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#315526:u8
        let s_15_0: bool = fn_state.gs_315526;
        // D s_15_1: write-var gs#315527 <= s_15_0
        fn_state.gs_315527 = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var mshadow#7654:i
        let s_16_0: i128 = fn_state.mshadow_7654;
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
        // D s_17_1: write-var gs#315525 <= s_17_0
        fn_state.gs_315525 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#315525:u8
        let s_18_0: bool = fn_state.gs_315525;
        // D s_18_1: write-var gs#315526 <= s_18_0
        fn_state.gs_315526 = s_18_0;
        // N s_18_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var mshadow#7654:i
        let s_19_0: i128 = fn_state.mshadow_7654;
        // D s_19_1: call __id(s_19_0)
        let s_19_1: i128 = u__id(state, tracer, s_19_0);
        // C s_19_2: const #31s : i
        let s_19_2: i128 = 31;
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
        // D s_20_1: write-var gs#315524 <= s_20_0
        fn_state.gs_315524 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#315524:u8
        let s_21_0: bool = fn_state.gs_315524;
        // D s_21_1: write-var gs#315525 <= s_21_0
        fn_state.gs_315525 = s_21_0;
        // N s_21_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var esizeshadow#7656:i64
        let s_22_0: i64 = fn_state.esizeshadow_7656;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: call __id(s_22_1)
        let s_22_2: i128 = u__id(state, tracer, s_22_1);
        // D s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: const #8s : i
        let s_22_4: i128 = 8;
        // D s_22_5: cast zx s_22_3 -> i
        let s_22_5: i128 = (i128::try_from(s_22_3).unwrap());
        // D s_22_6: cmp-eq s_22_5 s_22_4
        let s_22_6: bool = ((s_22_5) == (s_22_4));
        // N s_22_7: branch s_22_6 b37 b23
        if s_22_6 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var esizeshadow#7656:i64
        let s_23_0: i64 = fn_state.esizeshadow_7656;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: call __id(s_23_1)
        let s_23_2: i128 = u__id(state, tracer, s_23_1);
        // D s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: const #16s : i
        let s_23_4: i128 = 16;
        // D s_23_5: cast zx s_23_3 -> i
        let s_23_5: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_6: cmp-eq s_23_5 s_23_4
        let s_23_6: bool = ((s_23_5) == (s_23_4));
        // D s_23_7: write-var gs#315515 <= s_23_6
        fn_state.gs_315515 = s_23_6;
        // N s_23_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#315515:u8
        let s_24_0: bool = fn_state.gs_315515;
        // N s_24_1: branch s_24_0 b36 b25
        if s_24_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var esizeshadow#7656:i64
        let s_25_0: i64 = fn_state.esizeshadow_7656;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: call __id(s_25_1)
        let s_25_2: i128 = u__id(state, tracer, s_25_1);
        // D s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: const #32s : i
        let s_25_4: i128 = 32;
        // D s_25_5: cast zx s_25_3 -> i
        let s_25_5: i128 = (i128::try_from(s_25_3).unwrap());
        // D s_25_6: cmp-eq s_25_5 s_25_4
        let s_25_6: bool = ((s_25_5) == (s_25_4));
        // D s_25_7: write-var gs#315517 <= s_25_6
        fn_state.gs_315517 = s_25_6;
        // N s_25_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#315517:u8
        let s_26_0: bool = fn_state.gs_315517;
        // N s_26_1: branch s_26_0 b35 b27
        if s_26_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var esizeshadow#7656:i64
        let s_27_0: i64 = fn_state.esizeshadow_7656;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: call __id(s_27_1)
        let s_27_2: i128 = u__id(state, tracer, s_27_1);
        // D s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: const #64s : i
        let s_27_4: i128 = 64;
        // D s_27_5: cast zx s_27_3 -> i
        let s_27_5: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_6: cmp-eq s_27_5 s_27_4
        let s_27_6: bool = ((s_27_5) == (s_27_4));
        // D s_27_7: write-var gs#315519 <= s_27_6
        fn_state.gs_315519 = s_27_6;
        // N s_27_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#315519:u8
        let s_28_0: bool = fn_state.gs_315519;
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
        // D s_29_1: write-var gs#315523 <= s_29_0
        fn_state.gs_315523 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#315523:u8
        let s_30_0: bool = fn_state.gs_315523;
        // D s_30_1: write-var gs#315524 <= s_30_0
        fn_state.gs_315524 = s_30_0;
        // N s_30_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var d:i64
        let s_31_0: i64 = fn_state.d;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: call __id(s_31_1)
        let s_31_2: i128 = u__id(state, tracer, s_31_1);
        // D s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: const #0s : i
        let s_31_4: i128 = 0;
        // D s_31_5: cast zx s_31_3 -> i
        let s_31_5: i128 = (i128::try_from(s_31_3).unwrap());
        // D s_31_6: cmp-le s_31_4 s_31_5
        let s_31_6: bool = ((s_31_4) <= (s_31_5));
        // N s_31_7: branch s_31_6 b34 b32
        if s_31_6 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#315522 <= s_32_0
        fn_state.gs_315522 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#315522:u8
        let s_33_0: bool = fn_state.gs_315522;
        // D s_33_1: write-var gs#315523 <= s_33_0
        fn_state.gs_315523 = s_33_0;
        // N s_33_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var d:i64
        let s_34_0: i64 = fn_state.d;
        // D s_34_1: cast zx s_34_0 -> i
        let s_34_1: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_2: call __id(s_34_1)
        let s_34_2: i128 = u__id(state, tracer, s_34_1);
        // D s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: const #31s : i
        let s_34_4: i128 = 31;
        // D s_34_5: cast zx s_34_3 -> i
        let s_34_5: i128 = (i128::try_from(s_34_3).unwrap());
        // D s_34_6: cmp-le s_34_5 s_34_4
        let s_34_6: bool = ((s_34_5) <= (s_34_4));
        // D s_34_7: write-var gs#315522 <= s_34_6
        fn_state.gs_315522 = s_34_6;
        // N s_34_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#315519 <= s_35_0
        fn_state.gs_315519 = s_35_0;
        // N s_35_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#315517 <= s_36_0
        fn_state.gs_315517 = s_36_0;
        // N s_36_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#315515 <= s_37_0
        fn_state.gs_315515 = s_37_0;
        // N s_37_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #32s : i64
        let s_38_0: i64 = 32;
        // D s_38_1: write-var esize <= s_38_0
        fn_state.esize = s_38_0;
        // C s_38_2: const #2s : i
        let s_38_2: i128 = 2;
        // D s_38_3: write-var elements <= s_38_2
        fn_state.elements = s_38_2;
        // D s_38_4: read-var Vm:u8
        let s_38_4: u8 = fn_state.Vm;
        // D s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 4u16);
        // D s_38_6: cast zx s_38_5 -> i
        let s_38_6: i128 = (s_38_5.value() as i128);
        // D s_38_7: write-var m <= s_38_6
        fn_state.m = s_38_6;
        // D s_38_8: read-var M:u8
        let s_38_8: bool = fn_state.M;
        // D s_38_9: cast zx s_38_8 -> bv
        let s_38_9: Bits = Bits::new(s_38_8 as u128, 1u16);
        // D s_38_10: cast zx s_38_9 -> i
        let s_38_10: i128 = (s_38_9.value() as i128);
        // D s_38_11: write-var index <= s_38_10
        fn_state.index = s_38_10;
        // N s_38_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #16s : i64
        let s_39_0: i64 = 16;
        // D s_39_1: write-var esize <= s_39_0
        fn_state.esize = s_39_0;
        // C s_39_2: const #4s : i
        let s_39_2: i128 = 4;
        // D s_39_3: write-var elements <= s_39_2
        fn_state.elements = s_39_2;
        // C s_39_4: const #0s : i
        let s_39_4: i128 = 0;
        // D s_39_5: read-var Vm:u8
        let s_39_5: u8 = fn_state.Vm;
        // D s_39_6: cast zx s_39_5 -> bv
        let s_39_6: Bits = Bits::new(s_39_5 as u128, 4u16);
        // C s_39_7: const #1s : i64
        let s_39_7: i64 = 1;
        // C s_39_8: cast zx s_39_7 -> i
        let s_39_8: i128 = (i128::try_from(s_39_7).unwrap());
        // C s_39_9: const #2s : i
        let s_39_9: i128 = 2;
        // C s_39_10: add s_39_9 s_39_8
        let s_39_10: i128 = (s_39_9 + s_39_8);
        // D s_39_11: bit-extract s_39_6 s_39_4 s_39_10
        let s_39_11: Bits = (Bits::new(
            ((s_39_6) >> (s_39_4)).value(),
            u16::try_from(s_39_10).unwrap(),
        ));
        // D s_39_12: cast reint s_39_11 -> u8
        let s_39_12: u8 = (s_39_11.value() as u8);
        // D s_39_13: cast zx s_39_12 -> bv
        let s_39_13: Bits = Bits::new(s_39_12 as u128, 3u16);
        // D s_39_14: cast zx s_39_13 -> i
        let s_39_14: i128 = (s_39_13.value() as i128);
        // D s_39_15: write-var m <= s_39_14
        fn_state.m = s_39_14;
        // C s_39_16: const #3s : i
        let s_39_16: i128 = 3;
        // D s_39_17: read-var Vm:u8
        let s_39_17: u8 = fn_state.Vm;
        // D s_39_18: cast zx s_39_17 -> bv
        let s_39_18: Bits = Bits::new(s_39_17 as u128, 4u16);
        // C s_39_19: const #1u : u64
        let s_39_19: u64 = 1;
        // D s_39_20: bit-extract s_39_18 s_39_16 s_39_19
        let s_39_20: Bits = (Bits::new(
            ((s_39_18) >> (s_39_16)).value(),
            u16::try_from(s_39_19).unwrap(),
        ));
        // D s_39_21: cast reint s_39_20 -> u8
        let s_39_21: bool = ((s_39_20.value()) != 0);
        // C s_39_22: const #0s : i
        let s_39_22: i128 = 0;
        // C s_39_23: const #0u : u64
        let s_39_23: u64 = 0;
        // D s_39_24: cast zx s_39_21 -> u64
        let s_39_24: u64 = (s_39_21 as u64);
        // C s_39_25: const #1u : u64
        let s_39_25: u64 = 1;
        // D s_39_26: and s_39_24 s_39_25
        let s_39_26: u64 = ((s_39_24) & (s_39_25));
        // D s_39_27: cmp-eq s_39_26 s_39_25
        let s_39_27: bool = ((s_39_26) == (s_39_25));
        // D s_39_28: lsl s_39_24 s_39_22
        let s_39_28: u64 = s_39_24 << s_39_22;
        // D s_39_29: or s_39_23 s_39_28
        let s_39_29: u64 = ((s_39_23) | (s_39_28));
        // D s_39_30: cmpl s_39_28
        let s_39_30: u64 = !s_39_28;
        // D s_39_31: and s_39_23 s_39_30
        let s_39_31: u64 = ((s_39_23) & (s_39_30));
        // D s_39_32: select s_39_27 s_39_29 s_39_31
        let s_39_32: u64 = if s_39_27 { s_39_29 } else { s_39_31 };
        // D s_39_33: cast trunc s_39_32 -> u8
        let s_39_33: bool = ((s_39_32) != 0);
        // D s_39_34: read-var M:u8
        let s_39_34: bool = fn_state.M;
        // D s_39_35: cast zx s_39_34 -> bv
        let s_39_35: Bits = Bits::new(s_39_34 as u128, 1u16);
        // D s_39_36: cast zx s_39_33 -> bv
        let s_39_36: Bits = Bits::new(s_39_33 as u128, 1u16);
        // D s_39_37: cast reint s_39_35 -> u128
        let s_39_37: u128 = (s_39_35.value() as u128);
        // D s_39_38: size-of s_39_35
        let s_39_38: u16 = s_39_35.length();
        // D s_39_39: cast reint s_39_36 -> u128
        let s_39_39: u128 = (s_39_36.value() as u128);
        // D s_39_40: size-of s_39_36
        let s_39_40: u16 = s_39_36.length();
        // D s_39_41: lsl s_39_37 s_39_40
        let s_39_41: u128 = s_39_37 << s_39_40;
        // D s_39_42: or s_39_41 s_39_39
        let s_39_42: u128 = ((s_39_41) | (s_39_39));
        // D s_39_43: add s_39_38 s_39_40
        let s_39_43: u16 = (s_39_38 + s_39_40);
        // D s_39_44: create-bits s_39_42 s_39_43
        let s_39_44: Bits = Bits::new(s_39_42, s_39_43);
        // D s_39_45: cast reint s_39_44 -> u8
        let s_39_45: u8 = (s_39_44.value() as u8);
        // D s_39_46: cast zx s_39_45 -> bv
        let s_39_46: Bits = Bits::new(s_39_45 as u128, 2u16);
        // D s_39_47: cast zx s_39_46 -> i
        let s_39_47: i128 = (s_39_46.value() as i128);
        // D s_39_48: write-var index <= s_39_47
        fn_state.index = s_39_47;
        // N s_39_49: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_40_0: panic
        panic!("{:?}", ());
        // N s_40_1: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#315483 <= s_41_0
        fn_state.gs_315483 = s_41_0;
        // N s_41_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: panic
        panic!("{:?}", ());
        // N s_42_1: return
        return;
    }
}
