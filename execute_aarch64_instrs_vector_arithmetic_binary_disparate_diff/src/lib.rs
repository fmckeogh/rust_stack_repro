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
use Vpart_read::*;
use Elem_set::*;
use V_set::*;
use V_read::*;
use integer_subrange::*;
use asl_Int::*;
use Zeros::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_disparate_diff<T: Tracer>(
    state: &mut State,
    tracer: &T,
    accumulate: bool,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    part: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        esizeshadow_1803: i64,
        result: Bits,
        operand1: Bits,
        gs_166991: i64,
        operand2: Bits,
        accumulate: bool,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        part: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        accumulate,
        d,
        datasize,
        elements,
        esize,
        m,
        n,
        part,
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
        // D s_0_3: write-var esizeshadow#1803 <= s_0_2
        fn_state.esizeshadow_1803 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPAdvSIMDEnabled64(s_0_4)
        let s_0_5: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasize:i64
        let s_1_0: i64 = fn_state.datasize;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: read-var part:i64
        let s_1_5: i64 = fn_state.part;
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: cast zx s_1_2 -> i
        let s_1_7: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_8: call Vpart_read(s_1_4, s_1_6, s_1_7)
        let s_1_8: Bits = Vpart_read(state, tracer, s_1_4, s_1_6, s_1_7);
        // D s_1_9: write-var operand1 <= s_1_8
        fn_state.operand1 = s_1_8;
        // D s_1_10: read-var datasize:i64
        let s_1_10: i64 = fn_state.datasize;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: read-var m:i64
        let s_1_13: i64 = fn_state.m;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: read-var part:i64
        let s_1_15: i64 = fn_state.part;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_12 -> i
        let s_1_17: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_18: call Vpart_read(s_1_14, s_1_16, s_1_17)
        let s_1_18: Bits = Vpart_read(state, tracer, s_1_14, s_1_16, s_1_17);
        // D s_1_19: write-var operand2 <= s_1_18
        fn_state.operand2 = s_1_18;
        // D s_1_20: read-var accumulate:u8
        let s_1_20: bool = fn_state.accumulate;
        // N s_1_21: branch s_1_20 b7 b2
        if s_1_20 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #2s : i
        let s_2_0: i128 = 2;
        // D s_2_1: read-var datasize:i64
        let s_2_1: i64 = fn_state.datasize;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: mul s_2_0 s_2_2
        let s_2_3: i128 = ((s_2_0) * (s_2_2));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: call Zeros(s_2_5)
        let s_2_6: Bits = Zeros(state, tracer, s_2_5);
        // D s_2_7: write-var result <= s_2_6
        fn_state.result = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var elements:i
        let s_3_2: i128 = fn_state.elements;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#166991 <= s_3_4
        fn_state.gs_166991 = s_3_4;
        // D s_3_6: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#166991:i64
        let s_4_1: i64 = fn_state.gs_166991;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#1803:i64
        let s_5_0: i64 = fn_state.esizeshadow_1803;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var e:i64
        let s_5_3: i64 = fn_state.e;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var operand1:bv
        let s_5_6: Bits = fn_state.operand1;
        // D s_5_7: call Elem_read(s_5_6, s_5_4, s_5_5)
        let s_5_7: Bits = Elem_read(state, tracer, s_5_6, s_5_4, s_5_5);
        // D s_5_8: read-var is_unsigned:u8
        let s_5_8: bool = fn_state.is_unsigned;
        // D s_5_9: call asl_Int(s_5_7, s_5_8)
        let s_5_9: i128 = asl_Int(state, tracer, s_5_7, s_5_8);
        // D s_5_10: read-var esizeshadow#1803:i64
        let s_5_10: i64 = fn_state.esizeshadow_1803;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: cast reint s_5_11 -> i64
        let s_5_12: i64 = (s_5_11 as i64);
        // D s_5_13: read-var e:i64
        let s_5_13: i64 = fn_state.e;
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_15: cast zx s_5_12 -> i
        let s_5_15: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_16: read-var operand2:bv
        let s_5_16: Bits = fn_state.operand2;
        // D s_5_17: call Elem_read(s_5_16, s_5_14, s_5_15)
        let s_5_17: Bits = Elem_read(state, tracer, s_5_16, s_5_14, s_5_15);
        // D s_5_18: read-var is_unsigned:u8
        let s_5_18: bool = fn_state.is_unsigned;
        // D s_5_19: call asl_Int(s_5_17, s_5_18)
        let s_5_19: i128 = asl_Int(state, tracer, s_5_17, s_5_18);
        // D s_5_20: sub s_5_9 s_5_19
        let s_5_20: i128 = ((s_5_9) - (s_5_19));
        // D s_5_21: abs s_5_20
        let s_5_21: i128 = (s_5_20).abs();
        // C s_5_22: const #2s : i
        let s_5_22: i128 = 2;
        // D s_5_23: read-var esizeshadow#1803:i64
        let s_5_23: i64 = fn_state.esizeshadow_1803;
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: mul s_5_22 s_5_24
        let s_5_25: i128 = ((s_5_22) * (s_5_24));
        // D s_5_26: cast reint s_5_25 -> i64
        let s_5_26: i64 = (s_5_25 as i64);
        // C s_5_27: const #1s : i
        let s_5_27: i128 = 1;
        // D s_5_28: cast zx s_5_26 -> i
        let s_5_28: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_29: sub s_5_28 s_5_27
        let s_5_29: i128 = ((s_5_28) - (s_5_27));
        // D s_5_30: cast reint s_5_29 -> i64
        let s_5_30: i64 = (s_5_29 as i64);
        // C s_5_31: const #0s : i
        let s_5_31: i128 = 0;
        // D s_5_32: cast zx s_5_30 -> i
        let s_5_32: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_33: call integer_subrange(s_5_21, s_5_32, s_5_31)
        let s_5_33: Bits = integer_subrange(state, tracer, s_5_21, s_5_32, s_5_31);
        // C s_5_34: const #2s : i
        let s_5_34: i128 = 2;
        // D s_5_35: read-var esizeshadow#1803:i64
        let s_5_35: i64 = fn_state.esizeshadow_1803;
        // D s_5_36: cast zx s_5_35 -> i
        let s_5_36: i128 = (i128::try_from(s_5_35).unwrap());
        // D s_5_37: mul s_5_34 s_5_36
        let s_5_37: i128 = ((s_5_34) * (s_5_36));
        // D s_5_38: cast reint s_5_37 -> i64
        let s_5_38: i64 = (s_5_37 as i64);
        // D s_5_39: cast zx s_5_38 -> i
        let s_5_39: i128 = (i128::try_from(s_5_38).unwrap());
        // D s_5_40: cast reint s_5_39 -> i64
        let s_5_40: i64 = (s_5_39 as i64);
        // C s_5_41: const #2s : i
        let s_5_41: i128 = 2;
        // D s_5_42: read-var esizeshadow#1803:i64
        let s_5_42: i64 = fn_state.esizeshadow_1803;
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (i128::try_from(s_5_42).unwrap());
        // D s_5_44: mul s_5_41 s_5_43
        let s_5_44: i128 = ((s_5_41) * (s_5_43));
        // D s_5_45: cast reint s_5_44 -> i64
        let s_5_45: i64 = (s_5_44 as i64);
        // D s_5_46: cast zx s_5_45 -> i
        let s_5_46: i128 = (i128::try_from(s_5_45).unwrap());
        // D s_5_47: cast reint s_5_46 -> i64
        let s_5_47: i64 = (s_5_46 as i64);
        // D s_5_48: read-var e:i64
        let s_5_48: i64 = fn_state.e;
        // D s_5_49: cast zx s_5_48 -> i
        let s_5_49: i128 = (i128::try_from(s_5_48).unwrap());
        // D s_5_50: cast zx s_5_47 -> i
        let s_5_50: i128 = (i128::try_from(s_5_47).unwrap());
        // D s_5_51: read-var result:bv
        let s_5_51: Bits = fn_state.result;
        // D s_5_52: call Elem_read(s_5_51, s_5_49, s_5_50)
        let s_5_52: Bits = Elem_read(state, tracer, s_5_51, s_5_49, s_5_50);
        // D s_5_53: add s_5_52 s_5_33
        let s_5_53: Bits = (s_5_52 + s_5_33);
        // D s_5_54: read-var e:i64
        let s_5_54: i64 = fn_state.e;
        // D s_5_55: cast zx s_5_54 -> i
        let s_5_55: i128 = (i128::try_from(s_5_54).unwrap());
        // D s_5_56: cast zx s_5_40 -> i
        let s_5_56: i128 = (i128::try_from(s_5_40).unwrap());
        // D s_5_57: read-var result:bv
        let s_5_57: Bits = fn_state.result;
        // D s_5_58: call Elem_set(s_5_57, s_5_55, s_5_56, s_5_53)
        let s_5_58: Bits = Elem_set(state, tracer, s_5_57, s_5_55, s_5_56, s_5_53);
        // D s_5_59: write-var result <= s_5_58
        fn_state.result = s_5_58;
        // D s_5_60: read-var e:i64
        let s_5_60: i64 = fn_state.e;
        // C s_5_61: const #1s : i64
        let s_5_61: i64 = 1;
        // D s_5_62: add s_5_60 s_5_61
        let s_5_62: i64 = (s_5_60 + s_5_61);
        // D s_5_63: write-var e <= s_5_62
        fn_state.e = s_5_62;
        // N s_5_64: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2s : i
        let s_6_0: i128 = 2;
        // D s_6_1: read-var datasize:i64
        let s_6_1: i64 = fn_state.datasize;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: mul s_6_0 s_6_2
        let s_6_3: i128 = ((s_6_0) * (s_6_2));
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: cast reint s_6_5 -> i64
        let s_6_6: i64 = (s_6_5 as i64);
        // D s_6_7: read-var d:i64
        let s_6_7: i64 = fn_state.d;
        // D s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_9: read-var result:bv
        let s_6_9: Bits = fn_state.result;
        // D s_6_10: call V_set(s_6_8, s_6_6, s_6_9)
        let s_6_10: () = V_set(state, tracer, s_6_8, s_6_6, s_6_9);
        // N s_6_11: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i
        let s_7_0: i128 = 2;
        // D s_7_1: read-var datasize:i64
        let s_7_1: i64 = fn_state.datasize;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_0 s_7_2
        let s_7_3: i128 = ((s_7_0) * (s_7_2));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: cast reint s_7_5 -> i64
        let s_7_6: i64 = (s_7_5 as i64);
        // D s_7_7: read-var d:i64
        let s_7_7: i64 = fn_state.d;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: call V_read(s_7_8, s_7_6)
        let s_7_9: Bits = V_read(state, tracer, s_7_8, s_7_6);
        // D s_7_10: write-var result <= s_7_9
        fn_state.result = s_7_9;
        // N s_7_11: jump b3
        return block_3(state, tracer, fn_state);
    }
}
