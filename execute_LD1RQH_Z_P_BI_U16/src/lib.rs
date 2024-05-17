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
use CreateAccDescSVE::*;
use CheckSVEEnabled::*;
use ConstrainUnpredictableBool::*;
use X_read::*;
use Zeros::*;
use neq_int::*;
use Elem_set::*;
use SP_read::*;
use Mem_read::*;
use AnyActiveElement::*;
use P_read::*;
use ActivePredicateElement::*;
use replicate_bits_borealis_internal::*;
use CheckSPAlignment::*;
use Z_set::*;
use common::*;
pub fn execute_LD1RQH_Z_P_BI_U16<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    n: i64,
    offset: i128,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_308636: Bits,
        e: i64,
        base: u64,
        VLshadow_4825: i64,
        mbytes: i64,
        VLshadow_4826: i64,
        mask: Bits,
        gs_236615: i64,
        accdesc: ProductType9878976b5bcce9c9,
        elements: i64,
        gs_236627: bool,
        result: u128,
        ga_308635: i64,
        VL: i64,
        esize: i64,
        g: i64,
        n: i64,
        offset: i128,
        t: i64,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        n,
        offset,
        t,
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
        // D s_0_3: write-var VLshadow#4825 <= s_0_2
        fn_state.VLshadow_4825 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckSVEEnabled(s_0_4)
        let s_0_5: () = CheckSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4825:i64
        let s_1_0: i64 = fn_state.VLshadow_4825;
        // D s_1_1: write-var VLshadow#4826 <= s_1_0
        fn_state.VLshadow_4826 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4826:i64
        let s_1_3: i64 = fn_state.VLshadow_4826;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #128s : i
        let s_1_7: i128 = 128;
        // D s_1_8: read-var esize:i64
        let s_1_8: i64 = fn_state.esize;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: div s_1_7 s_1_9
        let s_1_10: i128 = ((s_1_7) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var elements <= s_1_11
        fn_state.elements = s_1_11;
        // D s_1_13: cast zx s_1_6 -> i
        let s_1_13: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var g:i64
        let s_1_15: i64 = fn_state.g;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call P_read(s_1_16, s_1_17)
        let s_1_18: Bits = P_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var mask <= s_1_18
        fn_state.mask = s_1_18;
        // C s_1_20: const #8s : i
        let s_1_20: i128 = 8;
        // D s_1_21: read-var esize:i64
        let s_1_21: i64 = fn_state.esize;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: div s_1_22 s_1_20
        let s_1_23: i128 = ((s_1_22) / (s_1_20));
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: write-var mbytes <= s_1_24
        fn_state.mbytes = s_1_24;
        // C s_1_26: const #31s : i
        let s_1_26: i128 = 31;
        // D s_1_27: read-var n:i64
        let s_1_27: i64 = fn_state.n;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: call neq_int(s_1_28, s_1_26)
        let s_1_29: bool = neq_int(state, tracer, s_1_28, s_1_26);
        // C s_1_30: const #0u : u32
        let s_1_30: u32 = 0;
        // C s_1_31: const #0u : u8
        let s_1_31: bool = false;
        // C s_1_32: const #1u : u8
        let s_1_32: bool = true;
        // D s_1_33: call CreateAccDescSVE(s_1_30, s_1_31, s_1_32, s_1_29)
        let s_1_33: ProductType9878976b5bcce9c9 = CreateAccDescSVE(
            state,
            tracer,
            s_1_30,
            s_1_31,
            s_1_32,
            s_1_29,
        );
        // D s_1_34: write-var accdesc <= s_1_33
        fn_state.accdesc = s_1_33;
        // D s_1_35: read-var esize:i64
        let s_1_35: i64 = fn_state.esize;
        // D s_1_36: cast zx s_1_35 -> i
        let s_1_36: i128 = (i128::try_from(s_1_35).unwrap());
        // D s_1_37: read-var mask:bv
        let s_1_37: Bits = fn_state.mask;
        // D s_1_38: call AnyActiveElement(s_1_37, s_1_36)
        let s_1_38: bool = AnyActiveElement(state, tracer, s_1_37, s_1_36);
        // D s_1_39: not s_1_38
        let s_1_39: bool = !s_1_38;
        // N s_1_40: branch s_1_39 b17 b2
        if s_1_39 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #31s : i
        let s_2_0: i128 = 31;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_0
        let s_2_3: bool = ((s_2_2) == (s_2_0));
        // N s_2_4: branch s_2_3 b16 b3
        if s_2_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #31s : i
        let s_4_0: i128 = 31;
        // D s_4_1: read-var n:i64
        let s_4_1: i64 = fn_state.n;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // N s_4_4: branch s_4_3 b15 b5
        if s_4_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: call X_read(s_5_2, s_5_0)
        let s_5_3: Bits = X_read(state, tracer, s_5_2, s_5_0);
        // D s_5_4: cast reint s_5_3 -> u64
        let s_5_4: u64 = (s_5_3.value() as u64);
        // D s_5_5: write-var base <= s_5_4
        fn_state.base = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i64
        let s_7_0: i64 = 0;
        // C s_7_1: const #1s : i
        let s_7_1: i128 = 1;
        // D s_7_2: read-var elements:i64
        let s_7_2: i64 = fn_state.elements;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: sub s_7_3 s_7_1
        let s_7_4: i128 = ((s_7_3) - (s_7_1));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: write-var gs#236615 <= s_7_5
        fn_state.gs_236615 = s_7_5;
        // D s_7_7: write-var e <= s_7_0
        fn_state.e = s_7_0;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var e:i64
        let s_8_0: i64 = fn_state.e;
        // D s_8_1: read-var gs#236615:i64
        let s_8_1: i64 = fn_state.gs_236615;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b14 b9
        if s_8_2 {
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
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var esize:i64
        let s_9_2: i64 = fn_state.esize;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var mask:bv
        let s_9_4: Bits = fn_state.mask;
        // D s_9_5: call ActivePredicateElement(s_9_4, s_9_1, s_9_3)
        let s_9_5: bool = ActivePredicateElement(state, tracer, s_9_4, s_9_1, s_9_3);
        // N s_9_6: branch s_9_5 b12 b10
        if s_9_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esize:i64
        let s_10_0: i64 = fn_state.esize;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var esize:i64
        let s_10_3: i64 = fn_state.esize;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: call Zeros(s_10_4)
        let s_10_5: Bits = Zeros(state, tracer, s_10_4);
        // D s_10_6: cast reint s_10_5 -> u16
        let s_10_6: u16 = (s_10_5.value() as u16);
        // D s_10_7: read-var result:u128
        let s_10_7: u128 = fn_state.result;
        // D s_10_8: cast zx s_10_7 -> bv
        let s_10_8: Bits = Bits::new(s_10_7 as u128, 128u16);
        // D s_10_9: read-var e:i64
        let s_10_9: i64 = fn_state.e;
        // D s_10_10: cast zx s_10_9 -> i
        let s_10_10: i128 = (i128::try_from(s_10_9).unwrap());
        // D s_10_11: cast zx s_10_2 -> i
        let s_10_11: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_12: cast zx s_10_6 -> bv
        let s_10_12: Bits = Bits::new(s_10_6 as u128, 16u16);
        // D s_10_13: call Elem_set(s_10_8, s_10_10, s_10_11, s_10_12)
        let s_10_13: Bits = Elem_set(state, tracer, s_10_8, s_10_10, s_10_11, s_10_12);
        // D s_10_14: cast reint s_10_13 -> u128
        let s_10_14: u128 = (s_10_13.value() as u128);
        // D s_10_15: write-var result <= s_10_14
        fn_state.result = s_10_14;
        // N s_10_16: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var e:i64
        let s_11_0: i64 = fn_state.e;
        // C s_11_1: const #1s : i64
        let s_11_1: i64 = 1;
        // D s_11_2: add s_11_0 s_11_1
        let s_11_2: i64 = (s_11_0 + s_11_1);
        // D s_11_3: write-var e <= s_11_2
        fn_state.e = s_11_2;
        // N s_11_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #16s : i
        let s_12_0: i128 = 16;
        // D s_12_1: read-var offset:i
        let s_12_1: i128 = fn_state.offset;
        // D s_12_2: mul s_12_1 s_12_0
        let s_12_2: i128 = ((s_12_1) * (s_12_0));
        // D s_12_3: read-var base:u64
        let s_12_3: u64 = fn_state.base;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 64u16);
        // D s_12_5: cast cvt s_12_2 -> bv
        let s_12_5: Bits = Bits::new(s_12_2 as u128, 128);
        // D s_12_6: add s_12_4 s_12_5
        let s_12_6: Bits = (s_12_4 + s_12_5);
        // D s_12_7: cast reint s_12_6 -> u64
        let s_12_7: u64 = (s_12_6.value() as u64);
        // D s_12_8: read-var e:i64
        let s_12_8: i64 = fn_state.e;
        // D s_12_9: cast zx s_12_8 -> i
        let s_12_9: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_10: read-var mbytes:i64
        let s_12_10: i64 = fn_state.mbytes;
        // D s_12_11: cast zx s_12_10 -> i
        let s_12_11: i128 = (i128::try_from(s_12_10).unwrap());
        // D s_12_12: mul s_12_9 s_12_11
        let s_12_12: i128 = ((s_12_9) * (s_12_11));
        // D s_12_13: cast reint s_12_12 -> i64
        let s_12_13: i64 = (s_12_12 as i64);
        // D s_12_14: cast zx s_12_7 -> bv
        let s_12_14: Bits = Bits::new(s_12_7 as u128, 64u16);
        // D s_12_15: cast zx s_12_13 -> i
        let s_12_15: i128 = (i128::try_from(s_12_13).unwrap());
        // D s_12_16: cast cvt s_12_15 -> bv
        let s_12_16: Bits = Bits::new(s_12_15 as u128, 128);
        // D s_12_17: add s_12_14 s_12_16
        let s_12_17: Bits = (s_12_14 + s_12_16);
        // D s_12_18: cast reint s_12_17 -> u64
        let s_12_18: u64 = (s_12_17.value() as u64);
        // D s_12_19: read-var esize:i64
        let s_12_19: i64 = fn_state.esize;
        // D s_12_20: cast zx s_12_19 -> i
        let s_12_20: i128 = (i128::try_from(s_12_19).unwrap());
        // D s_12_21: cast reint s_12_20 -> i64
        let s_12_21: i64 = (s_12_20 as i64);
        // D s_12_22: write-var ga#308635 <= s_12_21
        fn_state.ga_308635 = s_12_21;
        // D s_12_23: read-var mbytes:i64
        let s_12_23: i64 = fn_state.mbytes;
        // D s_12_24: cast zx s_12_23 -> i
        let s_12_24: i128 = (i128::try_from(s_12_23).unwrap());
        // D s_12_25: read-var accdesc:struct
        let s_12_25: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_12_26: call Mem_read(s_12_18, s_12_24, s_12_25)
        let s_12_26: Bits = Mem_read(state, tracer, s_12_18, s_12_24, s_12_25);
        // D s_12_27: write-var ga#308636 <= s_12_26
        fn_state.ga_308636 = s_12_26;
        // N s_12_28: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var result:u128
        let s_13_0: u128 = fn_state.result;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 128u16);
        // D s_13_2: read-var e:i64
        let s_13_2: i64 = fn_state.e;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: read-var ga#308635:i64
        let s_13_4: i64 = fn_state.ga_308635;
        // D s_13_5: cast zx s_13_4 -> i
        let s_13_5: i128 = (i128::try_from(s_13_4).unwrap());
        // D s_13_6: read-var ga#308636:bv
        let s_13_6: Bits = fn_state.ga_308636;
        // D s_13_7: call Elem_set(s_13_1, s_13_3, s_13_5, s_13_6)
        let s_13_7: Bits = Elem_set(state, tracer, s_13_1, s_13_3, s_13_5, s_13_6);
        // D s_13_8: cast reint s_13_7 -> u128
        let s_13_8: u128 = (s_13_7.value() as u128);
        // D s_13_9: write-var result <= s_13_8
        fn_state.result = s_13_8;
        // N s_13_10: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var VLshadow#4826:i64
        let s_14_0: i64 = fn_state.VLshadow_4826;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: cast reint s_14_1 -> i64
        let s_14_2: i64 = (s_14_1 as i64);
        // C s_14_3: const #128s : i
        let s_14_3: i128 = 128;
        // D s_14_4: read-var VLshadow#4826:i64
        let s_14_4: i64 = fn_state.VLshadow_4826;
        // D s_14_5: cast zx s_14_4 -> i
        let s_14_5: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_6: div s_14_5 s_14_3
        let s_14_6: i128 = ((s_14_5) / (s_14_3));
        // D s_14_7: cast reint s_14_6 -> i64
        let s_14_7: i64 = (s_14_6 as i64);
        // D s_14_8: read-var result:u128
        let s_14_8: u128 = fn_state.result;
        // D s_14_9: cast zx s_14_8 -> bv
        let s_14_9: Bits = Bits::new(s_14_8 as u128, 128u16);
        // D s_14_10: cast zx s_14_7 -> i
        let s_14_10: i128 = (i128::try_from(s_14_7).unwrap());
        // D s_14_11: cast reint s_14_10 -> u64
        let s_14_11: u64 = (s_14_10 as u64);
        // D s_14_12: call replicate_bits_borealis_internal(s_14_9, s_14_11)
        let s_14_12: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_14_9,
            s_14_11,
        );
        // D s_14_13: read-var t:i64
        let s_14_13: i64 = fn_state.t;
        // D s_14_14: cast zx s_14_13 -> i
        let s_14_14: i128 = (i128::try_from(s_14_13).unwrap());
        // D s_14_15: cast zx s_14_2 -> i
        let s_14_15: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_16: call Z_set(s_14_14, s_14_15, s_14_12)
        let s_14_16: () = Z_set(state, tracer, s_14_14, s_14_15, s_14_12);
        // N s_14_17: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call SP_read(s_15_0)
        let s_15_1: u64 = SP_read(state, tracer, s_15_0);
        // D s_15_2: write-var base <= s_15_1
        fn_state.base = s_15_1;
        // N s_15_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call CheckSPAlignment(s_16_0)
        let s_16_1: () = CheckSPAlignment(state, tracer, s_16_0);
        // N s_16_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #31s : i
        let s_17_0: i128 = 31;
        // D s_17_1: read-var n:i64
        let s_17_1: i64 = fn_state.n;
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // D s_17_3: cmp-eq s_17_2 s_17_0
        let s_17_3: bool = ((s_17_2) == (s_17_0));
        // N s_17_4: branch s_17_3 b23 b18
        if s_17_3 {
            return block_23(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#236627 <= s_18_0
        fn_state.gs_236627 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#236627:u8
        let s_19_0: bool = fn_state.gs_236627;
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
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call CheckSPAlignment(s_22_0)
        let s_22_1: () = CheckSPAlignment(state, tracer, s_22_0);
        // N s_22_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #53u : u32
        let s_23_0: u32 = 53;
        // S s_23_1: call ConstrainUnpredictableBool(s_23_0)
        let s_23_1: bool = ConstrainUnpredictableBool(state, tracer, s_23_0);
        // D s_23_2: write-var gs#236627 <= s_23_1
        fn_state.gs_236627 = s_23_1;
        // N s_23_3: jump b19
        return block_19(state, tracer, fn_state);
    }
}