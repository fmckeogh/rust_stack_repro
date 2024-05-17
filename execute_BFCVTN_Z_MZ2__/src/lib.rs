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
use CheckStreamingSVEEnabled::*;
use Elem_read::*;
use FPCR_read::*;
use Z_read::*;
use FPConvertBF__1::*;
use Elem_set::*;
use common::*;
pub fn execute_BFCVTN_Z_MZ2__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: u32,
        e: i64,
        gs_270675: i64,
        gs_842817: Bits,
        gs_842819: Bits,
        VLshadow_6038: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_6039: i64,
        res1: u16,
        operand2: Bits,
        VL: i64,
        d: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
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
        // D s_0_3: write-var VLshadow#6038 <= s_0_2
        fn_state.VLshadow_6038 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#6038:i64
        let s_1_0: i64 = fn_state.VLshadow_6038;
        // D s_1_1: write-var VLshadow#6039 <= s_1_0
        fn_state.VLshadow_6039 = s_1_0;
        // C s_1_2: const #32s : i
        let s_1_2: i128 = 32;
        // D s_1_3: read-var VLshadow#6039:i64
        let s_1_3: i64 = fn_state.VLshadow_6039;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #0s : i
        let s_1_7: i128 = 0;
        // D s_1_8: read-var n:i64
        let s_1_8: i64 = fn_state.n;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: add s_1_9 s_1_7
        let s_1_10: i128 = (s_1_9 + s_1_7);
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var VLshadow#6039:i64
        let s_1_12: i64 = fn_state.VLshadow_6039;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: cast zx s_1_11 -> i
        let s_1_15: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_16: cast zx s_1_14 -> i
        let s_1_16: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_17: call Z_read(s_1_15, s_1_16)
        let s_1_17: Bits = Z_read(state, tracer, s_1_15, s_1_16);
        // D s_1_18: write-var operand1 <= s_1_17
        fn_state.operand1 = s_1_17;
        // C s_1_19: const #1s : i
        let s_1_19: i128 = 1;
        // D s_1_20: read-var n:i64
        let s_1_20: i64 = fn_state.n;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: add s_1_21 s_1_19
        let s_1_22: i128 = (s_1_21 + s_1_19);
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var VLshadow#6039:i64
        let s_1_24: i64 = fn_state.VLshadow_6039;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: cast zx s_1_23 -> i
        let s_1_27: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_28: cast zx s_1_26 -> i
        let s_1_28: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_29: call Z_read(s_1_27, s_1_28)
        let s_1_29: Bits = Z_read(state, tracer, s_1_27, s_1_28);
        // D s_1_30: write-var operand2 <= s_1_29
        fn_state.operand2 = s_1_29;
        // C s_1_31: const #0s : i64
        let s_1_31: i64 = 0;
        // C s_1_32: const #1s : i
        let s_1_32: i128 = 1;
        // D s_1_33: cast zx s_1_6 -> i
        let s_1_33: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_34: sub s_1_33 s_1_32
        let s_1_34: i128 = ((s_1_33) - (s_1_32));
        // D s_1_35: cast reint s_1_34 -> i64
        let s_1_35: i64 = (s_1_34 as i64);
        // D s_1_36: write-var gs#270675 <= s_1_35
        fn_state.gs_270675 = s_1_35;
        // D s_1_37: write-var e <= s_1_31
        fn_state.e = s_1_31;
        // N s_1_38: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#270675:i64
        let s_2_1: i64 = fn_state.gs_270675;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b6 b3
        if s_2_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #32s : i64
        let s_3_0: i64 = 32;
        // D s_3_1: read-var e:i64
        let s_3_1: i64 = fn_state.e;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // C s_3_3: cast zx s_3_0 -> i
        let s_3_3: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_4: read-var operand1:bv
        let s_3_4: Bits = fn_state.operand1;
        // D s_3_5: call Elem_read(s_3_4, s_3_2, s_3_3)
        let s_3_5: Bits = Elem_read(state, tracer, s_3_4, s_3_2, s_3_3);
        // D s_3_6: cast reint s_3_5 -> u32
        let s_3_6: u32 = (s_3_5.value() as u32);
        // C s_3_7: const #32s : i64
        let s_3_7: i64 = 32;
        // D s_3_8: read-var e:i64
        let s_3_8: i64 = fn_state.e;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // C s_3_10: cast zx s_3_7 -> i
        let s_3_10: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_11: read-var operand2:bv
        let s_3_11: Bits = fn_state.operand2;
        // D s_3_12: call Elem_read(s_3_11, s_3_9, s_3_10)
        let s_3_12: Bits = Elem_read(state, tracer, s_3_11, s_3_9, s_3_10);
        // D s_3_13: cast reint s_3_12 -> u32
        let s_3_13: u32 = (s_3_12.value() as u32);
        // D s_3_14: write-var element2 <= s_3_13
        fn_state.element2 = s_3_13;
        // C s_3_15: const #() : ()
        let s_3_15: () = ();
        // S s_3_16: call FPCR_read(s_3_15)
        let s_3_16: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_3_15);
        // D s_3_17: cast zx s_3_6 -> bv
        let s_3_17: Bits = Bits::new(s_3_6 as u128, 32u16);
        // D s_3_18: call FPConvertBF__1(s_3_17, s_3_16)
        let s_3_18: Bits = FPConvertBF__1(state, tracer, s_3_17, s_3_16);
        // D s_3_19: write-var gs#842817 <= s_3_18
        fn_state.gs_842817 = s_3_18;
        // N s_3_20: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#842817:bv
        let s_4_0: Bits = fn_state.gs_842817;
        // D s_4_1: cast reint s_4_0 -> u16
        let s_4_1: u16 = (s_4_0.value() as u16);
        // D s_4_2: write-var res1 <= s_4_1
        fn_state.res1 = s_4_1;
        // C s_4_3: const #() : ()
        let s_4_3: () = ();
        // S s_4_4: call FPCR_read(s_4_3)
        let s_4_4: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_4_3);
        // D s_4_5: read-var element2:u32
        let s_4_5: u32 = fn_state.element2;
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 32u16);
        // D s_4_7: call FPConvertBF__1(s_4_6, s_4_4)
        let s_4_7: Bits = FPConvertBF__1(state, tracer, s_4_6, s_4_4);
        // D s_4_8: write-var gs#842819 <= s_4_7
        fn_state.gs_842819 = s_4_7;
        // N s_4_9: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#842819:bv
        let s_5_0: Bits = fn_state.gs_842819;
        // D s_5_1: cast reint s_5_0 -> u16
        let s_5_1: u16 = (s_5_0.value() as u16);
        // C s_5_2: const #2s : i
        let s_5_2: i128 = 2;
        // D s_5_3: read-var e:i64
        let s_5_3: i64 = fn_state.e;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: mul s_5_2 s_5_4
        let s_5_5: i128 = ((s_5_2) * (s_5_4));
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // C s_5_7: const #0s : i
        let s_5_7: i128 = 0;
        // D s_5_8: cast zx s_5_6 -> i
        let s_5_8: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_9: add s_5_8 s_5_7
        let s_5_9: i128 = (s_5_8 + s_5_7);
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // C s_5_11: const #16s : i64
        let s_5_11: i64 = 16;
        // D s_5_12: cast zx s_5_10 -> i
        let s_5_12: i128 = (i128::try_from(s_5_10).unwrap());
        // C s_5_13: cast zx s_5_11 -> i
        let s_5_13: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_14: read-var res1:u16
        let s_5_14: u16 = fn_state.res1;
        // D s_5_15: cast zx s_5_14 -> bv
        let s_5_15: Bits = Bits::new(s_5_14 as u128, 16u16);
        // D s_5_16: read-var result:bv
        let s_5_16: Bits = fn_state.result;
        // D s_5_17: call Elem_set(s_5_16, s_5_12, s_5_13, s_5_15)
        let s_5_17: Bits = Elem_set(state, tracer, s_5_16, s_5_12, s_5_13, s_5_15);
        // D s_5_18: write-var result <= s_5_17
        fn_state.result = s_5_17;
        // C s_5_19: const #2s : i
        let s_5_19: i128 = 2;
        // D s_5_20: read-var e:i64
        let s_5_20: i64 = fn_state.e;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: mul s_5_19 s_5_21
        let s_5_22: i128 = ((s_5_19) * (s_5_21));
        // D s_5_23: cast reint s_5_22 -> i64
        let s_5_23: i64 = (s_5_22 as i64);
        // C s_5_24: const #1s : i
        let s_5_24: i128 = 1;
        // D s_5_25: cast zx s_5_23 -> i
        let s_5_25: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_26: add s_5_25 s_5_24
        let s_5_26: i128 = (s_5_25 + s_5_24);
        // D s_5_27: cast reint s_5_26 -> i64
        let s_5_27: i64 = (s_5_26 as i64);
        // C s_5_28: const #16s : i64
        let s_5_28: i64 = 16;
        // D s_5_29: cast zx s_5_27 -> i
        let s_5_29: i128 = (i128::try_from(s_5_27).unwrap());
        // C s_5_30: cast zx s_5_28 -> i
        let s_5_30: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_31: cast zx s_5_1 -> bv
        let s_5_31: Bits = Bits::new(s_5_1 as u128, 16u16);
        // D s_5_32: read-var result:bv
        let s_5_32: Bits = fn_state.result;
        // D s_5_33: call Elem_set(s_5_32, s_5_29, s_5_30, s_5_31)
        let s_5_33: Bits = Elem_set(state, tracer, s_5_32, s_5_29, s_5_30, s_5_31);
        // D s_5_34: write-var result <= s_5_33
        fn_state.result = s_5_33;
        // D s_5_35: read-var e:i64
        let s_5_35: i64 = fn_state.e;
        // C s_5_36: const #1s : i64
        let s_5_36: i64 = 1;
        // D s_5_37: add s_5_35 s_5_36
        let s_5_37: i64 = (s_5_35 + s_5_36);
        // D s_5_38: write-var e <= s_5_37
        fn_state.e = s_5_37;
        // N s_5_39: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VLshadow#6039:i64
        let s_6_0: i64 = fn_state.VLshadow_6039;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var d:i64
        let s_6_3: i64 = fn_state.d;
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
}
