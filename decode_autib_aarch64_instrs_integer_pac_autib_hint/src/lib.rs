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
use execute_aarch64_instrs_integer_pac_autib_dp_1src::*;
use u__id::*;
use common::*;
pub fn decode_autib_aarch64_instrs_integer_pac_autib_hint<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op2: u8,
    CRm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_143919: bool,
        n: i128,
        ga_249711: u8,
        d: i64,
        nshadow_1070: i128,
        dshadow_1071: i64,
        source_is_sp: bool,
        op2: u8,
        CRm: u8,
    }
    let fn_state = FunctionState {
        op2,
        CRm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #17s : i64
        let s_0_0: i64 = 17;
        // D s_0_1: write-var d <= s_0_0
        fn_state.d = s_0_0;
        // C s_0_2: const #0u : u8
        let s_0_2: bool = false;
        // D s_0_3: write-var source_is_sp <= s_0_2
        fn_state.source_is_sp = s_0_2;
        // D s_0_4: read-var CRm:u8
        let s_0_4: u8 = fn_state.CRm;
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 4u16);
        // D s_0_6: read-var op2:u8
        let s_0_6: u8 = fn_state.op2;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 3u16);
        // D s_0_8: cast reint s_0_5 -> u128
        let s_0_8: u128 = (s_0_5.value() as u128);
        // D s_0_9: size-of s_0_5
        let s_0_9: u16 = s_0_5.length();
        // D s_0_10: cast reint s_0_7 -> u128
        let s_0_10: u128 = (s_0_7.value() as u128);
        // D s_0_11: size-of s_0_7
        let s_0_11: u16 = s_0_7.length();
        // D s_0_12: lsl s_0_8 s_0_11
        let s_0_12: u128 = s_0_8 << s_0_11;
        // D s_0_13: or s_0_12 s_0_10
        let s_0_13: u128 = ((s_0_12) | (s_0_10));
        // D s_0_14: add s_0_9 s_0_11
        let s_0_14: u16 = (s_0_9 + s_0_11);
        // D s_0_15: create-bits s_0_13 s_0_14
        let s_0_15: Bits = Bits::new(s_0_13, s_0_14);
        // D s_0_16: cast reint s_0_15 -> u8
        let s_0_16: u8 = (s_0_15.value() as u8);
        // D s_0_17: write-var ga#249711 <= s_0_16
        fn_state.ga_249711 = s_0_16;
        // D s_0_18: read-var ga#249711:u8
        let s_0_18: u8 = fn_state.ga_249711;
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 7u16);
        // C s_0_20: const #30u : u8
        let s_0_20: u8 = 30;
        // C s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 7u16);
        // D s_0_22: cmp-eq s_0_19 s_0_21
        let s_0_22: bool = ((s_0_19) == (s_0_21));
        // D s_0_23: not s_0_22
        let s_0_23: bool = !s_0_22;
        // N s_0_24: branch s_0_23 b6 b1
        if s_0_23 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #30s : i64
        let s_1_0: i64 = 30;
        // D s_1_1: write-var d <= s_1_0
        fn_state.d = s_1_0;
        // C s_1_2: const #31s : i
        let s_1_2: i128 = 31;
        // D s_1_3: write-var n <= s_1_2
        fn_state.n = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var n:i
        let s_2_0: i128 = fn_state.n;
        // D s_2_1: write-var nshadow#1070 <= s_2_0
        fn_state.nshadow_1070 = s_2_0;
        // D s_2_2: read-var d:i64
        let s_2_2: i64 = fn_state.d;
        // D s_2_3: write-var dshadow#1071 <= s_2_2
        fn_state.dshadow_1071 = s_2_2;
        // D s_2_4: read-var nshadow#1070:i
        let s_2_4: i128 = fn_state.nshadow_1070;
        // D s_2_5: call __id(s_2_4)
        let s_2_5: i128 = u__id(state, tracer, s_2_4);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // D s_2_7: cmp-le s_2_6 s_2_5
        let s_2_7: bool = ((s_2_6) <= (s_2_5));
        // N s_2_8: branch s_2_7 b5 b3
        if s_2_7 {
            return block_5(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#143919 <= s_3_0
        fn_state.gs_143919 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#143919:u8
        let s_4_0: bool = fn_state.gs_143919;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // D s_4_2: read-var nshadow#1070:i
        let s_4_2: i128 = fn_state.nshadow_1070;
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: read-var dshadow#1071:i64
        let s_4_4: i64 = fn_state.dshadow_1071;
        // D s_4_5: read-var source_is_sp:u8
        let s_4_5: bool = fn_state.source_is_sp;
        // D s_4_6: call execute_aarch64_instrs_integer_pac_autib_dp_1src(s_4_4, s_4_3, s_4_5)
        let s_4_6: () = execute_aarch64_instrs_integer_pac_autib_dp_1src(
            state,
            tracer,
            s_4_4,
            s_4_3,
            s_4_5,
        );
        // N s_4_7: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var nshadow#1070:i
        let s_5_0: i128 = fn_state.nshadow_1070;
        // D s_5_1: call __id(s_5_0)
        let s_5_1: i128 = u__id(state, tracer, s_5_0);
        // C s_5_2: const #31s : i
        let s_5_2: i128 = 31;
        // D s_5_3: cmp-le s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) <= (s_5_2));
        // D s_5_4: write-var gs#143919 <= s_5_3
        fn_state.gs_143919 = s_5_3;
        // N s_5_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#249711:u8
        let s_6_0: u8 = fn_state.ga_249711;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 7u16);
        // C s_6_2: const #31u : u8
        let s_6_2: u8 = 31;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 7u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #30s : i64
        let s_7_0: i64 = 30;
        // D s_7_1: write-var d <= s_7_0
        fn_state.d = s_7_0;
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // D s_7_3: write-var source_is_sp <= s_7_2
        fn_state.source_is_sp = s_7_2;
        // N s_7_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#249711:u8
        let s_8_0: u8 = fn_state.ga_249711;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 7u16);
        // C s_8_2: const #14u : u8
        let s_8_2: u8 = 14;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 7u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #17s : i64
        let s_9_0: i64 = 17;
        // D s_9_1: write-var d <= s_9_0
        fn_state.d = s_9_0;
        // C s_9_2: const #16s : i
        let s_9_2: i128 = 16;
        // D s_9_3: write-var n <= s_9_2
        fn_state.n = s_9_2;
        // N s_9_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#249711:u8
        let s_10_0: u8 = fn_state.ga_249711;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 7u16);
        // C s_10_2: const #8u : u8
        let s_10_2: u8 = 8;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 7u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b12 b11
        if s_10_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#249711:u8
        let s_12_0: u8 = fn_state.ga_249711;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 7u16);
        // C s_12_2: const #10u : u8
        let s_12_2: u8 = 10;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 7u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b14 b13
        if s_12_5 {
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
        // D s_14_0: read-var ga#249711:u8
        let s_14_0: u8 = fn_state.ga_249711;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 7u16);
        // C s_14_2: const #12u : u8
        let s_14_2: u8 = 12;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 7u16);
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
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var ga#249711:u8
        let s_16_0: u8 = fn_state.ga_249711;
        // C s_16_1: const #1s : i
        let s_16_1: i128 = 1;
        // D s_16_2: cast zx s_16_0 -> bv
        let s_16_2: Bits = Bits::new(s_16_0 as u128, 7u16);
        // C s_16_3: const #1s : i64
        let s_16_3: i64 = 1;
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // C s_16_5: const #5s : i
        let s_16_5: i128 = 5;
        // C s_16_6: add s_16_5 s_16_4
        let s_16_6: i128 = (s_16_5 + s_16_4);
        // D s_16_7: bit-extract s_16_2 s_16_1 s_16_6
        let s_16_7: Bits = (Bits::new(
            ((s_16_2) >> (s_16_1)).value(),
            u16::try_from(s_16_6).unwrap(),
        ));
        // D s_16_8: cast reint s_16_7 -> u8
        let s_16_8: u8 = (s_16_7.value() as u8);
        // D s_16_9: cast zx s_16_8 -> bv
        let s_16_9: Bits = Bits::new(s_16_8 as u128, 6u16);
        // C s_16_10: const #12u : u8
        let s_16_10: u8 = 12;
        // C s_16_11: cast zx s_16_10 -> bv
        let s_16_11: Bits = Bits::new(s_16_10 as u128, 6u16);
        // D s_16_12: cmp-eq s_16_9 s_16_11
        let s_16_12: bool = ((s_16_9) == (s_16_11));
        // D s_16_13: not s_16_12
        let s_16_13: bool = !s_16_12;
        // N s_16_14: branch s_16_13 b18 b17
        if s_16_13 {
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
        // D s_18_0: read-var ga#249711:u8
        let s_18_0: u8 = fn_state.ga_249711;
        // C s_18_1: const #1s : i
        let s_18_1: i128 = 1;
        // D s_18_2: cast zx s_18_0 -> bv
        let s_18_2: Bits = Bits::new(s_18_0 as u128, 7u16);
        // C s_18_3: const #1s : i64
        let s_18_3: i64 = 1;
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // C s_18_5: const #5s : i
        let s_18_5: i128 = 5;
        // C s_18_6: add s_18_5 s_18_4
        let s_18_6: i128 = (s_18_5 + s_18_4);
        // D s_18_7: bit-extract s_18_2 s_18_1 s_18_6
        let s_18_7: Bits = (Bits::new(
            ((s_18_2) >> (s_18_1)).value(),
            u16::try_from(s_18_6).unwrap(),
        ));
        // D s_18_8: cast reint s_18_7 -> u8
        let s_18_8: u8 = (s_18_7.value() as u8);
        // D s_18_9: cast zx s_18_8 -> bv
        let s_18_9: Bits = Bits::new(s_18_8 as u128, 6u16);
        // C s_18_10: const #13u : u8
        let s_18_10: u8 = 13;
        // C s_18_11: cast zx s_18_10 -> bv
        let s_18_11: Bits = Bits::new(s_18_10 as u128, 6u16);
        // D s_18_12: cmp-eq s_18_9 s_18_11
        let s_18_12: bool = ((s_18_9) == (s_18_11));
        // D s_18_13: not s_18_12
        let s_18_13: bool = !s_18_12;
        // N s_18_14: branch s_18_13 b20 b19
        if s_18_13 {
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
        // D s_20_0: read-var ga#249711:u8
        let s_20_0: u8 = fn_state.ga_249711;
        // C s_20_1: const #1s : i
        let s_20_1: i128 = 1;
        // D s_20_2: cast zx s_20_0 -> bv
        let s_20_2: Bits = Bits::new(s_20_0 as u128, 7u16);
        // C s_20_3: const #1s : i64
        let s_20_3: i64 = 1;
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // C s_20_5: const #5s : i
        let s_20_5: i128 = 5;
        // C s_20_6: add s_20_5 s_20_4
        let s_20_6: i128 = (s_20_5 + s_20_4);
        // D s_20_7: bit-extract s_20_2 s_20_1 s_20_6
        let s_20_7: Bits = (Bits::new(
            ((s_20_2) >> (s_20_1)).value(),
            u16::try_from(s_20_6).unwrap(),
        ));
        // D s_20_8: cast reint s_20_7 -> u8
        let s_20_8: u8 = (s_20_7.value() as u8);
        // D s_20_9: cast zx s_20_8 -> bv
        let s_20_9: Bits = Bits::new(s_20_8 as u128, 6u16);
        // C s_20_10: const #14u : u8
        let s_20_10: u8 = 14;
        // C s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 6u16);
        // D s_20_12: cmp-eq s_20_9 s_20_11
        let s_20_12: bool = ((s_20_9) == (s_20_11));
        // D s_20_13: not s_20_12
        let s_20_13: bool = !s_20_12;
        // N s_20_14: branch s_20_13 b22 b21
        if s_20_13 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var ga#249711:u8
        let s_22_0: u8 = fn_state.ga_249711;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 7u16);
        // C s_22_2: const #7u : u8
        let s_22_2: u8 = 7;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 7u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: not s_22_4
        let s_22_5: bool = !s_22_4;
        // N s_22_6: branch s_22_5 b24 b23
        if s_22_5 {
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
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
}
