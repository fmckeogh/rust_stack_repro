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
use HaveFP16Ext::*;
use execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp16_sisd::*;
use common::*;
pub fn decode_facgt_advsimd_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp16_sisd<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    ac: bool,
    Rm: u8,
    E: bool,
    U: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        cmp: u32,
        abs: bool,
        n: i64,
        ga_252812: u8,
        d: i64,
        Rd: u8,
        Rn: u8,
        ac: bool,
        Rm: u8,
        E: bool,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        ac,
        Rm,
        E,
        U,
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
        // S s_0_1: call HaveFP16Ext(s_0_0)
        let s_0_1: bool = HaveFP16Ext(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b13 b1
        if s_0_2 {
            return block_13(state, tracer, fn_state);
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
        // D s_1_10: read-var Rm:u8
        let s_1_10: u8 = fn_state.Rm;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 5u16);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (s_1_11.value() as i128);
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var m <= s_1_13
        fn_state.m = s_1_13;
        // D s_1_15: read-var E:u8
        let s_1_15: bool = fn_state.E;
        // D s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 1u16);
        // D s_1_17: read-var U:u8
        let s_1_17: bool = fn_state.U;
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // D s_1_19: cast reint s_1_16 -> u128
        let s_1_19: u128 = (s_1_16.value() as u128);
        // D s_1_20: size-of s_1_16
        let s_1_20: u16 = s_1_16.length();
        // D s_1_21: cast reint s_1_18 -> u128
        let s_1_21: u128 = (s_1_18.value() as u128);
        // D s_1_22: size-of s_1_18
        let s_1_22: u16 = s_1_18.length();
        // D s_1_23: lsl s_1_19 s_1_22
        let s_1_23: u128 = s_1_19 << s_1_22;
        // D s_1_24: or s_1_23 s_1_21
        let s_1_24: u128 = ((s_1_23) | (s_1_21));
        // D s_1_25: add s_1_20 s_1_22
        let s_1_25: u16 = (s_1_20 + s_1_22);
        // D s_1_26: create-bits s_1_24 s_1_25
        let s_1_26: Bits = Bits::new(s_1_24, s_1_25);
        // D s_1_27: cast reint s_1_26 -> u8
        let s_1_27: u8 = (s_1_26.value() as u8);
        // D s_1_28: cast zx s_1_27 -> bv
        let s_1_28: Bits = Bits::new(s_1_27 as u128, 2u16);
        // D s_1_29: read-var ac:u8
        let s_1_29: bool = fn_state.ac;
        // D s_1_30: cast zx s_1_29 -> bv
        let s_1_30: Bits = Bits::new(s_1_29 as u128, 1u16);
        // D s_1_31: cast reint s_1_28 -> u128
        let s_1_31: u128 = (s_1_28.value() as u128);
        // D s_1_32: size-of s_1_28
        let s_1_32: u16 = s_1_28.length();
        // D s_1_33: cast reint s_1_30 -> u128
        let s_1_33: u128 = (s_1_30.value() as u128);
        // D s_1_34: size-of s_1_30
        let s_1_34: u16 = s_1_30.length();
        // D s_1_35: lsl s_1_31 s_1_34
        let s_1_35: u128 = s_1_31 << s_1_34;
        // D s_1_36: or s_1_35 s_1_33
        let s_1_36: u128 = ((s_1_35) | (s_1_33));
        // D s_1_37: add s_1_32 s_1_34
        let s_1_37: u16 = (s_1_32 + s_1_34);
        // D s_1_38: create-bits s_1_36 s_1_37
        let s_1_38: Bits = Bits::new(s_1_36, s_1_37);
        // D s_1_39: cast reint s_1_38 -> u8
        let s_1_39: u8 = (s_1_38.value() as u8);
        // D s_1_40: write-var ga#252812 <= s_1_39
        fn_state.ga_252812 = s_1_39;
        // D s_1_41: read-var ga#252812:u8
        let s_1_41: u8 = fn_state.ga_252812;
        // D s_1_42: cast zx s_1_41 -> bv
        let s_1_42: Bits = Bits::new(s_1_41 as u128, 3u16);
        // C s_1_43: const #0u : u8
        let s_1_43: u8 = 0;
        // C s_1_44: cast zx s_1_43 -> bv
        let s_1_44: Bits = Bits::new(s_1_43 as u128, 3u16);
        // D s_1_45: cmp-eq s_1_42 s_1_44
        let s_1_45: bool = ((s_1_42) == (s_1_44));
        // D s_1_46: not s_1_45
        let s_1_46: bool = !s_1_45;
        // N s_1_47: branch s_1_46 b4 b2
        if s_1_46 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #2u : u32
        let s_2_0: u32 = 2;
        // D s_2_1: write-var cmp <= s_2_0
        fn_state.cmp = s_2_0;
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // D s_2_3: write-var abs <= s_2_2
        fn_state.abs = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var abs:u8
        let s_3_0: bool = fn_state.abs;
        // D s_3_1: read-var cmp:u32
        let s_3_1: u32 = fn_state.cmp;
        // C s_3_2: const #16s : i64
        let s_3_2: i64 = 16;
        // C s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // C s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // C s_3_5: const #16s : i64
        let s_3_5: i64 = 16;
        // C s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // C s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // C s_3_8: const #1s : i64
        let s_3_8: i64 = 1;
        // C s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: read-var d:i64
        let s_3_10: i64 = fn_state.d;
        // D s_3_11: read-var m:i64
        let s_3_11: i64 = fn_state.m;
        // D s_3_12: read-var n:i64
        let s_3_12: i64 = fn_state.n;
        // D s_3_13: call execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp16_sisd(s_3_0, s_3_1, s_3_10, s_3_4, s_3_9, s_3_7, s_3_11, s_3_12)
        let s_3_13: () = execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp16_sisd(
            state,
            tracer,
            s_3_0,
            s_3_1,
            s_3_10,
            s_3_4,
            s_3_9,
            s_3_7,
            s_3_11,
            s_3_12,
        );
        // N s_3_14: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#252812:u8
        let s_4_0: u8 = fn_state.ga_252812;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 3u16);
        // C s_4_2: const #2u : u8
        let s_4_2: u8 = 2;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 3u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u32
        let s_5_0: u32 = 1;
        // D s_5_1: write-var cmp <= s_5_0
        fn_state.cmp = s_5_0;
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // D s_5_3: write-var abs <= s_5_2
        fn_state.abs = s_5_2;
        // N s_5_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#252812:u8
        let s_6_0: u8 = fn_state.ga_252812;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 3u16);
        // C s_6_2: const #3u : u8
        let s_6_2: u8 = 3;
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
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // D s_7_3: write-var abs <= s_7_2
        fn_state.abs = s_7_2;
        // N s_7_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#252812:u8
        let s_8_0: u8 = fn_state.ga_252812;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 3u16);
        // C s_8_2: const #6u : u8
        let s_8_2: u8 = 6;
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
        // C s_9_0: const #0u : u32
        let s_9_0: u32 = 0;
        // D s_9_1: write-var cmp <= s_9_0
        fn_state.cmp = s_9_0;
        // C s_9_2: const #0u : u8
        let s_9_2: bool = false;
        // D s_9_3: write-var abs <= s_9_2
        fn_state.abs = s_9_2;
        // N s_9_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#252812:u8
        let s_10_0: u8 = fn_state.ga_252812;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 3u16);
        // C s_10_2: const #7u : u8
        let s_10_2: u8 = 7;
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
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // D s_11_3: write-var abs <= s_11_2
        fn_state.abs = s_11_2;
        // N s_11_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
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
}
