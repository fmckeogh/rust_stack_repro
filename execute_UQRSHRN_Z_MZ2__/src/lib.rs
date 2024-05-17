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
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_UQRSHRN_Z_MZ2__<T: Tracer>(
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
        VLshadow_4339: i64,
        gs_219874: i64,
        e: i64,
        VLshadow_4340: i64,
        result: Bits,
        i: i64,
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
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#4339 <= s_0_2
        fn_state.VLshadow_4339 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckSVEEnabled(s_0_4)
        let s_0_5: () = CheckSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4339:i64
        let s_1_0: i64 = fn_state.VLshadow_4339;
        // D s_1_1: write-var VLshadow#4340 <= s_1_0
        fn_state.VLshadow_4340 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esize:i64
        let s_1_3: i64 = fn_state.esize;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_2 s_1_4
        let s_1_5: i128 = ((s_1_2) * (s_1_4));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4340:i64
        let s_1_7: i64 = fn_state.VLshadow_4340;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // C s_1_12: const #0s : i64
        let s_1_12: i64 = 0;
        // C s_1_13: const #1s : i
        let s_1_13: i128 = 1;
        // D s_1_14: cast zx s_1_11 -> i
        let s_1_14: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_15: sub s_1_14 s_1_13
        let s_1_15: i128 = ((s_1_14) - (s_1_13));
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: write-var gs#219874 <= s_1_16
        fn_state.gs_219874 = s_1_16;
        // D s_1_18: write-var e <= s_1_12
        fn_state.e = s_1_12;
        // N s_1_19: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#219874:i64
        let s_2_1: i64 = fn_state.gs_219874;
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
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // D s_3_1: write-var i <= s_3_0
        fn_state.i = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #1s : i64
        let s_4_1: i64 = 1;
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
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var i:i64
        let s_5_2: i64 = fn_state.i;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: i128 = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: read-var VLshadow#4340:i64
        let s_5_6: i64 = fn_state.VLshadow_4340;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // D s_5_9: cast zx s_5_5 -> i
        let s_5_9: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_10: cast zx s_5_8 -> i
        let s_5_10: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_11: call Z_read(s_5_9, s_5_10)
        let s_5_11: Bits = Z_read(state, tracer, s_5_9, s_5_10);
        // C s_5_12: const #2s : i
        let s_5_12: i128 = 2;
        // D s_5_13: read-var esize:i64
        let s_5_13: i64 = fn_state.esize;
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_15: mul s_5_12 s_5_14
        let s_5_15: i128 = ((s_5_12) * (s_5_14));
        // D s_5_16: cast reint s_5_15 -> i64
        let s_5_16: i64 = (s_5_15 as i64);
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_18: cast reint s_5_17 -> i64
        let s_5_18: i64 = (s_5_17 as i64);
        // D s_5_19: read-var e:i64
        let s_5_19: i64 = fn_state.e;
        // D s_5_20: cast zx s_5_19 -> i
        let s_5_20: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_21: cast zx s_5_18 -> i
        let s_5_21: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_22: call Elem_read(s_5_11, s_5_20, s_5_21)
        let s_5_22: Bits = Elem_read(state, tracer, s_5_11, s_5_20, s_5_21);
        // D s_5_23: cast zx s_5_22 -> i
        let s_5_23: i128 = (s_5_22.value() as i128);
        // D s_5_24: cast reint s_5_23 -> i64
        let s_5_24: i64 = (s_5_23 as i64);
        // C s_5_25: const #1s : i
        let s_5_25: i128 = 1;
        // D s_5_26: read-var shift:i
        let s_5_26: i128 = fn_state.shift;
        // D s_5_27: sub s_5_26 s_5_25
        let s_5_27: i128 = ((s_5_26) - (s_5_25));
        // C s_5_28: const #1s : i
        let s_5_28: i128 = 1;
        // D s_5_29: call _shl_int_general(s_5_28, s_5_27)
        let s_5_29: i128 = u_shl_int_general(state, tracer, s_5_28, s_5_27);
        // D s_5_30: cast zx s_5_24 -> i
        let s_5_30: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_31: add s_5_30 s_5_29
        let s_5_31: i128 = (s_5_30 + s_5_29);
        // D s_5_32: read-var shift:i
        let s_5_32: i128 = fn_state.shift;
        // D s_5_33: call _shr_int_general(s_5_31, s_5_32)
        let s_5_33: i128 = u_shr_int_general(state, tracer, s_5_31, s_5_32);
        // C s_5_34: const #2s : i
        let s_5_34: i128 = 2;
        // D s_5_35: read-var e:i64
        let s_5_35: i64 = fn_state.e;
        // D s_5_36: cast zx s_5_35 -> i
        let s_5_36: i128 = (i128::try_from(s_5_35).unwrap());
        // D s_5_37: mul s_5_34 s_5_36
        let s_5_37: i128 = ((s_5_34) * (s_5_36));
        // D s_5_38: cast reint s_5_37 -> i64
        let s_5_38: i64 = (s_5_37 as i64);
        // D s_5_39: cast zx s_5_38 -> i
        let s_5_39: i128 = (i128::try_from(s_5_38).unwrap());
        // D s_5_40: read-var i:i64
        let s_5_40: i64 = fn_state.i;
        // D s_5_41: cast zx s_5_40 -> i
        let s_5_41: i128 = (i128::try_from(s_5_40).unwrap());
        // D s_5_42: add s_5_39 s_5_41
        let s_5_42: i128 = (s_5_39 + s_5_41);
        // D s_5_43: cast reint s_5_42 -> i64
        let s_5_43: i64 = (s_5_42 as i64);
        // D s_5_44: read-var esize:i64
        let s_5_44: i64 = fn_state.esize;
        // D s_5_45: cast zx s_5_44 -> i
        let s_5_45: i128 = (i128::try_from(s_5_44).unwrap());
        // D s_5_46: cast reint s_5_45 -> i64
        let s_5_46: i64 = (s_5_45 as i64);
        // D s_5_47: read-var esize:i64
        let s_5_47: i64 = fn_state.esize;
        // D s_5_48: cast zx s_5_47 -> i
        let s_5_48: i128 = (i128::try_from(s_5_47).unwrap());
        // D s_5_49: cast reint s_5_48 -> i64
        let s_5_49: i64 = (s_5_48 as i64);
        // D s_5_50: cast zx s_5_49 -> i
        let s_5_50: i128 = (i128::try_from(s_5_49).unwrap());
        // D s_5_51: call UnsignedSat(s_5_33, s_5_50)
        let s_5_51: Bits = UnsignedSat(state, tracer, s_5_33, s_5_50);
        // D s_5_52: cast reint s_5_51 -> u16
        let s_5_52: u16 = (s_5_51.value() as u16);
        // D s_5_53: cast zx s_5_43 -> i
        let s_5_53: i128 = (i128::try_from(s_5_43).unwrap());
        // D s_5_54: cast zx s_5_46 -> i
        let s_5_54: i128 = (i128::try_from(s_5_46).unwrap());
        // D s_5_55: cast zx s_5_52 -> bv
        let s_5_55: Bits = Bits::new(s_5_52 as u128, 16u16);
        // D s_5_56: read-var result:bv
        let s_5_56: Bits = fn_state.result;
        // D s_5_57: call Elem_set(s_5_56, s_5_53, s_5_54, s_5_55)
        let s_5_57: Bits = Elem_set(state, tracer, s_5_56, s_5_53, s_5_54, s_5_55);
        // D s_5_58: write-var result <= s_5_57
        fn_state.result = s_5_57;
        // D s_5_59: read-var i:i64
        let s_5_59: i64 = fn_state.i;
        // C s_5_60: const #1s : i64
        let s_5_60: i64 = 1;
        // D s_5_61: add s_5_59 s_5_60
        let s_5_61: i64 = (s_5_59 + s_5_60);
        // D s_5_62: write-var i <= s_5_61
        fn_state.i = s_5_61;
        // N s_5_63: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var e <= s_6_2
        fn_state.e = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VLshadow#4340:i64
        let s_7_0: i64 = fn_state.VLshadow_4340;
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
