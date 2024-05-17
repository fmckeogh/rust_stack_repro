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
pub fn decode_fcvtzu_advsimd_fix_aarch64_instrs_vector_shift_conv_float_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    immb: u8,
    immh: u8,
    U: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        gs_152164: bool,
        d: i64,
        gs_152171: bool,
        gs_152166: bool,
        gs_152159: bool,
        ga_254932: i64,
        Rd: u8,
        Rn: u8,
        immb: u8,
        immh: u8,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        immb,
        immh,
        U,
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
        // C s_0_11: const #1s : i
        let s_0_11: i128 = 1;
        // D s_0_12: cast zx s_0_10 -> bv
        let s_0_12: Bits = Bits::new(s_0_10 as u128, 4u16);
        // C s_0_13: const #1s : i64
        let s_0_13: i64 = 1;
        // C s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // C s_0_15: const #2s : i
        let s_0_15: i128 = 2;
        // C s_0_16: add s_0_15 s_0_14
        let s_0_16: i128 = (s_0_15 + s_0_14);
        // D s_0_17: bit-extract s_0_12 s_0_11 s_0_16
        let s_0_17: Bits = (Bits::new(
            ((s_0_12) >> (s_0_11)).value(),
            u16::try_from(s_0_16).unwrap(),
        ));
        // D s_0_18: cast reint s_0_17 -> u8
        let s_0_18: u8 = (s_0_17.value() as u8);
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 3u16);
        // C s_0_20: const #0u : u8
        let s_0_20: u8 = 0;
        // C s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 3u16);
        // D s_0_22: cmp-eq s_0_19 s_0_21
        let s_0_22: bool = ((s_0_19) == (s_0_21));
        // D s_0_23: not s_0_22
        let s_0_23: bool = !s_0_22;
        // N s_0_24: branch s_0_23 b19 b1
        if s_0_23 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1u : u8
        let s_1_0: bool = true;
        // D s_1_1: write-var gs#152159 <= s_1_0
        fn_state.gs_152159 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#152159:u8
        let s_2_0: bool = fn_state.gs_152159;
        // N s_2_1: branch s_2_0 b18 b3
        if s_2_0 {
            return block_18(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#152164 <= s_3_0
        fn_state.gs_152164 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#152164:u8
        let s_4_0: bool = fn_state.gs_152164;
        // N s_4_1: branch s_4_0 b17 b5
        if s_4_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var immh:u8
        let s_5_0: u8 = fn_state.immh;
        // C s_5_1: const #3s : i
        let s_5_1: i128 = 3;
        // D s_5_2: cast zx s_5_0 -> bv
        let s_5_2: Bits = Bits::new(s_5_0 as u128, 4u16);
        // C s_5_3: const #1s : i64
        let s_5_3: i64 = 1;
        // C s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: const #0s : i
        let s_5_5: i128 = 0;
        // C s_5_6: add s_5_5 s_5_4
        let s_5_6: i128 = (s_5_5 + s_5_4);
        // D s_5_7: bit-extract s_5_2 s_5_1 s_5_6
        let s_5_7: Bits = (Bits::new(
            ((s_5_2) >> (s_5_1)).value(),
            u16::try_from(s_5_6).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: bool = ((s_5_7.value()) != 0);
        // D s_5_9: cast zx s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 1u16);
        // C s_5_10: const #1u : u8
        let s_5_10: bool = true;
        // C s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 1u16);
        // D s_5_12: cmp-eq s_5_9 s_5_11
        let s_5_12: bool = ((s_5_9) == (s_5_11));
        // D s_5_13: not s_5_12
        let s_5_13: bool = !s_5_12;
        // N s_5_14: branch s_5_13 b16 b6
        if s_5_13 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var gs#152166 <= s_6_0
        fn_state.gs_152166 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#152166:u8
        let s_7_0: bool = fn_state.gs_152166;
        // N s_7_1: branch s_7_0 b15 b8
        if s_7_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var immh:u8
        let s_8_0: u8 = fn_state.immh;
        // C s_8_1: const #2s : i
        let s_8_1: i128 = 2;
        // D s_8_2: cast zx s_8_0 -> bv
        let s_8_2: Bits = Bits::new(s_8_0 as u128, 4u16);
        // C s_8_3: const #1s : i64
        let s_8_3: i64 = 1;
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #1s : i
        let s_8_5: i128 = 1;
        // C s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: bit-extract s_8_2 s_8_1 s_8_6
        let s_8_7: Bits = (Bits::new(
            ((s_8_2) >> (s_8_1)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u8
        let s_8_8: u8 = (s_8_7.value() as u8);
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 2u16);
        // C s_8_10: const #1u : u8
        let s_8_10: u8 = 1;
        // C s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 2u16);
        // D s_8_12: cmp-eq s_8_9 s_8_11
        let s_8_12: bool = ((s_8_9) == (s_8_11));
        // D s_8_13: not s_8_12
        let s_8_13: bool = !s_8_12;
        // N s_8_14: branch s_8_13 b14 b9
        if s_8_13 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#152171 <= s_9_0
        fn_state.gs_152171 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#152171:u8
        let s_10_0: bool = fn_state.gs_152171;
        // N s_10_1: branch s_10_0 b13 b11
        if s_10_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #16s : i64
        let s_11_0: i64 = 16;
        // D s_11_1: write-var ga#254932 <= s_11_0
        fn_state.ga_254932 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#254932:i64
        let s_12_0: i64 = fn_state.ga_254932;
        // C s_12_1: const #2s : i
        let s_12_1: i128 = 2;
        // D s_12_2: cast zx s_12_0 -> i
        let s_12_2: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_3: mul s_12_2 s_12_1
        let s_12_3: i128 = ((s_12_2) * (s_12_1));
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // D s_12_5: read-var immh:u8
        let s_12_5: u8 = fn_state.immh;
        // D s_12_6: cast zx s_12_5 -> bv
        let s_12_6: Bits = Bits::new(s_12_5 as u128, 4u16);
        // D s_12_7: read-var immb:u8
        let s_12_7: u8 = fn_state.immb;
        // D s_12_8: cast zx s_12_7 -> bv
        let s_12_8: Bits = Bits::new(s_12_7 as u128, 3u16);
        // D s_12_9: cast reint s_12_6 -> u128
        let s_12_9: u128 = (s_12_6.value() as u128);
        // D s_12_10: size-of s_12_6
        let s_12_10: u16 = s_12_6.length();
        // D s_12_11: cast reint s_12_8 -> u128
        let s_12_11: u128 = (s_12_8.value() as u128);
        // D s_12_12: size-of s_12_8
        let s_12_12: u16 = s_12_8.length();
        // D s_12_13: lsl s_12_9 s_12_12
        let s_12_13: u128 = s_12_9 << s_12_12;
        // D s_12_14: or s_12_13 s_12_11
        let s_12_14: u128 = ((s_12_13) | (s_12_11));
        // D s_12_15: add s_12_10 s_12_12
        let s_12_15: u16 = (s_12_10 + s_12_12);
        // D s_12_16: create-bits s_12_14 s_12_15
        let s_12_16: Bits = Bits::new(s_12_14, s_12_15);
        // D s_12_17: cast reint s_12_16 -> u8
        let s_12_17: u8 = (s_12_16.value() as u8);
        // D s_12_18: cast zx s_12_17 -> bv
        let s_12_18: Bits = Bits::new(s_12_17 as u128, 7u16);
        // D s_12_19: cast zx s_12_18 -> i
        let s_12_19: i128 = (s_12_18.value() as i128);
        // D s_12_20: cast reint s_12_19 -> i64
        let s_12_20: i64 = (s_12_19 as i64);
        // D s_12_21: cast zx s_12_4 -> i
        let s_12_21: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_22: cast zx s_12_20 -> i
        let s_12_22: i128 = (i128::try_from(s_12_20).unwrap());
        // D s_12_23: sub s_12_21 s_12_22
        let s_12_23: i128 = ((s_12_21) - (s_12_22));
        // D s_12_24: cast reint s_12_23 -> i64
        let s_12_24: i64 = (s_12_23 as i64);
        // D s_12_25: read-var U:u8
        let s_12_25: bool = fn_state.U;
        // D s_12_26: cast zx s_12_25 -> bv
        let s_12_26: Bits = Bits::new(s_12_25 as u128, 1u16);
        // C s_12_27: const #1u : u8
        let s_12_27: bool = true;
        // C s_12_28: cast zx s_12_27 -> bv
        let s_12_28: Bits = Bits::new(s_12_27 as u128, 1u16);
        // D s_12_29: cmp-eq s_12_26 s_12_28
        let s_12_29: bool = ((s_12_26) == (s_12_28));
        // D s_12_30: cast zx s_12_0 -> i
        let s_12_30: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_31: cast reint s_12_30 -> i64
        let s_12_31: i64 = (s_12_30 as i64);
        // D s_12_32: cast zx s_12_0 -> i
        let s_12_32: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_33: cast reint s_12_32 -> i64
        let s_12_33: i64 = (s_12_32 as i64);
        // C s_12_34: const #1s : i
        let s_12_34: i128 = 1;
        // D s_12_35: cast zx s_12_24 -> i
        let s_12_35: i128 = (i128::try_from(s_12_24).unwrap());
        // D s_12_36: read-var d:i64
        let s_12_36: i64 = fn_state.d;
        // D s_12_37: read-var n:i64
        let s_12_37: i64 = fn_state.n;
        // C s_12_38: const #3u : u32
        let s_12_38: u32 = 3;
        // D s_12_39: call execute_aarch64_instrs_vector_shift_conv_float_sisd(s_12_36, s_12_31, s_12_34, s_12_33, s_12_35, s_12_37, s_12_38, s_12_29)
        let s_12_39: () = execute_aarch64_instrs_vector_shift_conv_float_sisd(
            state,
            tracer,
            s_12_36,
            s_12_31,
            s_12_34,
            s_12_33,
            s_12_35,
            s_12_37,
            s_12_38,
            s_12_29,
        );
        // N s_12_40: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #32s : i64
        let s_13_0: i64 = 32;
        // D s_13_1: write-var ga#254932 <= s_13_0
        fn_state.ga_254932 = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#152171 <= s_14_0
        fn_state.gs_152171 = s_14_0;
        // N s_14_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // D s_15_1: write-var ga#254932 <= s_15_0
        fn_state.ga_254932 = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#152166 <= s_16_0
        fn_state.gs_152166 = s_16_0;
        // N s_16_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#152164 <= s_18_0
        fn_state.gs_152164 = s_18_0;
        // N s_18_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#152159 <= s_19_0
        fn_state.gs_152159 = s_19_0;
        // N s_19_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
