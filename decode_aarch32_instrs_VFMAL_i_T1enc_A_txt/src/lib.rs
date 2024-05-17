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
use execute_aarch32_instrs_VFMAL_i_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_VFMAL_i_T1enc_A_txt<T: Tracer>(
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
        n: i128,
        index: i128,
        gs_326800: bool,
        gs_326804: bool,
        gs_326806: bool,
        ga_366978: i64,
        gs_326770: bool,
        ga_366962: i128,
        ga_366974: i128,
        regs: i64,
        d: i64,
        sub_op: bool,
        ga_366967: i128,
        gs_326803: bool,
        ga_366976: i64,
        datasize: i64,
        gs_326802: bool,
        gs_326805: bool,
        gs_326798: bool,
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
        // S s_0_1: call InITBlock(s_0_0)
        let s_0_1: bool = InITBlock(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b45 b1
        if s_0_1 {
            return block_45(state, tracer, fn_state);
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
        // N s_1_3: branch s_1_2 b44 b2
        if s_1_2 {
            return block_44(state, tracer, fn_state);
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
        // N s_2_5: branch s_2_4 b43 b3
        if s_2_4 {
            return block_43(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#326770 <= s_3_0
        fn_state.gs_326770 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#326770:u8
        let s_4_0: bool = fn_state.gs_326770;
        // N s_4_1: branch s_4_0 b42 b5
        if s_4_0 {
            return block_42(state, tracer, fn_state);
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
        // N s_5_22: branch s_5_21 b41 b6
        if s_5_21 {
            return block_41(state, tracer, fn_state);
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
        // D s_6_15: write-var ga#366962 <= s_6_14
        fn_state.ga_366962 = s_6_14;
        // N s_6_16: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#366962:i
        let s_7_0: i128 = fn_state.ga_366962;
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
        // N s_7_7: branch s_7_6 b40 b8
        if s_7_6 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0s : i
        let s_8_0: i128 = 0;
        // D s_8_1: read-var Vm:u8
        let s_8_1: u8 = fn_state.Vm;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 4u16);
        // C s_8_3: const #1s : i64
        let s_8_3: i64 = 1;
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #2s : i
        let s_8_5: i128 = 2;
        // C s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: bit-extract s_8_2 s_8_0 s_8_6
        let s_8_7: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u8
        let s_8_8: u8 = (s_8_7.value() as u8);
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 3u16);
        // D s_8_10: read-var M:u8
        let s_8_10: bool = fn_state.M;
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 1u16);
        // D s_8_12: cast reint s_8_9 -> u128
        let s_8_12: u128 = (s_8_9.value() as u128);
        // D s_8_13: size-of s_8_9
        let s_8_13: u16 = s_8_9.length();
        // D s_8_14: cast reint s_8_11 -> u128
        let s_8_14: u128 = (s_8_11.value() as u128);
        // D s_8_15: size-of s_8_11
        let s_8_15: u16 = s_8_11.length();
        // D s_8_16: lsl s_8_12 s_8_15
        let s_8_16: u128 = s_8_12 << s_8_15;
        // D s_8_17: or s_8_16 s_8_14
        let s_8_17: u128 = ((s_8_16) | (s_8_14));
        // D s_8_18: add s_8_13 s_8_15
        let s_8_18: u16 = (s_8_13 + s_8_15);
        // D s_8_19: create-bits s_8_17 s_8_18
        let s_8_19: Bits = Bits::new(s_8_17, s_8_18);
        // D s_8_20: cast reint s_8_19 -> u8
        let s_8_20: u8 = (s_8_19.value() as u8);
        // D s_8_21: cast zx s_8_20 -> bv
        let s_8_21: Bits = Bits::new(s_8_20 as u128, 4u16);
        // D s_8_22: cast zx s_8_21 -> i
        let s_8_22: i128 = (s_8_21.value() as i128);
        // D s_8_23: write-var ga#366967 <= s_8_22
        fn_state.ga_366967 = s_8_22;
        // N s_8_24: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#366967:i
        let s_9_0: i128 = fn_state.ga_366967;
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
        // N s_9_7: branch s_9_6 b39 b10
        if s_9_6 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #3s : i
        let s_10_0: i128 = 3;
        // D s_10_1: read-var Vm:u8
        let s_10_1: u8 = fn_state.Vm;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 4u16);
        // C s_10_3: const #1u : u64
        let s_10_3: u64 = 1;
        // D s_10_4: bit-extract s_10_2 s_10_0 s_10_3
        let s_10_4: Bits = (Bits::new(
            ((s_10_2) >> (s_10_0)).value(),
            u16::try_from(s_10_3).unwrap(),
        ));
        // D s_10_5: cast reint s_10_4 -> u8
        let s_10_5: bool = ((s_10_4.value()) != 0);
        // C s_10_6: const #0s : i
        let s_10_6: i128 = 0;
        // C s_10_7: const #0u : u64
        let s_10_7: u64 = 0;
        // D s_10_8: cast zx s_10_5 -> u64
        let s_10_8: u64 = (s_10_5 as u64);
        // C s_10_9: const #1u : u64
        let s_10_9: u64 = 1;
        // D s_10_10: and s_10_8 s_10_9
        let s_10_10: u64 = ((s_10_8) & (s_10_9));
        // D s_10_11: cmp-eq s_10_10 s_10_9
        let s_10_11: bool = ((s_10_10) == (s_10_9));
        // D s_10_12: lsl s_10_8 s_10_6
        let s_10_12: u64 = s_10_8 << s_10_6;
        // D s_10_13: or s_10_7 s_10_12
        let s_10_13: u64 = ((s_10_7) | (s_10_12));
        // D s_10_14: cmpl s_10_12
        let s_10_14: u64 = !s_10_12;
        // D s_10_15: and s_10_7 s_10_14
        let s_10_15: u64 = ((s_10_7) & (s_10_14));
        // D s_10_16: select s_10_11 s_10_13 s_10_15
        let s_10_16: u64 = if s_10_11 { s_10_13 } else { s_10_15 };
        // D s_10_17: cast trunc s_10_16 -> u8
        let s_10_17: bool = ((s_10_16) != 0);
        // D s_10_18: cast zx s_10_17 -> bv
        let s_10_18: Bits = Bits::new(s_10_17 as u128, 1u16);
        // D s_10_19: cast zx s_10_18 -> i
        let s_10_19: i128 = (s_10_18.value() as i128);
        // D s_10_20: write-var ga#366974 <= s_10_19
        fn_state.ga_366974 = s_10_19;
        // N s_10_21: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#366974:i
        let s_11_0: i128 = fn_state.ga_366974;
        // D s_11_1: write-var index <= s_11_0
        fn_state.index = s_11_0;
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
        // N s_11_7: branch s_11_6 b38 b12
        if s_11_6 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1s : i64
        let s_12_0: i64 = 1;
        // D s_12_1: write-var ga#366976 <= s_12_0
        fn_state.ga_366976 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var ga#366976:i64
        let s_13_0: i64 = fn_state.ga_366976;
        // D s_13_1: write-var regs <= s_13_0
        fn_state.regs = s_13_0;
        // D s_13_2: read-var Q:u8
        let s_13_2: bool = fn_state.Q;
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // C s_13_4: const #1u : u8
        let s_13_4: bool = true;
        // C s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 1u16);
        // D s_13_6: cmp-eq s_13_3 s_13_5
        let s_13_6: bool = ((s_13_3) == (s_13_5));
        // N s_13_7: branch s_13_6 b37 b14
        if s_13_6 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #32s : i64
        let s_14_0: i64 = 32;
        // D s_14_1: write-var ga#366978 <= s_14_0
        fn_state.ga_366978 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ga#366978:i64
        let s_15_0: i64 = fn_state.ga_366978;
        // D s_15_1: write-var datasize <= s_15_0
        fn_state.datasize = s_15_0;
        // D s_15_2: read-var S:u8
        let s_15_2: bool = fn_state.S;
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // C s_15_4: const #1u : u8
        let s_15_4: bool = true;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 1u16);
        // D s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // D s_15_7: write-var sub_op <= s_15_6
        fn_state.sub_op = s_15_6;
        // D s_15_8: read-var n:i
        let s_15_8: i128 = fn_state.n;
        // D s_15_9: call __id(s_15_8)
        let s_15_9: i128 = u__id(state, tracer, s_15_8);
        // C s_15_10: const #0s : i
        let s_15_10: i128 = 0;
        // D s_15_11: cmp-le s_15_10 s_15_9
        let s_15_11: bool = ((s_15_10) <= (s_15_9));
        // N s_15_12: branch s_15_11 b18 b16
        if s_15_11 {
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
        // D s_16_1: write-var gs#326806 <= s_16_0
        fn_state.gs_326806 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#326806:u8
        let s_17_0: bool = fn_state.gs_326806;
        // N s_17_1: assert s_17_0
        let s_17_1: () = assert!(s_17_0);
        // D s_17_2: read-var datasize:i64
        let s_17_2: i64 = fn_state.datasize;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: cast reint s_17_3 -> i64
        let s_17_4: i64 = (s_17_3 as i64);
        // C s_17_5: const #32s : i64
        let s_17_5: i64 = 32;
        // D s_17_6: read-var index:i
        let s_17_6: i128 = fn_state.index;
        // D s_17_7: cast reint s_17_6 -> i64
        let s_17_7: i64 = (s_17_6 as i64);
        // D s_17_8: read-var m:i
        let s_17_8: i128 = fn_state.m;
        // D s_17_9: cast reint s_17_8 -> i64
        let s_17_9: i64 = (s_17_8 as i64);
        // D s_17_10: read-var n:i
        let s_17_10: i128 = fn_state.n;
        // D s_17_11: cast reint s_17_10 -> i64
        let s_17_11: i64 = (s_17_10 as i64);
        // D s_17_12: read-var Q:u8
        let s_17_12: bool = fn_state.Q;
        // D s_17_13: read-var d:i64
        let s_17_13: i64 = fn_state.d;
        // D s_17_14: read-var regs:i64
        let s_17_14: i64 = fn_state.regs;
        // D s_17_15: read-var sub_op:u8
        let s_17_15: bool = fn_state.sub_op;
        // D s_17_16: call execute_aarch32_instrs_VFMAL_i_Op_A_txt(s_17_12, s_17_13, s_17_4, s_17_5, s_17_7, s_17_9, s_17_11, s_17_14, s_17_15)
        let s_17_16: () = execute_aarch32_instrs_VFMAL_i_Op_A_txt(
            state,
            tracer,
            s_17_12,
            s_17_13,
            s_17_4,
            s_17_5,
            s_17_7,
            s_17_9,
            s_17_11,
            s_17_14,
            s_17_15,
        );
        // N s_17_17: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var n:i
        let s_18_0: i128 = fn_state.n;
        // D s_18_1: call __id(s_18_0)
        let s_18_1: i128 = u__id(state, tracer, s_18_0);
        // C s_18_2: const #31s : i
        let s_18_2: i128 = 31;
        // D s_18_3: cmp-le s_18_1 s_18_2
        let s_18_3: bool = ((s_18_1) <= (s_18_2));
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
        // D s_19_1: write-var gs#326805 <= s_19_0
        fn_state.gs_326805 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#326805:u8
        let s_20_0: bool = fn_state.gs_326805;
        // D s_20_1: write-var gs#326806 <= s_20_0
        fn_state.gs_326806 = s_20_0;
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
        // C s_21_2: const #0s : i
        let s_21_2: i128 = 0;
        // D s_21_3: cmp-le s_21_2 s_21_1
        let s_21_3: bool = ((s_21_2) <= (s_21_1));
        // N s_21_4: branch s_21_3 b24 b22
        if s_21_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#326804 <= s_22_0
        fn_state.gs_326804 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#326804:u8
        let s_23_0: bool = fn_state.gs_326804;
        // D s_23_1: write-var gs#326805 <= s_23_0
        fn_state.gs_326805 = s_23_0;
        // N s_23_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var m:i
        let s_24_0: i128 = fn_state.m;
        // D s_24_1: call __id(s_24_0)
        let s_24_1: i128 = u__id(state, tracer, s_24_0);
        // C s_24_2: const #15s : i
        let s_24_2: i128 = 15;
        // D s_24_3: cmp-le s_24_1 s_24_2
        let s_24_3: bool = ((s_24_1) <= (s_24_2));
        // N s_24_4: branch s_24_3 b27 b25
        if s_24_3 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#326803 <= s_25_0
        fn_state.gs_326803 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#326803:u8
        let s_26_0: bool = fn_state.gs_326803;
        // D s_26_1: write-var gs#326804 <= s_26_0
        fn_state.gs_326804 = s_26_0;
        // N s_26_2: jump b23
        return block_23(state, tracer, fn_state);
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
        // C s_27_2: const #0s : i
        let s_27_2: i128 = 0;
        // D s_27_3: cmp-eq s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) == (s_27_2));
        // N s_27_4: branch s_27_3 b36 b28
        if s_27_3 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var index:i
        let s_28_0: i128 = fn_state.index;
        // D s_28_1: call __id(s_28_0)
        let s_28_1: i128 = u__id(state, tracer, s_28_0);
        // C s_28_2: const #1s : i
        let s_28_2: i128 = 1;
        // D s_28_3: cmp-eq s_28_1 s_28_2
        let s_28_3: bool = ((s_28_1) == (s_28_2));
        // D s_28_4: write-var gs#326798 <= s_28_3
        fn_state.gs_326798 = s_28_3;
        // N s_28_5: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#326798:u8
        let s_29_0: bool = fn_state.gs_326798;
        // N s_29_1: branch s_29_0 b35 b30
        if s_29_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var index:i
        let s_30_0: i128 = fn_state.index;
        // D s_30_1: call __id(s_30_0)
        let s_30_1: i128 = u__id(state, tracer, s_30_0);
        // C s_30_2: const #2s : i
        let s_30_2: i128 = 2;
        // D s_30_3: cmp-eq s_30_1 s_30_2
        let s_30_3: bool = ((s_30_1) == (s_30_2));
        // D s_30_4: write-var gs#326800 <= s_30_3
        fn_state.gs_326800 = s_30_3;
        // N s_30_5: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#326800:u8
        let s_31_0: bool = fn_state.gs_326800;
        // N s_31_1: branch s_31_0 b34 b32
        if s_31_0 {
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
        // D s_32_0: read-var index:i
        let s_32_0: i128 = fn_state.index;
        // D s_32_1: call __id(s_32_0)
        let s_32_1: i128 = u__id(state, tracer, s_32_0);
        // C s_32_2: const #3s : i
        let s_32_2: i128 = 3;
        // D s_32_3: cmp-eq s_32_1 s_32_2
        let s_32_3: bool = ((s_32_1) == (s_32_2));
        // D s_32_4: write-var gs#326802 <= s_32_3
        fn_state.gs_326802 = s_32_3;
        // N s_32_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#326802:u8
        let s_33_0: bool = fn_state.gs_326802;
        // D s_33_1: write-var gs#326803 <= s_33_0
        fn_state.gs_326803 = s_33_0;
        // N s_33_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#326802 <= s_34_0
        fn_state.gs_326802 = s_34_0;
        // N s_34_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#326800 <= s_35_0
        fn_state.gs_326800 = s_35_0;
        // N s_35_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#326798 <= s_36_0
        fn_state.gs_326798 = s_36_0;
        // N s_36_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #64s : i64
        let s_37_0: i64 = 64;
        // D s_37_1: write-var ga#366978 <= s_37_0
        fn_state.ga_366978 = s_37_0;
        // N s_37_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #2s : i64
        let s_38_0: i64 = 2;
        // D s_38_1: write-var ga#366976 <= s_38_0
        fn_state.ga_366976 = s_38_0;
        // N s_38_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #3s : i
        let s_39_0: i128 = 3;
        // D s_39_1: read-var Vm:u8
        let s_39_1: u8 = fn_state.Vm;
        // D s_39_2: cast zx s_39_1 -> bv
        let s_39_2: Bits = Bits::new(s_39_1 as u128, 4u16);
        // C s_39_3: const #1u : u64
        let s_39_3: u64 = 1;
        // D s_39_4: bit-extract s_39_2 s_39_0 s_39_3
        let s_39_4: Bits = (Bits::new(
            ((s_39_2) >> (s_39_0)).value(),
            u16::try_from(s_39_3).unwrap(),
        ));
        // D s_39_5: cast reint s_39_4 -> u8
        let s_39_5: bool = ((s_39_4.value()) != 0);
        // C s_39_6: const #0s : i
        let s_39_6: i128 = 0;
        // C s_39_7: const #0u : u64
        let s_39_7: u64 = 0;
        // D s_39_8: cast zx s_39_5 -> u64
        let s_39_8: u64 = (s_39_5 as u64);
        // C s_39_9: const #1u : u64
        let s_39_9: u64 = 1;
        // D s_39_10: and s_39_8 s_39_9
        let s_39_10: u64 = ((s_39_8) & (s_39_9));
        // D s_39_11: cmp-eq s_39_10 s_39_9
        let s_39_11: bool = ((s_39_10) == (s_39_9));
        // D s_39_12: lsl s_39_8 s_39_6
        let s_39_12: u64 = s_39_8 << s_39_6;
        // D s_39_13: or s_39_7 s_39_12
        let s_39_13: u64 = ((s_39_7) | (s_39_12));
        // D s_39_14: cmpl s_39_12
        let s_39_14: u64 = !s_39_12;
        // D s_39_15: and s_39_7 s_39_14
        let s_39_15: u64 = ((s_39_7) & (s_39_14));
        // D s_39_16: select s_39_11 s_39_13 s_39_15
        let s_39_16: u64 = if s_39_11 { s_39_13 } else { s_39_15 };
        // D s_39_17: cast trunc s_39_16 -> u8
        let s_39_17: bool = ((s_39_16) != 0);
        // D s_39_18: read-var M:u8
        let s_39_18: bool = fn_state.M;
        // D s_39_19: cast zx s_39_18 -> bv
        let s_39_19: Bits = Bits::new(s_39_18 as u128, 1u16);
        // D s_39_20: cast zx s_39_17 -> bv
        let s_39_20: Bits = Bits::new(s_39_17 as u128, 1u16);
        // D s_39_21: cast reint s_39_19 -> u128
        let s_39_21: u128 = (s_39_19.value() as u128);
        // D s_39_22: size-of s_39_19
        let s_39_22: u16 = s_39_19.length();
        // D s_39_23: cast reint s_39_20 -> u128
        let s_39_23: u128 = (s_39_20.value() as u128);
        // D s_39_24: size-of s_39_20
        let s_39_24: u16 = s_39_20.length();
        // D s_39_25: lsl s_39_21 s_39_24
        let s_39_25: u128 = s_39_21 << s_39_24;
        // D s_39_26: or s_39_25 s_39_23
        let s_39_26: u128 = ((s_39_25) | (s_39_23));
        // D s_39_27: add s_39_22 s_39_24
        let s_39_27: u16 = (s_39_22 + s_39_24);
        // D s_39_28: create-bits s_39_26 s_39_27
        let s_39_28: Bits = Bits::new(s_39_26, s_39_27);
        // D s_39_29: cast reint s_39_28 -> u8
        let s_39_29: u8 = (s_39_28.value() as u8);
        // D s_39_30: cast zx s_39_29 -> bv
        let s_39_30: Bits = Bits::new(s_39_29 as u128, 2u16);
        // D s_39_31: cast zx s_39_30 -> i
        let s_39_31: i128 = (s_39_30.value() as i128);
        // D s_39_32: write-var ga#366974 <= s_39_31
        fn_state.ga_366974 = s_39_31;
        // N s_39_33: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0s : i
        let s_40_0: i128 = 0;
        // D s_40_1: read-var Vm:u8
        let s_40_1: u8 = fn_state.Vm;
        // D s_40_2: cast zx s_40_1 -> bv
        let s_40_2: Bits = Bits::new(s_40_1 as u128, 4u16);
        // C s_40_3: const #1s : i64
        let s_40_3: i64 = 1;
        // C s_40_4: cast zx s_40_3 -> i
        let s_40_4: i128 = (i128::try_from(s_40_3).unwrap());
        // C s_40_5: const #2s : i
        let s_40_5: i128 = 2;
        // C s_40_6: add s_40_5 s_40_4
        let s_40_6: i128 = (s_40_5 + s_40_4);
        // D s_40_7: bit-extract s_40_2 s_40_0 s_40_6
        let s_40_7: Bits = (Bits::new(
            ((s_40_2) >> (s_40_0)).value(),
            u16::try_from(s_40_6).unwrap(),
        ));
        // D s_40_8: cast reint s_40_7 -> u8
        let s_40_8: u8 = (s_40_7.value() as u8);
        // D s_40_9: cast zx s_40_8 -> bv
        let s_40_9: Bits = Bits::new(s_40_8 as u128, 3u16);
        // D s_40_10: cast zx s_40_9 -> i
        let s_40_10: i128 = (s_40_9.value() as i128);
        // D s_40_11: write-var ga#366967 <= s_40_10
        fn_state.ga_366967 = s_40_10;
        // N s_40_12: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var N:u8
        let s_41_0: bool = fn_state.N;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 1u16);
        // D s_41_2: read-var Vn:u8
        let s_41_2: u8 = fn_state.Vn;
        // D s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 4u16);
        // D s_41_4: cast reint s_41_1 -> u128
        let s_41_4: u128 = (s_41_1.value() as u128);
        // D s_41_5: size-of s_41_1
        let s_41_5: u16 = s_41_1.length();
        // D s_41_6: cast reint s_41_3 -> u128
        let s_41_6: u128 = (s_41_3.value() as u128);
        // D s_41_7: size-of s_41_3
        let s_41_7: u16 = s_41_3.length();
        // D s_41_8: lsl s_41_4 s_41_7
        let s_41_8: u128 = s_41_4 << s_41_7;
        // D s_41_9: or s_41_8 s_41_6
        let s_41_9: u128 = ((s_41_8) | (s_41_6));
        // D s_41_10: add s_41_5 s_41_7
        let s_41_10: u16 = (s_41_5 + s_41_7);
        // D s_41_11: create-bits s_41_9 s_41_10
        let s_41_11: Bits = Bits::new(s_41_9, s_41_10);
        // D s_41_12: cast reint s_41_11 -> u8
        let s_41_12: u8 = (s_41_11.value() as u8);
        // D s_41_13: cast zx s_41_12 -> bv
        let s_41_13: Bits = Bits::new(s_41_12 as u128, 5u16);
        // D s_41_14: cast zx s_41_13 -> i
        let s_41_14: i128 = (s_41_13.value() as i128);
        // D s_41_15: write-var ga#366962 <= s_41_14
        fn_state.ga_366962 = s_41_14;
        // N s_41_16: jump b7
        return block_7(state, tracer, fn_state);
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
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0s : i
        let s_43_0: i128 = 0;
        // D s_43_1: read-var Vd:u8
        let s_43_1: u8 = fn_state.Vd;
        // D s_43_2: cast zx s_43_1 -> bv
        let s_43_2: Bits = Bits::new(s_43_1 as u128, 4u16);
        // C s_43_3: const #1u : u64
        let s_43_3: u64 = 1;
        // D s_43_4: bit-extract s_43_2 s_43_0 s_43_3
        let s_43_4: Bits = (Bits::new(
            ((s_43_2) >> (s_43_0)).value(),
            u16::try_from(s_43_3).unwrap(),
        ));
        // D s_43_5: cast reint s_43_4 -> u8
        let s_43_5: bool = ((s_43_4.value()) != 0);
        // C s_43_6: const #0s : i
        let s_43_6: i128 = 0;
        // C s_43_7: const #0u : u64
        let s_43_7: u64 = 0;
        // D s_43_8: cast zx s_43_5 -> u64
        let s_43_8: u64 = (s_43_5 as u64);
        // C s_43_9: const #1u : u64
        let s_43_9: u64 = 1;
        // D s_43_10: and s_43_8 s_43_9
        let s_43_10: u64 = ((s_43_8) & (s_43_9));
        // D s_43_11: cmp-eq s_43_10 s_43_9
        let s_43_11: bool = ((s_43_10) == (s_43_9));
        // D s_43_12: lsl s_43_8 s_43_6
        let s_43_12: u64 = s_43_8 << s_43_6;
        // D s_43_13: or s_43_7 s_43_12
        let s_43_13: u64 = ((s_43_7) | (s_43_12));
        // D s_43_14: cmpl s_43_12
        let s_43_14: u64 = !s_43_12;
        // D s_43_15: and s_43_7 s_43_14
        let s_43_15: u64 = ((s_43_7) & (s_43_14));
        // D s_43_16: select s_43_11 s_43_13 s_43_15
        let s_43_16: u64 = if s_43_11 { s_43_13 } else { s_43_15 };
        // D s_43_17: cast trunc s_43_16 -> u8
        let s_43_17: bool = ((s_43_16) != 0);
        // D s_43_18: cast zx s_43_17 -> bv
        let s_43_18: Bits = Bits::new(s_43_17 as u128, 1u16);
        // C s_43_19: const #1u : u8
        let s_43_19: bool = true;
        // C s_43_20: cast zx s_43_19 -> bv
        let s_43_20: Bits = Bits::new(s_43_19 as u128, 1u16);
        // D s_43_21: cmp-eq s_43_18 s_43_20
        let s_43_21: bool = ((s_43_18) == (s_43_20));
        // D s_43_22: write-var gs#326770 <= s_43_21
        fn_state.gs_326770 = s_43_21;
        // N s_43_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: panic
        panic!("{:?}", ());
        // N s_44_1: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: panic
        panic!("{:?}", ());
        // N s_45_1: return
        return;
    }
}
