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
use CheckSVEEnabled::*;
use u_shl_int_general::*;
use UnsignedSat::*;
use u_shr_int_general::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_UQRSHRNB_Z_ZI__<T: Tracer>(
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
        VLshadow_3438: i64,
        e: i64,
        VLshadow_3437: i64,
        gs_204100: i64,
        esizeshadow_3436: i64,
        result: Bits,
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
        // D s_0_3: write-var esizeshadow#3436 <= s_0_2
        fn_state.esizeshadow_3436 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3437 <= s_0_6
        fn_state.VLshadow_3437 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3437:i64
        let s_1_0: i64 = fn_state.VLshadow_3437;
        // D s_1_1: write-var VLshadow#3438 <= s_1_0
        fn_state.VLshadow_3438 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esizeshadow#3436:i64
        let s_1_3: i64 = fn_state.esizeshadow_3436;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_2 s_1_4
        let s_1_5: i128 = ((s_1_2) * (s_1_4));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#3438:i64
        let s_1_7: i64 = fn_state.VLshadow_3438;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var VLshadow#3438:i64
        let s_1_12: i64 = fn_state.VLshadow_3438;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var n:i64
        let s_1_15: i64 = fn_state.n;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call Z_read(s_1_16, s_1_17)
        let s_1_18: Bits = Z_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var operand <= s_1_18
        fn_state.operand = s_1_18;
        // C s_1_20: const #0s : i64
        let s_1_20: i64 = 0;
        // C s_1_21: const #1s : i
        let s_1_21: i128 = 1;
        // D s_1_22: cast zx s_1_11 -> i
        let s_1_22: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_23: sub s_1_22 s_1_21
        let s_1_23: i128 = ((s_1_22) - (s_1_21));
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: write-var gs#204100 <= s_1_24
        fn_state.gs_204100 = s_1_24;
        // D s_1_26: write-var e <= s_1_20
        fn_state.e = s_1_20;
        // N s_1_27: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#204100:i64
        let s_2_1: i64 = fn_state.gs_204100;
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
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var esizeshadow#3436:i64
        let s_3_1: i64 = fn_state.esizeshadow_3436;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: read-var e:i64
        let s_3_7: i64 = fn_state.e;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cast zx s_3_6 -> i
        let s_3_9: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_10: read-var operand:bv
        let s_3_10: Bits = fn_state.operand;
        // D s_3_11: call Elem_read(s_3_10, s_3_8, s_3_9)
        let s_3_11: Bits = Elem_read(state, tracer, s_3_10, s_3_8, s_3_9);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (s_3_11.value() as i128);
        // C s_3_13: const #1s : i
        let s_3_13: i128 = 1;
        // D s_3_14: read-var shift:i
        let s_3_14: i128 = fn_state.shift;
        // D s_3_15: sub s_3_14 s_3_13
        let s_3_15: i128 = ((s_3_14) - (s_3_13));
        // C s_3_16: const #1s : i
        let s_3_16: i128 = 1;
        // D s_3_17: call _shl_int_general(s_3_16, s_3_15)
        let s_3_17: i128 = u_shl_int_general(state, tracer, s_3_16, s_3_15);
        // D s_3_18: add s_3_12 s_3_17
        let s_3_18: i128 = (s_3_12 + s_3_17);
        // D s_3_19: read-var shift:i
        let s_3_19: i128 = fn_state.shift;
        // D s_3_20: call _shr_int_general(s_3_18, s_3_19)
        let s_3_20: i128 = u_shr_int_general(state, tracer, s_3_18, s_3_19);
        // C s_3_21: const #2s : i
        let s_3_21: i128 = 2;
        // D s_3_22: read-var e:i64
        let s_3_22: i64 = fn_state.e;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: mul s_3_21 s_3_23
        let s_3_24: i128 = ((s_3_21) * (s_3_23));
        // D s_3_25: cast reint s_3_24 -> i64
        let s_3_25: i64 = (s_3_24 as i64);
        // C s_3_26: const #0s : i
        let s_3_26: i128 = 0;
        // D s_3_27: cast zx s_3_25 -> i
        let s_3_27: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_28: add s_3_27 s_3_26
        let s_3_28: i128 = (s_3_27 + s_3_26);
        // D s_3_29: cast reint s_3_28 -> i64
        let s_3_29: i64 = (s_3_28 as i64);
        // D s_3_30: read-var esizeshadow#3436:i64
        let s_3_30: i64 = fn_state.esizeshadow_3436;
        // D s_3_31: cast zx s_3_30 -> i
        let s_3_31: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_32: cast reint s_3_31 -> i64
        let s_3_32: i64 = (s_3_31 as i64);
        // D s_3_33: read-var esizeshadow#3436:i64
        let s_3_33: i64 = fn_state.esizeshadow_3436;
        // D s_3_34: cast zx s_3_33 -> i
        let s_3_34: i128 = (i128::try_from(s_3_33).unwrap());
        // D s_3_35: cast reint s_3_34 -> i64
        let s_3_35: i64 = (s_3_34 as i64);
        // D s_3_36: cast zx s_3_35 -> i
        let s_3_36: i128 = (i128::try_from(s_3_35).unwrap());
        // D s_3_37: call UnsignedSat(s_3_20, s_3_36)
        let s_3_37: Bits = UnsignedSat(state, tracer, s_3_20, s_3_36);
        // D s_3_38: cast zx s_3_29 -> i
        let s_3_38: i128 = (i128::try_from(s_3_29).unwrap());
        // D s_3_39: cast zx s_3_32 -> i
        let s_3_39: i128 = (i128::try_from(s_3_32).unwrap());
        // D s_3_40: read-var result:bv
        let s_3_40: Bits = fn_state.result;
        // D s_3_41: call Elem_set(s_3_40, s_3_38, s_3_39, s_3_37)
        let s_3_41: Bits = Elem_set(state, tracer, s_3_40, s_3_38, s_3_39, s_3_37);
        // D s_3_42: write-var result <= s_3_41
        fn_state.result = s_3_41;
        // C s_3_43: const #2s : i
        let s_3_43: i128 = 2;
        // D s_3_44: read-var e:i64
        let s_3_44: i64 = fn_state.e;
        // D s_3_45: cast zx s_3_44 -> i
        let s_3_45: i128 = (i128::try_from(s_3_44).unwrap());
        // D s_3_46: mul s_3_43 s_3_45
        let s_3_46: i128 = ((s_3_43) * (s_3_45));
        // D s_3_47: cast reint s_3_46 -> i64
        let s_3_47: i64 = (s_3_46 as i64);
        // C s_3_48: const #1s : i
        let s_3_48: i128 = 1;
        // D s_3_49: cast zx s_3_47 -> i
        let s_3_49: i128 = (i128::try_from(s_3_47).unwrap());
        // D s_3_50: add s_3_49 s_3_48
        let s_3_50: i128 = (s_3_49 + s_3_48);
        // D s_3_51: cast reint s_3_50 -> i64
        let s_3_51: i64 = (s_3_50 as i64);
        // D s_3_52: read-var esizeshadow#3436:i64
        let s_3_52: i64 = fn_state.esizeshadow_3436;
        // D s_3_53: cast zx s_3_52 -> i
        let s_3_53: i128 = (i128::try_from(s_3_52).unwrap());
        // D s_3_54: cast reint s_3_53 -> i64
        let s_3_54: i64 = (s_3_53 as i64);
        // D s_3_55: read-var esizeshadow#3436:i64
        let s_3_55: i64 = fn_state.esizeshadow_3436;
        // D s_3_56: cast zx s_3_55 -> i
        let s_3_56: i128 = (i128::try_from(s_3_55).unwrap());
        // D s_3_57: call Zeros(s_3_56)
        let s_3_57: Bits = Zeros(state, tracer, s_3_56);
        // D s_3_58: cast zx s_3_51 -> i
        let s_3_58: i128 = (i128::try_from(s_3_51).unwrap());
        // D s_3_59: cast zx s_3_54 -> i
        let s_3_59: i128 = (i128::try_from(s_3_54).unwrap());
        // D s_3_60: read-var result:bv
        let s_3_60: Bits = fn_state.result;
        // D s_3_61: call Elem_set(s_3_60, s_3_58, s_3_59, s_3_57)
        let s_3_61: Bits = Elem_set(state, tracer, s_3_60, s_3_58, s_3_59, s_3_57);
        // D s_3_62: write-var result <= s_3_61
        fn_state.result = s_3_61;
        // D s_3_63: read-var e:i64
        let s_3_63: i64 = fn_state.e;
        // C s_3_64: const #1s : i64
        let s_3_64: i64 = 1;
        // D s_3_65: add s_3_63 s_3_64
        let s_3_65: i64 = (s_3_63 + s_3_64);
        // D s_3_66: write-var e <= s_3_65
        fn_state.e = s_3_65;
        // N s_3_67: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#3438:i64
        let s_4_0: i64 = fn_state.VLshadow_3438;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
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
