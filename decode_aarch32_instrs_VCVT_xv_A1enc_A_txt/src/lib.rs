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
use ConditionPassed::*;
use execute_aarch32_instrs_VCVT_xv_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VCVT_xv_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    D: bool,
    op: bool,
    U: bool,
    Vd: u8,
    sf: u8,
    sx: bool,
    i: bool,
    imm4: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_352614: i64,
        gs_308408: bool,
        dshadow_7428: i64,
        fp_size: i64,
        frac_bits: i64,
        gs_308407: bool,
        d: i64,
        size: i64,
        is_unsigned: bool,
        to_fixed: bool,
        fp_sizeshadow_7427: i64,
        cond: u8,
        D: bool,
        op: bool,
        U: bool,
        Vd: u8,
        sf: u8,
        sx: bool,
        i: bool,
        imm4: u8,
    }
    let fn_state = FunctionState {
        cond,
        D,
        op,
        U,
        Vd,
        sf,
        sx,
        i,
        imm4,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var cond:u8
        let s_2_0: u8 = fn_state.cond;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // D s_2_6: read-var sf:u8
        let s_2_6: u8 = fn_state.sf;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 2u16);
        // C s_2_8: const #0u : u8
        let s_2_8: u8 = 0;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 2u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // N s_2_11: branch s_2_10 b24 b3
        if s_2_10 {
            return block_24(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#308407 <= s_3_0
        fn_state.gs_308407 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#308407:u8
        let s_4_0: bool = fn_state.gs_308407;
        // N s_4_1: branch s_4_0 b23 b5
        if s_4_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var sf:u8
        let s_5_0: u8 = fn_state.sf;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #1u : u8
        let s_5_2: u8 = 1;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b22 b6
        if s_5_4 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#308408 <= s_6_0
        fn_state.gs_308408 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#308408:u8
        let s_7_0: bool = fn_state.gs_308408;
        // N s_7_1: branch s_7_0 b21 b8
        if s_7_0 {
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
        // D s_8_0: read-var op:u8
        let s_8_0: bool = fn_state.op;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #1u : u8
        let s_8_2: bool = true;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: write-var to_fixed <= s_8_4
        fn_state.to_fixed = s_8_4;
        // D s_8_6: read-var U:u8
        let s_8_6: bool = fn_state.U;
        // D s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 1u16);
        // C s_8_8: const #1u : u8
        let s_8_8: bool = true;
        // C s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 1u16);
        // D s_8_10: cmp-eq s_8_7 s_8_9
        let s_8_10: bool = ((s_8_7) == (s_8_9));
        // D s_8_11: write-var is_unsigned <= s_8_10
        fn_state.is_unsigned = s_8_10;
        // D s_8_12: read-var sx:u8
        let s_8_12: bool = fn_state.sx;
        // D s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 1u16);
        // C s_8_14: const #0u : u8
        let s_8_14: bool = false;
        // C s_8_15: cast zx s_8_14 -> bv
        let s_8_15: Bits = Bits::new(s_8_14 as u128, 1u16);
        // D s_8_16: cmp-eq s_8_13 s_8_15
        let s_8_16: bool = ((s_8_13) == (s_8_15));
        // N s_8_17: branch s_8_16 b20 b9
        if s_8_16 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #32s : i64
        let s_9_0: i64 = 32;
        // D s_9_1: write-var ga#352614 <= s_9_0
        fn_state.ga_352614 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#352614:i64
        let s_10_0: i64 = fn_state.ga_352614;
        // D s_10_1: write-var size <= s_10_0
        fn_state.size = s_10_0;
        // D s_10_2: read-var imm4:u8
        let s_10_2: u8 = fn_state.imm4;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 4u16);
        // D s_10_4: read-var i:u8
        let s_10_4: bool = fn_state.i;
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: cast reint s_10_3 -> u128
        let s_10_6: u128 = (s_10_3.value() as u128);
        // D s_10_7: size-of s_10_3
        let s_10_7: u16 = s_10_3.length();
        // D s_10_8: cast reint s_10_5 -> u128
        let s_10_8: u128 = (s_10_5.value() as u128);
        // D s_10_9: size-of s_10_5
        let s_10_9: u16 = s_10_5.length();
        // D s_10_10: lsl s_10_6 s_10_9
        let s_10_10: u128 = s_10_6 << s_10_9;
        // D s_10_11: or s_10_10 s_10_8
        let s_10_11: u128 = ((s_10_10) | (s_10_8));
        // D s_10_12: add s_10_7 s_10_9
        let s_10_12: u16 = (s_10_7 + s_10_9);
        // D s_10_13: create-bits s_10_11 s_10_12
        let s_10_13: Bits = Bits::new(s_10_11, s_10_12);
        // D s_10_14: cast reint s_10_13 -> u8
        let s_10_14: u8 = (s_10_13.value() as u8);
        // D s_10_15: cast zx s_10_14 -> bv
        let s_10_15: Bits = Bits::new(s_10_14 as u128, 5u16);
        // D s_10_16: cast zx s_10_15 -> i
        let s_10_16: i128 = (s_10_15.value() as i128);
        // D s_10_17: cast reint s_10_16 -> i64
        let s_10_17: i64 = (s_10_16 as i64);
        // D s_10_18: read-var size:i64
        let s_10_18: i64 = fn_state.size;
        // D s_10_19: cast zx s_10_18 -> i
        let s_10_19: i128 = (i128::try_from(s_10_18).unwrap());
        // D s_10_20: cast zx s_10_17 -> i
        let s_10_20: i128 = (i128::try_from(s_10_17).unwrap());
        // D s_10_21: sub s_10_19 s_10_20
        let s_10_21: i128 = ((s_10_19) - (s_10_20));
        // D s_10_22: cast reint s_10_21 -> i64
        let s_10_22: i64 = (s_10_21 as i64);
        // D s_10_23: write-var frac_bits <= s_10_22
        fn_state.frac_bits = s_10_22;
        // C s_10_24: const #16s : i64
        let s_10_24: i64 = 16;
        // D s_10_25: write-var fp_size <= s_10_24
        fn_state.fp_size = s_10_24;
        // D s_10_26: read-var sf:u8
        let s_10_26: u8 = fn_state.sf;
        // D s_10_27: cast zx s_10_26 -> bv
        let s_10_27: Bits = Bits::new(s_10_26 as u128, 2u16);
        // C s_10_28: const #1u : u8
        let s_10_28: u8 = 1;
        // C s_10_29: cast zx s_10_28 -> bv
        let s_10_29: Bits = Bits::new(s_10_28 as u128, 2u16);
        // D s_10_30: cmp-eq s_10_27 s_10_29
        let s_10_30: bool = ((s_10_27) == (s_10_29));
        // D s_10_31: not s_10_30
        let s_10_31: bool = !s_10_30;
        // N s_10_32: branch s_10_31 b15 b11
        if s_10_31 {
            return block_15(state, tracer, fn_state);
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
        // D s_11_1: write-var fp_size <= s_11_0
        fn_state.fp_size = s_11_0;
        // D s_11_2: read-var Vd:u8
        let s_11_2: u8 = fn_state.Vd;
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 4u16);
        // D s_11_4: read-var D:u8
        let s_11_4: bool = fn_state.D;
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cast reint s_11_3 -> u128
        let s_11_6: u128 = (s_11_3.value() as u128);
        // D s_11_7: size-of s_11_3
        let s_11_7: u16 = s_11_3.length();
        // D s_11_8: cast reint s_11_5 -> u128
        let s_11_8: u128 = (s_11_5.value() as u128);
        // D s_11_9: size-of s_11_5
        let s_11_9: u16 = s_11_5.length();
        // D s_11_10: lsl s_11_6 s_11_9
        let s_11_10: u128 = s_11_6 << s_11_9;
        // D s_11_11: or s_11_10 s_11_8
        let s_11_11: u128 = ((s_11_10) | (s_11_8));
        // D s_11_12: add s_11_7 s_11_9
        let s_11_12: u16 = (s_11_7 + s_11_9);
        // D s_11_13: create-bits s_11_11 s_11_12
        let s_11_13: Bits = Bits::new(s_11_11, s_11_12);
        // D s_11_14: cast reint s_11_13 -> u8
        let s_11_14: u8 = (s_11_13.value() as u8);
        // D s_11_15: cast zx s_11_14 -> bv
        let s_11_15: Bits = Bits::new(s_11_14 as u128, 5u16);
        // D s_11_16: cast zx s_11_15 -> i
        let s_11_16: i128 = (s_11_15.value() as i128);
        // D s_11_17: cast reint s_11_16 -> i64
        let s_11_17: i64 = (s_11_16 as i64);
        // D s_11_18: write-var d <= s_11_17
        fn_state.d = s_11_17;
        // N s_11_19: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var fp_size:i64
        let s_12_0: i64 = fn_state.fp_size;
        // D s_12_1: write-var fp_sizeshadow#7427 <= s_12_0
        fn_state.fp_sizeshadow_7427 = s_12_0;
        // D s_12_2: read-var d:i64
        let s_12_2: i64 = fn_state.d;
        // D s_12_3: write-var dshadow#7428 <= s_12_2
        fn_state.dshadow_7428 = s_12_2;
        // C s_12_4: const #0s : i
        let s_12_4: i128 = 0;
        // D s_12_5: read-var frac_bits:i64
        let s_12_5: i64 = fn_state.frac_bits;
        // D s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_7: cmp-lt s_12_6 s_12_4
        let s_12_7: bool = ((s_12_6) < (s_12_4));
        // N s_12_8: branch s_12_7 b14 b13
        if s_12_7 {
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
        // D s_13_0: read-var dshadow#7428:i64
        let s_13_0: i64 = fn_state.dshadow_7428;
        // D s_13_1: read-var size:i64
        let s_13_1: i64 = fn_state.size;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // D s_13_4: read-var frac_bits:i64
        let s_13_4: i64 = fn_state.frac_bits;
        // D s_13_5: cast zx s_13_4 -> i
        let s_13_5: i128 = (i128::try_from(s_13_4).unwrap());
        // D s_13_6: read-var fp_sizeshadow#7427:i64
        let s_13_6: i64 = fn_state.fp_sizeshadow_7427;
        // D s_13_7: read-var to_fixed:u8
        let s_13_7: bool = fn_state.to_fixed;
        // D s_13_8: read-var is_unsigned:u8
        let s_13_8: bool = fn_state.is_unsigned;
        // D s_13_9: call execute_aarch32_instrs_VCVT_xv_Op_A_txt(s_13_0, s_13_6, s_13_5, s_13_3, s_13_7, s_13_8)
        let s_13_9: () = execute_aarch32_instrs_VCVT_xv_Op_A_txt(
            state,
            tracer,
            s_13_0,
            s_13_6,
            s_13_5,
            s_13_3,
            s_13_7,
            s_13_8,
        );
        // N s_13_10: return
        return;
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
        // D s_15_0: read-var sf:u8
        let s_15_0: u8 = fn_state.sf;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 2u16);
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 2u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: not s_15_4
        let s_15_5: bool = !s_15_4;
        // N s_15_6: branch s_15_5 b17 b16
        if s_15_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #32s : i64
        let s_16_0: i64 = 32;
        // D s_16_1: write-var fp_size <= s_16_0
        fn_state.fp_size = s_16_0;
        // D s_16_2: read-var Vd:u8
        let s_16_2: u8 = fn_state.Vd;
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 4u16);
        // D s_16_4: read-var D:u8
        let s_16_4: bool = fn_state.D;
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: cast reint s_16_3 -> u128
        let s_16_6: u128 = (s_16_3.value() as u128);
        // D s_16_7: size-of s_16_3
        let s_16_7: u16 = s_16_3.length();
        // D s_16_8: cast reint s_16_5 -> u128
        let s_16_8: u128 = (s_16_5.value() as u128);
        // D s_16_9: size-of s_16_5
        let s_16_9: u16 = s_16_5.length();
        // D s_16_10: lsl s_16_6 s_16_9
        let s_16_10: u128 = s_16_6 << s_16_9;
        // D s_16_11: or s_16_10 s_16_8
        let s_16_11: u128 = ((s_16_10) | (s_16_8));
        // D s_16_12: add s_16_7 s_16_9
        let s_16_12: u16 = (s_16_7 + s_16_9);
        // D s_16_13: create-bits s_16_11 s_16_12
        let s_16_13: Bits = Bits::new(s_16_11, s_16_12);
        // D s_16_14: cast reint s_16_13 -> u8
        let s_16_14: u8 = (s_16_13.value() as u8);
        // D s_16_15: cast zx s_16_14 -> bv
        let s_16_15: Bits = Bits::new(s_16_14 as u128, 5u16);
        // D s_16_16: cast zx s_16_15 -> i
        let s_16_16: i128 = (s_16_15.value() as i128);
        // D s_16_17: cast reint s_16_16 -> i64
        let s_16_17: i64 = (s_16_16 as i64);
        // D s_16_18: write-var d <= s_16_17
        fn_state.d = s_16_17;
        // N s_16_19: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var sf:u8
        let s_17_0: u8 = fn_state.sf;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #3u : u8
        let s_17_2: u8 = 3;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: not s_17_4
        let s_17_5: bool = !s_17_4;
        // N s_17_6: branch s_17_5 b19 b18
        if s_17_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #64s : i64
        let s_18_0: i64 = 64;
        // D s_18_1: write-var fp_size <= s_18_0
        fn_state.fp_size = s_18_0;
        // D s_18_2: read-var D:u8
        let s_18_2: bool = fn_state.D;
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: read-var Vd:u8
        let s_18_4: u8 = fn_state.Vd;
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 4u16);
        // D s_18_6: cast reint s_18_3 -> u128
        let s_18_6: u128 = (s_18_3.value() as u128);
        // D s_18_7: size-of s_18_3
        let s_18_7: u16 = s_18_3.length();
        // D s_18_8: cast reint s_18_5 -> u128
        let s_18_8: u128 = (s_18_5.value() as u128);
        // D s_18_9: size-of s_18_5
        let s_18_9: u16 = s_18_5.length();
        // D s_18_10: lsl s_18_6 s_18_9
        let s_18_10: u128 = s_18_6 << s_18_9;
        // D s_18_11: or s_18_10 s_18_8
        let s_18_11: u128 = ((s_18_10) | (s_18_8));
        // D s_18_12: add s_18_7 s_18_9
        let s_18_12: u16 = (s_18_7 + s_18_9);
        // D s_18_13: create-bits s_18_11 s_18_12
        let s_18_13: Bits = Bits::new(s_18_11, s_18_12);
        // D s_18_14: cast reint s_18_13 -> u8
        let s_18_14: u8 = (s_18_13.value() as u8);
        // D s_18_15: cast zx s_18_14 -> bv
        let s_18_15: Bits = Bits::new(s_18_14 as u128, 5u16);
        // D s_18_16: cast zx s_18_15 -> i
        let s_18_16: i128 = (s_18_15.value() as i128);
        // D s_18_17: cast reint s_18_16 -> i64
        let s_18_17: i64 = (s_18_16 as i64);
        // D s_18_18: write-var d <= s_18_17
        fn_state.d = s_18_17;
        // N s_18_19: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #16s : i64
        let s_20_0: i64 = 16;
        // D s_20_1: write-var ga#352614 <= s_20_0
        fn_state.ga_352614 = s_20_0;
        // N s_20_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var cond:u8
        let s_22_0: u8 = fn_state.cond;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 4u16);
        // C s_22_2: const #14u : u8
        let s_22_2: u8 = 14;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 4u16);
        // D s_22_4: cmp-ne s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) != (s_22_3));
        // D s_22_5: write-var gs#308408 <= s_22_4
        fn_state.gs_308408 = s_22_4;
        // N s_22_6: jump b7
        return block_7(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#308407 <= s_24_0
        fn_state.gs_308407 = s_24_0;
        // N s_24_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
