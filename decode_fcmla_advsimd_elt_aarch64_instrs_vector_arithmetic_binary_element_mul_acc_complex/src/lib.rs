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
use execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_complex::*;
use HaveFCADDExt::*;
use u__id::*;
use common::*;
pub fn decode_fcmla_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_complex<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    H: bool,
    rot: u8,
    Rm: u8,
    M: bool,
    L: bool,
    size: u8,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        n: i64,
        index: i128,
        indexshadow_1282: i128,
        ga_253537: i64,
        gs_149748: bool,
        gs_149746: bool,
        gs_149751: bool,
        gs_149747: bool,
        gs_149755: bool,
        gs_149733: bool,
        d: i64,
        gs_149745: bool,
        gs_149753: bool,
        elements: i64,
        datasize: i64,
        Rd: u8,
        Rn: u8,
        H: bool,
        rot: u8,
        Rm: u8,
        M: bool,
        L: bool,
        size: u8,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        H,
        rot,
        Rm,
        M,
        L,
        size,
        Q,
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
        // S s_0_1: call HaveFCADDExt(s_0_0)
        let s_0_1: bool = HaveFCADDExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b41 b1
        if s_0_2 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rd:u8
        let s_1_0: u8 = fn_state.Rd;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var d <= s_1_3
        fn_state.d = s_1_3;
        // D s_1_5: read-var Rn:u8
        let s_1_5: u8 = fn_state.Rn;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var n <= s_1_8
        fn_state.n = s_1_8;
        // D s_1_10: read-var M:u8
        let s_1_10: bool = fn_state.M;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 1u16);
        // D s_1_12: read-var Rm:u8
        let s_1_12: u8 = fn_state.Rm;
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 4u16);
        // D s_1_14: cast reint s_1_11 -> u128
        let s_1_14: u128 = (s_1_11.value() as u128);
        // D s_1_15: size-of s_1_11
        let s_1_15: u16 = s_1_11.length();
        // D s_1_16: cast reint s_1_13 -> u128
        let s_1_16: u128 = (s_1_13.value() as u128);
        // D s_1_17: size-of s_1_13
        let s_1_17: u16 = s_1_13.length();
        // D s_1_18: lsl s_1_14 s_1_17
        let s_1_18: u128 = s_1_14 << s_1_17;
        // D s_1_19: or s_1_18 s_1_16
        let s_1_19: u128 = ((s_1_18) | (s_1_16));
        // D s_1_20: add s_1_15 s_1_17
        let s_1_20: u16 = (s_1_15 + s_1_17);
        // D s_1_21: create-bits s_1_19 s_1_20
        let s_1_21: Bits = Bits::new(s_1_19, s_1_20);
        // D s_1_22: cast reint s_1_21 -> u8
        let s_1_22: u8 = (s_1_21.value() as u8);
        // D s_1_23: cast zx s_1_22 -> bv
        let s_1_23: Bits = Bits::new(s_1_22 as u128, 5u16);
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (s_1_23.value() as i128);
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: write-var m <= s_1_25
        fn_state.m = s_1_25;
        // D s_1_27: read-var size:u8
        let s_1_27: u8 = fn_state.size;
        // D s_1_28: cast zx s_1_27 -> bv
        let s_1_28: Bits = Bits::new(s_1_27 as u128, 2u16);
        // C s_1_29: const #0u : u8
        let s_1_29: u8 = 0;
        // C s_1_30: cast zx s_1_29 -> bv
        let s_1_30: Bits = Bits::new(s_1_29 as u128, 2u16);
        // D s_1_31: cmp-eq s_1_28 s_1_30
        let s_1_31: bool = ((s_1_28) == (s_1_30));
        // N s_1_32: branch s_1_31 b40 b2
        if s_1_31 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
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
        // D s_2_5: write-var gs#149733 <= s_2_4
        fn_state.gs_149733 = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#149733:u8
        let s_3_0: bool = fn_state.gs_149733;
        // N s_3_1: branch s_3_0 b39 b4
        if s_3_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var size:u8
        let s_4_0: u8 = fn_state.size;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #1u : u8
        let s_4_2: u8 = 1;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b38 b5
        if s_4_4 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var size:u8
        let s_6_0: u8 = fn_state.size;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b37 b7
        if s_6_4 {
            return block_37(state, tracer, fn_state);
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
        // D s_8_0: read-var index:i
        let s_8_0: i128 = fn_state.index;
        // D s_8_1: write-var indexshadow#1282 <= s_8_0
        fn_state.indexshadow_1282 = s_8_0;
        // D s_8_2: read-var size:u8
        let s_8_2: u8 = fn_state.size;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (s_8_3.value() as i128);
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // C s_8_6: const #8s : i64
        let s_8_6: i64 = 8;
        // D s_8_7: lsl s_8_6 s_8_5
        let s_8_7: i64 = s_8_6 << s_8_5;
        // D s_8_8: write-var esize <= s_8_7
        fn_state.esize = s_8_7;
        // D s_8_9: read-var Q:u8
        let s_8_9: bool = fn_state.Q;
        // D s_8_10: cast zx s_8_9 -> bv
        let s_8_10: Bits = Bits::new(s_8_9 as u128, 1u16);
        // C s_8_11: const #1u : u8
        let s_8_11: bool = true;
        // C s_8_12: cast zx s_8_11 -> bv
        let s_8_12: Bits = Bits::new(s_8_11 as u128, 1u16);
        // D s_8_13: cmp-eq s_8_10 s_8_12
        let s_8_13: bool = ((s_8_10) == (s_8_12));
        // N s_8_14: branch s_8_13 b36 b9
        if s_8_13 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // D s_9_1: write-var ga#253537 <= s_9_0
        fn_state.ga_253537 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#253537:i64
        let s_10_0: i64 = fn_state.ga_253537;
        // D s_10_1: write-var datasize <= s_10_0
        fn_state.datasize = s_10_0;
        // D s_10_2: read-var datasize:i64
        let s_10_2: i64 = fn_state.datasize;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: read-var esize:i64
        let s_10_4: i64 = fn_state.esize;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: div s_10_3 s_10_5
        let s_10_6: i128 = ((s_10_3) / (s_10_5));
        // D s_10_7: cast reint s_10_6 -> i64
        let s_10_7: i64 = (s_10_6 as i64);
        // D s_10_8: write-var elements <= s_10_7
        fn_state.elements = s_10_7;
        // D s_10_9: read-var size:u8
        let s_10_9: u8 = fn_state.size;
        // D s_10_10: cast zx s_10_9 -> bv
        let s_10_10: Bits = Bits::new(s_10_9 as u128, 2u16);
        // C s_10_11: const #2u : u8
        let s_10_11: u8 = 2;
        // C s_10_12: cast zx s_10_11 -> bv
        let s_10_12: Bits = Bits::new(s_10_11 as u128, 2u16);
        // D s_10_13: cmp-eq s_10_10 s_10_12
        let s_10_13: bool = ((s_10_10) == (s_10_12));
        // N s_10_14: branch s_10_13 b32 b11
        if s_10_13 {
            return block_32(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#149746 <= s_11_0
        fn_state.gs_149746 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#149746:u8
        let s_12_0: bool = fn_state.gs_149746;
        // N s_12_1: branch s_12_0 b31 b13
        if s_12_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var size:u8
        let s_13_0: u8 = fn_state.size;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #1u : u8
        let s_13_2: u8 = 1;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // N s_13_5: branch s_13_4 b30 b14
        if s_13_4 {
            return block_30(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#149747 <= s_14_0
        fn_state.gs_149747 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#149747:u8
        let s_15_0: bool = fn_state.gs_149747;
        // N s_15_1: branch s_15_0 b29 b16
        if s_15_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#149748 <= s_16_0
        fn_state.gs_149748 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#149748:u8
        let s_17_0: bool = fn_state.gs_149748;
        // N s_17_1: branch s_17_0 b28 b18
        if s_17_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var indexshadow#1282:i
        let s_18_0: i128 = fn_state.indexshadow_1282;
        // D s_18_1: call __id(s_18_0)
        let s_18_1: i128 = u__id(state, tracer, s_18_0);
        // C s_18_2: const #0s : i
        let s_18_2: i128 = 0;
        // D s_18_3: cmp-eq s_18_1 s_18_2
        let s_18_3: bool = ((s_18_1) == (s_18_2));
        // N s_18_4: branch s_18_3 b27 b19
        if s_18_3 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var indexshadow#1282:i
        let s_19_0: i128 = fn_state.indexshadow_1282;
        // D s_19_1: call __id(s_19_0)
        let s_19_1: i128 = u__id(state, tracer, s_19_0);
        // C s_19_2: const #1s : i
        let s_19_2: i128 = 1;
        // D s_19_3: cmp-eq s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) == (s_19_2));
        // D s_19_4: write-var gs#149751 <= s_19_3
        fn_state.gs_149751 = s_19_3;
        // N s_19_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#149751:u8
        let s_20_0: bool = fn_state.gs_149751;
        // N s_20_1: branch s_20_0 b26 b21
        if s_20_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var indexshadow#1282:i
        let s_21_0: i128 = fn_state.indexshadow_1282;
        // D s_21_1: call __id(s_21_0)
        let s_21_1: i128 = u__id(state, tracer, s_21_0);
        // C s_21_2: const #2s : i
        let s_21_2: i128 = 2;
        // D s_21_3: cmp-eq s_21_1 s_21_2
        let s_21_3: bool = ((s_21_1) == (s_21_2));
        // D s_21_4: write-var gs#149753 <= s_21_3
        fn_state.gs_149753 = s_21_3;
        // N s_21_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#149753:u8
        let s_22_0: bool = fn_state.gs_149753;
        // N s_22_1: branch s_22_0 b25 b23
        if s_22_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var indexshadow#1282:i
        let s_23_0: i128 = fn_state.indexshadow_1282;
        // D s_23_1: call __id(s_23_0)
        let s_23_1: i128 = u__id(state, tracer, s_23_0);
        // C s_23_2: const #3s : i
        let s_23_2: i128 = 3;
        // D s_23_3: cmp-eq s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) == (s_23_2));
        // D s_23_4: write-var gs#149755 <= s_23_3
        fn_state.gs_149755 = s_23_3;
        // N s_23_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#149755:u8
        let s_24_0: bool = fn_state.gs_149755;
        // N s_24_1: assert s_24_0
        let s_24_1: () = assert!(s_24_0);
        // D s_24_2: read-var datasize:i64
        let s_24_2: i64 = fn_state.datasize;
        // D s_24_3: cast zx s_24_2 -> i
        let s_24_3: i128 = (i128::try_from(s_24_2).unwrap());
        // D s_24_4: cast reint s_24_3 -> i64
        let s_24_4: i64 = (s_24_3 as i64);
        // D s_24_5: read-var esize:i64
        let s_24_5: i64 = fn_state.esize;
        // D s_24_6: cast zx s_24_5 -> i
        let s_24_6: i128 = (i128::try_from(s_24_5).unwrap());
        // D s_24_7: cast reint s_24_6 -> i64
        let s_24_7: i64 = (s_24_6 as i64);
        // D s_24_8: read-var elements:i64
        let s_24_8: i64 = fn_state.elements;
        // D s_24_9: cast zx s_24_8 -> i
        let s_24_9: i128 = (i128::try_from(s_24_8).unwrap());
        // D s_24_10: read-var indexshadow#1282:i
        let s_24_10: i128 = fn_state.indexshadow_1282;
        // D s_24_11: cast reint s_24_10 -> i64
        let s_24_11: i64 = (s_24_10 as i64);
        // D s_24_12: read-var d:i64
        let s_24_12: i64 = fn_state.d;
        // D s_24_13: read-var m:i64
        let s_24_13: i64 = fn_state.m;
        // D s_24_14: read-var n:i64
        let s_24_14: i64 = fn_state.n;
        // D s_24_15: read-var rot:u8
        let s_24_15: u8 = fn_state.rot;
        // D s_24_16: call execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_complex(s_24_12, s_24_4, s_24_9, s_24_7, s_24_11, s_24_13, s_24_14, s_24_15)
        let s_24_16: () = execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_complex(
            state,
            tracer,
            s_24_12,
            s_24_4,
            s_24_9,
            s_24_7,
            s_24_11,
            s_24_13,
            s_24_14,
            s_24_15,
        );
        // N s_24_17: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#149755 <= s_25_0
        fn_state.gs_149755 = s_25_0;
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#149753 <= s_26_0
        fn_state.gs_149753 = s_26_0;
        // N s_26_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#149751 <= s_27_0
        fn_state.gs_149751 = s_27_0;
        // N s_27_2: jump b20
        return block_20(state, tracer, fn_state);
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
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var Q:u8
        let s_29_0: bool = fn_state.Q;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #0u : u8
        let s_29_2: bool = false;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#149748 <= s_29_4
        fn_state.gs_149748 = s_29_4;
        // N s_29_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var H:u8
        let s_30_0: bool = fn_state.H;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#149747 <= s_30_4
        fn_state.gs_149747 = s_30_4;
        // N s_30_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var L:u8
        let s_32_0: bool = fn_state.L;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // N s_32_5: branch s_32_4 b35 b33
        if s_32_4 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var Q:u8
        let s_33_0: bool = fn_state.Q;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #0u : u8
        let s_33_2: bool = false;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#149745 <= s_33_4
        fn_state.gs_149745 = s_33_4;
        // N s_33_6: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#149745:u8
        let s_34_0: bool = fn_state.gs_149745;
        // D s_34_1: write-var gs#149746 <= s_34_0
        fn_state.gs_149746 = s_34_0;
        // N s_34_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#149745 <= s_35_0
        fn_state.gs_149745 = s_35_0;
        // N s_35_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #128s : i64
        let s_36_0: i64 = 128;
        // D s_36_1: write-var ga#253537 <= s_36_0
        fn_state.ga_253537 = s_36_0;
        // N s_36_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var H:u8
        let s_37_0: bool = fn_state.H;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // D s_37_2: cast zx s_37_1 -> i
        let s_37_2: i128 = (s_37_1.value() as i128);
        // D s_37_3: write-var index <= s_37_2
        fn_state.index = s_37_2;
        // N s_37_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var H:u8
        let s_38_0: bool = fn_state.H;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // D s_38_2: read-var L:u8
        let s_38_2: bool = fn_state.L;
        // D s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cast reint s_38_1 -> u128
        let s_38_4: u128 = (s_38_1.value() as u128);
        // D s_38_5: size-of s_38_1
        let s_38_5: u16 = s_38_1.length();
        // D s_38_6: cast reint s_38_3 -> u128
        let s_38_6: u128 = (s_38_3.value() as u128);
        // D s_38_7: size-of s_38_3
        let s_38_7: u16 = s_38_3.length();
        // D s_38_8: lsl s_38_4 s_38_7
        let s_38_8: u128 = s_38_4 << s_38_7;
        // D s_38_9: or s_38_8 s_38_6
        let s_38_9: u128 = ((s_38_8) | (s_38_6));
        // D s_38_10: add s_38_5 s_38_7
        let s_38_10: u16 = (s_38_5 + s_38_7);
        // D s_38_11: create-bits s_38_9 s_38_10
        let s_38_11: Bits = Bits::new(s_38_9, s_38_10);
        // D s_38_12: cast reint s_38_11 -> u8
        let s_38_12: u8 = (s_38_11.value() as u8);
        // D s_38_13: cast zx s_38_12 -> bv
        let s_38_13: Bits = Bits::new(s_38_12 as u128, 2u16);
        // D s_38_14: cast zx s_38_13 -> i
        let s_38_14: i128 = (s_38_13.value() as i128);
        // D s_38_15: write-var index <= s_38_14
        fn_state.index = s_38_14;
        // N s_38_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: panic
        panic!("{:?}", ());
        // N s_39_1: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#149733 <= s_40_0
        fn_state.gs_149733 = s_40_0;
        // N s_40_2: jump b3
        return block_3(state, tracer, fn_state);
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
}
