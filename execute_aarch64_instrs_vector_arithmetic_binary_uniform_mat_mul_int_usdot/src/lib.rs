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
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_mat_mul_int_usdot<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        operand3: Bits,
        datasizeshadow_2012: i64,
        b: i64,
        result: Bits,
        res: u32,
        operand1: Bits,
        gs_174266: i64,
        operand2: Bits,
        d: i64,
        datasize: i64,
        elements: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#2012 <= s_0_2
        fn_state.datasizeshadow_2012 = s_0_2;
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
        // D s_1_0: read-var datasizeshadow#2012:i64
        let s_1_0: i64 = fn_state.datasizeshadow_2012;
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
        // D s_1_7: read-var datasizeshadow#2012:i64
        let s_1_7: i64 = fn_state.datasizeshadow_2012;
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
        // D s_1_14: read-var datasizeshadow#2012:i64
        let s_1_14: i64 = fn_state.datasizeshadow_2012;
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
        // D s_1_23: read-var elements:i64
        let s_1_23: i64 = fn_state.elements;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: sub s_1_24 s_1_22
        let s_1_25: i128 = ((s_1_24) - (s_1_22));
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: write-var gs#174266 <= s_1_26
        fn_state.gs_174266 = s_1_26;
        // D s_1_28: write-var e <= s_1_21
        fn_state.e = s_1_21;
        // N s_1_29: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#174266:i64
        let s_2_1: i64 = fn_state.gs_174266;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #32s : i64
        let s_3_0: i64 = 32;
        // D s_3_1: read-var e:i64
        let s_3_1: i64 = fn_state.e;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // C s_3_3: cast zx s_3_0 -> i
        let s_3_3: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_4: read-var operand3:bv
        let s_3_4: Bits = fn_state.operand3;
        // D s_3_5: call Elem_read(s_3_4, s_3_2, s_3_3)
        let s_3_5: Bits = Elem_read(state, tracer, s_3_4, s_3_2, s_3_3);
        // D s_3_6: cast reint s_3_5 -> u32
        let s_3_6: u32 = (s_3_5.value() as u32);
        // D s_3_7: write-var res <= s_3_6
        fn_state.res = s_3_6;
        // C s_3_8: const #0s : i64
        let s_3_8: i64 = 0;
        // D s_3_9: write-var b <= s_3_8
        fn_state.b = s_3_8;
        // N s_3_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var b:i64
        let s_4_0: i64 = fn_state.b;
        // C s_4_1: const #3s : i64
        let s_4_1: i64 = 3;
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
        // C s_5_0: const #4s : i
        let s_5_0: i128 = 4;
        // D s_5_1: read-var e:i64
        let s_5_1: i64 = fn_state.e;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: read-var b:i64
        let s_5_6: i64 = fn_state.b;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: add s_5_5 s_5_7
        let s_5_8: i128 = (s_5_5 + s_5_7);
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // C s_5_10: const #8s : i64
        let s_5_10: i64 = 8;
        // D s_5_11: cast zx s_5_9 -> i
        let s_5_11: i128 = (i128::try_from(s_5_9).unwrap());
        // C s_5_12: cast zx s_5_10 -> i
        let s_5_12: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_13: read-var operand1:bv
        let s_5_13: Bits = fn_state.operand1;
        // D s_5_14: call Elem_read(s_5_13, s_5_11, s_5_12)
        let s_5_14: Bits = Elem_read(state, tracer, s_5_13, s_5_11, s_5_12);
        // D s_5_15: cast reint s_5_14 -> u8
        let s_5_15: u8 = (s_5_14.value() as u8);
        // D s_5_16: cast zx s_5_15 -> bv
        let s_5_16: Bits = Bits::new(s_5_15 as u128, 8u16);
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (s_5_16.value() as i128);
        // D s_5_18: cast reint s_5_17 -> i64
        let s_5_18: i64 = (s_5_17 as i64);
        // C s_5_19: const #4s : i
        let s_5_19: i128 = 4;
        // D s_5_20: read-var e:i64
        let s_5_20: i64 = fn_state.e;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: mul s_5_19 s_5_21
        let s_5_22: i128 = ((s_5_19) * (s_5_21));
        // D s_5_23: cast reint s_5_22 -> i64
        let s_5_23: i64 = (s_5_22 as i64);
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: read-var b:i64
        let s_5_25: i64 = fn_state.b;
        // D s_5_26: cast zx s_5_25 -> i
        let s_5_26: i128 = (i128::try_from(s_5_25).unwrap());
        // D s_5_27: add s_5_24 s_5_26
        let s_5_27: i128 = (s_5_24 + s_5_26);
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // C s_5_29: const #8s : i64
        let s_5_29: i64 = 8;
        // D s_5_30: cast zx s_5_28 -> i
        let s_5_30: i128 = (i128::try_from(s_5_28).unwrap());
        // C s_5_31: cast zx s_5_29 -> i
        let s_5_31: i128 = (i128::try_from(s_5_29).unwrap());
        // D s_5_32: read-var operand2:bv
        let s_5_32: Bits = fn_state.operand2;
        // D s_5_33: call Elem_read(s_5_32, s_5_30, s_5_31)
        let s_5_33: Bits = Elem_read(state, tracer, s_5_32, s_5_30, s_5_31);
        // D s_5_34: cast reint s_5_33 -> u8
        let s_5_34: u8 = (s_5_33.value() as u8);
        // D s_5_35: cast zx s_5_34 -> bv
        let s_5_35: Bits = Bits::new(s_5_34 as u128, 8u16);
        // D s_5_36: cast sx s_5_35 -> i
        let s_5_36: i128 = {
            let sign_bit = s_5_35.length() - 1;
            let mut result = s_5_35.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_37: cast reint s_5_36 -> i64
        let s_5_37: i64 = (s_5_36 as i64);
        // D s_5_38: cast zx s_5_18 -> i
        let s_5_38: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_39: cast zx s_5_37 -> i
        let s_5_39: i128 = (i128::try_from(s_5_37).unwrap());
        // D s_5_40: mul s_5_38 s_5_39
        let s_5_40: i128 = ((s_5_38) * (s_5_39));
        // D s_5_41: cast reint s_5_40 -> i64
        let s_5_41: i64 = (s_5_40 as i64);
        // D s_5_42: read-var res:u32
        let s_5_42: u32 = fn_state.res;
        // D s_5_43: cast zx s_5_42 -> bv
        let s_5_43: Bits = Bits::new(s_5_42 as u128, 32u16);
        // D s_5_44: cast zx s_5_41 -> i
        let s_5_44: i128 = (i128::try_from(s_5_41).unwrap());
        // D s_5_45: cast cvt s_5_44 -> bv
        let s_5_45: Bits = Bits::new(s_5_44 as u128, 128);
        // D s_5_46: add s_5_43 s_5_45
        let s_5_46: Bits = (s_5_43 + s_5_45);
        // D s_5_47: cast reint s_5_46 -> u32
        let s_5_47: u32 = (s_5_46.value() as u32);
        // D s_5_48: write-var res <= s_5_47
        fn_state.res = s_5_47;
        // D s_5_49: read-var b:i64
        let s_5_49: i64 = fn_state.b;
        // C s_5_50: const #1s : i64
        let s_5_50: i64 = 1;
        // D s_5_51: add s_5_49 s_5_50
        let s_5_51: i64 = (s_5_49 + s_5_50);
        // D s_5_52: write-var b <= s_5_51
        fn_state.b = s_5_51;
        // N s_5_53: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #32s : i64
        let s_6_0: i64 = 32;
        // D s_6_1: read-var e:i64
        let s_6_1: i64 = fn_state.e;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // C s_6_3: cast zx s_6_0 -> i
        let s_6_3: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_4: read-var res:u32
        let s_6_4: u32 = fn_state.res;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 32u16);
        // D s_6_6: read-var result:bv
        let s_6_6: Bits = fn_state.result;
        // D s_6_7: call Elem_set(s_6_6, s_6_2, s_6_3, s_6_5)
        let s_6_7: Bits = Elem_set(state, tracer, s_6_6, s_6_2, s_6_3, s_6_5);
        // D s_6_8: write-var result <= s_6_7
        fn_state.result = s_6_7;
        // D s_6_9: read-var e:i64
        let s_6_9: i64 = fn_state.e;
        // C s_6_10: const #1s : i64
        let s_6_10: i64 = 1;
        // D s_6_11: add s_6_9 s_6_10
        let s_6_11: i64 = (s_6_9 + s_6_10);
        // D s_6_12: write-var e <= s_6_11
        fn_state.e = s_6_11;
        // N s_6_13: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasizeshadow#2012:i64
        let s_7_0: i64 = fn_state.datasizeshadow_2012;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: read-var result:bv
        let s_7_5: Bits = fn_state.result;
        // D s_7_6: call V_set(s_7_4, s_7_2, s_7_5)
        let s_7_6: () = V_set(state, tracer, s_7_4, s_7_2, s_7_5);
        // N s_7_7: return
        return;
    }
}
