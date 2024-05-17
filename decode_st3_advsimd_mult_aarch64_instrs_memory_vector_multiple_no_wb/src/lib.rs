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
use neq_int::*;
use execute_aarch64_instrs_memory_vector_multiple_no_wb::*;
use u__UNKNOWN_integer::*;
use common::*;
pub fn decode_st3_advsimd_mult_aarch64_instrs_memory_vector_multiple_no_wb<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    size: u8,
    opcode: u8,
    L: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i128,
        t: i64,
        esize: i64,
        ga_258296: i64,
        n: i64,
        memop: u32,
        selemshadow_1566: i64,
        rpt: i64,
        tagchecked: bool,
        selem: i64,
        gs_157489: bool,
        elements: i64,
        rptshadow_1567: i64,
        datasize: i64,
        Rt: u8,
        Rn: u8,
        size: u8,
        opcode: u8,
        L: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        size,
        opcode,
        L,
        Q,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rt:u8
        let s_0_0: u8 = fn_state.Rt;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var t <= s_0_3
        fn_state.t = s_0_3;
        // D s_0_5: read-var Rn:u8
        let s_0_5: u8 = fn_state.Rn;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var n <= s_0_8
        fn_state.n = s_0_8;
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call __UNKNOWN_integer(s_0_10)
        let s_0_11: i128 = u__UNKNOWN_integer(state, tracer, s_0_10);
        // D s_0_12: write-var m <= s_0_11
        fn_state.m = s_0_11;
        // C s_0_13: const #31s : i
        let s_0_13: i128 = 31;
        // D s_0_14: read-var n:i64
        let s_0_14: i64 = fn_state.n;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: call neq_int(s_0_15, s_0_13)
        let s_0_16: bool = neq_int(state, tracer, s_0_15, s_0_13);
        // D s_0_17: write-var tagchecked <= s_0_16
        fn_state.tagchecked = s_0_16;
        // D s_0_18: read-var L:u8
        let s_0_18: bool = fn_state.L;
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 1u16);
        // C s_0_20: const #1u : u8
        let s_0_20: bool = true;
        // C s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 1u16);
        // D s_0_22: cmp-eq s_0_19 s_0_21
        let s_0_22: bool = ((s_0_19) == (s_0_21));
        // N s_0_23: branch s_0_22 b26 b1
        if s_0_22 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1u : u32
        let s_1_0: u32 = 1;
        // D s_1_1: write-var memop <= s_1_0
        fn_state.memop = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
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
        // N s_2_5: branch s_2_4 b25 b3
        if s_2_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: write-var ga#258296 <= s_3_0
        fn_state.ga_258296 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#258296:i64
        let s_4_0: i64 = fn_state.ga_258296;
        // D s_4_1: write-var datasize <= s_4_0
        fn_state.datasize = s_4_0;
        // D s_4_2: read-var size:u8
        let s_4_2: u8 = fn_state.size;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (s_4_3.value() as i128);
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // C s_4_6: const #8s : i64
        let s_4_6: i64 = 8;
        // D s_4_7: lsl s_4_6 s_4_5
        let s_4_7: i64 = s_4_6 << s_4_5;
        // D s_4_8: write-var esize <= s_4_7
        fn_state.esize = s_4_7;
        // D s_4_9: read-var datasize:i64
        let s_4_9: i64 = fn_state.datasize;
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: read-var esize:i64
        let s_4_11: i64 = fn_state.esize;
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_13: div s_4_10 s_4_12
        let s_4_13: i128 = ((s_4_10) / (s_4_12));
        // D s_4_14: cast reint s_4_13 -> i64
        let s_4_14: i64 = (s_4_13 as i64);
        // D s_4_15: write-var elements <= s_4_14
        fn_state.elements = s_4_14;
        // C s_4_16: const #1s : i64
        let s_4_16: i64 = 1;
        // D s_4_17: write-var rpt <= s_4_16
        fn_state.rpt = s_4_16;
        // C s_4_18: const #1s : i64
        let s_4_18: i64 = 1;
        // D s_4_19: write-var selem <= s_4_18
        fn_state.selem = s_4_18;
        // D s_4_20: read-var opcode:u8
        let s_4_20: u8 = fn_state.opcode;
        // D s_4_21: cast zx s_4_20 -> bv
        let s_4_21: Bits = Bits::new(s_4_20 as u128, 4u16);
        // C s_4_22: const #0u : u8
        let s_4_22: u8 = 0;
        // C s_4_23: cast zx s_4_22 -> bv
        let s_4_23: Bits = Bits::new(s_4_22 as u128, 4u16);
        // D s_4_24: cmp-eq s_4_21 s_4_23
        let s_4_24: bool = ((s_4_21) == (s_4_23));
        // D s_4_25: not s_4_24
        let s_4_25: bool = !s_4_24;
        // N s_4_26: branch s_4_25 b12 b5
        if s_4_25 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i64
        let s_5_0: i64 = 1;
        // D s_5_1: write-var rpt <= s_5_0
        fn_state.rpt = s_5_0;
        // C s_5_2: const #4s : i64
        let s_5_2: i64 = 4;
        // D s_5_3: write-var selem <= s_5_2
        fn_state.selem = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var selem:i64
        let s_6_0: i64 = fn_state.selem;
        // D s_6_1: write-var selemshadow#1566 <= s_6_0
        fn_state.selemshadow_1566 = s_6_0;
        // D s_6_2: read-var rpt:i64
        let s_6_2: i64 = fn_state.rpt;
        // D s_6_3: write-var rptshadow#1567 <= s_6_2
        fn_state.rptshadow_1567 = s_6_2;
        // D s_6_4: read-var size:u8
        let s_6_4: u8 = fn_state.size;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 2u16);
        // D s_6_6: read-var Q:u8
        let s_6_6: bool = fn_state.Q;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 1u16);
        // D s_6_8: cast reint s_6_5 -> u128
        let s_6_8: u128 = (s_6_5.value() as u128);
        // D s_6_9: size-of s_6_5
        let s_6_9: u16 = s_6_5.length();
        // D s_6_10: cast reint s_6_7 -> u128
        let s_6_10: u128 = (s_6_7.value() as u128);
        // D s_6_11: size-of s_6_7
        let s_6_11: u16 = s_6_7.length();
        // D s_6_12: lsl s_6_8 s_6_11
        let s_6_12: u128 = s_6_8 << s_6_11;
        // D s_6_13: or s_6_12 s_6_10
        let s_6_13: u128 = ((s_6_12) | (s_6_10));
        // D s_6_14: add s_6_9 s_6_11
        let s_6_14: u16 = (s_6_9 + s_6_11);
        // D s_6_15: create-bits s_6_13 s_6_14
        let s_6_15: Bits = Bits::new(s_6_13, s_6_14);
        // D s_6_16: cast reint s_6_15 -> u8
        let s_6_16: u8 = (s_6_15.value() as u8);
        // D s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 3u16);
        // C s_6_18: const #6u : u8
        let s_6_18: u8 = 6;
        // C s_6_19: cast zx s_6_18 -> bv
        let s_6_19: Bits = Bits::new(s_6_18 as u128, 3u16);
        // D s_6_20: cmp-eq s_6_17 s_6_19
        let s_6_20: bool = ((s_6_17) == (s_6_19));
        // N s_6_21: branch s_6_20 b11 b7
        if s_6_20 {
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
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#157489 <= s_7_0
        fn_state.gs_157489 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#157489:u8
        let s_8_0: bool = fn_state.gs_157489;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
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
        // D s_9_0: read-var datasize:i64
        let s_9_0: i64 = fn_state.datasize;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var esize:i64
        let s_9_3: i64 = fn_state.esize;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // D s_9_6: read-var elements:i64
        let s_9_6: i64 = fn_state.elements;
        // D s_9_7: cast zx s_9_6 -> i
        let s_9_7: i128 = (i128::try_from(s_9_6).unwrap());
        // D s_9_8: read-var m:i
        let s_9_8: i128 = fn_state.m;
        // D s_9_9: read-var memop:u32
        let s_9_9: u32 = fn_state.memop;
        // D s_9_10: read-var n:i64
        let s_9_10: i64 = fn_state.n;
        // C s_9_11: const #0u : u8
        let s_9_11: bool = false;
        // D s_9_12: read-var rptshadow#1567:i64
        let s_9_12: i64 = fn_state.rptshadow_1567;
        // D s_9_13: read-var selemshadow#1566:i64
        let s_9_13: i64 = fn_state.selemshadow_1566;
        // D s_9_14: read-var t:i64
        let s_9_14: i64 = fn_state.t;
        // D s_9_15: read-var tagchecked:u8
        let s_9_15: bool = fn_state.tagchecked;
        // C s_9_16: const #0u : u8
        let s_9_16: bool = false;
        // D s_9_17: call execute_aarch64_instrs_memory_vector_multiple_no_wb(s_9_2, s_9_7, s_9_5, s_9_8, s_9_9, s_9_10, s_9_11, s_9_12, s_9_13, s_9_14, s_9_15, s_9_16)
        let s_9_17: () = execute_aarch64_instrs_memory_vector_multiple_no_wb(
            state,
            tracer,
            s_9_2,
            s_9_7,
            s_9_5,
            s_9_8,
            s_9_9,
            s_9_10,
            s_9_11,
            s_9_12,
            s_9_13,
            s_9_14,
            s_9_15,
            s_9_16,
        );
        // N s_9_18: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1s : i
        let s_11_0: i128 = 1;
        // D s_11_1: read-var selemshadow#1566:i64
        let s_11_1: i64 = fn_state.selemshadow_1566;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: call neq_int(s_11_2, s_11_0)
        let s_11_3: bool = neq_int(state, tracer, s_11_2, s_11_0);
        // D s_11_4: write-var gs#157489 <= s_11_3
        fn_state.gs_157489 = s_11_3;
        // N s_11_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var opcode:u8
        let s_12_0: u8 = fn_state.opcode;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 4u16);
        // C s_12_2: const #2u : u8
        let s_12_2: u8 = 2;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 4u16);
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
        // C s_13_0: const #4s : i64
        let s_13_0: i64 = 4;
        // D s_13_1: write-var rpt <= s_13_0
        fn_state.rpt = s_13_0;
        // C s_13_2: const #1s : i64
        let s_13_2: i64 = 1;
        // D s_13_3: write-var selem <= s_13_2
        fn_state.selem = s_13_2;
        // N s_13_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var opcode:u8
        let s_14_0: u8 = fn_state.opcode;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 4u16);
        // C s_14_2: const #4u : u8
        let s_14_2: u8 = 4;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 4u16);
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
        // C s_15_0: const #1s : i64
        let s_15_0: i64 = 1;
        // D s_15_1: write-var rpt <= s_15_0
        fn_state.rpt = s_15_0;
        // C s_15_2: const #3s : i64
        let s_15_2: i64 = 3;
        // D s_15_3: write-var selem <= s_15_2
        fn_state.selem = s_15_2;
        // N s_15_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var opcode:u8
        let s_16_0: u8 = fn_state.opcode;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 4u16);
        // C s_16_2: const #6u : u8
        let s_16_2: u8 = 6;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 4u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: not s_16_4
        let s_16_5: bool = !s_16_4;
        // N s_16_6: branch s_16_5 b18 b17
        if s_16_5 {
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
        // C s_17_0: const #3s : i64
        let s_17_0: i64 = 3;
        // D s_17_1: write-var rpt <= s_17_0
        fn_state.rpt = s_17_0;
        // C s_17_2: const #1s : i64
        let s_17_2: i64 = 1;
        // D s_17_3: write-var selem <= s_17_2
        fn_state.selem = s_17_2;
        // N s_17_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var opcode:u8
        let s_18_0: u8 = fn_state.opcode;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 4u16);
        // C s_18_2: const #7u : u8
        let s_18_2: u8 = 7;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 4u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: not s_18_4
        let s_18_5: bool = !s_18_4;
        // N s_18_6: branch s_18_5 b20 b19
        if s_18_5 {
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
        // C s_19_0: const #1s : i64
        let s_19_0: i64 = 1;
        // D s_19_1: write-var rpt <= s_19_0
        fn_state.rpt = s_19_0;
        // C s_19_2: const #1s : i64
        let s_19_2: i64 = 1;
        // D s_19_3: write-var selem <= s_19_2
        fn_state.selem = s_19_2;
        // N s_19_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var opcode:u8
        let s_20_0: u8 = fn_state.opcode;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 4u16);
        // C s_20_2: const #8u : u8
        let s_20_2: u8 = 8;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 4u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: not s_20_4
        let s_20_5: bool = !s_20_4;
        // N s_20_6: branch s_20_5 b22 b21
        if s_20_5 {
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
        // C s_21_0: const #1s : i64
        let s_21_0: i64 = 1;
        // D s_21_1: write-var rpt <= s_21_0
        fn_state.rpt = s_21_0;
        // C s_21_2: const #2s : i64
        let s_21_2: i64 = 2;
        // D s_21_3: write-var selem <= s_21_2
        fn_state.selem = s_21_2;
        // N s_21_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var opcode:u8
        let s_22_0: u8 = fn_state.opcode;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 4u16);
        // C s_22_2: const #10u : u8
        let s_22_2: u8 = 10;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 4u16);
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
        // C s_23_0: const #2s : i64
        let s_23_0: i64 = 2;
        // D s_23_1: write-var rpt <= s_23_0
        fn_state.rpt = s_23_0;
        // C s_23_2: const #1s : i64
        let s_23_2: i64 = 1;
        // D s_23_3: write-var selem <= s_23_2
        fn_state.selem = s_23_2;
        // N s_23_4: jump b6
        return block_6(state, tracer, fn_state);
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
        // C s_25_0: const #128s : i64
        let s_25_0: i64 = 128;
        // D s_25_1: write-var ga#258296 <= s_25_0
        fn_state.ga_258296 = s_25_0;
        // N s_25_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u32
        let s_26_0: u32 = 0;
        // D s_26_1: write-var memop <= s_26_0
        fn_state.memop = s_26_0;
        // N s_26_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
