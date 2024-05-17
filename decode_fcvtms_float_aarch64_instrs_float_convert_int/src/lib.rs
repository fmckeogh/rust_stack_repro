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
use FPRoundingMode::*;
use neq_int::*;
use FPDecodeRounding::*;
use FPCR_read::*;
use HaveFJCVTZSExt::*;
use execute_aarch64_instrs_float_convert_int::*;
use common::*;
pub fn decode_fcvtms_float_aarch64_instrs_float_convert_int<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    opcode: u8,
    rmode: u8,
    ftype: u8,
    sf: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_150341: bool,
        part: i64,
        n: i64,
        ga_253875: u8,
        opshadow_1311: u32,
        gs_150350: bool,
        ga_253869: i64,
        op: u32,
        fltsize: i64,
        d: i64,
        intsize: i64,
        is_unsigned: bool,
        rounding: u32,
        partshadow_1312: i64,
        fltsizeshadow_1313: i64,
        Rd: u8,
        Rn: u8,
        opcode: u8,
        rmode: u8,
        ftype: u8,
        sf: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
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
        // N s_0_15: branch s_0_14 b51 b1
        if s_0_14 {
            return block_51(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#253869 <= s_1_0
        fn_state.ga_253869 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#253869:i64
        let s_2_0: i64 = fn_state.ga_253869;
        // D s_2_1: write-var intsize <= s_2_0
        fn_state.intsize = s_2_0;
        // C s_2_2: const #16s : i64
        let s_2_2: i64 = 16;
        // D s_2_3: write-var fltsize <= s_2_2
        fn_state.fltsize = s_2_2;
        // C s_2_4: const #0s : i64
        let s_2_4: i64 = 0;
        // D s_2_5: write-var part <= s_2_4
        fn_state.part = s_2_4;
        // D s_2_6: read-var ftype:u8
        let s_2_6: u8 = fn_state.ftype;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 2u16);
        // C s_2_8: const #0u : u8
        let s_2_8: u8 = 0;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 2u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // D s_2_11: not s_2_10
        let s_2_11: bool = !s_2_10;
        // N s_2_12: branch s_2_11 b44 b3
        if s_2_11 {
            return block_44(state, tracer, fn_state);
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
        // C s_4_0: const #1s : i
        let s_4_0: i128 = 1;
        // D s_4_1: read-var opcode:u8
        let s_4_1: u8 = fn_state.opcode;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 3u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #1s : i
        let s_4_5: i128 = 1;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_0 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: u8 = (s_4_7.value() as u8);
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 2u16);
        // D s_4_10: read-var rmode:u8
        let s_4_10: u8 = fn_state.rmode;
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 2u16);
        // D s_4_12: cast reint s_4_9 -> u128
        let s_4_12: u128 = (s_4_9.value() as u128);
        // D s_4_13: size-of s_4_9
        let s_4_13: u16 = s_4_9.length();
        // D s_4_14: cast reint s_4_11 -> u128
        let s_4_14: u128 = (s_4_11.value() as u128);
        // D s_4_15: size-of s_4_11
        let s_4_15: u16 = s_4_11.length();
        // D s_4_16: lsl s_4_12 s_4_15
        let s_4_16: u128 = s_4_12 << s_4_15;
        // D s_4_17: or s_4_16 s_4_14
        let s_4_17: u128 = ((s_4_16) | (s_4_14));
        // D s_4_18: add s_4_13 s_4_15
        let s_4_18: u16 = (s_4_13 + s_4_15);
        // D s_4_19: create-bits s_4_17 s_4_18
        let s_4_19: Bits = Bits::new(s_4_17, s_4_18);
        // D s_4_20: cast reint s_4_19 -> u8
        let s_4_20: u8 = (s_4_19.value() as u8);
        // D s_4_21: write-var ga#253875 <= s_4_20
        fn_state.ga_253875 = s_4_20;
        // D s_4_22: read-var ga#253875:u8
        let s_4_22: u8 = fn_state.ga_253875;
        // C s_4_23: const #2s : i
        let s_4_23: i128 = 2;
        // D s_4_24: cast zx s_4_22 -> bv
        let s_4_24: Bits = Bits::new(s_4_22 as u128, 4u16);
        // C s_4_25: const #1s : i64
        let s_4_25: i64 = 1;
        // C s_4_26: cast zx s_4_25 -> i
        let s_4_26: i128 = (i128::try_from(s_4_25).unwrap());
        // C s_4_27: const #1s : i
        let s_4_27: i128 = 1;
        // C s_4_28: add s_4_27 s_4_26
        let s_4_28: i128 = (s_4_27 + s_4_26);
        // D s_4_29: bit-extract s_4_24 s_4_23 s_4_28
        let s_4_29: Bits = (Bits::new(
            ((s_4_24) >> (s_4_23)).value(),
            u16::try_from(s_4_28).unwrap(),
        ));
        // D s_4_30: cast reint s_4_29 -> u8
        let s_4_30: u8 = (s_4_29.value() as u8);
        // D s_4_31: cast zx s_4_30 -> bv
        let s_4_31: Bits = Bits::new(s_4_30 as u128, 2u16);
        // C s_4_32: const #0u : u8
        let s_4_32: u8 = 0;
        // C s_4_33: cast zx s_4_32 -> bv
        let s_4_33: Bits = Bits::new(s_4_32 as u128, 2u16);
        // D s_4_34: cmp-eq s_4_31 s_4_33
        let s_4_34: bool = ((s_4_31) == (s_4_33));
        // D s_4_35: not s_4_34
        let s_4_35: bool = !s_4_34;
        // N s_4_36: branch s_4_35 b15 b5
        if s_4_35 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var rmode:u8
        let s_5_0: u8 = fn_state.rmode;
        // D s_5_1: call FPDecodeRounding(s_5_0)
        let s_5_1: u32 = FPDecodeRounding(state, tracer, s_5_0);
        // D s_5_2: write-var rounding <= s_5_1
        fn_state.rounding = s_5_1;
        // C s_5_3: const #0s : i
        let s_5_3: i128 = 0;
        // D s_5_4: read-var opcode:u8
        let s_5_4: u8 = fn_state.opcode;
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 3u16);
        // C s_5_6: const #1u : u64
        let s_5_6: u64 = 1;
        // D s_5_7: bit-extract s_5_5 s_5_3 s_5_6
        let s_5_7: Bits = (Bits::new(
            ((s_5_5) >> (s_5_3)).value(),
            u16::try_from(s_5_6).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: bool = ((s_5_7.value()) != 0);
        // C s_5_9: const #0s : i
        let s_5_9: i128 = 0;
        // C s_5_10: const #0u : u64
        let s_5_10: u64 = 0;
        // D s_5_11: cast zx s_5_8 -> u64
        let s_5_11: u64 = (s_5_8 as u64);
        // C s_5_12: const #1u : u64
        let s_5_12: u64 = 1;
        // D s_5_13: and s_5_11 s_5_12
        let s_5_13: u64 = ((s_5_11) & (s_5_12));
        // D s_5_14: cmp-eq s_5_13 s_5_12
        let s_5_14: bool = ((s_5_13) == (s_5_12));
        // D s_5_15: lsl s_5_11 s_5_9
        let s_5_15: u64 = s_5_11 << s_5_9;
        // D s_5_16: or s_5_10 s_5_15
        let s_5_16: u64 = ((s_5_10) | (s_5_15));
        // D s_5_17: cmpl s_5_15
        let s_5_17: u64 = !s_5_15;
        // D s_5_18: and s_5_10 s_5_17
        let s_5_18: u64 = ((s_5_10) & (s_5_17));
        // D s_5_19: select s_5_14 s_5_16 s_5_18
        let s_5_19: u64 = if s_5_14 { s_5_16 } else { s_5_18 };
        // D s_5_20: cast trunc s_5_19 -> u8
        let s_5_20: bool = ((s_5_19) != 0);
        // D s_5_21: cast zx s_5_20 -> bv
        let s_5_21: Bits = Bits::new(s_5_20 as u128, 1u16);
        // C s_5_22: const #1u : u8
        let s_5_22: bool = true;
        // C s_5_23: cast zx s_5_22 -> bv
        let s_5_23: Bits = Bits::new(s_5_22 as u128, 1u16);
        // D s_5_24: cmp-eq s_5_21 s_5_23
        let s_5_24: bool = ((s_5_21) == (s_5_23));
        // D s_5_25: write-var is_unsigned <= s_5_24
        fn_state.is_unsigned = s_5_24;
        // C s_5_26: const #0u : u32
        let s_5_26: u32 = 0;
        // D s_5_27: write-var op <= s_5_26
        fn_state.op = s_5_26;
        // N s_5_28: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var op:u32
        let s_6_0: u32 = fn_state.op;
        // D s_6_1: write-var opshadow#1311 <= s_6_0
        fn_state.opshadow_1311 = s_6_0;
        // D s_6_2: read-var part:i64
        let s_6_2: i64 = fn_state.part;
        // D s_6_3: write-var partshadow#1312 <= s_6_2
        fn_state.partshadow_1312 = s_6_2;
        // D s_6_4: read-var fltsize:i64
        let s_6_4: i64 = fn_state.fltsize;
        // D s_6_5: write-var fltsizeshadow#1313 <= s_6_4
        fn_state.fltsizeshadow_1313 = s_6_4;
        // D s_6_6: read-var fltsizeshadow#1313:i64
        let s_6_6: i64 = fn_state.fltsizeshadow_1313;
        // C s_6_7: const #16s : i
        let s_6_7: i128 = 16;
        // D s_6_8: cast zx s_6_6 -> i
        let s_6_8: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_9: cmp-eq s_6_8 s_6_7
        let s_6_9: bool = ((s_6_8) == (s_6_7));
        // D s_6_10: not s_6_9
        let s_6_10: bool = !s_6_9;
        // N s_6_11: branch s_6_10 b8 b7
        if s_6_10 {
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
        // C s_7_0: const #16s : i64
        let s_7_0: i64 = 16;
        // D s_7_1: read-var intsize:i64
        let s_7_1: i64 = fn_state.intsize;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // D s_7_4: read-var d:i64
        let s_7_4: i64 = fn_state.d;
        // D s_7_5: read-var n:i64
        let s_7_5: i64 = fn_state.n;
        // D s_7_6: read-var opshadow#1311:u32
        let s_7_6: u32 = fn_state.opshadow_1311;
        // D s_7_7: read-var partshadow#1312:i64
        let s_7_7: i64 = fn_state.partshadow_1312;
        // D s_7_8: read-var rounding:u32
        let s_7_8: u32 = fn_state.rounding;
        // D s_7_9: read-var is_unsigned:u8
        let s_7_9: bool = fn_state.is_unsigned;
        // D s_7_10: call execute_aarch64_instrs_float_convert_int(s_7_4, s_7_0, s_7_3, s_7_5, s_7_6, s_7_7, s_7_8, s_7_9)
        let s_7_10: () = execute_aarch64_instrs_float_convert_int(
            state,
            tracer,
            s_7_4,
            s_7_0,
            s_7_3,
            s_7_5,
            s_7_6,
            s_7_7,
            s_7_8,
            s_7_9,
        );
        // N s_7_11: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var fltsizeshadow#1313:i64
        let s_8_0: i64 = fn_state.fltsizeshadow_1313;
        // C s_8_1: const #32s : i
        let s_8_1: i128 = 32;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b10 b9
        if s_8_4 {
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
        // C s_9_0: const #32s : i64
        let s_9_0: i64 = 32;
        // D s_9_1: read-var intsize:i64
        let s_9_1: i64 = fn_state.intsize;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: read-var d:i64
        let s_9_4: i64 = fn_state.d;
        // D s_9_5: read-var n:i64
        let s_9_5: i64 = fn_state.n;
        // D s_9_6: read-var opshadow#1311:u32
        let s_9_6: u32 = fn_state.opshadow_1311;
        // D s_9_7: read-var partshadow#1312:i64
        let s_9_7: i64 = fn_state.partshadow_1312;
        // D s_9_8: read-var rounding:u32
        let s_9_8: u32 = fn_state.rounding;
        // D s_9_9: read-var is_unsigned:u8
        let s_9_9: bool = fn_state.is_unsigned;
        // D s_9_10: call execute_aarch64_instrs_float_convert_int(s_9_4, s_9_0, s_9_3, s_9_5, s_9_6, s_9_7, s_9_8, s_9_9)
        let s_9_10: () = execute_aarch64_instrs_float_convert_int(
            state,
            tracer,
            s_9_4,
            s_9_0,
            s_9_3,
            s_9_5,
            s_9_6,
            s_9_7,
            s_9_8,
            s_9_9,
        );
        // N s_9_11: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var fltsizeshadow#1313:i64
        let s_10_0: i64 = fn_state.fltsizeshadow_1313;
        // C s_10_1: const #64s : i
        let s_10_1: i128 = 64;
        // D s_10_2: cast zx s_10_0 -> i
        let s_10_2: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_1
        let s_10_3: bool = ((s_10_2) == (s_10_1));
        // D s_10_4: not s_10_3
        let s_10_4: bool = !s_10_3;
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
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
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // D s_11_1: read-var intsize:i64
        let s_11_1: i64 = fn_state.intsize;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // D s_11_4: read-var d:i64
        let s_11_4: i64 = fn_state.d;
        // D s_11_5: read-var n:i64
        let s_11_5: i64 = fn_state.n;
        // D s_11_6: read-var opshadow#1311:u32
        let s_11_6: u32 = fn_state.opshadow_1311;
        // D s_11_7: read-var partshadow#1312:i64
        let s_11_7: i64 = fn_state.partshadow_1312;
        // D s_11_8: read-var rounding:u32
        let s_11_8: u32 = fn_state.rounding;
        // D s_11_9: read-var is_unsigned:u8
        let s_11_9: bool = fn_state.is_unsigned;
        // D s_11_10: call execute_aarch64_instrs_float_convert_int(s_11_4, s_11_0, s_11_3, s_11_5, s_11_6, s_11_7, s_11_8, s_11_9)
        let s_11_10: () = execute_aarch64_instrs_float_convert_int(
            state,
            tracer,
            s_11_4,
            s_11_0,
            s_11_3,
            s_11_5,
            s_11_6,
            s_11_7,
            s_11_8,
            s_11_9,
        );
        // N s_11_11: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var fltsizeshadow#1313:i64
        let s_12_0: i64 = fn_state.fltsizeshadow_1313;
        // C s_12_1: const #128s : i
        let s_12_1: i128 = 128;
        // D s_12_2: cast zx s_12_0 -> i
        let s_12_2: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_3: cmp-eq s_12_2 s_12_1
        let s_12_3: bool = ((s_12_2) == (s_12_1));
        // D s_12_4: not s_12_3
        let s_12_4: bool = !s_12_3;
        // N s_12_5: branch s_12_4 b14 b13
        if s_12_4 {
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
        // C s_13_0: const #128s : i64
        let s_13_0: i64 = 128;
        // D s_13_1: read-var intsize:i64
        let s_13_1: i64 = fn_state.intsize;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // D s_13_4: read-var d:i64
        let s_13_4: i64 = fn_state.d;
        // D s_13_5: read-var n:i64
        let s_13_5: i64 = fn_state.n;
        // D s_13_6: read-var opshadow#1311:u32
        let s_13_6: u32 = fn_state.opshadow_1311;
        // D s_13_7: read-var partshadow#1312:i64
        let s_13_7: i64 = fn_state.partshadow_1312;
        // D s_13_8: read-var rounding:u32
        let s_13_8: u32 = fn_state.rounding;
        // D s_13_9: read-var is_unsigned:u8
        let s_13_9: bool = fn_state.is_unsigned;
        // D s_13_10: call execute_aarch64_instrs_float_convert_int(s_13_4, s_13_0, s_13_3, s_13_5, s_13_6, s_13_7, s_13_8, s_13_9)
        let s_13_10: () = execute_aarch64_instrs_float_convert_int(
            state,
            tracer,
            s_13_4,
            s_13_0,
            s_13_3,
            s_13_5,
            s_13_6,
            s_13_7,
            s_13_8,
            s_13_9,
        );
        // N s_13_11: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // N s_14_2: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ga#253875:u8
        let s_15_0: u8 = fn_state.ga_253875;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 4u16);
        // C s_15_2: const #4u : u8
        let s_15_2: u8 = 4;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 4u16);
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
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call FPCR_read(s_16_0)
        let s_16_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_16_0);
        // S s_16_2: call FPRoundingMode(s_16_1)
        let s_16_2: u32 = FPRoundingMode(state, tracer, s_16_1);
        // D s_16_3: write-var rounding <= s_16_2
        fn_state.rounding = s_16_2;
        // C s_16_4: const #0s : i
        let s_16_4: i128 = 0;
        // D s_16_5: read-var opcode:u8
        let s_16_5: u8 = fn_state.opcode;
        // D s_16_6: cast zx s_16_5 -> bv
        let s_16_6: Bits = Bits::new(s_16_5 as u128, 3u16);
        // C s_16_7: const #1u : u64
        let s_16_7: u64 = 1;
        // D s_16_8: bit-extract s_16_6 s_16_4 s_16_7
        let s_16_8: Bits = (Bits::new(
            ((s_16_6) >> (s_16_4)).value(),
            u16::try_from(s_16_7).unwrap(),
        ));
        // D s_16_9: cast reint s_16_8 -> u8
        let s_16_9: bool = ((s_16_8.value()) != 0);
        // C s_16_10: const #0s : i
        let s_16_10: i128 = 0;
        // C s_16_11: const #0u : u64
        let s_16_11: u64 = 0;
        // D s_16_12: cast zx s_16_9 -> u64
        let s_16_12: u64 = (s_16_9 as u64);
        // C s_16_13: const #1u : u64
        let s_16_13: u64 = 1;
        // D s_16_14: and s_16_12 s_16_13
        let s_16_14: u64 = ((s_16_12) & (s_16_13));
        // D s_16_15: cmp-eq s_16_14 s_16_13
        let s_16_15: bool = ((s_16_14) == (s_16_13));
        // D s_16_16: lsl s_16_12 s_16_10
        let s_16_16: u64 = s_16_12 << s_16_10;
        // D s_16_17: or s_16_11 s_16_16
        let s_16_17: u64 = ((s_16_11) | (s_16_16));
        // D s_16_18: cmpl s_16_16
        let s_16_18: u64 = !s_16_16;
        // D s_16_19: and s_16_11 s_16_18
        let s_16_19: u64 = ((s_16_11) & (s_16_18));
        // D s_16_20: select s_16_15 s_16_17 s_16_19
        let s_16_20: u64 = if s_16_15 { s_16_17 } else { s_16_19 };
        // D s_16_21: cast trunc s_16_20 -> u8
        let s_16_21: bool = ((s_16_20) != 0);
        // D s_16_22: cast zx s_16_21 -> bv
        let s_16_22: Bits = Bits::new(s_16_21 as u128, 1u16);
        // C s_16_23: const #1u : u8
        let s_16_23: bool = true;
        // C s_16_24: cast zx s_16_23 -> bv
        let s_16_24: Bits = Bits::new(s_16_23 as u128, 1u16);
        // D s_16_25: cmp-eq s_16_22 s_16_24
        let s_16_25: bool = ((s_16_22) == (s_16_24));
        // D s_16_26: write-var is_unsigned <= s_16_25
        fn_state.is_unsigned = s_16_25;
        // C s_16_27: const #1u : u32
        let s_16_27: u32 = 1;
        // D s_16_28: write-var op <= s_16_27
        fn_state.op = s_16_27;
        // N s_16_29: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var ga#253875:u8
        let s_17_0: u8 = fn_state.ga_253875;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 4u16);
        // C s_17_2: const #8u : u8
        let s_17_2: u8 = 8;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 4u16);
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
        // C s_18_0: const #4u : u32
        let s_18_0: u32 = 4;
        // D s_18_1: write-var rounding <= s_18_0
        fn_state.rounding = s_18_0;
        // C s_18_2: const #0s : i
        let s_18_2: i128 = 0;
        // D s_18_3: read-var opcode:u8
        let s_18_3: u8 = fn_state.opcode;
        // D s_18_4: cast zx s_18_3 -> bv
        let s_18_4: Bits = Bits::new(s_18_3 as u128, 3u16);
        // C s_18_5: const #1u : u64
        let s_18_5: u64 = 1;
        // D s_18_6: bit-extract s_18_4 s_18_2 s_18_5
        let s_18_6: Bits = (Bits::new(
            ((s_18_4) >> (s_18_2)).value(),
            u16::try_from(s_18_5).unwrap(),
        ));
        // D s_18_7: cast reint s_18_6 -> u8
        let s_18_7: bool = ((s_18_6.value()) != 0);
        // C s_18_8: const #0s : i
        let s_18_8: i128 = 0;
        // C s_18_9: const #0u : u64
        let s_18_9: u64 = 0;
        // D s_18_10: cast zx s_18_7 -> u64
        let s_18_10: u64 = (s_18_7 as u64);
        // C s_18_11: const #1u : u64
        let s_18_11: u64 = 1;
        // D s_18_12: and s_18_10 s_18_11
        let s_18_12: u64 = ((s_18_10) & (s_18_11));
        // D s_18_13: cmp-eq s_18_12 s_18_11
        let s_18_13: bool = ((s_18_12) == (s_18_11));
        // D s_18_14: lsl s_18_10 s_18_8
        let s_18_14: u64 = s_18_10 << s_18_8;
        // D s_18_15: or s_18_9 s_18_14
        let s_18_15: u64 = ((s_18_9) | (s_18_14));
        // D s_18_16: cmpl s_18_14
        let s_18_16: u64 = !s_18_14;
        // D s_18_17: and s_18_9 s_18_16
        let s_18_17: u64 = ((s_18_9) & (s_18_16));
        // D s_18_18: select s_18_13 s_18_15 s_18_17
        let s_18_18: u64 = if s_18_13 { s_18_15 } else { s_18_17 };
        // D s_18_19: cast trunc s_18_18 -> u8
        let s_18_19: bool = ((s_18_18) != 0);
        // D s_18_20: cast zx s_18_19 -> bv
        let s_18_20: Bits = Bits::new(s_18_19 as u128, 1u16);
        // C s_18_21: const #1u : u8
        let s_18_21: bool = true;
        // C s_18_22: cast zx s_18_21 -> bv
        let s_18_22: Bits = Bits::new(s_18_21 as u128, 1u16);
        // D s_18_23: cmp-eq s_18_20 s_18_22
        let s_18_23: bool = ((s_18_20) == (s_18_22));
        // D s_18_24: write-var is_unsigned <= s_18_23
        fn_state.is_unsigned = s_18_23;
        // C s_18_25: const #0u : u32
        let s_18_25: u32 = 0;
        // D s_18_26: write-var op <= s_18_25
        fn_state.op = s_18_25;
        // N s_18_27: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var ga#253875:u8
        let s_19_0: u8 = fn_state.ga_253875;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 4u16);
        // C s_19_2: const #12u : u8
        let s_19_2: u8 = 12;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 4u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: not s_19_4
        let s_19_5: bool = !s_19_4;
        // N s_19_6: branch s_19_5 b29 b20
        if s_19_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #16s : i
        let s_20_0: i128 = 16;
        // D s_20_1: read-var fltsize:i64
        let s_20_1: i64 = fn_state.fltsize;
        // D s_20_2: cast zx s_20_1 -> i
        let s_20_2: i128 = (i128::try_from(s_20_1).unwrap());
        // D s_20_3: call neq_int(s_20_2, s_20_0)
        let s_20_3: bool = neq_int(state, tracer, s_20_2, s_20_0);
        // N s_20_4: branch s_20_3 b28 b21
        if s_20_3 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#150341 <= s_21_0
        fn_state.gs_150341 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#150341:u8
        let s_22_0: bool = fn_state.gs_150341;
        // N s_22_1: branch s_22_0 b27 b23
        if s_22_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0s : i
        let s_23_0: i128 = 0;
        // D s_23_1: read-var opcode:u8
        let s_23_1: u8 = fn_state.opcode;
        // D s_23_2: cast zx s_23_1 -> bv
        let s_23_2: Bits = Bits::new(s_23_1 as u128, 3u16);
        // C s_23_3: const #1u : u64
        let s_23_3: u64 = 1;
        // D s_23_4: bit-extract s_23_2 s_23_0 s_23_3
        let s_23_4: Bits = (Bits::new(
            ((s_23_2) >> (s_23_0)).value(),
            u16::try_from(s_23_3).unwrap(),
        ));
        // D s_23_5: cast reint s_23_4 -> u8
        let s_23_5: bool = ((s_23_4.value()) != 0);
        // C s_23_6: const #0s : i
        let s_23_6: i128 = 0;
        // C s_23_7: const #0u : u64
        let s_23_7: u64 = 0;
        // D s_23_8: cast zx s_23_5 -> u64
        let s_23_8: u64 = (s_23_5 as u64);
        // C s_23_9: const #1u : u64
        let s_23_9: u64 = 1;
        // D s_23_10: and s_23_8 s_23_9
        let s_23_10: u64 = ((s_23_8) & (s_23_9));
        // D s_23_11: cmp-eq s_23_10 s_23_9
        let s_23_11: bool = ((s_23_10) == (s_23_9));
        // D s_23_12: lsl s_23_8 s_23_6
        let s_23_12: u64 = s_23_8 << s_23_6;
        // D s_23_13: or s_23_7 s_23_12
        let s_23_13: u64 = ((s_23_7) | (s_23_12));
        // D s_23_14: cmpl s_23_12
        let s_23_14: u64 = !s_23_12;
        // D s_23_15: and s_23_7 s_23_14
        let s_23_15: u64 = ((s_23_7) & (s_23_14));
        // D s_23_16: select s_23_11 s_23_13 s_23_15
        let s_23_16: u64 = if s_23_11 { s_23_13 } else { s_23_15 };
        // D s_23_17: cast trunc s_23_16 -> u8
        let s_23_17: bool = ((s_23_16) != 0);
        // D s_23_18: cast zx s_23_17 -> bv
        let s_23_18: Bits = Bits::new(s_23_17 as u128, 1u16);
        // C s_23_19: const #1u : u8
        let s_23_19: bool = true;
        // C s_23_20: cast zx s_23_19 -> bv
        let s_23_20: Bits = Bits::new(s_23_19 as u128, 1u16);
        // D s_23_21: cmp-eq s_23_18 s_23_20
        let s_23_21: bool = ((s_23_18) == (s_23_20));
        // N s_23_22: branch s_23_21 b26 b24
        if s_23_21 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #2u : u32
        let s_24_0: u32 = 2;
        // D s_24_1: write-var op <= s_24_0
        fn_state.op = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0s : i64
        let s_25_0: i64 = 0;
        // D s_25_1: write-var part <= s_25_0
        fn_state.part = s_25_0;
        // N s_25_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #3u : u32
        let s_26_0: u32 = 3;
        // D s_26_1: write-var op <= s_26_0
        fn_state.op = s_26_0;
        // N s_26_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var fltsize:i64
        let s_28_0: i64 = fn_state.fltsize;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: read-var intsize:i64
        let s_28_2: i64 = fn_state.intsize;
        // D s_28_3: cast zx s_28_2 -> i
        let s_28_3: i128 = (i128::try_from(s_28_2).unwrap());
        // D s_28_4: call neq_int(s_28_1, s_28_3)
        let s_28_4: bool = neq_int(state, tracer, s_28_1, s_28_3);
        // D s_28_5: write-var gs#150341 <= s_28_4
        fn_state.gs_150341 = s_28_4;
        // N s_28_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var ga#253875:u8
        let s_29_0: u8 = fn_state.ga_253875;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 4u16);
        // C s_29_2: const #13u : u8
        let s_29_2: u8 = 13;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 4u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: not s_29_4
        let s_29_5: bool = !s_29_4;
        // N s_29_6: branch s_29_5 b39 b30
        if s_29_5 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #64s : i
        let s_30_0: i128 = 64;
        // D s_30_1: read-var intsize:i64
        let s_30_1: i64 = fn_state.intsize;
        // D s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (i128::try_from(s_30_1).unwrap());
        // D s_30_3: call neq_int(s_30_2, s_30_0)
        let s_30_3: bool = neq_int(state, tracer, s_30_2, s_30_0);
        // N s_30_4: branch s_30_3 b38 b31
        if s_30_3 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #128s : i
        let s_31_0: i128 = 128;
        // D s_31_1: read-var fltsize:i64
        let s_31_1: i64 = fn_state.fltsize;
        // D s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (i128::try_from(s_31_1).unwrap());
        // D s_31_3: call neq_int(s_31_2, s_31_0)
        let s_31_3: bool = neq_int(state, tracer, s_31_2, s_31_0);
        // D s_31_4: write-var gs#150350 <= s_31_3
        fn_state.gs_150350 = s_31_3;
        // N s_31_5: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#150350:u8
        let s_32_0: bool = fn_state.gs_150350;
        // N s_32_1: branch s_32_0 b37 b33
        if s_32_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0s : i
        let s_33_0: i128 = 0;
        // D s_33_1: read-var opcode:u8
        let s_33_1: u8 = fn_state.opcode;
        // D s_33_2: cast zx s_33_1 -> bv
        let s_33_2: Bits = Bits::new(s_33_1 as u128, 3u16);
        // C s_33_3: const #1u : u64
        let s_33_3: u64 = 1;
        // D s_33_4: bit-extract s_33_2 s_33_0 s_33_3
        let s_33_4: Bits = (Bits::new(
            ((s_33_2) >> (s_33_0)).value(),
            u16::try_from(s_33_3).unwrap(),
        ));
        // D s_33_5: cast reint s_33_4 -> u8
        let s_33_5: bool = ((s_33_4.value()) != 0);
        // C s_33_6: const #0s : i
        let s_33_6: i128 = 0;
        // C s_33_7: const #0u : u64
        let s_33_7: u64 = 0;
        // D s_33_8: cast zx s_33_5 -> u64
        let s_33_8: u64 = (s_33_5 as u64);
        // C s_33_9: const #1u : u64
        let s_33_9: u64 = 1;
        // D s_33_10: and s_33_8 s_33_9
        let s_33_10: u64 = ((s_33_8) & (s_33_9));
        // D s_33_11: cmp-eq s_33_10 s_33_9
        let s_33_11: bool = ((s_33_10) == (s_33_9));
        // D s_33_12: lsl s_33_8 s_33_6
        let s_33_12: u64 = s_33_8 << s_33_6;
        // D s_33_13: or s_33_7 s_33_12
        let s_33_13: u64 = ((s_33_7) | (s_33_12));
        // D s_33_14: cmpl s_33_12
        let s_33_14: u64 = !s_33_12;
        // D s_33_15: and s_33_7 s_33_14
        let s_33_15: u64 = ((s_33_7) & (s_33_14));
        // D s_33_16: select s_33_11 s_33_13 s_33_15
        let s_33_16: u64 = if s_33_11 { s_33_13 } else { s_33_15 };
        // D s_33_17: cast trunc s_33_16 -> u8
        let s_33_17: bool = ((s_33_16) != 0);
        // D s_33_18: cast zx s_33_17 -> bv
        let s_33_18: Bits = Bits::new(s_33_17 as u128, 1u16);
        // C s_33_19: const #1u : u8
        let s_33_19: bool = true;
        // C s_33_20: cast zx s_33_19 -> bv
        let s_33_20: Bits = Bits::new(s_33_19 as u128, 1u16);
        // D s_33_21: cmp-eq s_33_18 s_33_20
        let s_33_21: bool = ((s_33_18) == (s_33_20));
        // N s_33_22: branch s_33_21 b36 b34
        if s_33_21 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #2u : u32
        let s_34_0: u32 = 2;
        // D s_34_1: write-var op <= s_34_0
        fn_state.op = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1s : i64
        let s_35_0: i64 = 1;
        // D s_35_1: write-var part <= s_35_0
        fn_state.part = s_35_0;
        // C s_35_2: const #64s : i64
        let s_35_2: i64 = 64;
        // D s_35_3: write-var fltsize <= s_35_2
        fn_state.fltsize = s_35_2;
        // N s_35_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #3u : u32
        let s_36_0: u32 = 3;
        // D s_36_1: write-var op <= s_36_0
        fn_state.op = s_36_0;
        // N s_36_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: panic
        panic!("{:?}", ());
        // N s_37_1: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#150350 <= s_38_0
        fn_state.gs_150350 = s_38_0;
        // N s_38_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var ga#253875:u8
        let s_39_0: u8 = fn_state.ga_253875;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 4u16);
        // C s_39_2: const #15u : u8
        let s_39_2: u8 = 15;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 4u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: not s_39_4
        let s_39_5: bool = !s_39_4;
        // N s_39_6: branch s_39_5 b43 b40
        if s_39_5 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call HaveFJCVTZSExt(s_40_0)
        let s_40_1: bool = HaveFJCVTZSExt(state, tracer, s_40_0);
        // S s_40_2: not s_40_1
        let s_40_2: bool = !s_40_1;
        // N s_40_3: branch s_40_2 b42 b41
        if s_40_2 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #3u : u32
        let s_41_0: u32 = 3;
        // D s_41_1: write-var rounding <= s_41_0
        fn_state.rounding = s_41_0;
        // C s_41_2: const #0s : i
        let s_41_2: i128 = 0;
        // D s_41_3: read-var opcode:u8
        let s_41_3: u8 = fn_state.opcode;
        // D s_41_4: cast zx s_41_3 -> bv
        let s_41_4: Bits = Bits::new(s_41_3 as u128, 3u16);
        // C s_41_5: const #1u : u64
        let s_41_5: u64 = 1;
        // D s_41_6: bit-extract s_41_4 s_41_2 s_41_5
        let s_41_6: Bits = (Bits::new(
            ((s_41_4) >> (s_41_2)).value(),
            u16::try_from(s_41_5).unwrap(),
        ));
        // D s_41_7: cast reint s_41_6 -> u8
        let s_41_7: bool = ((s_41_6.value()) != 0);
        // C s_41_8: const #0s : i
        let s_41_8: i128 = 0;
        // C s_41_9: const #0u : u64
        let s_41_9: u64 = 0;
        // D s_41_10: cast zx s_41_7 -> u64
        let s_41_10: u64 = (s_41_7 as u64);
        // C s_41_11: const #1u : u64
        let s_41_11: u64 = 1;
        // D s_41_12: and s_41_10 s_41_11
        let s_41_12: u64 = ((s_41_10) & (s_41_11));
        // D s_41_13: cmp-eq s_41_12 s_41_11
        let s_41_13: bool = ((s_41_12) == (s_41_11));
        // D s_41_14: lsl s_41_10 s_41_8
        let s_41_14: u64 = s_41_10 << s_41_8;
        // D s_41_15: or s_41_9 s_41_14
        let s_41_15: u64 = ((s_41_9) | (s_41_14));
        // D s_41_16: cmpl s_41_14
        let s_41_16: u64 = !s_41_14;
        // D s_41_17: and s_41_9 s_41_16
        let s_41_17: u64 = ((s_41_9) & (s_41_16));
        // D s_41_18: select s_41_13 s_41_15 s_41_17
        let s_41_18: u64 = if s_41_13 { s_41_15 } else { s_41_17 };
        // D s_41_19: cast trunc s_41_18 -> u8
        let s_41_19: bool = ((s_41_18) != 0);
        // D s_41_20: cast zx s_41_19 -> bv
        let s_41_20: Bits = Bits::new(s_41_19 as u128, 1u16);
        // C s_41_21: const #1u : u8
        let s_41_21: bool = true;
        // C s_41_22: cast zx s_41_21 -> bv
        let s_41_22: Bits = Bits::new(s_41_21 as u128, 1u16);
        // D s_41_23: cmp-eq s_41_20 s_41_22
        let s_41_23: bool = ((s_41_20) == (s_41_22));
        // D s_41_24: write-var is_unsigned <= s_41_23
        fn_state.is_unsigned = s_41_23;
        // C s_41_25: const #4u : u32
        let s_41_25: u32 = 4;
        // D s_41_26: write-var op <= s_41_25
        fn_state.op = s_41_25;
        // N s_41_27: jump b6
        return block_6(state, tracer, fn_state);
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
        // N s_43_0: panic
        panic!("{:?}", ());
        // N s_43_1: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var ftype:u8
        let s_44_0: u8 = fn_state.ftype;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 2u16);
        // C s_44_2: const #1u : u8
        let s_44_2: u8 = 1;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 2u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: not s_44_4
        let s_44_5: bool = !s_44_4;
        // N s_44_6: branch s_44_5 b46 b45
        if s_44_5 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #64s : i64
        let s_45_0: i64 = 64;
        // D s_45_1: write-var fltsize <= s_45_0
        fn_state.fltsize = s_45_0;
        // N s_45_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var ftype:u8
        let s_46_0: u8 = fn_state.ftype;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 2u16);
        // C s_46_2: const #2u : u8
        let s_46_2: u8 = 2;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 2u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: not s_46_4
        let s_46_5: bool = !s_46_4;
        // N s_46_6: branch s_46_5 b50 b47
        if s_46_5 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1s : i
        let s_47_0: i128 = 1;
        // D s_47_1: read-var opcode:u8
        let s_47_1: u8 = fn_state.opcode;
        // D s_47_2: cast zx s_47_1 -> bv
        let s_47_2: Bits = Bits::new(s_47_1 as u128, 3u16);
        // C s_47_3: const #1s : i64
        let s_47_3: i64 = 1;
        // C s_47_4: cast zx s_47_3 -> i
        let s_47_4: i128 = (i128::try_from(s_47_3).unwrap());
        // C s_47_5: const #1s : i
        let s_47_5: i128 = 1;
        // C s_47_6: add s_47_5 s_47_4
        let s_47_6: i128 = (s_47_5 + s_47_4);
        // D s_47_7: bit-extract s_47_2 s_47_0 s_47_6
        let s_47_7: Bits = (Bits::new(
            ((s_47_2) >> (s_47_0)).value(),
            u16::try_from(s_47_6).unwrap(),
        ));
        // D s_47_8: cast reint s_47_7 -> u8
        let s_47_8: u8 = (s_47_7.value() as u8);
        // D s_47_9: cast zx s_47_8 -> bv
        let s_47_9: Bits = Bits::new(s_47_8 as u128, 2u16);
        // D s_47_10: read-var rmode:u8
        let s_47_10: u8 = fn_state.rmode;
        // D s_47_11: cast zx s_47_10 -> bv
        let s_47_11: Bits = Bits::new(s_47_10 as u128, 2u16);
        // D s_47_12: cast reint s_47_9 -> u128
        let s_47_12: u128 = (s_47_9.value() as u128);
        // D s_47_13: size-of s_47_9
        let s_47_13: u16 = s_47_9.length();
        // D s_47_14: cast reint s_47_11 -> u128
        let s_47_14: u128 = (s_47_11.value() as u128);
        // D s_47_15: size-of s_47_11
        let s_47_15: u16 = s_47_11.length();
        // D s_47_16: lsl s_47_12 s_47_15
        let s_47_16: u128 = s_47_12 << s_47_15;
        // D s_47_17: or s_47_16 s_47_14
        let s_47_17: u128 = ((s_47_16) | (s_47_14));
        // D s_47_18: add s_47_13 s_47_15
        let s_47_18: u16 = (s_47_13 + s_47_15);
        // D s_47_19: create-bits s_47_17 s_47_18
        let s_47_19: Bits = Bits::new(s_47_17, s_47_18);
        // D s_47_20: cast reint s_47_19 -> u8
        let s_47_20: u8 = (s_47_19.value() as u8);
        // D s_47_21: cast zx s_47_20 -> bv
        let s_47_21: Bits = Bits::new(s_47_20 as u128, 4u16);
        // C s_47_22: const #13u : u8
        let s_47_22: u8 = 13;
        // C s_47_23: cast zx s_47_22 -> bv
        let s_47_23: Bits = Bits::new(s_47_22 as u128, 4u16);
        // D s_47_24: cmp-ne s_47_21 s_47_23
        let s_47_24: bool = ((s_47_21) != (s_47_23));
        // N s_47_25: branch s_47_24 b49 b48
        if s_47_24 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #128s : i64
        let s_48_0: i64 = 128;
        // D s_48_1: write-var fltsize <= s_48_0
        fn_state.fltsize = s_48_0;
        // N s_48_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_49_0: panic
        panic!("{:?}", ());
        // N s_49_1: return
        return;
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #16s : i64
        let s_50_0: i64 = 16;
        // D s_50_1: write-var fltsize <= s_50_0
        fn_state.fltsize = s_50_0;
        // N s_50_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #64s : i64
        let s_51_0: i64 = 64;
        // D s_51_1: write-var ga#253869 <= s_51_0
        fn_state.ga_253869 = s_51_0;
        // N s_51_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
