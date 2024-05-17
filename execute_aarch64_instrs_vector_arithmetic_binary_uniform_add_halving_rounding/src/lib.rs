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
use asl_Int::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use V_set::*;
use integer_subrange::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_add_halving_rounding<
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
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        datasizeshadow_1976: i64,
        result: Bits,
        esizeshadow_1975: i64,
        gs_172518: i64,
        operand1: Bits,
        operand2: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        m,
        n,
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
        // D s_0_3: write-var esizeshadow#1975 <= s_0_2
        fn_state.esizeshadow_1975 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1976 <= s_0_6
        fn_state.datasizeshadow_1976 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1976:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1976;
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
        // D s_1_7: read-var datasizeshadow#1976:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1976;
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
        // D s_1_19: write-var gs#172518 <= s_1_18
        fn_state.gs_172518 = s_1_18;
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
        // D s_2_1: read-var gs#172518:i64
        let s_2_1: i64 = fn_state.gs_172518;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#1975:i64
        let s_3_0: i64 = fn_state.esizeshadow_1975;
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
        // D s_3_10: read-var esizeshadow#1975:i64
        let s_3_10: i64 = fn_state.esizeshadow_1975;
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
        // D s_3_18: read-var is_unsigned:u8
        let s_3_18: bool = fn_state.is_unsigned;
        // D s_3_19: call asl_Int(s_3_17, s_3_18)
        let s_3_19: i128 = asl_Int(state, tracer, s_3_17, s_3_18);
        // D s_3_20: add s_3_9 s_3_19
        let s_3_20: i128 = (s_3_9 + s_3_19);
        // C s_3_21: const #1s : i
        let s_3_21: i128 = 1;
        // D s_3_22: add s_3_20 s_3_21
        let s_3_22: i128 = (s_3_20 + s_3_21);
        // C s_3_23: const #1s : i
        let s_3_23: i128 = 1;
        // D s_3_24: lsr s_3_22 s_3_23
        let s_3_24: i128 = s_3_22 >> s_3_23;
        // D s_3_25: read-var esizeshadow#1975:i64
        let s_3_25: i64 = fn_state.esizeshadow_1975;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // C s_3_28: const #1s : i
        let s_3_28: i128 = 1;
        // D s_3_29: read-var esizeshadow#1975:i64
        let s_3_29: i64 = fn_state.esizeshadow_1975;
        // D s_3_30: cast zx s_3_29 -> i
        let s_3_30: i128 = (i128::try_from(s_3_29).unwrap());
        // D s_3_31: sub s_3_30 s_3_28
        let s_3_31: i128 = ((s_3_30) - (s_3_28));
        // D s_3_32: cast reint s_3_31 -> i64
        let s_3_32: i64 = (s_3_31 as i64);
        // C s_3_33: const #0s : i
        let s_3_33: i128 = 0;
        // D s_3_34: cast zx s_3_32 -> i
        let s_3_34: i128 = (i128::try_from(s_3_32).unwrap());
        // D s_3_35: call integer_subrange(s_3_24, s_3_34, s_3_33)
        let s_3_35: Bits = integer_subrange(state, tracer, s_3_24, s_3_34, s_3_33);
        // D s_3_36: read-var e:i64
        let s_3_36: i64 = fn_state.e;
        // D s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_38: cast zx s_3_27 -> i
        let s_3_38: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_39: read-var result:bv
        let s_3_39: Bits = fn_state.result;
        // D s_3_40: call Elem_set(s_3_39, s_3_37, s_3_38, s_3_35)
        let s_3_40: Bits = Elem_set(state, tracer, s_3_39, s_3_37, s_3_38, s_3_35);
        // D s_3_41: write-var result <= s_3_40
        fn_state.result = s_3_40;
        // D s_3_42: read-var e:i64
        let s_3_42: i64 = fn_state.e;
        // C s_3_43: const #1s : i64
        let s_3_43: i64 = 1;
        // D s_3_44: add s_3_42 s_3_43
        let s_3_44: i64 = (s_3_42 + s_3_43);
        // D s_3_45: write-var e <= s_3_44
        fn_state.e = s_3_44;
        // N s_3_46: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasizeshadow#1976:i64
        let s_4_0: i64 = fn_state.datasizeshadow_1976;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: read-var result:bv
        let s_4_5: Bits = fn_state.result;
        // D s_4_6: call V_set(s_4_4, s_4_2, s_4_5)
        let s_4_6: () = V_set(state, tracer, s_4_4, s_4_2, s_4_5);
        // N s_4_7: return
        return;
    }
}
