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
use CheckVFPEnabled::*;
use D_set::*;
use FPSCR_read::*;
use FPToFixed::*;
use S_set::*;
use S_read::*;
use D_read::*;
use FixedToFP::*;
use common::*;
pub fn execute_aarch32_instrs_VCVT_xv_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    fp_size: i64,
    frac_bits: i128,
    size: i64,
    to_fixed: bool,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_352574: u32,
        resultshadow_7424: Bits,
        ga_352583: u64,
        ga_352579: u32,
        sizeshadow_7423: i64,
        gs_893761: Bits,
        resultshadow_7425: Bits,
        gs_893740: Bits,
        gs_893752: Bits,
        resultshadow_7426: Bits,
        d: i64,
        fp_size: i64,
        frac_bits: i128,
        size: i64,
        to_fixed: bool,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        fp_size,
        frac_bits,
        size,
        to_fixed,
        is_unsigned,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var size:i64
        let s_0_0: i64 = fn_state.size;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var sizeshadow#7423 <= s_0_2
        fn_state.sizeshadow_7423 = s_0_2;
        // C s_0_4: const #1u : u8
        let s_0_4: bool = true;
        // S s_0_5: call CheckVFPEnabled(s_0_4)
        let s_0_5: () = CheckVFPEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var to_fixed:u8
        let s_1_0: bool = fn_state.to_fixed;
        // N s_1_1: branch s_1_0 b12 b2
        if s_1_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var fp_size:i64
        let s_2_0: i64 = fn_state.fp_size;
        // C s_2_1: const #16s : i
        let s_2_1: i128 = 16;
        // D s_2_2: cast zx s_2_0 -> i
        let s_2_2: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_1
        let s_2_3: bool = ((s_2_2) == (s_2_1));
        // D s_2_4: not s_2_3
        let s_2_4: bool = !s_2_3;
        // N s_2_5: branch s_2_4 b5 b3
        if s_2_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var d:i64
        let s_3_0: i64 = fn_state.d;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call S_read(s_3_1)
        let s_3_2: u32 = S_read(state, tracer, s_3_1);
        // C s_3_3: const #1s : i
        let s_3_3: i128 = 1;
        // D s_3_4: read-var sizeshadow#7423:i64
        let s_3_4: i64 = fn_state.sizeshadow_7423;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: sub s_3_5 s_3_3
        let s_3_6: i128 = ((s_3_5) - (s_3_3));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // C s_3_8: const #0s : i
        let s_3_8: i128 = 0;
        // D s_3_9: cast zx s_3_2 -> bv
        let s_3_9: Bits = Bits::new(s_3_2 as u128, 32u16);
        // D s_3_10: cast zx s_3_7 -> i
        let s_3_10: i128 = (i128::try_from(s_3_7).unwrap());
        // C s_3_11: const #1s : i64
        let s_3_11: i64 = 1;
        // C s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: sub s_3_10 s_3_8
        let s_3_13: i128 = ((s_3_10) - (s_3_8));
        // D s_3_14: add s_3_13 s_3_12
        let s_3_14: i128 = (s_3_13 + s_3_12);
        // D s_3_15: bit-extract s_3_9 s_3_8 s_3_14
        let s_3_15: Bits = (Bits::new(
            ((s_3_9) >> (s_3_8)).value(),
            u16::try_from(s_3_14).unwrap(),
        ));
        // C s_3_16: const #() : ()
        let s_3_16: () = ();
        // S s_3_17: call FPSCR_read(s_3_16)
        let s_3_17: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_3_16);
        // C s_3_18: const #16s : i64
        let s_3_18: i64 = 16;
        // D s_3_19: read-var frac_bits:i
        let s_3_19: i128 = fn_state.frac_bits;
        // D s_3_20: read-var is_unsigned:u8
        let s_3_20: bool = fn_state.is_unsigned;
        // C s_3_21: const #0u : u32
        let s_3_21: u32 = 0;
        // D s_3_22: call FixedToFP(s_3_15, s_3_19, s_3_20, s_3_17, s_3_21, s_3_18)
        let s_3_22: Bits = FixedToFP(
            state,
            tracer,
            s_3_15,
            s_3_19,
            s_3_20,
            s_3_17,
            s_3_21,
            s_3_18,
        );
        // D s_3_23: write-var gs#893740 <= s_3_22
        fn_state.gs_893740 = s_3_22;
        // N s_3_24: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#893740:bv
        let s_4_0: Bits = fn_state.gs_893740;
        // D s_4_1: cast reint s_4_0 -> u16
        let s_4_1: u16 = (s_4_0.value() as u16);
        // C s_4_2: const #0u : u16
        let s_4_2: u16 = 0;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 16u16);
        // D s_4_4: cast zx s_4_1 -> bv
        let s_4_4: Bits = Bits::new(s_4_1 as u128, 16u16);
        // C s_4_5: cast reint s_4_3 -> u128
        let s_4_5: u128 = (s_4_3.value() as u128);
        // D s_4_6: size-of s_4_3
        let s_4_6: u16 = s_4_3.length();
        // D s_4_7: cast reint s_4_4 -> u128
        let s_4_7: u128 = (s_4_4.value() as u128);
        // D s_4_8: size-of s_4_4
        let s_4_8: u16 = s_4_4.length();
        // D s_4_9: lsl s_4_5 s_4_8
        let s_4_9: u128 = s_4_5 << s_4_8;
        // D s_4_10: or s_4_9 s_4_7
        let s_4_10: u128 = ((s_4_9) | (s_4_7));
        // D s_4_11: add s_4_6 s_4_8
        let s_4_11: u16 = (s_4_6 + s_4_8);
        // D s_4_12: create-bits s_4_10 s_4_11
        let s_4_12: Bits = Bits::new(s_4_10, s_4_11);
        // D s_4_13: cast reint s_4_12 -> u32
        let s_4_13: u32 = (s_4_12.value() as u32);
        // D s_4_14: read-var d:i64
        let s_4_14: i64 = fn_state.d;
        // D s_4_15: cast zx s_4_14 -> i
        let s_4_15: i128 = (i128::try_from(s_4_14).unwrap());
        // D s_4_16: call S_set(s_4_15, s_4_13)
        let s_4_16: () = S_set(state, tracer, s_4_15, s_4_13);
        // N s_4_17: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var fp_size:i64
        let s_5_0: i64 = fn_state.fp_size;
        // C s_5_1: const #32s : i
        let s_5_1: i128 = 32;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b8 b6
        if s_5_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var d:i64
        let s_6_0: i64 = fn_state.d;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call S_read(s_6_1)
        let s_6_2: u32 = S_read(state, tracer, s_6_1);
        // C s_6_3: const #1s : i
        let s_6_3: i128 = 1;
        // D s_6_4: read-var sizeshadow#7423:i64
        let s_6_4: i64 = fn_state.sizeshadow_7423;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: sub s_6_5 s_6_3
        let s_6_6: i128 = ((s_6_5) - (s_6_3));
        // D s_6_7: cast reint s_6_6 -> i64
        let s_6_7: i64 = (s_6_6 as i64);
        // C s_6_8: const #0s : i
        let s_6_8: i128 = 0;
        // D s_6_9: cast zx s_6_2 -> bv
        let s_6_9: Bits = Bits::new(s_6_2 as u128, 32u16);
        // D s_6_10: cast zx s_6_7 -> i
        let s_6_10: i128 = (i128::try_from(s_6_7).unwrap());
        // C s_6_11: const #1s : i64
        let s_6_11: i64 = 1;
        // C s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_13: sub s_6_10 s_6_8
        let s_6_13: i128 = ((s_6_10) - (s_6_8));
        // D s_6_14: add s_6_13 s_6_12
        let s_6_14: i128 = (s_6_13 + s_6_12);
        // D s_6_15: bit-extract s_6_9 s_6_8 s_6_14
        let s_6_15: Bits = (Bits::new(
            ((s_6_9) >> (s_6_8)).value(),
            u16::try_from(s_6_14).unwrap(),
        ));
        // C s_6_16: const #() : ()
        let s_6_16: () = ();
        // S s_6_17: call FPSCR_read(s_6_16)
        let s_6_17: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_6_16);
        // C s_6_18: const #32s : i64
        let s_6_18: i64 = 32;
        // D s_6_19: read-var frac_bits:i
        let s_6_19: i128 = fn_state.frac_bits;
        // D s_6_20: read-var is_unsigned:u8
        let s_6_20: bool = fn_state.is_unsigned;
        // C s_6_21: const #0u : u32
        let s_6_21: u32 = 0;
        // D s_6_22: call FixedToFP(s_6_15, s_6_19, s_6_20, s_6_17, s_6_21, s_6_18)
        let s_6_22: Bits = FixedToFP(
            state,
            tracer,
            s_6_15,
            s_6_19,
            s_6_20,
            s_6_17,
            s_6_21,
            s_6_18,
        );
        // D s_6_23: write-var gs#893752 <= s_6_22
        fn_state.gs_893752 = s_6_22;
        // N s_6_24: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#893752:bv
        let s_7_0: Bits = fn_state.gs_893752;
        // D s_7_1: cast reint s_7_0 -> u32
        let s_7_1: u32 = (s_7_0.value() as u32);
        // D s_7_2: read-var d:i64
        let s_7_2: i64 = fn_state.d;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: call S_set(s_7_3, s_7_1)
        let s_7_4: () = S_set(state, tracer, s_7_3, s_7_1);
        // N s_7_5: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var fp_size:i64
        let s_8_0: i64 = fn_state.fp_size;
        // C s_8_1: const #64s : i
        let s_8_1: i128 = 64;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b11 b9
        if s_8_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var d:i64
        let s_9_0: i64 = fn_state.d;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call D_read(s_9_1)
        let s_9_2: u64 = D_read(state, tracer, s_9_1);
        // C s_9_3: const #1s : i
        let s_9_3: i128 = 1;
        // D s_9_4: read-var sizeshadow#7423:i64
        let s_9_4: i64 = fn_state.sizeshadow_7423;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: sub s_9_5 s_9_3
        let s_9_6: i128 = ((s_9_5) - (s_9_3));
        // D s_9_7: cast reint s_9_6 -> i64
        let s_9_7: i64 = (s_9_6 as i64);
        // C s_9_8: const #0s : i
        let s_9_8: i128 = 0;
        // D s_9_9: cast zx s_9_2 -> bv
        let s_9_9: Bits = Bits::new(s_9_2 as u128, 64u16);
        // D s_9_10: cast zx s_9_7 -> i
        let s_9_10: i128 = (i128::try_from(s_9_7).unwrap());
        // C s_9_11: const #1s : i64
        let s_9_11: i64 = 1;
        // C s_9_12: cast zx s_9_11 -> i
        let s_9_12: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_13: sub s_9_10 s_9_8
        let s_9_13: i128 = ((s_9_10) - (s_9_8));
        // D s_9_14: add s_9_13 s_9_12
        let s_9_14: i128 = (s_9_13 + s_9_12);
        // D s_9_15: bit-extract s_9_9 s_9_8 s_9_14
        let s_9_15: Bits = (Bits::new(
            ((s_9_9) >> (s_9_8)).value(),
            u16::try_from(s_9_14).unwrap(),
        ));
        // C s_9_16: const #() : ()
        let s_9_16: () = ();
        // S s_9_17: call FPSCR_read(s_9_16)
        let s_9_17: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_9_16);
        // C s_9_18: const #64s : i64
        let s_9_18: i64 = 64;
        // D s_9_19: read-var frac_bits:i
        let s_9_19: i128 = fn_state.frac_bits;
        // D s_9_20: read-var is_unsigned:u8
        let s_9_20: bool = fn_state.is_unsigned;
        // C s_9_21: const #0u : u32
        let s_9_21: u32 = 0;
        // D s_9_22: call FixedToFP(s_9_15, s_9_19, s_9_20, s_9_17, s_9_21, s_9_18)
        let s_9_22: Bits = FixedToFP(
            state,
            tracer,
            s_9_15,
            s_9_19,
            s_9_20,
            s_9_17,
            s_9_21,
            s_9_18,
        );
        // D s_9_23: write-var gs#893761 <= s_9_22
        fn_state.gs_893761 = s_9_22;
        // N s_9_24: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#893761:bv
        let s_10_0: Bits = fn_state.gs_893761;
        // D s_10_1: cast reint s_10_0 -> u64
        let s_10_1: u64 = (s_10_0.value() as u64);
        // D s_10_2: read-var d:i64
        let s_10_2: i64 = fn_state.d;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: call D_set(s_10_3, s_10_1)
        let s_10_4: () = D_set(state, tracer, s_10_3, s_10_1);
        // N s_10_5: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var fp_size:i64
        let s_12_0: i64 = fn_state.fp_size;
        // C s_12_1: const #16s : i
        let s_12_1: i128 = 16;
        // D s_12_2: cast zx s_12_0 -> i
        let s_12_2: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_3: cmp-eq s_12_2 s_12_1
        let s_12_3: bool = ((s_12_2) == (s_12_1));
        // D s_12_4: not s_12_3
        let s_12_4: bool = !s_12_3;
        // N s_12_5: branch s_12_4 b18 b13
        if s_12_4 {
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
        // D s_13_0: read-var d:i64
        let s_13_0: i64 = fn_state.d;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call S_read(s_13_1)
        let s_13_2: u32 = S_read(state, tracer, s_13_1);
        // C s_13_3: const #0s : i
        let s_13_3: i128 = 0;
        // D s_13_4: cast zx s_13_2 -> bv
        let s_13_4: Bits = Bits::new(s_13_2 as u128, 32u16);
        // C s_13_5: const #1s : i64
        let s_13_5: i64 = 1;
        // C s_13_6: cast zx s_13_5 -> i
        let s_13_6: i128 = (i128::try_from(s_13_5).unwrap());
        // C s_13_7: const #15s : i
        let s_13_7: i128 = 15;
        // C s_13_8: add s_13_7 s_13_6
        let s_13_8: i128 = (s_13_7 + s_13_6);
        // D s_13_9: bit-extract s_13_4 s_13_3 s_13_8
        let s_13_9: Bits = (Bits::new(
            ((s_13_4) >> (s_13_3)).value(),
            u16::try_from(s_13_8).unwrap(),
        ));
        // D s_13_10: cast reint s_13_9 -> u16
        let s_13_10: u16 = (s_13_9.value() as u16);
        // C s_13_11: const #() : ()
        let s_13_11: () = ();
        // S s_13_12: call FPSCR_read(s_13_11)
        let s_13_12: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_13_11);
        // D s_13_13: read-var sizeshadow#7423:i64
        let s_13_13: i64 = fn_state.sizeshadow_7423;
        // D s_13_14: cast zx s_13_13 -> i
        let s_13_14: i128 = (i128::try_from(s_13_13).unwrap());
        // D s_13_15: cast reint s_13_14 -> i64
        let s_13_15: i64 = (s_13_14 as i64);
        // D s_13_16: cast zx s_13_10 -> bv
        let s_13_16: Bits = Bits::new(s_13_10 as u128, 16u16);
        // D s_13_17: read-var frac_bits:i
        let s_13_17: i128 = fn_state.frac_bits;
        // D s_13_18: read-var is_unsigned:u8
        let s_13_18: bool = fn_state.is_unsigned;
        // C s_13_19: const #3u : u32
        let s_13_19: u32 = 3;
        // D s_13_20: call FPToFixed(s_13_16, s_13_17, s_13_18, s_13_12, s_13_19, s_13_15)
        let s_13_20: Bits = FPToFixed(
            state,
            tracer,
            s_13_16,
            s_13_17,
            s_13_18,
            s_13_12,
            s_13_19,
            s_13_15,
        );
        // D s_13_21: write-var resultshadow#7424 <= s_13_20
        fn_state.resultshadow_7424 = s_13_20;
        // N s_13_22: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var is_unsigned:u8
        let s_14_0: bool = fn_state.is_unsigned;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
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
        // C s_15_0: const #32s : i
        let s_15_0: i128 = 32;
        // D s_15_1: read-var resultshadow#7424:bv
        let s_15_1: Bits = fn_state.resultshadow_7424;
        // D s_15_2: bits-cast sx s_15_1 -> bv length s_15_0
        let s_15_2: Bits = s_15_1.sign_extend(s_15_0);
        // D s_15_3: cast reint s_15_2 -> u32
        let s_15_3: u32 = (s_15_2.value() as u32);
        // D s_15_4: write-var ga#352574 <= s_15_3
        fn_state.ga_352574 = s_15_3;
        // N s_15_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var d:i64
        let s_16_0: i64 = fn_state.d;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: read-var ga#352574:u32
        let s_16_2: u32 = fn_state.ga_352574;
        // D s_16_3: call S_set(s_16_1, s_16_2)
        let s_16_3: () = S_set(state, tracer, s_16_1, s_16_2);
        // N s_16_4: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #32s : i
        let s_17_0: i128 = 32;
        // D s_17_1: read-var resultshadow#7424:bv
        let s_17_1: Bits = fn_state.resultshadow_7424;
        // D s_17_2: bits-cast zx s_17_1 -> bv length s_17_0
        let s_17_2: Bits = s_17_1.zero_extend(s_17_0);
        // D s_17_3: cast reint s_17_2 -> u32
        let s_17_3: u32 = (s_17_2.value() as u32);
        // D s_17_4: write-var ga#352574 <= s_17_3
        fn_state.ga_352574 = s_17_3;
        // N s_17_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var fp_size:i64
        let s_18_0: i64 = fn_state.fp_size;
        // C s_18_1: const #32s : i
        let s_18_1: i128 = 32;
        // D s_18_2: cast zx s_18_0 -> i
        let s_18_2: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_3: cmp-eq s_18_2 s_18_1
        let s_18_3: bool = ((s_18_2) == (s_18_1));
        // D s_18_4: not s_18_3
        let s_18_4: bool = !s_18_3;
        // N s_18_5: branch s_18_4 b24 b19
        if s_18_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var d:i64
        let s_19_0: i64 = fn_state.d;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: call S_read(s_19_1)
        let s_19_2: u32 = S_read(state, tracer, s_19_1);
        // C s_19_3: const #() : ()
        let s_19_3: () = ();
        // S s_19_4: call FPSCR_read(s_19_3)
        let s_19_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_19_3);
        // D s_19_5: read-var sizeshadow#7423:i64
        let s_19_5: i64 = fn_state.sizeshadow_7423;
        // D s_19_6: cast zx s_19_5 -> i
        let s_19_6: i128 = (i128::try_from(s_19_5).unwrap());
        // D s_19_7: cast reint s_19_6 -> i64
        let s_19_7: i64 = (s_19_6 as i64);
        // D s_19_8: cast zx s_19_2 -> bv
        let s_19_8: Bits = Bits::new(s_19_2 as u128, 32u16);
        // D s_19_9: read-var frac_bits:i
        let s_19_9: i128 = fn_state.frac_bits;
        // D s_19_10: read-var is_unsigned:u8
        let s_19_10: bool = fn_state.is_unsigned;
        // C s_19_11: const #3u : u32
        let s_19_11: u32 = 3;
        // D s_19_12: call FPToFixed(s_19_8, s_19_9, s_19_10, s_19_4, s_19_11, s_19_7)
        let s_19_12: Bits = FPToFixed(
            state,
            tracer,
            s_19_8,
            s_19_9,
            s_19_10,
            s_19_4,
            s_19_11,
            s_19_7,
        );
        // D s_19_13: write-var resultshadow#7425 <= s_19_12
        fn_state.resultshadow_7425 = s_19_12;
        // N s_19_14: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var is_unsigned:u8
        let s_20_0: bool = fn_state.is_unsigned;
        // N s_20_1: branch s_20_0 b23 b21
        if s_20_0 {
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
        // C s_21_0: const #32s : i
        let s_21_0: i128 = 32;
        // D s_21_1: read-var resultshadow#7425:bv
        let s_21_1: Bits = fn_state.resultshadow_7425;
        // D s_21_2: bits-cast sx s_21_1 -> bv length s_21_0
        let s_21_2: Bits = s_21_1.sign_extend(s_21_0);
        // D s_21_3: cast reint s_21_2 -> u32
        let s_21_3: u32 = (s_21_2.value() as u32);
        // D s_21_4: write-var ga#352579 <= s_21_3
        fn_state.ga_352579 = s_21_3;
        // N s_21_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var d:i64
        let s_22_0: i64 = fn_state.d;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: read-var ga#352579:u32
        let s_22_2: u32 = fn_state.ga_352579;
        // D s_22_3: call S_set(s_22_1, s_22_2)
        let s_22_3: () = S_set(state, tracer, s_22_1, s_22_2);
        // N s_22_4: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #32s : i
        let s_23_0: i128 = 32;
        // D s_23_1: read-var resultshadow#7425:bv
        let s_23_1: Bits = fn_state.resultshadow_7425;
        // D s_23_2: bits-cast zx s_23_1 -> bv length s_23_0
        let s_23_2: Bits = s_23_1.zero_extend(s_23_0);
        // D s_23_3: cast reint s_23_2 -> u32
        let s_23_3: u32 = (s_23_2.value() as u32);
        // D s_23_4: write-var ga#352579 <= s_23_3
        fn_state.ga_352579 = s_23_3;
        // N s_23_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var fp_size:i64
        let s_24_0: i64 = fn_state.fp_size;
        // C s_24_1: const #64s : i
        let s_24_1: i128 = 64;
        // D s_24_2: cast zx s_24_0 -> i
        let s_24_2: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_3: cmp-eq s_24_2 s_24_1
        let s_24_3: bool = ((s_24_2) == (s_24_1));
        // D s_24_4: not s_24_3
        let s_24_4: bool = !s_24_3;
        // N s_24_5: branch s_24_4 b30 b25
        if s_24_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var d:i64
        let s_25_0: i64 = fn_state.d;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: call D_read(s_25_1)
        let s_25_2: u64 = D_read(state, tracer, s_25_1);
        // C s_25_3: const #() : ()
        let s_25_3: () = ();
        // S s_25_4: call FPSCR_read(s_25_3)
        let s_25_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_25_3);
        // D s_25_5: read-var sizeshadow#7423:i64
        let s_25_5: i64 = fn_state.sizeshadow_7423;
        // D s_25_6: cast zx s_25_5 -> i
        let s_25_6: i128 = (i128::try_from(s_25_5).unwrap());
        // D s_25_7: cast reint s_25_6 -> i64
        let s_25_7: i64 = (s_25_6 as i64);
        // D s_25_8: cast zx s_25_2 -> bv
        let s_25_8: Bits = Bits::new(s_25_2 as u128, 64u16);
        // D s_25_9: read-var frac_bits:i
        let s_25_9: i128 = fn_state.frac_bits;
        // D s_25_10: read-var is_unsigned:u8
        let s_25_10: bool = fn_state.is_unsigned;
        // C s_25_11: const #3u : u32
        let s_25_11: u32 = 3;
        // D s_25_12: call FPToFixed(s_25_8, s_25_9, s_25_10, s_25_4, s_25_11, s_25_7)
        let s_25_12: Bits = FPToFixed(
            state,
            tracer,
            s_25_8,
            s_25_9,
            s_25_10,
            s_25_4,
            s_25_11,
            s_25_7,
        );
        // D s_25_13: write-var resultshadow#7426 <= s_25_12
        fn_state.resultshadow_7426 = s_25_12;
        // N s_25_14: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var is_unsigned:u8
        let s_26_0: bool = fn_state.is_unsigned;
        // N s_26_1: branch s_26_0 b29 b27
        if s_26_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #64s : i
        let s_27_0: i128 = 64;
        // D s_27_1: read-var resultshadow#7426:bv
        let s_27_1: Bits = fn_state.resultshadow_7426;
        // D s_27_2: bits-cast sx s_27_1 -> bv length s_27_0
        let s_27_2: Bits = s_27_1.sign_extend(s_27_0);
        // D s_27_3: cast reint s_27_2 -> u64
        let s_27_3: u64 = (s_27_2.value() as u64);
        // D s_27_4: write-var ga#352583 <= s_27_3
        fn_state.ga_352583 = s_27_3;
        // N s_27_5: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var d:i64
        let s_28_0: i64 = fn_state.d;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: read-var ga#352583:u64
        let s_28_2: u64 = fn_state.ga_352583;
        // D s_28_3: call D_set(s_28_1, s_28_2)
        let s_28_3: () = D_set(state, tracer, s_28_1, s_28_2);
        // N s_28_4: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #64s : i
        let s_29_0: i128 = 64;
        // D s_29_1: read-var resultshadow#7426:bv
        let s_29_1: Bits = fn_state.resultshadow_7426;
        // D s_29_2: bits-cast zx s_29_1 -> bv length s_29_0
        let s_29_2: Bits = s_29_1.zero_extend(s_29_0);
        // D s_29_3: cast reint s_29_2 -> u64
        let s_29_3: u64 = (s_29_2.value() as u64);
        // D s_29_4: write-var ga#352583 <= s_29_3
        fn_state.ga_352583 = s_29_3;
        // N s_29_5: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: return
        return;
    }
}
