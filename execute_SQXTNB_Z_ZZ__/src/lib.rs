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
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use SignedSat::*;
use Zeros::*;
use common::*;
pub fn execute_SQXTNB_Z_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        halfesize: i64,
        e: i64,
        esizeshadow_3596: i64,
        gs_205506: i64,
        VLshadow_3597: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_3598: i64,
        VL: i64,
        d: i64,
        esize: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        n,
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
        // D s_0_3: write-var esizeshadow#3596 <= s_0_2
        fn_state.esizeshadow_3596 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3597 <= s_0_6
        fn_state.VLshadow_3597 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3597:i64
        let s_1_0: i64 = fn_state.VLshadow_3597;
        // D s_1_1: write-var VLshadow#3598 <= s_1_0
        fn_state.VLshadow_3598 = s_1_0;
        // D s_1_2: read-var VLshadow#3598:i64
        let s_1_2: i64 = fn_state.VLshadow_3598;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#3596:i64
        let s_1_4: i64 = fn_state.esizeshadow_3596;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#3598:i64
        let s_1_8: i64 = fn_state.VLshadow_3598;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var n:i64
        let s_1_11: i64 = fn_state.n;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast zx s_1_10 -> i
        let s_1_13: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_14: call Z_read(s_1_12, s_1_13)
        let s_1_14: Bits = Z_read(state, tracer, s_1_12, s_1_13);
        // D s_1_15: write-var operand1 <= s_1_14
        fn_state.operand1 = s_1_14;
        // C s_1_16: const #2s : i
        let s_1_16: i128 = 2;
        // D s_1_17: read-var esizeshadow#3596:i64
        let s_1_17: i64 = fn_state.esizeshadow_3596;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: div s_1_18 s_1_16
        let s_1_19: i128 = ((s_1_18) / (s_1_16));
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: write-var halfesize <= s_1_20
        fn_state.halfesize = s_1_20;
        // C s_1_22: const #0s : i64
        let s_1_22: i64 = 0;
        // C s_1_23: const #1s : i
        let s_1_23: i128 = 1;
        // D s_1_24: cast zx s_1_7 -> i
        let s_1_24: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_25: sub s_1_24 s_1_23
        let s_1_25: i128 = ((s_1_24) - (s_1_23));
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: write-var gs#205506 <= s_1_26
        fn_state.gs_205506 = s_1_26;
        // D s_1_28: write-var e <= s_1_22
        fn_state.e = s_1_22;
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
        // D s_2_1: read-var gs#205506:i64
        let s_2_1: i64 = fn_state.gs_205506;
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
        // D s_3_0: read-var esizeshadow#3596:i64
        let s_3_0: i64 = fn_state.esizeshadow_3596;
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
        // D s_3_8: cast sx s_3_7 -> i
        let s_3_8: i128 = {
            let sign_bit = s_3_7.length() - 1;
            let mut result = s_3_7.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: read-var halfesize:i64
        let s_3_10: i64 = fn_state.halfesize;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: cast zx s_3_9 -> i
        let s_3_13: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_14: cast zx s_3_12 -> i
        let s_3_14: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_15: call SignedSat(s_3_13, s_3_14)
        let s_3_15: Bits = SignedSat(state, tracer, s_3_13, s_3_14);
        // C s_3_16: const #2s : i
        let s_3_16: i128 = 2;
        // D s_3_17: read-var e:i64
        let s_3_17: i64 = fn_state.e;
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: mul s_3_16 s_3_18
        let s_3_19: i128 = ((s_3_16) * (s_3_18));
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // C s_3_21: const #0s : i
        let s_3_21: i128 = 0;
        // D s_3_22: cast zx s_3_20 -> i
        let s_3_22: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_23: add s_3_22 s_3_21
        let s_3_23: i128 = (s_3_22 + s_3_21);
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: read-var halfesize:i64
        let s_3_25: i64 = fn_state.halfesize;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // D s_3_28: cast zx s_3_24 -> i
        let s_3_28: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_29: cast zx s_3_27 -> i
        let s_3_29: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_30: read-var result:bv
        let s_3_30: Bits = fn_state.result;
        // D s_3_31: call Elem_set(s_3_30, s_3_28, s_3_29, s_3_15)
        let s_3_31: Bits = Elem_set(state, tracer, s_3_30, s_3_28, s_3_29, s_3_15);
        // D s_3_32: write-var result <= s_3_31
        fn_state.result = s_3_31;
        // C s_3_33: const #2s : i
        let s_3_33: i128 = 2;
        // D s_3_34: read-var e:i64
        let s_3_34: i64 = fn_state.e;
        // D s_3_35: cast zx s_3_34 -> i
        let s_3_35: i128 = (i128::try_from(s_3_34).unwrap());
        // D s_3_36: mul s_3_33 s_3_35
        let s_3_36: i128 = ((s_3_33) * (s_3_35));
        // D s_3_37: cast reint s_3_36 -> i64
        let s_3_37: i64 = (s_3_36 as i64);
        // C s_3_38: const #1s : i
        let s_3_38: i128 = 1;
        // D s_3_39: cast zx s_3_37 -> i
        let s_3_39: i128 = (i128::try_from(s_3_37).unwrap());
        // D s_3_40: add s_3_39 s_3_38
        let s_3_40: i128 = (s_3_39 + s_3_38);
        // D s_3_41: cast reint s_3_40 -> i64
        let s_3_41: i64 = (s_3_40 as i64);
        // D s_3_42: read-var halfesize:i64
        let s_3_42: i64 = fn_state.halfesize;
        // D s_3_43: cast zx s_3_42 -> i
        let s_3_43: i128 = (i128::try_from(s_3_42).unwrap());
        // D s_3_44: cast reint s_3_43 -> i64
        let s_3_44: i64 = (s_3_43 as i64);
        // D s_3_45: read-var halfesize:i64
        let s_3_45: i64 = fn_state.halfesize;
        // D s_3_46: cast zx s_3_45 -> i
        let s_3_46: i128 = (i128::try_from(s_3_45).unwrap());
        // D s_3_47: call Zeros(s_3_46)
        let s_3_47: Bits = Zeros(state, tracer, s_3_46);
        // D s_3_48: cast zx s_3_41 -> i
        let s_3_48: i128 = (i128::try_from(s_3_41).unwrap());
        // D s_3_49: cast zx s_3_44 -> i
        let s_3_49: i128 = (i128::try_from(s_3_44).unwrap());
        // D s_3_50: read-var result:bv
        let s_3_50: Bits = fn_state.result;
        // D s_3_51: call Elem_set(s_3_50, s_3_48, s_3_49, s_3_47)
        let s_3_51: Bits = Elem_set(state, tracer, s_3_50, s_3_48, s_3_49, s_3_47);
        // D s_3_52: write-var result <= s_3_51
        fn_state.result = s_3_51;
        // D s_3_53: read-var e:i64
        let s_3_53: i64 = fn_state.e;
        // C s_3_54: const #1s : i64
        let s_3_54: i64 = 1;
        // D s_3_55: add s_3_53 s_3_54
        let s_3_55: i64 = (s_3_53 + s_3_54);
        // D s_3_56: write-var e <= s_3_55
        fn_state.e = s_3_55;
        // N s_3_57: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#3598:i64
        let s_4_0: i64 = fn_state.VLshadow_3598;
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
