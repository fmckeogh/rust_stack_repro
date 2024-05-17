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
use InITBlock::*;
use execute_aarch32_instrs_VFMAL_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VFMAL_T1enc_A_txt<T: Tracer>(
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
        ga_366860: i128,
        gs_326674: bool,
        regs: i64,
        ga_366864: i64,
        gs_326656: bool,
        n: i128,
        gs_326675: bool,
        d: i64,
        sub_op: bool,
        gs_326673: bool,
        ga_366856: i128,
        datasize: i64,
        ga_366862: i64,
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
        // S s_0_1: call InITBlock(s_0_0)
        let s_0_1: bool = InITBlock(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b30 b1
        if s_0_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveFP16MulNoRoundingToFP32Ext(s_1_0)
        let s_1_1: bool = HaveFP16MulNoRoundingToFP32Ext(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // N s_1_3: branch s_1_2 b29 b2
        if s_1_2 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
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
        // N s_2_5: branch s_2_4 b28 b3
        if s_2_4 {
            return block_28(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#326656 <= s_3_0
        fn_state.gs_326656 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#326656:u8
        let s_4_0: bool = fn_state.gs_326656;
        // N s_4_1: branch s_4_0 b27 b5
        if s_4_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var D:u8
        let s_5_0: bool = fn_state.D;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // D s_5_2: read-var Vd:u8
        let s_5_2: u8 = fn_state.Vd;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
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
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // D s_5_16: write-var d <= s_5_15
        fn_state.d = s_5_15;
        // D s_5_17: read-var Q:u8
        let s_5_17: bool = fn_state.Q;
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // C s_5_19: const #1u : u8
        let s_5_19: bool = true;
        // C s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // D s_5_21: cmp-eq s_5_18 s_5_20
        let s_5_21: bool = ((s_5_18) == (s_5_20));
        // N s_5_22: branch s_5_21 b26 b6
        if s_5_21 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var Vn:u8
        let s_6_0: u8 = fn_state.Vn;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 4u16);
        // D s_6_2: read-var N:u8
        let s_6_2: bool = fn_state.N;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
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
        // D s_6_15: write-var ga#366856 <= s_6_14
        fn_state.ga_366856 = s_6_14;
        // N s_6_16: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#366856:i
        let s_7_0: i128 = fn_state.ga_366856;
        // D s_7_1: write-var n <= s_7_0
        fn_state.n = s_7_0;
        // D s_7_2: read-var Q:u8
        let s_7_2: bool = fn_state.Q;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // C s_7_4: const #1u : u8
        let s_7_4: bool = true;
        // C s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 1u16);
        // D s_7_6: cmp-eq s_7_3 s_7_5
        let s_7_6: bool = ((s_7_3) == (s_7_5));
        // N s_7_7: branch s_7_6 b25 b8
        if s_7_6 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var Vm:u8
        let s_8_0: u8 = fn_state.Vm;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 4u16);
        // D s_8_2: read-var M:u8
        let s_8_2: bool = fn_state.M;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cast reint s_8_1 -> u128
        let s_8_4: u128 = (s_8_1.value() as u128);
        // D s_8_5: size-of s_8_1
        let s_8_5: u16 = s_8_1.length();
        // D s_8_6: cast reint s_8_3 -> u128
        let s_8_6: u128 = (s_8_3.value() as u128);
        // D s_8_7: size-of s_8_3
        let s_8_7: u16 = s_8_3.length();
        // D s_8_8: lsl s_8_4 s_8_7
        let s_8_8: u128 = s_8_4 << s_8_7;
        // D s_8_9: or s_8_8 s_8_6
        let s_8_9: u128 = ((s_8_8) | (s_8_6));
        // D s_8_10: add s_8_5 s_8_7
        let s_8_10: u16 = (s_8_5 + s_8_7);
        // D s_8_11: create-bits s_8_9 s_8_10
        let s_8_11: Bits = Bits::new(s_8_9, s_8_10);
        // D s_8_12: cast reint s_8_11 -> u8
        let s_8_12: u8 = (s_8_11.value() as u8);
        // D s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 5u16);
        // D s_8_14: cast zx s_8_13 -> i
        let s_8_14: i128 = (s_8_13.value() as i128);
        // D s_8_15: write-var ga#366860 <= s_8_14
        fn_state.ga_366860 = s_8_14;
        // N s_8_16: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#366860:i
        let s_9_0: i128 = fn_state.ga_366860;
        // D s_9_1: write-var m <= s_9_0
        fn_state.m = s_9_0;
        // D s_9_2: read-var Q:u8
        let s_9_2: bool = fn_state.Q;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #1u : u8
        let s_9_4: bool = true;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // N s_9_7: branch s_9_6 b24 b10
        if s_9_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1s : i64
        let s_10_0: i64 = 1;
        // D s_10_1: write-var ga#366862 <= s_10_0
        fn_state.ga_366862 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#366862:i64
        let s_11_0: i64 = fn_state.ga_366862;
        // D s_11_1: write-var regs <= s_11_0
        fn_state.regs = s_11_0;
        // D s_11_2: read-var Q:u8
        let s_11_2: bool = fn_state.Q;
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #1u : u8
        let s_11_4: bool = true;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // N s_11_7: branch s_11_6 b23 b12
        if s_11_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #32s : i64
        let s_12_0: i64 = 32;
        // D s_12_1: write-var ga#366864 <= s_12_0
        fn_state.ga_366864 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var ga#366864:i64
        let s_13_0: i64 = fn_state.ga_366864;
        // D s_13_1: write-var datasize <= s_13_0
        fn_state.datasize = s_13_0;
        // D s_13_2: read-var S:u8
        let s_13_2: bool = fn_state.S;
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // C s_13_4: const #1u : u8
        let s_13_4: bool = true;
        // C s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 1u16);
        // D s_13_6: cmp-eq s_13_3 s_13_5
        let s_13_6: bool = ((s_13_3) == (s_13_5));
        // D s_13_7: write-var sub_op <= s_13_6
        fn_state.sub_op = s_13_6;
        // D s_13_8: read-var n:i
        let s_13_8: i128 = fn_state.n;
        // D s_13_9: call __id(s_13_8)
        let s_13_9: i128 = u__id(state, tracer, s_13_8);
        // C s_13_10: const #0s : i
        let s_13_10: i128 = 0;
        // D s_13_11: cmp-le s_13_10 s_13_9
        let s_13_11: bool = ((s_13_10) <= (s_13_9));
        // N s_13_12: branch s_13_11 b16 b14
        if s_13_11 {
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
        // D s_14_1: write-var gs#326675 <= s_14_0
        fn_state.gs_326675 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#326675:u8
        let s_15_0: bool = fn_state.gs_326675;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // D s_15_2: read-var datasize:i64
        let s_15_2: i64 = fn_state.datasize;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: cast reint s_15_3 -> i64
        let s_15_4: i64 = (s_15_3 as i64);
        // C s_15_5: const #32s : i64
        let s_15_5: i64 = 32;
        // D s_15_6: read-var m:i
        let s_15_6: i128 = fn_state.m;
        // D s_15_7: cast reint s_15_6 -> i64
        let s_15_7: i64 = (s_15_6 as i64);
        // D s_15_8: read-var n:i
        let s_15_8: i128 = fn_state.n;
        // D s_15_9: cast reint s_15_8 -> i64
        let s_15_9: i64 = (s_15_8 as i64);
        // D s_15_10: read-var Q:u8
        let s_15_10: bool = fn_state.Q;
        // D s_15_11: read-var d:i64
        let s_15_11: i64 = fn_state.d;
        // D s_15_12: read-var regs:i64
        let s_15_12: i64 = fn_state.regs;
        // D s_15_13: read-var sub_op:u8
        let s_15_13: bool = fn_state.sub_op;
        // D s_15_14: call execute_aarch32_instrs_VFMAL_Op_A_txt(s_15_10, s_15_11, s_15_4, s_15_5, s_15_7, s_15_9, s_15_12, s_15_13)
        let s_15_14: () = execute_aarch32_instrs_VFMAL_Op_A_txt(
            state,
            tracer,
            s_15_10,
            s_15_11,
            s_15_4,
            s_15_5,
            s_15_7,
            s_15_9,
            s_15_12,
            s_15_13,
        );
        // N s_15_15: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var n:i
        let s_16_0: i128 = fn_state.n;
        // D s_16_1: call __id(s_16_0)
        let s_16_1: i128 = u__id(state, tracer, s_16_0);
        // C s_16_2: const #31s : i
        let s_16_2: i128 = 31;
        // D s_16_3: cmp-le s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) <= (s_16_2));
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
        // D s_17_1: write-var gs#326674 <= s_17_0
        fn_state.gs_326674 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#326674:u8
        let s_18_0: bool = fn_state.gs_326674;
        // D s_18_1: write-var gs#326675 <= s_18_0
        fn_state.gs_326675 = s_18_0;
        // N s_18_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var m:i
        let s_19_0: i128 = fn_state.m;
        // D s_19_1: call __id(s_19_0)
        let s_19_1: i128 = u__id(state, tracer, s_19_0);
        // C s_19_2: const #0s : i
        let s_19_2: i128 = 0;
        // D s_19_3: cmp-le s_19_2 s_19_1
        let s_19_3: bool = ((s_19_2) <= (s_19_1));
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
        // D s_20_1: write-var gs#326673 <= s_20_0
        fn_state.gs_326673 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#326673:u8
        let s_21_0: bool = fn_state.gs_326673;
        // D s_21_1: write-var gs#326674 <= s_21_0
        fn_state.gs_326674 = s_21_0;
        // N s_21_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var m:i
        let s_22_0: i128 = fn_state.m;
        // D s_22_1: call __id(s_22_0)
        let s_22_1: i128 = u__id(state, tracer, s_22_0);
        // C s_22_2: const #31s : i
        let s_22_2: i128 = 31;
        // D s_22_3: cmp-le s_22_1 s_22_2
        let s_22_3: bool = ((s_22_1) <= (s_22_2));
        // D s_22_4: write-var gs#326673 <= s_22_3
        fn_state.gs_326673 = s_22_3;
        // N s_22_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #64s : i64
        let s_23_0: i64 = 64;
        // D s_23_1: write-var ga#366864 <= s_23_0
        fn_state.ga_366864 = s_23_0;
        // N s_23_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #2s : i64
        let s_24_0: i64 = 2;
        // D s_24_1: write-var ga#366862 <= s_24_0
        fn_state.ga_366862 = s_24_0;
        // N s_24_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var M:u8
        let s_25_0: bool = fn_state.M;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // D s_25_2: read-var Vm:u8
        let s_25_2: u8 = fn_state.Vm;
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
        // D s_25_15: write-var ga#366860 <= s_25_14
        fn_state.ga_366860 = s_25_14;
        // N s_25_16: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var N:u8
        let s_26_0: bool = fn_state.N;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // D s_26_2: read-var Vn:u8
        let s_26_2: u8 = fn_state.Vn;
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 4u16);
        // D s_26_4: cast reint s_26_1 -> u128
        let s_26_4: u128 = (s_26_1.value() as u128);
        // D s_26_5: size-of s_26_1
        let s_26_5: u16 = s_26_1.length();
        // D s_26_6: cast reint s_26_3 -> u128
        let s_26_6: u128 = (s_26_3.value() as u128);
        // D s_26_7: size-of s_26_3
        let s_26_7: u16 = s_26_3.length();
        // D s_26_8: lsl s_26_4 s_26_7
        let s_26_8: u128 = s_26_4 << s_26_7;
        // D s_26_9: or s_26_8 s_26_6
        let s_26_9: u128 = ((s_26_8) | (s_26_6));
        // D s_26_10: add s_26_5 s_26_7
        let s_26_10: u16 = (s_26_5 + s_26_7);
        // D s_26_11: create-bits s_26_9 s_26_10
        let s_26_11: Bits = Bits::new(s_26_9, s_26_10);
        // D s_26_12: cast reint s_26_11 -> u8
        let s_26_12: u8 = (s_26_11.value() as u8);
        // D s_26_13: cast zx s_26_12 -> bv
        let s_26_13: Bits = Bits::new(s_26_12 as u128, 5u16);
        // D s_26_14: cast zx s_26_13 -> i
        let s_26_14: i128 = (s_26_13.value() as i128);
        // D s_26_15: write-var ga#366856 <= s_26_14
        fn_state.ga_366856 = s_26_14;
        // N s_26_16: jump b7
        return block_7(state, tracer, fn_state);
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
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0s : i
        let s_28_0: i128 = 0;
        // D s_28_1: read-var Vd:u8
        let s_28_1: u8 = fn_state.Vd;
        // D s_28_2: cast zx s_28_1 -> bv
        let s_28_2: Bits = Bits::new(s_28_1 as u128, 4u16);
        // C s_28_3: const #1u : u64
        let s_28_3: u64 = 1;
        // D s_28_4: bit-extract s_28_2 s_28_0 s_28_3
        let s_28_4: Bits = (Bits::new(
            ((s_28_2) >> (s_28_0)).value(),
            u16::try_from(s_28_3).unwrap(),
        ));
        // D s_28_5: cast reint s_28_4 -> u8
        let s_28_5: bool = ((s_28_4.value()) != 0);
        // C s_28_6: const #0s : i
        let s_28_6: i128 = 0;
        // C s_28_7: const #0u : u64
        let s_28_7: u64 = 0;
        // D s_28_8: cast zx s_28_5 -> u64
        let s_28_8: u64 = (s_28_5 as u64);
        // C s_28_9: const #1u : u64
        let s_28_9: u64 = 1;
        // D s_28_10: and s_28_8 s_28_9
        let s_28_10: u64 = ((s_28_8) & (s_28_9));
        // D s_28_11: cmp-eq s_28_10 s_28_9
        let s_28_11: bool = ((s_28_10) == (s_28_9));
        // D s_28_12: lsl s_28_8 s_28_6
        let s_28_12: u64 = s_28_8 << s_28_6;
        // D s_28_13: or s_28_7 s_28_12
        let s_28_13: u64 = ((s_28_7) | (s_28_12));
        // D s_28_14: cmpl s_28_12
        let s_28_14: u64 = !s_28_12;
        // D s_28_15: and s_28_7 s_28_14
        let s_28_15: u64 = ((s_28_7) & (s_28_14));
        // D s_28_16: select s_28_11 s_28_13 s_28_15
        let s_28_16: u64 = if s_28_11 { s_28_13 } else { s_28_15 };
        // D s_28_17: cast trunc s_28_16 -> u8
        let s_28_17: bool = ((s_28_16) != 0);
        // D s_28_18: cast zx s_28_17 -> bv
        let s_28_18: Bits = Bits::new(s_28_17 as u128, 1u16);
        // C s_28_19: const #1u : u8
        let s_28_19: bool = true;
        // C s_28_20: cast zx s_28_19 -> bv
        let s_28_20: Bits = Bits::new(s_28_19 as u128, 1u16);
        // D s_28_21: cmp-eq s_28_18 s_28_20
        let s_28_21: bool = ((s_28_18) == (s_28_20));
        // D s_28_22: write-var gs#326656 <= s_28_21
        fn_state.gs_326656 = s_28_21;
        // N s_28_23: jump b4
        return block_4(state, tracer, fn_state);
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
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
}
