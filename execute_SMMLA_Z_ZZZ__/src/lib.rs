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
use Z_set::*;
use Elem_set::*;
use Zeros::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use MatMulAdd::*;
use common::*;
pub fn execute_SMMLA_Z_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    m: i64,
    n: i64,
    op1_unsigned: bool,
    op2_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_4141: i64,
        s: i64,
        operand3: Bits,
        gs_216891: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_4140: i64,
        operand2: Bits,
        VL: i64,
        da: i64,
        m: i64,
        n: i64,
        op1_unsigned: bool,
        op2_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        da,
        m,
        n,
        op1_unsigned,
        op2_unsigned,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#4140 <= s_0_2
        fn_state.VLshadow_4140 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckNonStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4140:i64
        let s_1_0: i64 = fn_state.VLshadow_4140;
        // D s_1_1: write-var VLshadow#4141 <= s_1_0
        fn_state.VLshadow_4141 = s_1_0;
        // C s_1_2: const #128s : i
        let s_1_2: i128 = 128;
        // D s_1_3: read-var VLshadow#4141:i64
        let s_1_3: i64 = fn_state.VLshadow_4141;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4141:i64
        let s_1_7: i64 = fn_state.VLshadow_4141;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var n:i64
        let s_1_10: i64 = fn_state.n;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cast zx s_1_9 -> i
        let s_1_12: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_13: call Z_read(s_1_11, s_1_12)
        let s_1_13: Bits = Z_read(state, tracer, s_1_11, s_1_12);
        // D s_1_14: write-var operand1 <= s_1_13
        fn_state.operand1 = s_1_13;
        // D s_1_15: read-var VLshadow#4141:i64
        let s_1_15: i64 = fn_state.VLshadow_4141;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: read-var m:i64
        let s_1_18: i64 = fn_state.m;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: cast zx s_1_17 -> i
        let s_1_20: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_21: call Z_read(s_1_19, s_1_20)
        let s_1_21: Bits = Z_read(state, tracer, s_1_19, s_1_20);
        // D s_1_22: write-var operand2 <= s_1_21
        fn_state.operand2 = s_1_21;
        // D s_1_23: read-var VLshadow#4141:i64
        let s_1_23: i64 = fn_state.VLshadow_4141;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: read-var da:i64
        let s_1_26: i64 = fn_state.da;
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: cast zx s_1_25 -> i
        let s_1_28: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_29: call Z_read(s_1_27, s_1_28)
        let s_1_29: Bits = Z_read(state, tracer, s_1_27, s_1_28);
        // D s_1_30: write-var operand3 <= s_1_29
        fn_state.operand3 = s_1_29;
        // D s_1_31: read-var VLshadow#4141:i64
        let s_1_31: i64 = fn_state.VLshadow_4141;
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_33: call Zeros(s_1_32)
        let s_1_33: Bits = Zeros(state, tracer, s_1_32);
        // D s_1_34: write-var result <= s_1_33
        fn_state.result = s_1_33;
        // C s_1_35: const #0s : i64
        let s_1_35: i64 = 0;
        // C s_1_36: const #1s : i
        let s_1_36: i128 = 1;
        // D s_1_37: cast zx s_1_6 -> i
        let s_1_37: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_38: sub s_1_37 s_1_36
        let s_1_38: i128 = ((s_1_37) - (s_1_36));
        // D s_1_39: cast reint s_1_38 -> i64
        let s_1_39: i64 = (s_1_38 as i64);
        // D s_1_40: write-var gs#216891 <= s_1_39
        fn_state.gs_216891 = s_1_39;
        // D s_1_41: write-var s <= s_1_35
        fn_state.s = s_1_35;
        // N s_1_42: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var s:i64
        let s_2_0: i64 = fn_state.s;
        // D s_2_1: read-var gs#216891:i64
        let s_2_1: i64 = fn_state.gs_216891;
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
        // C s_3_0: const #128s : i64
        let s_3_0: i64 = 128;
        // D s_3_1: read-var s:i64
        let s_3_1: i64 = fn_state.s;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // C s_3_3: cast zx s_3_0 -> i
        let s_3_3: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_4: read-var operand1:bv
        let s_3_4: Bits = fn_state.operand1;
        // D s_3_5: call Elem_read(s_3_4, s_3_2, s_3_3)
        let s_3_5: Bits = Elem_read(state, tracer, s_3_4, s_3_2, s_3_3);
        // D s_3_6: cast reint s_3_5 -> u128
        let s_3_6: u128 = (s_3_5.value() as u128);
        // C s_3_7: const #128s : i64
        let s_3_7: i64 = 128;
        // D s_3_8: read-var s:i64
        let s_3_8: i64 = fn_state.s;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // C s_3_10: cast zx s_3_7 -> i
        let s_3_10: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_11: read-var operand2:bv
        let s_3_11: Bits = fn_state.operand2;
        // D s_3_12: call Elem_read(s_3_11, s_3_9, s_3_10)
        let s_3_12: Bits = Elem_read(state, tracer, s_3_11, s_3_9, s_3_10);
        // D s_3_13: cast reint s_3_12 -> u128
        let s_3_13: u128 = (s_3_12.value() as u128);
        // C s_3_14: const #128s : i64
        let s_3_14: i64 = 128;
        // D s_3_15: read-var s:i64
        let s_3_15: i64 = fn_state.s;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // C s_3_17: cast zx s_3_14 -> i
        let s_3_17: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_18: read-var operand3:bv
        let s_3_18: Bits = fn_state.operand3;
        // D s_3_19: call Elem_read(s_3_18, s_3_16, s_3_17)
        let s_3_19: Bits = Elem_read(state, tracer, s_3_18, s_3_16, s_3_17);
        // D s_3_20: cast reint s_3_19 -> u128
        let s_3_20: u128 = (s_3_19.value() as u128);
        // D s_3_21: cast zx s_3_20 -> bv
        let s_3_21: Bits = Bits::new(s_3_20 as u128, 128u16);
        // D s_3_22: cast zx s_3_6 -> bv
        let s_3_22: Bits = Bits::new(s_3_6 as u128, 128u16);
        // D s_3_23: cast zx s_3_13 -> bv
        let s_3_23: Bits = Bits::new(s_3_13 as u128, 128u16);
        // D s_3_24: read-var op1_unsigned:u8
        let s_3_24: bool = fn_state.op1_unsigned;
        // D s_3_25: read-var op2_unsigned:u8
        let s_3_25: bool = fn_state.op2_unsigned;
        // D s_3_26: call MatMulAdd(s_3_21, s_3_22, s_3_23, s_3_24, s_3_25)
        let s_3_26: Bits = MatMulAdd(
            state,
            tracer,
            s_3_21,
            s_3_22,
            s_3_23,
            s_3_24,
            s_3_25,
        );
        // D s_3_27: cast reint s_3_26 -> u128
        let s_3_27: u128 = (s_3_26.value() as u128);
        // C s_3_28: const #128s : i64
        let s_3_28: i64 = 128;
        // D s_3_29: read-var s:i64
        let s_3_29: i64 = fn_state.s;
        // D s_3_30: cast zx s_3_29 -> i
        let s_3_30: i128 = (i128::try_from(s_3_29).unwrap());
        // C s_3_31: cast zx s_3_28 -> i
        let s_3_31: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_32: cast zx s_3_27 -> bv
        let s_3_32: Bits = Bits::new(s_3_27 as u128, 128u16);
        // D s_3_33: read-var result:bv
        let s_3_33: Bits = fn_state.result;
        // D s_3_34: call Elem_set(s_3_33, s_3_30, s_3_31, s_3_32)
        let s_3_34: Bits = Elem_set(state, tracer, s_3_33, s_3_30, s_3_31, s_3_32);
        // D s_3_35: write-var result <= s_3_34
        fn_state.result = s_3_34;
        // D s_3_36: read-var s:i64
        let s_3_36: i64 = fn_state.s;
        // C s_3_37: const #1s : i64
        let s_3_37: i64 = 1;
        // D s_3_38: add s_3_36 s_3_37
        let s_3_38: i64 = (s_3_36 + s_3_37);
        // D s_3_39: write-var s <= s_3_38
        fn_state.s = s_3_38;
        // N s_3_40: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#4141:i64
        let s_4_0: i64 = fn_state.VLshadow_4141;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var da:i64
        let s_4_3: i64 = fn_state.da;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var result:bv
        let s_4_6: Bits = fn_state.result;
        // D s_4_7: call Z_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = Z_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
