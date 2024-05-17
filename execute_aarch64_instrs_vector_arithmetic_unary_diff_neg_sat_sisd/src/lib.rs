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
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use V_set::*;
use SignedSatQ::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_diff_neg_sat_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    n: i64,
    neg: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        gs_170380: i64,
        e: i64,
        esizeshadow_1895: i64,
        datasizeshadow_1896: i64,
        ga_267134: ProductTypef506aa96a892fe52,
        element: i128,
        result: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        n: i64,
        neg: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        n,
        neg,
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
        // D s_0_3: write-var esizeshadow#1895 <= s_0_2
        fn_state.esizeshadow_1895 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1896 <= s_0_6
        fn_state.datasizeshadow_1896 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1896:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1896;
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
        // D s_1_12: write-var gs#170380 <= s_1_11
        fn_state.gs_170380 = s_1_11;
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
        // D s_2_1: read-var gs#170380:i64
        let s_2_1: i64 = fn_state.gs_170380;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#1895:i64
        let s_3_0: i64 = fn_state.esizeshadow_1895;
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
        // D s_3_10: read-var neg:u8
        let s_3_10: bool = fn_state.neg;
        // N s_3_11: branch s_3_10 b9 b4
        if s_3_10 {
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
        // D s_4_0: read-var element:i
        let s_4_0: i128 = fn_state.element;
        // D s_4_1: abs s_4_0
        let s_4_1: i128 = (s_4_0).abs();
        // D s_4_2: write-var element <= s_4_1
        fn_state.element = s_4_1;
        // N s_4_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#1895:i64
        let s_5_0: i64 = fn_state.esizeshadow_1895;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var element:i
        let s_5_4: i128 = fn_state.element;
        // D s_5_5: call SignedSatQ(s_5_4, s_5_3)
        let s_5_5: ProductTypef506aa96a892fe52 = SignedSatQ(state, tracer, s_5_4, s_5_3);
        // D s_5_6: write-var ga#267134 <= s_5_5
        fn_state.ga_267134 = s_5_5;
        // D s_5_7: read-var ga#267134.0:struct
        let s_5_7: Bits = fn_state.ga_267134._0;
        // D s_5_8: read-var ga#267134.1:struct
        let s_5_8: bool = fn_state.ga_267134._1;
        // D s_5_9: read-var esizeshadow#1895:i64
        let s_5_9: i64 = fn_state.esizeshadow_1895;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // D s_5_12: read-var e:i64
        let s_5_12: i64 = fn_state.e;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: cast zx s_5_11 -> i
        let s_5_14: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_15: read-var result:bv
        let s_5_15: Bits = fn_state.result;
        // D s_5_16: call Elem_set(s_5_15, s_5_13, s_5_14, s_5_7)
        let s_5_16: Bits = Elem_set(state, tracer, s_5_15, s_5_13, s_5_14, s_5_7);
        // D s_5_17: write-var result <= s_5_16
        fn_state.result = s_5_16;
        // N s_5_18: branch s_5_8 b8 b6
        if s_5_8 {
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
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var e <= s_7_2
        fn_state.e = s_7_2;
        // N s_7_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #20696u : u32
        let s_8_0: u32 = 20696;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #20696u : u32
        let s_8_2: u32 = 20696;
        // N s_8_3: write-reg s_8_2 <= s_8_1
        let s_8_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_2 as isize, s_8_1);
            tracer.write_register(s_8_2 as isize, s_8_1);
        };
        // N s_8_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var element:i
        let s_9_0: i128 = fn_state.element;
        // D s_9_1: neg s_9_0
        let s_9_1: i128 = -s_9_0;
        // D s_9_2: write-var element <= s_9_1
        fn_state.element = s_9_1;
        // N s_9_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var datasizeshadow#1896:i64
        let s_10_0: i64 = fn_state.datasizeshadow_1896;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var d:i64
        let s_10_3: i64 = fn_state.d;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: read-var result:bv
        let s_10_5: Bits = fn_state.result;
        // D s_10_6: call V_set(s_10_4, s_10_2, s_10_5)
        let s_10_6: () = V_set(state, tracer, s_10_4, s_10_2, s_10_5);
        // N s_10_7: return
        return;
    }
}
