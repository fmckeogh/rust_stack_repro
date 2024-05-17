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
pub fn execute_CMPGE_P_P_ZW__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    m: i64,
    n: i64,
    op: u32,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_189133: i64,
        e: i64,
        esizeshadow_2658: i64,
        VLshadow_2659: i64,
        PL: i64,
        mask: Bits,
        element2: i128,
        VLshadow_2660: i64,
        element1: i128,
        pbit: bool,
        psize: i64,
        elements: i64,
        result: Bits,
        operand1: Bits,
        cond: bool,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        m: i64,
        n: i64,
        op: u32,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
        m,
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
        // D s_0_3: write-var esizeshadow#2658 <= s_0_2
        fn_state.esizeshadow_2658 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2659 <= s_0_6
        fn_state.VLshadow_2659 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2659:i64
        let s_1_0: i64 = fn_state.VLshadow_2659;
        // D s_1_1: write-var VLshadow#2660 <= s_1_0
        fn_state.VLshadow_2660 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2660:i64
        let s_1_3: i64 = fn_state.VLshadow_2660;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: write-var PL <= s_1_6
        fn_state.PL = s_1_6;
        // D s_1_8: read-var VLshadow#2660:i64
        let s_1_8: i64 = fn_state.VLshadow_2660;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: read-var esizeshadow#2658:i64
        let s_1_10: i64 = fn_state.esizeshadow_2658;
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
        // D s_1_23: read-var esizeshadow#2658:i64
        let s_1_23: i64 = fn_state.esizeshadow_2658;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: read-var mask:bv
        let s_1_25: Bits = fn_state.mask;
        // D s_1_26: call AnyActiveElement(s_1_25, s_1_24)
        let s_1_26: bool = AnyActiveElement(state, tracer, s_1_25, s_1_24);
        // N s_1_27: branch s_1_26 b29 b2
        if s_1_26 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2660:i64
        let s_2_0: i64 = fn_state.VLshadow_2660;
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
        // D s_3_0: read-var esizeshadow#2658:i64
        let s_3_0: i64 = fn_state.esizeshadow_2658;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var mask:bv
        let s_3_2: Bits = fn_state.mask;
        // D s_3_3: call AnyActiveElement(s_3_2, s_3_1)
        let s_3_3: bool = AnyActiveElement(state, tracer, s_3_2, s_3_1);
        // N s_3_4: branch s_3_3 b28 b4
        if s_3_3 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2660:i64
        let s_4_0: i64 = fn_state.VLshadow_2660;
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
        // D s_5_1: read-var esizeshadow#2658:i64
        let s_5_1: i64 = fn_state.esizeshadow_2658;
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
        // D s_5_12: write-var gs#189133 <= s_5_11
        fn_state.gs_189133 = s_5_11;
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
        // D s_6_1: read-var gs#189133:i64
        let s_6_1: i64 = fn_state.gs_189133;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b27 b7
        if s_6_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esizeshadow#2658:i64
        let s_7_0: i64 = fn_state.esizeshadow_2658;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var e:i64
        let s_7_3: i64 = fn_state.e;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast zx s_7_2 -> i
        let s_7_5: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_6: read-var operand1:bv
        let s_7_6: Bits = fn_state.operand1;
        // D s_7_7: call Elem_read(s_7_6, s_7_4, s_7_5)
        let s_7_7: Bits = Elem_read(state, tracer, s_7_6, s_7_4, s_7_5);
        // D s_7_8: read-var is_unsigned:u8
        let s_7_8: bool = fn_state.is_unsigned;
        // D s_7_9: call asl_Int(s_7_7, s_7_8)
        let s_7_9: i128 = asl_Int(state, tracer, s_7_7, s_7_8);
        // D s_7_10: write-var element1 <= s_7_9
        fn_state.element1 = s_7_9;
        // D s_7_11: read-var e:i64
        let s_7_11: i64 = fn_state.e;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: read-var esizeshadow#2658:i64
        let s_7_13: i64 = fn_state.esizeshadow_2658;
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_15: read-var mask:bv
        let s_7_15: Bits = fn_state.mask;
        // D s_7_16: call ActivePredicateElement(s_7_15, s_7_12, s_7_14)
        let s_7_16: bool = ActivePredicateElement(state, tracer, s_7_15, s_7_12, s_7_14);
        // N s_7_17: branch s_7_16 b10 b8
        if s_7_16 {
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
        // D s_10_0: read-var e:i64
        let s_10_0: i64 = fn_state.e;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var esizeshadow#2658:i64
        let s_10_2: i64 = fn_state.esizeshadow_2658;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: mul s_10_1 s_10_3
        let s_10_4: i128 = ((s_10_1) * (s_10_3));
        // D s_10_5: cast reint s_10_4 -> i64
        let s_10_5: i64 = (s_10_4 as i64);
        // C s_10_6: const #64s : i
        let s_10_6: i128 = 64;
        // D s_10_7: cast zx s_10_5 -> i
        let s_10_7: i128 = (i128::try_from(s_10_5).unwrap());
        // D s_10_8: div s_10_7 s_10_6
        let s_10_8: i128 = ((s_10_7) / (s_10_6));
        // D s_10_9: cast reint s_10_8 -> i64
        let s_10_9: i64 = (s_10_8 as i64);
        // C s_10_10: const #64s : i64
        let s_10_10: i64 = 64;
        // D s_10_11: cast zx s_10_9 -> i
        let s_10_11: i128 = (i128::try_from(s_10_9).unwrap());
        // C s_10_12: cast zx s_10_10 -> i
        let s_10_12: i128 = (i128::try_from(s_10_10).unwrap());
        // D s_10_13: read-var operand2:bv
        let s_10_13: Bits = fn_state.operand2;
        // D s_10_14: call Elem_read(s_10_13, s_10_11, s_10_12)
        let s_10_14: Bits = Elem_read(state, tracer, s_10_13, s_10_11, s_10_12);
        // D s_10_15: cast reint s_10_14 -> u64
        let s_10_15: u64 = (s_10_14.value() as u64);
        // D s_10_16: cast zx s_10_15 -> bv
        let s_10_16: Bits = Bits::new(s_10_15 as u128, 64u16);
        // D s_10_17: read-var is_unsigned:u8
        let s_10_17: bool = fn_state.is_unsigned;
        // D s_10_18: call asl_Int(s_10_16, s_10_17)
        let s_10_18: i128 = asl_Int(state, tracer, s_10_16, s_10_17);
        // D s_10_19: write-var element2 <= s_10_18
        fn_state.element2 = s_10_18;
        // C s_10_20: const #0u : u32
        let s_10_20: u32 = 0;
        // D s_10_21: read-var op:u32
        let s_10_21: u32 = fn_state.op;
        // D s_10_22: cmp-eq s_10_20 s_10_21
        let s_10_22: bool = ((s_10_20) == (s_10_21));
        // D s_10_23: not s_10_22
        let s_10_23: bool = !s_10_22;
        // N s_10_24: branch s_10_23 b16 b11
        if s_10_23 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var element1:i
        let s_11_0: i128 = fn_state.element1;
        // D s_11_1: read-var element2:i
        let s_11_1: i128 = fn_state.element2;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: write-var cond <= s_11_2
        fn_state.cond = s_11_2;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var cond:u8
        let s_12_0: bool = fn_state.cond;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var pbit <= s_13_0
        fn_state.pbit = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var psize:i64
        let s_14_0: i64 = fn_state.psize;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: cast reint s_14_1 -> i64
        let s_14_2: i64 = (s_14_1 as i64);
        // D s_14_3: read-var pbit:u8
        let s_14_3: bool = fn_state.pbit;
        // D s_14_4: cast zx s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 1u16);
        // D s_14_5: read-var psize:i64
        let s_14_5: i64 = fn_state.psize;
        // D s_14_6: cast zx s_14_5 -> i
        let s_14_6: i128 = (i128::try_from(s_14_5).unwrap());
        // D s_14_7: bits-cast zx s_14_4 -> bv length s_14_6
        let s_14_7: Bits = s_14_4.zero_extend(s_14_6);
        // D s_14_8: read-var e:i64
        let s_14_8: i64 = fn_state.e;
        // D s_14_9: cast zx s_14_8 -> i
        let s_14_9: i128 = (i128::try_from(s_14_8).unwrap());
        // D s_14_10: cast zx s_14_2 -> i
        let s_14_10: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_11: read-var result:bv
        let s_14_11: Bits = fn_state.result;
        // D s_14_12: call Elem_set(s_14_11, s_14_9, s_14_10, s_14_7)
        let s_14_12: Bits = Elem_set(state, tracer, s_14_11, s_14_9, s_14_10, s_14_7);
        // D s_14_13: write-var result <= s_14_12
        fn_state.result = s_14_12;
        // N s_14_14: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var pbit <= s_15_0
        fn_state.pbit = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u32
        let s_16_0: u32 = 1;
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
        // D s_17_1: read-var element2:i
        let s_17_1: i128 = fn_state.element2;
        // D s_17_2: call neq_int(s_17_0, s_17_1)
        let s_17_2: bool = neq_int(state, tracer, s_17_0, s_17_1);
        // D s_17_3: write-var cond <= s_17_2
        fn_state.cond = s_17_2;
        // N s_17_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #2u : u32
        let s_18_0: u32 = 2;
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
        // D s_19_1: read-var element2:i
        let s_19_1: i128 = fn_state.element2;
        // D s_19_2: cmp-ge s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) >= (s_19_1));
        // D s_19_3: write-var cond <= s_19_2
        fn_state.cond = s_19_2;
        // N s_19_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #4u : u32
        let s_20_0: u32 = 4;
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
        // D s_21_1: read-var element2:i
        let s_21_1: i128 = fn_state.element2;
        // D s_21_2: cmp-lt s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) < (s_21_1));
        // D s_21_3: write-var cond <= s_21_2
        fn_state.cond = s_21_2;
        // N s_21_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #3u : u32
        let s_22_0: u32 = 3;
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
        // D s_23_1: read-var element2:i
        let s_23_1: i128 = fn_state.element2;
        // D s_23_2: cmp-gt s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) > (s_23_1));
        // D s_23_3: write-var cond <= s_23_2
        fn_state.cond = s_23_2;
        // N s_23_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #5u : u32
        let s_24_0: u32 = 5;
        // D s_24_1: read-var op:u32
        let s_24_1: u32 = fn_state.op;
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
        // D s_24_3: not s_24_2
        let s_24_3: bool = !s_24_2;
        // N s_24_4: branch s_24_3 b26 b25
        if s_24_3 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var element1:i
        let s_25_0: i128 = fn_state.element1;
        // D s_25_1: read-var element2:i
        let s_25_1: i128 = fn_state.element2;
        // D s_25_2: cmp-le s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) <= (s_25_1));
        // D s_25_3: write-var cond <= s_25_2
        fn_state.cond = s_25_2;
        // N s_25_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var esizeshadow#2658:i64
        let s_27_0: i64 = fn_state.esizeshadow_2658;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: read-var mask:bv
        let s_27_2: Bits = fn_state.mask;
        // D s_27_3: read-var result:bv
        let s_27_3: Bits = fn_state.result;
        // D s_27_4: call PredTest(s_27_2, s_27_3, s_27_1)
        let s_27_4: u8 = PredTest(state, tracer, s_27_2, s_27_3, s_27_1);
        // C s_27_5: const #3s : i
        let s_27_5: i128 = 3;
        // D s_27_6: cast zx s_27_4 -> bv
        let s_27_6: Bits = Bits::new(s_27_4 as u128, 4u16);
        // C s_27_7: const #1s : i64
        let s_27_7: i64 = 1;
        // C s_27_8: cast zx s_27_7 -> i
        let s_27_8: i128 = (i128::try_from(s_27_7).unwrap());
        // C s_27_9: const #0s : i
        let s_27_9: i128 = 0;
        // C s_27_10: add s_27_9 s_27_8
        let s_27_10: i128 = (s_27_9 + s_27_8);
        // D s_27_11: bit-extract s_27_6 s_27_5 s_27_10
        let s_27_11: Bits = (Bits::new(
            ((s_27_6) >> (s_27_5)).value(),
            u16::try_from(s_27_10).unwrap(),
        ));
        // D s_27_12: cast reint s_27_11 -> u8
        let s_27_12: bool = ((s_27_11.value()) != 0);
        // C s_27_13: const #16984u : u32
        let s_27_13: u32 = 16984;
        // N s_27_14: write-reg s_27_13 <= s_27_12
        let s_27_14: () = {
            state.write_register::<bool>(s_27_13 as isize, s_27_12);
            tracer.write_register(s_27_13 as isize, s_27_12);
        };
        // C s_27_15: const #2s : i
        let s_27_15: i128 = 2;
        // D s_27_16: cast zx s_27_4 -> bv
        let s_27_16: Bits = Bits::new(s_27_4 as u128, 4u16);
        // C s_27_17: const #1s : i64
        let s_27_17: i64 = 1;
        // C s_27_18: cast zx s_27_17 -> i
        let s_27_18: i128 = (i128::try_from(s_27_17).unwrap());
        // C s_27_19: const #0s : i
        let s_27_19: i128 = 0;
        // C s_27_20: add s_27_19 s_27_18
        let s_27_20: i128 = (s_27_19 + s_27_18);
        // D s_27_21: bit-extract s_27_16 s_27_15 s_27_20
        let s_27_21: Bits = (Bits::new(
            ((s_27_16) >> (s_27_15)).value(),
            u16::try_from(s_27_20).unwrap(),
        ));
        // D s_27_22: cast reint s_27_21 -> u8
        let s_27_22: bool = ((s_27_21.value()) != 0);
        // C s_27_23: const #16997u : u32
        let s_27_23: u32 = 16997;
        // N s_27_24: write-reg s_27_23 <= s_27_22
        let s_27_24: () = {
            state.write_register::<bool>(s_27_23 as isize, s_27_22);
            tracer.write_register(s_27_23 as isize, s_27_22);
        };
        // C s_27_25: const #1s : i
        let s_27_25: i128 = 1;
        // D s_27_26: cast zx s_27_4 -> bv
        let s_27_26: Bits = Bits::new(s_27_4 as u128, 4u16);
        // C s_27_27: const #1s : i64
        let s_27_27: i64 = 1;
        // C s_27_28: cast zx s_27_27 -> i
        let s_27_28: i128 = (i128::try_from(s_27_27).unwrap());
        // C s_27_29: const #0s : i
        let s_27_29: i128 = 0;
        // C s_27_30: add s_27_29 s_27_28
        let s_27_30: i128 = (s_27_29 + s_27_28);
        // D s_27_31: bit-extract s_27_26 s_27_25 s_27_30
        let s_27_31: Bits = (Bits::new(
            ((s_27_26) >> (s_27_25)).value(),
            u16::try_from(s_27_30).unwrap(),
        ));
        // D s_27_32: cast reint s_27_31 -> u8
        let s_27_32: bool = ((s_27_31.value()) != 0);
        // C s_27_33: const #16971u : u32
        let s_27_33: u32 = 16971;
        // N s_27_34: write-reg s_27_33 <= s_27_32
        let s_27_34: () = {
            state.write_register::<bool>(s_27_33 as isize, s_27_32);
            tracer.write_register(s_27_33 as isize, s_27_32);
        };
        // C s_27_35: const #0s : i
        let s_27_35: i128 = 0;
        // D s_27_36: cast zx s_27_4 -> bv
        let s_27_36: Bits = Bits::new(s_27_4 as u128, 4u16);
        // C s_27_37: const #1s : i64
        let s_27_37: i64 = 1;
        // C s_27_38: cast zx s_27_37 -> i
        let s_27_38: i128 = (i128::try_from(s_27_37).unwrap());
        // C s_27_39: const #0s : i
        let s_27_39: i128 = 0;
        // C s_27_40: add s_27_39 s_27_38
        let s_27_40: i128 = (s_27_39 + s_27_38);
        // D s_27_41: bit-extract s_27_36 s_27_35 s_27_40
        let s_27_41: Bits = (Bits::new(
            ((s_27_36) >> (s_27_35)).value(),
            u16::try_from(s_27_40).unwrap(),
        ));
        // D s_27_42: cast reint s_27_41 -> u8
        let s_27_42: bool = ((s_27_41.value()) != 0);
        // C s_27_43: const #16996u : u32
        let s_27_43: u32 = 16996;
        // N s_27_44: write-reg s_27_43 <= s_27_42
        let s_27_44: () = {
            state.write_register::<bool>(s_27_43 as isize, s_27_42);
            tracer.write_register(s_27_43 as isize, s_27_42);
        };
        // D s_27_45: read-var PL:i64
        let s_27_45: i64 = fn_state.PL;
        // D s_27_46: cast zx s_27_45 -> i
        let s_27_46: i128 = (i128::try_from(s_27_45).unwrap());
        // D s_27_47: cast reint s_27_46 -> i64
        let s_27_47: i64 = (s_27_46 as i64);
        // D s_27_48: read-var d:i64
        let s_27_48: i64 = fn_state.d;
        // D s_27_49: cast zx s_27_48 -> i
        let s_27_49: i128 = (i128::try_from(s_27_48).unwrap());
        // D s_27_50: cast zx s_27_47 -> i
        let s_27_50: i128 = (i128::try_from(s_27_47).unwrap());
        // D s_27_51: read-var result:bv
        let s_27_51: Bits = fn_state.result;
        // D s_27_52: call P_set(s_27_49, s_27_50, s_27_51)
        let s_27_52: () = P_set(state, tracer, s_27_49, s_27_50, s_27_51);
        // N s_27_53: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var VLshadow#2660:i64
        let s_28_0: i64 = fn_state.VLshadow_2660;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: cast reint s_28_1 -> i64
        let s_28_2: i64 = (s_28_1 as i64);
        // D s_28_3: read-var m:i64
        let s_28_3: i64 = fn_state.m;
        // D s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // D s_28_5: cast zx s_28_2 -> i
        let s_28_5: i128 = (i128::try_from(s_28_2).unwrap());
        // D s_28_6: call Z_read(s_28_4, s_28_5)
        let s_28_6: Bits = Z_read(state, tracer, s_28_4, s_28_5);
        // D s_28_7: write-var operand2 <= s_28_6
        fn_state.operand2 = s_28_6;
        // N s_28_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var VLshadow#2660:i64
        let s_29_0: i64 = fn_state.VLshadow_2660;
        // D s_29_1: cast zx s_29_0 -> i
        let s_29_1: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_2: cast reint s_29_1 -> i64
        let s_29_2: i64 = (s_29_1 as i64);
        // D s_29_3: read-var n:i64
        let s_29_3: i64 = fn_state.n;
        // D s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // D s_29_5: cast zx s_29_2 -> i
        let s_29_5: i128 = (i128::try_from(s_29_2).unwrap());
        // D s_29_6: call Z_read(s_29_4, s_29_5)
        let s_29_6: Bits = Z_read(state, tracer, s_29_4, s_29_5);
        // D s_29_7: write-var operand1 <= s_29_6
        fn_state.operand1 = s_29_6;
        // N s_29_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
