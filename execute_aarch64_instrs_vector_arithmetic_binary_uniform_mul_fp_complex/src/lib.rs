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
use CheckFPAdvSIMDEnabled64::*;
use Elem_set::*;
use V_read::*;
use FPCR_read::*;
use u__id::*;
use V_set::*;
use FPNeg::*;
use Elem_read::*;
use FPMulAdd::*;
use fdiv_int::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_fp_complex<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    rot: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_253401: i64,
        gs_149599: bool,
        gs_149601: bool,
        ga_253410: Bits,
        element4: Bits,
        ga_253409: i64,
        gs_149582: bool,
        ga_253402: Bits,
        element2: Bits,
        esizeshadow_1274: i64,
        element1: Bits,
        ga_253400: i128,
        operand1: Bits,
        ga_253408: i128,
        operand2: Bits,
        fpcr: ProductType5c790c8ef59cc8b2,
        e: i64,
        element3: Bits,
        gs_149565: bool,
        gs_149614: bool,
        operand3: Bits,
        gs_149580: bool,
        gs_149548: i64,
        gs_149563: bool,
        datasizeshadow_1275: i64,
        gs_149612: bool,
        result: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        rot: u8,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        m,
        n,
        rot,
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
        // D s_0_3: write-var esizeshadow#1274 <= s_0_2
        fn_state.esizeshadow_1274 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1275 <= s_0_6
        fn_state.datasizeshadow_1275 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPAdvSIMDEnabled64(s_0_8)
        let s_0_9: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1275:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1275;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var operand1 <= s_1_5
        fn_state.operand1 = s_1_5;
        // D s_1_7: read-var datasizeshadow#1275:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1275;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var m:i64
        let s_1_10: i64 = fn_state.m;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: call V_read(s_1_11, s_1_9)
        let s_1_12: Bits = V_read(state, tracer, s_1_11, s_1_9);
        // D s_1_13: write-var operand2 <= s_1_12
        fn_state.operand2 = s_1_12;
        // D s_1_14: read-var datasizeshadow#1275:i64
        let s_1_14: i64 = fn_state.datasizeshadow_1275;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: read-var d:i64
        let s_1_17: i64 = fn_state.d;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: call V_read(s_1_18, s_1_16)
        let s_1_19: Bits = V_read(state, tracer, s_1_18, s_1_16);
        // D s_1_20: write-var operand3 <= s_1_19
        fn_state.operand3 = s_1_19;
        // C s_1_21: const #() : ()
        let s_1_21: () = ();
        // S s_1_22: call FPCR_read(s_1_21)
        let s_1_22: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_1_21);
        // D s_1_23: write-var fpcr <= s_1_22
        fn_state.fpcr = s_1_22;
        // C s_1_24: const #0s : i64
        let s_1_24: i64 = 0;
        // C s_1_25: const #2s : i
        let s_1_25: i128 = 2;
        // D s_1_26: read-var elements:i
        let s_1_26: i128 = fn_state.elements;
        // D s_1_27: call fdiv_int(s_1_26, s_1_25)
        let s_1_27: i128 = fdiv_int(state, tracer, s_1_26, s_1_25);
        // C s_1_28: const #1s : i
        let s_1_28: i128 = 1;
        // D s_1_29: sub s_1_27 s_1_28
        let s_1_29: i128 = ((s_1_27) - (s_1_28));
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: write-var gs#149548 <= s_1_30
        fn_state.gs_149548 = s_1_30;
        // D s_1_32: write-var e <= s_1_24
        fn_state.e = s_1_24;
        // N s_1_33: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#149548:i64
        let s_2_1: i64 = fn_state.gs_149548;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b41 b3
        if s_2_2 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var rot:u8
        let s_3_0: u8 = fn_state.rot;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #0u : u8
        let s_3_2: u8 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // N s_3_6: branch s_3_5 b14 b4
        if s_3_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2s : i
        let s_4_0: i128 = 2;
        // D s_4_1: read-var e:i64
        let s_4_1: i64 = fn_state.e;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: mul s_4_2 s_4_0
        let s_4_3: i128 = ((s_4_2) * (s_4_0));
        // D s_4_4: read-var esizeshadow#1274:i64
        let s_4_4: i64 = fn_state.esizeshadow_1274;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: cast reint s_4_5 -> i64
        let s_4_6: i64 = (s_4_5 as i64);
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: read-var operand2:bv
        let s_4_8: Bits = fn_state.operand2;
        // D s_4_9: call Elem_read(s_4_8, s_4_3, s_4_7)
        let s_4_9: Bits = Elem_read(state, tracer, s_4_8, s_4_3, s_4_7);
        // D s_4_10: write-var element1 <= s_4_9
        fn_state.element1 = s_4_9;
        // C s_4_11: const #2s : i
        let s_4_11: i128 = 2;
        // D s_4_12: read-var e:i64
        let s_4_12: i64 = fn_state.e;
        // D s_4_13: cast zx s_4_12 -> i
        let s_4_13: i128 = (i128::try_from(s_4_12).unwrap());
        // D s_4_14: mul s_4_13 s_4_11
        let s_4_14: i128 = ((s_4_13) * (s_4_11));
        // D s_4_15: read-var esizeshadow#1274:i64
        let s_4_15: i64 = fn_state.esizeshadow_1274;
        // D s_4_16: cast zx s_4_15 -> i
        let s_4_16: i128 = (i128::try_from(s_4_15).unwrap());
        // D s_4_17: cast reint s_4_16 -> i64
        let s_4_17: i64 = (s_4_16 as i64);
        // D s_4_18: cast zx s_4_17 -> i
        let s_4_18: i128 = (i128::try_from(s_4_17).unwrap());
        // D s_4_19: read-var operand1:bv
        let s_4_19: Bits = fn_state.operand1;
        // D s_4_20: call Elem_read(s_4_19, s_4_14, s_4_18)
        let s_4_20: Bits = Elem_read(state, tracer, s_4_19, s_4_14, s_4_18);
        // D s_4_21: write-var element2 <= s_4_20
        fn_state.element2 = s_4_20;
        // C s_4_22: const #2s : i
        let s_4_22: i128 = 2;
        // D s_4_23: read-var e:i64
        let s_4_23: i64 = fn_state.e;
        // D s_4_24: cast zx s_4_23 -> i
        let s_4_24: i128 = (i128::try_from(s_4_23).unwrap());
        // D s_4_25: mul s_4_24 s_4_22
        let s_4_25: i128 = ((s_4_24) * (s_4_22));
        // C s_4_26: const #1s : i
        let s_4_26: i128 = 1;
        // D s_4_27: add s_4_25 s_4_26
        let s_4_27: i128 = (s_4_25 + s_4_26);
        // D s_4_28: read-var esizeshadow#1274:i64
        let s_4_28: i64 = fn_state.esizeshadow_1274;
        // D s_4_29: cast zx s_4_28 -> i
        let s_4_29: i128 = (i128::try_from(s_4_28).unwrap());
        // D s_4_30: cast reint s_4_29 -> i64
        let s_4_30: i64 = (s_4_29 as i64);
        // D s_4_31: cast zx s_4_30 -> i
        let s_4_31: i128 = (i128::try_from(s_4_30).unwrap());
        // D s_4_32: read-var operand2:bv
        let s_4_32: Bits = fn_state.operand2;
        // D s_4_33: call Elem_read(s_4_32, s_4_27, s_4_31)
        let s_4_33: Bits = Elem_read(state, tracer, s_4_32, s_4_27, s_4_31);
        // D s_4_34: write-var element3 <= s_4_33
        fn_state.element3 = s_4_33;
        // C s_4_35: const #2s : i
        let s_4_35: i128 = 2;
        // D s_4_36: read-var e:i64
        let s_4_36: i64 = fn_state.e;
        // D s_4_37: cast zx s_4_36 -> i
        let s_4_37: i128 = (i128::try_from(s_4_36).unwrap());
        // D s_4_38: mul s_4_37 s_4_35
        let s_4_38: i128 = ((s_4_37) * (s_4_35));
        // D s_4_39: read-var esizeshadow#1274:i64
        let s_4_39: i64 = fn_state.esizeshadow_1274;
        // D s_4_40: cast zx s_4_39 -> i
        let s_4_40: i128 = (i128::try_from(s_4_39).unwrap());
        // D s_4_41: cast reint s_4_40 -> i64
        let s_4_41: i64 = (s_4_40 as i64);
        // D s_4_42: cast zx s_4_41 -> i
        let s_4_42: i128 = (i128::try_from(s_4_41).unwrap());
        // D s_4_43: read-var operand1:bv
        let s_4_43: Bits = fn_state.operand1;
        // D s_4_44: call Elem_read(s_4_43, s_4_38, s_4_42)
        let s_4_44: Bits = Elem_read(state, tracer, s_4_43, s_4_38, s_4_42);
        // D s_4_45: write-var element4 <= s_4_44
        fn_state.element4 = s_4_44;
        // N s_4_46: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#1274:i64
        let s_5_0: i64 = fn_state.esizeshadow_1274;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call __id(s_5_1)
        let s_5_2: i128 = u__id(state, tracer, s_5_1);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #16s : i
        let s_5_4: i128 = 16;
        // D s_5_5: cast zx s_5_3 -> i
        let s_5_5: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_6: cmp-eq s_5_5 s_5_4
        let s_5_6: bool = ((s_5_5) == (s_5_4));
        // N s_5_7: branch s_5_6 b13 b6
        if s_5_6 {
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
        // D s_6_0: read-var esizeshadow#1274:i64
        let s_6_0: i64 = fn_state.esizeshadow_1274;
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
        // D s_6_7: write-var gs#149612 <= s_6_6
        fn_state.gs_149612 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#149612:u8
        let s_7_0: bool = fn_state.gs_149612;
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
        // D s_8_0: read-var esizeshadow#1274:i64
        let s_8_0: i64 = fn_state.esizeshadow_1274;
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
        // D s_8_7: write-var gs#149614 <= s_8_6
        fn_state.gs_149614 = s_8_6;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#149614:u8
        let s_9_0: bool = fn_state.gs_149614;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // C s_9_2: const #2s : i
        let s_9_2: i128 = 2;
        // D s_9_3: read-var e:i64
        let s_9_3: i64 = fn_state.e;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: mul s_9_4 s_9_2
        let s_9_5: i128 = ((s_9_4) * (s_9_2));
        // D s_9_6: write-var ga#253400 <= s_9_5
        fn_state.ga_253400 = s_9_5;
        // D s_9_7: read-var esizeshadow#1274:i64
        let s_9_7: i64 = fn_state.esizeshadow_1274;
        // D s_9_8: cast zx s_9_7 -> i
        let s_9_8: i128 = (i128::try_from(s_9_7).unwrap());
        // D s_9_9: cast reint s_9_8 -> i64
        let s_9_9: i64 = (s_9_8 as i64);
        // D s_9_10: write-var ga#253401 <= s_9_9
        fn_state.ga_253401 = s_9_9;
        // C s_9_11: const #2s : i
        let s_9_11: i128 = 2;
        // D s_9_12: read-var e:i64
        let s_9_12: i64 = fn_state.e;
        // D s_9_13: cast zx s_9_12 -> i
        let s_9_13: i128 = (i128::try_from(s_9_12).unwrap());
        // D s_9_14: mul s_9_13 s_9_11
        let s_9_14: i128 = ((s_9_13) * (s_9_11));
        // D s_9_15: read-var esizeshadow#1274:i64
        let s_9_15: i64 = fn_state.esizeshadow_1274;
        // D s_9_16: cast zx s_9_15 -> i
        let s_9_16: i128 = (i128::try_from(s_9_15).unwrap());
        // D s_9_17: cast reint s_9_16 -> i64
        let s_9_17: i64 = (s_9_16 as i64);
        // D s_9_18: cast zx s_9_17 -> i
        let s_9_18: i128 = (i128::try_from(s_9_17).unwrap());
        // D s_9_19: read-var operand3:bv
        let s_9_19: Bits = fn_state.operand3;
        // D s_9_20: call Elem_read(s_9_19, s_9_14, s_9_18)
        let s_9_20: Bits = Elem_read(state, tracer, s_9_19, s_9_14, s_9_18);
        // D s_9_21: read-var element2:bv
        let s_9_21: Bits = fn_state.element2;
        // D s_9_22: read-var element1:bv
        let s_9_22: Bits = fn_state.element1;
        // D s_9_23: read-var fpcr:struct
        let s_9_23: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_9_24: call FPMulAdd(s_9_20, s_9_21, s_9_22, s_9_23)
        let s_9_24: Bits = FPMulAdd(state, tracer, s_9_20, s_9_21, s_9_22, s_9_23);
        // D s_9_25: write-var ga#253402 <= s_9_24
        fn_state.ga_253402 = s_9_24;
        // N s_9_26: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#253401:i64
        let s_10_0: i64 = fn_state.ga_253401;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var result:bv
        let s_10_2: Bits = fn_state.result;
        // D s_10_3: read-var ga#253400:i
        let s_10_3: i128 = fn_state.ga_253400;
        // D s_10_4: read-var ga#253402:bv
        let s_10_4: Bits = fn_state.ga_253402;
        // D s_10_5: call Elem_set(s_10_2, s_10_3, s_10_1, s_10_4)
        let s_10_5: Bits = Elem_set(state, tracer, s_10_2, s_10_3, s_10_1, s_10_4);
        // D s_10_6: write-var result <= s_10_5
        fn_state.result = s_10_5;
        // C s_10_7: const #2s : i
        let s_10_7: i128 = 2;
        // D s_10_8: read-var e:i64
        let s_10_8: i64 = fn_state.e;
        // D s_10_9: cast zx s_10_8 -> i
        let s_10_9: i128 = (i128::try_from(s_10_8).unwrap());
        // D s_10_10: mul s_10_9 s_10_7
        let s_10_10: i128 = ((s_10_9) * (s_10_7));
        // C s_10_11: const #1s : i
        let s_10_11: i128 = 1;
        // D s_10_12: add s_10_10 s_10_11
        let s_10_12: i128 = (s_10_10 + s_10_11);
        // D s_10_13: write-var ga#253408 <= s_10_12
        fn_state.ga_253408 = s_10_12;
        // D s_10_14: read-var esizeshadow#1274:i64
        let s_10_14: i64 = fn_state.esizeshadow_1274;
        // D s_10_15: cast zx s_10_14 -> i
        let s_10_15: i128 = (i128::try_from(s_10_14).unwrap());
        // D s_10_16: cast reint s_10_15 -> i64
        let s_10_16: i64 = (s_10_15 as i64);
        // D s_10_17: write-var ga#253409 <= s_10_16
        fn_state.ga_253409 = s_10_16;
        // C s_10_18: const #2s : i
        let s_10_18: i128 = 2;
        // D s_10_19: read-var e:i64
        let s_10_19: i64 = fn_state.e;
        // D s_10_20: cast zx s_10_19 -> i
        let s_10_20: i128 = (i128::try_from(s_10_19).unwrap());
        // D s_10_21: mul s_10_20 s_10_18
        let s_10_21: i128 = ((s_10_20) * (s_10_18));
        // C s_10_22: const #1s : i
        let s_10_22: i128 = 1;
        // D s_10_23: add s_10_21 s_10_22
        let s_10_23: i128 = (s_10_21 + s_10_22);
        // D s_10_24: read-var esizeshadow#1274:i64
        let s_10_24: i64 = fn_state.esizeshadow_1274;
        // D s_10_25: cast zx s_10_24 -> i
        let s_10_25: i128 = (i128::try_from(s_10_24).unwrap());
        // D s_10_26: cast reint s_10_25 -> i64
        let s_10_26: i64 = (s_10_25 as i64);
        // D s_10_27: cast zx s_10_26 -> i
        let s_10_27: i128 = (i128::try_from(s_10_26).unwrap());
        // D s_10_28: read-var operand3:bv
        let s_10_28: Bits = fn_state.operand3;
        // D s_10_29: call Elem_read(s_10_28, s_10_23, s_10_27)
        let s_10_29: Bits = Elem_read(state, tracer, s_10_28, s_10_23, s_10_27);
        // D s_10_30: read-var element4:bv
        let s_10_30: Bits = fn_state.element4;
        // D s_10_31: read-var element3:bv
        let s_10_31: Bits = fn_state.element3;
        // D s_10_32: read-var fpcr:struct
        let s_10_32: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_10_33: call FPMulAdd(s_10_29, s_10_30, s_10_31, s_10_32)
        let s_10_33: Bits = FPMulAdd(state, tracer, s_10_29, s_10_30, s_10_31, s_10_32);
        // D s_10_34: write-var ga#253410 <= s_10_33
        fn_state.ga_253410 = s_10_33;
        // N s_10_35: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#253409:i64
        let s_11_0: i64 = fn_state.ga_253409;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: read-var result:bv
        let s_11_2: Bits = fn_state.result;
        // D s_11_3: read-var ga#253408:i
        let s_11_3: i128 = fn_state.ga_253408;
        // D s_11_4: read-var ga#253410:bv
        let s_11_4: Bits = fn_state.ga_253410;
        // D s_11_5: call Elem_set(s_11_2, s_11_3, s_11_1, s_11_4)
        let s_11_5: Bits = Elem_set(state, tracer, s_11_2, s_11_3, s_11_1, s_11_4);
        // D s_11_6: write-var result <= s_11_5
        fn_state.result = s_11_5;
        // D s_11_7: read-var e:i64
        let s_11_7: i64 = fn_state.e;
        // C s_11_8: const #1s : i64
        let s_11_8: i64 = 1;
        // D s_11_9: add s_11_7 s_11_8
        let s_11_9: i64 = (s_11_7 + s_11_8);
        // D s_11_10: write-var e <= s_11_9
        fn_state.e = s_11_9;
        // N s_11_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#149614 <= s_12_0
        fn_state.gs_149614 = s_12_0;
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
        // D s_13_1: write-var gs#149612 <= s_13_0
        fn_state.gs_149612 = s_13_0;
        // N s_13_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var rot:u8
        let s_14_0: u8 = fn_state.rot;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 2u16);
        // C s_14_2: const #1u : u8
        let s_14_2: u8 = 1;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 2u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: not s_14_4
        let s_14_5: bool = !s_14_4;
        // N s_14_6: branch s_14_5 b23 b15
        if s_14_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esizeshadow#1274:i64
        let s_15_0: i64 = fn_state.esizeshadow_1274;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call __id(s_15_1)
        let s_15_2: i128 = u__id(state, tracer, s_15_1);
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: const #16s : i
        let s_15_4: i128 = 16;
        // D s_15_5: cast zx s_15_3 -> i
        let s_15_5: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_6: cmp-eq s_15_5 s_15_4
        let s_15_6: bool = ((s_15_5) == (s_15_4));
        // N s_15_7: branch s_15_6 b22 b16
        if s_15_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var esizeshadow#1274:i64
        let s_16_0: i64 = fn_state.esizeshadow_1274;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call __id(s_16_1)
        let s_16_2: i128 = u__id(state, tracer, s_16_1);
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: const #32s : i
        let s_16_4: i128 = 32;
        // D s_16_5: cast zx s_16_3 -> i
        let s_16_5: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_6: cmp-eq s_16_5 s_16_4
        let s_16_6: bool = ((s_16_5) == (s_16_4));
        // D s_16_7: write-var gs#149563 <= s_16_6
        fn_state.gs_149563 = s_16_6;
        // N s_16_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#149563:u8
        let s_17_0: bool = fn_state.gs_149563;
        // N s_17_1: branch s_17_0 b21 b18
        if s_17_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var esizeshadow#1274:i64
        let s_18_0: i64 = fn_state.esizeshadow_1274;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: call __id(s_18_1)
        let s_18_2: i128 = u__id(state, tracer, s_18_1);
        // D s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: const #64s : i
        let s_18_4: i128 = 64;
        // D s_18_5: cast zx s_18_3 -> i
        let s_18_5: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_6: cmp-eq s_18_5 s_18_4
        let s_18_6: bool = ((s_18_5) == (s_18_4));
        // D s_18_7: write-var gs#149565 <= s_18_6
        fn_state.gs_149565 = s_18_6;
        // N s_18_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#149565:u8
        let s_19_0: bool = fn_state.gs_149565;
        // N s_19_1: assert s_19_0
        let s_19_1: () = assert!(s_19_0);
        // C s_19_2: const #2s : i
        let s_19_2: i128 = 2;
        // D s_19_3: read-var e:i64
        let s_19_3: i64 = fn_state.e;
        // D s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (i128::try_from(s_19_3).unwrap());
        // D s_19_5: mul s_19_4 s_19_2
        let s_19_5: i128 = ((s_19_4) * (s_19_2));
        // C s_19_6: const #1s : i
        let s_19_6: i128 = 1;
        // D s_19_7: add s_19_5 s_19_6
        let s_19_7: i128 = (s_19_5 + s_19_6);
        // D s_19_8: read-var esizeshadow#1274:i64
        let s_19_8: i64 = fn_state.esizeshadow_1274;
        // D s_19_9: cast zx s_19_8 -> i
        let s_19_9: i128 = (i128::try_from(s_19_8).unwrap());
        // D s_19_10: cast reint s_19_9 -> i64
        let s_19_10: i64 = (s_19_9 as i64);
        // D s_19_11: cast zx s_19_10 -> i
        let s_19_11: i128 = (i128::try_from(s_19_10).unwrap());
        // D s_19_12: read-var operand2:bv
        let s_19_12: Bits = fn_state.operand2;
        // D s_19_13: call Elem_read(s_19_12, s_19_7, s_19_11)
        let s_19_13: Bits = Elem_read(state, tracer, s_19_12, s_19_7, s_19_11);
        // D s_19_14: call FPNeg(s_19_13)
        let s_19_14: Bits = FPNeg(state, tracer, s_19_13);
        // D s_19_15: write-var element1 <= s_19_14
        fn_state.element1 = s_19_14;
        // N s_19_16: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #2s : i
        let s_20_0: i128 = 2;
        // D s_20_1: read-var e:i64
        let s_20_1: i64 = fn_state.e;
        // D s_20_2: cast zx s_20_1 -> i
        let s_20_2: i128 = (i128::try_from(s_20_1).unwrap());
        // D s_20_3: mul s_20_2 s_20_0
        let s_20_3: i128 = ((s_20_2) * (s_20_0));
        // C s_20_4: const #1s : i
        let s_20_4: i128 = 1;
        // D s_20_5: add s_20_3 s_20_4
        let s_20_5: i128 = (s_20_3 + s_20_4);
        // D s_20_6: read-var esizeshadow#1274:i64
        let s_20_6: i64 = fn_state.esizeshadow_1274;
        // D s_20_7: cast zx s_20_6 -> i
        let s_20_7: i128 = (i128::try_from(s_20_6).unwrap());
        // D s_20_8: cast reint s_20_7 -> i64
        let s_20_8: i64 = (s_20_7 as i64);
        // D s_20_9: cast zx s_20_8 -> i
        let s_20_9: i128 = (i128::try_from(s_20_8).unwrap());
        // D s_20_10: read-var operand1:bv
        let s_20_10: Bits = fn_state.operand1;
        // D s_20_11: call Elem_read(s_20_10, s_20_5, s_20_9)
        let s_20_11: Bits = Elem_read(state, tracer, s_20_10, s_20_5, s_20_9);
        // D s_20_12: write-var element2 <= s_20_11
        fn_state.element2 = s_20_11;
        // C s_20_13: const #2s : i
        let s_20_13: i128 = 2;
        // D s_20_14: read-var e:i64
        let s_20_14: i64 = fn_state.e;
        // D s_20_15: cast zx s_20_14 -> i
        let s_20_15: i128 = (i128::try_from(s_20_14).unwrap());
        // D s_20_16: mul s_20_15 s_20_13
        let s_20_16: i128 = ((s_20_15) * (s_20_13));
        // D s_20_17: read-var esizeshadow#1274:i64
        let s_20_17: i64 = fn_state.esizeshadow_1274;
        // D s_20_18: cast zx s_20_17 -> i
        let s_20_18: i128 = (i128::try_from(s_20_17).unwrap());
        // D s_20_19: cast reint s_20_18 -> i64
        let s_20_19: i64 = (s_20_18 as i64);
        // D s_20_20: cast zx s_20_19 -> i
        let s_20_20: i128 = (i128::try_from(s_20_19).unwrap());
        // D s_20_21: read-var operand2:bv
        let s_20_21: Bits = fn_state.operand2;
        // D s_20_22: call Elem_read(s_20_21, s_20_16, s_20_20)
        let s_20_22: Bits = Elem_read(state, tracer, s_20_21, s_20_16, s_20_20);
        // D s_20_23: write-var element3 <= s_20_22
        fn_state.element3 = s_20_22;
        // C s_20_24: const #2s : i
        let s_20_24: i128 = 2;
        // D s_20_25: read-var e:i64
        let s_20_25: i64 = fn_state.e;
        // D s_20_26: cast zx s_20_25 -> i
        let s_20_26: i128 = (i128::try_from(s_20_25).unwrap());
        // D s_20_27: mul s_20_26 s_20_24
        let s_20_27: i128 = ((s_20_26) * (s_20_24));
        // C s_20_28: const #1s : i
        let s_20_28: i128 = 1;
        // D s_20_29: add s_20_27 s_20_28
        let s_20_29: i128 = (s_20_27 + s_20_28);
        // D s_20_30: read-var esizeshadow#1274:i64
        let s_20_30: i64 = fn_state.esizeshadow_1274;
        // D s_20_31: cast zx s_20_30 -> i
        let s_20_31: i128 = (i128::try_from(s_20_30).unwrap());
        // D s_20_32: cast reint s_20_31 -> i64
        let s_20_32: i64 = (s_20_31 as i64);
        // D s_20_33: cast zx s_20_32 -> i
        let s_20_33: i128 = (i128::try_from(s_20_32).unwrap());
        // D s_20_34: read-var operand1:bv
        let s_20_34: Bits = fn_state.operand1;
        // D s_20_35: call Elem_read(s_20_34, s_20_29, s_20_33)
        let s_20_35: Bits = Elem_read(state, tracer, s_20_34, s_20_29, s_20_33);
        // D s_20_36: write-var element4 <= s_20_35
        fn_state.element4 = s_20_35;
        // N s_20_37: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#149565 <= s_21_0
        fn_state.gs_149565 = s_21_0;
        // N s_21_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#149563 <= s_22_0
        fn_state.gs_149563 = s_22_0;
        // N s_22_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var rot:u8
        let s_23_0: u8 = fn_state.rot;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 2u16);
        // C s_23_2: const #2u : u8
        let s_23_2: u8 = 2;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 2u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: not s_23_4
        let s_23_5: bool = !s_23_4;
        // N s_23_6: branch s_23_5 b33 b24
        if s_23_5 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var esizeshadow#1274:i64
        let s_24_0: i64 = fn_state.esizeshadow_1274;
        // D s_24_1: cast zx s_24_0 -> i
        let s_24_1: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_2: call __id(s_24_1)
        let s_24_2: i128 = u__id(state, tracer, s_24_1);
        // D s_24_3: cast reint s_24_2 -> i64
        let s_24_3: i64 = (s_24_2 as i64);
        // C s_24_4: const #16s : i
        let s_24_4: i128 = 16;
        // D s_24_5: cast zx s_24_3 -> i
        let s_24_5: i128 = (i128::try_from(s_24_3).unwrap());
        // D s_24_6: cmp-eq s_24_5 s_24_4
        let s_24_6: bool = ((s_24_5) == (s_24_4));
        // N s_24_7: branch s_24_6 b32 b25
        if s_24_6 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var esizeshadow#1274:i64
        let s_25_0: i64 = fn_state.esizeshadow_1274;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: call __id(s_25_1)
        let s_25_2: i128 = u__id(state, tracer, s_25_1);
        // D s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: const #32s : i
        let s_25_4: i128 = 32;
        // D s_25_5: cast zx s_25_3 -> i
        let s_25_5: i128 = (i128::try_from(s_25_3).unwrap());
        // D s_25_6: cmp-eq s_25_5 s_25_4
        let s_25_6: bool = ((s_25_5) == (s_25_4));
        // D s_25_7: write-var gs#149580 <= s_25_6
        fn_state.gs_149580 = s_25_6;
        // N s_25_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#149580:u8
        let s_26_0: bool = fn_state.gs_149580;
        // N s_26_1: branch s_26_0 b31 b27
        if s_26_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var esizeshadow#1274:i64
        let s_27_0: i64 = fn_state.esizeshadow_1274;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: call __id(s_27_1)
        let s_27_2: i128 = u__id(state, tracer, s_27_1);
        // D s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: const #64s : i
        let s_27_4: i128 = 64;
        // D s_27_5: cast zx s_27_3 -> i
        let s_27_5: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_6: cmp-eq s_27_5 s_27_4
        let s_27_6: bool = ((s_27_5) == (s_27_4));
        // D s_27_7: write-var gs#149582 <= s_27_6
        fn_state.gs_149582 = s_27_6;
        // N s_27_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#149582:u8
        let s_28_0: bool = fn_state.gs_149582;
        // N s_28_1: assert s_28_0
        let s_28_1: () = assert!(s_28_0);
        // C s_28_2: const #2s : i
        let s_28_2: i128 = 2;
        // D s_28_3: read-var e:i64
        let s_28_3: i64 = fn_state.e;
        // D s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // D s_28_5: mul s_28_4 s_28_2
        let s_28_5: i128 = ((s_28_4) * (s_28_2));
        // D s_28_6: read-var esizeshadow#1274:i64
        let s_28_6: i64 = fn_state.esizeshadow_1274;
        // D s_28_7: cast zx s_28_6 -> i
        let s_28_7: i128 = (i128::try_from(s_28_6).unwrap());
        // D s_28_8: cast reint s_28_7 -> i64
        let s_28_8: i64 = (s_28_7 as i64);
        // D s_28_9: cast zx s_28_8 -> i
        let s_28_9: i128 = (i128::try_from(s_28_8).unwrap());
        // D s_28_10: read-var operand2:bv
        let s_28_10: Bits = fn_state.operand2;
        // D s_28_11: call Elem_read(s_28_10, s_28_5, s_28_9)
        let s_28_11: Bits = Elem_read(state, tracer, s_28_10, s_28_5, s_28_9);
        // D s_28_12: call FPNeg(s_28_11)
        let s_28_12: Bits = FPNeg(state, tracer, s_28_11);
        // D s_28_13: write-var element1 <= s_28_12
        fn_state.element1 = s_28_12;
        // N s_28_14: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #2s : i
        let s_29_0: i128 = 2;
        // D s_29_1: read-var e:i64
        let s_29_1: i64 = fn_state.e;
        // D s_29_2: cast zx s_29_1 -> i
        let s_29_2: i128 = (i128::try_from(s_29_1).unwrap());
        // D s_29_3: mul s_29_2 s_29_0
        let s_29_3: i128 = ((s_29_2) * (s_29_0));
        // D s_29_4: read-var esizeshadow#1274:i64
        let s_29_4: i64 = fn_state.esizeshadow_1274;
        // D s_29_5: cast zx s_29_4 -> i
        let s_29_5: i128 = (i128::try_from(s_29_4).unwrap());
        // D s_29_6: cast reint s_29_5 -> i64
        let s_29_6: i64 = (s_29_5 as i64);
        // D s_29_7: cast zx s_29_6 -> i
        let s_29_7: i128 = (i128::try_from(s_29_6).unwrap());
        // D s_29_8: read-var operand1:bv
        let s_29_8: Bits = fn_state.operand1;
        // D s_29_9: call Elem_read(s_29_8, s_29_3, s_29_7)
        let s_29_9: Bits = Elem_read(state, tracer, s_29_8, s_29_3, s_29_7);
        // D s_29_10: write-var element2 <= s_29_9
        fn_state.element2 = s_29_9;
        // C s_29_11: const #2s : i
        let s_29_11: i128 = 2;
        // D s_29_12: read-var e:i64
        let s_29_12: i64 = fn_state.e;
        // D s_29_13: cast zx s_29_12 -> i
        let s_29_13: i128 = (i128::try_from(s_29_12).unwrap());
        // D s_29_14: mul s_29_13 s_29_11
        let s_29_14: i128 = ((s_29_13) * (s_29_11));
        // C s_29_15: const #1s : i
        let s_29_15: i128 = 1;
        // D s_29_16: add s_29_14 s_29_15
        let s_29_16: i128 = (s_29_14 + s_29_15);
        // D s_29_17: read-var esizeshadow#1274:i64
        let s_29_17: i64 = fn_state.esizeshadow_1274;
        // D s_29_18: cast zx s_29_17 -> i
        let s_29_18: i128 = (i128::try_from(s_29_17).unwrap());
        // D s_29_19: cast reint s_29_18 -> i64
        let s_29_19: i64 = (s_29_18 as i64);
        // D s_29_20: cast zx s_29_19 -> i
        let s_29_20: i128 = (i128::try_from(s_29_19).unwrap());
        // D s_29_21: read-var operand2:bv
        let s_29_21: Bits = fn_state.operand2;
        // D s_29_22: call Elem_read(s_29_21, s_29_16, s_29_20)
        let s_29_22: Bits = Elem_read(state, tracer, s_29_21, s_29_16, s_29_20);
        // D s_29_23: call FPNeg(s_29_22)
        let s_29_23: Bits = FPNeg(state, tracer, s_29_22);
        // D s_29_24: write-var element3 <= s_29_23
        fn_state.element3 = s_29_23;
        // N s_29_25: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #2s : i
        let s_30_0: i128 = 2;
        // D s_30_1: read-var e:i64
        let s_30_1: i64 = fn_state.e;
        // D s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (i128::try_from(s_30_1).unwrap());
        // D s_30_3: mul s_30_2 s_30_0
        let s_30_3: i128 = ((s_30_2) * (s_30_0));
        // D s_30_4: read-var esizeshadow#1274:i64
        let s_30_4: i64 = fn_state.esizeshadow_1274;
        // D s_30_5: cast zx s_30_4 -> i
        let s_30_5: i128 = (i128::try_from(s_30_4).unwrap());
        // D s_30_6: cast reint s_30_5 -> i64
        let s_30_6: i64 = (s_30_5 as i64);
        // D s_30_7: cast zx s_30_6 -> i
        let s_30_7: i128 = (i128::try_from(s_30_6).unwrap());
        // D s_30_8: read-var operand1:bv
        let s_30_8: Bits = fn_state.operand1;
        // D s_30_9: call Elem_read(s_30_8, s_30_3, s_30_7)
        let s_30_9: Bits = Elem_read(state, tracer, s_30_8, s_30_3, s_30_7);
        // D s_30_10: write-var element4 <= s_30_9
        fn_state.element4 = s_30_9;
        // N s_30_11: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#149582 <= s_31_0
        fn_state.gs_149582 = s_31_0;
        // N s_31_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#149580 <= s_32_0
        fn_state.gs_149580 = s_32_0;
        // N s_32_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #2s : i
        let s_33_0: i128 = 2;
        // D s_33_1: read-var e:i64
        let s_33_1: i64 = fn_state.e;
        // D s_33_2: cast zx s_33_1 -> i
        let s_33_2: i128 = (i128::try_from(s_33_1).unwrap());
        // D s_33_3: mul s_33_2 s_33_0
        let s_33_3: i128 = ((s_33_2) * (s_33_0));
        // C s_33_4: const #1s : i
        let s_33_4: i128 = 1;
        // D s_33_5: add s_33_3 s_33_4
        let s_33_5: i128 = (s_33_3 + s_33_4);
        // D s_33_6: read-var esizeshadow#1274:i64
        let s_33_6: i64 = fn_state.esizeshadow_1274;
        // D s_33_7: cast zx s_33_6 -> i
        let s_33_7: i128 = (i128::try_from(s_33_6).unwrap());
        // D s_33_8: cast reint s_33_7 -> i64
        let s_33_8: i64 = (s_33_7 as i64);
        // D s_33_9: cast zx s_33_8 -> i
        let s_33_9: i128 = (i128::try_from(s_33_8).unwrap());
        // D s_33_10: read-var operand2:bv
        let s_33_10: Bits = fn_state.operand2;
        // D s_33_11: call Elem_read(s_33_10, s_33_5, s_33_9)
        let s_33_11: Bits = Elem_read(state, tracer, s_33_10, s_33_5, s_33_9);
        // D s_33_12: write-var element1 <= s_33_11
        fn_state.element1 = s_33_11;
        // C s_33_13: const #2s : i
        let s_33_13: i128 = 2;
        // D s_33_14: read-var e:i64
        let s_33_14: i64 = fn_state.e;
        // D s_33_15: cast zx s_33_14 -> i
        let s_33_15: i128 = (i128::try_from(s_33_14).unwrap());
        // D s_33_16: mul s_33_15 s_33_13
        let s_33_16: i128 = ((s_33_15) * (s_33_13));
        // C s_33_17: const #1s : i
        let s_33_17: i128 = 1;
        // D s_33_18: add s_33_16 s_33_17
        let s_33_18: i128 = (s_33_16 + s_33_17);
        // D s_33_19: read-var esizeshadow#1274:i64
        let s_33_19: i64 = fn_state.esizeshadow_1274;
        // D s_33_20: cast zx s_33_19 -> i
        let s_33_20: i128 = (i128::try_from(s_33_19).unwrap());
        // D s_33_21: cast reint s_33_20 -> i64
        let s_33_21: i64 = (s_33_20 as i64);
        // D s_33_22: cast zx s_33_21 -> i
        let s_33_22: i128 = (i128::try_from(s_33_21).unwrap());
        // D s_33_23: read-var operand1:bv
        let s_33_23: Bits = fn_state.operand1;
        // D s_33_24: call Elem_read(s_33_23, s_33_18, s_33_22)
        let s_33_24: Bits = Elem_read(state, tracer, s_33_23, s_33_18, s_33_22);
        // D s_33_25: write-var element2 <= s_33_24
        fn_state.element2 = s_33_24;
        // D s_33_26: read-var esizeshadow#1274:i64
        let s_33_26: i64 = fn_state.esizeshadow_1274;
        // D s_33_27: cast zx s_33_26 -> i
        let s_33_27: i128 = (i128::try_from(s_33_26).unwrap());
        // D s_33_28: call __id(s_33_27)
        let s_33_28: i128 = u__id(state, tracer, s_33_27);
        // D s_33_29: cast reint s_33_28 -> i64
        let s_33_29: i64 = (s_33_28 as i64);
        // C s_33_30: const #16s : i
        let s_33_30: i128 = 16;
        // D s_33_31: cast zx s_33_29 -> i
        let s_33_31: i128 = (i128::try_from(s_33_29).unwrap());
        // D s_33_32: cmp-eq s_33_31 s_33_30
        let s_33_32: bool = ((s_33_31) == (s_33_30));
        // N s_33_33: branch s_33_32 b40 b34
        if s_33_32 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var esizeshadow#1274:i64
        let s_34_0: i64 = fn_state.esizeshadow_1274;
        // D s_34_1: cast zx s_34_0 -> i
        let s_34_1: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_2: call __id(s_34_1)
        let s_34_2: i128 = u__id(state, tracer, s_34_1);
        // D s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: const #32s : i
        let s_34_4: i128 = 32;
        // D s_34_5: cast zx s_34_3 -> i
        let s_34_5: i128 = (i128::try_from(s_34_3).unwrap());
        // D s_34_6: cmp-eq s_34_5 s_34_4
        let s_34_6: bool = ((s_34_5) == (s_34_4));
        // D s_34_7: write-var gs#149599 <= s_34_6
        fn_state.gs_149599 = s_34_6;
        // N s_34_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#149599:u8
        let s_35_0: bool = fn_state.gs_149599;
        // N s_35_1: branch s_35_0 b39 b36
        if s_35_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var esizeshadow#1274:i64
        let s_36_0: i64 = fn_state.esizeshadow_1274;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: call __id(s_36_1)
        let s_36_2: i128 = u__id(state, tracer, s_36_1);
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: const #64s : i
        let s_36_4: i128 = 64;
        // D s_36_5: cast zx s_36_3 -> i
        let s_36_5: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_6: cmp-eq s_36_5 s_36_4
        let s_36_6: bool = ((s_36_5) == (s_36_4));
        // D s_36_7: write-var gs#149601 <= s_36_6
        fn_state.gs_149601 = s_36_6;
        // N s_36_8: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#149601:u8
        let s_37_0: bool = fn_state.gs_149601;
        // N s_37_1: assert s_37_0
        let s_37_1: () = assert!(s_37_0);
        // C s_37_2: const #2s : i
        let s_37_2: i128 = 2;
        // D s_37_3: read-var e:i64
        let s_37_3: i64 = fn_state.e;
        // D s_37_4: cast zx s_37_3 -> i
        let s_37_4: i128 = (i128::try_from(s_37_3).unwrap());
        // D s_37_5: mul s_37_4 s_37_2
        let s_37_5: i128 = ((s_37_4) * (s_37_2));
        // D s_37_6: read-var esizeshadow#1274:i64
        let s_37_6: i64 = fn_state.esizeshadow_1274;
        // D s_37_7: cast zx s_37_6 -> i
        let s_37_7: i128 = (i128::try_from(s_37_6).unwrap());
        // D s_37_8: cast reint s_37_7 -> i64
        let s_37_8: i64 = (s_37_7 as i64);
        // D s_37_9: cast zx s_37_8 -> i
        let s_37_9: i128 = (i128::try_from(s_37_8).unwrap());
        // D s_37_10: read-var operand2:bv
        let s_37_10: Bits = fn_state.operand2;
        // D s_37_11: call Elem_read(s_37_10, s_37_5, s_37_9)
        let s_37_11: Bits = Elem_read(state, tracer, s_37_10, s_37_5, s_37_9);
        // D s_37_12: call FPNeg(s_37_11)
        let s_37_12: Bits = FPNeg(state, tracer, s_37_11);
        // D s_37_13: write-var element3 <= s_37_12
        fn_state.element3 = s_37_12;
        // N s_37_14: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #2s : i
        let s_38_0: i128 = 2;
        // D s_38_1: read-var e:i64
        let s_38_1: i64 = fn_state.e;
        // D s_38_2: cast zx s_38_1 -> i
        let s_38_2: i128 = (i128::try_from(s_38_1).unwrap());
        // D s_38_3: mul s_38_2 s_38_0
        let s_38_3: i128 = ((s_38_2) * (s_38_0));
        // C s_38_4: const #1s : i
        let s_38_4: i128 = 1;
        // D s_38_5: add s_38_3 s_38_4
        let s_38_5: i128 = (s_38_3 + s_38_4);
        // D s_38_6: read-var esizeshadow#1274:i64
        let s_38_6: i64 = fn_state.esizeshadow_1274;
        // D s_38_7: cast zx s_38_6 -> i
        let s_38_7: i128 = (i128::try_from(s_38_6).unwrap());
        // D s_38_8: cast reint s_38_7 -> i64
        let s_38_8: i64 = (s_38_7 as i64);
        // D s_38_9: cast zx s_38_8 -> i
        let s_38_9: i128 = (i128::try_from(s_38_8).unwrap());
        // D s_38_10: read-var operand1:bv
        let s_38_10: Bits = fn_state.operand1;
        // D s_38_11: call Elem_read(s_38_10, s_38_5, s_38_9)
        let s_38_11: Bits = Elem_read(state, tracer, s_38_10, s_38_5, s_38_9);
        // D s_38_12: write-var element4 <= s_38_11
        fn_state.element4 = s_38_11;
        // N s_38_13: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#149601 <= s_39_0
        fn_state.gs_149601 = s_39_0;
        // N s_39_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#149599 <= s_40_0
        fn_state.gs_149599 = s_40_0;
        // N s_40_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var datasizeshadow#1275:i64
        let s_41_0: i64 = fn_state.datasizeshadow_1275;
        // D s_41_1: cast zx s_41_0 -> i
        let s_41_1: i128 = (i128::try_from(s_41_0).unwrap());
        // D s_41_2: cast reint s_41_1 -> i64
        let s_41_2: i64 = (s_41_1 as i64);
        // D s_41_3: read-var d:i64
        let s_41_3: i64 = fn_state.d;
        // D s_41_4: cast zx s_41_3 -> i
        let s_41_4: i128 = (i128::try_from(s_41_3).unwrap());
        // D s_41_5: read-var result:bv
        let s_41_5: Bits = fn_state.result;
        // D s_41_6: call V_set(s_41_4, s_41_2, s_41_5)
        let s_41_6: () = V_set(state, tracer, s_41_4, s_41_2, s_41_5);
        // N s_41_7: return
        return;
    }
}
