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
use FPCR_read::*;
use HaveFrintExt::*;
use execute_aarch64_instrs_float_arithmetic_round_frint_32_64::*;
use common::*;
pub fn decode_frint32z_float_aarch64_instrs_float_arithmetic_round_frint_32_64<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, op: u8, ftype: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        n: i64,
        ga_256292: i64,
        d: i64,
        intsize: i64,
        rounding: u32,
        esizeshadow_1431: i64,
        Rd: u8,
        Rn: u8,
        op: u8,
        ftype: u8,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op,
        ftype,
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
        // S s_0_1: call HaveFrintExt(s_0_0)
        let s_0_1: bool = HaveFrintExt(state, tracer, s_0_0);
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
        // C s_1_10: const #32s : i64
        let s_1_10: i64 = 32;
        // D s_1_11: write-var esize <= s_1_10
        fn_state.esize = s_1_10;
        // D s_1_12: read-var ftype:u8
        let s_1_12: u8 = fn_state.ftype;
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 2u16);
        // C s_1_14: const #0u : u8
        let s_1_14: u8 = 0;
        // C s_1_15: cast zx s_1_14 -> bv
        let s_1_15: Bits = Bits::new(s_1_14 as u128, 2u16);
        // D s_1_16: cmp-eq s_1_13 s_1_15
        let s_1_16: bool = ((s_1_13) == (s_1_15));
        // D s_1_17: not s_1_16
        let s_1_17: bool = !s_1_16;
        // N s_1_18: branch s_1_17 b10 b2
        if s_1_17 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #32s : i64
        let s_2_0: i64 = 32;
        // D s_2_1: write-var esize <= s_2_0
        fn_state.esize = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esize:i64
        let s_3_0: i64 = fn_state.esize;
        // D s_3_1: write-var esizeshadow#1431 <= s_3_0
        fn_state.esizeshadow_1431 = s_3_0;
        // C s_3_2: const #1s : i
        let s_3_2: i128 = 1;
        // D s_3_3: read-var op:u8
        let s_3_3: u8 = fn_state.op;
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // C s_3_5: const #1u : u64
        let s_3_5: u64 = 1;
        // D s_3_6: bit-extract s_3_4 s_3_2 s_3_5
        let s_3_6: Bits = (Bits::new(
            ((s_3_4) >> (s_3_2)).value(),
            u16::try_from(s_3_5).unwrap(),
        ));
        // D s_3_7: cast reint s_3_6 -> u8
        let s_3_7: bool = ((s_3_6.value()) != 0);
        // C s_3_8: const #0s : i
        let s_3_8: i128 = 0;
        // C s_3_9: const #0u : u64
        let s_3_9: u64 = 0;
        // D s_3_10: cast zx s_3_7 -> u64
        let s_3_10: u64 = (s_3_7 as u64);
        // C s_3_11: const #1u : u64
        let s_3_11: u64 = 1;
        // D s_3_12: and s_3_10 s_3_11
        let s_3_12: u64 = ((s_3_10) & (s_3_11));
        // D s_3_13: cmp-eq s_3_12 s_3_11
        let s_3_13: bool = ((s_3_12) == (s_3_11));
        // D s_3_14: lsl s_3_10 s_3_8
        let s_3_14: u64 = s_3_10 << s_3_8;
        // D s_3_15: or s_3_9 s_3_14
        let s_3_15: u64 = ((s_3_9) | (s_3_14));
        // D s_3_16: cmpl s_3_14
        let s_3_16: u64 = !s_3_14;
        // D s_3_17: and s_3_9 s_3_16
        let s_3_17: u64 = ((s_3_9) & (s_3_16));
        // D s_3_18: select s_3_13 s_3_15 s_3_17
        let s_3_18: u64 = if s_3_13 { s_3_15 } else { s_3_17 };
        // D s_3_19: cast trunc s_3_18 -> u8
        let s_3_19: bool = ((s_3_18) != 0);
        // D s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 1u16);
        // C s_3_21: const #0u : u8
        let s_3_21: bool = false;
        // C s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 1u16);
        // D s_3_23: cmp-eq s_3_20 s_3_22
        let s_3_23: bool = ((s_3_20) == (s_3_22));
        // N s_3_24: branch s_3_23 b9 b4
        if s_3_23 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64s : i64
        let s_4_0: i64 = 64;
        // D s_4_1: write-var ga#256292 <= s_4_0
        fn_state.ga_256292 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#256292:i64
        let s_5_0: i64 = fn_state.ga_256292;
        // D s_5_1: write-var intsize <= s_5_0
        fn_state.intsize = s_5_0;
        // C s_5_2: const #0s : i
        let s_5_2: i128 = 0;
        // D s_5_3: read-var op:u8
        let s_5_3: u8 = fn_state.op;
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // C s_5_5: const #1u : u64
        let s_5_5: u64 = 1;
        // D s_5_6: bit-extract s_5_4 s_5_2 s_5_5
        let s_5_6: Bits = (Bits::new(
            ((s_5_4) >> (s_5_2)).value(),
            u16::try_from(s_5_5).unwrap(),
        ));
        // D s_5_7: cast reint s_5_6 -> u8
        let s_5_7: bool = ((s_5_6.value()) != 0);
        // C s_5_8: const #0s : i
        let s_5_8: i128 = 0;
        // C s_5_9: const #0u : u64
        let s_5_9: u64 = 0;
        // D s_5_10: cast zx s_5_7 -> u64
        let s_5_10: u64 = (s_5_7 as u64);
        // C s_5_11: const #1u : u64
        let s_5_11: u64 = 1;
        // D s_5_12: and s_5_10 s_5_11
        let s_5_12: u64 = ((s_5_10) & (s_5_11));
        // D s_5_13: cmp-eq s_5_12 s_5_11
        let s_5_13: bool = ((s_5_12) == (s_5_11));
        // D s_5_14: lsl s_5_10 s_5_8
        let s_5_14: u64 = s_5_10 << s_5_8;
        // D s_5_15: or s_5_9 s_5_14
        let s_5_15: u64 = ((s_5_9) | (s_5_14));
        // D s_5_16: cmpl s_5_14
        let s_5_16: u64 = !s_5_14;
        // D s_5_17: and s_5_9 s_5_16
        let s_5_17: u64 = ((s_5_9) & (s_5_16));
        // D s_5_18: select s_5_13 s_5_15 s_5_17
        let s_5_18: u64 = if s_5_13 { s_5_15 } else { s_5_17 };
        // D s_5_19: cast trunc s_5_18 -> u8
        let s_5_19: bool = ((s_5_18) != 0);
        // D s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // C s_5_21: const #0u : u8
        let s_5_21: bool = false;
        // C s_5_22: cast zx s_5_21 -> bv
        let s_5_22: Bits = Bits::new(s_5_21 as u128, 1u16);
        // D s_5_23: cmp-eq s_5_20 s_5_22
        let s_5_23: bool = ((s_5_20) == (s_5_22));
        // N s_5_24: branch s_5_23 b8 b6
        if s_5_23 {
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
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call FPCR_read(s_6_0)
        let s_6_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_6_0);
        // S s_6_2: call FPRoundingMode(s_6_1)
        let s_6_2: u32 = FPRoundingMode(state, tracer, s_6_1);
        // D s_6_3: write-var rounding <= s_6_2
        fn_state.rounding = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esizeshadow#1431:i64
        let s_7_0: i64 = fn_state.esizeshadow_1431;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: read-var intsize:i64
        let s_7_4: i64 = fn_state.intsize;
        // D s_7_5: read-var n:i64
        let s_7_5: i64 = fn_state.n;
        // D s_7_6: read-var rounding:u32
        let s_7_6: u32 = fn_state.rounding;
        // D s_7_7: call execute_aarch64_instrs_float_arithmetic_round_frint_32_64(s_7_3, s_7_2, s_7_4, s_7_5, s_7_6)
        let s_7_7: () = execute_aarch64_instrs_float_arithmetic_round_frint_32_64(
            state,
            tracer,
            s_7_3,
            s_7_2,
            s_7_4,
            s_7_5,
            s_7_6,
        );
        // N s_7_8: return
        return;
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
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #32s : i64
        let s_9_0: i64 = 32;
        // D s_9_1: write-var ga#256292 <= s_9_0
        fn_state.ga_256292 = s_9_0;
        // N s_9_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ftype:u8
        let s_10_0: u8 = fn_state.ftype;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #1u : u8
        let s_10_2: u8 = 1;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
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
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // D s_11_1: write-var esize <= s_11_0
        fn_state.esize = s_11_0;
        // N s_11_2: jump b3
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
