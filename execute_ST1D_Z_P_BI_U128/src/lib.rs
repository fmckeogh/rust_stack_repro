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
use CreateAccDescSVE::*;
use SP_read::*;
use AnyActiveElement::*;
use P_read::*;
use ActivePredicateElement::*;
use ConstrainUnpredictableBool::*;
use X_read::*;
use Elem_read::*;
use CheckSPAlignment::*;
use Z_read::*;
use CheckNonStreamingSVEEnabled::*;
use Mem_set::*;
use common::*;
pub fn execute_ST1D_Z_P_BI_U128<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    msize: i64,
    n: i64,
    offset: i128,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_5157: i64,
        gs_246798: bool,
        e: i64,
        base: u64,
        mbytes: i64,
        accdesc: ProductType9878976b5bcce9c9,
        elements: i64,
        gs_246803: i64,
        src: Bits,
        VLshadow_5158: i64,
        mask: Bits,
        VL: i64,
        esize: i64,
        g: i64,
        msize: i64,
        n: i64,
        offset: i128,
        t: i64,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        msize,
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
        // D s_0_3: write-var VLshadow#5157 <= s_0_2
        fn_state.VLshadow_5157 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckNonStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#5157:i64
        let s_1_0: i64 = fn_state.VLshadow_5157;
        // D s_1_1: write-var VLshadow#5158 <= s_1_0
        fn_state.VLshadow_5158 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#5158:i64
        let s_1_3: i64 = fn_state.VLshadow_5158;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#5158:i64
        let s_1_7: i64 = fn_state.VLshadow_5158;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esize:i64
        let s_1_9: i64 = fn_state.esize;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var elements <= s_1_12
        fn_state.elements = s_1_12;
        // D s_1_14: cast zx s_1_6 -> i
        let s_1_14: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var g:i64
        let s_1_16: i64 = fn_state.g;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call P_read(s_1_17, s_1_18)
        let s_1_19: Bits = P_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: write-var mask <= s_1_19
        fn_state.mask = s_1_19;
        // C s_1_21: const #8s : i
        let s_1_21: i128 = 8;
        // D s_1_22: read-var msize:i64
        let s_1_22: i64 = fn_state.msize;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: div s_1_23 s_1_21
        let s_1_24: i128 = ((s_1_23) / (s_1_21));
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: write-var mbytes <= s_1_25
        fn_state.mbytes = s_1_25;
        // C s_1_27: const #31s : i
        let s_1_27: i128 = 31;
        // D s_1_28: read-var n:i64
        let s_1_28: i64 = fn_state.n;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: call neq_int(s_1_29, s_1_27)
        let s_1_30: bool = neq_int(state, tracer, s_1_29, s_1_27);
        // C s_1_31: const #1u : u32
        let s_1_31: u32 = 1;
        // C s_1_32: const #0u : u8
        let s_1_32: bool = false;
        // C s_1_33: const #1u : u8
        let s_1_33: bool = true;
        // D s_1_34: call CreateAccDescSVE(s_1_31, s_1_32, s_1_33, s_1_30)
        let s_1_34: ProductType9878976b5bcce9c9 = CreateAccDescSVE(
            state,
            tracer,
            s_1_31,
            s_1_32,
            s_1_33,
            s_1_30,
        );
        // D s_1_35: write-var accdesc <= s_1_34
        fn_state.accdesc = s_1_34;
        // D s_1_36: read-var esize:i64
        let s_1_36: i64 = fn_state.esize;
        // D s_1_37: cast zx s_1_36 -> i
        let s_1_37: i128 = (i128::try_from(s_1_36).unwrap());
        // D s_1_38: read-var mask:bv
        let s_1_38: Bits = fn_state.mask;
        // D s_1_39: call AnyActiveElement(s_1_38, s_1_37)
        let s_1_39: bool = AnyActiveElement(state, tracer, s_1_38, s_1_37);
        // D s_1_40: not s_1_39
        let s_1_40: bool = !s_1_39;
        // N s_1_41: branch s_1_40 b17 b2
        if s_1_40 {
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
        // D s_6_0: read-var VLshadow#5158:i64
        let s_6_0: i64 = fn_state.VLshadow_5158;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var t:i64
        let s_6_3: i64 = fn_state.t;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast zx s_6_2 -> i
        let s_6_5: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_6: call Z_read(s_6_4, s_6_5)
        let s_6_6: Bits = Z_read(state, tracer, s_6_4, s_6_5);
        // D s_6_7: write-var src <= s_6_6
        fn_state.src = s_6_6;
        // N s_6_8: jump b7
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
        // D s_7_6: write-var gs#246803 <= s_7_5
        fn_state.gs_246803 = s_7_5;
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
        // D s_8_1: read-var gs#246803:i64
        let s_8_1: i64 = fn_state.gs_246803;
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
        // N s_10_0: jump b11
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
        // D s_12_0: read-var elements:i64
        let s_12_0: i64 = fn_state.elements;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var offset:i
        let s_12_2: i128 = fn_state.offset;
        // D s_12_3: mul s_12_2 s_12_1
        let s_12_3: i128 = ((s_12_2) * (s_12_1));
        // D s_12_4: read-var e:i64
        let s_12_4: i64 = fn_state.e;
        // D s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_6: add s_12_3 s_12_5
        let s_12_6: i128 = (s_12_3 + s_12_5);
        // D s_12_7: read-var mbytes:i64
        let s_12_7: i64 = fn_state.mbytes;
        // D s_12_8: cast zx s_12_7 -> i
        let s_12_8: i128 = (i128::try_from(s_12_7).unwrap());
        // D s_12_9: mul s_12_6 s_12_8
        let s_12_9: i128 = ((s_12_6) * (s_12_8));
        // D s_12_10: read-var base:u64
        let s_12_10: u64 = fn_state.base;
        // D s_12_11: cast zx s_12_10 -> bv
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 64u16);
        // D s_12_12: cast cvt s_12_9 -> bv
        let s_12_12: Bits = Bits::new(s_12_9 as u128, 128);
        // D s_12_13: add s_12_11 s_12_12
        let s_12_13: Bits = (s_12_11 + s_12_12);
        // D s_12_14: cast reint s_12_13 -> u64
        let s_12_14: u64 = (s_12_13.value() as u64);
        // D s_12_15: read-var esize:i64
        let s_12_15: i64 = fn_state.esize;
        // D s_12_16: cast zx s_12_15 -> i
        let s_12_16: i128 = (i128::try_from(s_12_15).unwrap());
        // D s_12_17: cast reint s_12_16 -> i64
        let s_12_17: i64 = (s_12_16 as i64);
        // D s_12_18: read-var e:i64
        let s_12_18: i64 = fn_state.e;
        // D s_12_19: cast zx s_12_18 -> i
        let s_12_19: i128 = (i128::try_from(s_12_18).unwrap());
        // D s_12_20: cast zx s_12_17 -> i
        let s_12_20: i128 = (i128::try_from(s_12_17).unwrap());
        // D s_12_21: read-var src:bv
        let s_12_21: Bits = fn_state.src;
        // D s_12_22: call Elem_read(s_12_21, s_12_19, s_12_20)
        let s_12_22: Bits = Elem_read(state, tracer, s_12_21, s_12_19, s_12_20);
        // D s_12_23: cast reint s_12_22 -> u128
        let s_12_23: u128 = (s_12_22.value() as u128);
        // C s_12_24: const #0s : i
        let s_12_24: i128 = 0;
        // C s_12_25: const #64s : i
        let s_12_25: i128 = 64;
        // D s_12_26: cast zx s_12_23 -> bv
        let s_12_26: Bits = Bits::new(s_12_23 as u128, 128u16);
        // D s_12_27: bit-extract s_12_26 s_12_24 s_12_25
        let s_12_27: Bits = (Bits::new(
            ((s_12_26) >> (s_12_24)).value(),
            u16::try_from(s_12_25).unwrap(),
        ));
        // D s_12_28: cast reint s_12_27 -> u64
        let s_12_28: u64 = (s_12_27.value() as u64);
        // D s_12_29: read-var mbytes:i64
        let s_12_29: i64 = fn_state.mbytes;
        // D s_12_30: cast zx s_12_29 -> i
        let s_12_30: i128 = (i128::try_from(s_12_29).unwrap());
        // D s_12_31: cast zx s_12_28 -> bv
        let s_12_31: Bits = Bits::new(s_12_28 as u128, 64u16);
        // D s_12_32: read-var accdesc:struct
        let s_12_32: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_12_33: call Mem_set(s_12_14, s_12_30, s_12_32, s_12_31)
        let s_12_33: () = Mem_set(state, tracer, s_12_14, s_12_30, s_12_32, s_12_31);
        // N s_12_34: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
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
        // D s_18_1: write-var gs#246798 <= s_18_0
        fn_state.gs_246798 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#246798:u8
        let s_19_0: bool = fn_state.gs_246798;
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
        // D s_23_2: write-var gs#246798 <= s_23_1
        fn_state.gs_246798 = s_23_1;
        // N s_23_3: jump b19
        return block_19(state, tracer, fn_state);
    }
}
