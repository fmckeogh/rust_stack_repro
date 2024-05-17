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
use execute_aarch64_instrs_integer_pac_pacib_dp_1src::*;
use BTypeCompatible_PACIXSP::*;
use HaveBTIExt::*;
use u__id::*;
use SetBTypeCompatible::*;
use common::*;
pub fn decode_pacib_aarch64_instrs_integer_pac_pacib_hint<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op2: u8,
    CRm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_165689: bool,
        nshadow_1777: i128,
        n: i128,
        ga_263879: u8,
        d: i64,
        dshadow_1778: i64,
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
        // D s_0_17: write-var ga#263879 <= s_0_16
        fn_state.ga_263879 = s_0_16;
        // D s_0_18: read-var ga#263879:u8
        let s_0_18: u8 = fn_state.ga_263879;
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 7u16);
        // C s_0_20: const #26u : u8
        let s_0_20: u8 = 26;
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
        // D s_2_1: write-var nshadow#1777 <= s_2_0
        fn_state.nshadow_1777 = s_2_0;
        // D s_2_2: read-var d:i64
        let s_2_2: i64 = fn_state.d;
        // D s_2_3: write-var dshadow#1778 <= s_2_2
        fn_state.dshadow_1778 = s_2_2;
        // D s_2_4: read-var nshadow#1777:i
        let s_2_4: i128 = fn_state.nshadow_1777;
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
        // D s_3_1: write-var gs#165689 <= s_3_0
        fn_state.gs_165689 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#165689:u8
        let s_4_0: bool = fn_state.gs_165689;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // D s_4_2: read-var nshadow#1777:i
        let s_4_2: i128 = fn_state.nshadow_1777;
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: read-var dshadow#1778:i64
        let s_4_4: i64 = fn_state.dshadow_1778;
        // D s_4_5: read-var source_is_sp:u8
        let s_4_5: bool = fn_state.source_is_sp;
        // D s_4_6: call execute_aarch64_instrs_integer_pac_pacib_dp_1src(s_4_4, s_4_3, s_4_5)
        let s_4_6: () = execute_aarch64_instrs_integer_pac_pacib_dp_1src(
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
        // D s_5_0: read-var nshadow#1777:i
        let s_5_0: i128 = fn_state.nshadow_1777;
        // D s_5_1: call __id(s_5_0)
        let s_5_1: i128 = u__id(state, tracer, s_5_0);
        // C s_5_2: const #31s : i
        let s_5_2: i128 = 31;
        // D s_5_3: cmp-le s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) <= (s_5_2));
        // D s_5_4: write-var gs#165689 <= s_5_3
        fn_state.gs_165689 = s_5_3;
        // N s_5_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#263879:u8
        let s_6_0: u8 = fn_state.ga_263879;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 7u16);
        // C s_6_2: const #27u : u8
        let s_6_2: u8 = 27;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 7u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b11 b7
        if s_6_5 {
            return block_11(state, tracer, fn_state);
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
        // C s_7_4: const #() : ()
        let s_7_4: () = ();
        // S s_7_5: call HaveBTIExt(s_7_4)
        let s_7_5: bool = HaveBTIExt(state, tracer, s_7_4);
        // N s_7_6: branch s_7_5 b10 b8
        if s_7_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call BTypeCompatible_PACIXSP(s_10_0)
        let s_10_1: bool = BTypeCompatible_PACIXSP(state, tracer, s_10_0);
        // S s_10_2: call SetBTypeCompatible(s_10_1)
        let s_10_2: () = SetBTypeCompatible(state, tracer, s_10_1);
        // N s_10_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#263879:u8
        let s_11_0: u8 = fn_state.ga_263879;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 7u16);
        // C s_11_2: const #10u : u8
        let s_11_2: u8 = 10;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 7u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: not s_11_4
        let s_11_5: bool = !s_11_4;
        // N s_11_6: branch s_11_5 b13 b12
        if s_11_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #17s : i64
        let s_12_0: i64 = 17;
        // D s_12_1: write-var d <= s_12_0
        fn_state.d = s_12_0;
        // C s_12_2: const #16s : i
        let s_12_2: i128 = 16;
        // D s_12_3: write-var n <= s_12_2
        fn_state.n = s_12_2;
        // N s_12_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var ga#263879:u8
        let s_13_0: u8 = fn_state.ga_263879;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 7u16);
        // C s_13_2: const #8u : u8
        let s_13_2: u8 = 8;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 7u16);
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
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ga#263879:u8
        let s_15_0: u8 = fn_state.ga_263879;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 7u16);
        // C s_15_2: const #12u : u8
        let s_15_2: u8 = 12;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 7u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: not s_15_4
        let s_15_5: bool = !s_15_4;
        // N s_15_6: branch s_15_5 b17 b16
        if s_15_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
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
        // D s_17_0: read-var ga#263879:u8
        let s_17_0: u8 = fn_state.ga_263879;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 7u16);
        // C s_17_2: const #14u : u8
        let s_17_2: u8 = 14;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 7u16);
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
        // D s_19_0: read-var ga#263879:u8
        let s_19_0: u8 = fn_state.ga_263879;
        // C s_19_1: const #1s : i
        let s_19_1: i128 = 1;
        // D s_19_2: cast zx s_19_0 -> bv
        let s_19_2: Bits = Bits::new(s_19_0 as u128, 7u16);
        // C s_19_3: const #1s : i64
        let s_19_3: i64 = 1;
        // C s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (i128::try_from(s_19_3).unwrap());
        // C s_19_5: const #5s : i
        let s_19_5: i128 = 5;
        // C s_19_6: add s_19_5 s_19_4
        let s_19_6: i128 = (s_19_5 + s_19_4);
        // D s_19_7: bit-extract s_19_2 s_19_1 s_19_6
        let s_19_7: Bits = (Bits::new(
            ((s_19_2) >> (s_19_1)).value(),
            u16::try_from(s_19_6).unwrap(),
        ));
        // D s_19_8: cast reint s_19_7 -> u8
        let s_19_8: u8 = (s_19_7.value() as u8);
        // D s_19_9: cast zx s_19_8 -> bv
        let s_19_9: Bits = Bits::new(s_19_8 as u128, 6u16);
        // C s_19_10: const #12u : u8
        let s_19_10: u8 = 12;
        // C s_19_11: cast zx s_19_10 -> bv
        let s_19_11: Bits = Bits::new(s_19_10 as u128, 6u16);
        // D s_19_12: cmp-eq s_19_9 s_19_11
        let s_19_12: bool = ((s_19_9) == (s_19_11));
        // D s_19_13: not s_19_12
        let s_19_13: bool = !s_19_12;
        // N s_19_14: branch s_19_13 b21 b20
        if s_19_13 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
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
        // D s_21_0: read-var ga#263879:u8
        let s_21_0: u8 = fn_state.ga_263879;
        // C s_21_1: const #1s : i
        let s_21_1: i128 = 1;
        // D s_21_2: cast zx s_21_0 -> bv
        let s_21_2: Bits = Bits::new(s_21_0 as u128, 7u16);
        // C s_21_3: const #1s : i64
        let s_21_3: i64 = 1;
        // C s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // C s_21_5: const #5s : i
        let s_21_5: i128 = 5;
        // C s_21_6: add s_21_5 s_21_4
        let s_21_6: i128 = (s_21_5 + s_21_4);
        // D s_21_7: bit-extract s_21_2 s_21_1 s_21_6
        let s_21_7: Bits = (Bits::new(
            ((s_21_2) >> (s_21_1)).value(),
            u16::try_from(s_21_6).unwrap(),
        ));
        // D s_21_8: cast reint s_21_7 -> u8
        let s_21_8: u8 = (s_21_7.value() as u8);
        // D s_21_9: cast zx s_21_8 -> bv
        let s_21_9: Bits = Bits::new(s_21_8 as u128, 6u16);
        // C s_21_10: const #14u : u8
        let s_21_10: u8 = 14;
        // C s_21_11: cast zx s_21_10 -> bv
        let s_21_11: Bits = Bits::new(s_21_10 as u128, 6u16);
        // D s_21_12: cmp-eq s_21_9 s_21_11
        let s_21_12: bool = ((s_21_9) == (s_21_11));
        // D s_21_13: not s_21_12
        let s_21_13: bool = !s_21_12;
        // N s_21_14: branch s_21_13 b23 b22
        if s_21_13 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
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
        // D s_23_0: read-var ga#263879:u8
        let s_23_0: u8 = fn_state.ga_263879;
        // C s_23_1: const #1s : i
        let s_23_1: i128 = 1;
        // D s_23_2: cast zx s_23_0 -> bv
        let s_23_2: Bits = Bits::new(s_23_0 as u128, 7u16);
        // C s_23_3: const #1s : i64
        let s_23_3: i64 = 1;
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #5s : i
        let s_23_5: i128 = 5;
        // C s_23_6: add s_23_5 s_23_4
        let s_23_6: i128 = (s_23_5 + s_23_4);
        // D s_23_7: bit-extract s_23_2 s_23_1 s_23_6
        let s_23_7: Bits = (Bits::new(
            ((s_23_2) >> (s_23_1)).value(),
            u16::try_from(s_23_6).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: u8 = (s_23_7.value() as u8);
        // D s_23_9: cast zx s_23_8 -> bv
        let s_23_9: Bits = Bits::new(s_23_8 as u128, 6u16);
        // C s_23_10: const #15u : u8
        let s_23_10: u8 = 15;
        // C s_23_11: cast zx s_23_10 -> bv
        let s_23_11: Bits = Bits::new(s_23_10 as u128, 6u16);
        // D s_23_12: cmp-eq s_23_9 s_23_11
        let s_23_12: bool = ((s_23_9) == (s_23_11));
        // D s_23_13: not s_23_12
        let s_23_13: bool = !s_23_12;
        // N s_23_14: branch s_23_13 b25 b24
        if s_23_13 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
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
        // D s_25_0: read-var ga#263879:u8
        let s_25_0: u8 = fn_state.ga_263879;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 7u16);
        // C s_25_2: const #7u : u8
        let s_25_2: u8 = 7;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 7u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: not s_25_4
        let s_25_5: bool = !s_25_4;
        // N s_25_6: branch s_25_5 b27 b26
        if s_25_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
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
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
}
