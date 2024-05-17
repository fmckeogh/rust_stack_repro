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
use FPCR_read::*;
use FPMatMulAdd::*;
use Zeros::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_FMMLA_Z_ZZZ_D<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_2393: i64,
        operand3: Bits,
        VLshadow_2392: i64,
        res: Bits,
        s: i64,
        gs_183394: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
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
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#2392 <= s_0_2
        fn_state.VLshadow_2392 = s_0_2;
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
        // D s_1_0: read-var VLshadow#2392:i64
        let s_1_0: i64 = fn_state.VLshadow_2392;
        // D s_1_1: write-var VLshadow#2393 <= s_1_0
        fn_state.VLshadow_2393 = s_1_0;
        // C s_1_2: const #4s : i
        let s_1_2: i128 = 4;
        // D s_1_3: read-var esize:i64
        let s_1_3: i64 = fn_state.esize;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) * (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2393:i64
        let s_1_7: i64 = fn_state.VLshadow_2393;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: cmp-lt s_1_8 s_1_9
        let s_1_10: bool = ((s_1_8) < (s_1_9));
        // N s_1_11: branch s_1_10 b7 b2
        if s_1_10 {
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
        // C s_2_0: const #4s : i
        let s_2_0: i128 = 4;
        // D s_2_1: read-var esize:i64
        let s_2_1: i64 = fn_state.esize;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: mul s_2_0 s_2_2
        let s_2_3: i128 = ((s_2_0) * (s_2_2));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: read-var VLshadow#2393:i64
        let s_2_5: i64 = fn_state.VLshadow_2393;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: cast zx s_2_4 -> i
        let s_2_7: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_8: div s_2_6 s_2_7
        let s_2_8: i128 = ((s_2_6) / (s_2_7));
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: read-var VLshadow#2393:i64
        let s_2_10: i64 = fn_state.VLshadow_2393;
        // D s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (i128::try_from(s_2_10).unwrap());
        // D s_2_12: cast reint s_2_11 -> i64
        let s_2_12: i64 = (s_2_11 as i64);
        // D s_2_13: read-var n:i64
        let s_2_13: i64 = fn_state.n;
        // D s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_15: cast zx s_2_12 -> i
        let s_2_15: i128 = (i128::try_from(s_2_12).unwrap());
        // D s_2_16: call Z_read(s_2_14, s_2_15)
        let s_2_16: Bits = Z_read(state, tracer, s_2_14, s_2_15);
        // D s_2_17: write-var operand1 <= s_2_16
        fn_state.operand1 = s_2_16;
        // D s_2_18: read-var VLshadow#2393:i64
        let s_2_18: i64 = fn_state.VLshadow_2393;
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (i128::try_from(s_2_18).unwrap());
        // D s_2_20: cast reint s_2_19 -> i64
        let s_2_20: i64 = (s_2_19 as i64);
        // D s_2_21: read-var m:i64
        let s_2_21: i64 = fn_state.m;
        // D s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_23: cast zx s_2_20 -> i
        let s_2_23: i128 = (i128::try_from(s_2_20).unwrap());
        // D s_2_24: call Z_read(s_2_22, s_2_23)
        let s_2_24: Bits = Z_read(state, tracer, s_2_22, s_2_23);
        // D s_2_25: write-var operand2 <= s_2_24
        fn_state.operand2 = s_2_24;
        // D s_2_26: read-var VLshadow#2393:i64
        let s_2_26: i64 = fn_state.VLshadow_2393;
        // D s_2_27: cast zx s_2_26 -> i
        let s_2_27: i128 = (i128::try_from(s_2_26).unwrap());
        // D s_2_28: cast reint s_2_27 -> i64
        let s_2_28: i64 = (s_2_27 as i64);
        // D s_2_29: read-var da:i64
        let s_2_29: i64 = fn_state.da;
        // D s_2_30: cast zx s_2_29 -> i
        let s_2_30: i128 = (i128::try_from(s_2_29).unwrap());
        // D s_2_31: cast zx s_2_28 -> i
        let s_2_31: i128 = (i128::try_from(s_2_28).unwrap());
        // D s_2_32: call Z_read(s_2_30, s_2_31)
        let s_2_32: Bits = Z_read(state, tracer, s_2_30, s_2_31);
        // D s_2_33: write-var operand3 <= s_2_32
        fn_state.operand3 = s_2_32;
        // D s_2_34: read-var VLshadow#2393:i64
        let s_2_34: i64 = fn_state.VLshadow_2393;
        // D s_2_35: cast zx s_2_34 -> i
        let s_2_35: i128 = (i128::try_from(s_2_34).unwrap());
        // D s_2_36: call Zeros(s_2_35)
        let s_2_36: Bits = Zeros(state, tracer, s_2_35);
        // D s_2_37: write-var result <= s_2_36
        fn_state.result = s_2_36;
        // C s_2_38: const #0s : i64
        let s_2_38: i64 = 0;
        // C s_2_39: const #1s : i
        let s_2_39: i128 = 1;
        // D s_2_40: cast zx s_2_9 -> i
        let s_2_40: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_41: sub s_2_40 s_2_39
        let s_2_41: i128 = ((s_2_40) - (s_2_39));
        // D s_2_42: cast reint s_2_41 -> i64
        let s_2_42: i64 = (s_2_41 as i64);
        // D s_2_43: write-var gs#183394 <= s_2_42
        fn_state.gs_183394 = s_2_42;
        // D s_2_44: write-var s <= s_2_38
        fn_state.s = s_2_38;
        // N s_2_45: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var s:i64
        let s_3_0: i64 = fn_state.s;
        // D s_3_1: read-var gs#183394:i64
        let s_3_1: i64 = fn_state.gs_183394;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b6 b4
        if s_3_2 {
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
        // C s_4_0: const #4s : i
        let s_4_0: i128 = 4;
        // D s_4_1: read-var esize:i64
        let s_4_1: i64 = fn_state.esize;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: mul s_4_0 s_4_2
        let s_4_3: i128 = ((s_4_0) * (s_4_2));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: cast reint s_4_5 -> i64
        let s_4_6: i64 = (s_4_5 as i64);
        // D s_4_7: read-var s:i64
        let s_4_7: i64 = fn_state.s;
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_9: cast zx s_4_6 -> i
        let s_4_9: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_10: read-var operand1:bv
        let s_4_10: Bits = fn_state.operand1;
        // D s_4_11: call Elem_read(s_4_10, s_4_8, s_4_9)
        let s_4_11: Bits = Elem_read(state, tracer, s_4_10, s_4_8, s_4_9);
        // C s_4_12: const #4s : i
        let s_4_12: i128 = 4;
        // D s_4_13: read-var esize:i64
        let s_4_13: i64 = fn_state.esize;
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_15: mul s_4_12 s_4_14
        let s_4_15: i128 = ((s_4_12) * (s_4_14));
        // D s_4_16: cast reint s_4_15 -> i64
        let s_4_16: i64 = (s_4_15 as i64);
        // D s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // D s_4_18: cast reint s_4_17 -> i64
        let s_4_18: i64 = (s_4_17 as i64);
        // D s_4_19: read-var s:i64
        let s_4_19: i64 = fn_state.s;
        // D s_4_20: cast zx s_4_19 -> i
        let s_4_20: i128 = (i128::try_from(s_4_19).unwrap());
        // D s_4_21: cast zx s_4_18 -> i
        let s_4_21: i128 = (i128::try_from(s_4_18).unwrap());
        // D s_4_22: read-var operand2:bv
        let s_4_22: Bits = fn_state.operand2;
        // D s_4_23: call Elem_read(s_4_22, s_4_20, s_4_21)
        let s_4_23: Bits = Elem_read(state, tracer, s_4_22, s_4_20, s_4_21);
        // C s_4_24: const #4s : i
        let s_4_24: i128 = 4;
        // D s_4_25: read-var esize:i64
        let s_4_25: i64 = fn_state.esize;
        // D s_4_26: cast zx s_4_25 -> i
        let s_4_26: i128 = (i128::try_from(s_4_25).unwrap());
        // D s_4_27: mul s_4_24 s_4_26
        let s_4_27: i128 = ((s_4_24) * (s_4_26));
        // D s_4_28: cast reint s_4_27 -> i64
        let s_4_28: i64 = (s_4_27 as i64);
        // D s_4_29: cast zx s_4_28 -> i
        let s_4_29: i128 = (i128::try_from(s_4_28).unwrap());
        // D s_4_30: cast reint s_4_29 -> i64
        let s_4_30: i64 = (s_4_29 as i64);
        // D s_4_31: read-var s:i64
        let s_4_31: i64 = fn_state.s;
        // D s_4_32: cast zx s_4_31 -> i
        let s_4_32: i128 = (i128::try_from(s_4_31).unwrap());
        // D s_4_33: cast zx s_4_30 -> i
        let s_4_33: i128 = (i128::try_from(s_4_30).unwrap());
        // D s_4_34: read-var operand3:bv
        let s_4_34: Bits = fn_state.operand3;
        // D s_4_35: call Elem_read(s_4_34, s_4_32, s_4_33)
        let s_4_35: Bits = Elem_read(state, tracer, s_4_34, s_4_32, s_4_33);
        // D s_4_36: read-var esize:i64
        let s_4_36: i64 = fn_state.esize;
        // D s_4_37: cast zx s_4_36 -> i
        let s_4_37: i128 = (i128::try_from(s_4_36).unwrap());
        // D s_4_38: cast reint s_4_37 -> i64
        let s_4_38: i64 = (s_4_37 as i64);
        // C s_4_39: const #() : ()
        let s_4_39: () = ();
        // S s_4_40: call FPCR_read(s_4_39)
        let s_4_40: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_4_39);
        // D s_4_41: cast zx s_4_38 -> i
        let s_4_41: i128 = (i128::try_from(s_4_38).unwrap());
        // D s_4_42: call FPMatMulAdd(s_4_35, s_4_11, s_4_23, s_4_41, s_4_40)
        let s_4_42: Bits = FPMatMulAdd(
            state,
            tracer,
            s_4_35,
            s_4_11,
            s_4_23,
            s_4_41,
            s_4_40,
        );
        // D s_4_43: write-var res <= s_4_42
        fn_state.res = s_4_42;
        // N s_4_44: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #4s : i
        let s_5_0: i128 = 4;
        // D s_5_1: read-var esize:i64
        let s_5_1: i64 = fn_state.esize;
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
        // D s_5_7: read-var s:i64
        let s_5_7: i64 = fn_state.s;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: cast zx s_5_6 -> i
        let s_5_9: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_10: read-var result:bv
        let s_5_10: Bits = fn_state.result;
        // D s_5_11: read-var res:bv
        let s_5_11: Bits = fn_state.res;
        // D s_5_12: call Elem_set(s_5_10, s_5_8, s_5_9, s_5_11)
        let s_5_12: Bits = Elem_set(state, tracer, s_5_10, s_5_8, s_5_9, s_5_11);
        // D s_5_13: write-var result <= s_5_12
        fn_state.result = s_5_12;
        // D s_5_14: read-var s:i64
        let s_5_14: i64 = fn_state.s;
        // C s_5_15: const #1s : i64
        let s_5_15: i64 = 1;
        // D s_5_16: add s_5_14 s_5_15
        let s_5_16: i64 = (s_5_14 + s_5_15);
        // D s_5_17: write-var s <= s_5_16
        fn_state.s = s_5_16;
        // N s_5_18: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VLshadow#2393:i64
        let s_6_0: i64 = fn_state.VLshadow_2393;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var da:i64
        let s_6_3: i64 = fn_state.da;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast zx s_6_2 -> i
        let s_6_5: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_6: read-var result:bv
        let s_6_6: Bits = fn_state.result;
        // D s_6_7: call Z_set(s_6_4, s_6_5, s_6_6)
        let s_6_7: () = Z_set(state, tracer, s_6_4, s_6_5, s_6_6);
        // N s_6_8: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
}
