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
pub fn execute_FMMLA_Z_ZZZ_S<T: Tracer>(
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
        s: i64,
        operand3: Bits,
        gs_183337: i64,
        VLshadow_2391: i64,
        result: Bits,
        res: Bits,
        operand1: Bits,
        VLshadow_2390: i64,
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
        // D s_0_3: write-var VLshadow#2390 <= s_0_2
        fn_state.VLshadow_2390 = s_0_2;
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
        // D s_1_0: read-var VLshadow#2390:i64
        let s_1_0: i64 = fn_state.VLshadow_2390;
        // D s_1_1: write-var VLshadow#2391 <= s_1_0
        fn_state.VLshadow_2391 = s_1_0;
        // C s_1_2: const #4s : i
        let s_1_2: i128 = 4;
        // D s_1_3: read-var esize:i64
        let s_1_3: i64 = fn_state.esize;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_2 s_1_4
        let s_1_5: i128 = ((s_1_2) * (s_1_4));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2391:i64
        let s_1_7: i64 = fn_state.VLshadow_2391;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var VLshadow#2391:i64
        let s_1_12: i64 = fn_state.VLshadow_2391;
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
        // D s_1_19: write-var operand1 <= s_1_18
        fn_state.operand1 = s_1_18;
        // D s_1_20: read-var VLshadow#2391:i64
        let s_1_20: i64 = fn_state.VLshadow_2391;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var m:i64
        let s_1_23: i64 = fn_state.m;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_22 -> i
        let s_1_25: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_26: call Z_read(s_1_24, s_1_25)
        let s_1_26: Bits = Z_read(state, tracer, s_1_24, s_1_25);
        // D s_1_27: write-var operand2 <= s_1_26
        fn_state.operand2 = s_1_26;
        // D s_1_28: read-var VLshadow#2391:i64
        let s_1_28: i64 = fn_state.VLshadow_2391;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: read-var da:i64
        let s_1_31: i64 = fn_state.da;
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_33: cast zx s_1_30 -> i
        let s_1_33: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_34: call Z_read(s_1_32, s_1_33)
        let s_1_34: Bits = Z_read(state, tracer, s_1_32, s_1_33);
        // D s_1_35: write-var operand3 <= s_1_34
        fn_state.operand3 = s_1_34;
        // D s_1_36: read-var VLshadow#2391:i64
        let s_1_36: i64 = fn_state.VLshadow_2391;
        // D s_1_37: cast zx s_1_36 -> i
        let s_1_37: i128 = (i128::try_from(s_1_36).unwrap());
        // D s_1_38: call Zeros(s_1_37)
        let s_1_38: Bits = Zeros(state, tracer, s_1_37);
        // D s_1_39: write-var result <= s_1_38
        fn_state.result = s_1_38;
        // C s_1_40: const #0s : i64
        let s_1_40: i64 = 0;
        // C s_1_41: const #1s : i
        let s_1_41: i128 = 1;
        // D s_1_42: cast zx s_1_11 -> i
        let s_1_42: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_43: sub s_1_42 s_1_41
        let s_1_43: i128 = ((s_1_42) - (s_1_41));
        // D s_1_44: cast reint s_1_43 -> i64
        let s_1_44: i64 = (s_1_43 as i64);
        // D s_1_45: write-var gs#183337 <= s_1_44
        fn_state.gs_183337 = s_1_44;
        // D s_1_46: write-var s <= s_1_40
        fn_state.s = s_1_40;
        // N s_1_47: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var s:i64
        let s_2_0: i64 = fn_state.s;
        // D s_2_1: read-var gs#183337:i64
        let s_2_1: i64 = fn_state.gs_183337;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b5 b3
        if s_2_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #4s : i
        let s_3_0: i128 = 4;
        // D s_3_1: read-var esize:i64
        let s_3_1: i64 = fn_state.esize;
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
        // D s_3_7: read-var s:i64
        let s_3_7: i64 = fn_state.s;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cast zx s_3_6 -> i
        let s_3_9: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_10: read-var operand1:bv
        let s_3_10: Bits = fn_state.operand1;
        // D s_3_11: call Elem_read(s_3_10, s_3_8, s_3_9)
        let s_3_11: Bits = Elem_read(state, tracer, s_3_10, s_3_8, s_3_9);
        // C s_3_12: const #4s : i
        let s_3_12: i128 = 4;
        // D s_3_13: read-var esize:i64
        let s_3_13: i64 = fn_state.esize;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: mul s_3_12 s_3_14
        let s_3_15: i128 = ((s_3_12) * (s_3_14));
        // D s_3_16: cast reint s_3_15 -> i64
        let s_3_16: i64 = (s_3_15 as i64);
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: read-var s:i64
        let s_3_19: i64 = fn_state.s;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: cast zx s_3_18 -> i
        let s_3_21: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_22: read-var operand2:bv
        let s_3_22: Bits = fn_state.operand2;
        // D s_3_23: call Elem_read(s_3_22, s_3_20, s_3_21)
        let s_3_23: Bits = Elem_read(state, tracer, s_3_22, s_3_20, s_3_21);
        // C s_3_24: const #4s : i
        let s_3_24: i128 = 4;
        // D s_3_25: read-var esize:i64
        let s_3_25: i64 = fn_state.esize;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: mul s_3_24 s_3_26
        let s_3_27: i128 = ((s_3_24) * (s_3_26));
        // D s_3_28: cast reint s_3_27 -> i64
        let s_3_28: i64 = (s_3_27 as i64);
        // D s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_30: cast reint s_3_29 -> i64
        let s_3_30: i64 = (s_3_29 as i64);
        // D s_3_31: read-var s:i64
        let s_3_31: i64 = fn_state.s;
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: cast zx s_3_30 -> i
        let s_3_33: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_34: read-var operand3:bv
        let s_3_34: Bits = fn_state.operand3;
        // D s_3_35: call Elem_read(s_3_34, s_3_32, s_3_33)
        let s_3_35: Bits = Elem_read(state, tracer, s_3_34, s_3_32, s_3_33);
        // D s_3_36: read-var esize:i64
        let s_3_36: i64 = fn_state.esize;
        // D s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_38: cast reint s_3_37 -> i64
        let s_3_38: i64 = (s_3_37 as i64);
        // C s_3_39: const #() : ()
        let s_3_39: () = ();
        // S s_3_40: call FPCR_read(s_3_39)
        let s_3_40: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_3_39);
        // D s_3_41: cast zx s_3_38 -> i
        let s_3_41: i128 = (i128::try_from(s_3_38).unwrap());
        // D s_3_42: call FPMatMulAdd(s_3_35, s_3_11, s_3_23, s_3_41, s_3_40)
        let s_3_42: Bits = FPMatMulAdd(
            state,
            tracer,
            s_3_35,
            s_3_11,
            s_3_23,
            s_3_41,
            s_3_40,
        );
        // D s_3_43: write-var res <= s_3_42
        fn_state.res = s_3_42;
        // N s_3_44: jump b4
        return block_4(state, tracer, fn_state);
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
        // D s_4_10: read-var result:bv
        let s_4_10: Bits = fn_state.result;
        // D s_4_11: read-var res:bv
        let s_4_11: Bits = fn_state.res;
        // D s_4_12: call Elem_set(s_4_10, s_4_8, s_4_9, s_4_11)
        let s_4_12: Bits = Elem_set(state, tracer, s_4_10, s_4_8, s_4_9, s_4_11);
        // D s_4_13: write-var result <= s_4_12
        fn_state.result = s_4_12;
        // D s_4_14: read-var s:i64
        let s_4_14: i64 = fn_state.s;
        // C s_4_15: const #1s : i64
        let s_4_15: i64 = 1;
        // D s_4_16: add s_4_14 s_4_15
        let s_4_16: i64 = (s_4_14 + s_4_15);
        // D s_4_17: write-var s <= s_4_16
        fn_state.s = s_4_16;
        // N s_4_18: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VLshadow#2391:i64
        let s_5_0: i64 = fn_state.VLshadow_2391;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var da:i64
        let s_5_3: i64 = fn_state.da;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var result:bv
        let s_5_6: Bits = fn_state.result;
        // D s_5_7: call Z_set(s_5_4, s_5_5, s_5_6)
        let s_5_7: () = Z_set(state, tracer, s_5_4, s_5_5, s_5_6);
        // N s_5_8: return
        return;
    }
}
