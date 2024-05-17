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
use CheckSVEEnabled::*;
use P_read::*;
use ActivePredicateElement::*;
use asl_Int::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_SQDECP_Z_P_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    m: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_2729: i64,
        e: i64,
        gs_190788: i64,
        VLshadow_2730: i64,
        count: i128,
        gs_190799: i64,
        elements: i64,
        countshadow_2731: i128,
        result: Bits,
        u_3243: i64,
        esizeshadow_2728: i64,
        operand1: Bits,
        ga_280944: ProductTypef506aa96a892fe52,
        operand2: Bits,
        VL: i64,
        dn: i64,
        esize: i64,
        m: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        m,
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
        // D s_0_3: write-var esizeshadow#2728 <= s_0_2
        fn_state.esizeshadow_2728 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2729 <= s_0_6
        fn_state.VLshadow_2729 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2729:i64
        let s_1_0: i64 = fn_state.VLshadow_2729;
        // D s_1_1: write-var VLshadow#2730 <= s_1_0
        fn_state.VLshadow_2730 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2730:i64
        let s_1_3: i64 = fn_state.VLshadow_2730;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2730:i64
        let s_1_7: i64 = fn_state.VLshadow_2730;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#2728:i64
        let s_1_9: i64 = fn_state.esizeshadow_2728;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var elements <= s_1_12
        fn_state.elements = s_1_12;
        // D s_1_14: read-var VLshadow#2730:i64
        let s_1_14: i64 = fn_state.VLshadow_2730;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: read-var dn:i64
        let s_1_17: i64 = fn_state.dn;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: cast zx s_1_16 -> i
        let s_1_19: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_20: call Z_read(s_1_18, s_1_19)
        let s_1_20: Bits = Z_read(state, tracer, s_1_18, s_1_19);
        // D s_1_21: write-var operand1 <= s_1_20
        fn_state.operand1 = s_1_20;
        // D s_1_22: cast zx s_1_6 -> i
        let s_1_22: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var m:i64
        let s_1_24: i64 = fn_state.m;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast zx s_1_23 -> i
        let s_1_26: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_27: call P_read(s_1_25, s_1_26)
        let s_1_27: Bits = P_read(state, tracer, s_1_25, s_1_26);
        // D s_1_28: write-var operand2 <= s_1_27
        fn_state.operand2 = s_1_27;
        // C s_1_29: const #0s : i
        let s_1_29: i128 = 0;
        // D s_1_30: write-var count <= s_1_29
        fn_state.count = s_1_29;
        // C s_1_31: const #0s : i64
        let s_1_31: i64 = 0;
        // C s_1_32: const #1s : i
        let s_1_32: i128 = 1;
        // D s_1_33: read-var elements:i64
        let s_1_33: i64 = fn_state.elements;
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: sub s_1_34 s_1_32
        let s_1_35: i128 = ((s_1_34) - (s_1_32));
        // D s_1_36: cast reint s_1_35 -> i64
        let s_1_36: i64 = (s_1_35 as i64);
        // D s_1_37: write-var gs#190788 <= s_1_36
        fn_state.gs_190788 = s_1_36;
        // D s_1_38: write-var e <= s_1_31
        fn_state.e = s_1_31;
        // N s_1_39: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#190788:i64
        let s_2_1: i64 = fn_state.gs_190788;
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
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var esizeshadow#2728:i64
        let s_3_2: i64 = fn_state.esizeshadow_2728;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-var operand2:bv
        let s_3_4: Bits = fn_state.operand2;
        // D s_3_5: call ActivePredicateElement(s_3_4, s_3_1, s_3_3)
        let s_3_5: bool = ActivePredicateElement(state, tracer, s_3_4, s_3_1, s_3_3);
        // N s_3_6: branch s_3_5 b6 b4
        if s_3_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // C s_5_1: const #1s : i64
        let s_5_1: i64 = 1;
        // D s_5_2: add s_5_0 s_5_1
        let s_5_2: i64 = (s_5_0 + s_5_1);
        // D s_5_3: write-var e <= s_5_2
        fn_state.e = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1s : i
        let s_6_0: i128 = 1;
        // D s_6_1: read-var count:i
        let s_6_1: i128 = fn_state.count;
        // D s_6_2: add s_6_1 s_6_0
        let s_6_2: i128 = (s_6_1 + s_6_0);
        // D s_6_3: write-var count <= s_6_2
        fn_state.count = s_6_2;
        // N s_6_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var count:i
        let s_7_0: i128 = fn_state.count;
        // D s_7_1: write-var countshadow#2731 <= s_7_0
        fn_state.countshadow_2731 = s_7_0;
        // C s_7_2: const #0s : i64
        let s_7_2: i64 = 0;
        // C s_7_3: const #1s : i
        let s_7_3: i128 = 1;
        // D s_7_4: read-var elements:i64
        let s_7_4: i64 = fn_state.elements;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: sub s_7_5 s_7_3
        let s_7_6: i128 = ((s_7_5) - (s_7_3));
        // D s_7_7: cast reint s_7_6 -> i64
        let s_7_7: i64 = (s_7_6 as i64);
        // D s_7_8: write-var gs#190799 <= s_7_7
        fn_state.gs_190799 = s_7_7;
        // D s_7_9: write-var u#3243 <= s_7_2
        fn_state.u_3243 = s_7_2;
        // N s_7_10: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var u#3243:i64
        let s_8_0: i64 = fn_state.u_3243;
        // D s_8_1: read-var gs#190799:i64
        let s_8_1: i64 = fn_state.gs_190799;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b10 b9
        if s_8_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esizeshadow#2728:i64
        let s_9_0: i64 = fn_state.esizeshadow_2728;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var u#3243:i64
        let s_9_3: i64 = fn_state.u_3243;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cast zx s_9_2 -> i
        let s_9_5: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_6: read-var operand1:bv
        let s_9_6: Bits = fn_state.operand1;
        // D s_9_7: call Elem_read(s_9_6, s_9_4, s_9_5)
        let s_9_7: Bits = Elem_read(state, tracer, s_9_6, s_9_4, s_9_5);
        // D s_9_8: read-var is_unsigned:u8
        let s_9_8: bool = fn_state.is_unsigned;
        // D s_9_9: call asl_Int(s_9_7, s_9_8)
        let s_9_9: i128 = asl_Int(state, tracer, s_9_7, s_9_8);
        // D s_9_10: read-var countshadow#2731:i
        let s_9_10: i128 = fn_state.countshadow_2731;
        // D s_9_11: sub s_9_9 s_9_10
        let s_9_11: i128 = ((s_9_9) - (s_9_10));
        // D s_9_12: read-var esizeshadow#2728:i64
        let s_9_12: i64 = fn_state.esizeshadow_2728;
        // D s_9_13: cast zx s_9_12 -> i
        let s_9_13: i128 = (i128::try_from(s_9_12).unwrap());
        // D s_9_14: cast reint s_9_13 -> i64
        let s_9_14: i64 = (s_9_13 as i64);
        // D s_9_15: cast zx s_9_14 -> i
        let s_9_15: i128 = (i128::try_from(s_9_14).unwrap());
        // D s_9_16: read-var is_unsigned:u8
        let s_9_16: bool = fn_state.is_unsigned;
        // D s_9_17: call SatQ(s_9_11, s_9_15, s_9_16)
        let s_9_17: ProductTypef506aa96a892fe52 = SatQ(
            state,
            tracer,
            s_9_11,
            s_9_15,
            s_9_16,
        );
        // D s_9_18: write-var ga#280944 <= s_9_17
        fn_state.ga_280944 = s_9_17;
        // D s_9_19: read-var ga#280944.0:struct
        let s_9_19: Bits = fn_state.ga_280944._0;
        // D s_9_20: read-var esizeshadow#2728:i64
        let s_9_20: i64 = fn_state.esizeshadow_2728;
        // D s_9_21: cast zx s_9_20 -> i
        let s_9_21: i128 = (i128::try_from(s_9_20).unwrap());
        // D s_9_22: cast reint s_9_21 -> i64
        let s_9_22: i64 = (s_9_21 as i64);
        // D s_9_23: read-var u#3243:i64
        let s_9_23: i64 = fn_state.u_3243;
        // D s_9_24: cast zx s_9_23 -> i
        let s_9_24: i128 = (i128::try_from(s_9_23).unwrap());
        // D s_9_25: cast zx s_9_22 -> i
        let s_9_25: i128 = (i128::try_from(s_9_22).unwrap());
        // D s_9_26: read-var result:bv
        let s_9_26: Bits = fn_state.result;
        // D s_9_27: call Elem_set(s_9_26, s_9_24, s_9_25, s_9_19)
        let s_9_27: Bits = Elem_set(state, tracer, s_9_26, s_9_24, s_9_25, s_9_19);
        // D s_9_28: write-var result <= s_9_27
        fn_state.result = s_9_27;
        // D s_9_29: read-var u#3243:i64
        let s_9_29: i64 = fn_state.u_3243;
        // C s_9_30: const #1s : i64
        let s_9_30: i64 = 1;
        // D s_9_31: add s_9_29 s_9_30
        let s_9_31: i64 = (s_9_29 + s_9_30);
        // D s_9_32: write-var u#3243 <= s_9_31
        fn_state.u_3243 = s_9_31;
        // N s_9_33: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#2730:i64
        let s_10_0: i64 = fn_state.VLshadow_2730;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var dn:i64
        let s_10_3: i64 = fn_state.dn;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: read-var result:bv
        let s_10_6: Bits = fn_state.result;
        // D s_10_7: call Z_set(s_10_4, s_10_5, s_10_6)
        let s_10_7: () = Z_set(state, tracer, s_10_4, s_10_5, s_10_6);
        // N s_10_8: return
        return;
    }
}
