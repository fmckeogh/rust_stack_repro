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
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use integer_subrange::*;
use Z_set::*;
use common::*;
pub fn execute_SMIN_Z_ZI__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    imm: i128,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_185656: i64,
        e: i64,
        VLshadow_2495: i64,
        esizeshadow_2493: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_2494: i64,
        VL: i64,
        dn: i64,
        esize: i64,
        imm: i128,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        imm,
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
        // D s_0_3: write-var esizeshadow#2493 <= s_0_2
        fn_state.esizeshadow_2493 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2494 <= s_0_6
        fn_state.VLshadow_2494 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSVEEnabled(s_0_8)
        let s_0_9: () = CheckSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2494:i64
        let s_1_0: i64 = fn_state.VLshadow_2494;
        // D s_1_1: write-var VLshadow#2495 <= s_1_0
        fn_state.VLshadow_2495 = s_1_0;
        // D s_1_2: read-var VLshadow#2495:i64
        let s_1_2: i64 = fn_state.VLshadow_2495;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#2493:i64
        let s_1_4: i64 = fn_state.esizeshadow_2493;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#2495:i64
        let s_1_8: i64 = fn_state.VLshadow_2495;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var dn:i64
        let s_1_11: i64 = fn_state.dn;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast zx s_1_10 -> i
        let s_1_13: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_14: call Z_read(s_1_12, s_1_13)
        let s_1_14: Bits = Z_read(state, tracer, s_1_12, s_1_13);
        // D s_1_15: write-var operand1 <= s_1_14
        fn_state.operand1 = s_1_14;
        // C s_1_16: const #0s : i64
        let s_1_16: i64 = 0;
        // C s_1_17: const #1s : i
        let s_1_17: i128 = 1;
        // D s_1_18: cast zx s_1_7 -> i
        let s_1_18: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_19: sub s_1_18 s_1_17
        let s_1_19: i128 = ((s_1_18) - (s_1_17));
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: write-var gs#185656 <= s_1_20
        fn_state.gs_185656 = s_1_20;
        // D s_1_22: write-var e <= s_1_16
        fn_state.e = s_1_16;
        // N s_1_23: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#185656:i64
        let s_2_1: i64 = fn_state.gs_185656;
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
        // D s_3_0: read-var esizeshadow#2493:i64
        let s_3_0: i64 = fn_state.esizeshadow_2493;
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
        // D s_3_10: read-var esizeshadow#2493:i64
        let s_3_10: i64 = fn_state.esizeshadow_2493;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: read-var imm:i
        let s_3_13: i128 = fn_state.imm;
        // D s_3_14: cmp-lt s_3_9 s_3_13
        let s_3_14: bool = ((s_3_9) < (s_3_13));
        // D s_3_15: select s_3_14 s_3_9 s_3_13
        let s_3_15: i128 = if s_3_14 { s_3_9 } else { s_3_13 };
        // C s_3_16: const #1s : i
        let s_3_16: i128 = 1;
        // D s_3_17: read-var esizeshadow#2493:i64
        let s_3_17: i64 = fn_state.esizeshadow_2493;
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: sub s_3_18 s_3_16
        let s_3_19: i128 = ((s_3_18) - (s_3_16));
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // C s_3_21: const #0s : i
        let s_3_21: i128 = 0;
        // D s_3_22: cast zx s_3_20 -> i
        let s_3_22: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_23: call integer_subrange(s_3_15, s_3_22, s_3_21)
        let s_3_23: Bits = integer_subrange(state, tracer, s_3_15, s_3_22, s_3_21);
        // D s_3_24: read-var e:i64
        let s_3_24: i64 = fn_state.e;
        // D s_3_25: cast zx s_3_24 -> i
        let s_3_25: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_26: cast zx s_3_12 -> i
        let s_3_26: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_27: read-var result:bv
        let s_3_27: Bits = fn_state.result;
        // D s_3_28: call Elem_set(s_3_27, s_3_25, s_3_26, s_3_23)
        let s_3_28: Bits = Elem_set(state, tracer, s_3_27, s_3_25, s_3_26, s_3_23);
        // D s_3_29: write-var result <= s_3_28
        fn_state.result = s_3_28;
        // D s_3_30: read-var e:i64
        let s_3_30: i64 = fn_state.e;
        // C s_3_31: const #1s : i64
        let s_3_31: i64 = 1;
        // D s_3_32: add s_3_30 s_3_31
        let s_3_32: i64 = (s_3_30 + s_3_31);
        // D s_3_33: write-var e <= s_3_32
        fn_state.e = s_3_32;
        // N s_3_34: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2495:i64
        let s_4_0: i64 = fn_state.VLshadow_2495;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var dn:i64
        let s_4_3: i64 = fn_state.dn;
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