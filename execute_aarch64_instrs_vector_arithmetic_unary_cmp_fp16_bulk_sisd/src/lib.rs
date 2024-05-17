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
use FPCompareEQ::*;
use Elem_set::*;
use V_read::*;
use FPCR_read::*;
use Ones::*;
use FPCompareGE::*;
use V_set::*;
use FPZero::*;
use Zeros::*;
use Elem_read::*;
use FPCompareGT::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_cmp_fp16_bulk_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    comparison: u32,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        gs_149234: i64,
        datasizeshadow_1257: i64,
        ga_253180: i64,
        e: i64,
        esizeshadow_1256: i64,
        test_passed: bool,
        ga_253181: Bits,
        element: Bits,
        result: Bits,
        zero: Bits,
        comparison: u32,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        comparison,
        d,
        datasize,
        elements,
        esize,
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
        // D s_0_3: write-var esizeshadow#1256 <= s_0_2
        fn_state.esizeshadow_1256 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1257 <= s_0_6
        fn_state.datasizeshadow_1257 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPAdvSIMDEnabled64(s_0_8)
        let s_0_9: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1257:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1257;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var operand <= s_1_5
        fn_state.operand = s_1_5;
        // D s_1_7: read-var esizeshadow#1256:i64
        let s_1_7: i64 = fn_state.esizeshadow_1256;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // C s_1_10: const #0u : u8
        let s_1_10: bool = false;
        // D s_1_11: call FPZero(s_1_10, s_1_9)
        let s_1_11: Bits = FPZero(state, tracer, s_1_10, s_1_9);
        // D s_1_12: write-var zero <= s_1_11
        fn_state.zero = s_1_11;
        // C s_1_13: const #0s : i64
        let s_1_13: i64 = 0;
        // C s_1_14: const #1s : i
        let s_1_14: i128 = 1;
        // D s_1_15: read-var elements:i
        let s_1_15: i128 = fn_state.elements;
        // D s_1_16: sub s_1_15 s_1_14
        let s_1_16: i128 = ((s_1_15) - (s_1_14));
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: write-var gs#149234 <= s_1_17
        fn_state.gs_149234 = s_1_17;
        // D s_1_19: write-var e <= s_1_13
        fn_state.e = s_1_13;
        // N s_1_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#149234:i64
        let s_2_1: i64 = fn_state.gs_149234;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b23 b3
        if s_2_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#1256:i64
        let s_3_0: i64 = fn_state.esizeshadow_1256;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand:bv
        let s_3_6: Bits = fn_state.operand;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: write-var element <= s_3_7
        fn_state.element = s_3_7;
        // C s_3_9: const #0u : u32
        let s_3_9: u32 = 0;
        // D s_3_10: read-var comparison:u32
        let s_3_10: u32 = fn_state.comparison;
        // D s_3_11: cmp-eq s_3_9 s_3_10
        let s_3_11: bool = ((s_3_9) == (s_3_10));
        // D s_3_12: not s_3_11
        let s_3_12: bool = !s_3_11;
        // N s_3_13: branch s_3_12 b10 b4
        if s_3_12 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call FPCR_read(s_4_0)
        let s_4_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_4_0);
        // D s_4_2: read-var element:bv
        let s_4_2: Bits = fn_state.element;
        // D s_4_3: read-var zero:bv
        let s_4_3: Bits = fn_state.zero;
        // D s_4_4: call FPCompareGT(s_4_2, s_4_3, s_4_1)
        let s_4_4: bool = FPCompareGT(state, tracer, s_4_2, s_4_3, s_4_1);
        // D s_4_5: write-var test_passed <= s_4_4
        fn_state.test_passed = s_4_4;
        // N s_4_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#1256:i64
        let s_6_0: i64 = fn_state.esizeshadow_1256;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: write-var ga#253180 <= s_6_2
        fn_state.ga_253180 = s_6_2;
        // D s_6_4: read-var test_passed:u8
        let s_6_4: bool = fn_state.test_passed;
        // N s_6_5: branch s_6_4 b9 b7
        if s_6_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esizeshadow#1256:i64
        let s_7_0: i64 = fn_state.esizeshadow_1256;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call Zeros(s_7_1)
        let s_7_2: Bits = Zeros(state, tracer, s_7_1);
        // D s_7_3: write-var ga#253181 <= s_7_2
        fn_state.ga_253181 = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var e:i64
        let s_8_0: i64 = fn_state.e;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var ga#253180:i64
        let s_8_2: i64 = fn_state.ga_253180;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: read-var result:bv
        let s_8_4: Bits = fn_state.result;
        // D s_8_5: read-var ga#253181:bv
        let s_8_5: Bits = fn_state.ga_253181;
        // D s_8_6: call Elem_set(s_8_4, s_8_1, s_8_3, s_8_5)
        let s_8_6: Bits = Elem_set(state, tracer, s_8_4, s_8_1, s_8_3, s_8_5);
        // D s_8_7: write-var result <= s_8_6
        fn_state.result = s_8_6;
        // D s_8_8: read-var e:i64
        let s_8_8: i64 = fn_state.e;
        // C s_8_9: const #1s : i64
        let s_8_9: i64 = 1;
        // D s_8_10: add s_8_8 s_8_9
        let s_8_10: i64 = (s_8_8 + s_8_9);
        // D s_8_11: write-var e <= s_8_10
        fn_state.e = s_8_10;
        // N s_8_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esizeshadow#1256:i64
        let s_9_0: i64 = fn_state.esizeshadow_1256;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call Ones(s_9_1)
        let s_9_2: Bits = Ones(state, tracer, s_9_1);
        // D s_9_3: write-var ga#253181 <= s_9_2
        fn_state.ga_253181 = s_9_2;
        // N s_9_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u32
        let s_10_0: u32 = 1;
        // D s_10_1: read-var comparison:u32
        let s_10_1: u32 = fn_state.comparison;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b13 b11
        if s_10_3 {
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
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call FPCR_read(s_11_0)
        let s_11_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_11_0);
        // D s_11_2: read-var element:bv
        let s_11_2: Bits = fn_state.element;
        // D s_11_3: read-var zero:bv
        let s_11_3: Bits = fn_state.zero;
        // D s_11_4: call FPCompareGE(s_11_2, s_11_3, s_11_1)
        let s_11_4: bool = FPCompareGE(state, tracer, s_11_2, s_11_3, s_11_1);
        // D s_11_5: write-var test_passed <= s_11_4
        fn_state.test_passed = s_11_4;
        // N s_11_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #2u : u32
        let s_13_0: u32 = 2;
        // D s_13_1: read-var comparison:u32
        let s_13_1: u32 = fn_state.comparison;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b16 b14
        if s_13_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call FPCR_read(s_14_0)
        let s_14_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_14_0);
        // D s_14_2: read-var element:bv
        let s_14_2: Bits = fn_state.element;
        // D s_14_3: read-var zero:bv
        let s_14_3: Bits = fn_state.zero;
        // D s_14_4: call FPCompareEQ(s_14_2, s_14_3, s_14_1)
        let s_14_4: bool = FPCompareEQ(state, tracer, s_14_2, s_14_3, s_14_1);
        // D s_14_5: write-var test_passed <= s_14_4
        fn_state.test_passed = s_14_4;
        // N s_14_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #3u : u32
        let s_16_0: u32 = 3;
        // D s_16_1: read-var comparison:u32
        let s_16_1: u32 = fn_state.comparison;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b19 b17
        if s_16_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call FPCR_read(s_17_0)
        let s_17_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_17_0);
        // D s_17_2: read-var zero:bv
        let s_17_2: Bits = fn_state.zero;
        // D s_17_3: read-var element:bv
        let s_17_3: Bits = fn_state.element;
        // D s_17_4: call FPCompareGE(s_17_2, s_17_3, s_17_1)
        let s_17_4: bool = FPCompareGE(state, tracer, s_17_2, s_17_3, s_17_1);
        // D s_17_5: write-var test_passed <= s_17_4
        fn_state.test_passed = s_17_4;
        // N s_17_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #4u : u32
        let s_19_0: u32 = 4;
        // D s_19_1: read-var comparison:u32
        let s_19_1: u32 = fn_state.comparison;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // D s_19_3: not s_19_2
        let s_19_3: bool = !s_19_2;
        // N s_19_4: branch s_19_3 b22 b20
        if s_19_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call FPCR_read(s_20_0)
        let s_20_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_20_0);
        // D s_20_2: read-var zero:bv
        let s_20_2: Bits = fn_state.zero;
        // D s_20_3: read-var element:bv
        let s_20_3: Bits = fn_state.element;
        // D s_20_4: call FPCompareGT(s_20_2, s_20_3, s_20_1)
        let s_20_4: bool = FPCompareGT(state, tracer, s_20_2, s_20_3, s_20_1);
        // D s_20_5: write-var test_passed <= s_20_4
        fn_state.test_passed = s_20_4;
        // N s_20_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var datasizeshadow#1257:i64
        let s_23_0: i64 = fn_state.datasizeshadow_1257;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: cast reint s_23_1 -> i64
        let s_23_2: i64 = (s_23_1 as i64);
        // D s_23_3: read-var d:i64
        let s_23_3: i64 = fn_state.d;
        // D s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_5: read-var result:bv
        let s_23_5: Bits = fn_state.result;
        // D s_23_6: call V_set(s_23_4, s_23_2, s_23_5)
        let s_23_6: () = V_set(state, tracer, s_23_4, s_23_2, s_23_5);
        // N s_23_7: return
        return;
    }
}
