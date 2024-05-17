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
use ShiftSat::*;
use CheckStreamingSVEEnabled::*;
use Elem_set::*;
use u_shl_int_general::*;
use integer_subrange::*;
use u_shr_int_general::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_URSHL_MZ_ZZW_2x2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    m: i64,
    nreg: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_285353: i64,
        r: i64,
        e: i64,
        esizeshadow_6653: i64,
        shift: i128,
        gs_285380: i64,
        res: i128,
        gs_285359: i64,
        VLshadow_6654: i64,
        VLshadow_6655: i64,
        elements: i64,
        element: i128,
        operand1: Bits,
        u_8835: i64,
        results: [Bits; 4usize],
        operand2: Bits,
        VL: i64,
        dn: i64,
        esize: i64,
        m: i64,
        nreg: i64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        m,
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
        // D s_0_3: write-var esizeshadow#6653 <= s_0_2
        fn_state.esizeshadow_6653 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6654 <= s_0_6
        fn_state.VLshadow_6654 = s_0_6;
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
        // D s_1_0: read-var VLshadow#6654:i64
        let s_1_0: i64 = fn_state.VLshadow_6654;
        // D s_1_1: write-var VLshadow#6655 <= s_1_0
        fn_state.VLshadow_6655 = s_1_0;
        // D s_1_2: read-var VLshadow#6655:i64
        let s_1_2: i64 = fn_state.VLshadow_6655;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#6653:i64
        let s_1_4: i64 = fn_state.esizeshadow_6653;
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
        // D s_1_15: write-var gs#285353 <= s_1_14
        fn_state.gs_285353 = s_1_14;
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
        // D s_2_1: read-var gs#285353:i64
        let s_2_1: i64 = fn_state.gs_285353;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var dn:i64
        let s_3_0: i64 = fn_state.dn;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var r:i64
        let s_3_2: i64 = fn_state.r;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: add s_3_1 s_3_3
        let s_3_4: i128 = (s_3_1 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var VLshadow#6655:i64
        let s_3_6: i64 = fn_state.VLshadow_6655;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: cast zx s_3_5 -> i
        let s_3_9: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_10: cast zx s_3_8 -> i
        let s_3_10: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_11: call Z_read(s_3_9, s_3_10)
        let s_3_11: Bits = Z_read(state, tracer, s_3_9, s_3_10);
        // D s_3_12: write-var operand1 <= s_3_11
        fn_state.operand1 = s_3_11;
        // D s_3_13: read-var m:i64
        let s_3_13: i64 = fn_state.m;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: read-var r:i64
        let s_3_15: i64 = fn_state.r;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: add s_3_14 s_3_16
        let s_3_17: i128 = (s_3_14 + s_3_16);
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: read-var VLshadow#6655:i64
        let s_3_19: i64 = fn_state.VLshadow_6655;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: cast zx s_3_18 -> i
        let s_3_22: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_23: cast zx s_3_21 -> i
        let s_3_23: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_24: call Z_read(s_3_22, s_3_23)
        let s_3_24: Bits = Z_read(state, tracer, s_3_22, s_3_23);
        // D s_3_25: write-var operand2 <= s_3_24
        fn_state.operand2 = s_3_24;
        // C s_3_26: const #0s : i64
        let s_3_26: i64 = 0;
        // C s_3_27: const #1s : i
        let s_3_27: i128 = 1;
        // D s_3_28: read-var elements:i64
        let s_3_28: i64 = fn_state.elements;
        // D s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_30: sub s_3_29 s_3_27
        let s_3_30: i128 = ((s_3_29) - (s_3_27));
        // D s_3_31: cast reint s_3_30 -> i64
        let s_3_31: i64 = (s_3_30 as i64);
        // D s_3_32: write-var gs#285359 <= s_3_31
        fn_state.gs_285359 = s_3_31;
        // D s_3_33: write-var e <= s_3_26
        fn_state.e = s_3_26;
        // N s_3_34: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#285359:i64
        let s_4_1: i64 = fn_state.gs_285359;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#6653:i64
        let s_5_0: i64 = fn_state.esizeshadow_6653;
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
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (s_5_7.value() as i128);
        // D s_5_9: write-var element <= s_5_8
        fn_state.element = s_5_8;
        // D s_5_10: read-var esizeshadow#6653:i64
        let s_5_10: i64 = fn_state.esizeshadow_6653;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: cast reint s_5_11 -> i64
        let s_5_12: i64 = (s_5_11 as i64);
        // D s_5_13: read-var e:i64
        let s_5_13: i64 = fn_state.e;
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_15: cast zx s_5_12 -> i
        let s_5_15: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_16: read-var operand2:bv
        let s_5_16: Bits = fn_state.operand2;
        // D s_5_17: call Elem_read(s_5_16, s_5_14, s_5_15)
        let s_5_17: Bits = Elem_read(state, tracer, s_5_16, s_5_14, s_5_15);
        // D s_5_18: cast sx s_5_17 -> i
        let s_5_18: i128 = {
            let sign_bit = s_5_17.length() - 1;
            let mut result = s_5_17.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_19: cast reint s_5_18 -> i64
        let s_5_19: i64 = (s_5_18 as i64);
        // D s_5_20: cast zx s_5_19 -> i
        let s_5_20: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_21: read-var esizeshadow#6653:i64
        let s_5_21: i64 = fn_state.esizeshadow_6653;
        // D s_5_22: cast zx s_5_21 -> i
        let s_5_22: i128 = (i128::try_from(s_5_21).unwrap());
        // D s_5_23: call ShiftSat(s_5_20, s_5_22)
        let s_5_23: i128 = ShiftSat(state, tracer, s_5_20, s_5_22);
        // D s_5_24: write-var shift <= s_5_23
        fn_state.shift = s_5_23;
        // C s_5_25: const #0s : i
        let s_5_25: i128 = 0;
        // D s_5_26: read-var shift:i
        let s_5_26: i128 = fn_state.shift;
        // D s_5_27: cmp-ge s_5_26 s_5_25
        let s_5_27: bool = ((s_5_26) >= (s_5_25));
        // N s_5_28: branch s_5_27 b8 b6
        if s_5_27 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var shift:i
        let s_6_0: i128 = fn_state.shift;
        // D s_6_1: neg s_6_0
        let s_6_1: i128 = -s_6_0;
        // C s_6_2: const #1s : i
        let s_6_2: i128 = 1;
        // D s_6_3: sub s_6_1 s_6_2
        let s_6_3: i128 = ((s_6_1) - (s_6_2));
        // C s_6_4: const #1s : i
        let s_6_4: i128 = 1;
        // D s_6_5: call _shl_int_general(s_6_4, s_6_3)
        let s_6_5: i128 = u_shl_int_general(state, tracer, s_6_4, s_6_3);
        // D s_6_6: read-var element:i
        let s_6_6: i128 = fn_state.element;
        // D s_6_7: add s_6_6 s_6_5
        let s_6_7: i128 = (s_6_6 + s_6_5);
        // D s_6_8: call _shr_int_general(s_6_7, s_6_1)
        let s_6_8: i128 = u_shr_int_general(state, tracer, s_6_7, s_6_1);
        // D s_6_9: write-var res <= s_6_8
        fn_state.res = s_6_8;
        // N s_6_10: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var res:i
        let s_7_0: i128 = fn_state.res;
        // D s_7_1: read-var results:[bv; 4]
        let s_7_1: [Bits; 4usize] = fn_state.results;
        // D s_7_2: cast cvt s_7_1 -> [bv; 0]
        let s_7_2: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_7_1);
        // D s_7_3: read-var r:i64
        let s_7_3: i64 = fn_state.r;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: read-element s_7_2[s_7_4]
        let s_7_5: Bits = s_7_2[(s_7_4) as usize];
        // D s_7_6: read-var esizeshadow#6653:i64
        let s_7_6: i64 = fn_state.esizeshadow_6653;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // C s_7_9: const #1s : i
        let s_7_9: i128 = 1;
        // D s_7_10: read-var esizeshadow#6653:i64
        let s_7_10: i64 = fn_state.esizeshadow_6653;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: sub s_7_11 s_7_9
        let s_7_12: i128 = ((s_7_11) - (s_7_9));
        // D s_7_13: cast reint s_7_12 -> i64
        let s_7_13: i64 = (s_7_12 as i64);
        // C s_7_14: const #0s : i
        let s_7_14: i128 = 0;
        // D s_7_15: cast zx s_7_13 -> i
        let s_7_15: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_16: call integer_subrange(s_7_0, s_7_15, s_7_14)
        let s_7_16: Bits = integer_subrange(state, tracer, s_7_0, s_7_15, s_7_14);
        // D s_7_17: read-var e:i64
        let s_7_17: i64 = fn_state.e;
        // D s_7_18: cast zx s_7_17 -> i
        let s_7_18: i128 = (i128::try_from(s_7_17).unwrap());
        // D s_7_19: cast zx s_7_8 -> i
        let s_7_19: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_20: call Elem_set(s_7_5, s_7_18, s_7_19, s_7_16)
        let s_7_20: Bits = Elem_set(state, tracer, s_7_5, s_7_18, s_7_19, s_7_16);
        // D s_7_21: read-var results:[bv; 4]
        let s_7_21: [Bits; 4usize] = fn_state.results;
        // D s_7_22: cast cvt s_7_21 -> [bv; 0]
        let s_7_22: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_7_21);
        // D s_7_23: read-var r:i64
        let s_7_23: i64 = fn_state.r;
        // D s_7_24: cast zx s_7_23 -> i
        let s_7_24: i128 = (i128::try_from(s_7_23).unwrap());
        // D s_7_25: mutate-element s_7_22[s_7_24] <= s_7_20
        let s_7_25: alloc::vec::Vec<Bits> = {
            let mut local = s_7_22.clone();
            local[(s_7_24) as usize] = s_7_20;
            local
        };
        // D s_7_26: cast cvt s_7_25 -> [bv; 4]
        let s_7_26: [Bits; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_7_25);
            buf
        };
        // D s_7_27: write-var results <= s_7_26
        fn_state.results = s_7_26;
        // D s_7_28: read-var e:i64
        let s_7_28: i64 = fn_state.e;
        // C s_7_29: const #1s : i64
        let s_7_29: i64 = 1;
        // D s_7_30: add s_7_28 s_7_29
        let s_7_30: i64 = (s_7_28 + s_7_29);
        // D s_7_31: write-var e <= s_7_30
        fn_state.e = s_7_30;
        // N s_7_32: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var element:i
        let s_8_0: i128 = fn_state.element;
        // D s_8_1: read-var shift:i
        let s_8_1: i128 = fn_state.shift;
        // D s_8_2: call _shl_int_general(s_8_0, s_8_1)
        let s_8_2: i128 = u_shl_int_general(state, tracer, s_8_0, s_8_1);
        // D s_8_3: write-var res <= s_8_2
        fn_state.res = s_8_2;
        // N s_8_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var r:i64
        let s_9_0: i64 = fn_state.r;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var r <= s_9_2
        fn_state.r = s_9_2;
        // N s_9_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i64
        let s_10_0: i64 = 0;
        // C s_10_1: const #1s : i
        let s_10_1: i128 = 1;
        // D s_10_2: read-var nreg:i64
        let s_10_2: i64 = fn_state.nreg;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: sub s_10_3 s_10_1
        let s_10_4: i128 = ((s_10_3) - (s_10_1));
        // D s_10_5: cast reint s_10_4 -> i64
        let s_10_5: i64 = (s_10_4 as i64);
        // D s_10_6: write-var gs#285380 <= s_10_5
        fn_state.gs_285380 = s_10_5;
        // D s_10_7: write-var u#8835 <= s_10_0
        fn_state.u_8835 = s_10_0;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var u#8835:i64
        let s_11_0: i64 = fn_state.u_8835;
        // D s_11_1: read-var gs#285380:i64
        let s_11_1: i64 = fn_state.gs_285380;
        // D s_11_2: cmp-gt s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) > (s_11_1));
        // N s_11_3: branch s_11_2 b13 b12
        if s_11_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var dn:i64
        let s_12_0: i64 = fn_state.dn;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var u#8835:i64
        let s_12_2: i64 = fn_state.u_8835;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: add s_12_1 s_12_3
        let s_12_4: i128 = (s_12_1 + s_12_3);
        // D s_12_5: cast reint s_12_4 -> i64
        let s_12_5: i64 = (s_12_4 as i64);
        // D s_12_6: read-var VLshadow#6655:i64
        let s_12_6: i64 = fn_state.VLshadow_6655;
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_8: cast reint s_12_7 -> i64
        let s_12_8: i64 = (s_12_7 as i64);
        // D s_12_9: read-var results:[bv; 4]
        let s_12_9: [Bits; 4usize] = fn_state.results;
        // D s_12_10: cast cvt s_12_9 -> [bv; 0]
        let s_12_10: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_12_9);
        // D s_12_11: read-var u#8835:i64
        let s_12_11: i64 = fn_state.u_8835;
        // D s_12_12: cast zx s_12_11 -> i
        let s_12_12: i128 = (i128::try_from(s_12_11).unwrap());
        // D s_12_13: read-element s_12_10[s_12_12]
        let s_12_13: Bits = s_12_10[(s_12_12) as usize];
        // D s_12_14: cast zx s_12_5 -> i
        let s_12_14: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_15: cast zx s_12_8 -> i
        let s_12_15: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_16: call Z_set(s_12_14, s_12_15, s_12_13)
        let s_12_16: () = Z_set(state, tracer, s_12_14, s_12_15, s_12_13);
        // D s_12_17: read-var u#8835:i64
        let s_12_17: i64 = fn_state.u_8835;
        // C s_12_18: const #1s : i64
        let s_12_18: i64 = 1;
        // D s_12_19: add s_12_17 s_12_18
        let s_12_19: i64 = (s_12_17 + s_12_18);
        // D s_12_20: write-var u#8835 <= s_12_19
        fn_state.u_8835 = s_12_19;
        // N s_12_21: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: return
        return;
    }
}
