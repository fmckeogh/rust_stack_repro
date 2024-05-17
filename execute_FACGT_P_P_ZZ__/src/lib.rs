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
use CheckSVEEnabled::*;
use FPCR_read::*;
use Elem_read::*;
use Zeros::*;
use P_set::*;
use FPCompareGT::*;
use Z_read::*;
use Elem_set::*;
use u__id::*;
use FPAbs::*;
use AnyActiveElement::*;
use P_read::*;
use ActivePredicateElement::*;
use FPCompareGE::*;
use common::*;
pub fn execute_FACGT_P_P_ZZ__<T: Tracer>(
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
        e: i64,
        esizeshadow_2221: i64,
        gs_179277: bool,
        gs_179268: i64,
        VLshadow_2223: i64,
        VLshadow_2222: i64,
        ga_273644: Bits,
        gs_179284: bool,
        ga_273643: Bits,
        gs_179275: bool,
        res: bool,
        PL: i64,
        gs_179282: bool,
        mask: Bits,
        element2: Bits,
        element1: Bits,
        pbit: bool,
        ga_273635: Bits,
        ga_273634: Bits,
        psize: i64,
        elements: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
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
        // D s_0_3: write-var esizeshadow#2221 <= s_0_2
        fn_state.esizeshadow_2221 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2222 <= s_0_6
        fn_state.VLshadow_2222 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2222:i64
        let s_1_0: i64 = fn_state.VLshadow_2222;
        // D s_1_1: write-var VLshadow#2223 <= s_1_0
        fn_state.VLshadow_2223 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2223:i64
        let s_1_3: i64 = fn_state.VLshadow_2223;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: write-var PL <= s_1_6
        fn_state.PL = s_1_6;
        // D s_1_8: read-var VLshadow#2223:i64
        let s_1_8: i64 = fn_state.VLshadow_2223;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: read-var esizeshadow#2221:i64
        let s_1_10: i64 = fn_state.esizeshadow_2221;
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
        // D s_1_23: read-var esizeshadow#2221:i64
        let s_1_23: i64 = fn_state.esizeshadow_2221;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: read-var mask:bv
        let s_1_25: Bits = fn_state.mask;
        // D s_1_26: call AnyActiveElement(s_1_25, s_1_24)
        let s_1_26: bool = AnyActiveElement(state, tracer, s_1_25, s_1_24);
        // N s_1_27: branch s_1_26 b39 b2
        if s_1_26 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2223:i64
        let s_2_0: i64 = fn_state.VLshadow_2223;
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
        // D s_3_0: read-var esizeshadow#2221:i64
        let s_3_0: i64 = fn_state.esizeshadow_2221;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var mask:bv
        let s_3_2: Bits = fn_state.mask;
        // D s_3_3: call AnyActiveElement(s_3_2, s_3_1)
        let s_3_3: bool = AnyActiveElement(state, tracer, s_3_2, s_3_1);
        // N s_3_4: branch s_3_3 b38 b4
        if s_3_3 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2223:i64
        let s_4_0: i64 = fn_state.VLshadow_2223;
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
        // D s_5_1: read-var esizeshadow#2221:i64
        let s_5_1: i64 = fn_state.esizeshadow_2221;
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
        // D s_5_12: write-var gs#179268 <= s_5_11
        fn_state.gs_179268 = s_5_11;
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
        // D s_6_1: read-var gs#179268:i64
        let s_6_1: i64 = fn_state.gs_179268;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b37 b7
        if s_6_2 {
            return block_37(state, tracer, fn_state);
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
        // D s_7_2: read-var esizeshadow#2221:i64
        let s_7_2: i64 = fn_state.esizeshadow_2221;
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
        // D s_10_0: read-var esizeshadow#2221:i64
        let s_10_0: i64 = fn_state.esizeshadow_2221;
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
        // D s_10_9: read-var esizeshadow#2221:i64
        let s_10_9: i64 = fn_state.esizeshadow_2221;
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
        // C s_10_18: const #2u : u32
        let s_10_18: u32 = 2;
        // D s_10_19: read-var op:u32
        let s_10_19: u32 = fn_state.op;
        // D s_10_20: cmp-eq s_10_18 s_10_19
        let s_10_20: bool = ((s_10_18) == (s_10_19));
        // D s_10_21: not s_10_20
        let s_10_21: bool = !s_10_20;
        // N s_10_22: branch s_10_21 b25 b11
        if s_10_21 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#2221:i64
        let s_11_0: i64 = fn_state.esizeshadow_2221;
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
        // N s_11_7: branch s_11_6 b24 b12
        if s_11_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esizeshadow#2221:i64
        let s_12_0: i64 = fn_state.esizeshadow_2221;
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
        // D s_12_7: write-var gs#179275 <= s_12_6
        fn_state.gs_179275 = s_12_6;
        // N s_12_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#179275:u8
        let s_13_0: bool = fn_state.gs_179275;
        // N s_13_1: branch s_13_0 b23 b14
        if s_13_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var esizeshadow#2221:i64
        let s_14_0: i64 = fn_state.esizeshadow_2221;
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
        // D s_14_7: write-var gs#179277 <= s_14_6
        fn_state.gs_179277 = s_14_6;
        // N s_14_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#179277:u8
        let s_15_0: bool = fn_state.gs_179277;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // D s_15_2: read-var element1:bv
        let s_15_2: Bits = fn_state.element1;
        // D s_15_3: call FPAbs(s_15_2)
        let s_15_3: Bits = FPAbs(state, tracer, s_15_2);
        // D s_15_4: write-var ga#273634 <= s_15_3
        fn_state.ga_273634 = s_15_3;
        // N s_15_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var element2:bv
        let s_16_0: Bits = fn_state.element2;
        // D s_16_1: call FPAbs(s_16_0)
        let s_16_1: Bits = FPAbs(state, tracer, s_16_0);
        // D s_16_2: write-var ga#273635 <= s_16_1
        fn_state.ga_273635 = s_16_1;
        // N s_16_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call FPCR_read(s_17_0)
        let s_17_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_17_0);
        // D s_17_2: read-var ga#273634:bv
        let s_17_2: Bits = fn_state.ga_273634;
        // D s_17_3: read-var ga#273635:bv
        let s_17_3: Bits = fn_state.ga_273635;
        // D s_17_4: call FPCompareGE(s_17_2, s_17_3, s_17_1)
        let s_17_4: bool = FPCompareGE(state, tracer, s_17_2, s_17_3, s_17_1);
        // D s_17_5: write-var res <= s_17_4
        fn_state.res = s_17_4;
        // N s_17_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var res:u8
        let s_19_0: bool = fn_state.res;
        // N s_19_1: branch s_19_0 b22 b20
        if s_19_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var pbit <= s_20_0
        fn_state.pbit = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var psize:i64
        let s_21_0: i64 = fn_state.psize;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: cast reint s_21_1 -> i64
        let s_21_2: i64 = (s_21_1 as i64);
        // D s_21_3: read-var pbit:u8
        let s_21_3: bool = fn_state.pbit;
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 1u16);
        // D s_21_5: read-var psize:i64
        let s_21_5: i64 = fn_state.psize;
        // D s_21_6: cast zx s_21_5 -> i
        let s_21_6: i128 = (i128::try_from(s_21_5).unwrap());
        // D s_21_7: bits-cast zx s_21_4 -> bv length s_21_6
        let s_21_7: Bits = s_21_4.zero_extend(s_21_6);
        // D s_21_8: read-var e:i64
        let s_21_8: i64 = fn_state.e;
        // D s_21_9: cast zx s_21_8 -> i
        let s_21_9: i128 = (i128::try_from(s_21_8).unwrap());
        // D s_21_10: cast zx s_21_2 -> i
        let s_21_10: i128 = (i128::try_from(s_21_2).unwrap());
        // D s_21_11: read-var result:bv
        let s_21_11: Bits = fn_state.result;
        // D s_21_12: call Elem_set(s_21_11, s_21_9, s_21_10, s_21_7)
        let s_21_12: Bits = Elem_set(state, tracer, s_21_11, s_21_9, s_21_10, s_21_7);
        // D s_21_13: write-var result <= s_21_12
        fn_state.result = s_21_12;
        // N s_21_14: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var pbit <= s_22_0
        fn_state.pbit = s_22_0;
        // N s_22_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#179277 <= s_23_0
        fn_state.gs_179277 = s_23_0;
        // N s_23_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#179275 <= s_24_0
        fn_state.gs_179275 = s_24_0;
        // N s_24_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #3u : u32
        let s_25_0: u32 = 3;
        // D s_25_1: read-var op:u32
        let s_25_1: u32 = fn_state.op;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // D s_25_3: not s_25_2
        let s_25_3: bool = !s_25_2;
        // N s_25_4: branch s_25_3 b36 b26
        if s_25_3 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var esizeshadow#2221:i64
        let s_26_0: i64 = fn_state.esizeshadow_2221;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: call __id(s_26_1)
        let s_26_2: i128 = u__id(state, tracer, s_26_1);
        // D s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: const #16s : i
        let s_26_4: i128 = 16;
        // D s_26_5: cast zx s_26_3 -> i
        let s_26_5: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_6: cmp-eq s_26_5 s_26_4
        let s_26_6: bool = ((s_26_5) == (s_26_4));
        // N s_26_7: branch s_26_6 b35 b27
        if s_26_6 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var esizeshadow#2221:i64
        let s_27_0: i64 = fn_state.esizeshadow_2221;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: call __id(s_27_1)
        let s_27_2: i128 = u__id(state, tracer, s_27_1);
        // D s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: const #32s : i
        let s_27_4: i128 = 32;
        // D s_27_5: cast zx s_27_3 -> i
        let s_27_5: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_6: cmp-eq s_27_5 s_27_4
        let s_27_6: bool = ((s_27_5) == (s_27_4));
        // D s_27_7: write-var gs#179282 <= s_27_6
        fn_state.gs_179282 = s_27_6;
        // N s_27_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#179282:u8
        let s_28_0: bool = fn_state.gs_179282;
        // N s_28_1: branch s_28_0 b34 b29
        if s_28_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var esizeshadow#2221:i64
        let s_29_0: i64 = fn_state.esizeshadow_2221;
        // D s_29_1: cast zx s_29_0 -> i
        let s_29_1: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_2: call __id(s_29_1)
        let s_29_2: i128 = u__id(state, tracer, s_29_1);
        // D s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // C s_29_4: const #64s : i
        let s_29_4: i128 = 64;
        // D s_29_5: cast zx s_29_3 -> i
        let s_29_5: i128 = (i128::try_from(s_29_3).unwrap());
        // D s_29_6: cmp-eq s_29_5 s_29_4
        let s_29_6: bool = ((s_29_5) == (s_29_4));
        // D s_29_7: write-var gs#179284 <= s_29_6
        fn_state.gs_179284 = s_29_6;
        // N s_29_8: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#179284:u8
        let s_30_0: bool = fn_state.gs_179284;
        // N s_30_1: assert s_30_0
        let s_30_1: () = assert!(s_30_0);
        // D s_30_2: read-var element1:bv
        let s_30_2: Bits = fn_state.element1;
        // D s_30_3: call FPAbs(s_30_2)
        let s_30_3: Bits = FPAbs(state, tracer, s_30_2);
        // D s_30_4: write-var ga#273643 <= s_30_3
        fn_state.ga_273643 = s_30_3;
        // N s_30_5: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var element2:bv
        let s_31_0: Bits = fn_state.element2;
        // D s_31_1: call FPAbs(s_31_0)
        let s_31_1: Bits = FPAbs(state, tracer, s_31_0);
        // D s_31_2: write-var ga#273644 <= s_31_1
        fn_state.ga_273644 = s_31_1;
        // N s_31_3: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call FPCR_read(s_32_0)
        let s_32_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_32_0);
        // D s_32_2: read-var ga#273643:bv
        let s_32_2: Bits = fn_state.ga_273643;
        // D s_32_3: read-var ga#273644:bv
        let s_32_3: Bits = fn_state.ga_273644;
        // D s_32_4: call FPCompareGT(s_32_2, s_32_3, s_32_1)
        let s_32_4: bool = FPCompareGT(state, tracer, s_32_2, s_32_3, s_32_1);
        // D s_32_5: write-var res <= s_32_4
        fn_state.res = s_32_4;
        // N s_32_6: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#179284 <= s_34_0
        fn_state.gs_179284 = s_34_0;
        // N s_34_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#179282 <= s_35_0
        fn_state.gs_179282 = s_35_0;
        // N s_35_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var PL:i64
        let s_37_0: i64 = fn_state.PL;
        // D s_37_1: cast zx s_37_0 -> i
        let s_37_1: i128 = (i128::try_from(s_37_0).unwrap());
        // D s_37_2: cast reint s_37_1 -> i64
        let s_37_2: i64 = (s_37_1 as i64);
        // D s_37_3: read-var d:i64
        let s_37_3: i64 = fn_state.d;
        // D s_37_4: cast zx s_37_3 -> i
        let s_37_4: i128 = (i128::try_from(s_37_3).unwrap());
        // D s_37_5: cast zx s_37_2 -> i
        let s_37_5: i128 = (i128::try_from(s_37_2).unwrap());
        // D s_37_6: read-var result:bv
        let s_37_6: Bits = fn_state.result;
        // D s_37_7: call P_set(s_37_4, s_37_5, s_37_6)
        let s_37_7: () = P_set(state, tracer, s_37_4, s_37_5, s_37_6);
        // N s_37_8: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var VLshadow#2223:i64
        let s_38_0: i64 = fn_state.VLshadow_2223;
        // D s_38_1: cast zx s_38_0 -> i
        let s_38_1: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_2: cast reint s_38_1 -> i64
        let s_38_2: i64 = (s_38_1 as i64);
        // D s_38_3: read-var m:i64
        let s_38_3: i64 = fn_state.m;
        // D s_38_4: cast zx s_38_3 -> i
        let s_38_4: i128 = (i128::try_from(s_38_3).unwrap());
        // D s_38_5: cast zx s_38_2 -> i
        let s_38_5: i128 = (i128::try_from(s_38_2).unwrap());
        // D s_38_6: call Z_read(s_38_4, s_38_5)
        let s_38_6: Bits = Z_read(state, tracer, s_38_4, s_38_5);
        // D s_38_7: write-var operand2 <= s_38_6
        fn_state.operand2 = s_38_6;
        // N s_38_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var VLshadow#2223:i64
        let s_39_0: i64 = fn_state.VLshadow_2223;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: cast reint s_39_1 -> i64
        let s_39_2: i64 = (s_39_1 as i64);
        // D s_39_3: read-var n:i64
        let s_39_3: i64 = fn_state.n;
        // D s_39_4: cast zx s_39_3 -> i
        let s_39_4: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_5: cast zx s_39_2 -> i
        let s_39_5: i128 = (i128::try_from(s_39_2).unwrap());
        // D s_39_6: call Z_read(s_39_4, s_39_5)
        let s_39_6: Bits = Z_read(state, tracer, s_39_4, s_39_5);
        // D s_39_7: write-var operand1 <= s_39_6
        fn_state.operand1 = s_39_6;
        // N s_39_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
