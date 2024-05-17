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
use u__id::*;
use FPCR_read::*;
use FPMinNum::*;
use Elem_set::*;
use FPMaxNum::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_FCLAMP_MZ_ZZ_4<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
    nreg: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        gs_270526: bool,
        e: i64,
        element3: Bits,
        operand3: Bits,
        ga_328206: Bits,
        gs_270513: i64,
        esizeshadow_6031: i64,
        ga_328210: Bits,
        VLshadow_6033: i64,
        VLshadow_6032: i64,
        element2: Bits,
        ga_328208: Bits,
        element1: Bits,
        u_8066: i64,
        gs_270519: i64,
        ga_328209: i64,
        elements: i64,
        operand1: Bits,
        gs_270524: bool,
        results: [Bits; 4usize],
        operand2: Bits,
        gs_270534: i64,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
        nreg: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
        n,
        nreg,
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
        // D s_0_3: write-var esizeshadow#6031 <= s_0_2
        fn_state.esizeshadow_6031 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6032 <= s_0_6
        fn_state.VLshadow_6032 = s_0_6;
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
        // D s_1_0: read-var VLshadow#6032:i64
        let s_1_0: i64 = fn_state.VLshadow_6032;
        // D s_1_1: write-var VLshadow#6033 <= s_1_0
        fn_state.VLshadow_6033 = s_1_0;
        // D s_1_2: read-var VLshadow#6033:i64
        let s_1_2: i64 = fn_state.VLshadow_6033;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#6031:i64
        let s_1_4: i64 = fn_state.esizeshadow_6031;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: write-var elements <= s_1_7
        fn_state.elements = s_1_7;
        // C s_1_9: const #0s : i64
        let s_1_9: i64 = 0;
        // C s_1_10: const #1s : i
        let s_1_10: i128 = 1;
        // D s_1_11: read-var nreg:i64
        let s_1_11: i64 = fn_state.nreg;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: sub s_1_12 s_1_10
        let s_1_13: i128 = ((s_1_12) - (s_1_10));
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: write-var gs#270513 <= s_1_14
        fn_state.gs_270513 = s_1_14;
        // D s_1_16: write-var r <= s_1_9
        fn_state.r = s_1_9;
        // N s_1_17: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#270513:i64
        let s_2_1: i64 = fn_state.gs_270513;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b15 b3
        if s_2_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var VLshadow#6033:i64
        let s_3_0: i64 = fn_state.VLshadow_6033;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var n:i64
        let s_3_3: i64 = fn_state.n;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: call Z_read(s_3_4, s_3_5)
        let s_3_6: Bits = Z_read(state, tracer, s_3_4, s_3_5);
        // D s_3_7: write-var operand1 <= s_3_6
        fn_state.operand1 = s_3_6;
        // D s_3_8: read-var VLshadow#6033:i64
        let s_3_8: i64 = fn_state.VLshadow_6033;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: read-var m:i64
        let s_3_11: i64 = fn_state.m;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: cast zx s_3_10 -> i
        let s_3_13: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_14: call Z_read(s_3_12, s_3_13)
        let s_3_14: Bits = Z_read(state, tracer, s_3_12, s_3_13);
        // D s_3_15: write-var operand2 <= s_3_14
        fn_state.operand2 = s_3_14;
        // D s_3_16: read-var d:i64
        let s_3_16: i64 = fn_state.d;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: read-var r:i64
        let s_3_18: i64 = fn_state.r;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: add s_3_17 s_3_19
        let s_3_20: i128 = (s_3_17 + s_3_19);
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: read-var VLshadow#6033:i64
        let s_3_22: i64 = fn_state.VLshadow_6033;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: cast zx s_3_21 -> i
        let s_3_25: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_26: cast zx s_3_24 -> i
        let s_3_26: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_27: call Z_read(s_3_25, s_3_26)
        let s_3_27: Bits = Z_read(state, tracer, s_3_25, s_3_26);
        // D s_3_28: write-var operand3 <= s_3_27
        fn_state.operand3 = s_3_27;
        // C s_3_29: const #0s : i64
        let s_3_29: i64 = 0;
        // C s_3_30: const #1s : i
        let s_3_30: i128 = 1;
        // D s_3_31: read-var elements:i64
        let s_3_31: i64 = fn_state.elements;
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: sub s_3_32 s_3_30
        let s_3_33: i128 = ((s_3_32) - (s_3_30));
        // D s_3_34: cast reint s_3_33 -> i64
        let s_3_34: i64 = (s_3_33 as i64);
        // D s_3_35: write-var gs#270519 <= s_3_34
        fn_state.gs_270519 = s_3_34;
        // D s_3_36: write-var e <= s_3_29
        fn_state.e = s_3_29;
        // N s_3_37: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#270519:i64
        let s_4_1: i64 = fn_state.gs_270519;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b14 b5
        if s_4_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#6031:i64
        let s_5_0: i64 = fn_state.esizeshadow_6031;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var e:i64
        let s_5_3: i64 = fn_state.e;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var operand1:bv
        let s_5_6: Bits = fn_state.operand1;
        // D s_5_7: call Elem_read(s_5_6, s_5_4, s_5_5)
        let s_5_7: Bits = Elem_read(state, tracer, s_5_6, s_5_4, s_5_5);
        // D s_5_8: write-var element1 <= s_5_7
        fn_state.element1 = s_5_7;
        // D s_5_9: read-var esizeshadow#6031:i64
        let s_5_9: i64 = fn_state.esizeshadow_6031;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // D s_5_12: read-var e:i64
        let s_5_12: i64 = fn_state.e;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: cast zx s_5_11 -> i
        let s_5_14: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_15: read-var operand2:bv
        let s_5_15: Bits = fn_state.operand2;
        // D s_5_16: call Elem_read(s_5_15, s_5_13, s_5_14)
        let s_5_16: Bits = Elem_read(state, tracer, s_5_15, s_5_13, s_5_14);
        // D s_5_17: write-var element2 <= s_5_16
        fn_state.element2 = s_5_16;
        // D s_5_18: read-var esizeshadow#6031:i64
        let s_5_18: i64 = fn_state.esizeshadow_6031;
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: cast reint s_5_19 -> i64
        let s_5_20: i64 = (s_5_19 as i64);
        // D s_5_21: read-var e:i64
        let s_5_21: i64 = fn_state.e;
        // D s_5_22: cast zx s_5_21 -> i
        let s_5_22: i128 = (i128::try_from(s_5_21).unwrap());
        // D s_5_23: cast zx s_5_20 -> i
        let s_5_23: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_24: read-var operand3:bv
        let s_5_24: Bits = fn_state.operand3;
        // D s_5_25: call Elem_read(s_5_24, s_5_22, s_5_23)
        let s_5_25: Bits = Elem_read(state, tracer, s_5_24, s_5_22, s_5_23);
        // D s_5_26: write-var element3 <= s_5_25
        fn_state.element3 = s_5_25;
        // D s_5_27: read-var esizeshadow#6031:i64
        let s_5_27: i64 = fn_state.esizeshadow_6031;
        // D s_5_28: cast zx s_5_27 -> i
        let s_5_28: i128 = (i128::try_from(s_5_27).unwrap());
        // D s_5_29: call __id(s_5_28)
        let s_5_29: i128 = u__id(state, tracer, s_5_28);
        // D s_5_30: cast reint s_5_29 -> i64
        let s_5_30: i64 = (s_5_29 as i64);
        // C s_5_31: const #16s : i
        let s_5_31: i128 = 16;
        // D s_5_32: cast zx s_5_30 -> i
        let s_5_32: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_33: cmp-eq s_5_32 s_5_31
        let s_5_33: bool = ((s_5_32) == (s_5_31));
        // N s_5_34: branch s_5_33 b13 b6
        if s_5_33 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#6031:i64
        let s_6_0: i64 = fn_state.esizeshadow_6031;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call __id(s_6_1)
        let s_6_2: i128 = u__id(state, tracer, s_6_1);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #32s : i
        let s_6_4: i128 = 32;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: cmp-eq s_6_5 s_6_4
        let s_6_6: bool = ((s_6_5) == (s_6_4));
        // D s_6_7: write-var gs#270524 <= s_6_6
        fn_state.gs_270524 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#270524:u8
        let s_7_0: bool = fn_state.gs_270524;
        // N s_7_1: branch s_7_0 b12 b8
        if s_7_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esizeshadow#6031:i64
        let s_8_0: i64 = fn_state.esizeshadow_6031;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call __id(s_8_1)
        let s_8_2: i128 = u__id(state, tracer, s_8_1);
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // C s_8_4: const #64s : i
        let s_8_4: i128 = 64;
        // D s_8_5: cast zx s_8_3 -> i
        let s_8_5: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_6: cmp-eq s_8_5 s_8_4
        let s_8_6: bool = ((s_8_5) == (s_8_4));
        // D s_8_7: write-var gs#270526 <= s_8_6
        fn_state.gs_270526 = s_8_6;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#270526:u8
        let s_9_0: bool = fn_state.gs_270526;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // D s_9_2: read-var results:[bv; 4]
        let s_9_2: [Bits; 4usize] = fn_state.results;
        // D s_9_3: cast cvt s_9_2 -> [bv; 0]
        let s_9_3: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_9_2);
        // D s_9_4: read-var r:i64
        let s_9_4: i64 = fn_state.r;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-element s_9_3[s_9_5]
        let s_9_6: Bits = s_9_3[(s_9_5) as usize];
        // D s_9_7: write-var ga#328208 <= s_9_6
        fn_state.ga_328208 = s_9_6;
        // D s_9_8: read-var esizeshadow#6031:i64
        let s_9_8: i64 = fn_state.esizeshadow_6031;
        // D s_9_9: cast zx s_9_8 -> i
        let s_9_9: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_10: cast reint s_9_9 -> i64
        let s_9_10: i64 = (s_9_9 as i64);
        // D s_9_11: write-var ga#328209 <= s_9_10
        fn_state.ga_328209 = s_9_10;
        // C s_9_12: const #() : ()
        let s_9_12: () = ();
        // S s_9_13: call FPCR_read(s_9_12)
        let s_9_13: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_9_12);
        // D s_9_14: read-var element1:bv
        let s_9_14: Bits = fn_state.element1;
        // D s_9_15: read-var element3:bv
        let s_9_15: Bits = fn_state.element3;
        // D s_9_16: call FPMaxNum(s_9_14, s_9_15, s_9_13)
        let s_9_16: Bits = FPMaxNum(state, tracer, s_9_14, s_9_15, s_9_13);
        // D s_9_17: write-var ga#328206 <= s_9_16
        fn_state.ga_328206 = s_9_16;
        // N s_9_18: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call FPCR_read(s_10_0)
        let s_10_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_10_0);
        // D s_10_2: read-var ga#328206:bv
        let s_10_2: Bits = fn_state.ga_328206;
        // D s_10_3: read-var element2:bv
        let s_10_3: Bits = fn_state.element2;
        // D s_10_4: call FPMinNum(s_10_2, s_10_3, s_10_1)
        let s_10_4: Bits = FPMinNum(state, tracer, s_10_2, s_10_3, s_10_1);
        // D s_10_5: write-var ga#328210 <= s_10_4
        fn_state.ga_328210 = s_10_4;
        // N s_10_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var e:i64
        let s_11_0: i64 = fn_state.e;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: read-var ga#328209:i64
        let s_11_2: i64 = fn_state.ga_328209;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: read-var ga#328208:bv
        let s_11_4: Bits = fn_state.ga_328208;
        // D s_11_5: read-var ga#328210:bv
        let s_11_5: Bits = fn_state.ga_328210;
        // D s_11_6: call Elem_set(s_11_4, s_11_1, s_11_3, s_11_5)
        let s_11_6: Bits = Elem_set(state, tracer, s_11_4, s_11_1, s_11_3, s_11_5);
        // D s_11_7: read-var results:[bv; 4]
        let s_11_7: [Bits; 4usize] = fn_state.results;
        // D s_11_8: cast cvt s_11_7 -> [bv; 0]
        let s_11_8: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_11_7);
        // D s_11_9: read-var r:i64
        let s_11_9: i64 = fn_state.r;
        // D s_11_10: cast zx s_11_9 -> i
        let s_11_10: i128 = (i128::try_from(s_11_9).unwrap());
        // D s_11_11: mutate-element s_11_8[s_11_10] <= s_11_6
        let s_11_11: alloc::vec::Vec<Bits> = {
            let mut local = s_11_8.clone();
            local[(s_11_10) as usize] = s_11_6;
            local
        };
        // D s_11_12: cast cvt s_11_11 -> [bv; 4]
        let s_11_12: [Bits; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_11_11);
            buf
        };
        // D s_11_13: write-var results <= s_11_12
        fn_state.results = s_11_12;
        // D s_11_14: read-var e:i64
        let s_11_14: i64 = fn_state.e;
        // C s_11_15: const #1s : i64
        let s_11_15: i64 = 1;
        // D s_11_16: add s_11_14 s_11_15
        let s_11_16: i64 = (s_11_14 + s_11_15);
        // D s_11_17: write-var e <= s_11_16
        fn_state.e = s_11_16;
        // N s_11_18: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#270526 <= s_12_0
        fn_state.gs_270526 = s_12_0;
        // N s_12_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#270524 <= s_13_0
        fn_state.gs_270524 = s_13_0;
        // N s_13_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var r:i64
        let s_14_0: i64 = fn_state.r;
        // C s_14_1: const #1s : i64
        let s_14_1: i64 = 1;
        // D s_14_2: add s_14_0 s_14_1
        let s_14_2: i64 = (s_14_0 + s_14_1);
        // D s_14_3: write-var r <= s_14_2
        fn_state.r = s_14_2;
        // N s_14_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0s : i64
        let s_15_0: i64 = 0;
        // C s_15_1: const #1s : i
        let s_15_1: i128 = 1;
        // D s_15_2: read-var nreg:i64
        let s_15_2: i64 = fn_state.nreg;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: sub s_15_3 s_15_1
        let s_15_4: i128 = ((s_15_3) - (s_15_1));
        // D s_15_5: cast reint s_15_4 -> i64
        let s_15_5: i64 = (s_15_4 as i64);
        // D s_15_6: write-var gs#270534 <= s_15_5
        fn_state.gs_270534 = s_15_5;
        // D s_15_7: write-var u#8066 <= s_15_0
        fn_state.u_8066 = s_15_0;
        // N s_15_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var u#8066:i64
        let s_16_0: i64 = fn_state.u_8066;
        // D s_16_1: read-var gs#270534:i64
        let s_16_1: i64 = fn_state.gs_270534;
        // D s_16_2: cmp-gt s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) > (s_16_1));
        // N s_16_3: branch s_16_2 b18 b17
        if s_16_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var d:i64
        let s_17_0: i64 = fn_state.d;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: read-var u#8066:i64
        let s_17_2: i64 = fn_state.u_8066;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: add s_17_1 s_17_3
        let s_17_4: i128 = (s_17_1 + s_17_3);
        // D s_17_5: cast reint s_17_4 -> i64
        let s_17_5: i64 = (s_17_4 as i64);
        // D s_17_6: read-var VLshadow#6033:i64
        let s_17_6: i64 = fn_state.VLshadow_6033;
        // D s_17_7: cast zx s_17_6 -> i
        let s_17_7: i128 = (i128::try_from(s_17_6).unwrap());
        // D s_17_8: cast reint s_17_7 -> i64
        let s_17_8: i64 = (s_17_7 as i64);
        // D s_17_9: read-var results:[bv; 4]
        let s_17_9: [Bits; 4usize] = fn_state.results;
        // D s_17_10: cast cvt s_17_9 -> [bv; 0]
        let s_17_10: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_17_9);
        // D s_17_11: read-var u#8066:i64
        let s_17_11: i64 = fn_state.u_8066;
        // D s_17_12: cast zx s_17_11 -> i
        let s_17_12: i128 = (i128::try_from(s_17_11).unwrap());
        // D s_17_13: read-element s_17_10[s_17_12]
        let s_17_13: Bits = s_17_10[(s_17_12) as usize];
        // D s_17_14: cast zx s_17_5 -> i
        let s_17_14: i128 = (i128::try_from(s_17_5).unwrap());
        // D s_17_15: cast zx s_17_8 -> i
        let s_17_15: i128 = (i128::try_from(s_17_8).unwrap());
        // D s_17_16: call Z_set(s_17_14, s_17_15, s_17_13)
        let s_17_16: () = Z_set(state, tracer, s_17_14, s_17_15, s_17_13);
        // D s_17_17: read-var u#8066:i64
        let s_17_17: i64 = fn_state.u_8066;
        // C s_17_18: const #1s : i64
        let s_17_18: i64 = 1;
        // D s_17_19: add s_17_17 s_17_18
        let s_17_19: i64 = (s_17_17 + s_17_18);
        // D s_17_20: write-var u#8066 <= s_17_19
        fn_state.u_8066 = s_17_19;
        // N s_17_21: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: return
        return;
    }
}
