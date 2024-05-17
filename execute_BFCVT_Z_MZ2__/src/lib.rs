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
pub fn execute_BFCVT_Z_MZ2__<T: Tracer>(
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
        VLshadow_6034: i64,
        VLshadow_6035: i64,
        gs_842693: Bits,
        gs_270583: i64,
        gs_842691: Bits,
        elements: i64,
        result: Bits,
        operand1: Bits,
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
        // D s_0_3: write-var VLshadow#6034 <= s_0_2
        fn_state.VLshadow_6034 = s_0_2;
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
        // D s_1_0: read-var VLshadow#6034:i64
        let s_1_0: i64 = fn_state.VLshadow_6034;
        // D s_1_1: write-var VLshadow#6035 <= s_1_0
        fn_state.VLshadow_6035 = s_1_0;
        // C s_1_2: const #32s : i
        let s_1_2: i128 = 32;
        // D s_1_3: read-var VLshadow#6035:i64
        let s_1_3: i64 = fn_state.VLshadow_6035;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: write-var elements <= s_1_6
        fn_state.elements = s_1_6;
        // C s_1_8: const #0s : i
        let s_1_8: i128 = 0;
        // D s_1_9: read-var n:i64
        let s_1_9: i64 = fn_state.n;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: add s_1_10 s_1_8
        let s_1_11: i128 = (s_1_10 + s_1_8);
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: read-var VLshadow#6035:i64
        let s_1_13: i64 = fn_state.VLshadow_6035;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: cast zx s_1_12 -> i
        let s_1_16: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_17: cast zx s_1_15 -> i
        let s_1_17: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_18: call Z_read(s_1_16, s_1_17)
        let s_1_18: Bits = Z_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var operand1 <= s_1_18
        fn_state.operand1 = s_1_18;
        // C s_1_20: const #1s : i
        let s_1_20: i128 = 1;
        // D s_1_21: read-var n:i64
        let s_1_21: i64 = fn_state.n;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: add s_1_22 s_1_20
        let s_1_23: i128 = (s_1_22 + s_1_20);
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: read-var VLshadow#6035:i64
        let s_1_25: i64 = fn_state.VLshadow_6035;
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: cast reint s_1_26 -> i64
        let s_1_27: i64 = (s_1_26 as i64);
        // D s_1_28: cast zx s_1_24 -> i
        let s_1_28: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_29: cast zx s_1_27 -> i
        let s_1_29: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_30: call Z_read(s_1_28, s_1_29)
        let s_1_30: Bits = Z_read(state, tracer, s_1_28, s_1_29);
        // D s_1_31: write-var operand2 <= s_1_30
        fn_state.operand2 = s_1_30;
        // C s_1_32: const #0s : i64
        let s_1_32: i64 = 0;
        // C s_1_33: const #1s : i
        let s_1_33: i128 = 1;
        // D s_1_34: read-var elements:i64
        let s_1_34: i64 = fn_state.elements;
        // D s_1_35: cast zx s_1_34 -> i
        let s_1_35: i128 = (i128::try_from(s_1_34).unwrap());
        // D s_1_36: sub s_1_35 s_1_33
        let s_1_36: i128 = ((s_1_35) - (s_1_33));
        // D s_1_37: cast reint s_1_36 -> i64
        let s_1_37: i64 = (s_1_36 as i64);
        // D s_1_38: write-var gs#270583 <= s_1_37
        fn_state.gs_270583 = s_1_37;
        // D s_1_39: write-var e <= s_1_32
        fn_state.e = s_1_32;
        // N s_1_40: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#270583:i64
        let s_2_1: i64 = fn_state.gs_270583;
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
        // D s_3_19: write-var gs#842691 <= s_3_18
        fn_state.gs_842691 = s_3_18;
        // N s_3_20: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#842691:bv
        let s_4_0: Bits = fn_state.gs_842691;
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
        // D s_4_8: write-var gs#842693 <= s_4_7
        fn_state.gs_842693 = s_4_7;
        // N s_4_9: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#842693:bv
        let s_5_0: Bits = fn_state.gs_842693;
        // D s_5_1: cast reint s_5_0 -> u16
        let s_5_1: u16 = (s_5_0.value() as u16);
        // C s_5_2: const #16s : i64
        let s_5_2: i64 = 16;
        // D s_5_3: read-var e:i64
        let s_5_3: i64 = fn_state.e;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var res1:u16
        let s_5_6: u16 = fn_state.res1;
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 16u16);
        // D s_5_8: read-var result:bv
        let s_5_8: Bits = fn_state.result;
        // D s_5_9: call Elem_set(s_5_8, s_5_4, s_5_5, s_5_7)
        let s_5_9: Bits = Elem_set(state, tracer, s_5_8, s_5_4, s_5_5, s_5_7);
        // D s_5_10: write-var result <= s_5_9
        fn_state.result = s_5_9;
        // D s_5_11: read-var elements:i64
        let s_5_11: i64 = fn_state.elements;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: read-var e:i64
        let s_5_13: i64 = fn_state.e;
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_15: add s_5_12 s_5_14
        let s_5_15: i128 = (s_5_12 + s_5_14);
        // D s_5_16: cast reint s_5_15 -> i64
        let s_5_16: i64 = (s_5_15 as i64);
        // C s_5_17: const #16s : i64
        let s_5_17: i64 = 16;
        // D s_5_18: cast zx s_5_16 -> i
        let s_5_18: i128 = (i128::try_from(s_5_16).unwrap());
        // C s_5_19: cast zx s_5_17 -> i
        let s_5_19: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_20: cast zx s_5_1 -> bv
        let s_5_20: Bits = Bits::new(s_5_1 as u128, 16u16);
        // D s_5_21: read-var result:bv
        let s_5_21: Bits = fn_state.result;
        // D s_5_22: call Elem_set(s_5_21, s_5_18, s_5_19, s_5_20)
        let s_5_22: Bits = Elem_set(state, tracer, s_5_21, s_5_18, s_5_19, s_5_20);
        // D s_5_23: write-var result <= s_5_22
        fn_state.result = s_5_22;
        // D s_5_24: read-var e:i64
        let s_5_24: i64 = fn_state.e;
        // C s_5_25: const #1s : i64
        let s_5_25: i64 = 1;
        // D s_5_26: add s_5_24 s_5_25
        let s_5_26: i64 = (s_5_24 + s_5_25);
        // D s_5_27: write-var e <= s_5_26
        fn_state.e = s_5_26;
        // N s_5_28: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VLshadow#6035:i64
        let s_6_0: i64 = fn_state.VLshadow_6035;
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
