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
use execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp16_sisd::*;
use common::*;
pub fn decode_facgt_advsimd_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp_simd<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    ac: bool,
    Rm: u8,
    sz: bool,
    E: bool,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        cmp: u32,
        esize: i64,
        abs: bool,
        n: i64,
        d: i64,
        elements: i64,
        ga_252849: u8,
        datasize: i64,
        ga_252846: i64,
        Rd: u8,
        Rn: u8,
        ac: bool,
        Rm: u8,
        sz: bool,
        E: bool,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        ac,
        Rm,
        sz,
        E,
        U,
        Q,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var d <= s_0_3
        fn_state.d = s_0_3;
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
        // D s_0_10: read-var Rm:u8
        let s_0_10: u8 = fn_state.Rm;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: write-var m <= s_0_13
        fn_state.m = s_0_13;
        // D s_0_15: read-var sz:u8
        let s_0_15: bool = fn_state.sz;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 1u16);
        // D s_0_17: read-var Q:u8
        let s_0_17: bool = fn_state.Q;
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_19: cast reint s_0_16 -> u128
        let s_0_19: u128 = (s_0_16.value() as u128);
        // D s_0_20: size-of s_0_16
        let s_0_20: u16 = s_0_16.length();
        // D s_0_21: cast reint s_0_18 -> u128
        let s_0_21: u128 = (s_0_18.value() as u128);
        // D s_0_22: size-of s_0_18
        let s_0_22: u16 = s_0_18.length();
        // D s_0_23: lsl s_0_19 s_0_22
        let s_0_23: u128 = s_0_19 << s_0_22;
        // D s_0_24: or s_0_23 s_0_21
        let s_0_24: u128 = ((s_0_23) | (s_0_21));
        // D s_0_25: add s_0_20 s_0_22
        let s_0_25: u16 = (s_0_20 + s_0_22);
        // D s_0_26: create-bits s_0_24 s_0_25
        let s_0_26: Bits = Bits::new(s_0_24, s_0_25);
        // D s_0_27: cast reint s_0_26 -> u8
        let s_0_27: u8 = (s_0_26.value() as u8);
        // D s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 2u16);
        // C s_0_29: const #2u : u8
        let s_0_29: u8 = 2;
        // C s_0_30: cast zx s_0_29 -> bv
        let s_0_30: Bits = Bits::new(s_0_29 as u128, 2u16);
        // D s_0_31: cmp-eq s_0_28 s_0_30
        let s_0_31: bool = ((s_0_28) == (s_0_30));
        // N s_0_32: branch s_0_31 b16 b1
        if s_0_31 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var sz:u8
        let s_1_0: bool = fn_state.sz;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #32s : i64
        let s_1_4: i64 = 32;
        // D s_1_5: lsl s_1_4 s_1_3
        let s_1_5: i64 = s_1_4 << s_1_3;
        // D s_1_6: write-var esize <= s_1_5
        fn_state.esize = s_1_5;
        // D s_1_7: read-var Q:u8
        let s_1_7: bool = fn_state.Q;
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 1u16);
        // C s_1_9: const #1u : u8
        let s_1_9: bool = true;
        // C s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 1u16);
        // D s_1_11: cmp-eq s_1_8 s_1_10
        let s_1_11: bool = ((s_1_8) == (s_1_10));
        // N s_1_12: branch s_1_11 b15 b2
        if s_1_11 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: write-var ga#252846 <= s_2_0
        fn_state.ga_252846 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#252846:i64
        let s_3_0: i64 = fn_state.ga_252846;
        // D s_3_1: write-var datasize <= s_3_0
        fn_state.datasize = s_3_0;
        // D s_3_2: read-var datasize:i64
        let s_3_2: i64 = fn_state.datasize;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-var esize:i64
        let s_3_4: i64 = fn_state.esize;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: div s_3_3 s_3_5
        let s_3_6: i128 = ((s_3_3) / (s_3_5));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: write-var elements <= s_3_7
        fn_state.elements = s_3_7;
        // D s_3_9: read-var E:u8
        let s_3_9: bool = fn_state.E;
        // D s_3_10: cast zx s_3_9 -> bv
        let s_3_10: Bits = Bits::new(s_3_9 as u128, 1u16);
        // D s_3_11: read-var U:u8
        let s_3_11: bool = fn_state.U;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 1u16);
        // D s_3_13: cast reint s_3_10 -> u128
        let s_3_13: u128 = (s_3_10.value() as u128);
        // D s_3_14: size-of s_3_10
        let s_3_14: u16 = s_3_10.length();
        // D s_3_15: cast reint s_3_12 -> u128
        let s_3_15: u128 = (s_3_12.value() as u128);
        // D s_3_16: size-of s_3_12
        let s_3_16: u16 = s_3_12.length();
        // D s_3_17: lsl s_3_13 s_3_16
        let s_3_17: u128 = s_3_13 << s_3_16;
        // D s_3_18: or s_3_17 s_3_15
        let s_3_18: u128 = ((s_3_17) | (s_3_15));
        // D s_3_19: add s_3_14 s_3_16
        let s_3_19: u16 = (s_3_14 + s_3_16);
        // D s_3_20: create-bits s_3_18 s_3_19
        let s_3_20: Bits = Bits::new(s_3_18, s_3_19);
        // D s_3_21: cast reint s_3_20 -> u8
        let s_3_21: u8 = (s_3_20.value() as u8);
        // D s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 2u16);
        // D s_3_23: read-var ac:u8
        let s_3_23: bool = fn_state.ac;
        // D s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 1u16);
        // D s_3_25: cast reint s_3_22 -> u128
        let s_3_25: u128 = (s_3_22.value() as u128);
        // D s_3_26: size-of s_3_22
        let s_3_26: u16 = s_3_22.length();
        // D s_3_27: cast reint s_3_24 -> u128
        let s_3_27: u128 = (s_3_24.value() as u128);
        // D s_3_28: size-of s_3_24
        let s_3_28: u16 = s_3_24.length();
        // D s_3_29: lsl s_3_25 s_3_28
        let s_3_29: u128 = s_3_25 << s_3_28;
        // D s_3_30: or s_3_29 s_3_27
        let s_3_30: u128 = ((s_3_29) | (s_3_27));
        // D s_3_31: add s_3_26 s_3_28
        let s_3_31: u16 = (s_3_26 + s_3_28);
        // D s_3_32: create-bits s_3_30 s_3_31
        let s_3_32: Bits = Bits::new(s_3_30, s_3_31);
        // D s_3_33: cast reint s_3_32 -> u8
        let s_3_33: u8 = (s_3_32.value() as u8);
        // D s_3_34: write-var ga#252849 <= s_3_33
        fn_state.ga_252849 = s_3_33;
        // D s_3_35: read-var ga#252849:u8
        let s_3_35: u8 = fn_state.ga_252849;
        // D s_3_36: cast zx s_3_35 -> bv
        let s_3_36: Bits = Bits::new(s_3_35 as u128, 3u16);
        // C s_3_37: const #0u : u8
        let s_3_37: u8 = 0;
        // C s_3_38: cast zx s_3_37 -> bv
        let s_3_38: Bits = Bits::new(s_3_37 as u128, 3u16);
        // D s_3_39: cmp-eq s_3_36 s_3_38
        let s_3_39: bool = ((s_3_36) == (s_3_38));
        // D s_3_40: not s_3_39
        let s_3_40: bool = !s_3_39;
        // N s_3_41: branch s_3_40 b6 b4
        if s_3_40 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2u : u32
        let s_4_0: u32 = 2;
        // D s_4_1: write-var cmp <= s_4_0
        fn_state.cmp = s_4_0;
        // C s_4_2: const #0u : u8
        let s_4_2: bool = false;
        // D s_4_3: write-var abs <= s_4_2
        fn_state.abs = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var abs:u8
        let s_5_0: bool = fn_state.abs;
        // D s_5_1: read-var cmp:u32
        let s_5_1: u32 = fn_state.cmp;
        // D s_5_2: read-var datasize:i64
        let s_5_2: i64 = fn_state.datasize;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: read-var esize:i64
        let s_5_5: i64 = fn_state.esize;
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: cast reint s_5_6 -> i64
        let s_5_7: i64 = (s_5_6 as i64);
        // D s_5_8: read-var elements:i64
        let s_5_8: i64 = fn_state.elements;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: read-var d:i64
        let s_5_10: i64 = fn_state.d;
        // D s_5_11: read-var m:i64
        let s_5_11: i64 = fn_state.m;
        // D s_5_12: read-var n:i64
        let s_5_12: i64 = fn_state.n;
        // D s_5_13: call execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp16_sisd(s_5_0, s_5_1, s_5_10, s_5_4, s_5_9, s_5_7, s_5_11, s_5_12)
        let s_5_13: () = execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp16_sisd(
            state,
            tracer,
            s_5_0,
            s_5_1,
            s_5_10,
            s_5_4,
            s_5_9,
            s_5_7,
            s_5_11,
            s_5_12,
        );
        // N s_5_14: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#252849:u8
        let s_6_0: u8 = fn_state.ga_252849;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 3u16);
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 3u16);
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
        // C s_7_0: const #1u : u32
        let s_7_0: u32 = 1;
        // D s_7_1: write-var cmp <= s_7_0
        fn_state.cmp = s_7_0;
        // C s_7_2: const #0u : u8
        let s_7_2: bool = false;
        // D s_7_3: write-var abs <= s_7_2
        fn_state.abs = s_7_2;
        // N s_7_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#252849:u8
        let s_8_0: u8 = fn_state.ga_252849;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 3u16);
        // C s_8_2: const #3u : u8
        let s_8_2: u8 = 3;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 3u16);
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
        // C s_9_0: const #1u : u32
        let s_9_0: u32 = 1;
        // D s_9_1: write-var cmp <= s_9_0
        fn_state.cmp = s_9_0;
        // C s_9_2: const #1u : u8
        let s_9_2: bool = true;
        // D s_9_3: write-var abs <= s_9_2
        fn_state.abs = s_9_2;
        // N s_9_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#252849:u8
        let s_10_0: u8 = fn_state.ga_252849;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 3u16);
        // C s_10_2: const #6u : u8
        let s_10_2: u8 = 6;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 3u16);
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
        // C s_11_0: const #0u : u32
        let s_11_0: u32 = 0;
        // D s_11_1: write-var cmp <= s_11_0
        fn_state.cmp = s_11_0;
        // C s_11_2: const #0u : u8
        let s_11_2: bool = false;
        // D s_11_3: write-var abs <= s_11_2
        fn_state.abs = s_11_2;
        // N s_11_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#252849:u8
        let s_12_0: u8 = fn_state.ga_252849;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 3u16);
        // C s_12_2: const #7u : u8
        let s_12_2: u8 = 7;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 3u16);
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
        // C s_13_0: const #0u : u32
        let s_13_0: u32 = 0;
        // D s_13_1: write-var cmp <= s_13_0
        fn_state.cmp = s_13_0;
        // C s_13_2: const #1u : u8
        let s_13_2: bool = true;
        // D s_13_3: write-var abs <= s_13_2
        fn_state.abs = s_13_2;
        // N s_13_4: jump b5
        return block_5(state, tracer, fn_state);
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
        // C s_15_0: const #128s : i64
        let s_15_0: i64 = 128;
        // D s_15_1: write-var ga#252846 <= s_15_0
        fn_state.ga_252846 = s_15_0;
        // N s_15_2: jump b3
        return block_3(state, tracer, fn_state);
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
}
