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
use u_shr_int_general::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use SignedSat::*;
use Z_set::*;
use common::*;
pub fn execute_SQSHRNB_Z_ZI__<T: Tracer>(
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
        esizeshadow_3412: i64,
        e: i64,
        gs_203703: i64,
        result: Bits,
        VLshadow_3413: i64,
        VLshadow_3414: i64,
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
        // D s_0_3: write-var esizeshadow#3412 <= s_0_2
        fn_state.esizeshadow_3412 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3413 <= s_0_6
        fn_state.VLshadow_3413 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3413:i64
        let s_1_0: i64 = fn_state.VLshadow_3413;
        // D s_1_1: write-var VLshadow#3414 <= s_1_0
        fn_state.VLshadow_3414 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esizeshadow#3412:i64
        let s_1_3: i64 = fn_state.esizeshadow_3412;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_2 s_1_4
        let s_1_5: i128 = ((s_1_2) * (s_1_4));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#3414:i64
        let s_1_7: i64 = fn_state.VLshadow_3414;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var VLshadow#3414:i64
        let s_1_12: i64 = fn_state.VLshadow_3414;
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
        // D s_1_25: write-var gs#203703 <= s_1_24
        fn_state.gs_203703 = s_1_24;
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
        // D s_2_1: read-var gs#203703:i64
        let s_2_1: i64 = fn_state.gs_203703;
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
        // D s_3_1: read-var esizeshadow#3412:i64
        let s_3_1: i64 = fn_state.esizeshadow_3412;
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
        // D s_3_12: cast sx s_3_11 -> i
        let s_3_12: i128 = {
            let sign_bit = s_3_11.length() - 1;
            let mut result = s_3_11.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: read-var shift:i
        let s_3_15: i128 = fn_state.shift;
        // D s_3_16: call _shr_int_general(s_3_14, s_3_15)
        let s_3_16: i128 = u_shr_int_general(state, tracer, s_3_14, s_3_15);
        // C s_3_17: const #2s : i
        let s_3_17: i128 = 2;
        // D s_3_18: read-var e:i64
        let s_3_18: i64 = fn_state.e;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: mul s_3_17 s_3_19
        let s_3_20: i128 = ((s_3_17) * (s_3_19));
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // C s_3_22: const #0s : i
        let s_3_22: i128 = 0;
        // D s_3_23: cast zx s_3_21 -> i
        let s_3_23: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_24: add s_3_23 s_3_22
        let s_3_24: i128 = (s_3_23 + s_3_22);
        // D s_3_25: cast reint s_3_24 -> i64
        let s_3_25: i64 = (s_3_24 as i64);
        // D s_3_26: read-var esizeshadow#3412:i64
        let s_3_26: i64 = fn_state.esizeshadow_3412;
        // D s_3_27: cast zx s_3_26 -> i
        let s_3_27: i128 = (i128::try_from(s_3_26).unwrap());
        // D s_3_28: cast reint s_3_27 -> i64
        let s_3_28: i64 = (s_3_27 as i64);
        // D s_3_29: read-var esizeshadow#3412:i64
        let s_3_29: i64 = fn_state.esizeshadow_3412;
        // D s_3_30: cast zx s_3_29 -> i
        let s_3_30: i128 = (i128::try_from(s_3_29).unwrap());
        // D s_3_31: cast reint s_3_30 -> i64
        let s_3_31: i64 = (s_3_30 as i64);
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: call SignedSat(s_3_16, s_3_32)
        let s_3_33: Bits = SignedSat(state, tracer, s_3_16, s_3_32);
        // D s_3_34: cast zx s_3_25 -> i
        let s_3_34: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_35: cast zx s_3_28 -> i
        let s_3_35: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_36: read-var result:bv
        let s_3_36: Bits = fn_state.result;
        // D s_3_37: call Elem_set(s_3_36, s_3_34, s_3_35, s_3_33)
        let s_3_37: Bits = Elem_set(state, tracer, s_3_36, s_3_34, s_3_35, s_3_33);
        // D s_3_38: write-var result <= s_3_37
        fn_state.result = s_3_37;
        // C s_3_39: const #2s : i
        let s_3_39: i128 = 2;
        // D s_3_40: read-var e:i64
        let s_3_40: i64 = fn_state.e;
        // D s_3_41: cast zx s_3_40 -> i
        let s_3_41: i128 = (i128::try_from(s_3_40).unwrap());
        // D s_3_42: mul s_3_39 s_3_41
        let s_3_42: i128 = ((s_3_39) * (s_3_41));
        // D s_3_43: cast reint s_3_42 -> i64
        let s_3_43: i64 = (s_3_42 as i64);
        // C s_3_44: const #1s : i
        let s_3_44: i128 = 1;
        // D s_3_45: cast zx s_3_43 -> i
        let s_3_45: i128 = (i128::try_from(s_3_43).unwrap());
        // D s_3_46: add s_3_45 s_3_44
        let s_3_46: i128 = (s_3_45 + s_3_44);
        // D s_3_47: cast reint s_3_46 -> i64
        let s_3_47: i64 = (s_3_46 as i64);
        // D s_3_48: read-var esizeshadow#3412:i64
        let s_3_48: i64 = fn_state.esizeshadow_3412;
        // D s_3_49: cast zx s_3_48 -> i
        let s_3_49: i128 = (i128::try_from(s_3_48).unwrap());
        // D s_3_50: cast reint s_3_49 -> i64
        let s_3_50: i64 = (s_3_49 as i64);
        // D s_3_51: read-var esizeshadow#3412:i64
        let s_3_51: i64 = fn_state.esizeshadow_3412;
        // D s_3_52: cast zx s_3_51 -> i
        let s_3_52: i128 = (i128::try_from(s_3_51).unwrap());
        // D s_3_53: call Zeros(s_3_52)
        let s_3_53: Bits = Zeros(state, tracer, s_3_52);
        // D s_3_54: cast zx s_3_47 -> i
        let s_3_54: i128 = (i128::try_from(s_3_47).unwrap());
        // D s_3_55: cast zx s_3_50 -> i
        let s_3_55: i128 = (i128::try_from(s_3_50).unwrap());
        // D s_3_56: read-var result:bv
        let s_3_56: Bits = fn_state.result;
        // D s_3_57: call Elem_set(s_3_56, s_3_54, s_3_55, s_3_53)
        let s_3_57: Bits = Elem_set(state, tracer, s_3_56, s_3_54, s_3_55, s_3_53);
        // D s_3_58: write-var result <= s_3_57
        fn_state.result = s_3_57;
        // D s_3_59: read-var e:i64
        let s_3_59: i64 = fn_state.e;
        // C s_3_60: const #1s : i64
        let s_3_60: i64 = 1;
        // D s_3_61: add s_3_59 s_3_60
        let s_3_61: i64 = (s_3_59 + s_3_60);
        // D s_3_62: write-var e <= s_3_61
        fn_state.e = s_3_61;
        // N s_3_63: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#3414:i64
        let s_4_0: i64 = fn_state.VLshadow_3414;
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
