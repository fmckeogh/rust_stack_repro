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
use u__id::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use P_read::*;
use ActivePredicateElement::*;
use SP_read::*;
use ConstrainUnpredictableBool::*;
use X_read::*;
use Elem_read::*;
use CheckSPAlignment::*;
use Z_read::*;
use Mem_set::*;
use common::*;
pub fn execute_ST1H_Z_P_BI__<T: Tracer>(
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
        VLshadow_5149: i64,
        e: i64,
        base: u64,
        addr: u64,
        mbytes: i64,
        esizeshadow_5147: i64,
        gs_246517: bool,
        mask: Bits,
        gs_246522: i64,
        VLshadow_5148: i64,
        accdesc: ProductType9878976b5bcce9c9,
        elements: i64,
        gs_246530: bool,
        src: Bits,
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
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#5147 <= s_0_2
        fn_state.esizeshadow_5147 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#5148 <= s_0_6
        fn_state.VLshadow_5148 = s_0_6;
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
        // D s_1_0: read-var VLshadow#5148:i64
        let s_1_0: i64 = fn_state.VLshadow_5148;
        // D s_1_1: write-var VLshadow#5149 <= s_1_0
        fn_state.VLshadow_5149 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#5149:i64
        let s_1_3: i64 = fn_state.VLshadow_5149;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#5149:i64
        let s_1_7: i64 = fn_state.VLshadow_5149;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#5147:i64
        let s_1_9: i64 = fn_state.esizeshadow_5147;
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
        // D s_1_36: read-var esizeshadow#5147:i64
        let s_1_36: i64 = fn_state.esizeshadow_5147;
        // D s_1_37: cast zx s_1_36 -> i
        let s_1_37: i128 = (i128::try_from(s_1_36).unwrap());
        // D s_1_38: read-var mask:bv
        let s_1_38: Bits = fn_state.mask;
        // D s_1_39: call AnyActiveElement(s_1_38, s_1_37)
        let s_1_39: bool = AnyActiveElement(state, tracer, s_1_38, s_1_37);
        // D s_1_40: not s_1_39
        let s_1_40: bool = !s_1_39;
        // N s_1_41: branch s_1_40 b20 b2
        if s_1_40 {
            return block_20(state, tracer, fn_state);
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
        // N s_2_4: branch s_2_3 b19 b3
        if s_2_3 {
            return block_19(state, tracer, fn_state);
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
        // N s_4_4: branch s_4_3 b18 b5
        if s_4_3 {
            return block_18(state, tracer, fn_state);
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
        // D s_6_0: read-var VLshadow#5149:i64
        let s_6_0: i64 = fn_state.VLshadow_5149;
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
        // D s_7_6: write-var gs#246522 <= s_7_5
        fn_state.gs_246522 = s_7_5;
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
        // D s_8_1: read-var gs#246522:i64
        let s_8_1: i64 = fn_state.gs_246522;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b17 b9
        if s_8_2 {
            return block_17(state, tracer, fn_state);
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
        // D s_9_2: read-var esizeshadow#5147:i64
        let s_9_2: i64 = fn_state.esizeshadow_5147;
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
        // D s_12_15: write-var addr <= s_12_14
        fn_state.addr = s_12_14;
        // D s_12_16: read-var msize:i64
        let s_12_16: i64 = fn_state.msize;
        // D s_12_17: cast zx s_12_16 -> i
        let s_12_17: i128 = (i128::try_from(s_12_16).unwrap());
        // D s_12_18: call __id(s_12_17)
        let s_12_18: i128 = u__id(state, tracer, s_12_17);
        // D s_12_19: cast reint s_12_18 -> i64
        let s_12_19: i64 = (s_12_18 as i64);
        // C s_12_20: const #1s : i
        let s_12_20: i128 = 1;
        // D s_12_21: cast zx s_12_19 -> i
        let s_12_21: i128 = (i128::try_from(s_12_19).unwrap());
        // D s_12_22: sub s_12_21 s_12_20
        let s_12_22: i128 = ((s_12_21) - (s_12_20));
        // D s_12_23: cast reint s_12_22 -> i64
        let s_12_23: i64 = (s_12_22 as i64);
        // C s_12_24: const #0s : i
        let s_12_24: i128 = 0;
        // D s_12_25: cast zx s_12_23 -> i
        let s_12_25: i128 = (i128::try_from(s_12_23).unwrap());
        // D s_12_26: cmp-le s_12_24 s_12_25
        let s_12_26: bool = ((s_12_24) <= (s_12_25));
        // N s_12_27: branch s_12_26 b16 b13
        if s_12_26 {
            return block_16(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#246530 <= s_13_0
        fn_state.gs_246530 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#246530:u8
        let s_14_0: bool = fn_state.gs_246530;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // D s_14_2: read-var esizeshadow#5147:i64
        let s_14_2: i64 = fn_state.esizeshadow_5147;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: cast reint s_14_3 -> i64
        let s_14_4: i64 = (s_14_3 as i64);
        // D s_14_5: read-var e:i64
        let s_14_5: i64 = fn_state.e;
        // D s_14_6: cast zx s_14_5 -> i
        let s_14_6: i128 = (i128::try_from(s_14_5).unwrap());
        // D s_14_7: cast zx s_14_4 -> i
        let s_14_7: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_8: read-var src:bv
        let s_14_8: Bits = fn_state.src;
        // D s_14_9: call Elem_read(s_14_8, s_14_6, s_14_7)
        let s_14_9: Bits = Elem_read(state, tracer, s_14_8, s_14_6, s_14_7);
        // C s_14_10: const #0s : i
        let s_14_10: i128 = 0;
        // C s_14_11: const #16s : i
        let s_14_11: i128 = 16;
        // D s_14_12: bit-extract s_14_9 s_14_10 s_14_11
        let s_14_12: Bits = (Bits::new(
            ((s_14_9) >> (s_14_10)).value(),
            u16::try_from(s_14_11).unwrap(),
        ));
        // D s_14_13: cast reint s_14_12 -> u16
        let s_14_13: u16 = (s_14_12.value() as u16);
        // D s_14_14: read-var mbytes:i64
        let s_14_14: i64 = fn_state.mbytes;
        // D s_14_15: cast zx s_14_14 -> i
        let s_14_15: i128 = (i128::try_from(s_14_14).unwrap());
        // D s_14_16: cast zx s_14_13 -> bv
        let s_14_16: Bits = Bits::new(s_14_13 as u128, 16u16);
        // D s_14_17: read-var addr:u64
        let s_14_17: u64 = fn_state.addr;
        // D s_14_18: read-var accdesc:struct
        let s_14_18: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_19: call Mem_set(s_14_17, s_14_15, s_14_18, s_14_16)
        let s_14_19: () = Mem_set(state, tracer, s_14_17, s_14_15, s_14_18, s_14_16);
        // N s_14_20: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var msize:i64
        let s_16_0: i64 = fn_state.msize;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call __id(s_16_1)
        let s_16_2: i128 = u__id(state, tracer, s_16_1);
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: const #1s : i
        let s_16_4: i128 = 1;
        // D s_16_5: cast zx s_16_3 -> i
        let s_16_5: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_6: sub s_16_5 s_16_4
        let s_16_6: i128 = ((s_16_5) - (s_16_4));
        // D s_16_7: cast reint s_16_6 -> i64
        let s_16_7: i64 = (s_16_6 as i64);
        // D s_16_8: read-var esizeshadow#5147:i64
        let s_16_8: i64 = fn_state.esizeshadow_5147;
        // D s_16_9: cast zx s_16_8 -> i
        let s_16_9: i128 = (i128::try_from(s_16_8).unwrap());
        // D s_16_10: call __id(s_16_9)
        let s_16_10: i128 = u__id(state, tracer, s_16_9);
        // D s_16_11: cast reint s_16_10 -> i64
        let s_16_11: i64 = (s_16_10 as i64);
        // D s_16_12: cast zx s_16_7 -> i
        let s_16_12: i128 = (i128::try_from(s_16_7).unwrap());
        // D s_16_13: cast zx s_16_11 -> i
        let s_16_13: i128 = (i128::try_from(s_16_11).unwrap());
        // D s_16_14: cmp-lt s_16_12 s_16_13
        let s_16_14: bool = ((s_16_12) < (s_16_13));
        // D s_16_15: write-var gs#246530 <= s_16_14
        fn_state.gs_246530 = s_16_14;
        // N s_16_16: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call SP_read(s_18_0)
        let s_18_1: u64 = SP_read(state, tracer, s_18_0);
        // D s_18_2: write-var base <= s_18_1
        fn_state.base = s_18_1;
        // N s_18_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call CheckSPAlignment(s_19_0)
        let s_19_1: () = CheckSPAlignment(state, tracer, s_19_0);
        // N s_19_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #31s : i
        let s_20_0: i128 = 31;
        // D s_20_1: read-var n:i64
        let s_20_1: i64 = fn_state.n;
        // D s_20_2: cast zx s_20_1 -> i
        let s_20_2: i128 = (i128::try_from(s_20_1).unwrap());
        // D s_20_3: cmp-eq s_20_2 s_20_0
        let s_20_3: bool = ((s_20_2) == (s_20_0));
        // N s_20_4: branch s_20_3 b26 b21
        if s_20_3 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#246517 <= s_21_0
        fn_state.gs_246517 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#246517:u8
        let s_22_0: bool = fn_state.gs_246517;
        // N s_22_1: branch s_22_0 b25 b23
        if s_22_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call CheckSPAlignment(s_25_0)
        let s_25_1: () = CheckSPAlignment(state, tracer, s_25_0);
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #53u : u32
        let s_26_0: u32 = 53;
        // S s_26_1: call ConstrainUnpredictableBool(s_26_0)
        let s_26_1: bool = ConstrainUnpredictableBool(state, tracer, s_26_0);
        // D s_26_2: write-var gs#246517 <= s_26_1
        fn_state.gs_246517 = s_26_1;
        // N s_26_3: jump b22
        return block_22(state, tracer, fn_state);
    }
}
