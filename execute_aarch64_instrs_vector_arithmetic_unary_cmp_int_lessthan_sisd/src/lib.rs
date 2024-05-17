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
use V_set::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use Zeros::*;
use Ones::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_cmp_int_lessthan_sisd<T: Tracer>(
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
        ga_251350: i64,
        ga_251351: Bits,
        datasizeshadow_1135: i64,
        e: i64,
        element: i128,
        result: Bits,
        test_passed: bool,
        gs_146357: i64,
        esizeshadow_1134: i64,
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
        // D s_0_3: write-var esizeshadow#1134 <= s_0_2
        fn_state.esizeshadow_1134 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1135 <= s_0_6
        fn_state.datasizeshadow_1135 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1135:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1135;
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
        // C s_1_7: const #0s : i64
        let s_1_7: i64 = 0;
        // C s_1_8: const #1s : i
        let s_1_8: i128 = 1;
        // D s_1_9: read-var elements:i
        let s_1_9: i128 = fn_state.elements;
        // D s_1_10: sub s_1_9 s_1_8
        let s_1_10: i128 = ((s_1_9) - (s_1_8));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var gs#146357 <= s_1_11
        fn_state.gs_146357 = s_1_11;
        // D s_1_13: write-var e <= s_1_7
        fn_state.e = s_1_7;
        // N s_1_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#146357:i64
        let s_2_1: i64 = fn_state.gs_146357;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b18 b3
        if s_2_2 {
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
        // D s_3_0: read-var esizeshadow#1134:i64
        let s_3_0: i64 = fn_state.esizeshadow_1134;
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
        // D s_3_8: cast sx s_3_7 -> i
        let s_3_8: i128 = {
            let sign_bit = s_3_7.length() - 1;
            let mut result = s_3_7.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_9: write-var element <= s_3_8
        fn_state.element = s_3_8;
        // C s_3_10: const #0u : u32
        let s_3_10: u32 = 0;
        // D s_3_11: read-var comparison:u32
        let s_3_11: u32 = fn_state.comparison;
        // D s_3_12: cmp-eq s_3_10 s_3_11
        let s_3_12: bool = ((s_3_10) == (s_3_11));
        // D s_3_13: not s_3_12
        let s_3_13: bool = !s_3_12;
        // N s_3_14: branch s_3_13 b9 b4
        if s_3_13 {
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
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var element:i
        let s_4_1: i128 = fn_state.element;
        // D s_4_2: cmp-gt s_4_1 s_4_0
        let s_4_2: bool = ((s_4_1) > (s_4_0));
        // D s_4_3: write-var test_passed <= s_4_2
        fn_state.test_passed = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#1134:i64
        let s_5_0: i64 = fn_state.esizeshadow_1134;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: write-var ga#251350 <= s_5_2
        fn_state.ga_251350 = s_5_2;
        // D s_5_4: read-var test_passed:u8
        let s_5_4: bool = fn_state.test_passed;
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
        // D s_6_0: read-var esizeshadow#1134:i64
        let s_6_0: i64 = fn_state.esizeshadow_1134;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call Zeros(s_6_1)
        let s_6_2: Bits = Zeros(state, tracer, s_6_1);
        // D s_6_3: write-var ga#251351 <= s_6_2
        fn_state.ga_251351 = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var ga#251350:i64
        let s_7_2: i64 = fn_state.ga_251350;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var result:bv
        let s_7_4: Bits = fn_state.result;
        // D s_7_5: read-var ga#251351:bv
        let s_7_5: Bits = fn_state.ga_251351;
        // D s_7_6: call Elem_set(s_7_4, s_7_1, s_7_3, s_7_5)
        let s_7_6: Bits = Elem_set(state, tracer, s_7_4, s_7_1, s_7_3, s_7_5);
        // D s_7_7: write-var result <= s_7_6
        fn_state.result = s_7_6;
        // D s_7_8: read-var e:i64
        let s_7_8: i64 = fn_state.e;
        // C s_7_9: const #1s : i64
        let s_7_9: i64 = 1;
        // D s_7_10: add s_7_8 s_7_9
        let s_7_10: i64 = (s_7_8 + s_7_9);
        // D s_7_11: write-var e <= s_7_10
        fn_state.e = s_7_10;
        // N s_7_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esizeshadow#1134:i64
        let s_8_0: i64 = fn_state.esizeshadow_1134;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call Ones(s_8_1)
        let s_8_2: Bits = Ones(state, tracer, s_8_1);
        // D s_8_3: write-var ga#251351 <= s_8_2
        fn_state.ga_251351 = s_8_2;
        // N s_8_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u32
        let s_9_0: u32 = 1;
        // D s_9_1: read-var comparison:u32
        let s_9_1: u32 = fn_state.comparison;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b11 b10
        if s_9_3 {
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
        // C s_10_0: const #0s : i
        let s_10_0: i128 = 0;
        // D s_10_1: read-var element:i
        let s_10_1: i128 = fn_state.element;
        // D s_10_2: cmp-ge s_10_1 s_10_0
        let s_10_2: bool = ((s_10_1) >= (s_10_0));
        // D s_10_3: write-var test_passed <= s_10_2
        fn_state.test_passed = s_10_2;
        // N s_10_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2u : u32
        let s_11_0: u32 = 2;
        // D s_11_1: read-var comparison:u32
        let s_11_1: u32 = fn_state.comparison;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: not s_11_2
        let s_11_3: bool = !s_11_2;
        // N s_11_4: branch s_11_3 b13 b12
        if s_11_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var element:i
        let s_12_1: i128 = fn_state.element;
        // D s_12_2: cmp-eq s_12_1 s_12_0
        let s_12_2: bool = ((s_12_1) == (s_12_0));
        // D s_12_3: write-var test_passed <= s_12_2
        fn_state.test_passed = s_12_2;
        // N s_12_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #3u : u32
        let s_13_0: u32 = 3;
        // D s_13_1: read-var comparison:u32
        let s_13_1: u32 = fn_state.comparison;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b15 b14
        if s_13_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: read-var element:i
        let s_14_1: i128 = fn_state.element;
        // D s_14_2: cmp-le s_14_1 s_14_0
        let s_14_2: bool = ((s_14_1) <= (s_14_0));
        // D s_14_3: write-var test_passed <= s_14_2
        fn_state.test_passed = s_14_2;
        // N s_14_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #4u : u32
        let s_15_0: u32 = 4;
        // D s_15_1: read-var comparison:u32
        let s_15_1: u32 = fn_state.comparison;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // N s_15_4: branch s_15_3 b17 b16
        if s_15_3 {
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
        // C s_16_0: const #0s : i
        let s_16_0: i128 = 0;
        // D s_16_1: read-var element:i
        let s_16_1: i128 = fn_state.element;
        // D s_16_2: cmp-lt s_16_1 s_16_0
        let s_16_2: bool = ((s_16_1) < (s_16_0));
        // D s_16_3: write-var test_passed <= s_16_2
        fn_state.test_passed = s_16_2;
        // N s_16_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var datasizeshadow#1135:i64
        let s_18_0: i64 = fn_state.datasizeshadow_1135;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: cast reint s_18_1 -> i64
        let s_18_2: i64 = (s_18_1 as i64);
        // D s_18_3: read-var d:i64
        let s_18_3: i64 = fn_state.d;
        // D s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_5: read-var result:bv
        let s_18_5: Bits = fn_state.result;
        // D s_18_6: call V_set(s_18_4, s_18_2, s_18_5)
        let s_18_6: () = V_set(state, tracer, s_18_4, s_18_2, s_18_5);
        // N s_18_7: return
        return;
    }
}
