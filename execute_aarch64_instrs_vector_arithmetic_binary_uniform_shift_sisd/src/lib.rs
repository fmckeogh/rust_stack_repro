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
use SatQ::*;
use Elem_set::*;
use V_read::*;
use V_set::*;
use u_shl_int_general::*;
use integer_subrange::*;
use asl_Int::*;
use Elem_read::*;
use RShr::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_shift_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    rounding: bool,
    saturating: bool,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_267813: ProductTypef506aa96a892fe52,
        e: i64,
        datasizeshadow_1948: i64,
        esizeshadow_1947: i64,
        shift: i128,
        gs_171335: i64,
        elementshadow_1949: i128,
        element: i128,
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
        saturating: bool,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        m,
        n,
        rounding,
        saturating,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#1947 <= s_0_2
        fn_state.esizeshadow_1947 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1948 <= s_0_6
        fn_state.datasizeshadow_1948 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1948:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1948;
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
        // D s_1_7: read-var datasizeshadow#1948:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1948;
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
        // C s_1_14: const #0s : i64
        let s_1_14: i64 = 0;
        // C s_1_15: const #1s : i
        let s_1_15: i128 = 1;
        // D s_1_16: read-var elements:i
        let s_1_16: i128 = fn_state.elements;
        // D s_1_17: sub s_1_16 s_1_15
        let s_1_17: i128 = ((s_1_16) - (s_1_15));
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: write-var gs#171335 <= s_1_18
        fn_state.gs_171335 = s_1_18;
        // D s_1_20: write-var e <= s_1_14
        fn_state.e = s_1_14;
        // N s_1_21: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#171335:i64
        let s_2_1: i64 = fn_state.gs_171335;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b13 b3
        if s_2_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#1947:i64
        let s_3_0: i64 = fn_state.esizeshadow_1947;
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
        // D s_3_8: read-var is_unsigned:u8
        let s_3_8: bool = fn_state.is_unsigned;
        // D s_3_9: call asl_Int(s_3_7, s_3_8)
        let s_3_9: i128 = asl_Int(state, tracer, s_3_7, s_3_8);
        // D s_3_10: write-var element <= s_3_9
        fn_state.element = s_3_9;
        // D s_3_11: read-var esizeshadow#1947:i64
        let s_3_11: i64 = fn_state.esizeshadow_1947;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: read-var e:i64
        let s_3_14: i64 = fn_state.e;
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: cast zx s_3_13 -> i
        let s_3_16: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_17: read-var operand2:bv
        let s_3_17: Bits = fn_state.operand2;
        // D s_3_18: call Elem_read(s_3_17, s_3_15, s_3_16)
        let s_3_18: Bits = Elem_read(state, tracer, s_3_17, s_3_15, s_3_16);
        // C s_3_19: const #0s : i
        let s_3_19: i128 = 0;
        // C s_3_20: const #1s : i64
        let s_3_20: i64 = 1;
        // C s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // C s_3_22: const #7s : i
        let s_3_22: i128 = 7;
        // C s_3_23: add s_3_22 s_3_21
        let s_3_23: i128 = (s_3_22 + s_3_21);
        // D s_3_24: bit-extract s_3_18 s_3_19 s_3_23
        let s_3_24: Bits = (Bits::new(
            ((s_3_18) >> (s_3_19)).value(),
            u16::try_from(s_3_23).unwrap(),
        ));
        // D s_3_25: cast reint s_3_24 -> u8
        let s_3_25: u8 = (s_3_24.value() as u8);
        // D s_3_26: cast zx s_3_25 -> bv
        let s_3_26: Bits = Bits::new(s_3_25 as u128, 8u16);
        // D s_3_27: cast sx s_3_26 -> i
        let s_3_27: i128 = {
            let sign_bit = s_3_26.length() - 1;
            let mut result = s_3_26.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_28: write-var shift <= s_3_27
        fn_state.shift = s_3_27;
        // C s_3_29: const #0s : i
        let s_3_29: i128 = 0;
        // D s_3_30: read-var shift:i
        let s_3_30: i128 = fn_state.shift;
        // D s_3_31: cmp-ge s_3_30 s_3_29
        let s_3_31: bool = ((s_3_30) >= (s_3_29));
        // N s_3_32: branch s_3_31 b12 b4
        if s_3_31 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var shift:i
        let s_4_0: i128 = fn_state.shift;
        // D s_4_1: neg s_4_0
        let s_4_1: i128 = -s_4_0;
        // D s_4_2: read-var element:i
        let s_4_2: i128 = fn_state.element;
        // D s_4_3: read-var rounding:u8
        let s_4_3: bool = fn_state.rounding;
        // D s_4_4: call RShr(s_4_2, s_4_1, s_4_3)
        let s_4_4: i128 = RShr(state, tracer, s_4_2, s_4_1, s_4_3);
        // D s_4_5: write-var element <= s_4_4
        fn_state.element = s_4_4;
        // N s_4_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var element:i
        let s_5_0: i128 = fn_state.element;
        // D s_5_1: write-var elementshadow#1949 <= s_5_0
        fn_state.elementshadow_1949 = s_5_0;
        // D s_5_2: read-var saturating:u8
        let s_5_2: bool = fn_state.saturating;
        // N s_5_3: branch s_5_2 b8 b6
        if s_5_2 {
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
        // D s_6_0: read-var esizeshadow#1947:i64
        let s_6_0: i64 = fn_state.esizeshadow_1947;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // C s_6_3: const #1s : i
        let s_6_3: i128 = 1;
        // D s_6_4: read-var esizeshadow#1947:i64
        let s_6_4: i64 = fn_state.esizeshadow_1947;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: sub s_6_5 s_6_3
        let s_6_6: i128 = ((s_6_5) - (s_6_3));
        // D s_6_7: cast reint s_6_6 -> i64
        let s_6_7: i64 = (s_6_6 as i64);
        // C s_6_8: const #0s : i
        let s_6_8: i128 = 0;
        // D s_6_9: cast zx s_6_7 -> i
        let s_6_9: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_10: read-var elementshadow#1949:i
        let s_6_10: i128 = fn_state.elementshadow_1949;
        // D s_6_11: call integer_subrange(s_6_10, s_6_9, s_6_8)
        let s_6_11: Bits = integer_subrange(state, tracer, s_6_10, s_6_9, s_6_8);
        // D s_6_12: read-var e:i64
        let s_6_12: i64 = fn_state.e;
        // D s_6_13: cast zx s_6_12 -> i
        let s_6_13: i128 = (i128::try_from(s_6_12).unwrap());
        // D s_6_14: cast zx s_6_2 -> i
        let s_6_14: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_15: read-var result:bv
        let s_6_15: Bits = fn_state.result;
        // D s_6_16: call Elem_set(s_6_15, s_6_13, s_6_14, s_6_11)
        let s_6_16: Bits = Elem_set(state, tracer, s_6_15, s_6_13, s_6_14, s_6_11);
        // D s_6_17: write-var result <= s_6_16
        fn_state.result = s_6_16;
        // N s_6_18: jump b7
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
        // D s_8_0: read-var esizeshadow#1947:i64
        let s_8_0: i64 = fn_state.esizeshadow_1947;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: read-var elementshadow#1949:i
        let s_8_4: i128 = fn_state.elementshadow_1949;
        // D s_8_5: read-var is_unsigned:u8
        let s_8_5: bool = fn_state.is_unsigned;
        // D s_8_6: call SatQ(s_8_4, s_8_3, s_8_5)
        let s_8_6: ProductTypef506aa96a892fe52 = SatQ(
            state,
            tracer,
            s_8_4,
            s_8_3,
            s_8_5,
        );
        // D s_8_7: write-var ga#267813 <= s_8_6
        fn_state.ga_267813 = s_8_6;
        // D s_8_8: read-var ga#267813.0:struct
        let s_8_8: Bits = fn_state.ga_267813._0;
        // D s_8_9: read-var ga#267813.1:struct
        let s_8_9: bool = fn_state.ga_267813._1;
        // D s_8_10: read-var esizeshadow#1947:i64
        let s_8_10: i64 = fn_state.esizeshadow_1947;
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: cast reint s_8_11 -> i64
        let s_8_12: i64 = (s_8_11 as i64);
        // D s_8_13: read-var e:i64
        let s_8_13: i64 = fn_state.e;
        // D s_8_14: cast zx s_8_13 -> i
        let s_8_14: i128 = (i128::try_from(s_8_13).unwrap());
        // D s_8_15: cast zx s_8_12 -> i
        let s_8_15: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_16: read-var result:bv
        let s_8_16: Bits = fn_state.result;
        // D s_8_17: call Elem_set(s_8_16, s_8_14, s_8_15, s_8_8)
        let s_8_17: Bits = Elem_set(state, tracer, s_8_16, s_8_14, s_8_15, s_8_8);
        // D s_8_18: write-var result <= s_8_17
        fn_state.result = s_8_17;
        // N s_8_19: branch s_8_9 b11 b9
        if s_8_9 {
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
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #20696u : u32
        let s_11_0: u32 = 20696;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // C s_11_2: const #20696u : u32
        let s_11_2: u32 = 20696;
        // N s_11_3: write-reg s_11_2 <= s_11_1
        let s_11_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_11_2 as isize, s_11_1);
            tracer.write_register(s_11_2 as isize, s_11_1);
        };
        // N s_11_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var element:i
        let s_12_0: i128 = fn_state.element;
        // D s_12_1: read-var shift:i
        let s_12_1: i128 = fn_state.shift;
        // D s_12_2: call _shl_int_general(s_12_0, s_12_1)
        let s_12_2: i128 = u_shl_int_general(state, tracer, s_12_0, s_12_1);
        // D s_12_3: write-var element <= s_12_2
        fn_state.element = s_12_2;
        // N s_12_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var datasizeshadow#1948:i64
        let s_13_0: i64 = fn_state.datasizeshadow_1948;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // D s_13_3: read-var d:i64
        let s_13_3: i64 = fn_state.d;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: read-var result:bv
        let s_13_5: Bits = fn_state.result;
        // D s_13_6: call V_set(s_13_4, s_13_2, s_13_5)
        let s_13_6: () = V_set(state, tracer, s_13_4, s_13_2, s_13_5);
        // N s_13_7: return
        return;
    }
}
