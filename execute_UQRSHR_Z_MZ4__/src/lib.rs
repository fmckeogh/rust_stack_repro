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
use CheckStreamingSVEEnabled::*;
use Elem_set::*;
use UnsignedSat::*;
use u_shl_int_general::*;
use u_shr_int_general::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_UQRSHR_Z_MZ4__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    n: i64,
    shift: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        r: i64,
        e: i64,
        VLshadow_6758: i64,
        esizeshadow_6757: i64,
        VLshadow_6759: i64,
        elements: i64,
        result: Bits,
        gs_287314: i64,
        VL: i64,
        d: i64,
        esize: i64,
        n: i64,
        shift: i128,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        n,
        shift,
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
        // D s_0_3: write-var esizeshadow#6757 <= s_0_2
        fn_state.esizeshadow_6757 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6758 <= s_0_6
        fn_state.VLshadow_6758 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckStreamingSVEEnabled(s_0_8)
        let s_0_9: () = CheckStreamingSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#6758:i64
        let s_1_0: i64 = fn_state.VLshadow_6758;
        // D s_1_1: write-var VLshadow#6759 <= s_1_0
        fn_state.VLshadow_6759 = s_1_0;
        // C s_1_2: const #4s : i
        let s_1_2: i128 = 4;
        // D s_1_3: read-var esizeshadow#6757:i64
        let s_1_3: i64 = fn_state.esizeshadow_6757;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_2 s_1_4
        let s_1_5: i128 = ((s_1_2) * (s_1_4));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#6759:i64
        let s_1_7: i64 = fn_state.VLshadow_6759;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var elements <= s_1_11
        fn_state.elements = s_1_11;
        // C s_1_13: const #0s : i64
        let s_1_13: i64 = 0;
        // D s_1_14: write-var r <= s_1_13
        fn_state.r = s_1_13;
        // N s_1_15: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // C s_2_1: const #3s : i64
        let s_2_1: i64 = 3;
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
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var r:i64
        let s_3_2: i64 = fn_state.r;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: add s_3_1 s_3_3
        let s_3_4: i128 = (s_3_1 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var VLshadow#6759:i64
        let s_3_6: i64 = fn_state.VLshadow_6759;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: cast zx s_3_5 -> i
        let s_3_9: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_10: cast zx s_3_8 -> i
        let s_3_10: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_11: call Z_read(s_3_9, s_3_10)
        let s_3_11: Bits = Z_read(state, tracer, s_3_9, s_3_10);
        // D s_3_12: write-var operand <= s_3_11
        fn_state.operand = s_3_11;
        // C s_3_13: const #0s : i64
        let s_3_13: i64 = 0;
        // C s_3_14: const #1s : i
        let s_3_14: i128 = 1;
        // D s_3_15: read-var elements:i64
        let s_3_15: i64 = fn_state.elements;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: sub s_3_16 s_3_14
        let s_3_17: i128 = ((s_3_16) - (s_3_14));
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: write-var gs#287314 <= s_3_18
        fn_state.gs_287314 = s_3_18;
        // D s_3_20: write-var e <= s_3_13
        fn_state.e = s_3_13;
        // N s_3_21: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#287314:i64
        let s_4_1: i64 = fn_state.gs_287314;
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
        // D s_5_1: read-var esizeshadow#6757:i64
        let s_5_1: i64 = fn_state.esizeshadow_6757;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // D s_5_7: read-var e:i64
        let s_5_7: i64 = fn_state.e;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: cast zx s_5_6 -> i
        let s_5_9: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_10: read-var operand:bv
        let s_5_10: Bits = fn_state.operand;
        // D s_5_11: call Elem_read(s_5_10, s_5_8, s_5_9)
        let s_5_11: Bits = Elem_read(state, tracer, s_5_10, s_5_8, s_5_9);
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (s_5_11.value() as i128);
        // C s_5_13: const #1s : i
        let s_5_13: i128 = 1;
        // D s_5_14: read-var shift:i
        let s_5_14: i128 = fn_state.shift;
        // D s_5_15: sub s_5_14 s_5_13
        let s_5_15: i128 = ((s_5_14) - (s_5_13));
        // C s_5_16: const #1s : i
        let s_5_16: i128 = 1;
        // D s_5_17: call _shl_int_general(s_5_16, s_5_15)
        let s_5_17: i128 = u_shl_int_general(state, tracer, s_5_16, s_5_15);
        // D s_5_18: add s_5_12 s_5_17
        let s_5_18: i128 = (s_5_12 + s_5_17);
        // D s_5_19: read-var shift:i
        let s_5_19: i128 = fn_state.shift;
        // D s_5_20: call _shr_int_general(s_5_18, s_5_19)
        let s_5_20: i128 = u_shr_int_general(state, tracer, s_5_18, s_5_19);
        // D s_5_21: read-var r:i64
        let s_5_21: i64 = fn_state.r;
        // D s_5_22: cast zx s_5_21 -> i
        let s_5_22: i128 = (i128::try_from(s_5_21).unwrap());
        // D s_5_23: read-var elements:i64
        let s_5_23: i64 = fn_state.elements;
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: mul s_5_22 s_5_24
        let s_5_25: i128 = ((s_5_22) * (s_5_24));
        // D s_5_26: cast reint s_5_25 -> i64
        let s_5_26: i64 = (s_5_25 as i64);
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_28: read-var e:i64
        let s_5_28: i64 = fn_state.e;
        // D s_5_29: cast zx s_5_28 -> i
        let s_5_29: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_30: add s_5_27 s_5_29
        let s_5_30: i128 = (s_5_27 + s_5_29);
        // D s_5_31: cast reint s_5_30 -> i64
        let s_5_31: i64 = (s_5_30 as i64);
        // D s_5_32: read-var esizeshadow#6757:i64
        let s_5_32: i64 = fn_state.esizeshadow_6757;
        // D s_5_33: cast zx s_5_32 -> i
        let s_5_33: i128 = (i128::try_from(s_5_32).unwrap());
        // D s_5_34: cast reint s_5_33 -> i64
        let s_5_34: i64 = (s_5_33 as i64);
        // D s_5_35: read-var esizeshadow#6757:i64
        let s_5_35: i64 = fn_state.esizeshadow_6757;
        // D s_5_36: cast zx s_5_35 -> i
        let s_5_36: i128 = (i128::try_from(s_5_35).unwrap());
        // D s_5_37: cast reint s_5_36 -> i64
        let s_5_37: i64 = (s_5_36 as i64);
        // D s_5_38: cast zx s_5_37 -> i
        let s_5_38: i128 = (i128::try_from(s_5_37).unwrap());
        // D s_5_39: call UnsignedSat(s_5_20, s_5_38)
        let s_5_39: Bits = UnsignedSat(state, tracer, s_5_20, s_5_38);
        // D s_5_40: cast zx s_5_31 -> i
        let s_5_40: i128 = (i128::try_from(s_5_31).unwrap());
        // D s_5_41: cast zx s_5_34 -> i
        let s_5_41: i128 = (i128::try_from(s_5_34).unwrap());
        // D s_5_42: read-var result:bv
        let s_5_42: Bits = fn_state.result;
        // D s_5_43: call Elem_set(s_5_42, s_5_40, s_5_41, s_5_39)
        let s_5_43: Bits = Elem_set(state, tracer, s_5_42, s_5_40, s_5_41, s_5_39);
        // D s_5_44: write-var result <= s_5_43
        fn_state.result = s_5_43;
        // D s_5_45: read-var e:i64
        let s_5_45: i64 = fn_state.e;
        // C s_5_46: const #1s : i64
        let s_5_46: i64 = 1;
        // D s_5_47: add s_5_45 s_5_46
        let s_5_47: i64 = (s_5_45 + s_5_46);
        // D s_5_48: write-var e <= s_5_47
        fn_state.e = s_5_47;
        // N s_5_49: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var r:i64
        let s_6_0: i64 = fn_state.r;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var r <= s_6_2
        fn_state.r = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VLshadow#6759:i64
        let s_7_0: i64 = fn_state.VLshadow_6759;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast zx s_7_2 -> i
        let s_7_5: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_6: read-var result:bv
        let s_7_6: Bits = fn_state.result;
        // D s_7_7: call Z_set(s_7_4, s_7_5, s_7_6)
        let s_7_7: () = Z_set(state, tracer, s_7_4, s_7_5, s_7_6);
        // N s_7_8: return
        return;
    }
}