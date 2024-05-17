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
use HaveFP16MulNoRoundingToFP32Ext::*;
use u__id::*;
use execute_aarch32_instrs_VFMAL_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VFMAL_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    S: bool,
    D: bool,
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
        gs_326644: bool,
        gs_326628: bool,
        gs_326645: bool,
        regs: i64,
        ga_366827: i128,
        ga_366831: i64,
        n: i128,
        d: i64,
        sub_op: bool,
        gs_326646: bool,
        datasize: i64,
        ga_366829: i64,
        ga_366823: i128,
        S: bool,
        D: bool,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        S,
        D,
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
        // N s_0_3: branch s_0_2 b28 b1
        if s_0_2 {
            return block_28(state, tracer, fn_state);
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
        // N s_1_5: branch s_1_4 b27 b2
        if s_1_4 {
            return block_27(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#326628 <= s_2_0
        fn_state.gs_326628 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#326628:u8
        let s_3_0: bool = fn_state.gs_326628;
        // N s_3_1: branch s_3_0 b26 b4
        if s_3_0 {
            return block_26(state, tracer, fn_state);
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
        // N s_4_22: branch s_4_21 b25 b5
        if s_4_21 {
            return block_25(state, tracer, fn_state);
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
        // D s_5_15: write-var ga#366823 <= s_5_14
        fn_state.ga_366823 = s_5_14;
        // N s_5_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#366823:i
        let s_6_0: i128 = fn_state.ga_366823;
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
        // N s_6_7: branch s_6_6 b24 b7
        if s_6_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var Vm:u8
        let s_7_0: u8 = fn_state.Vm;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 4u16);
        // D s_7_2: read-var M:u8
        let s_7_2: bool = fn_state.M;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cast reint s_7_1 -> u128
        let s_7_4: u128 = (s_7_1.value() as u128);
        // D s_7_5: size-of s_7_1
        let s_7_5: u16 = s_7_1.length();
        // D s_7_6: cast reint s_7_3 -> u128
        let s_7_6: u128 = (s_7_3.value() as u128);
        // D s_7_7: size-of s_7_3
        let s_7_7: u16 = s_7_3.length();
        // D s_7_8: lsl s_7_4 s_7_7
        let s_7_8: u128 = s_7_4 << s_7_7;
        // D s_7_9: or s_7_8 s_7_6
        let s_7_9: u128 = ((s_7_8) | (s_7_6));
        // D s_7_10: add s_7_5 s_7_7
        let s_7_10: u16 = (s_7_5 + s_7_7);
        // D s_7_11: create-bits s_7_9 s_7_10
        let s_7_11: Bits = Bits::new(s_7_9, s_7_10);
        // D s_7_12: cast reint s_7_11 -> u8
        let s_7_12: u8 = (s_7_11.value() as u8);
        // D s_7_13: cast zx s_7_12 -> bv
        let s_7_13: Bits = Bits::new(s_7_12 as u128, 5u16);
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (s_7_13.value() as i128);
        // D s_7_15: write-var ga#366827 <= s_7_14
        fn_state.ga_366827 = s_7_14;
        // N s_7_16: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#366827:i
        let s_8_0: i128 = fn_state.ga_366827;
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
        // N s_8_7: branch s_8_6 b23 b9
        if s_8_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1s : i64
        let s_9_0: i64 = 1;
        // D s_9_1: write-var ga#366829 <= s_9_0
        fn_state.ga_366829 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#366829:i64
        let s_10_0: i64 = fn_state.ga_366829;
        // D s_10_1: write-var regs <= s_10_0
        fn_state.regs = s_10_0;
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
        // N s_10_7: branch s_10_6 b22 b11
        if s_10_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #32s : i64
        let s_11_0: i64 = 32;
        // D s_11_1: write-var ga#366831 <= s_11_0
        fn_state.ga_366831 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#366831:i64
        let s_12_0: i64 = fn_state.ga_366831;
        // D s_12_1: write-var datasize <= s_12_0
        fn_state.datasize = s_12_0;
        // D s_12_2: read-var S:u8
        let s_12_2: bool = fn_state.S;
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // C s_12_4: const #1u : u8
        let s_12_4: bool = true;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // D s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // D s_12_7: write-var sub_op <= s_12_6
        fn_state.sub_op = s_12_6;
        // D s_12_8: read-var n:i
        let s_12_8: i128 = fn_state.n;
        // D s_12_9: call __id(s_12_8)
        let s_12_9: i128 = u__id(state, tracer, s_12_8);
        // C s_12_10: const #0s : i
        let s_12_10: i128 = 0;
        // D s_12_11: cmp-le s_12_10 s_12_9
        let s_12_11: bool = ((s_12_10) <= (s_12_9));
        // N s_12_12: branch s_12_11 b15 b13
        if s_12_11 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#326646 <= s_13_0
        fn_state.gs_326646 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#326646:u8
        let s_14_0: bool = fn_state.gs_326646;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // D s_14_2: read-var datasize:i64
        let s_14_2: i64 = fn_state.datasize;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: cast reint s_14_3 -> i64
        let s_14_4: i64 = (s_14_3 as i64);
        // C s_14_5: const #32s : i64
        let s_14_5: i64 = 32;
        // D s_14_6: read-var m:i
        let s_14_6: i128 = fn_state.m;
        // D s_14_7: cast reint s_14_6 -> i64
        let s_14_7: i64 = (s_14_6 as i64);
        // D s_14_8: read-var n:i
        let s_14_8: i128 = fn_state.n;
        // D s_14_9: cast reint s_14_8 -> i64
        let s_14_9: i64 = (s_14_8 as i64);
        // D s_14_10: read-var Q:u8
        let s_14_10: bool = fn_state.Q;
        // D s_14_11: read-var d:i64
        let s_14_11: i64 = fn_state.d;
        // D s_14_12: read-var regs:i64
        let s_14_12: i64 = fn_state.regs;
        // D s_14_13: read-var sub_op:u8
        let s_14_13: bool = fn_state.sub_op;
        // D s_14_14: call execute_aarch32_instrs_VFMAL_Op_A_txt(s_14_10, s_14_11, s_14_4, s_14_5, s_14_7, s_14_9, s_14_12, s_14_13)
        let s_14_14: () = execute_aarch32_instrs_VFMAL_Op_A_txt(
            state,
            tracer,
            s_14_10,
            s_14_11,
            s_14_4,
            s_14_5,
            s_14_7,
            s_14_9,
            s_14_12,
            s_14_13,
        );
        // N s_14_15: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var n:i
        let s_15_0: i128 = fn_state.n;
        // D s_15_1: call __id(s_15_0)
        let s_15_1: i128 = u__id(state, tracer, s_15_0);
        // C s_15_2: const #31s : i
        let s_15_2: i128 = 31;
        // D s_15_3: cmp-le s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) <= (s_15_2));
        // N s_15_4: branch s_15_3 b18 b16
        if s_15_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#326645 <= s_16_0
        fn_state.gs_326645 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#326645:u8
        let s_17_0: bool = fn_state.gs_326645;
        // D s_17_1: write-var gs#326646 <= s_17_0
        fn_state.gs_326646 = s_17_0;
        // N s_17_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var m:i
        let s_18_0: i128 = fn_state.m;
        // D s_18_1: call __id(s_18_0)
        let s_18_1: i128 = u__id(state, tracer, s_18_0);
        // C s_18_2: const #0s : i
        let s_18_2: i128 = 0;
        // D s_18_3: cmp-le s_18_2 s_18_1
        let s_18_3: bool = ((s_18_2) <= (s_18_1));
        // N s_18_4: branch s_18_3 b21 b19
        if s_18_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#326644 <= s_19_0
        fn_state.gs_326644 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#326644:u8
        let s_20_0: bool = fn_state.gs_326644;
        // D s_20_1: write-var gs#326645 <= s_20_0
        fn_state.gs_326645 = s_20_0;
        // N s_20_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var m:i
        let s_21_0: i128 = fn_state.m;
        // D s_21_1: call __id(s_21_0)
        let s_21_1: i128 = u__id(state, tracer, s_21_0);
        // C s_21_2: const #31s : i
        let s_21_2: i128 = 31;
        // D s_21_3: cmp-le s_21_1 s_21_2
        let s_21_3: bool = ((s_21_1) <= (s_21_2));
        // D s_21_4: write-var gs#326644 <= s_21_3
        fn_state.gs_326644 = s_21_3;
        // N s_21_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #64s : i64
        let s_22_0: i64 = 64;
        // D s_22_1: write-var ga#366831 <= s_22_0
        fn_state.ga_366831 = s_22_0;
        // N s_22_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #2s : i64
        let s_23_0: i64 = 2;
        // D s_23_1: write-var ga#366829 <= s_23_0
        fn_state.ga_366829 = s_23_0;
        // N s_23_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var M:u8
        let s_24_0: bool = fn_state.M;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // D s_24_2: read-var Vm:u8
        let s_24_2: u8 = fn_state.Vm;
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 4u16);
        // D s_24_4: cast reint s_24_1 -> u128
        let s_24_4: u128 = (s_24_1.value() as u128);
        // D s_24_5: size-of s_24_1
        let s_24_5: u16 = s_24_1.length();
        // D s_24_6: cast reint s_24_3 -> u128
        let s_24_6: u128 = (s_24_3.value() as u128);
        // D s_24_7: size-of s_24_3
        let s_24_7: u16 = s_24_3.length();
        // D s_24_8: lsl s_24_4 s_24_7
        let s_24_8: u128 = s_24_4 << s_24_7;
        // D s_24_9: or s_24_8 s_24_6
        let s_24_9: u128 = ((s_24_8) | (s_24_6));
        // D s_24_10: add s_24_5 s_24_7
        let s_24_10: u16 = (s_24_5 + s_24_7);
        // D s_24_11: create-bits s_24_9 s_24_10
        let s_24_11: Bits = Bits::new(s_24_9, s_24_10);
        // D s_24_12: cast reint s_24_11 -> u8
        let s_24_12: u8 = (s_24_11.value() as u8);
        // D s_24_13: cast zx s_24_12 -> bv
        let s_24_13: Bits = Bits::new(s_24_12 as u128, 5u16);
        // D s_24_14: cast zx s_24_13 -> i
        let s_24_14: i128 = (s_24_13.value() as i128);
        // D s_24_15: write-var ga#366827 <= s_24_14
        fn_state.ga_366827 = s_24_14;
        // N s_24_16: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var N:u8
        let s_25_0: bool = fn_state.N;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // D s_25_2: read-var Vn:u8
        let s_25_2: u8 = fn_state.Vn;
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 4u16);
        // D s_25_4: cast reint s_25_1 -> u128
        let s_25_4: u128 = (s_25_1.value() as u128);
        // D s_25_5: size-of s_25_1
        let s_25_5: u16 = s_25_1.length();
        // D s_25_6: cast reint s_25_3 -> u128
        let s_25_6: u128 = (s_25_3.value() as u128);
        // D s_25_7: size-of s_25_3
        let s_25_7: u16 = s_25_3.length();
        // D s_25_8: lsl s_25_4 s_25_7
        let s_25_8: u128 = s_25_4 << s_25_7;
        // D s_25_9: or s_25_8 s_25_6
        let s_25_9: u128 = ((s_25_8) | (s_25_6));
        // D s_25_10: add s_25_5 s_25_7
        let s_25_10: u16 = (s_25_5 + s_25_7);
        // D s_25_11: create-bits s_25_9 s_25_10
        let s_25_11: Bits = Bits::new(s_25_9, s_25_10);
        // D s_25_12: cast reint s_25_11 -> u8
        let s_25_12: u8 = (s_25_11.value() as u8);
        // D s_25_13: cast zx s_25_12 -> bv
        let s_25_13: Bits = Bits::new(s_25_12 as u128, 5u16);
        // D s_25_14: cast zx s_25_13 -> i
        let s_25_14: i128 = (s_25_13.value() as i128);
        // D s_25_15: write-var ga#366823 <= s_25_14
        fn_state.ga_366823 = s_25_14;
        // N s_25_16: jump b6
        return block_6(state, tracer, fn_state);
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
        // C s_27_0: const #0s : i
        let s_27_0: i128 = 0;
        // D s_27_1: read-var Vd:u8
        let s_27_1: u8 = fn_state.Vd;
        // D s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 4u16);
        // C s_27_3: const #1u : u64
        let s_27_3: u64 = 1;
        // D s_27_4: bit-extract s_27_2 s_27_0 s_27_3
        let s_27_4: Bits = (Bits::new(
            ((s_27_2) >> (s_27_0)).value(),
            u16::try_from(s_27_3).unwrap(),
        ));
        // D s_27_5: cast reint s_27_4 -> u8
        let s_27_5: bool = ((s_27_4.value()) != 0);
        // C s_27_6: const #0s : i
        let s_27_6: i128 = 0;
        // C s_27_7: const #0u : u64
        let s_27_7: u64 = 0;
        // D s_27_8: cast zx s_27_5 -> u64
        let s_27_8: u64 = (s_27_5 as u64);
        // C s_27_9: const #1u : u64
        let s_27_9: u64 = 1;
        // D s_27_10: and s_27_8 s_27_9
        let s_27_10: u64 = ((s_27_8) & (s_27_9));
        // D s_27_11: cmp-eq s_27_10 s_27_9
        let s_27_11: bool = ((s_27_10) == (s_27_9));
        // D s_27_12: lsl s_27_8 s_27_6
        let s_27_12: u64 = s_27_8 << s_27_6;
        // D s_27_13: or s_27_7 s_27_12
        let s_27_13: u64 = ((s_27_7) | (s_27_12));
        // D s_27_14: cmpl s_27_12
        let s_27_14: u64 = !s_27_12;
        // D s_27_15: and s_27_7 s_27_14
        let s_27_15: u64 = ((s_27_7) & (s_27_14));
        // D s_27_16: select s_27_11 s_27_13 s_27_15
        let s_27_16: u64 = if s_27_11 { s_27_13 } else { s_27_15 };
        // D s_27_17: cast trunc s_27_16 -> u8
        let s_27_17: bool = ((s_27_16) != 0);
        // D s_27_18: cast zx s_27_17 -> bv
        let s_27_18: Bits = Bits::new(s_27_17 as u128, 1u16);
        // C s_27_19: const #1u : u8
        let s_27_19: bool = true;
        // C s_27_20: cast zx s_27_19 -> bv
        let s_27_20: Bits = Bits::new(s_27_19 as u128, 1u16);
        // D s_27_21: cmp-eq s_27_18 s_27_20
        let s_27_21: bool = ((s_27_18) == (s_27_20));
        // D s_27_22: write-var gs#326628 <= s_27_21
        fn_state.gs_326628 = s_27_21;
        // N s_27_23: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
}
