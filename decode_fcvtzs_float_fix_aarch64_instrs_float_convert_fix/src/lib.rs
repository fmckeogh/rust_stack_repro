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
use FPCR_read::*;
use FPRoundingMode::*;
use execute_aarch64_instrs_float_convert_fix::*;
use common::*;
pub fn decode_fcvtzs_float_fix_aarch64_instrs_float_convert_fix<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    scale: u8,
    opcode: u8,
    rmode: u8,
    ftype: u8,
    sf: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        fracbits: i64,
        fltsizeshadow_1357: i64,
        op: u32,
        fltsize: i64,
        n: i64,
        gs_151921: bool,
        d: i64,
        intsize: i64,
        ga_254785: i64,
        is_unsigned: bool,
        rounding: u32,
        ga_254795: u8,
        Rd: u8,
        Rn: u8,
        scale: u8,
        opcode: u8,
        rmode: u8,
        ftype: u8,
        sf: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        scale,
        opcode,
        rmode,
        ftype,
        sf,
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
        // D s_0_10: read-var sf:u8
        let s_0_10: bool = fn_state.sf;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // C s_0_12: const #1u : u8
        let s_0_12: bool = true;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b20 b1
        if s_0_14 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: write-var ga#254785 <= s_1_0
        fn_state.ga_254785 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#254785:i64
        let s_2_0: i64 = fn_state.ga_254785;
        // D s_2_1: write-var intsize <= s_2_0
        fn_state.intsize = s_2_0;
        // C s_2_2: const #16s : i64
        let s_2_2: i64 = 16;
        // D s_2_3: write-var fltsize <= s_2_2
        fn_state.fltsize = s_2_2;
        // D s_2_4: read-var ftype:u8
        let s_2_4: u8 = fn_state.ftype;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // C s_2_6: const #0u : u8
        let s_2_6: u8 = 0;
        // C s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 2u16);
        // D s_2_8: cmp-eq s_2_5 s_2_7
        let s_2_8: bool = ((s_2_5) == (s_2_7));
        // D s_2_9: not s_2_8
        let s_2_9: bool = !s_2_8;
        // N s_2_10: branch s_2_9 b15 b3
        if s_2_9 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #32s : i64
        let s_3_0: i64 = 32;
        // D s_3_1: write-var fltsize <= s_3_0
        fn_state.fltsize = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var fltsize:i64
        let s_4_0: i64 = fn_state.fltsize;
        // D s_4_1: write-var fltsizeshadow#1357 <= s_4_0
        fn_state.fltsizeshadow_1357 = s_4_0;
        // D s_4_2: read-var sf:u8
        let s_4_2: bool = fn_state.sf;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // C s_4_4: const #0u : u8
        let s_4_4: bool = false;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // D s_4_6: cmp-eq s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) == (s_4_5));
        // N s_4_7: branch s_4_6 b14 b5
        if s_4_6 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#151921 <= s_5_0
        fn_state.gs_151921 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#151921:u8
        let s_6_0: bool = fn_state.gs_151921;
        // N s_6_1: branch s_6_0 b13 b7
        if s_6_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var scale:u8
        let s_7_0: u8 = fn_state.scale;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 6u16);
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (s_7_1.value() as i128);
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #64s : i
        let s_7_4: i128 = 64;
        // D s_7_5: cast zx s_7_3 -> i
        let s_7_5: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_6: sub s_7_4 s_7_5
        let s_7_6: i128 = ((s_7_4) - (s_7_5));
        // D s_7_7: cast reint s_7_6 -> i64
        let s_7_7: i64 = (s_7_6 as i64);
        // D s_7_8: write-var fracbits <= s_7_7
        fn_state.fracbits = s_7_7;
        // C s_7_9: const #1s : i
        let s_7_9: i128 = 1;
        // D s_7_10: read-var opcode:u8
        let s_7_10: u8 = fn_state.opcode;
        // D s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 3u16);
        // C s_7_12: const #1s : i64
        let s_7_12: i64 = 1;
        // C s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // C s_7_14: const #1s : i
        let s_7_14: i128 = 1;
        // C s_7_15: add s_7_14 s_7_13
        let s_7_15: i128 = (s_7_14 + s_7_13);
        // D s_7_16: bit-extract s_7_11 s_7_9 s_7_15
        let s_7_16: Bits = (Bits::new(
            ((s_7_11) >> (s_7_9)).value(),
            u16::try_from(s_7_15).unwrap(),
        ));
        // D s_7_17: cast reint s_7_16 -> u8
        let s_7_17: u8 = (s_7_16.value() as u8);
        // D s_7_18: cast zx s_7_17 -> bv
        let s_7_18: Bits = Bits::new(s_7_17 as u128, 2u16);
        // D s_7_19: read-var rmode:u8
        let s_7_19: u8 = fn_state.rmode;
        // D s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 2u16);
        // D s_7_21: cast reint s_7_18 -> u128
        let s_7_21: u128 = (s_7_18.value() as u128);
        // D s_7_22: size-of s_7_18
        let s_7_22: u16 = s_7_18.length();
        // D s_7_23: cast reint s_7_20 -> u128
        let s_7_23: u128 = (s_7_20.value() as u128);
        // D s_7_24: size-of s_7_20
        let s_7_24: u16 = s_7_20.length();
        // D s_7_25: lsl s_7_21 s_7_24
        let s_7_25: u128 = s_7_21 << s_7_24;
        // D s_7_26: or s_7_25 s_7_23
        let s_7_26: u128 = ((s_7_25) | (s_7_23));
        // D s_7_27: add s_7_22 s_7_24
        let s_7_27: u16 = (s_7_22 + s_7_24);
        // D s_7_28: create-bits s_7_26 s_7_27
        let s_7_28: Bits = Bits::new(s_7_26, s_7_27);
        // D s_7_29: cast reint s_7_28 -> u8
        let s_7_29: u8 = (s_7_28.value() as u8);
        // D s_7_30: write-var ga#254795 <= s_7_29
        fn_state.ga_254795 = s_7_29;
        // D s_7_31: read-var ga#254795:u8
        let s_7_31: u8 = fn_state.ga_254795;
        // D s_7_32: cast zx s_7_31 -> bv
        let s_7_32: Bits = Bits::new(s_7_31 as u128, 4u16);
        // C s_7_33: const #3u : u8
        let s_7_33: u8 = 3;
        // C s_7_34: cast zx s_7_33 -> bv
        let s_7_34: Bits = Bits::new(s_7_33 as u128, 4u16);
        // D s_7_35: cmp-eq s_7_32 s_7_34
        let s_7_35: bool = ((s_7_32) == (s_7_34));
        // D s_7_36: not s_7_35
        let s_7_36: bool = !s_7_35;
        // N s_7_37: branch s_7_36 b10 b8
        if s_7_36 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #3u : u32
        let s_8_0: u32 = 3;
        // D s_8_1: write-var rounding <= s_8_0
        fn_state.rounding = s_8_0;
        // C s_8_2: const #0s : i
        let s_8_2: i128 = 0;
        // D s_8_3: read-var opcode:u8
        let s_8_3: u8 = fn_state.opcode;
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 3u16);
        // C s_8_5: const #1u : u64
        let s_8_5: u64 = 1;
        // D s_8_6: bit-extract s_8_4 s_8_2 s_8_5
        let s_8_6: Bits = (Bits::new(
            ((s_8_4) >> (s_8_2)).value(),
            u16::try_from(s_8_5).unwrap(),
        ));
        // D s_8_7: cast reint s_8_6 -> u8
        let s_8_7: bool = ((s_8_6.value()) != 0);
        // C s_8_8: const #0s : i
        let s_8_8: i128 = 0;
        // C s_8_9: const #0u : u64
        let s_8_9: u64 = 0;
        // D s_8_10: cast zx s_8_7 -> u64
        let s_8_10: u64 = (s_8_7 as u64);
        // C s_8_11: const #1u : u64
        let s_8_11: u64 = 1;
        // D s_8_12: and s_8_10 s_8_11
        let s_8_12: u64 = ((s_8_10) & (s_8_11));
        // D s_8_13: cmp-eq s_8_12 s_8_11
        let s_8_13: bool = ((s_8_12) == (s_8_11));
        // D s_8_14: lsl s_8_10 s_8_8
        let s_8_14: u64 = s_8_10 << s_8_8;
        // D s_8_15: or s_8_9 s_8_14
        let s_8_15: u64 = ((s_8_9) | (s_8_14));
        // D s_8_16: cmpl s_8_14
        let s_8_16: u64 = !s_8_14;
        // D s_8_17: and s_8_9 s_8_16
        let s_8_17: u64 = ((s_8_9) & (s_8_16));
        // D s_8_18: select s_8_13 s_8_15 s_8_17
        let s_8_18: u64 = if s_8_13 { s_8_15 } else { s_8_17 };
        // D s_8_19: cast trunc s_8_18 -> u8
        let s_8_19: bool = ((s_8_18) != 0);
        // D s_8_20: cast zx s_8_19 -> bv
        let s_8_20: Bits = Bits::new(s_8_19 as u128, 1u16);
        // C s_8_21: const #1u : u8
        let s_8_21: bool = true;
        // C s_8_22: cast zx s_8_21 -> bv
        let s_8_22: Bits = Bits::new(s_8_21 as u128, 1u16);
        // D s_8_23: cmp-eq s_8_20 s_8_22
        let s_8_23: bool = ((s_8_20) == (s_8_22));
        // D s_8_24: write-var is_unsigned <= s_8_23
        fn_state.is_unsigned = s_8_23;
        // C s_8_25: const #0u : u32
        let s_8_25: u32 = 0;
        // D s_8_26: write-var op <= s_8_25
        fn_state.op = s_8_25;
        // N s_8_27: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var fltsizeshadow#1357:i64
        let s_9_0: i64 = fn_state.fltsizeshadow_1357;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var intsize:i64
        let s_9_3: i64 = fn_state.intsize;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // D s_9_6: read-var d:i64
        let s_9_6: i64 = fn_state.d;
        // D s_9_7: read-var fracbits:i64
        let s_9_7: i64 = fn_state.fracbits;
        // D s_9_8: read-var n:i64
        let s_9_8: i64 = fn_state.n;
        // D s_9_9: read-var op:u32
        let s_9_9: u32 = fn_state.op;
        // D s_9_10: read-var rounding:u32
        let s_9_10: u32 = fn_state.rounding;
        // D s_9_11: read-var is_unsigned:u8
        let s_9_11: bool = fn_state.is_unsigned;
        // D s_9_12: call execute_aarch64_instrs_float_convert_fix(s_9_6, s_9_2, s_9_7, s_9_5, s_9_8, s_9_9, s_9_10, s_9_11)
        let s_9_12: () = execute_aarch64_instrs_float_convert_fix(
            state,
            tracer,
            s_9_6,
            s_9_2,
            s_9_7,
            s_9_5,
            s_9_8,
            s_9_9,
            s_9_10,
            s_9_11,
        );
        // N s_9_13: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#254795:u8
        let s_10_0: u8 = fn_state.ga_254795;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 4u16);
        // C s_10_2: const #4u : u8
        let s_10_2: u8 = 4;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 4u16);
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
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call FPCR_read(s_11_0)
        let s_11_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_11_0);
        // S s_11_2: call FPRoundingMode(s_11_1)
        let s_11_2: u32 = FPRoundingMode(state, tracer, s_11_1);
        // D s_11_3: write-var rounding <= s_11_2
        fn_state.rounding = s_11_2;
        // C s_11_4: const #0s : i
        let s_11_4: i128 = 0;
        // D s_11_5: read-var opcode:u8
        let s_11_5: u8 = fn_state.opcode;
        // D s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 3u16);
        // C s_11_7: const #1u : u64
        let s_11_7: u64 = 1;
        // D s_11_8: bit-extract s_11_6 s_11_4 s_11_7
        let s_11_8: Bits = (Bits::new(
            ((s_11_6) >> (s_11_4)).value(),
            u16::try_from(s_11_7).unwrap(),
        ));
        // D s_11_9: cast reint s_11_8 -> u8
        let s_11_9: bool = ((s_11_8.value()) != 0);
        // C s_11_10: const #0s : i
        let s_11_10: i128 = 0;
        // C s_11_11: const #0u : u64
        let s_11_11: u64 = 0;
        // D s_11_12: cast zx s_11_9 -> u64
        let s_11_12: u64 = (s_11_9 as u64);
        // C s_11_13: const #1u : u64
        let s_11_13: u64 = 1;
        // D s_11_14: and s_11_12 s_11_13
        let s_11_14: u64 = ((s_11_12) & (s_11_13));
        // D s_11_15: cmp-eq s_11_14 s_11_13
        let s_11_15: bool = ((s_11_14) == (s_11_13));
        // D s_11_16: lsl s_11_12 s_11_10
        let s_11_16: u64 = s_11_12 << s_11_10;
        // D s_11_17: or s_11_11 s_11_16
        let s_11_17: u64 = ((s_11_11) | (s_11_16));
        // D s_11_18: cmpl s_11_16
        let s_11_18: u64 = !s_11_16;
        // D s_11_19: and s_11_11 s_11_18
        let s_11_19: u64 = ((s_11_11) & (s_11_18));
        // D s_11_20: select s_11_15 s_11_17 s_11_19
        let s_11_20: u64 = if s_11_15 { s_11_17 } else { s_11_19 };
        // D s_11_21: cast trunc s_11_20 -> u8
        let s_11_21: bool = ((s_11_20) != 0);
        // D s_11_22: cast zx s_11_21 -> bv
        let s_11_22: Bits = Bits::new(s_11_21 as u128, 1u16);
        // C s_11_23: const #1u : u8
        let s_11_23: bool = true;
        // C s_11_24: cast zx s_11_23 -> bv
        let s_11_24: Bits = Bits::new(s_11_23 as u128, 1u16);
        // D s_11_25: cmp-eq s_11_22 s_11_24
        let s_11_25: bool = ((s_11_22) == (s_11_24));
        // D s_11_26: write-var is_unsigned <= s_11_25
        fn_state.is_unsigned = s_11_25;
        // C s_11_27: const #1u : u32
        let s_11_27: u32 = 1;
        // D s_11_28: write-var op <= s_11_27
        fn_state.op = s_11_27;
        // N s_11_29: jump b9
        return block_9(state, tracer, fn_state);
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
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #5s : i
        let s_14_0: i128 = 5;
        // D s_14_1: read-var scale:u8
        let s_14_1: u8 = fn_state.scale;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 6u16);
        // C s_14_3: const #1u : u64
        let s_14_3: u64 = 1;
        // D s_14_4: bit-extract s_14_2 s_14_0 s_14_3
        let s_14_4: Bits = (Bits::new(
            ((s_14_2) >> (s_14_0)).value(),
            u16::try_from(s_14_3).unwrap(),
        ));
        // D s_14_5: cast reint s_14_4 -> u8
        let s_14_5: bool = ((s_14_4.value()) != 0);
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // C s_14_7: const #0u : u64
        let s_14_7: u64 = 0;
        // D s_14_8: cast zx s_14_5 -> u64
        let s_14_8: u64 = (s_14_5 as u64);
        // C s_14_9: const #1u : u64
        let s_14_9: u64 = 1;
        // D s_14_10: and s_14_8 s_14_9
        let s_14_10: u64 = ((s_14_8) & (s_14_9));
        // D s_14_11: cmp-eq s_14_10 s_14_9
        let s_14_11: bool = ((s_14_10) == (s_14_9));
        // D s_14_12: lsl s_14_8 s_14_6
        let s_14_12: u64 = s_14_8 << s_14_6;
        // D s_14_13: or s_14_7 s_14_12
        let s_14_13: u64 = ((s_14_7) | (s_14_12));
        // D s_14_14: cmpl s_14_12
        let s_14_14: u64 = !s_14_12;
        // D s_14_15: and s_14_7 s_14_14
        let s_14_15: u64 = ((s_14_7) & (s_14_14));
        // D s_14_16: select s_14_11 s_14_13 s_14_15
        let s_14_16: u64 = if s_14_11 { s_14_13 } else { s_14_15 };
        // D s_14_17: cast trunc s_14_16 -> u8
        let s_14_17: bool = ((s_14_16) != 0);
        // D s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 1u16);
        // C s_14_19: const #0u : u8
        let s_14_19: bool = false;
        // C s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 1u16);
        // D s_14_21: cmp-eq s_14_18 s_14_20
        let s_14_21: bool = ((s_14_18) == (s_14_20));
        // D s_14_22: write-var gs#151921 <= s_14_21
        fn_state.gs_151921 = s_14_21;
        // N s_14_23: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ftype:u8
        let s_15_0: u8 = fn_state.ftype;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 2u16);
        // C s_15_2: const #1u : u8
        let s_15_2: u8 = 1;
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
        // C s_16_0: const #64s : i64
        let s_16_0: i64 = 64;
        // D s_16_1: write-var fltsize <= s_16_0
        fn_state.fltsize = s_16_0;
        // N s_16_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var ftype:u8
        let s_17_0: u8 = fn_state.ftype;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
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
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #16s : i64
        let s_19_0: i64 = 16;
        // D s_19_1: write-var fltsize <= s_19_0
        fn_state.fltsize = s_19_0;
        // N s_19_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #64s : i64
        let s_20_0: i64 = 64;
        // D s_20_1: write-var ga#254785 <= s_20_0
        fn_state.ga_254785 = s_20_0;
        // N s_20_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
