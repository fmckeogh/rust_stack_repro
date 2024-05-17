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
use RShr::*;
use SignedSatQ::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    rounding: bool,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: i128,
        e: i64,
        element1: i128,
        element3: i128,
        operand3: Bits,
        ga_267644: ProductTypef506aa96a892fe52,
        accum: i128,
        esizeshadow_1934: i64,
        gs_171110: i64,
        datasizeshadow_1935: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        rounding: bool,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        m,
        n,
        rounding,
        sub_op,
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
        // D s_0_3: write-var esizeshadow#1934 <= s_0_2
        fn_state.esizeshadow_1934 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1935 <= s_0_6
        fn_state.datasizeshadow_1935 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1935:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1935;
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
        // D s_1_6: write-var operand1 <= s_1_5
        fn_state.operand1 = s_1_5;
        // D s_1_7: read-var datasizeshadow#1935:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1935;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var m:i64
        let s_1_10: i64 = fn_state.m;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: call V_read(s_1_11, s_1_9)
        let s_1_12: Bits = V_read(state, tracer, s_1_11, s_1_9);
        // D s_1_13: write-var operand2 <= s_1_12
        fn_state.operand2 = s_1_12;
        // D s_1_14: read-var datasizeshadow#1935:i64
        let s_1_14: i64 = fn_state.datasizeshadow_1935;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: read-var d:i64
        let s_1_17: i64 = fn_state.d;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: call V_read(s_1_18, s_1_16)
        let s_1_19: Bits = V_read(state, tracer, s_1_18, s_1_16);
        // D s_1_20: write-var operand3 <= s_1_19
        fn_state.operand3 = s_1_19;
        // C s_1_21: const #0s : i64
        let s_1_21: i64 = 0;
        // C s_1_22: const #1s : i
        let s_1_22: i128 = 1;
        // D s_1_23: read-var elements:i
        let s_1_23: i128 = fn_state.elements;
        // D s_1_24: sub s_1_23 s_1_22
        let s_1_24: i128 = ((s_1_23) - (s_1_22));
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: write-var gs#171110 <= s_1_25
        fn_state.gs_171110 = s_1_25;
        // D s_1_27: write-var e <= s_1_21
        fn_state.e = s_1_21;
        // N s_1_28: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#171110:i64
        let s_2_1: i64 = fn_state.gs_171110;
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
        // D s_3_0: read-var esizeshadow#1934:i64
        let s_3_0: i64 = fn_state.esizeshadow_1934;
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
        // D s_3_6: read-var operand1:bv
        let s_3_6: Bits = fn_state.operand1;
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
        // D s_3_9: write-var element1 <= s_3_8
        fn_state.element1 = s_3_8;
        // D s_3_10: read-var esizeshadow#1934:i64
        let s_3_10: i64 = fn_state.esizeshadow_1934;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: read-var e:i64
        let s_3_13: i64 = fn_state.e;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: cast zx s_3_12 -> i
        let s_3_15: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_16: read-var operand2:bv
        let s_3_16: Bits = fn_state.operand2;
        // D s_3_17: call Elem_read(s_3_16, s_3_14, s_3_15)
        let s_3_17: Bits = Elem_read(state, tracer, s_3_16, s_3_14, s_3_15);
        // D s_3_18: cast sx s_3_17 -> i
        let s_3_18: i128 = {
            let sign_bit = s_3_17.length() - 1;
            let mut result = s_3_17.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_19: write-var element2 <= s_3_18
        fn_state.element2 = s_3_18;
        // D s_3_20: read-var esizeshadow#1934:i64
        let s_3_20: i64 = fn_state.esizeshadow_1934;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: cast reint s_3_21 -> i64
        let s_3_22: i64 = (s_3_21 as i64);
        // D s_3_23: read-var e:i64
        let s_3_23: i64 = fn_state.e;
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: cast zx s_3_22 -> i
        let s_3_25: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_26: read-var operand3:bv
        let s_3_26: Bits = fn_state.operand3;
        // D s_3_27: call Elem_read(s_3_26, s_3_24, s_3_25)
        let s_3_27: Bits = Elem_read(state, tracer, s_3_26, s_3_24, s_3_25);
        // D s_3_28: cast sx s_3_27 -> i
        let s_3_28: i128 = {
            let sign_bit = s_3_27.length() - 1;
            let mut result = s_3_27.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_29: write-var element3 <= s_3_28
        fn_state.element3 = s_3_28;
        // D s_3_30: read-var sub_op:u8
        let s_3_30: bool = fn_state.sub_op;
        // N s_3_31: branch s_3_30 b9 b4
        if s_3_30 {
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
        // D s_4_0: read-var esizeshadow#1934:i64
        let s_4_0: i64 = fn_state.esizeshadow_1934;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var element3:i
        let s_4_2: i128 = fn_state.element3;
        // D s_4_3: lsl s_4_2 s_4_1
        let s_4_3: i128 = s_4_2 << s_4_1;
        // D s_4_4: read-var element1:i
        let s_4_4: i128 = fn_state.element1;
        // D s_4_5: read-var element2:i
        let s_4_5: i128 = fn_state.element2;
        // D s_4_6: mul s_4_4 s_4_5
        let s_4_6: i128 = ((s_4_4) * (s_4_5));
        // C s_4_7: const #2s : i
        let s_4_7: i128 = 2;
        // D s_4_8: mul s_4_7 s_4_6
        let s_4_8: i128 = ((s_4_7) * (s_4_6));
        // D s_4_9: add s_4_3 s_4_8
        let s_4_9: i128 = (s_4_3 + s_4_8);
        // D s_4_10: write-var accum <= s_4_9
        fn_state.accum = s_4_9;
        // N s_4_11: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#1934:i64
        let s_5_0: i64 = fn_state.esizeshadow_1934;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var accum:i
        let s_5_2: i128 = fn_state.accum;
        // D s_5_3: read-var rounding:u8
        let s_5_3: bool = fn_state.rounding;
        // D s_5_4: call RShr(s_5_2, s_5_1, s_5_3)
        let s_5_4: i128 = RShr(state, tracer, s_5_2, s_5_1, s_5_3);
        // D s_5_5: write-var accum <= s_5_4
        fn_state.accum = s_5_4;
        // D s_5_6: read-var esizeshadow#1934:i64
        let s_5_6: i64 = fn_state.esizeshadow_1934;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: read-var accum:i
        let s_5_10: i128 = fn_state.accum;
        // D s_5_11: call SignedSatQ(s_5_10, s_5_9)
        let s_5_11: ProductTypef506aa96a892fe52 = SignedSatQ(
            state,
            tracer,
            s_5_10,
            s_5_9,
        );
        // D s_5_12: write-var ga#267644 <= s_5_11
        fn_state.ga_267644 = s_5_11;
        // D s_5_13: read-var ga#267644.0:struct
        let s_5_13: Bits = fn_state.ga_267644._0;
        // D s_5_14: read-var ga#267644.1:struct
        let s_5_14: bool = fn_state.ga_267644._1;
        // D s_5_15: read-var esizeshadow#1934:i64
        let s_5_15: i64 = fn_state.esizeshadow_1934;
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_17: cast reint s_5_16 -> i64
        let s_5_17: i64 = (s_5_16 as i64);
        // D s_5_18: read-var e:i64
        let s_5_18: i64 = fn_state.e;
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: cast zx s_5_17 -> i
        let s_5_20: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_21: read-var result:bv
        let s_5_21: Bits = fn_state.result;
        // D s_5_22: call Elem_set(s_5_21, s_5_19, s_5_20, s_5_13)
        let s_5_22: Bits = Elem_set(state, tracer, s_5_21, s_5_19, s_5_20, s_5_13);
        // D s_5_23: write-var result <= s_5_22
        fn_state.result = s_5_22;
        // N s_5_24: branch s_5_14 b8 b6
        if s_5_14 {
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
        // D s_9_0: read-var esizeshadow#1934:i64
        let s_9_0: i64 = fn_state.esizeshadow_1934;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var element3:i
        let s_9_2: i128 = fn_state.element3;
        // D s_9_3: lsl s_9_2 s_9_1
        let s_9_3: i128 = s_9_2 << s_9_1;
        // D s_9_4: read-var element1:i
        let s_9_4: i128 = fn_state.element1;
        // D s_9_5: read-var element2:i
        let s_9_5: i128 = fn_state.element2;
        // D s_9_6: mul s_9_4 s_9_5
        let s_9_6: i128 = ((s_9_4) * (s_9_5));
        // C s_9_7: const #2s : i
        let s_9_7: i128 = 2;
        // D s_9_8: mul s_9_7 s_9_6
        let s_9_8: i128 = ((s_9_7) * (s_9_6));
        // D s_9_9: sub s_9_3 s_9_8
        let s_9_9: i128 = ((s_9_3) - (s_9_8));
        // D s_9_10: write-var accum <= s_9_9
        fn_state.accum = s_9_9;
        // N s_9_11: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var datasizeshadow#1935:i64
        let s_10_0: i64 = fn_state.datasizeshadow_1935;
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
