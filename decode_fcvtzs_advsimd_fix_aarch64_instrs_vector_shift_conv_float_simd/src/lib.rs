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
use execute_aarch64_instrs_vector_shift_conv_float_sisd::*;
use common::*;
pub fn decode_fcvtzs_advsimd_fix_aarch64_instrs_vector_shift_conv_float_simd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    immb: u8,
    immh: u8,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_254912: i64,
        esize: i64,
        n: i64,
        gs_152125: bool,
        ga_254914: i64,
        gs_152120: bool,
        d: i64,
        gs_152131: bool,
        gs_152136: bool,
        Rd: u8,
        Rn: u8,
        immb: u8,
        immh: u8,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        immb,
        immh,
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
        // D s_0_10: read-var immh:u8
        let s_0_10: u8 = fn_state.immh;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 4u16);
        // C s_0_12: const #0u : u8
        let s_0_12: u8 = 0;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 4u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b26 b1
        if s_0_14 {
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
        // D s_1_0: read-var immh:u8
        let s_1_0: u8 = fn_state.immh;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: cast zx s_1_0 -> bv
        let s_1_2: Bits = Bits::new(s_1_0 as u128, 4u16);
        // C s_1_3: const #1s : i64
        let s_1_3: i64 = 1;
        // C s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // C s_1_5: const #2s : i
        let s_1_5: i128 = 2;
        // C s_1_6: add s_1_5 s_1_4
        let s_1_6: i128 = (s_1_5 + s_1_4);
        // D s_1_7: bit-extract s_1_2 s_1_1 s_1_6
        let s_1_7: Bits = (Bits::new(
            ((s_1_2) >> (s_1_1)).value(),
            u16::try_from(s_1_6).unwrap(),
        ));
        // D s_1_8: cast reint s_1_7 -> u8
        let s_1_8: u8 = (s_1_7.value() as u8);
        // D s_1_9: cast zx s_1_8 -> bv
        let s_1_9: Bits = Bits::new(s_1_8 as u128, 3u16);
        // C s_1_10: const #0u : u8
        let s_1_10: u8 = 0;
        // C s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 3u16);
        // D s_1_12: cmp-eq s_1_9 s_1_11
        let s_1_12: bool = ((s_1_9) == (s_1_11));
        // D s_1_13: not s_1_12
        let s_1_13: bool = !s_1_12;
        // N s_1_14: branch s_1_13 b25 b2
        if s_1_13 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #1u : u8
        let s_2_0: bool = true;
        // D s_2_1: write-var gs#152120 <= s_2_0
        fn_state.gs_152120 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#152120:u8
        let s_3_0: bool = fn_state.gs_152120;
        // N s_3_1: branch s_3_0 b24 b4
        if s_3_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#152125 <= s_4_0
        fn_state.gs_152125 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#152125:u8
        let s_5_0: bool = fn_state.gs_152125;
        // N s_5_1: branch s_5_0 b23 b6
        if s_5_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #3s : i
        let s_6_0: i128 = 3;
        // D s_6_1: read-var immh:u8
        let s_6_1: u8 = fn_state.immh;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 4u16);
        // C s_6_3: const #1u : u64
        let s_6_3: u64 = 1;
        // D s_6_4: bit-extract s_6_2 s_6_0 s_6_3
        let s_6_4: Bits = (Bits::new(
            ((s_6_2) >> (s_6_0)).value(),
            u16::try_from(s_6_3).unwrap(),
        ));
        // D s_6_5: cast reint s_6_4 -> u8
        let s_6_5: bool = ((s_6_4.value()) != 0);
        // C s_6_6: const #0s : i
        let s_6_6: i128 = 0;
        // C s_6_7: const #0u : u64
        let s_6_7: u64 = 0;
        // D s_6_8: cast zx s_6_5 -> u64
        let s_6_8: u64 = (s_6_5 as u64);
        // C s_6_9: const #1u : u64
        let s_6_9: u64 = 1;
        // D s_6_10: and s_6_8 s_6_9
        let s_6_10: u64 = ((s_6_8) & (s_6_9));
        // D s_6_11: cmp-eq s_6_10 s_6_9
        let s_6_11: bool = ((s_6_10) == (s_6_9));
        // D s_6_12: lsl s_6_8 s_6_6
        let s_6_12: u64 = s_6_8 << s_6_6;
        // D s_6_13: or s_6_7 s_6_12
        let s_6_13: u64 = ((s_6_7) | (s_6_12));
        // D s_6_14: cmpl s_6_12
        let s_6_14: u64 = !s_6_12;
        // D s_6_15: and s_6_7 s_6_14
        let s_6_15: u64 = ((s_6_7) & (s_6_14));
        // D s_6_16: select s_6_11 s_6_13 s_6_15
        let s_6_16: u64 = if s_6_11 { s_6_13 } else { s_6_15 };
        // D s_6_17: cast trunc s_6_16 -> u8
        let s_6_17: bool = ((s_6_16) != 0);
        // D s_6_18: cast zx s_6_17 -> bv
        let s_6_18: Bits = Bits::new(s_6_17 as u128, 1u16);
        // D s_6_19: read-var Q:u8
        let s_6_19: bool = fn_state.Q;
        // D s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 1u16);
        // D s_6_21: cast reint s_6_18 -> u128
        let s_6_21: u128 = (s_6_18.value() as u128);
        // D s_6_22: size-of s_6_18
        let s_6_22: u16 = s_6_18.length();
        // D s_6_23: cast reint s_6_20 -> u128
        let s_6_23: u128 = (s_6_20.value() as u128);
        // D s_6_24: size-of s_6_20
        let s_6_24: u16 = s_6_20.length();
        // D s_6_25: lsl s_6_21 s_6_24
        let s_6_25: u128 = s_6_21 << s_6_24;
        // D s_6_26: or s_6_25 s_6_23
        let s_6_26: u128 = ((s_6_25) | (s_6_23));
        // D s_6_27: add s_6_22 s_6_24
        let s_6_27: u16 = (s_6_22 + s_6_24);
        // D s_6_28: create-bits s_6_26 s_6_27
        let s_6_28: Bits = Bits::new(s_6_26, s_6_27);
        // D s_6_29: cast reint s_6_28 -> u8
        let s_6_29: u8 = (s_6_28.value() as u8);
        // D s_6_30: cast zx s_6_29 -> bv
        let s_6_30: Bits = Bits::new(s_6_29 as u128, 2u16);
        // C s_6_31: const #2u : u8
        let s_6_31: u8 = 2;
        // C s_6_32: cast zx s_6_31 -> bv
        let s_6_32: Bits = Bits::new(s_6_31 as u128, 2u16);
        // D s_6_33: cmp-eq s_6_30 s_6_32
        let s_6_33: bool = ((s_6_30) == (s_6_32));
        // N s_6_34: branch s_6_33 b22 b7
        if s_6_33 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var immh:u8
        let s_7_0: u8 = fn_state.immh;
        // C s_7_1: const #3s : i
        let s_7_1: i128 = 3;
        // D s_7_2: cast zx s_7_0 -> bv
        let s_7_2: Bits = Bits::new(s_7_0 as u128, 4u16);
        // C s_7_3: const #1s : i64
        let s_7_3: i64 = 1;
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #0s : i
        let s_7_5: i128 = 0;
        // C s_7_6: add s_7_5 s_7_4
        let s_7_6: i128 = (s_7_5 + s_7_4);
        // D s_7_7: bit-extract s_7_2 s_7_1 s_7_6
        let s_7_7: Bits = (Bits::new(
            ((s_7_2) >> (s_7_1)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: bool = ((s_7_7.value()) != 0);
        // D s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 1u16);
        // C s_7_10: const #1u : u8
        let s_7_10: bool = true;
        // C s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 1u16);
        // D s_7_12: cmp-eq s_7_9 s_7_11
        let s_7_12: bool = ((s_7_9) == (s_7_11));
        // D s_7_13: not s_7_12
        let s_7_13: bool = !s_7_12;
        // N s_7_14: branch s_7_13 b21 b8
        if s_7_13 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#152131 <= s_8_0
        fn_state.gs_152131 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#152131:u8
        let s_9_0: bool = fn_state.gs_152131;
        // N s_9_1: branch s_9_0 b20 b10
        if s_9_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var immh:u8
        let s_10_0: u8 = fn_state.immh;
        // C s_10_1: const #2s : i
        let s_10_1: i128 = 2;
        // D s_10_2: cast zx s_10_0 -> bv
        let s_10_2: Bits = Bits::new(s_10_0 as u128, 4u16);
        // C s_10_3: const #1s : i64
        let s_10_3: i64 = 1;
        // C s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // C s_10_5: const #1s : i
        let s_10_5: i128 = 1;
        // C s_10_6: add s_10_5 s_10_4
        let s_10_6: i128 = (s_10_5 + s_10_4);
        // D s_10_7: bit-extract s_10_2 s_10_1 s_10_6
        let s_10_7: Bits = (Bits::new(
            ((s_10_2) >> (s_10_1)).value(),
            u16::try_from(s_10_6).unwrap(),
        ));
        // D s_10_8: cast reint s_10_7 -> u8
        let s_10_8: u8 = (s_10_7.value() as u8);
        // D s_10_9: cast zx s_10_8 -> bv
        let s_10_9: Bits = Bits::new(s_10_8 as u128, 2u16);
        // C s_10_10: const #1u : u8
        let s_10_10: u8 = 1;
        // C s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 2u16);
        // D s_10_12: cmp-eq s_10_9 s_10_11
        let s_10_12: bool = ((s_10_9) == (s_10_11));
        // D s_10_13: not s_10_12
        let s_10_13: bool = !s_10_12;
        // N s_10_14: branch s_10_13 b19 b11
        if s_10_13 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#152136 <= s_11_0
        fn_state.gs_152136 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#152136:u8
        let s_12_0: bool = fn_state.gs_152136;
        // N s_12_1: branch s_12_0 b18 b13
        if s_12_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #16s : i64
        let s_13_0: i64 = 16;
        // D s_13_1: write-var ga#254912 <= s_13_0
        fn_state.ga_254912 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var ga#254912:i64
        let s_14_0: i64 = fn_state.ga_254912;
        // D s_14_1: write-var esize <= s_14_0
        fn_state.esize = s_14_0;
        // D s_14_2: read-var Q:u8
        let s_14_2: bool = fn_state.Q;
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #1u : u8
        let s_14_4: bool = true;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // D s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // N s_14_7: branch s_14_6 b17 b15
        if s_14_6 {
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
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // D s_15_1: write-var ga#254914 <= s_15_0
        fn_state.ga_254914 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var ga#254914:i64
        let s_16_0: i64 = fn_state.ga_254914;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: read-var esize:i64
        let s_16_2: i64 = fn_state.esize;
        // D s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_4: div s_16_1 s_16_3
        let s_16_4: i128 = ((s_16_1) / (s_16_3));
        // D s_16_5: cast reint s_16_4 -> i64
        let s_16_5: i64 = (s_16_4 as i64);
        // C s_16_6: const #2s : i
        let s_16_6: i128 = 2;
        // D s_16_7: read-var esize:i64
        let s_16_7: i64 = fn_state.esize;
        // D s_16_8: cast zx s_16_7 -> i
        let s_16_8: i128 = (i128::try_from(s_16_7).unwrap());
        // D s_16_9: mul s_16_8 s_16_6
        let s_16_9: i128 = ((s_16_8) * (s_16_6));
        // D s_16_10: cast reint s_16_9 -> i64
        let s_16_10: i64 = (s_16_9 as i64);
        // D s_16_11: read-var immh:u8
        let s_16_11: u8 = fn_state.immh;
        // D s_16_12: cast zx s_16_11 -> bv
        let s_16_12: Bits = Bits::new(s_16_11 as u128, 4u16);
        // D s_16_13: read-var immb:u8
        let s_16_13: u8 = fn_state.immb;
        // D s_16_14: cast zx s_16_13 -> bv
        let s_16_14: Bits = Bits::new(s_16_13 as u128, 3u16);
        // D s_16_15: cast reint s_16_12 -> u128
        let s_16_15: u128 = (s_16_12.value() as u128);
        // D s_16_16: size-of s_16_12
        let s_16_16: u16 = s_16_12.length();
        // D s_16_17: cast reint s_16_14 -> u128
        let s_16_17: u128 = (s_16_14.value() as u128);
        // D s_16_18: size-of s_16_14
        let s_16_18: u16 = s_16_14.length();
        // D s_16_19: lsl s_16_15 s_16_18
        let s_16_19: u128 = s_16_15 << s_16_18;
        // D s_16_20: or s_16_19 s_16_17
        let s_16_20: u128 = ((s_16_19) | (s_16_17));
        // D s_16_21: add s_16_16 s_16_18
        let s_16_21: u16 = (s_16_16 + s_16_18);
        // D s_16_22: create-bits s_16_20 s_16_21
        let s_16_22: Bits = Bits::new(s_16_20, s_16_21);
        // D s_16_23: cast reint s_16_22 -> u8
        let s_16_23: u8 = (s_16_22.value() as u8);
        // D s_16_24: cast zx s_16_23 -> bv
        let s_16_24: Bits = Bits::new(s_16_23 as u128, 7u16);
        // D s_16_25: cast zx s_16_24 -> i
        let s_16_25: i128 = (s_16_24.value() as i128);
        // D s_16_26: cast reint s_16_25 -> i64
        let s_16_26: i64 = (s_16_25 as i64);
        // D s_16_27: cast zx s_16_10 -> i
        let s_16_27: i128 = (i128::try_from(s_16_10).unwrap());
        // D s_16_28: cast zx s_16_26 -> i
        let s_16_28: i128 = (i128::try_from(s_16_26).unwrap());
        // D s_16_29: sub s_16_27 s_16_28
        let s_16_29: i128 = ((s_16_27) - (s_16_28));
        // D s_16_30: cast reint s_16_29 -> i64
        let s_16_30: i64 = (s_16_29 as i64);
        // D s_16_31: read-var U:u8
        let s_16_31: bool = fn_state.U;
        // D s_16_32: cast zx s_16_31 -> bv
        let s_16_32: Bits = Bits::new(s_16_31 as u128, 1u16);
        // C s_16_33: const #1u : u8
        let s_16_33: bool = true;
        // C s_16_34: cast zx s_16_33 -> bv
        let s_16_34: Bits = Bits::new(s_16_33 as u128, 1u16);
        // D s_16_35: cmp-eq s_16_32 s_16_34
        let s_16_35: bool = ((s_16_32) == (s_16_34));
        // D s_16_36: cast zx s_16_0 -> i
        let s_16_36: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_37: cast reint s_16_36 -> i64
        let s_16_37: i64 = (s_16_36 as i64);
        // D s_16_38: read-var esize:i64
        let s_16_38: i64 = fn_state.esize;
        // D s_16_39: cast zx s_16_38 -> i
        let s_16_39: i128 = (i128::try_from(s_16_38).unwrap());
        // D s_16_40: cast reint s_16_39 -> i64
        let s_16_40: i64 = (s_16_39 as i64);
        // D s_16_41: cast zx s_16_5 -> i
        let s_16_41: i128 = (i128::try_from(s_16_5).unwrap());
        // D s_16_42: cast zx s_16_30 -> i
        let s_16_42: i128 = (i128::try_from(s_16_30).unwrap());
        // D s_16_43: read-var d:i64
        let s_16_43: i64 = fn_state.d;
        // D s_16_44: read-var n:i64
        let s_16_44: i64 = fn_state.n;
        // C s_16_45: const #3u : u32
        let s_16_45: u32 = 3;
        // D s_16_46: call execute_aarch64_instrs_vector_shift_conv_float_sisd(s_16_43, s_16_37, s_16_41, s_16_40, s_16_42, s_16_44, s_16_45, s_16_35)
        let s_16_46: () = execute_aarch64_instrs_vector_shift_conv_float_sisd(
            state,
            tracer,
            s_16_43,
            s_16_37,
            s_16_41,
            s_16_40,
            s_16_42,
            s_16_44,
            s_16_45,
            s_16_35,
        );
        // N s_16_47: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #128s : i64
        let s_17_0: i64 = 128;
        // D s_17_1: write-var ga#254914 <= s_17_0
        fn_state.ga_254914 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #32s : i64
        let s_18_0: i64 = 32;
        // D s_18_1: write-var ga#254912 <= s_18_0
        fn_state.ga_254912 = s_18_0;
        // N s_18_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#152136 <= s_19_0
        fn_state.gs_152136 = s_19_0;
        // N s_19_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #64s : i64
        let s_20_0: i64 = 64;
        // D s_20_1: write-var ga#254912 <= s_20_0
        fn_state.ga_254912 = s_20_0;
        // N s_20_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#152131 <= s_21_0
        fn_state.gs_152131 = s_21_0;
        // N s_21_2: jump b9
        return block_9(state, tracer, fn_state);
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
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#152125 <= s_24_0
        fn_state.gs_152125 = s_24_0;
        // N s_24_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#152120 <= s_25_0
        fn_state.gs_152120 = s_25_0;
        // N s_25_2: jump b3
        return block_3(state, tracer, fn_state);
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
}
