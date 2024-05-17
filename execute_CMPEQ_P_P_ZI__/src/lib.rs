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
use neq_int::*;
use Elem_set::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use PredTest::*;
use asl_Int::*;
use Zeros::*;
use Elem_read::*;
use P_set::*;
use Z_read::*;
use common::*;
pub fn execute_CMPEQ_P_P_ZI__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    imm: i128,
    n: i64,
    op: u32,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        VLshadow_2611: i64,
        gs_187919: i64,
        PL: i64,
        mask: Bits,
        esizeshadow_2610: i64,
        element1: i128,
        pbit: bool,
        VLshadow_2612: i64,
        psize: i64,
        elements: i64,
        result: Bits,
        operand1: Bits,
        cond: bool,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        imm: i128,
        n: i64,
        op: u32,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
        imm,
        n,
        op,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#2610 <= s_0_2
        fn_state.esizeshadow_2610 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2611 <= s_0_6
        fn_state.VLshadow_2611 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2611:i64
        let s_1_0: i64 = fn_state.VLshadow_2611;
        // D s_1_1: write-var VLshadow#2612 <= s_1_0
        fn_state.VLshadow_2612 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2612:i64
        let s_1_3: i64 = fn_state.VLshadow_2612;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: write-var PL <= s_1_6
        fn_state.PL = s_1_6;
        // D s_1_8: read-var VLshadow#2612:i64
        let s_1_8: i64 = fn_state.VLshadow_2612;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: read-var esizeshadow#2610:i64
        let s_1_10: i64 = fn_state.esizeshadow_2610;
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
        // D s_1_23: read-var esizeshadow#2610:i64
        let s_1_23: i64 = fn_state.esizeshadow_2610;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: read-var mask:bv
        let s_1_25: Bits = fn_state.mask;
        // D s_1_26: call AnyActiveElement(s_1_25, s_1_24)
        let s_1_26: bool = AnyActiveElement(state, tracer, s_1_25, s_1_24);
        // N s_1_27: branch s_1_26 b26 b2
        if s_1_26 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2612:i64
        let s_2_0: i64 = fn_state.VLshadow_2612;
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
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var esizeshadow#2610:i64
        let s_3_1: i64 = fn_state.esizeshadow_2610;
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
        // D s_3_12: write-var gs#187919 <= s_3_11
        fn_state.gs_187919 = s_3_11;
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
        // D s_4_1: read-var gs#187919:i64
        let s_4_1: i64 = fn_state.gs_187919;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b25 b5
        if s_4_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#2610:i64
        let s_5_0: i64 = fn_state.esizeshadow_2610;
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
        // D s_5_8: read-var is_unsigned:u8
        let s_5_8: bool = fn_state.is_unsigned;
        // D s_5_9: call asl_Int(s_5_7, s_5_8)
        let s_5_9: i128 = asl_Int(state, tracer, s_5_7, s_5_8);
        // D s_5_10: write-var element1 <= s_5_9
        fn_state.element1 = s_5_9;
        // D s_5_11: read-var e:i64
        let s_5_11: i64 = fn_state.e;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: read-var esizeshadow#2610:i64
        let s_5_13: i64 = fn_state.esizeshadow_2610;
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_15: read-var mask:bv
        let s_5_15: Bits = fn_state.mask;
        // D s_5_16: call ActivePredicateElement(s_5_15, s_5_12, s_5_14)
        let s_5_16: bool = ActivePredicateElement(state, tracer, s_5_15, s_5_12, s_5_14);
        // N s_5_17: branch s_5_16 b8 b6
        if s_5_16 {
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
        // C s_8_0: const #0u : u32
        let s_8_0: u32 = 0;
        // D s_8_1: read-var op:u32
        let s_8_1: u32 = fn_state.op;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b14 b9
        if s_8_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var element1:i
        let s_9_0: i128 = fn_state.element1;
        // D s_9_1: read-var imm:i
        let s_9_1: i128 = fn_state.imm;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: write-var cond <= s_9_2
        fn_state.cond = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var cond:u8
        let s_10_0: bool = fn_state.cond;
        // N s_10_1: branch s_10_0 b13 b11
        if s_10_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var pbit <= s_11_0
        fn_state.pbit = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var psize:i64
        let s_12_0: i64 = fn_state.psize;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: read-var pbit:u8
        let s_12_3: bool = fn_state.pbit;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 1u16);
        // D s_12_5: read-var psize:i64
        let s_12_5: i64 = fn_state.psize;
        // D s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_7: bits-cast zx s_12_4 -> bv length s_12_6
        let s_12_7: Bits = s_12_4.zero_extend(s_12_6);
        // D s_12_8: read-var e:i64
        let s_12_8: i64 = fn_state.e;
        // D s_12_9: cast zx s_12_8 -> i
        let s_12_9: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_10: cast zx s_12_2 -> i
        let s_12_10: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_11: read-var result:bv
        let s_12_11: Bits = fn_state.result;
        // D s_12_12: call Elem_set(s_12_11, s_12_9, s_12_10, s_12_7)
        let s_12_12: Bits = Elem_set(state, tracer, s_12_11, s_12_9, s_12_10, s_12_7);
        // D s_12_13: write-var result <= s_12_12
        fn_state.result = s_12_12;
        // N s_12_14: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var pbit <= s_13_0
        fn_state.pbit = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u32
        let s_14_0: u32 = 1;
        // D s_14_1: read-var op:u32
        let s_14_1: u32 = fn_state.op;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // D s_14_3: not s_14_2
        let s_14_3: bool = !s_14_2;
        // N s_14_4: branch s_14_3 b16 b15
        if s_14_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var element1:i
        let s_15_0: i128 = fn_state.element1;
        // D s_15_1: read-var imm:i
        let s_15_1: i128 = fn_state.imm;
        // D s_15_2: call neq_int(s_15_0, s_15_1)
        let s_15_2: bool = neq_int(state, tracer, s_15_0, s_15_1);
        // D s_15_3: write-var cond <= s_15_2
        fn_state.cond = s_15_2;
        // N s_15_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #2u : u32
        let s_16_0: u32 = 2;
        // D s_16_1: read-var op:u32
        let s_16_1: u32 = fn_state.op;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b18 b17
        if s_16_3 {
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
        // D s_17_0: read-var element1:i
        let s_17_0: i128 = fn_state.element1;
        // D s_17_1: read-var imm:i
        let s_17_1: i128 = fn_state.imm;
        // D s_17_2: cmp-ge s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) >= (s_17_1));
        // D s_17_3: write-var cond <= s_17_2
        fn_state.cond = s_17_2;
        // N s_17_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #4u : u32
        let s_18_0: u32 = 4;
        // D s_18_1: read-var op:u32
        let s_18_1: u32 = fn_state.op;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: not s_18_2
        let s_18_3: bool = !s_18_2;
        // N s_18_4: branch s_18_3 b20 b19
        if s_18_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var element1:i
        let s_19_0: i128 = fn_state.element1;
        // D s_19_1: read-var imm:i
        let s_19_1: i128 = fn_state.imm;
        // D s_19_2: cmp-lt s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) < (s_19_1));
        // D s_19_3: write-var cond <= s_19_2
        fn_state.cond = s_19_2;
        // N s_19_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #3u : u32
        let s_20_0: u32 = 3;
        // D s_20_1: read-var op:u32
        let s_20_1: u32 = fn_state.op;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // D s_20_3: not s_20_2
        let s_20_3: bool = !s_20_2;
        // N s_20_4: branch s_20_3 b22 b21
        if s_20_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var element1:i
        let s_21_0: i128 = fn_state.element1;
        // D s_21_1: read-var imm:i
        let s_21_1: i128 = fn_state.imm;
        // D s_21_2: cmp-gt s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) > (s_21_1));
        // D s_21_3: write-var cond <= s_21_2
        fn_state.cond = s_21_2;
        // N s_21_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #5u : u32
        let s_22_0: u32 = 5;
        // D s_22_1: read-var op:u32
        let s_22_1: u32 = fn_state.op;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // D s_22_3: not s_22_2
        let s_22_3: bool = !s_22_2;
        // N s_22_4: branch s_22_3 b24 b23
        if s_22_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var element1:i
        let s_23_0: i128 = fn_state.element1;
        // D s_23_1: read-var imm:i
        let s_23_1: i128 = fn_state.imm;
        // D s_23_2: cmp-le s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) <= (s_23_1));
        // D s_23_3: write-var cond <= s_23_2
        fn_state.cond = s_23_2;
        // N s_23_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var esizeshadow#2610:i64
        let s_25_0: i64 = fn_state.esizeshadow_2610;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: read-var mask:bv
        let s_25_2: Bits = fn_state.mask;
        // D s_25_3: read-var result:bv
        let s_25_3: Bits = fn_state.result;
        // D s_25_4: call PredTest(s_25_2, s_25_3, s_25_1)
        let s_25_4: u8 = PredTest(state, tracer, s_25_2, s_25_3, s_25_1);
        // C s_25_5: const #3s : i
        let s_25_5: i128 = 3;
        // D s_25_6: cast zx s_25_4 -> bv
        let s_25_6: Bits = Bits::new(s_25_4 as u128, 4u16);
        // C s_25_7: const #1s : i64
        let s_25_7: i64 = 1;
        // C s_25_8: cast zx s_25_7 -> i
        let s_25_8: i128 = (i128::try_from(s_25_7).unwrap());
        // C s_25_9: const #0s : i
        let s_25_9: i128 = 0;
        // C s_25_10: add s_25_9 s_25_8
        let s_25_10: i128 = (s_25_9 + s_25_8);
        // D s_25_11: bit-extract s_25_6 s_25_5 s_25_10
        let s_25_11: Bits = (Bits::new(
            ((s_25_6) >> (s_25_5)).value(),
            u16::try_from(s_25_10).unwrap(),
        ));
        // D s_25_12: cast reint s_25_11 -> u8
        let s_25_12: bool = ((s_25_11.value()) != 0);
        // C s_25_13: const #16984u : u32
        let s_25_13: u32 = 16984;
        // N s_25_14: write-reg s_25_13 <= s_25_12
        let s_25_14: () = {
            state.write_register::<bool>(s_25_13 as isize, s_25_12);
            tracer.write_register(s_25_13 as isize, s_25_12);
        };
        // C s_25_15: const #2s : i
        let s_25_15: i128 = 2;
        // D s_25_16: cast zx s_25_4 -> bv
        let s_25_16: Bits = Bits::new(s_25_4 as u128, 4u16);
        // C s_25_17: const #1s : i64
        let s_25_17: i64 = 1;
        // C s_25_18: cast zx s_25_17 -> i
        let s_25_18: i128 = (i128::try_from(s_25_17).unwrap());
        // C s_25_19: const #0s : i
        let s_25_19: i128 = 0;
        // C s_25_20: add s_25_19 s_25_18
        let s_25_20: i128 = (s_25_19 + s_25_18);
        // D s_25_21: bit-extract s_25_16 s_25_15 s_25_20
        let s_25_21: Bits = (Bits::new(
            ((s_25_16) >> (s_25_15)).value(),
            u16::try_from(s_25_20).unwrap(),
        ));
        // D s_25_22: cast reint s_25_21 -> u8
        let s_25_22: bool = ((s_25_21.value()) != 0);
        // C s_25_23: const #16997u : u32
        let s_25_23: u32 = 16997;
        // N s_25_24: write-reg s_25_23 <= s_25_22
        let s_25_24: () = {
            state.write_register::<bool>(s_25_23 as isize, s_25_22);
            tracer.write_register(s_25_23 as isize, s_25_22);
        };
        // C s_25_25: const #1s : i
        let s_25_25: i128 = 1;
        // D s_25_26: cast zx s_25_4 -> bv
        let s_25_26: Bits = Bits::new(s_25_4 as u128, 4u16);
        // C s_25_27: const #1s : i64
        let s_25_27: i64 = 1;
        // C s_25_28: cast zx s_25_27 -> i
        let s_25_28: i128 = (i128::try_from(s_25_27).unwrap());
        // C s_25_29: const #0s : i
        let s_25_29: i128 = 0;
        // C s_25_30: add s_25_29 s_25_28
        let s_25_30: i128 = (s_25_29 + s_25_28);
        // D s_25_31: bit-extract s_25_26 s_25_25 s_25_30
        let s_25_31: Bits = (Bits::new(
            ((s_25_26) >> (s_25_25)).value(),
            u16::try_from(s_25_30).unwrap(),
        ));
        // D s_25_32: cast reint s_25_31 -> u8
        let s_25_32: bool = ((s_25_31.value()) != 0);
        // C s_25_33: const #16971u : u32
        let s_25_33: u32 = 16971;
        // N s_25_34: write-reg s_25_33 <= s_25_32
        let s_25_34: () = {
            state.write_register::<bool>(s_25_33 as isize, s_25_32);
            tracer.write_register(s_25_33 as isize, s_25_32);
        };
        // C s_25_35: const #0s : i
        let s_25_35: i128 = 0;
        // D s_25_36: cast zx s_25_4 -> bv
        let s_25_36: Bits = Bits::new(s_25_4 as u128, 4u16);
        // C s_25_37: const #1s : i64
        let s_25_37: i64 = 1;
        // C s_25_38: cast zx s_25_37 -> i
        let s_25_38: i128 = (i128::try_from(s_25_37).unwrap());
        // C s_25_39: const #0s : i
        let s_25_39: i128 = 0;
        // C s_25_40: add s_25_39 s_25_38
        let s_25_40: i128 = (s_25_39 + s_25_38);
        // D s_25_41: bit-extract s_25_36 s_25_35 s_25_40
        let s_25_41: Bits = (Bits::new(
            ((s_25_36) >> (s_25_35)).value(),
            u16::try_from(s_25_40).unwrap(),
        ));
        // D s_25_42: cast reint s_25_41 -> u8
        let s_25_42: bool = ((s_25_41.value()) != 0);
        // C s_25_43: const #16996u : u32
        let s_25_43: u32 = 16996;
        // N s_25_44: write-reg s_25_43 <= s_25_42
        let s_25_44: () = {
            state.write_register::<bool>(s_25_43 as isize, s_25_42);
            tracer.write_register(s_25_43 as isize, s_25_42);
        };
        // D s_25_45: read-var PL:i64
        let s_25_45: i64 = fn_state.PL;
        // D s_25_46: cast zx s_25_45 -> i
        let s_25_46: i128 = (i128::try_from(s_25_45).unwrap());
        // D s_25_47: cast reint s_25_46 -> i64
        let s_25_47: i64 = (s_25_46 as i64);
        // D s_25_48: read-var d:i64
        let s_25_48: i64 = fn_state.d;
        // D s_25_49: cast zx s_25_48 -> i
        let s_25_49: i128 = (i128::try_from(s_25_48).unwrap());
        // D s_25_50: cast zx s_25_47 -> i
        let s_25_50: i128 = (i128::try_from(s_25_47).unwrap());
        // D s_25_51: read-var result:bv
        let s_25_51: Bits = fn_state.result;
        // D s_25_52: call P_set(s_25_49, s_25_50, s_25_51)
        let s_25_52: () = P_set(state, tracer, s_25_49, s_25_50, s_25_51);
        // N s_25_53: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var VLshadow#2612:i64
        let s_26_0: i64 = fn_state.VLshadow_2612;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: cast reint s_26_1 -> i64
        let s_26_2: i64 = (s_26_1 as i64);
        // D s_26_3: read-var n:i64
        let s_26_3: i64 = fn_state.n;
        // D s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_5: cast zx s_26_2 -> i
        let s_26_5: i128 = (i128::try_from(s_26_2).unwrap());
        // D s_26_6: call Z_read(s_26_4, s_26_5)
        let s_26_6: Bits = Z_read(state, tracer, s_26_4, s_26_5);
        // D s_26_7: write-var operand1 <= s_26_6
        fn_state.operand1 = s_26_6;
        // N s_26_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
