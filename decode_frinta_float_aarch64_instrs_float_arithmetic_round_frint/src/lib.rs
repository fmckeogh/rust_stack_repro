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
use FPDecodeRounding::*;
use execute_aarch64_instrs_float_arithmetic_round_frint::*;
use FPCR_read::*;
use common::*;
pub fn decode_frinta_float_aarch64_instrs_float_arithmetic_round_frint<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    rmode: u8,
    ftype: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        exact: bool,
        n: i64,
        d: i64,
        esizeshadow_1451: i64,
        rounding: u32,
        Rd: u8,
        Rn: u8,
        rmode: u8,
        ftype: u8,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        rmode,
        ftype,
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
        // C s_0_10: const #16s : i64
        let s_0_10: i64 = 16;
        // D s_0_11: write-var esize <= s_0_10
        fn_state.esize = s_0_10;
        // D s_0_12: read-var ftype:u8
        let s_0_12: u8 = fn_state.ftype;
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // C s_0_14: const #0u : u8
        let s_0_14: u8 = 0;
        // C s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cmp-eq s_0_13 s_0_15
        let s_0_16: bool = ((s_0_13) == (s_0_15));
        // D s_0_17: not s_0_16
        let s_0_17: bool = !s_0_16;
        // N s_0_18: branch s_0_17 b12 b1
        if s_0_17 {
            return block_12(state, tracer, fn_state);
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
        // D s_1_1: write-var esize <= s_1_0
        fn_state.esize = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var esize:i64
        let s_2_0: i64 = fn_state.esize;
        // D s_2_1: write-var esizeshadow#1451 <= s_2_0
        fn_state.esizeshadow_1451 = s_2_0;
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // D s_2_3: write-var exact <= s_2_2
        fn_state.exact = s_2_2;
        // D s_2_4: read-var rmode:u8
        let s_2_4: u8 = fn_state.rmode;
        // C s_2_5: const #2s : i
        let s_2_5: i128 = 2;
        // D s_2_6: cast zx s_2_4 -> bv
        let s_2_6: Bits = Bits::new(s_2_4 as u128, 3u16);
        // C s_2_7: const #1s : i64
        let s_2_7: i64 = 1;
        // C s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // C s_2_9: const #0s : i
        let s_2_9: i128 = 0;
        // C s_2_10: add s_2_9 s_2_8
        let s_2_10: i128 = (s_2_9 + s_2_8);
        // D s_2_11: bit-extract s_2_6 s_2_5 s_2_10
        let s_2_11: Bits = (Bits::new(
            ((s_2_6) >> (s_2_5)).value(),
            u16::try_from(s_2_10).unwrap(),
        ));
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: bool = ((s_2_11.value()) != 0);
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 1u16);
        // C s_2_14: const #0u : u8
        let s_2_14: bool = false;
        // C s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 1u16);
        // D s_2_16: cmp-eq s_2_13 s_2_15
        let s_2_16: bool = ((s_2_13) == (s_2_15));
        // D s_2_17: not s_2_16
        let s_2_17: bool = !s_2_16;
        // N s_2_18: branch s_2_17 b5 b3
        if s_2_17 {
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
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var rmode:u8
        let s_3_1: u8 = fn_state.rmode;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 3u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #1s : i
        let s_3_5: i128 = 1;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_0 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: u8 = (s_3_7.value() as u8);
        // D s_3_9: call FPDecodeRounding(s_3_8)
        let s_3_9: u32 = FPDecodeRounding(state, tracer, s_3_8);
        // D s_3_10: write-var rounding <= s_3_9
        fn_state.rounding = s_3_9;
        // N s_3_11: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var rounding:u32
        let s_4_0: u32 = fn_state.rounding;
        // D s_4_1: read-var esizeshadow#1451:i64
        let s_4_1: i64 = fn_state.esizeshadow_1451;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: read-var d:i64
        let s_4_4: i64 = fn_state.d;
        // D s_4_5: read-var exact:u8
        let s_4_5: bool = fn_state.exact;
        // D s_4_6: read-var n:i64
        let s_4_6: i64 = fn_state.n;
        // D s_4_7: call execute_aarch64_instrs_float_arithmetic_round_frint(s_4_4, s_4_3, s_4_5, s_4_6, s_4_0)
        let s_4_7: () = execute_aarch64_instrs_float_arithmetic_round_frint(
            state,
            tracer,
            s_4_4,
            s_4_3,
            s_4_5,
            s_4_6,
            s_4_0,
        );
        // N s_4_8: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var rmode:u8
        let s_5_0: u8 = fn_state.rmode;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 3u16);
        // C s_5_2: const #4u : u8
        let s_5_2: u8 = 4;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 3u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #4u : u32
        let s_6_0: u32 = 4;
        // D s_6_1: write-var rounding <= s_6_0
        fn_state.rounding = s_6_0;
        // N s_6_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var rmode:u8
        let s_7_0: u8 = fn_state.rmode;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 3u16);
        // C s_7_2: const #5u : u8
        let s_7_2: u8 = 5;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 3u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var rmode:u8
        let s_9_0: u8 = fn_state.rmode;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 3u16);
        // C s_9_2: const #6u : u8
        let s_9_2: u8 = 6;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 3u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: not s_9_4
        let s_9_5: bool = !s_9_4;
        // N s_9_6: branch s_9_5 b11 b10
        if s_9_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call FPCR_read(s_10_0)
        let s_10_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_10_0);
        // S s_10_2: call FPRoundingMode(s_10_1)
        let s_10_2: u32 = FPRoundingMode(state, tracer, s_10_1);
        // D s_10_3: write-var rounding <= s_10_2
        fn_state.rounding = s_10_2;
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // D s_10_5: write-var exact <= s_10_4
        fn_state.exact = s_10_4;
        // N s_10_6: jump b4
        return block_4(state, tracer, fn_state);
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
        // N s_11_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ftype:u8
        let s_12_0: u8 = fn_state.ftype;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 2u16);
        // C s_12_2: const #1u : u8
        let s_12_2: u8 = 1;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
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
        // C s_13_0: const #64s : i64
        let s_13_0: i64 = 64;
        // D s_13_1: write-var esize <= s_13_0
        fn_state.esize = s_13_0;
        // N s_13_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var ftype:u8
        let s_14_0: u8 = fn_state.ftype;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 2u16);
        // C s_14_2: const #2u : u8
        let s_14_2: u8 = 2;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 2u16);
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
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #16s : i64
        let s_16_0: i64 = 16;
        // D s_16_1: write-var esize <= s_16_0
        fn_state.esize = s_16_0;
        // N s_16_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
