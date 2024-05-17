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
use execute_aarch32_instrs_VFMAL_i_Op_A_txt::*;
use HaveFP16MulNoRoundingToFP32Ext::*;
use u__id::*;
use common::*;
pub fn decode_aarch32_instrs_VFMAL_i_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    S: bool,
    Vn: u8,
    Vd: u8,
    N: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i128,
        ga_366927: i64,
        ga_366913: i128,
        gs_326757: bool,
        n: i128,
        index: i128,
        gs_326751: bool,
        ga_366929: i64,
        gs_326758: bool,
        gs_326724: bool,
        regs: i64,
        gs_326755: bool,
        gs_326759: bool,
        gs_326756: bool,
        d: i64,
        ga_366918: i128,
        sub_op: bool,
        gs_326753: bool,
        datasize: i64,
        ga_366925: i128,
        D: bool,
        S: bool,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        S,
        Vn,
        Vd,
        N,
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
        // S s_0_1: call HaveFP16MulNoRoundingToFP32Ext(s_0_0)
        let s_0_1: bool = HaveFP16MulNoRoundingToFP32Ext(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b43 b1
        if s_0_2 {
            return block_43(state, tracer, fn_state);
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
        // N s_1_5: branch s_1_4 b42 b2
        if s_1_4 {
            return block_42(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#326724 <= s_2_0
        fn_state.gs_326724 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#326724:u8
        let s_3_0: bool = fn_state.gs_326724;
        // N s_3_1: branch s_3_0 b41 b4
        if s_3_0 {
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
        // D s_4_0: read-var D:u8
        let s_4_0: bool = fn_state.D;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // D s_4_2: read-var Vd:u8
        let s_4_2: u8 = fn_state.Vd;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 4u16);
        // D s_4_4: cast reint s_4_1 -> u128
        let s_4_4: u128 = (s_4_1.value() as u128);
        // D s_4_5: size-of s_4_1
        let s_4_5: u16 = s_4_1.length();
        // D s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: lsl s_4_4 s_4_7
        let s_4_8: u128 = s_4_4 << s_4_7;
        // D s_4_9: or s_4_8 s_4_6
        let s_4_9: u128 = ((s_4_8) | (s_4_6));
        // D s_4_10: add s_4_5 s_4_7
        let s_4_10: u16 = (s_4_5 + s_4_7);
        // D s_4_11: create-bits s_4_9 s_4_10
        let s_4_11: Bits = Bits::new(s_4_9, s_4_10);
        // D s_4_12: cast reint s_4_11 -> u8
        let s_4_12: u8 = (s_4_11.value() as u8);
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 5u16);
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (s_4_13.value() as i128);
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: write-var d <= s_4_15
        fn_state.d = s_4_15;
        // D s_4_17: read-var Q:u8
        let s_4_17: bool = fn_state.Q;
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // C s_4_19: const #1u : u8
        let s_4_19: bool = true;
        // C s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 1u16);
        // D s_4_21: cmp-eq s_4_18 s_4_20
        let s_4_21: bool = ((s_4_18) == (s_4_20));
        // N s_4_22: branch s_4_21 b40 b5
        if s_4_21 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Vn:u8
        let s_5_0: u8 = fn_state.Vn;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // D s_5_2: read-var N:u8
        let s_5_2: bool = fn_state.N;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cast reint s_5_1 -> u128
        let s_5_4: u128 = (s_5_1.value() as u128);
        // D s_5_5: size-of s_5_1
        let s_5_5: u16 = s_5_1.length();
        // D s_5_6: cast reint s_5_3 -> u128
        let s_5_6: u128 = (s_5_3.value() as u128);
        // D s_5_7: size-of s_5_3
        let s_5_7: u16 = s_5_3.length();
        // D s_5_8: lsl s_5_4 s_5_7
        let s_5_8: u128 = s_5_4 << s_5_7;
        // D s_5_9: or s_5_8 s_5_6
        let s_5_9: u128 = ((s_5_8) | (s_5_6));
        // D s_5_10: add s_5_5 s_5_7
        let s_5_10: u16 = (s_5_5 + s_5_7);
        // D s_5_11: create-bits s_5_9 s_5_10
        let s_5_11: Bits = Bits::new(s_5_9, s_5_10);
        // D s_5_12: cast reint s_5_11 -> u8
        let s_5_12: u8 = (s_5_11.value() as u8);
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 5u16);
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (s_5_13.value() as i128);
        // D s_5_15: write-var ga#366913 <= s_5_14
        fn_state.ga_366913 = s_5_14;
        // N s_5_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#366913:i
        let s_6_0: i128 = fn_state.ga_366913;
        // D s_6_1: write-var n <= s_6_0
        fn_state.n = s_6_0;
        // D s_6_2: read-var Q:u8
        let s_6_2: bool = fn_state.Q;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #1u : u8
        let s_6_4: bool = true;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b39 b7
        if s_6_6 {
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
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: read-var Vm:u8
        let s_7_1: u8 = fn_state.Vm;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 4u16);
        // C s_7_3: const #1s : i64
        let s_7_3: i64 = 1;
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #2s : i
        let s_7_5: i128 = 2;
        // C s_7_6: add s_7_5 s_7_4
        let s_7_6: i128 = (s_7_5 + s_7_4);
        // D s_7_7: bit-extract s_7_2 s_7_0 s_7_6
        let s_7_7: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: u8 = (s_7_7.value() as u8);
        // D s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 3u16);
        // D s_7_10: read-var M:u8
        let s_7_10: bool = fn_state.M;
        // D s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 1u16);
        // D s_7_12: cast reint s_7_9 -> u128
        let s_7_12: u128 = (s_7_9.value() as u128);
        // D s_7_13: size-of s_7_9
        let s_7_13: u16 = s_7_9.length();
        // D s_7_14: cast reint s_7_11 -> u128
        let s_7_14: u128 = (s_7_11.value() as u128);
        // D s_7_15: size-of s_7_11
        let s_7_15: u16 = s_7_11.length();
        // D s_7_16: lsl s_7_12 s_7_15
        let s_7_16: u128 = s_7_12 << s_7_15;
        // D s_7_17: or s_7_16 s_7_14
        let s_7_17: u128 = ((s_7_16) | (s_7_14));
        // D s_7_18: add s_7_13 s_7_15
        let s_7_18: u16 = (s_7_13 + s_7_15);
        // D s_7_19: create-bits s_7_17 s_7_18
        let s_7_19: Bits = Bits::new(s_7_17, s_7_18);
        // D s_7_20: cast reint s_7_19 -> u8
        let s_7_20: u8 = (s_7_19.value() as u8);
        // D s_7_21: cast zx s_7_20 -> bv
        let s_7_21: Bits = Bits::new(s_7_20 as u128, 4u16);
        // D s_7_22: cast zx s_7_21 -> i
        let s_7_22: i128 = (s_7_21.value() as i128);
        // D s_7_23: write-var ga#366918 <= s_7_22
        fn_state.ga_366918 = s_7_22;
        // N s_7_24: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#366918:i
        let s_8_0: i128 = fn_state.ga_366918;
        // D s_8_1: write-var m <= s_8_0
        fn_state.m = s_8_0;
        // D s_8_2: read-var Q:u8
        let s_8_2: bool = fn_state.Q;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #1u : u8
        let s_8_4: bool = true;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // D s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // N s_8_7: branch s_8_6 b38 b9
        if s_8_6 {
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
        // C s_9_0: const #3s : i
        let s_9_0: i128 = 3;
        // D s_9_1: read-var Vm:u8
        let s_9_1: u8 = fn_state.Vm;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 4u16);
        // C s_9_3: const #1u : u64
        let s_9_3: u64 = 1;
        // D s_9_4: bit-extract s_9_2 s_9_0 s_9_3
        let s_9_4: Bits = (Bits::new(
            ((s_9_2) >> (s_9_0)).value(),
            u16::try_from(s_9_3).unwrap(),
        ));
        // D s_9_5: cast reint s_9_4 -> u8
        let s_9_5: bool = ((s_9_4.value()) != 0);
        // C s_9_6: const #0s : i
        let s_9_6: i128 = 0;
        // C s_9_7: const #0u : u64
        let s_9_7: u64 = 0;
        // D s_9_8: cast zx s_9_5 -> u64
        let s_9_8: u64 = (s_9_5 as u64);
        // C s_9_9: const #1u : u64
        let s_9_9: u64 = 1;
        // D s_9_10: and s_9_8 s_9_9
        let s_9_10: u64 = ((s_9_8) & (s_9_9));
        // D s_9_11: cmp-eq s_9_10 s_9_9
        let s_9_11: bool = ((s_9_10) == (s_9_9));
        // D s_9_12: lsl s_9_8 s_9_6
        let s_9_12: u64 = s_9_8 << s_9_6;
        // D s_9_13: or s_9_7 s_9_12
        let s_9_13: u64 = ((s_9_7) | (s_9_12));
        // D s_9_14: cmpl s_9_12
        let s_9_14: u64 = !s_9_12;
        // D s_9_15: and s_9_7 s_9_14
        let s_9_15: u64 = ((s_9_7) & (s_9_14));
        // D s_9_16: select s_9_11 s_9_13 s_9_15
        let s_9_16: u64 = if s_9_11 { s_9_13 } else { s_9_15 };
        // D s_9_17: cast trunc s_9_16 -> u8
        let s_9_17: bool = ((s_9_16) != 0);
        // D s_9_18: cast zx s_9_17 -> bv
        let s_9_18: Bits = Bits::new(s_9_17 as u128, 1u16);
        // D s_9_19: cast zx s_9_18 -> i
        let s_9_19: i128 = (s_9_18.value() as i128);
        // D s_9_20: write-var ga#366925 <= s_9_19
        fn_state.ga_366925 = s_9_19;
        // N s_9_21: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#366925:i
        let s_10_0: i128 = fn_state.ga_366925;
        // D s_10_1: write-var index <= s_10_0
        fn_state.index = s_10_0;
        // D s_10_2: read-var Q:u8
        let s_10_2: bool = fn_state.Q;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // N s_10_7: branch s_10_6 b37 b11
        if s_10_6 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1s : i64
        let s_11_0: i64 = 1;
        // D s_11_1: write-var ga#366927 <= s_11_0
        fn_state.ga_366927 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#366927:i64
        let s_12_0: i64 = fn_state.ga_366927;
        // D s_12_1: write-var regs <= s_12_0
        fn_state.regs = s_12_0;
        // D s_12_2: read-var Q:u8
        let s_12_2: bool = fn_state.Q;
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // C s_12_4: const #1u : u8
        let s_12_4: bool = true;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // D s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // N s_12_7: branch s_12_6 b36 b13
        if s_12_6 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #32s : i64
        let s_13_0: i64 = 32;
        // D s_13_1: write-var ga#366929 <= s_13_0
        fn_state.ga_366929 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var ga#366929:i64
        let s_14_0: i64 = fn_state.ga_366929;
        // D s_14_1: write-var datasize <= s_14_0
        fn_state.datasize = s_14_0;
        // D s_14_2: read-var S:u8
        let s_14_2: bool = fn_state.S;
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #1u : u8
        let s_14_4: bool = true;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // D s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // D s_14_7: write-var sub_op <= s_14_6
        fn_state.sub_op = s_14_6;
        // D s_14_8: read-var n:i
        let s_14_8: i128 = fn_state.n;
        // D s_14_9: call __id(s_14_8)
        let s_14_9: i128 = u__id(state, tracer, s_14_8);
        // C s_14_10: const #0s : i
        let s_14_10: i128 = 0;
        // D s_14_11: cmp-le s_14_10 s_14_9
        let s_14_11: bool = ((s_14_10) <= (s_14_9));
        // N s_14_12: branch s_14_11 b17 b15
        if s_14_11 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#326759 <= s_15_0
        fn_state.gs_326759 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#326759:u8
        let s_16_0: bool = fn_state.gs_326759;
        // N s_16_1: assert s_16_0
        let s_16_1: () = assert!(s_16_0);
        // D s_16_2: read-var datasize:i64
        let s_16_2: i64 = fn_state.datasize;
        // D s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_4: cast reint s_16_3 -> i64
        let s_16_4: i64 = (s_16_3 as i64);
        // C s_16_5: const #32s : i64
        let s_16_5: i64 = 32;
        // D s_16_6: read-var index:i
        let s_16_6: i128 = fn_state.index;
        // D s_16_7: cast reint s_16_6 -> i64
        let s_16_7: i64 = (s_16_6 as i64);
        // D s_16_8: read-var m:i
        let s_16_8: i128 = fn_state.m;
        // D s_16_9: cast reint s_16_8 -> i64
        let s_16_9: i64 = (s_16_8 as i64);
        // D s_16_10: read-var n:i
        let s_16_10: i128 = fn_state.n;
        // D s_16_11: cast reint s_16_10 -> i64
        let s_16_11: i64 = (s_16_10 as i64);
        // D s_16_12: read-var Q:u8
        let s_16_12: bool = fn_state.Q;
        // D s_16_13: read-var d:i64
        let s_16_13: i64 = fn_state.d;
        // D s_16_14: read-var regs:i64
        let s_16_14: i64 = fn_state.regs;
        // D s_16_15: read-var sub_op:u8
        let s_16_15: bool = fn_state.sub_op;
        // D s_16_16: call execute_aarch32_instrs_VFMAL_i_Op_A_txt(s_16_12, s_16_13, s_16_4, s_16_5, s_16_7, s_16_9, s_16_11, s_16_14, s_16_15)
        let s_16_16: () = execute_aarch32_instrs_VFMAL_i_Op_A_txt(
            state,
            tracer,
            s_16_12,
            s_16_13,
            s_16_4,
            s_16_5,
            s_16_7,
            s_16_9,
            s_16_11,
            s_16_14,
            s_16_15,
        );
        // N s_16_17: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var n:i
        let s_17_0: i128 = fn_state.n;
        // D s_17_1: call __id(s_17_0)
        let s_17_1: i128 = u__id(state, tracer, s_17_0);
        // C s_17_2: const #31s : i
        let s_17_2: i128 = 31;
        // D s_17_3: cmp-le s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) <= (s_17_2));
        // N s_17_4: branch s_17_3 b20 b18
        if s_17_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#326758 <= s_18_0
        fn_state.gs_326758 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#326758:u8
        let s_19_0: bool = fn_state.gs_326758;
        // D s_19_1: write-var gs#326759 <= s_19_0
        fn_state.gs_326759 = s_19_0;
        // N s_19_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var m:i
        let s_20_0: i128 = fn_state.m;
        // D s_20_1: call __id(s_20_0)
        let s_20_1: i128 = u__id(state, tracer, s_20_0);
        // C s_20_2: const #0s : i
        let s_20_2: i128 = 0;
        // D s_20_3: cmp-le s_20_2 s_20_1
        let s_20_3: bool = ((s_20_2) <= (s_20_1));
        // N s_20_4: branch s_20_3 b23 b21
        if s_20_3 {
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
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#326757 <= s_21_0
        fn_state.gs_326757 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#326757:u8
        let s_22_0: bool = fn_state.gs_326757;
        // D s_22_1: write-var gs#326758 <= s_22_0
        fn_state.gs_326758 = s_22_0;
        // N s_22_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var m:i
        let s_23_0: i128 = fn_state.m;
        // D s_23_1: call __id(s_23_0)
        let s_23_1: i128 = u__id(state, tracer, s_23_0);
        // C s_23_2: const #15s : i
        let s_23_2: i128 = 15;
        // D s_23_3: cmp-le s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) <= (s_23_2));
        // N s_23_4: branch s_23_3 b26 b24
        if s_23_3 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#326756 <= s_24_0
        fn_state.gs_326756 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#326756:u8
        let s_25_0: bool = fn_state.gs_326756;
        // D s_25_1: write-var gs#326757 <= s_25_0
        fn_state.gs_326757 = s_25_0;
        // N s_25_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var index:i
        let s_26_0: i128 = fn_state.index;
        // D s_26_1: call __id(s_26_0)
        let s_26_1: i128 = u__id(state, tracer, s_26_0);
        // C s_26_2: const #0s : i
        let s_26_2: i128 = 0;
        // D s_26_3: cmp-eq s_26_1 s_26_2
        let s_26_3: bool = ((s_26_1) == (s_26_2));
        // N s_26_4: branch s_26_3 b35 b27
        if s_26_3 {
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
        // D s_27_0: read-var index:i
        let s_27_0: i128 = fn_state.index;
        // D s_27_1: call __id(s_27_0)
        let s_27_1: i128 = u__id(state, tracer, s_27_0);
        // C s_27_2: const #1s : i
        let s_27_2: i128 = 1;
        // D s_27_3: cmp-eq s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) == (s_27_2));
        // D s_27_4: write-var gs#326751 <= s_27_3
        fn_state.gs_326751 = s_27_3;
        // N s_27_5: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#326751:u8
        let s_28_0: bool = fn_state.gs_326751;
        // N s_28_1: branch s_28_0 b34 b29
        if s_28_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var index:i
        let s_29_0: i128 = fn_state.index;
        // D s_29_1: call __id(s_29_0)
        let s_29_1: i128 = u__id(state, tracer, s_29_0);
        // C s_29_2: const #2s : i
        let s_29_2: i128 = 2;
        // D s_29_3: cmp-eq s_29_1 s_29_2
        let s_29_3: bool = ((s_29_1) == (s_29_2));
        // D s_29_4: write-var gs#326753 <= s_29_3
        fn_state.gs_326753 = s_29_3;
        // N s_29_5: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#326753:u8
        let s_30_0: bool = fn_state.gs_326753;
        // N s_30_1: branch s_30_0 b33 b31
        if s_30_0 {
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
        // D s_31_0: read-var index:i
        let s_31_0: i128 = fn_state.index;
        // D s_31_1: call __id(s_31_0)
        let s_31_1: i128 = u__id(state, tracer, s_31_0);
        // C s_31_2: const #3s : i
        let s_31_2: i128 = 3;
        // D s_31_3: cmp-eq s_31_1 s_31_2
        let s_31_3: bool = ((s_31_1) == (s_31_2));
        // D s_31_4: write-var gs#326755 <= s_31_3
        fn_state.gs_326755 = s_31_3;
        // N s_31_5: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#326755:u8
        let s_32_0: bool = fn_state.gs_326755;
        // D s_32_1: write-var gs#326756 <= s_32_0
        fn_state.gs_326756 = s_32_0;
        // N s_32_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#326755 <= s_33_0
        fn_state.gs_326755 = s_33_0;
        // N s_33_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#326753 <= s_34_0
        fn_state.gs_326753 = s_34_0;
        // N s_34_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#326751 <= s_35_0
        fn_state.gs_326751 = s_35_0;
        // N s_35_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #64s : i64
        let s_36_0: i64 = 64;
        // D s_36_1: write-var ga#366929 <= s_36_0
        fn_state.ga_366929 = s_36_0;
        // N s_36_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #2s : i64
        let s_37_0: i64 = 2;
        // D s_37_1: write-var ga#366927 <= s_37_0
        fn_state.ga_366927 = s_37_0;
        // N s_37_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #3s : i
        let s_38_0: i128 = 3;
        // D s_38_1: read-var Vm:u8
        let s_38_1: u8 = fn_state.Vm;
        // D s_38_2: cast zx s_38_1 -> bv
        let s_38_2: Bits = Bits::new(s_38_1 as u128, 4u16);
        // C s_38_3: const #1u : u64
        let s_38_3: u64 = 1;
        // D s_38_4: bit-extract s_38_2 s_38_0 s_38_3
        let s_38_4: Bits = (Bits::new(
            ((s_38_2) >> (s_38_0)).value(),
            u16::try_from(s_38_3).unwrap(),
        ));
        // D s_38_5: cast reint s_38_4 -> u8
        let s_38_5: bool = ((s_38_4.value()) != 0);
        // C s_38_6: const #0s : i
        let s_38_6: i128 = 0;
        // C s_38_7: const #0u : u64
        let s_38_7: u64 = 0;
        // D s_38_8: cast zx s_38_5 -> u64
        let s_38_8: u64 = (s_38_5 as u64);
        // C s_38_9: const #1u : u64
        let s_38_9: u64 = 1;
        // D s_38_10: and s_38_8 s_38_9
        let s_38_10: u64 = ((s_38_8) & (s_38_9));
        // D s_38_11: cmp-eq s_38_10 s_38_9
        let s_38_11: bool = ((s_38_10) == (s_38_9));
        // D s_38_12: lsl s_38_8 s_38_6
        let s_38_12: u64 = s_38_8 << s_38_6;
        // D s_38_13: or s_38_7 s_38_12
        let s_38_13: u64 = ((s_38_7) | (s_38_12));
        // D s_38_14: cmpl s_38_12
        let s_38_14: u64 = !s_38_12;
        // D s_38_15: and s_38_7 s_38_14
        let s_38_15: u64 = ((s_38_7) & (s_38_14));
        // D s_38_16: select s_38_11 s_38_13 s_38_15
        let s_38_16: u64 = if s_38_11 { s_38_13 } else { s_38_15 };
        // D s_38_17: cast trunc s_38_16 -> u8
        let s_38_17: bool = ((s_38_16) != 0);
        // D s_38_18: read-var M:u8
        let s_38_18: bool = fn_state.M;
        // D s_38_19: cast zx s_38_18 -> bv
        let s_38_19: Bits = Bits::new(s_38_18 as u128, 1u16);
        // D s_38_20: cast zx s_38_17 -> bv
        let s_38_20: Bits = Bits::new(s_38_17 as u128, 1u16);
        // D s_38_21: cast reint s_38_19 -> u128
        let s_38_21: u128 = (s_38_19.value() as u128);
        // D s_38_22: size-of s_38_19
        let s_38_22: u16 = s_38_19.length();
        // D s_38_23: cast reint s_38_20 -> u128
        let s_38_23: u128 = (s_38_20.value() as u128);
        // D s_38_24: size-of s_38_20
        let s_38_24: u16 = s_38_20.length();
        // D s_38_25: lsl s_38_21 s_38_24
        let s_38_25: u128 = s_38_21 << s_38_24;
        // D s_38_26: or s_38_25 s_38_23
        let s_38_26: u128 = ((s_38_25) | (s_38_23));
        // D s_38_27: add s_38_22 s_38_24
        let s_38_27: u16 = (s_38_22 + s_38_24);
        // D s_38_28: create-bits s_38_26 s_38_27
        let s_38_28: Bits = Bits::new(s_38_26, s_38_27);
        // D s_38_29: cast reint s_38_28 -> u8
        let s_38_29: u8 = (s_38_28.value() as u8);
        // D s_38_30: cast zx s_38_29 -> bv
        let s_38_30: Bits = Bits::new(s_38_29 as u128, 2u16);
        // D s_38_31: cast zx s_38_30 -> i
        let s_38_31: i128 = (s_38_30.value() as i128);
        // D s_38_32: write-var ga#366925 <= s_38_31
        fn_state.ga_366925 = s_38_31;
        // N s_38_33: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0s : i
        let s_39_0: i128 = 0;
        // D s_39_1: read-var Vm:u8
        let s_39_1: u8 = fn_state.Vm;
        // D s_39_2: cast zx s_39_1 -> bv
        let s_39_2: Bits = Bits::new(s_39_1 as u128, 4u16);
        // C s_39_3: const #1s : i64
        let s_39_3: i64 = 1;
        // C s_39_4: cast zx s_39_3 -> i
        let s_39_4: i128 = (i128::try_from(s_39_3).unwrap());
        // C s_39_5: const #2s : i
        let s_39_5: i128 = 2;
        // C s_39_6: add s_39_5 s_39_4
        let s_39_6: i128 = (s_39_5 + s_39_4);
        // D s_39_7: bit-extract s_39_2 s_39_0 s_39_6
        let s_39_7: Bits = (Bits::new(
            ((s_39_2) >> (s_39_0)).value(),
            u16::try_from(s_39_6).unwrap(),
        ));
        // D s_39_8: cast reint s_39_7 -> u8
        let s_39_8: u8 = (s_39_7.value() as u8);
        // D s_39_9: cast zx s_39_8 -> bv
        let s_39_9: Bits = Bits::new(s_39_8 as u128, 3u16);
        // D s_39_10: cast zx s_39_9 -> i
        let s_39_10: i128 = (s_39_9.value() as i128);
        // D s_39_11: write-var ga#366918 <= s_39_10
        fn_state.ga_366918 = s_39_10;
        // N s_39_12: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var N:u8
        let s_40_0: bool = fn_state.N;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // D s_40_2: read-var Vn:u8
        let s_40_2: u8 = fn_state.Vn;
        // D s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 4u16);
        // D s_40_4: cast reint s_40_1 -> u128
        let s_40_4: u128 = (s_40_1.value() as u128);
        // D s_40_5: size-of s_40_1
        let s_40_5: u16 = s_40_1.length();
        // D s_40_6: cast reint s_40_3 -> u128
        let s_40_6: u128 = (s_40_3.value() as u128);
        // D s_40_7: size-of s_40_3
        let s_40_7: u16 = s_40_3.length();
        // D s_40_8: lsl s_40_4 s_40_7
        let s_40_8: u128 = s_40_4 << s_40_7;
        // D s_40_9: or s_40_8 s_40_6
        let s_40_9: u128 = ((s_40_8) | (s_40_6));
        // D s_40_10: add s_40_5 s_40_7
        let s_40_10: u16 = (s_40_5 + s_40_7);
        // D s_40_11: create-bits s_40_9 s_40_10
        let s_40_11: Bits = Bits::new(s_40_9, s_40_10);
        // D s_40_12: cast reint s_40_11 -> u8
        let s_40_12: u8 = (s_40_11.value() as u8);
        // D s_40_13: cast zx s_40_12 -> bv
        let s_40_13: Bits = Bits::new(s_40_12 as u128, 5u16);
        // D s_40_14: cast zx s_40_13 -> i
        let s_40_14: i128 = (s_40_13.value() as i128);
        // D s_40_15: write-var ga#366913 <= s_40_14
        fn_state.ga_366913 = s_40_14;
        // N s_40_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: panic
        panic!("{:?}", ());
        // N s_41_1: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0s : i
        let s_42_0: i128 = 0;
        // D s_42_1: read-var Vd:u8
        let s_42_1: u8 = fn_state.Vd;
        // D s_42_2: cast zx s_42_1 -> bv
        let s_42_2: Bits = Bits::new(s_42_1 as u128, 4u16);
        // C s_42_3: const #1u : u64
        let s_42_3: u64 = 1;
        // D s_42_4: bit-extract s_42_2 s_42_0 s_42_3
        let s_42_4: Bits = (Bits::new(
            ((s_42_2) >> (s_42_0)).value(),
            u16::try_from(s_42_3).unwrap(),
        ));
        // D s_42_5: cast reint s_42_4 -> u8
        let s_42_5: bool = ((s_42_4.value()) != 0);
        // C s_42_6: const #0s : i
        let s_42_6: i128 = 0;
        // C s_42_7: const #0u : u64
        let s_42_7: u64 = 0;
        // D s_42_8: cast zx s_42_5 -> u64
        let s_42_8: u64 = (s_42_5 as u64);
        // C s_42_9: const #1u : u64
        let s_42_9: u64 = 1;
        // D s_42_10: and s_42_8 s_42_9
        let s_42_10: u64 = ((s_42_8) & (s_42_9));
        // D s_42_11: cmp-eq s_42_10 s_42_9
        let s_42_11: bool = ((s_42_10) == (s_42_9));
        // D s_42_12: lsl s_42_8 s_42_6
        let s_42_12: u64 = s_42_8 << s_42_6;
        // D s_42_13: or s_42_7 s_42_12
        let s_42_13: u64 = ((s_42_7) | (s_42_12));
        // D s_42_14: cmpl s_42_12
        let s_42_14: u64 = !s_42_12;
        // D s_42_15: and s_42_7 s_42_14
        let s_42_15: u64 = ((s_42_7) & (s_42_14));
        // D s_42_16: select s_42_11 s_42_13 s_42_15
        let s_42_16: u64 = if s_42_11 { s_42_13 } else { s_42_15 };
        // D s_42_17: cast trunc s_42_16 -> u8
        let s_42_17: bool = ((s_42_16) != 0);
        // D s_42_18: cast zx s_42_17 -> bv
        let s_42_18: Bits = Bits::new(s_42_17 as u128, 1u16);
        // C s_42_19: const #1u : u8
        let s_42_19: bool = true;
        // C s_42_20: cast zx s_42_19 -> bv
        let s_42_20: Bits = Bits::new(s_42_19 as u128, 1u16);
        // D s_42_21: cmp-eq s_42_18 s_42_20
        let s_42_21: bool = ((s_42_18) == (s_42_20));
        // D s_42_22: write-var gs#326724 <= s_42_21
        fn_state.gs_326724 = s_42_21;
        // N s_42_23: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: panic
        panic!("{:?}", ());
        // N s_43_1: return
        return;
    }
}
