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
use integer_subrange::*;
use common::*;
pub fn execute_FCMGT_P_P_Z0__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    n: i64,
    op: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_175125: bool,
        gs_175115: bool,
        mask: Bits,
        gs_175106: i64,
        VLshadow_2047: i64,
        gs_175143: bool,
        VLshadow_2046: i64,
        psize: i64,
        elements: i64,
        element: Bits,
        gs_175164: bool,
        gs_175123: bool,
        operand: Bits,
        e: i64,
        gs_175163: bool,
        res: bool,
        PL: i64,
        gs_175145: bool,
        esizeshadow_2045: i64,
        pbit: bool,
        gs_175135: bool,
        gs_175182: bool,
        gs_175113: bool,
        gs_175133: bool,
        result: Bits,
        gs_175183: bool,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        n: i64,
        op: u32,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
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
        // D s_0_3: write-var esizeshadow#2045 <= s_0_2
        fn_state.esizeshadow_2045 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2046 <= s_0_6
        fn_state.VLshadow_2046 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2046:i64
        let s_1_0: i64 = fn_state.VLshadow_2046;
        // D s_1_1: write-var VLshadow#2047 <= s_1_0
        fn_state.VLshadow_2047 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2047:i64
        let s_1_3: i64 = fn_state.VLshadow_2047;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: write-var PL <= s_1_6
        fn_state.PL = s_1_6;
        // D s_1_8: read-var VLshadow#2047:i64
        let s_1_8: i64 = fn_state.VLshadow_2047;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: read-var esizeshadow#2045:i64
        let s_1_10: i64 = fn_state.esizeshadow_2045;
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
        // D s_1_23: read-var esizeshadow#2045:i64
        let s_1_23: i64 = fn_state.esizeshadow_2045;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: read-var mask:bv
        let s_1_25: Bits = fn_state.mask;
        // D s_1_26: call AnyActiveElement(s_1_25, s_1_24)
        let s_1_26: bool = AnyActiveElement(state, tracer, s_1_25, s_1_24);
        // N s_1_27: branch s_1_26 b68 b2
        if s_1_26 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2047:i64
        let s_2_0: i64 = fn_state.VLshadow_2047;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand <= s_2_2
        fn_state.operand = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var esizeshadow#2045:i64
        let s_3_1: i64 = fn_state.esizeshadow_2045;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: div s_3_2 s_3_0
        let s_3_3: i128 = ((s_3_2) / (s_3_0));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var psize <= s_3_4
        fn_state.psize = s_3_4;
        // C s_3_6: const #0s : i64
        let s_3_6: i64 = 0;
        // C s_3_7: const #1s : i
        let s_3_7: i128 = 1;
        // D s_3_8: read-var elements:i64
        let s_3_8: i64 = fn_state.elements;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: sub s_3_9 s_3_7
        let s_3_10: i128 = ((s_3_9) - (s_3_7));
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: write-var gs#175106 <= s_3_11
        fn_state.gs_175106 = s_3_11;
        // D s_3_13: write-var e <= s_3_6
        fn_state.e = s_3_6;
        // N s_3_14: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#175106:i64
        let s_4_1: i64 = fn_state.gs_175106;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b67 b5
        if s_4_2 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var esizeshadow#2045:i64
        let s_5_2: i64 = fn_state.esizeshadow_2045;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var mask:bv
        let s_5_4: Bits = fn_state.mask;
        // D s_5_5: call ActivePredicateElement(s_5_4, s_5_1, s_5_3)
        let s_5_5: bool = ActivePredicateElement(state, tracer, s_5_4, s_5_1, s_5_3);
        // N s_5_6: branch s_5_5 b8 b6
        if s_5_5 {
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
        // D s_6_0: read-var psize:i64
        let s_6_0: i64 = fn_state.psize;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // C s_6_3: const #0u : u8
        let s_6_3: bool = false;
        // C s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 1u16);
        // D s_6_5: read-var psize:i64
        let s_6_5: i64 = fn_state.psize;
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: bits-cast zx s_6_4 -> bv length s_6_6
        let s_6_7: Bits = s_6_4.zero_extend(s_6_6);
        // D s_6_8: read-var e:i64
        let s_6_8: i64 = fn_state.e;
        // D s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_10: cast zx s_6_2 -> i
        let s_6_10: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_11: read-var result:bv
        let s_6_11: Bits = fn_state.result;
        // D s_6_12: call Elem_set(s_6_11, s_6_9, s_6_10, s_6_7)
        let s_6_12: Bits = Elem_set(state, tracer, s_6_11, s_6_9, s_6_10, s_6_7);
        // D s_6_13: write-var result <= s_6_12
        fn_state.result = s_6_12;
        // N s_6_14: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var e <= s_7_2
        fn_state.e = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esizeshadow#2045:i64
        let s_8_0: i64 = fn_state.esizeshadow_2045;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var e:i64
        let s_8_3: i64 = fn_state.e;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cast zx s_8_2 -> i
        let s_8_5: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_6: read-var operand:bv
        let s_8_6: Bits = fn_state.operand;
        // D s_8_7: call Elem_read(s_8_6, s_8_4, s_8_5)
        let s_8_7: Bits = Elem_read(state, tracer, s_8_6, s_8_4, s_8_5);
        // D s_8_8: write-var element <= s_8_7
        fn_state.element = s_8_7;
        // C s_8_9: const #0u : u32
        let s_8_9: u32 = 0;
        // D s_8_10: read-var op:u32
        let s_8_10: u32 = fn_state.op;
        // D s_8_11: cmp-eq s_8_9 s_8_10
        let s_8_11: bool = ((s_8_9) == (s_8_10));
        // D s_8_12: not s_8_11
        let s_8_12: bool = !s_8_11;
        // N s_8_13: branch s_8_12 b21 b9
        if s_8_12 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esizeshadow#2045:i64
        let s_9_0: i64 = fn_state.esizeshadow_2045;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call __id(s_9_1)
        let s_9_2: i128 = u__id(state, tracer, s_9_1);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #16s : i
        let s_9_4: i128 = 16;
        // D s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_6: cmp-eq s_9_5 s_9_4
        let s_9_6: bool = ((s_9_5) == (s_9_4));
        // N s_9_7: branch s_9_6 b20 b10
        if s_9_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esizeshadow#2045:i64
        let s_10_0: i64 = fn_state.esizeshadow_2045;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call __id(s_10_1)
        let s_10_2: i128 = u__id(state, tracer, s_10_1);
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #32s : i
        let s_10_4: i128 = 32;
        // D s_10_5: cast zx s_10_3 -> i
        let s_10_5: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_6: cmp-eq s_10_5 s_10_4
        let s_10_6: bool = ((s_10_5) == (s_10_4));
        // D s_10_7: write-var gs#175113 <= s_10_6
        fn_state.gs_175113 = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#175113:u8
        let s_11_0: bool = fn_state.gs_175113;
        // N s_11_1: branch s_11_0 b19 b12
        if s_11_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esizeshadow#2045:i64
        let s_12_0: i64 = fn_state.esizeshadow_2045;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: call __id(s_12_1)
        let s_12_2: i128 = u__id(state, tracer, s_12_1);
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // C s_12_4: const #64s : i
        let s_12_4: i128 = 64;
        // D s_12_5: cast zx s_12_3 -> i
        let s_12_5: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_6: cmp-eq s_12_5 s_12_4
        let s_12_6: bool = ((s_12_5) == (s_12_4));
        // D s_12_7: write-var gs#175115 <= s_12_6
        fn_state.gs_175115 = s_12_6;
        // N s_12_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#175115:u8
        let s_13_0: bool = fn_state.gs_175115;
        // N s_13_1: assert s_13_0
        let s_13_1: () = assert!(s_13_0);
        // C s_13_2: const #1s : i
        let s_13_2: i128 = 1;
        // D s_13_3: read-var esizeshadow#2045:i64
        let s_13_3: i64 = fn_state.esizeshadow_2045;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: sub s_13_4 s_13_2
        let s_13_5: i128 = ((s_13_4) - (s_13_2));
        // D s_13_6: cast reint s_13_5 -> i64
        let s_13_6: i64 = (s_13_5 as i64);
        // C s_13_7: const #0s : i
        let s_13_7: i128 = 0;
        // C s_13_8: const #0s : i
        let s_13_8: i128 = 0;
        // D s_13_9: cast zx s_13_6 -> i
        let s_13_9: i128 = (i128::try_from(s_13_6).unwrap());
        // D s_13_10: call integer_subrange(s_13_7, s_13_9, s_13_8)
        let s_13_10: Bits = integer_subrange(state, tracer, s_13_7, s_13_9, s_13_8);
        // C s_13_11: const #() : ()
        let s_13_11: () = ();
        // S s_13_12: call FPCR_read(s_13_11)
        let s_13_12: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_13_11);
        // D s_13_13: read-var element:bv
        let s_13_13: Bits = fn_state.element;
        // D s_13_14: call FPCompareEQ(s_13_13, s_13_10, s_13_12)
        let s_13_14: bool = FPCompareEQ(state, tracer, s_13_13, s_13_10, s_13_12);
        // D s_13_15: write-var res <= s_13_14
        fn_state.res = s_13_14;
        // N s_13_16: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var res:u8
        let s_15_0: bool = fn_state.res;
        // N s_15_1: branch s_15_0 b18 b16
        if s_15_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var pbit <= s_16_0
        fn_state.pbit = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var psize:i64
        let s_17_0: i64 = fn_state.psize;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: cast reint s_17_1 -> i64
        let s_17_2: i64 = (s_17_1 as i64);
        // D s_17_3: read-var pbit:u8
        let s_17_3: bool = fn_state.pbit;
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 1u16);
        // D s_17_5: read-var psize:i64
        let s_17_5: i64 = fn_state.psize;
        // D s_17_6: cast zx s_17_5 -> i
        let s_17_6: i128 = (i128::try_from(s_17_5).unwrap());
        // D s_17_7: bits-cast zx s_17_4 -> bv length s_17_6
        let s_17_7: Bits = s_17_4.zero_extend(s_17_6);
        // D s_17_8: read-var e:i64
        let s_17_8: i64 = fn_state.e;
        // D s_17_9: cast zx s_17_8 -> i
        let s_17_9: i128 = (i128::try_from(s_17_8).unwrap());
        // D s_17_10: cast zx s_17_2 -> i
        let s_17_10: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_11: read-var result:bv
        let s_17_11: Bits = fn_state.result;
        // D s_17_12: call Elem_set(s_17_11, s_17_9, s_17_10, s_17_7)
        let s_17_12: Bits = Elem_set(state, tracer, s_17_11, s_17_9, s_17_10, s_17_7);
        // D s_17_13: write-var result <= s_17_12
        fn_state.result = s_17_12;
        // N s_17_14: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var pbit <= s_18_0
        fn_state.pbit = s_18_0;
        // N s_18_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#175115 <= s_19_0
        fn_state.gs_175115 = s_19_0;
        // N s_19_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#175113 <= s_20_0
        fn_state.gs_175113 = s_20_0;
        // N s_20_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #2u : u32
        let s_21_0: u32 = 2;
        // D s_21_1: read-var op:u32
        let s_21_1: u32 = fn_state.op;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // D s_21_3: not s_21_2
        let s_21_3: bool = !s_21_2;
        // N s_21_4: branch s_21_3 b30 b22
        if s_21_3 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var esizeshadow#2045:i64
        let s_22_0: i64 = fn_state.esizeshadow_2045;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: call __id(s_22_1)
        let s_22_2: i128 = u__id(state, tracer, s_22_1);
        // D s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: const #16s : i
        let s_22_4: i128 = 16;
        // D s_22_5: cast zx s_22_3 -> i
        let s_22_5: i128 = (i128::try_from(s_22_3).unwrap());
        // D s_22_6: cmp-eq s_22_5 s_22_4
        let s_22_6: bool = ((s_22_5) == (s_22_4));
        // N s_22_7: branch s_22_6 b29 b23
        if s_22_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var esizeshadow#2045:i64
        let s_23_0: i64 = fn_state.esizeshadow_2045;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: call __id(s_23_1)
        let s_23_2: i128 = u__id(state, tracer, s_23_1);
        // D s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: const #32s : i
        let s_23_4: i128 = 32;
        // D s_23_5: cast zx s_23_3 -> i
        let s_23_5: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_6: cmp-eq s_23_5 s_23_4
        let s_23_6: bool = ((s_23_5) == (s_23_4));
        // D s_23_7: write-var gs#175123 <= s_23_6
        fn_state.gs_175123 = s_23_6;
        // N s_23_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#175123:u8
        let s_24_0: bool = fn_state.gs_175123;
        // N s_24_1: branch s_24_0 b28 b25
        if s_24_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var esizeshadow#2045:i64
        let s_25_0: i64 = fn_state.esizeshadow_2045;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: call __id(s_25_1)
        let s_25_2: i128 = u__id(state, tracer, s_25_1);
        // D s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: const #64s : i
        let s_25_4: i128 = 64;
        // D s_25_5: cast zx s_25_3 -> i
        let s_25_5: i128 = (i128::try_from(s_25_3).unwrap());
        // D s_25_6: cmp-eq s_25_5 s_25_4
        let s_25_6: bool = ((s_25_5) == (s_25_4));
        // D s_25_7: write-var gs#175125 <= s_25_6
        fn_state.gs_175125 = s_25_6;
        // N s_25_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#175125:u8
        let s_26_0: bool = fn_state.gs_175125;
        // N s_26_1: assert s_26_0
        let s_26_1: () = assert!(s_26_0);
        // C s_26_2: const #1s : i
        let s_26_2: i128 = 1;
        // D s_26_3: read-var esizeshadow#2045:i64
        let s_26_3: i64 = fn_state.esizeshadow_2045;
        // D s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_5: sub s_26_4 s_26_2
        let s_26_5: i128 = ((s_26_4) - (s_26_2));
        // D s_26_6: cast reint s_26_5 -> i64
        let s_26_6: i64 = (s_26_5 as i64);
        // C s_26_7: const #0s : i
        let s_26_7: i128 = 0;
        // C s_26_8: const #0s : i
        let s_26_8: i128 = 0;
        // D s_26_9: cast zx s_26_6 -> i
        let s_26_9: i128 = (i128::try_from(s_26_6).unwrap());
        // D s_26_10: call integer_subrange(s_26_7, s_26_9, s_26_8)
        let s_26_10: Bits = integer_subrange(state, tracer, s_26_7, s_26_9, s_26_8);
        // C s_26_11: const #() : ()
        let s_26_11: () = ();
        // S s_26_12: call FPCR_read(s_26_11)
        let s_26_12: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_26_11);
        // D s_26_13: read-var element:bv
        let s_26_13: Bits = fn_state.element;
        // D s_26_14: call FPCompareGE(s_26_13, s_26_10, s_26_12)
        let s_26_14: bool = FPCompareGE(state, tracer, s_26_13, s_26_10, s_26_12);
        // D s_26_15: write-var res <= s_26_14
        fn_state.res = s_26_14;
        // N s_26_16: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#175125 <= s_28_0
        fn_state.gs_175125 = s_28_0;
        // N s_28_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#175123 <= s_29_0
        fn_state.gs_175123 = s_29_0;
        // N s_29_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #3u : u32
        let s_30_0: u32 = 3;
        // D s_30_1: read-var op:u32
        let s_30_1: u32 = fn_state.op;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // N s_30_4: branch s_30_3 b39 b31
        if s_30_3 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var esizeshadow#2045:i64
        let s_31_0: i64 = fn_state.esizeshadow_2045;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: call __id(s_31_1)
        let s_31_2: i128 = u__id(state, tracer, s_31_1);
        // D s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: const #16s : i
        let s_31_4: i128 = 16;
        // D s_31_5: cast zx s_31_3 -> i
        let s_31_5: i128 = (i128::try_from(s_31_3).unwrap());
        // D s_31_6: cmp-eq s_31_5 s_31_4
        let s_31_6: bool = ((s_31_5) == (s_31_4));
        // N s_31_7: branch s_31_6 b38 b32
        if s_31_6 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var esizeshadow#2045:i64
        let s_32_0: i64 = fn_state.esizeshadow_2045;
        // D s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_2: call __id(s_32_1)
        let s_32_2: i128 = u__id(state, tracer, s_32_1);
        // D s_32_3: cast reint s_32_2 -> i64
        let s_32_3: i64 = (s_32_2 as i64);
        // C s_32_4: const #32s : i
        let s_32_4: i128 = 32;
        // D s_32_5: cast zx s_32_3 -> i
        let s_32_5: i128 = (i128::try_from(s_32_3).unwrap());
        // D s_32_6: cmp-eq s_32_5 s_32_4
        let s_32_6: bool = ((s_32_5) == (s_32_4));
        // D s_32_7: write-var gs#175133 <= s_32_6
        fn_state.gs_175133 = s_32_6;
        // N s_32_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#175133:u8
        let s_33_0: bool = fn_state.gs_175133;
        // N s_33_1: branch s_33_0 b37 b34
        if s_33_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var esizeshadow#2045:i64
        let s_34_0: i64 = fn_state.esizeshadow_2045;
        // D s_34_1: cast zx s_34_0 -> i
        let s_34_1: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_2: call __id(s_34_1)
        let s_34_2: i128 = u__id(state, tracer, s_34_1);
        // D s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: const #64s : i
        let s_34_4: i128 = 64;
        // D s_34_5: cast zx s_34_3 -> i
        let s_34_5: i128 = (i128::try_from(s_34_3).unwrap());
        // D s_34_6: cmp-eq s_34_5 s_34_4
        let s_34_6: bool = ((s_34_5) == (s_34_4));
        // D s_34_7: write-var gs#175135 <= s_34_6
        fn_state.gs_175135 = s_34_6;
        // N s_34_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#175135:u8
        let s_35_0: bool = fn_state.gs_175135;
        // N s_35_1: assert s_35_0
        let s_35_1: () = assert!(s_35_0);
        // C s_35_2: const #1s : i
        let s_35_2: i128 = 1;
        // D s_35_3: read-var esizeshadow#2045:i64
        let s_35_3: i64 = fn_state.esizeshadow_2045;
        // D s_35_4: cast zx s_35_3 -> i
        let s_35_4: i128 = (i128::try_from(s_35_3).unwrap());
        // D s_35_5: sub s_35_4 s_35_2
        let s_35_5: i128 = ((s_35_4) - (s_35_2));
        // D s_35_6: cast reint s_35_5 -> i64
        let s_35_6: i64 = (s_35_5 as i64);
        // C s_35_7: const #0s : i
        let s_35_7: i128 = 0;
        // C s_35_8: const #0s : i
        let s_35_8: i128 = 0;
        // D s_35_9: cast zx s_35_6 -> i
        let s_35_9: i128 = (i128::try_from(s_35_6).unwrap());
        // D s_35_10: call integer_subrange(s_35_7, s_35_9, s_35_8)
        let s_35_10: Bits = integer_subrange(state, tracer, s_35_7, s_35_9, s_35_8);
        // C s_35_11: const #() : ()
        let s_35_11: () = ();
        // S s_35_12: call FPCR_read(s_35_11)
        let s_35_12: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_35_11);
        // D s_35_13: read-var element:bv
        let s_35_13: Bits = fn_state.element;
        // D s_35_14: call FPCompareGT(s_35_13, s_35_10, s_35_12)
        let s_35_14: bool = FPCompareGT(state, tracer, s_35_13, s_35_10, s_35_12);
        // D s_35_15: write-var res <= s_35_14
        fn_state.res = s_35_14;
        // N s_35_16: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#175135 <= s_37_0
        fn_state.gs_175135 = s_37_0;
        // N s_37_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#175133 <= s_38_0
        fn_state.gs_175133 = s_38_0;
        // N s_38_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u32
        let s_39_0: u32 = 1;
        // D s_39_1: read-var op:u32
        let s_39_1: u32 = fn_state.op;
        // D s_39_2: cmp-eq s_39_0 s_39_1
        let s_39_2: bool = ((s_39_0) == (s_39_1));
        // D s_39_3: not s_39_2
        let s_39_3: bool = !s_39_2;
        // N s_39_4: branch s_39_3 b48 b40
        if s_39_3 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var esizeshadow#2045:i64
        let s_40_0: i64 = fn_state.esizeshadow_2045;
        // D s_40_1: cast zx s_40_0 -> i
        let s_40_1: i128 = (i128::try_from(s_40_0).unwrap());
        // D s_40_2: call __id(s_40_1)
        let s_40_2: i128 = u__id(state, tracer, s_40_1);
        // D s_40_3: cast reint s_40_2 -> i64
        let s_40_3: i64 = (s_40_2 as i64);
        // C s_40_4: const #16s : i
        let s_40_4: i128 = 16;
        // D s_40_5: cast zx s_40_3 -> i
        let s_40_5: i128 = (i128::try_from(s_40_3).unwrap());
        // D s_40_6: cmp-eq s_40_5 s_40_4
        let s_40_6: bool = ((s_40_5) == (s_40_4));
        // N s_40_7: branch s_40_6 b47 b41
        if s_40_6 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var esizeshadow#2045:i64
        let s_41_0: i64 = fn_state.esizeshadow_2045;
        // D s_41_1: cast zx s_41_0 -> i
        let s_41_1: i128 = (i128::try_from(s_41_0).unwrap());
        // D s_41_2: call __id(s_41_1)
        let s_41_2: i128 = u__id(state, tracer, s_41_1);
        // D s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: const #32s : i
        let s_41_4: i128 = 32;
        // D s_41_5: cast zx s_41_3 -> i
        let s_41_5: i128 = (i128::try_from(s_41_3).unwrap());
        // D s_41_6: cmp-eq s_41_5 s_41_4
        let s_41_6: bool = ((s_41_5) == (s_41_4));
        // D s_41_7: write-var gs#175143 <= s_41_6
        fn_state.gs_175143 = s_41_6;
        // N s_41_8: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#175143:u8
        let s_42_0: bool = fn_state.gs_175143;
        // N s_42_1: branch s_42_0 b46 b43
        if s_42_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var esizeshadow#2045:i64
        let s_43_0: i64 = fn_state.esizeshadow_2045;
        // D s_43_1: cast zx s_43_0 -> i
        let s_43_1: i128 = (i128::try_from(s_43_0).unwrap());
        // D s_43_2: call __id(s_43_1)
        let s_43_2: i128 = u__id(state, tracer, s_43_1);
        // D s_43_3: cast reint s_43_2 -> i64
        let s_43_3: i64 = (s_43_2 as i64);
        // C s_43_4: const #64s : i
        let s_43_4: i128 = 64;
        // D s_43_5: cast zx s_43_3 -> i
        let s_43_5: i128 = (i128::try_from(s_43_3).unwrap());
        // D s_43_6: cmp-eq s_43_5 s_43_4
        let s_43_6: bool = ((s_43_5) == (s_43_4));
        // D s_43_7: write-var gs#175145 <= s_43_6
        fn_state.gs_175145 = s_43_6;
        // N s_43_8: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#175145:u8
        let s_44_0: bool = fn_state.gs_175145;
        // N s_44_1: assert s_44_0
        let s_44_1: () = assert!(s_44_0);
        // C s_44_2: const #1s : i
        let s_44_2: i128 = 1;
        // D s_44_3: read-var esizeshadow#2045:i64
        let s_44_3: i64 = fn_state.esizeshadow_2045;
        // D s_44_4: cast zx s_44_3 -> i
        let s_44_4: i128 = (i128::try_from(s_44_3).unwrap());
        // D s_44_5: sub s_44_4 s_44_2
        let s_44_5: i128 = ((s_44_4) - (s_44_2));
        // D s_44_6: cast reint s_44_5 -> i64
        let s_44_6: i64 = (s_44_5 as i64);
        // C s_44_7: const #0s : i
        let s_44_7: i128 = 0;
        // C s_44_8: const #0s : i
        let s_44_8: i128 = 0;
        // D s_44_9: cast zx s_44_6 -> i
        let s_44_9: i128 = (i128::try_from(s_44_6).unwrap());
        // D s_44_10: call integer_subrange(s_44_7, s_44_9, s_44_8)
        let s_44_10: Bits = integer_subrange(state, tracer, s_44_7, s_44_9, s_44_8);
        // C s_44_11: const #() : ()
        let s_44_11: () = ();
        // S s_44_12: call FPCR_read(s_44_11)
        let s_44_12: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_44_11);
        // D s_44_13: read-var element:bv
        let s_44_13: Bits = fn_state.element;
        // D s_44_14: call FPCompareNE(s_44_13, s_44_10, s_44_12)
        let s_44_14: bool = FPCompareNE(state, tracer, s_44_13, s_44_10, s_44_12);
        // D s_44_15: write-var res <= s_44_14
        fn_state.res = s_44_14;
        // N s_44_16: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#175145 <= s_46_0
        fn_state.gs_175145 = s_46_0;
        // N s_46_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#175143 <= s_47_0
        fn_state.gs_175143 = s_47_0;
        // N s_47_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #4u : u32
        let s_48_0: u32 = 4;
        // D s_48_1: read-var op:u32
        let s_48_1: u32 = fn_state.op;
        // D s_48_2: cmp-eq s_48_0 s_48_1
        let s_48_2: bool = ((s_48_0) == (s_48_1));
        // D s_48_3: not s_48_2
        let s_48_3: bool = !s_48_2;
        // N s_48_4: branch s_48_3 b57 b49
        if s_48_3 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var esizeshadow#2045:i64
        let s_49_0: i64 = fn_state.esizeshadow_2045;
        // D s_49_1: cast zx s_49_0 -> i
        let s_49_1: i128 = (i128::try_from(s_49_0).unwrap());
        // D s_49_2: call __id(s_49_1)
        let s_49_2: i128 = u__id(state, tracer, s_49_1);
        // D s_49_3: cast reint s_49_2 -> i64
        let s_49_3: i64 = (s_49_2 as i64);
        // C s_49_4: const #1s : i
        let s_49_4: i128 = 1;
        // D s_49_5: cast zx s_49_3 -> i
        let s_49_5: i128 = (i128::try_from(s_49_3).unwrap());
        // D s_49_6: sub s_49_5 s_49_4
        let s_49_6: i128 = ((s_49_5) - (s_49_4));
        // D s_49_7: cast reint s_49_6 -> i64
        let s_49_7: i64 = (s_49_6 as i64);
        // C s_49_8: const #0s : i
        let s_49_8: i128 = 0;
        // D s_49_9: cast zx s_49_7 -> i
        let s_49_9: i128 = (i128::try_from(s_49_7).unwrap());
        // D s_49_10: sub s_49_9 s_49_8
        let s_49_10: i128 = ((s_49_9) - (s_49_8));
        // D s_49_11: cast reint s_49_10 -> i64
        let s_49_11: i64 = (s_49_10 as i64);
        // C s_49_12: const #1s : i
        let s_49_12: i128 = 1;
        // D s_49_13: cast zx s_49_11 -> i
        let s_49_13: i128 = (i128::try_from(s_49_11).unwrap());
        // D s_49_14: add s_49_13 s_49_12
        let s_49_14: i128 = (s_49_13 + s_49_12);
        // D s_49_15: cast reint s_49_14 -> i64
        let s_49_15: i64 = (s_49_14 as i64);
        // C s_49_16: const #16s : i
        let s_49_16: i128 = 16;
        // D s_49_17: cast zx s_49_15 -> i
        let s_49_17: i128 = (i128::try_from(s_49_15).unwrap());
        // D s_49_18: cmp-eq s_49_17 s_49_16
        let s_49_18: bool = ((s_49_17) == (s_49_16));
        // N s_49_19: branch s_49_18 b56 b50
        if s_49_18 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var esizeshadow#2045:i64
        let s_50_0: i64 = fn_state.esizeshadow_2045;
        // D s_50_1: cast zx s_50_0 -> i
        let s_50_1: i128 = (i128::try_from(s_50_0).unwrap());
        // D s_50_2: call __id(s_50_1)
        let s_50_2: i128 = u__id(state, tracer, s_50_1);
        // D s_50_3: cast reint s_50_2 -> i64
        let s_50_3: i64 = (s_50_2 as i64);
        // C s_50_4: const #1s : i
        let s_50_4: i128 = 1;
        // D s_50_5: cast zx s_50_3 -> i
        let s_50_5: i128 = (i128::try_from(s_50_3).unwrap());
        // D s_50_6: sub s_50_5 s_50_4
        let s_50_6: i128 = ((s_50_5) - (s_50_4));
        // D s_50_7: cast reint s_50_6 -> i64
        let s_50_7: i64 = (s_50_6 as i64);
        // C s_50_8: const #0s : i
        let s_50_8: i128 = 0;
        // D s_50_9: cast zx s_50_7 -> i
        let s_50_9: i128 = (i128::try_from(s_50_7).unwrap());
        // D s_50_10: sub s_50_9 s_50_8
        let s_50_10: i128 = ((s_50_9) - (s_50_8));
        // D s_50_11: cast reint s_50_10 -> i64
        let s_50_11: i64 = (s_50_10 as i64);
        // C s_50_12: const #1s : i
        let s_50_12: i128 = 1;
        // D s_50_13: cast zx s_50_11 -> i
        let s_50_13: i128 = (i128::try_from(s_50_11).unwrap());
        // D s_50_14: add s_50_13 s_50_12
        let s_50_14: i128 = (s_50_13 + s_50_12);
        // D s_50_15: cast reint s_50_14 -> i64
        let s_50_15: i64 = (s_50_14 as i64);
        // C s_50_16: const #32s : i
        let s_50_16: i128 = 32;
        // D s_50_17: cast zx s_50_15 -> i
        let s_50_17: i128 = (i128::try_from(s_50_15).unwrap());
        // D s_50_18: cmp-eq s_50_17 s_50_16
        let s_50_18: bool = ((s_50_17) == (s_50_16));
        // N s_50_19: branch s_50_18 b55 b51
        if s_50_18 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var esizeshadow#2045:i64
        let s_51_0: i64 = fn_state.esizeshadow_2045;
        // D s_51_1: cast zx s_51_0 -> i
        let s_51_1: i128 = (i128::try_from(s_51_0).unwrap());
        // D s_51_2: call __id(s_51_1)
        let s_51_2: i128 = u__id(state, tracer, s_51_1);
        // D s_51_3: cast reint s_51_2 -> i64
        let s_51_3: i64 = (s_51_2 as i64);
        // C s_51_4: const #1s : i
        let s_51_4: i128 = 1;
        // D s_51_5: cast zx s_51_3 -> i
        let s_51_5: i128 = (i128::try_from(s_51_3).unwrap());
        // D s_51_6: sub s_51_5 s_51_4
        let s_51_6: i128 = ((s_51_5) - (s_51_4));
        // D s_51_7: cast reint s_51_6 -> i64
        let s_51_7: i64 = (s_51_6 as i64);
        // C s_51_8: const #0s : i
        let s_51_8: i128 = 0;
        // D s_51_9: cast zx s_51_7 -> i
        let s_51_9: i128 = (i128::try_from(s_51_7).unwrap());
        // D s_51_10: sub s_51_9 s_51_8
        let s_51_10: i128 = ((s_51_9) - (s_51_8));
        // D s_51_11: cast reint s_51_10 -> i64
        let s_51_11: i64 = (s_51_10 as i64);
        // C s_51_12: const #1s : i
        let s_51_12: i128 = 1;
        // D s_51_13: cast zx s_51_11 -> i
        let s_51_13: i128 = (i128::try_from(s_51_11).unwrap());
        // D s_51_14: add s_51_13 s_51_12
        let s_51_14: i128 = (s_51_13 + s_51_12);
        // D s_51_15: cast reint s_51_14 -> i64
        let s_51_15: i64 = (s_51_14 as i64);
        // C s_51_16: const #64s : i
        let s_51_16: i128 = 64;
        // D s_51_17: cast zx s_51_15 -> i
        let s_51_17: i128 = (i128::try_from(s_51_15).unwrap());
        // D s_51_18: cmp-eq s_51_17 s_51_16
        let s_51_18: bool = ((s_51_17) == (s_51_16));
        // D s_51_19: write-var gs#175163 <= s_51_18
        fn_state.gs_175163 = s_51_18;
        // N s_51_20: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#175163:u8
        let s_52_0: bool = fn_state.gs_175163;
        // D s_52_1: write-var gs#175164 <= s_52_0
        fn_state.gs_175164 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#175164:u8
        let s_53_0: bool = fn_state.gs_175164;
        // N s_53_1: assert s_53_0
        let s_53_1: () = assert!(s_53_0);
        // C s_53_2: const #1s : i
        let s_53_2: i128 = 1;
        // D s_53_3: read-var esizeshadow#2045:i64
        let s_53_3: i64 = fn_state.esizeshadow_2045;
        // D s_53_4: cast zx s_53_3 -> i
        let s_53_4: i128 = (i128::try_from(s_53_3).unwrap());
        // D s_53_5: sub s_53_4 s_53_2
        let s_53_5: i128 = ((s_53_4) - (s_53_2));
        // D s_53_6: cast reint s_53_5 -> i64
        let s_53_6: i64 = (s_53_5 as i64);
        // C s_53_7: const #0s : i
        let s_53_7: i128 = 0;
        // C s_53_8: const #0s : i
        let s_53_8: i128 = 0;
        // D s_53_9: cast zx s_53_6 -> i
        let s_53_9: i128 = (i128::try_from(s_53_6).unwrap());
        // D s_53_10: call integer_subrange(s_53_7, s_53_9, s_53_8)
        let s_53_10: Bits = integer_subrange(state, tracer, s_53_7, s_53_9, s_53_8);
        // C s_53_11: const #() : ()
        let s_53_11: () = ();
        // S s_53_12: call FPCR_read(s_53_11)
        let s_53_12: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_53_11);
        // D s_53_13: read-var element:bv
        let s_53_13: Bits = fn_state.element;
        // D s_53_14: call FPCompareGT(s_53_10, s_53_13, s_53_12)
        let s_53_14: bool = FPCompareGT(state, tracer, s_53_10, s_53_13, s_53_12);
        // D s_53_15: write-var res <= s_53_14
        fn_state.res = s_53_14;
        // N s_53_16: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_54_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#175163 <= s_55_0
        fn_state.gs_175163 = s_55_0;
        // N s_55_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#175164 <= s_56_0
        fn_state.gs_175164 = s_56_0;
        // N s_56_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #5u : u32
        let s_57_0: u32 = 5;
        // D s_57_1: read-var op:u32
        let s_57_1: u32 = fn_state.op;
        // D s_57_2: cmp-eq s_57_0 s_57_1
        let s_57_2: bool = ((s_57_0) == (s_57_1));
        // D s_57_3: not s_57_2
        let s_57_3: bool = !s_57_2;
        // N s_57_4: branch s_57_3 b66 b58
        if s_57_3 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var esizeshadow#2045:i64
        let s_58_0: i64 = fn_state.esizeshadow_2045;
        // D s_58_1: cast zx s_58_0 -> i
        let s_58_1: i128 = (i128::try_from(s_58_0).unwrap());
        // D s_58_2: call __id(s_58_1)
        let s_58_2: i128 = u__id(state, tracer, s_58_1);
        // D s_58_3: cast reint s_58_2 -> i64
        let s_58_3: i64 = (s_58_2 as i64);
        // C s_58_4: const #1s : i
        let s_58_4: i128 = 1;
        // D s_58_5: cast zx s_58_3 -> i
        let s_58_5: i128 = (i128::try_from(s_58_3).unwrap());
        // D s_58_6: sub s_58_5 s_58_4
        let s_58_6: i128 = ((s_58_5) - (s_58_4));
        // D s_58_7: cast reint s_58_6 -> i64
        let s_58_7: i64 = (s_58_6 as i64);
        // C s_58_8: const #0s : i
        let s_58_8: i128 = 0;
        // D s_58_9: cast zx s_58_7 -> i
        let s_58_9: i128 = (i128::try_from(s_58_7).unwrap());
        // D s_58_10: sub s_58_9 s_58_8
        let s_58_10: i128 = ((s_58_9) - (s_58_8));
        // D s_58_11: cast reint s_58_10 -> i64
        let s_58_11: i64 = (s_58_10 as i64);
        // C s_58_12: const #1s : i
        let s_58_12: i128 = 1;
        // D s_58_13: cast zx s_58_11 -> i
        let s_58_13: i128 = (i128::try_from(s_58_11).unwrap());
        // D s_58_14: add s_58_13 s_58_12
        let s_58_14: i128 = (s_58_13 + s_58_12);
        // D s_58_15: cast reint s_58_14 -> i64
        let s_58_15: i64 = (s_58_14 as i64);
        // C s_58_16: const #16s : i
        let s_58_16: i128 = 16;
        // D s_58_17: cast zx s_58_15 -> i
        let s_58_17: i128 = (i128::try_from(s_58_15).unwrap());
        // D s_58_18: cmp-eq s_58_17 s_58_16
        let s_58_18: bool = ((s_58_17) == (s_58_16));
        // N s_58_19: branch s_58_18 b65 b59
        if s_58_18 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var esizeshadow#2045:i64
        let s_59_0: i64 = fn_state.esizeshadow_2045;
        // D s_59_1: cast zx s_59_0 -> i
        let s_59_1: i128 = (i128::try_from(s_59_0).unwrap());
        // D s_59_2: call __id(s_59_1)
        let s_59_2: i128 = u__id(state, tracer, s_59_1);
        // D s_59_3: cast reint s_59_2 -> i64
        let s_59_3: i64 = (s_59_2 as i64);
        // C s_59_4: const #1s : i
        let s_59_4: i128 = 1;
        // D s_59_5: cast zx s_59_3 -> i
        let s_59_5: i128 = (i128::try_from(s_59_3).unwrap());
        // D s_59_6: sub s_59_5 s_59_4
        let s_59_6: i128 = ((s_59_5) - (s_59_4));
        // D s_59_7: cast reint s_59_6 -> i64
        let s_59_7: i64 = (s_59_6 as i64);
        // C s_59_8: const #0s : i
        let s_59_8: i128 = 0;
        // D s_59_9: cast zx s_59_7 -> i
        let s_59_9: i128 = (i128::try_from(s_59_7).unwrap());
        // D s_59_10: sub s_59_9 s_59_8
        let s_59_10: i128 = ((s_59_9) - (s_59_8));
        // D s_59_11: cast reint s_59_10 -> i64
        let s_59_11: i64 = (s_59_10 as i64);
        // C s_59_12: const #1s : i
        let s_59_12: i128 = 1;
        // D s_59_13: cast zx s_59_11 -> i
        let s_59_13: i128 = (i128::try_from(s_59_11).unwrap());
        // D s_59_14: add s_59_13 s_59_12
        let s_59_14: i128 = (s_59_13 + s_59_12);
        // D s_59_15: cast reint s_59_14 -> i64
        let s_59_15: i64 = (s_59_14 as i64);
        // C s_59_16: const #32s : i
        let s_59_16: i128 = 32;
        // D s_59_17: cast zx s_59_15 -> i
        let s_59_17: i128 = (i128::try_from(s_59_15).unwrap());
        // D s_59_18: cmp-eq s_59_17 s_59_16
        let s_59_18: bool = ((s_59_17) == (s_59_16));
        // N s_59_19: branch s_59_18 b64 b60
        if s_59_18 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var esizeshadow#2045:i64
        let s_60_0: i64 = fn_state.esizeshadow_2045;
        // D s_60_1: cast zx s_60_0 -> i
        let s_60_1: i128 = (i128::try_from(s_60_0).unwrap());
        // D s_60_2: call __id(s_60_1)
        let s_60_2: i128 = u__id(state, tracer, s_60_1);
        // D s_60_3: cast reint s_60_2 -> i64
        let s_60_3: i64 = (s_60_2 as i64);
        // C s_60_4: const #1s : i
        let s_60_4: i128 = 1;
        // D s_60_5: cast zx s_60_3 -> i
        let s_60_5: i128 = (i128::try_from(s_60_3).unwrap());
        // D s_60_6: sub s_60_5 s_60_4
        let s_60_6: i128 = ((s_60_5) - (s_60_4));
        // D s_60_7: cast reint s_60_6 -> i64
        let s_60_7: i64 = (s_60_6 as i64);
        // C s_60_8: const #0s : i
        let s_60_8: i128 = 0;
        // D s_60_9: cast zx s_60_7 -> i
        let s_60_9: i128 = (i128::try_from(s_60_7).unwrap());
        // D s_60_10: sub s_60_9 s_60_8
        let s_60_10: i128 = ((s_60_9) - (s_60_8));
        // D s_60_11: cast reint s_60_10 -> i64
        let s_60_11: i64 = (s_60_10 as i64);
        // C s_60_12: const #1s : i
        let s_60_12: i128 = 1;
        // D s_60_13: cast zx s_60_11 -> i
        let s_60_13: i128 = (i128::try_from(s_60_11).unwrap());
        // D s_60_14: add s_60_13 s_60_12
        let s_60_14: i128 = (s_60_13 + s_60_12);
        // D s_60_15: cast reint s_60_14 -> i64
        let s_60_15: i64 = (s_60_14 as i64);
        // C s_60_16: const #64s : i
        let s_60_16: i128 = 64;
        // D s_60_17: cast zx s_60_15 -> i
        let s_60_17: i128 = (i128::try_from(s_60_15).unwrap());
        // D s_60_18: cmp-eq s_60_17 s_60_16
        let s_60_18: bool = ((s_60_17) == (s_60_16));
        // D s_60_19: write-var gs#175182 <= s_60_18
        fn_state.gs_175182 = s_60_18;
        // N s_60_20: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#175182:u8
        let s_61_0: bool = fn_state.gs_175182;
        // D s_61_1: write-var gs#175183 <= s_61_0
        fn_state.gs_175183 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#175183:u8
        let s_62_0: bool = fn_state.gs_175183;
        // N s_62_1: assert s_62_0
        let s_62_1: () = assert!(s_62_0);
        // C s_62_2: const #1s : i
        let s_62_2: i128 = 1;
        // D s_62_3: read-var esizeshadow#2045:i64
        let s_62_3: i64 = fn_state.esizeshadow_2045;
        // D s_62_4: cast zx s_62_3 -> i
        let s_62_4: i128 = (i128::try_from(s_62_3).unwrap());
        // D s_62_5: sub s_62_4 s_62_2
        let s_62_5: i128 = ((s_62_4) - (s_62_2));
        // D s_62_6: cast reint s_62_5 -> i64
        let s_62_6: i64 = (s_62_5 as i64);
        // C s_62_7: const #0s : i
        let s_62_7: i128 = 0;
        // C s_62_8: const #0s : i
        let s_62_8: i128 = 0;
        // D s_62_9: cast zx s_62_6 -> i
        let s_62_9: i128 = (i128::try_from(s_62_6).unwrap());
        // D s_62_10: call integer_subrange(s_62_7, s_62_9, s_62_8)
        let s_62_10: Bits = integer_subrange(state, tracer, s_62_7, s_62_9, s_62_8);
        // C s_62_11: const #() : ()
        let s_62_11: () = ();
        // S s_62_12: call FPCR_read(s_62_11)
        let s_62_12: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_62_11);
        // D s_62_13: read-var element:bv
        let s_62_13: Bits = fn_state.element;
        // D s_62_14: call FPCompareGE(s_62_10, s_62_13, s_62_12)
        let s_62_14: bool = FPCompareGE(state, tracer, s_62_10, s_62_13, s_62_12);
        // D s_62_15: write-var res <= s_62_14
        fn_state.res = s_62_14;
        // N s_62_16: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_63_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // D s_64_1: write-var gs#175182 <= s_64_0
        fn_state.gs_175182 = s_64_0;
        // N s_64_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#175183 <= s_65_0
        fn_state.gs_175183 = s_65_0;
        // N s_65_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_66_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var PL:i64
        let s_67_0: i64 = fn_state.PL;
        // D s_67_1: cast zx s_67_0 -> i
        let s_67_1: i128 = (i128::try_from(s_67_0).unwrap());
        // D s_67_2: cast reint s_67_1 -> i64
        let s_67_2: i64 = (s_67_1 as i64);
        // D s_67_3: read-var d:i64
        let s_67_3: i64 = fn_state.d;
        // D s_67_4: cast zx s_67_3 -> i
        let s_67_4: i128 = (i128::try_from(s_67_3).unwrap());
        // D s_67_5: cast zx s_67_2 -> i
        let s_67_5: i128 = (i128::try_from(s_67_2).unwrap());
        // D s_67_6: read-var result:bv
        let s_67_6: Bits = fn_state.result;
        // D s_67_7: call P_set(s_67_4, s_67_5, s_67_6)
        let s_67_7: () = P_set(state, tracer, s_67_4, s_67_5, s_67_6);
        // N s_67_8: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var VLshadow#2047:i64
        let s_68_0: i64 = fn_state.VLshadow_2047;
        // D s_68_1: cast zx s_68_0 -> i
        let s_68_1: i128 = (i128::try_from(s_68_0).unwrap());
        // D s_68_2: cast reint s_68_1 -> i64
        let s_68_2: i64 = (s_68_1 as i64);
        // D s_68_3: read-var n:i64
        let s_68_3: i64 = fn_state.n;
        // D s_68_4: cast zx s_68_3 -> i
        let s_68_4: i128 = (i128::try_from(s_68_3).unwrap());
        // D s_68_5: cast zx s_68_2 -> i
        let s_68_5: i128 = (i128::try_from(s_68_2).unwrap());
        // D s_68_6: call Z_read(s_68_4, s_68_5)
        let s_68_6: Bits = Z_read(state, tracer, s_68_4, s_68_5);
        // D s_68_7: write-var operand <= s_68_6
        fn_state.operand = s_68_6;
        // N s_68_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
