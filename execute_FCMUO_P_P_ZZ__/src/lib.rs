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
use FPCompareNE::*;
use CheckSVEEnabled::*;
use FPCR_read::*;
use FPCompareUN::*;
use Elem_read::*;
use Zeros::*;
use P_set::*;
use FPCompareGT::*;
use Z_read::*;
use FPCompareEQ::*;
use Elem_set::*;
use u__id::*;
use FPCompareGE::*;
use AnyActiveElement::*;
use P_read::*;
use ActivePredicateElement::*;
use common::*;
pub fn execute_FCMUO_P_P_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    m: i64,
    n: i64,
    op: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_179344: bool,
        gs_179374: bool,
        gs_179379: bool,
        mask: Bits,
        element2: Bits,
        gs_179365: bool,
        element1: Bits,
        psize: i64,
        VLshadow_2226: i64,
        elements: i64,
        gs_179381: bool,
        operand1: Bits,
        gs_179388: bool,
        operand2: Bits,
        e: i64,
        gs_179360: bool,
        gs_179337: i64,
        res: bool,
        PL: i64,
        gs_179386: bool,
        gs_179353: bool,
        gs_179346: bool,
        gs_179351: bool,
        pbit: bool,
        esizeshadow_2224: i64,
        VLshadow_2225: i64,
        gs_179367: bool,
        result: Bits,
        gs_179358: bool,
        gs_179372: bool,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        m: i64,
        n: i64,
        op: u32,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
        m,
        n,
        op,
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
        // D s_0_3: write-var esizeshadow#2224 <= s_0_2
        fn_state.esizeshadow_2224 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2225 <= s_0_6
        fn_state.VLshadow_2225 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2225:i64
        let s_1_0: i64 = fn_state.VLshadow_2225;
        // D s_1_1: write-var VLshadow#2226 <= s_1_0
        fn_state.VLshadow_2226 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2226:i64
        let s_1_3: i64 = fn_state.VLshadow_2226;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: write-var PL <= s_1_6
        fn_state.PL = s_1_6;
        // D s_1_8: read-var VLshadow#2226:i64
        let s_1_8: i64 = fn_state.VLshadow_2226;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: read-var esizeshadow#2224:i64
        let s_1_10: i64 = fn_state.esizeshadow_2224;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: div s_1_9 s_1_11
        let s_1_12: i128 = ((s_1_9) / (s_1_11));
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var elements <= s_1_13
        fn_state.elements = s_1_13;
        // D s_1_15: read-var PL:i64
        let s_1_15: i64 = fn_state.PL;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: read-var g:i64
        let s_1_18: i64 = fn_state.g;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: cast zx s_1_17 -> i
        let s_1_20: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_21: call P_read(s_1_19, s_1_20)
        let s_1_21: Bits = P_read(state, tracer, s_1_19, s_1_20);
        // D s_1_22: write-var mask <= s_1_21
        fn_state.mask = s_1_21;
        // D s_1_23: read-var esizeshadow#2224:i64
        let s_1_23: i64 = fn_state.esizeshadow_2224;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: read-var mask:bv
        let s_1_25: Bits = fn_state.mask;
        // D s_1_26: call AnyActiveElement(s_1_25, s_1_24)
        let s_1_26: bool = AnyActiveElement(state, tracer, s_1_25, s_1_24);
        // N s_1_27: branch s_1_26 b80 b2
        if s_1_26 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2226:i64
        let s_2_0: i64 = fn_state.VLshadow_2226;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand1 <= s_2_2
        fn_state.operand1 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#2224:i64
        let s_3_0: i64 = fn_state.esizeshadow_2224;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var mask:bv
        let s_3_2: Bits = fn_state.mask;
        // D s_3_3: call AnyActiveElement(s_3_2, s_3_1)
        let s_3_3: bool = AnyActiveElement(state, tracer, s_3_2, s_3_1);
        // N s_3_4: branch s_3_3 b79 b4
        if s_3_3 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2226:i64
        let s_4_0: i64 = fn_state.VLshadow_2226;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call Zeros(s_4_1)
        let s_4_2: Bits = Zeros(state, tracer, s_4_1);
        // D s_4_3: write-var operand2 <= s_4_2
        fn_state.operand2 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #8s : i
        let s_5_0: i128 = 8;
        // D s_5_1: read-var esizeshadow#2224:i64
        let s_5_1: i64 = fn_state.esizeshadow_2224;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: div s_5_2 s_5_0
        let s_5_3: i128 = ((s_5_2) / (s_5_0));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: write-var psize <= s_5_4
        fn_state.psize = s_5_4;
        // C s_5_6: const #0s : i64
        let s_5_6: i64 = 0;
        // C s_5_7: const #1s : i
        let s_5_7: i128 = 1;
        // D s_5_8: read-var elements:i64
        let s_5_8: i64 = fn_state.elements;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: sub s_5_9 s_5_7
        let s_5_10: i128 = ((s_5_9) - (s_5_7));
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // D s_5_12: write-var gs#179337 <= s_5_11
        fn_state.gs_179337 = s_5_11;
        // D s_5_13: write-var e <= s_5_6
        fn_state.e = s_5_6;
        // N s_5_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#179337:i64
        let s_6_1: i64 = fn_state.gs_179337;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b78 b7
        if s_6_2 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var esizeshadow#2224:i64
        let s_7_2: i64 = fn_state.esizeshadow_2224;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var mask:bv
        let s_7_4: Bits = fn_state.mask;
        // D s_7_5: call ActivePredicateElement(s_7_4, s_7_1, s_7_3)
        let s_7_5: bool = ActivePredicateElement(state, tracer, s_7_4, s_7_1, s_7_3);
        // N s_7_6: branch s_7_5 b10 b8
        if s_7_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var psize:i64
        let s_8_0: i64 = fn_state.psize;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // C s_8_3: const #0u : u8
        let s_8_3: bool = false;
        // C s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 1u16);
        // D s_8_5: read-var psize:i64
        let s_8_5: i64 = fn_state.psize;
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: bits-cast zx s_8_4 -> bv length s_8_6
        let s_8_7: Bits = s_8_4.zero_extend(s_8_6);
        // D s_8_8: read-var e:i64
        let s_8_8: i64 = fn_state.e;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: cast zx s_8_2 -> i
        let s_8_10: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_11: read-var result:bv
        let s_8_11: Bits = fn_state.result;
        // D s_8_12: call Elem_set(s_8_11, s_8_9, s_8_10, s_8_7)
        let s_8_12: Bits = Elem_set(state, tracer, s_8_11, s_8_9, s_8_10, s_8_7);
        // D s_8_13: write-var result <= s_8_12
        fn_state.result = s_8_12;
        // N s_8_14: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var e <= s_9_2
        fn_state.e = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esizeshadow#2224:i64
        let s_10_0: i64 = fn_state.esizeshadow_2224;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var e:i64
        let s_10_3: i64 = fn_state.e;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: read-var operand1:bv
        let s_10_6: Bits = fn_state.operand1;
        // D s_10_7: call Elem_read(s_10_6, s_10_4, s_10_5)
        let s_10_7: Bits = Elem_read(state, tracer, s_10_6, s_10_4, s_10_5);
        // D s_10_8: write-var element1 <= s_10_7
        fn_state.element1 = s_10_7;
        // D s_10_9: read-var esizeshadow#2224:i64
        let s_10_9: i64 = fn_state.esizeshadow_2224;
        // D s_10_10: cast zx s_10_9 -> i
        let s_10_10: i128 = (i128::try_from(s_10_9).unwrap());
        // D s_10_11: cast reint s_10_10 -> i64
        let s_10_11: i64 = (s_10_10 as i64);
        // D s_10_12: read-var e:i64
        let s_10_12: i64 = fn_state.e;
        // D s_10_13: cast zx s_10_12 -> i
        let s_10_13: i128 = (i128::try_from(s_10_12).unwrap());
        // D s_10_14: cast zx s_10_11 -> i
        let s_10_14: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_15: read-var operand2:bv
        let s_10_15: Bits = fn_state.operand2;
        // D s_10_16: call Elem_read(s_10_15, s_10_13, s_10_14)
        let s_10_16: Bits = Elem_read(state, tracer, s_10_15, s_10_13, s_10_14);
        // D s_10_17: write-var element2 <= s_10_16
        fn_state.element2 = s_10_16;
        // C s_10_18: const #0u : u32
        let s_10_18: u32 = 0;
        // D s_10_19: read-var op:u32
        let s_10_19: u32 = fn_state.op;
        // D s_10_20: cmp-eq s_10_18 s_10_19
        let s_10_20: bool = ((s_10_18) == (s_10_19));
        // D s_10_21: not s_10_20
        let s_10_21: bool = !s_10_20;
        // N s_10_22: branch s_10_21 b23 b11
        if s_10_21 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#2224:i64
        let s_11_0: i64 = fn_state.esizeshadow_2224;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call __id(s_11_1)
        let s_11_2: i128 = u__id(state, tracer, s_11_1);
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #16s : i
        let s_11_4: i128 = 16;
        // D s_11_5: cast zx s_11_3 -> i
        let s_11_5: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_6: cmp-eq s_11_5 s_11_4
        let s_11_6: bool = ((s_11_5) == (s_11_4));
        // N s_11_7: branch s_11_6 b22 b12
        if s_11_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esizeshadow#2224:i64
        let s_12_0: i64 = fn_state.esizeshadow_2224;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: call __id(s_12_1)
        let s_12_2: i128 = u__id(state, tracer, s_12_1);
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // C s_12_4: const #32s : i
        let s_12_4: i128 = 32;
        // D s_12_5: cast zx s_12_3 -> i
        let s_12_5: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_6: cmp-eq s_12_5 s_12_4
        let s_12_6: bool = ((s_12_5) == (s_12_4));
        // D s_12_7: write-var gs#179344 <= s_12_6
        fn_state.gs_179344 = s_12_6;
        // N s_12_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#179344:u8
        let s_13_0: bool = fn_state.gs_179344;
        // N s_13_1: branch s_13_0 b21 b14
        if s_13_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var esizeshadow#2224:i64
        let s_14_0: i64 = fn_state.esizeshadow_2224;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call __id(s_14_1)
        let s_14_2: i128 = u__id(state, tracer, s_14_1);
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: const #64s : i
        let s_14_4: i128 = 64;
        // D s_14_5: cast zx s_14_3 -> i
        let s_14_5: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_6: cmp-eq s_14_5 s_14_4
        let s_14_6: bool = ((s_14_5) == (s_14_4));
        // D s_14_7: write-var gs#179346 <= s_14_6
        fn_state.gs_179346 = s_14_6;
        // N s_14_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#179346:u8
        let s_15_0: bool = fn_state.gs_179346;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // C s_15_2: const #() : ()
        let s_15_2: () = ();
        // S s_15_3: call FPCR_read(s_15_2)
        let s_15_3: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_15_2);
        // D s_15_4: read-var element1:bv
        let s_15_4: Bits = fn_state.element1;
        // D s_15_5: read-var element2:bv
        let s_15_5: Bits = fn_state.element2;
        // D s_15_6: call FPCompareEQ(s_15_4, s_15_5, s_15_3)
        let s_15_6: bool = FPCompareEQ(state, tracer, s_15_4, s_15_5, s_15_3);
        // D s_15_7: write-var res <= s_15_6
        fn_state.res = s_15_6;
        // N s_15_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var res:u8
        let s_17_0: bool = fn_state.res;
        // N s_17_1: branch s_17_0 b20 b18
        if s_17_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var pbit <= s_18_0
        fn_state.pbit = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var psize:i64
        let s_19_0: i64 = fn_state.psize;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: cast reint s_19_1 -> i64
        let s_19_2: i64 = (s_19_1 as i64);
        // D s_19_3: read-var pbit:u8
        let s_19_3: bool = fn_state.pbit;
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 1u16);
        // D s_19_5: read-var psize:i64
        let s_19_5: i64 = fn_state.psize;
        // D s_19_6: cast zx s_19_5 -> i
        let s_19_6: i128 = (i128::try_from(s_19_5).unwrap());
        // D s_19_7: bits-cast zx s_19_4 -> bv length s_19_6
        let s_19_7: Bits = s_19_4.zero_extend(s_19_6);
        // D s_19_8: read-var e:i64
        let s_19_8: i64 = fn_state.e;
        // D s_19_9: cast zx s_19_8 -> i
        let s_19_9: i128 = (i128::try_from(s_19_8).unwrap());
        // D s_19_10: cast zx s_19_2 -> i
        let s_19_10: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_11: read-var result:bv
        let s_19_11: Bits = fn_state.result;
        // D s_19_12: call Elem_set(s_19_11, s_19_9, s_19_10, s_19_7)
        let s_19_12: Bits = Elem_set(state, tracer, s_19_11, s_19_9, s_19_10, s_19_7);
        // D s_19_13: write-var result <= s_19_12
        fn_state.result = s_19_12;
        // N s_19_14: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var pbit <= s_20_0
        fn_state.pbit = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#179346 <= s_21_0
        fn_state.gs_179346 = s_21_0;
        // N s_21_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#179344 <= s_22_0
        fn_state.gs_179344 = s_22_0;
        // N s_22_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #2u : u32
        let s_23_0: u32 = 2;
        // D s_23_1: read-var op:u32
        let s_23_1: u32 = fn_state.op;
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // D s_23_3: not s_23_2
        let s_23_3: bool = !s_23_2;
        // N s_23_4: branch s_23_3 b32 b24
        if s_23_3 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var esizeshadow#2224:i64
        let s_24_0: i64 = fn_state.esizeshadow_2224;
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
        // N s_24_7: branch s_24_6 b31 b25
        if s_24_6 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var esizeshadow#2224:i64
        let s_25_0: i64 = fn_state.esizeshadow_2224;
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
        // D s_25_7: write-var gs#179351 <= s_25_6
        fn_state.gs_179351 = s_25_6;
        // N s_25_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#179351:u8
        let s_26_0: bool = fn_state.gs_179351;
        // N s_26_1: branch s_26_0 b30 b27
        if s_26_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var esizeshadow#2224:i64
        let s_27_0: i64 = fn_state.esizeshadow_2224;
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
        // D s_27_7: write-var gs#179353 <= s_27_6
        fn_state.gs_179353 = s_27_6;
        // N s_27_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#179353:u8
        let s_28_0: bool = fn_state.gs_179353;
        // N s_28_1: assert s_28_0
        let s_28_1: () = assert!(s_28_0);
        // C s_28_2: const #() : ()
        let s_28_2: () = ();
        // S s_28_3: call FPCR_read(s_28_2)
        let s_28_3: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_28_2);
        // D s_28_4: read-var element1:bv
        let s_28_4: Bits = fn_state.element1;
        // D s_28_5: read-var element2:bv
        let s_28_5: Bits = fn_state.element2;
        // D s_28_6: call FPCompareGE(s_28_4, s_28_5, s_28_3)
        let s_28_6: bool = FPCompareGE(state, tracer, s_28_4, s_28_5, s_28_3);
        // D s_28_7: write-var res <= s_28_6
        fn_state.res = s_28_6;
        // N s_28_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#179353 <= s_30_0
        fn_state.gs_179353 = s_30_0;
        // N s_30_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#179351 <= s_31_0
        fn_state.gs_179351 = s_31_0;
        // N s_31_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #3u : u32
        let s_32_0: u32 = 3;
        // D s_32_1: read-var op:u32
        let s_32_1: u32 = fn_state.op;
        // D s_32_2: cmp-eq s_32_0 s_32_1
        let s_32_2: bool = ((s_32_0) == (s_32_1));
        // D s_32_3: not s_32_2
        let s_32_3: bool = !s_32_2;
        // N s_32_4: branch s_32_3 b41 b33
        if s_32_3 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var esizeshadow#2224:i64
        let s_33_0: i64 = fn_state.esizeshadow_2224;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call __id(s_33_1)
        let s_33_2: i128 = u__id(state, tracer, s_33_1);
        // D s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: const #16s : i
        let s_33_4: i128 = 16;
        // D s_33_5: cast zx s_33_3 -> i
        let s_33_5: i128 = (i128::try_from(s_33_3).unwrap());
        // D s_33_6: cmp-eq s_33_5 s_33_4
        let s_33_6: bool = ((s_33_5) == (s_33_4));
        // N s_33_7: branch s_33_6 b40 b34
        if s_33_6 {
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
        // D s_34_0: read-var esizeshadow#2224:i64
        let s_34_0: i64 = fn_state.esizeshadow_2224;
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
        // D s_34_7: write-var gs#179358 <= s_34_6
        fn_state.gs_179358 = s_34_6;
        // N s_34_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#179358:u8
        let s_35_0: bool = fn_state.gs_179358;
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
        // D s_36_0: read-var esizeshadow#2224:i64
        let s_36_0: i64 = fn_state.esizeshadow_2224;
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
        // D s_36_7: write-var gs#179360 <= s_36_6
        fn_state.gs_179360 = s_36_6;
        // N s_36_8: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#179360:u8
        let s_37_0: bool = fn_state.gs_179360;
        // N s_37_1: assert s_37_0
        let s_37_1: () = assert!(s_37_0);
        // C s_37_2: const #() : ()
        let s_37_2: () = ();
        // S s_37_3: call FPCR_read(s_37_2)
        let s_37_3: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_37_2);
        // D s_37_4: read-var element1:bv
        let s_37_4: Bits = fn_state.element1;
        // D s_37_5: read-var element2:bv
        let s_37_5: Bits = fn_state.element2;
        // D s_37_6: call FPCompareGT(s_37_4, s_37_5, s_37_3)
        let s_37_6: bool = FPCompareGT(state, tracer, s_37_4, s_37_5, s_37_3);
        // D s_37_7: write-var res <= s_37_6
        fn_state.res = s_37_6;
        // N s_37_8: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#179360 <= s_39_0
        fn_state.gs_179360 = s_39_0;
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
        // D s_40_1: write-var gs#179358 <= s_40_0
        fn_state.gs_179358 = s_40_0;
        // N s_40_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #6u : u32
        let s_41_0: u32 = 6;
        // D s_41_1: read-var op:u32
        let s_41_1: u32 = fn_state.op;
        // D s_41_2: cmp-eq s_41_0 s_41_1
        let s_41_2: bool = ((s_41_0) == (s_41_1));
        // D s_41_3: not s_41_2
        let s_41_3: bool = !s_41_2;
        // N s_41_4: branch s_41_3 b50 b42
        if s_41_3 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var esizeshadow#2224:i64
        let s_42_0: i64 = fn_state.esizeshadow_2224;
        // D s_42_1: cast zx s_42_0 -> i
        let s_42_1: i128 = (i128::try_from(s_42_0).unwrap());
        // D s_42_2: call __id(s_42_1)
        let s_42_2: i128 = u__id(state, tracer, s_42_1);
        // D s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // C s_42_4: const #16s : i
        let s_42_4: i128 = 16;
        // D s_42_5: cast zx s_42_3 -> i
        let s_42_5: i128 = (i128::try_from(s_42_3).unwrap());
        // D s_42_6: cmp-eq s_42_5 s_42_4
        let s_42_6: bool = ((s_42_5) == (s_42_4));
        // N s_42_7: branch s_42_6 b49 b43
        if s_42_6 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var esizeshadow#2224:i64
        let s_43_0: i64 = fn_state.esizeshadow_2224;
        // D s_43_1: cast zx s_43_0 -> i
        let s_43_1: i128 = (i128::try_from(s_43_0).unwrap());
        // D s_43_2: call __id(s_43_1)
        let s_43_2: i128 = u__id(state, tracer, s_43_1);
        // D s_43_3: cast reint s_43_2 -> i64
        let s_43_3: i64 = (s_43_2 as i64);
        // C s_43_4: const #32s : i
        let s_43_4: i128 = 32;
        // D s_43_5: cast zx s_43_3 -> i
        let s_43_5: i128 = (i128::try_from(s_43_3).unwrap());
        // D s_43_6: cmp-eq s_43_5 s_43_4
        let s_43_6: bool = ((s_43_5) == (s_43_4));
        // D s_43_7: write-var gs#179365 <= s_43_6
        fn_state.gs_179365 = s_43_6;
        // N s_43_8: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#179365:u8
        let s_44_0: bool = fn_state.gs_179365;
        // N s_44_1: branch s_44_0 b48 b45
        if s_44_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var esizeshadow#2224:i64
        let s_45_0: i64 = fn_state.esizeshadow_2224;
        // D s_45_1: cast zx s_45_0 -> i
        let s_45_1: i128 = (i128::try_from(s_45_0).unwrap());
        // D s_45_2: call __id(s_45_1)
        let s_45_2: i128 = u__id(state, tracer, s_45_1);
        // D s_45_3: cast reint s_45_2 -> i64
        let s_45_3: i64 = (s_45_2 as i64);
        // C s_45_4: const #64s : i
        let s_45_4: i128 = 64;
        // D s_45_5: cast zx s_45_3 -> i
        let s_45_5: i128 = (i128::try_from(s_45_3).unwrap());
        // D s_45_6: cmp-eq s_45_5 s_45_4
        let s_45_6: bool = ((s_45_5) == (s_45_4));
        // D s_45_7: write-var gs#179367 <= s_45_6
        fn_state.gs_179367 = s_45_6;
        // N s_45_8: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#179367:u8
        let s_46_0: bool = fn_state.gs_179367;
        // N s_46_1: assert s_46_0
        let s_46_1: () = assert!(s_46_0);
        // C s_46_2: const #() : ()
        let s_46_2: () = ();
        // S s_46_3: call FPCR_read(s_46_2)
        let s_46_3: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_46_2);
        // D s_46_4: read-var element1:bv
        let s_46_4: Bits = fn_state.element1;
        // D s_46_5: read-var element2:bv
        let s_46_5: Bits = fn_state.element2;
        // D s_46_6: call FPCompareUN(s_46_4, s_46_5, s_46_3)
        let s_46_6: bool = FPCompareUN(state, tracer, s_46_4, s_46_5, s_46_3);
        // D s_46_7: write-var res <= s_46_6
        fn_state.res = s_46_6;
        // N s_46_8: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_47_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#179367 <= s_48_0
        fn_state.gs_179367 = s_48_0;
        // N s_48_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#179365 <= s_49_0
        fn_state.gs_179365 = s_49_0;
        // N s_49_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #1u : u32
        let s_50_0: u32 = 1;
        // D s_50_1: read-var op:u32
        let s_50_1: u32 = fn_state.op;
        // D s_50_2: cmp-eq s_50_0 s_50_1
        let s_50_2: bool = ((s_50_0) == (s_50_1));
        // D s_50_3: not s_50_2
        let s_50_3: bool = !s_50_2;
        // N s_50_4: branch s_50_3 b59 b51
        if s_50_3 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var esizeshadow#2224:i64
        let s_51_0: i64 = fn_state.esizeshadow_2224;
        // D s_51_1: cast zx s_51_0 -> i
        let s_51_1: i128 = (i128::try_from(s_51_0).unwrap());
        // D s_51_2: call __id(s_51_1)
        let s_51_2: i128 = u__id(state, tracer, s_51_1);
        // D s_51_3: cast reint s_51_2 -> i64
        let s_51_3: i64 = (s_51_2 as i64);
        // C s_51_4: const #16s : i
        let s_51_4: i128 = 16;
        // D s_51_5: cast zx s_51_3 -> i
        let s_51_5: i128 = (i128::try_from(s_51_3).unwrap());
        // D s_51_6: cmp-eq s_51_5 s_51_4
        let s_51_6: bool = ((s_51_5) == (s_51_4));
        // N s_51_7: branch s_51_6 b58 b52
        if s_51_6 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var esizeshadow#2224:i64
        let s_52_0: i64 = fn_state.esizeshadow_2224;
        // D s_52_1: cast zx s_52_0 -> i
        let s_52_1: i128 = (i128::try_from(s_52_0).unwrap());
        // D s_52_2: call __id(s_52_1)
        let s_52_2: i128 = u__id(state, tracer, s_52_1);
        // D s_52_3: cast reint s_52_2 -> i64
        let s_52_3: i64 = (s_52_2 as i64);
        // C s_52_4: const #32s : i
        let s_52_4: i128 = 32;
        // D s_52_5: cast zx s_52_3 -> i
        let s_52_5: i128 = (i128::try_from(s_52_3).unwrap());
        // D s_52_6: cmp-eq s_52_5 s_52_4
        let s_52_6: bool = ((s_52_5) == (s_52_4));
        // D s_52_7: write-var gs#179372 <= s_52_6
        fn_state.gs_179372 = s_52_6;
        // N s_52_8: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#179372:u8
        let s_53_0: bool = fn_state.gs_179372;
        // N s_53_1: branch s_53_0 b57 b54
        if s_53_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var esizeshadow#2224:i64
        let s_54_0: i64 = fn_state.esizeshadow_2224;
        // D s_54_1: cast zx s_54_0 -> i
        let s_54_1: i128 = (i128::try_from(s_54_0).unwrap());
        // D s_54_2: call __id(s_54_1)
        let s_54_2: i128 = u__id(state, tracer, s_54_1);
        // D s_54_3: cast reint s_54_2 -> i64
        let s_54_3: i64 = (s_54_2 as i64);
        // C s_54_4: const #64s : i
        let s_54_4: i128 = 64;
        // D s_54_5: cast zx s_54_3 -> i
        let s_54_5: i128 = (i128::try_from(s_54_3).unwrap());
        // D s_54_6: cmp-eq s_54_5 s_54_4
        let s_54_6: bool = ((s_54_5) == (s_54_4));
        // D s_54_7: write-var gs#179374 <= s_54_6
        fn_state.gs_179374 = s_54_6;
        // N s_54_8: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#179374:u8
        let s_55_0: bool = fn_state.gs_179374;
        // N s_55_1: assert s_55_0
        let s_55_1: () = assert!(s_55_0);
        // C s_55_2: const #() : ()
        let s_55_2: () = ();
        // S s_55_3: call FPCR_read(s_55_2)
        let s_55_3: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_55_2);
        // D s_55_4: read-var element1:bv
        let s_55_4: Bits = fn_state.element1;
        // D s_55_5: read-var element2:bv
        let s_55_5: Bits = fn_state.element2;
        // D s_55_6: call FPCompareNE(s_55_4, s_55_5, s_55_3)
        let s_55_6: bool = FPCompareNE(state, tracer, s_55_4, s_55_5, s_55_3);
        // D s_55_7: write-var res <= s_55_6
        fn_state.res = s_55_6;
        // N s_55_8: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_56_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#179374 <= s_57_0
        fn_state.gs_179374 = s_57_0;
        // N s_57_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#179372 <= s_58_0
        fn_state.gs_179372 = s_58_0;
        // N s_58_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #4u : u32
        let s_59_0: u32 = 4;
        // D s_59_1: read-var op:u32
        let s_59_1: u32 = fn_state.op;
        // D s_59_2: cmp-eq s_59_0 s_59_1
        let s_59_2: bool = ((s_59_0) == (s_59_1));
        // D s_59_3: not s_59_2
        let s_59_3: bool = !s_59_2;
        // N s_59_4: branch s_59_3 b68 b60
        if s_59_3 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var esizeshadow#2224:i64
        let s_60_0: i64 = fn_state.esizeshadow_2224;
        // D s_60_1: cast zx s_60_0 -> i
        let s_60_1: i128 = (i128::try_from(s_60_0).unwrap());
        // D s_60_2: call __id(s_60_1)
        let s_60_2: i128 = u__id(state, tracer, s_60_1);
        // D s_60_3: cast reint s_60_2 -> i64
        let s_60_3: i64 = (s_60_2 as i64);
        // C s_60_4: const #16s : i
        let s_60_4: i128 = 16;
        // D s_60_5: cast zx s_60_3 -> i
        let s_60_5: i128 = (i128::try_from(s_60_3).unwrap());
        // D s_60_6: cmp-eq s_60_5 s_60_4
        let s_60_6: bool = ((s_60_5) == (s_60_4));
        // N s_60_7: branch s_60_6 b67 b61
        if s_60_6 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var esizeshadow#2224:i64
        let s_61_0: i64 = fn_state.esizeshadow_2224;
        // D s_61_1: cast zx s_61_0 -> i
        let s_61_1: i128 = (i128::try_from(s_61_0).unwrap());
        // D s_61_2: call __id(s_61_1)
        let s_61_2: i128 = u__id(state, tracer, s_61_1);
        // D s_61_3: cast reint s_61_2 -> i64
        let s_61_3: i64 = (s_61_2 as i64);
        // C s_61_4: const #32s : i
        let s_61_4: i128 = 32;
        // D s_61_5: cast zx s_61_3 -> i
        let s_61_5: i128 = (i128::try_from(s_61_3).unwrap());
        // D s_61_6: cmp-eq s_61_5 s_61_4
        let s_61_6: bool = ((s_61_5) == (s_61_4));
        // D s_61_7: write-var gs#179379 <= s_61_6
        fn_state.gs_179379 = s_61_6;
        // N s_61_8: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#179379:u8
        let s_62_0: bool = fn_state.gs_179379;
        // N s_62_1: branch s_62_0 b66 b63
        if s_62_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var esizeshadow#2224:i64
        let s_63_0: i64 = fn_state.esizeshadow_2224;
        // D s_63_1: cast zx s_63_0 -> i
        let s_63_1: i128 = (i128::try_from(s_63_0).unwrap());
        // D s_63_2: call __id(s_63_1)
        let s_63_2: i128 = u__id(state, tracer, s_63_1);
        // D s_63_3: cast reint s_63_2 -> i64
        let s_63_3: i64 = (s_63_2 as i64);
        // C s_63_4: const #64s : i
        let s_63_4: i128 = 64;
        // D s_63_5: cast zx s_63_3 -> i
        let s_63_5: i128 = (i128::try_from(s_63_3).unwrap());
        // D s_63_6: cmp-eq s_63_5 s_63_4
        let s_63_6: bool = ((s_63_5) == (s_63_4));
        // D s_63_7: write-var gs#179381 <= s_63_6
        fn_state.gs_179381 = s_63_6;
        // N s_63_8: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#179381:u8
        let s_64_0: bool = fn_state.gs_179381;
        // N s_64_1: assert s_64_0
        let s_64_1: () = assert!(s_64_0);
        // C s_64_2: const #() : ()
        let s_64_2: () = ();
        // S s_64_3: call FPCR_read(s_64_2)
        let s_64_3: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_64_2);
        // D s_64_4: read-var element2:bv
        let s_64_4: Bits = fn_state.element2;
        // D s_64_5: read-var element1:bv
        let s_64_5: Bits = fn_state.element1;
        // D s_64_6: call FPCompareGT(s_64_4, s_64_5, s_64_3)
        let s_64_6: bool = FPCompareGT(state, tracer, s_64_4, s_64_5, s_64_3);
        // D s_64_7: write-var res <= s_64_6
        fn_state.res = s_64_6;
        // N s_64_8: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_65_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: write-var gs#179381 <= s_66_0
        fn_state.gs_179381 = s_66_0;
        // N s_66_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#179379 <= s_67_0
        fn_state.gs_179379 = s_67_0;
        // N s_67_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #5u : u32
        let s_68_0: u32 = 5;
        // D s_68_1: read-var op:u32
        let s_68_1: u32 = fn_state.op;
        // D s_68_2: cmp-eq s_68_0 s_68_1
        let s_68_2: bool = ((s_68_0) == (s_68_1));
        // D s_68_3: not s_68_2
        let s_68_3: bool = !s_68_2;
        // N s_68_4: branch s_68_3 b77 b69
        if s_68_3 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var esizeshadow#2224:i64
        let s_69_0: i64 = fn_state.esizeshadow_2224;
        // D s_69_1: cast zx s_69_0 -> i
        let s_69_1: i128 = (i128::try_from(s_69_0).unwrap());
        // D s_69_2: call __id(s_69_1)
        let s_69_2: i128 = u__id(state, tracer, s_69_1);
        // D s_69_3: cast reint s_69_2 -> i64
        let s_69_3: i64 = (s_69_2 as i64);
        // C s_69_4: const #16s : i
        let s_69_4: i128 = 16;
        // D s_69_5: cast zx s_69_3 -> i
        let s_69_5: i128 = (i128::try_from(s_69_3).unwrap());
        // D s_69_6: cmp-eq s_69_5 s_69_4
        let s_69_6: bool = ((s_69_5) == (s_69_4));
        // N s_69_7: branch s_69_6 b76 b70
        if s_69_6 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var esizeshadow#2224:i64
        let s_70_0: i64 = fn_state.esizeshadow_2224;
        // D s_70_1: cast zx s_70_0 -> i
        let s_70_1: i128 = (i128::try_from(s_70_0).unwrap());
        // D s_70_2: call __id(s_70_1)
        let s_70_2: i128 = u__id(state, tracer, s_70_1);
        // D s_70_3: cast reint s_70_2 -> i64
        let s_70_3: i64 = (s_70_2 as i64);
        // C s_70_4: const #32s : i
        let s_70_4: i128 = 32;
        // D s_70_5: cast zx s_70_3 -> i
        let s_70_5: i128 = (i128::try_from(s_70_3).unwrap());
        // D s_70_6: cmp-eq s_70_5 s_70_4
        let s_70_6: bool = ((s_70_5) == (s_70_4));
        // D s_70_7: write-var gs#179386 <= s_70_6
        fn_state.gs_179386 = s_70_6;
        // N s_70_8: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#179386:u8
        let s_71_0: bool = fn_state.gs_179386;
        // N s_71_1: branch s_71_0 b75 b72
        if s_71_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var esizeshadow#2224:i64
        let s_72_0: i64 = fn_state.esizeshadow_2224;
        // D s_72_1: cast zx s_72_0 -> i
        let s_72_1: i128 = (i128::try_from(s_72_0).unwrap());
        // D s_72_2: call __id(s_72_1)
        let s_72_2: i128 = u__id(state, tracer, s_72_1);
        // D s_72_3: cast reint s_72_2 -> i64
        let s_72_3: i64 = (s_72_2 as i64);
        // C s_72_4: const #64s : i
        let s_72_4: i128 = 64;
        // D s_72_5: cast zx s_72_3 -> i
        let s_72_5: i128 = (i128::try_from(s_72_3).unwrap());
        // D s_72_6: cmp-eq s_72_5 s_72_4
        let s_72_6: bool = ((s_72_5) == (s_72_4));
        // D s_72_7: write-var gs#179388 <= s_72_6
        fn_state.gs_179388 = s_72_6;
        // N s_72_8: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#179388:u8
        let s_73_0: bool = fn_state.gs_179388;
        // N s_73_1: assert s_73_0
        let s_73_1: () = assert!(s_73_0);
        // C s_73_2: const #() : ()
        let s_73_2: () = ();
        // S s_73_3: call FPCR_read(s_73_2)
        let s_73_3: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_73_2);
        // D s_73_4: read-var element2:bv
        let s_73_4: Bits = fn_state.element2;
        // D s_73_5: read-var element1:bv
        let s_73_5: Bits = fn_state.element1;
        // D s_73_6: call FPCompareGE(s_73_4, s_73_5, s_73_3)
        let s_73_6: bool = FPCompareGE(state, tracer, s_73_4, s_73_5, s_73_3);
        // D s_73_7: write-var res <= s_73_6
        fn_state.res = s_73_6;
        // N s_73_8: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_74_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#179388 <= s_75_0
        fn_state.gs_179388 = s_75_0;
        // N s_75_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #1u : u8
        let s_76_0: bool = true;
        // D s_76_1: write-var gs#179386 <= s_76_0
        fn_state.gs_179386 = s_76_0;
        // N s_76_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_77_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var PL:i64
        let s_78_0: i64 = fn_state.PL;
        // D s_78_1: cast zx s_78_0 -> i
        let s_78_1: i128 = (i128::try_from(s_78_0).unwrap());
        // D s_78_2: cast reint s_78_1 -> i64
        let s_78_2: i64 = (s_78_1 as i64);
        // D s_78_3: read-var d:i64
        let s_78_3: i64 = fn_state.d;
        // D s_78_4: cast zx s_78_3 -> i
        let s_78_4: i128 = (i128::try_from(s_78_3).unwrap());
        // D s_78_5: cast zx s_78_2 -> i
        let s_78_5: i128 = (i128::try_from(s_78_2).unwrap());
        // D s_78_6: read-var result:bv
        let s_78_6: Bits = fn_state.result;
        // D s_78_7: call P_set(s_78_4, s_78_5, s_78_6)
        let s_78_7: () = P_set(state, tracer, s_78_4, s_78_5, s_78_6);
        // N s_78_8: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var VLshadow#2226:i64
        let s_79_0: i64 = fn_state.VLshadow_2226;
        // D s_79_1: cast zx s_79_0 -> i
        let s_79_1: i128 = (i128::try_from(s_79_0).unwrap());
        // D s_79_2: cast reint s_79_1 -> i64
        let s_79_2: i64 = (s_79_1 as i64);
        // D s_79_3: read-var m:i64
        let s_79_3: i64 = fn_state.m;
        // D s_79_4: cast zx s_79_3 -> i
        let s_79_4: i128 = (i128::try_from(s_79_3).unwrap());
        // D s_79_5: cast zx s_79_2 -> i
        let s_79_5: i128 = (i128::try_from(s_79_2).unwrap());
        // D s_79_6: call Z_read(s_79_4, s_79_5)
        let s_79_6: Bits = Z_read(state, tracer, s_79_4, s_79_5);
        // D s_79_7: write-var operand2 <= s_79_6
        fn_state.operand2 = s_79_6;
        // N s_79_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var VLshadow#2226:i64
        let s_80_0: i64 = fn_state.VLshadow_2226;
        // D s_80_1: cast zx s_80_0 -> i
        let s_80_1: i128 = (i128::try_from(s_80_0).unwrap());
        // D s_80_2: cast reint s_80_1 -> i64
        let s_80_2: i64 = (s_80_1 as i64);
        // D s_80_3: read-var n:i64
        let s_80_3: i64 = fn_state.n;
        // D s_80_4: cast zx s_80_3 -> i
        let s_80_4: i128 = (i128::try_from(s_80_3).unwrap());
        // D s_80_5: cast zx s_80_2 -> i
        let s_80_5: i128 = (i128::try_from(s_80_2).unwrap());
        // D s_80_6: call Z_read(s_80_4, s_80_5)
        let s_80_6: Bits = Z_read(state, tracer, s_80_4, s_80_5);
        // D s_80_7: write-var operand1 <= s_80_6
        fn_state.operand1 = s_80_6;
        // N s_80_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
