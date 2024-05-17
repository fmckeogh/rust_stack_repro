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
use CheckFPEnabled64::*;
use Elem_set::*;
use V_read::*;
use FPCR_read::*;
use V_set::*;
use FPAbs::*;
use IsMerging::*;
use FPNeg::*;
use FPSqrt::*;
use common::*;
pub fn execute_aarch64_instrs_float_arithmetic_unary<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    esize: i64,
    fpop: u32,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        fpcr: ProductType5c790c8ef59cc8b2,
        ga_252682: i64,
        ga_252680: i64,
        gs_148227: bool,
        esizeshadow_1191: i64,
        ga_252683: Bits,
        result: u128,
        ga_252684: i64,
        ga_252681: Bits,
        ga_252685: Bits,
        d: i64,
        esize: i64,
        fpop: u32,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        esize,
        fpop,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#1191 <= s_0_2
        fn_state.esizeshadow_1191 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPEnabled64(s_0_4)
        let s_0_5: () = CheckFPEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call FPCR_read(s_1_0)
        let s_1_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_1_0);
        // D s_1_2: write-var fpcr <= s_1_1
        fn_state.fpcr = s_1_1;
        // D s_1_3: read-var fpop:u32
        let s_1_3: u32 = fn_state.fpop;
        // C s_1_4: const #1u : u32
        let s_1_4: u32 = 1;
        // D s_1_5: cmp-eq s_1_3 s_1_4
        let s_1_5: bool = ((s_1_3) == (s_1_4));
        // N s_1_6: branch s_1_5 b19 b2
        if s_1_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#148227 <= s_2_0
        fn_state.gs_148227 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#148227:u8
        let s_3_0: bool = fn_state.gs_148227;
        // N s_3_1: branch s_3_0 b18 b4
        if s_3_0 {
            return block_18(state, tracer, fn_state);
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
        let s_4_0: u8 = 0;
        // C s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 0u16);
        // C s_4_2: const #0u : u64
        let s_4_2: u64 = 0;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 64u16);
        // C s_4_4: cast reint s_4_1 -> u128
        let s_4_4: u128 = (s_4_1.value() as u128);
        // D s_4_5: size-of s_4_1
        let s_4_5: u16 = s_4_1.length();
        // C s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: lsl s_4_4 s_4_7
        let s_4_8: u128 = s_4_4 << s_4_7;
        // D s_4_9: or s_4_8 s_4_6
        let s_4_9: u128 = ((s_4_8) | (s_4_6));
        // D s_4_10: add s_4_5 s_4_7
        let s_4_10: u16 = (s_4_5 + s_4_7);
        // D s_4_11: create-bits s_4_9 s_4_10
        let s_4_11: Bits = Bits::new(s_4_9, s_4_10);
        // C s_4_12: const #0u : u64
        let s_4_12: u64 = 0;
        // C s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 64u16);
        // D s_4_14: cast reint s_4_11 -> u128
        let s_4_14: u128 = (s_4_11.value() as u128);
        // D s_4_15: size-of s_4_11
        let s_4_15: u16 = s_4_11.length();
        // C s_4_16: cast reint s_4_13 -> u128
        let s_4_16: u128 = (s_4_13.value() as u128);
        // D s_4_17: size-of s_4_13
        let s_4_17: u16 = s_4_13.length();
        // D s_4_18: lsl s_4_14 s_4_17
        let s_4_18: u128 = s_4_14 << s_4_17;
        // D s_4_19: or s_4_18 s_4_16
        let s_4_19: u128 = ((s_4_18) | (s_4_16));
        // D s_4_20: add s_4_15 s_4_17
        let s_4_20: u16 = (s_4_15 + s_4_17);
        // D s_4_21: create-bits s_4_19 s_4_20
        let s_4_21: Bits = Bits::new(s_4_19, s_4_20);
        // D s_4_22: cast reint s_4_21 -> u128
        let s_4_22: u128 = (s_4_21.value() as u128);
        // D s_4_23: write-var result <= s_4_22
        fn_state.result = s_4_22;
        // N s_4_24: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#1191:i64
        let s_5_0: i64 = fn_state.esizeshadow_1191;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var n:i64
        let s_5_3: i64 = fn_state.n;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: call V_read(s_5_4, s_5_2)
        let s_5_5: Bits = V_read(state, tracer, s_5_4, s_5_2);
        // D s_5_6: write-var operand <= s_5_5
        fn_state.operand = s_5_5;
        // C s_5_7: const #1u : u32
        let s_5_7: u32 = 1;
        // D s_5_8: read-var fpop:u32
        let s_5_8: u32 = fn_state.fpop;
        // D s_5_9: cmp-eq s_5_7 s_5_8
        let s_5_9: bool = ((s_5_7) == (s_5_8));
        // D s_5_10: not s_5_9
        let s_5_10: bool = !s_5_9;
        // N s_5_11: branch s_5_10 b8 b6
        if s_5_10 {
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
        // D s_6_0: read-var esizeshadow#1191:i64
        let s_6_0: i64 = fn_state.esizeshadow_1191;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // C s_6_3: const #0s : i
        let s_6_3: i128 = 0;
        // D s_6_4: read-var result:u128
        let s_6_4: u128 = fn_state.result;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 128u16);
        // D s_6_6: cast zx s_6_2 -> i
        let s_6_6: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_7: read-var operand:bv
        let s_6_7: Bits = fn_state.operand;
        // D s_6_8: call Elem_set(s_6_5, s_6_3, s_6_6, s_6_7)
        let s_6_8: Bits = Elem_set(state, tracer, s_6_5, s_6_3, s_6_6, s_6_7);
        // D s_6_9: cast reint s_6_8 -> u128
        let s_6_9: u128 = (s_6_8.value() as u128);
        // D s_6_10: write-var result <= s_6_9
        fn_state.result = s_6_9;
        // N s_6_11: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #128s : i64
        let s_7_0: i64 = 128;
        // D s_7_1: read-var d:i64
        let s_7_1: i64 = fn_state.d;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: read-var result:u128
        let s_7_3: u128 = fn_state.result;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 128u16);
        // D s_7_5: call V_set(s_7_2, s_7_0, s_7_4)
        let s_7_5: () = V_set(state, tracer, s_7_2, s_7_0, s_7_4);
        // N s_7_6: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u32
        let s_8_0: u32 = 0;
        // D s_8_1: read-var fpop:u32
        let s_8_1: u32 = fn_state.fpop;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b11 b9
        if s_8_3 {
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
        // D s_9_0: read-var esizeshadow#1191:i64
        let s_9_0: i64 = fn_state.esizeshadow_1191;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: write-var ga#252680 <= s_9_2
        fn_state.ga_252680 = s_9_2;
        // D s_9_4: read-var operand:bv
        let s_9_4: Bits = fn_state.operand;
        // D s_9_5: call FPAbs(s_9_4)
        let s_9_5: Bits = FPAbs(state, tracer, s_9_4);
        // D s_9_6: write-var ga#252681 <= s_9_5
        fn_state.ga_252681 = s_9_5;
        // N s_9_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i
        let s_10_0: i128 = 0;
        // D s_10_1: read-var result:u128
        let s_10_1: u128 = fn_state.result;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 128u16);
        // D s_10_3: read-var ga#252680:i64
        let s_10_3: i64 = fn_state.ga_252680;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: read-var ga#252681:bv
        let s_10_5: Bits = fn_state.ga_252681;
        // D s_10_6: call Elem_set(s_10_2, s_10_0, s_10_4, s_10_5)
        let s_10_6: Bits = Elem_set(state, tracer, s_10_2, s_10_0, s_10_4, s_10_5);
        // D s_10_7: cast reint s_10_6 -> u128
        let s_10_7: u128 = (s_10_6.value() as u128);
        // D s_10_8: write-var result <= s_10_7
        fn_state.result = s_10_7;
        // N s_10_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2u : u32
        let s_11_0: u32 = 2;
        // D s_11_1: read-var fpop:u32
        let s_11_1: u32 = fn_state.fpop;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: not s_11_2
        let s_11_3: bool = !s_11_2;
        // N s_11_4: branch s_11_3 b14 b12
        if s_11_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esizeshadow#1191:i64
        let s_12_0: i64 = fn_state.esizeshadow_1191;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: write-var ga#252682 <= s_12_2
        fn_state.ga_252682 = s_12_2;
        // D s_12_4: read-var operand:bv
        let s_12_4: Bits = fn_state.operand;
        // D s_12_5: call FPNeg(s_12_4)
        let s_12_5: Bits = FPNeg(state, tracer, s_12_4);
        // D s_12_6: write-var ga#252683 <= s_12_5
        fn_state.ga_252683 = s_12_5;
        // N s_12_7: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var result:u128
        let s_13_1: u128 = fn_state.result;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 128u16);
        // D s_13_3: read-var ga#252682:i64
        let s_13_3: i64 = fn_state.ga_252682;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: read-var ga#252683:bv
        let s_13_5: Bits = fn_state.ga_252683;
        // D s_13_6: call Elem_set(s_13_2, s_13_0, s_13_4, s_13_5)
        let s_13_6: Bits = Elem_set(state, tracer, s_13_2, s_13_0, s_13_4, s_13_5);
        // D s_13_7: cast reint s_13_6 -> u128
        let s_13_7: u128 = (s_13_6.value() as u128);
        // D s_13_8: write-var result <= s_13_7
        fn_state.result = s_13_7;
        // N s_13_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #3u : u32
        let s_14_0: u32 = 3;
        // D s_14_1: read-var fpop:u32
        let s_14_1: u32 = fn_state.fpop;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // D s_14_3: not s_14_2
        let s_14_3: bool = !s_14_2;
        // N s_14_4: branch s_14_3 b17 b15
        if s_14_3 {
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
        // D s_15_0: read-var esizeshadow#1191:i64
        let s_15_0: i64 = fn_state.esizeshadow_1191;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: cast reint s_15_1 -> i64
        let s_15_2: i64 = (s_15_1 as i64);
        // D s_15_3: write-var ga#252684 <= s_15_2
        fn_state.ga_252684 = s_15_2;
        // D s_15_4: read-var operand:bv
        let s_15_4: Bits = fn_state.operand;
        // D s_15_5: read-var fpcr:struct
        let s_15_5: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_15_6: call FPSqrt(s_15_4, s_15_5)
        let s_15_6: Bits = FPSqrt(state, tracer, s_15_4, s_15_5);
        // D s_15_7: write-var ga#252685 <= s_15_6
        fn_state.ga_252685 = s_15_6;
        // N s_15_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0s : i
        let s_16_0: i128 = 0;
        // D s_16_1: read-var result:u128
        let s_16_1: u128 = fn_state.result;
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 128u16);
        // D s_16_3: read-var ga#252684:i64
        let s_16_3: i64 = fn_state.ga_252684;
        // D s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: read-var ga#252685:bv
        let s_16_5: Bits = fn_state.ga_252685;
        // D s_16_6: call Elem_set(s_16_2, s_16_0, s_16_4, s_16_5)
        let s_16_6: Bits = Elem_set(state, tracer, s_16_2, s_16_0, s_16_4, s_16_5);
        // D s_16_7: cast reint s_16_6 -> u128
        let s_16_7: u128 = (s_16_6.value() as u128);
        // D s_16_8: write-var result <= s_16_7
        fn_state.result = s_16_7;
        // N s_16_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #128s : i64
        let s_18_0: i64 = 128;
        // D s_18_1: read-var d:i64
        let s_18_1: i64 = fn_state.d;
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // D s_18_3: call V_read(s_18_2, s_18_0)
        let s_18_3: Bits = V_read(state, tracer, s_18_2, s_18_0);
        // D s_18_4: cast reint s_18_3 -> u128
        let s_18_4: u128 = (s_18_3.value() as u128);
        // D s_18_5: write-var result <= s_18_4
        fn_state.result = s_18_4;
        // N s_18_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var fpcr:struct
        let s_19_0: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_19_1: call IsMerging(s_19_0)
        let s_19_1: bool = IsMerging(state, tracer, s_19_0);
        // D s_19_2: write-var gs#148227 <= s_19_1
        fn_state.gs_148227 = s_19_1;
        // N s_19_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
