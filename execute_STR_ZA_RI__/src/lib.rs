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
use FailTransaction::*;
use IsAligned__1::*;
use CheckSMEAndZAEnabled::*;
use X_read::*;
use Elem_read::*;
use HaveTME::*;
use ZAvector_read::*;
use neq_int::*;
use AlignmentEnforced::*;
use SP_read::*;
use CreateAccDescSME::*;
use AArch64_Abort::*;
use CheckSPAlignment::*;
use AArch64_MemSingle_set::*;
use AlignmentFault::*;
use common::*;
pub fn execute_STR_ZA_RI__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    SVL: i64,
    n: i64,
    offset: i64,
    v: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        moffs: i128,
        e: i64,
        base: u64,
        dim: i64,
        SVLshadow_5513: i64,
        accdesc: ProductType9878976b5bcce9c9,
        gs_257174: i64,
        aligned: bool,
        src: Bits,
        gs_257162: bool,
        SVLshadow_5512: i64,
        vec: i64,
        gs_257169: bool,
        SVL: i64,
        n: i64,
        offset: i64,
        v: i64,
    }
    let fn_state = FunctionState {
        SVL,
        n,
        offset,
        v,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var SVL:i64
        let s_0_0: i64 = fn_state.SVL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var SVLshadow#5512 <= s_0_2
        fn_state.SVLshadow_5512 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckSMEAndZAEnabled(s_0_4)
        let s_0_5: () = CheckSMEAndZAEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var SVLshadow#5512:i64
        let s_1_0: i64 = fn_state.SVLshadow_5512;
        // D s_1_1: write-var SVLshadow#5513 <= s_1_0
        fn_state.SVLshadow_5513 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var SVLshadow#5513:i64
        let s_1_3: i64 = fn_state.SVLshadow_5513;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: write-var dim <= s_1_6
        fn_state.dim = s_1_6;
        // D s_1_8: read-var offset:i64
        let s_1_8: i64 = fn_state.offset;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: read-var dim:i64
        let s_1_10: i64 = fn_state.dim;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: mul s_1_9 s_1_11
        let s_1_12: i128 = ((s_1_9) * (s_1_11));
        // D s_1_13: write-var moffs <= s_1_12
        fn_state.moffs = s_1_12;
        // C s_1_14: const #32s : i64
        let s_1_14: i64 = 32;
        // D s_1_15: read-var v:i64
        let s_1_15: i64 = fn_state.v;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: call X_read(s_1_16, s_1_14)
        let s_1_17: Bits = X_read(state, tracer, s_1_16, s_1_14);
        // D s_1_18: cast reint s_1_17 -> u32
        let s_1_18: u32 = (s_1_17.value() as u32);
        // D s_1_19: cast zx s_1_18 -> bv
        let s_1_19: Bits = Bits::new(s_1_18 as u128, 32u16);
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (s_1_19.value() as i128);
        // D s_1_21: cast reint s_1_20 -> i64
        let s_1_21: i64 = (s_1_20 as i64);
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: read-var offset:i64
        let s_1_23: i64 = fn_state.offset;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: add s_1_22 s_1_24
        let s_1_25: i128 = (s_1_22 + s_1_24);
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: read-var dim:i64
        let s_1_28: i64 = fn_state.dim;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: mod s_1_27 s_1_29
        let s_1_30: i128 = ((s_1_27) % (s_1_29));
        // D s_1_31: cast reint s_1_30 -> i64
        let s_1_31: i64 = (s_1_30 as i64);
        // D s_1_32: write-var vec <= s_1_31
        fn_state.vec = s_1_31;
        // C s_1_33: const #31s : i
        let s_1_33: i128 = 31;
        // D s_1_34: read-var n:i64
        let s_1_34: i64 = fn_state.n;
        // D s_1_35: cast zx s_1_34 -> i
        let s_1_35: i128 = (i128::try_from(s_1_34).unwrap());
        // D s_1_36: call neq_int(s_1_35, s_1_33)
        let s_1_36: bool = neq_int(state, tracer, s_1_35, s_1_33);
        // C s_1_37: const #1u : u32
        let s_1_37: u32 = 1;
        // C s_1_38: const #0u : u8
        let s_1_38: bool = false;
        // C s_1_39: const #1u : u8
        let s_1_39: bool = true;
        // D s_1_40: call CreateAccDescSME(s_1_37, s_1_38, s_1_39, s_1_36)
        let s_1_40: ProductType9878976b5bcce9c9 = CreateAccDescSME(
            state,
            tracer,
            s_1_37,
            s_1_38,
            s_1_39,
            s_1_36,
        );
        // D s_1_41: write-var accdesc <= s_1_40
        fn_state.accdesc = s_1_40;
        // C s_1_42: const #() : ()
        let s_1_42: () = ();
        // S s_1_43: call HaveTME(s_1_42)
        let s_1_43: bool = HaveTME(state, tracer, s_1_42);
        // N s_1_44: branch s_1_43 b21 b2
        if s_1_43 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#257162 <= s_2_0
        fn_state.gs_257162 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#257162:u8
        let s_3_0: bool = fn_state.gs_257162;
        // N s_3_1: branch s_3_0 b20 b4
        if s_3_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #31s : i
        let s_5_0: i128 = 31;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // N s_5_4: branch s_5_3 b18 b6
        if s_5_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: read-var n:i64
        let s_6_1: i64 = fn_state.n;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: call X_read(s_6_2, s_6_0)
        let s_6_3: Bits = X_read(state, tracer, s_6_2, s_6_0);
        // D s_6_4: cast reint s_6_3 -> u64
        let s_6_4: u64 = (s_6_3.value() as u64);
        // D s_6_5: write-var base <= s_6_4
        fn_state.base = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var SVLshadow#5513:i64
        let s_7_0: i64 = fn_state.SVLshadow_5513;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var vec:i64
        let s_7_3: i64 = fn_state.vec;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast zx s_7_2 -> i
        let s_7_5: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_6: call ZAvector_read(s_7_4, s_7_5)
        let s_7_6: Bits = ZAvector_read(state, tracer, s_7_4, s_7_5);
        // D s_7_7: write-var src <= s_7_6
        fn_state.src = s_7_6;
        // D s_7_8: read-var base:u64
        let s_7_8: u64 = fn_state.base;
        // D s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 64u16);
        // D s_7_10: read-var moffs:i
        let s_7_10: i128 = fn_state.moffs;
        // D s_7_11: cast cvt s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 128);
        // D s_7_12: add s_7_9 s_7_11
        let s_7_12: Bits = (s_7_9 + s_7_11);
        // D s_7_13: cast reint s_7_12 -> u64
        let s_7_13: u64 = (s_7_12.value() as u64);
        // C s_7_14: const #16s : i
        let s_7_14: i128 = 16;
        // D s_7_15: cast zx s_7_13 -> bv
        let s_7_15: Bits = Bits::new(s_7_13 as u128, 64u16);
        // D s_7_16: call IsAligned__1(s_7_15, s_7_14)
        let s_7_16: bool = IsAligned__1(state, tracer, s_7_15, s_7_14);
        // D s_7_17: write-var aligned <= s_7_16
        fn_state.aligned = s_7_16;
        // D s_7_18: read-var aligned:u8
        let s_7_18: bool = fn_state.aligned;
        // D s_7_19: not s_7_18
        let s_7_19: bool = !s_7_18;
        // N s_7_20: branch s_7_19 b17 b8
        if s_7_19 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#257169 <= s_8_0
        fn_state.gs_257169 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#257169:u8
        let s_9_0: bool = fn_state.gs_257169;
        // N s_9_1: branch s_9_0 b16 b10
        if s_9_0 {
            return block_16(state, tracer, fn_state);
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
        // C s_11_0: const #0s : i64
        let s_11_0: i64 = 0;
        // C s_11_1: const #1s : i
        let s_11_1: i128 = 1;
        // D s_11_2: read-var dim:i64
        let s_11_2: i64 = fn_state.dim;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: sub s_11_3 s_11_1
        let s_11_4: i128 = ((s_11_3) - (s_11_1));
        // D s_11_5: cast reint s_11_4 -> i64
        let s_11_5: i64 = (s_11_4 as i64);
        // D s_11_6: write-var gs#257174 <= s_11_5
        fn_state.gs_257174 = s_11_5;
        // D s_11_7: write-var e <= s_11_0
        fn_state.e = s_11_0;
        // N s_11_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var e:i64
        let s_12_0: i64 = fn_state.e;
        // D s_12_1: read-var gs#257174:i64
        let s_12_1: i64 = fn_state.gs_257174;
        // D s_12_2: cmp-gt s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) > (s_12_1));
        // N s_12_3: branch s_12_2 b15 b13
        if s_12_2 {
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
        // D s_13_0: read-var moffs:i
        let s_13_0: i128 = fn_state.moffs;
        // D s_13_1: read-var base:u64
        let s_13_1: u64 = fn_state.base;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 64u16);
        // D s_13_3: cast cvt s_13_0 -> bv
        let s_13_3: Bits = Bits::new(s_13_0 as u128, 128);
        // D s_13_4: add s_13_2 s_13_3
        let s_13_4: Bits = (s_13_2 + s_13_3);
        // D s_13_5: cast reint s_13_4 -> u64
        let s_13_5: u64 = (s_13_4.value() as u64);
        // C s_13_6: const #8s : i64
        let s_13_6: i64 = 8;
        // D s_13_7: read-var e:i64
        let s_13_7: i64 = fn_state.e;
        // D s_13_8: cast zx s_13_7 -> i
        let s_13_8: i128 = (i128::try_from(s_13_7).unwrap());
        // C s_13_9: cast zx s_13_6 -> i
        let s_13_9: i128 = (i128::try_from(s_13_6).unwrap());
        // D s_13_10: read-var src:bv
        let s_13_10: Bits = fn_state.src;
        // D s_13_11: call Elem_read(s_13_10, s_13_8, s_13_9)
        let s_13_11: Bits = Elem_read(state, tracer, s_13_10, s_13_8, s_13_9);
        // D s_13_12: cast reint s_13_11 -> u8
        let s_13_12: u8 = (s_13_11.value() as u8);
        // C s_13_13: const #1s : i64
        let s_13_13: i64 = 1;
        // D s_13_14: cast zx s_13_12 -> bv
        let s_13_14: Bits = Bits::new(s_13_12 as u128, 8u16);
        // D s_13_15: read-var accdesc:struct
        let s_13_15: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_13_16: read-var aligned:u8
        let s_13_16: bool = fn_state.aligned;
        // D s_13_17: call AArch64_MemSingle_set(s_13_5, s_13_13, s_13_15, s_13_16, s_13_14)
        let s_13_17: () = AArch64_MemSingle_set(
            state,
            tracer,
            s_13_5,
            s_13_13,
            s_13_15,
            s_13_16,
            s_13_14,
        );
        // N s_13_18: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1s : i
        let s_14_0: i128 = 1;
        // D s_14_1: read-var moffs:i
        let s_14_1: i128 = fn_state.moffs;
        // D s_14_2: add s_14_1 s_14_0
        let s_14_2: i128 = (s_14_1 + s_14_0);
        // D s_14_3: write-var moffs <= s_14_2
        fn_state.moffs = s_14_2;
        // D s_14_4: read-var e:i64
        let s_14_4: i64 = fn_state.e;
        // C s_14_5: const #1s : i64
        let s_14_5: i64 = 1;
        // D s_14_6: add s_14_4 s_14_5
        let s_14_6: i64 = (s_14_4 + s_14_5);
        // D s_14_7: write-var e <= s_14_6
        fn_state.e = s_14_6;
        // N s_14_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var base:u64
        let s_16_0: u64 = fn_state.base;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 64u16);
        // D s_16_2: read-var moffs:i
        let s_16_2: i128 = fn_state.moffs;
        // D s_16_3: cast cvt s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 128);
        // D s_16_4: add s_16_1 s_16_3
        let s_16_4: Bits = (s_16_1 + s_16_3);
        // D s_16_5: cast reint s_16_4 -> u64
        let s_16_5: u64 = (s_16_4.value() as u64);
        // D s_16_6: read-var accdesc:struct
        let s_16_6: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_16_7: call AlignmentFault(s_16_6)
        let s_16_7: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_16_6);
        // D s_16_8: call AArch64_Abort(s_16_5, s_16_7)
        let s_16_8: () = AArch64_Abort(state, tracer, s_16_5, s_16_7);
        // N s_16_9: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call AlignmentEnforced(s_17_0)
        let s_17_1: bool = AlignmentEnforced(state, tracer, s_17_0);
        // D s_17_2: write-var gs#257169 <= s_17_1
        fn_state.gs_257169 = s_17_1;
        // N s_17_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call CheckSPAlignment(s_18_0)
        let s_18_1: () = CheckSPAlignment(state, tracer, s_18_0);
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call SP_read(s_19_0)
        let s_19_1: u64 = SP_read(state, tracer, s_19_0);
        // D s_19_2: write-var base <= s_19_1
        fn_state.base = s_19_1;
        // N s_19_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #2u : u32
        let s_20_0: u32 = 2;
        // C s_20_1: const #0u : u8
        let s_20_1: bool = false;
        // S s_20_2: call FailTransaction(s_20_0, s_20_1)
        let s_20_2: () = FailTransaction(state, tracer, s_20_0, s_20_1);
        // N s_20_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #100180u : u32
        let s_21_0: u32 = 100180;
        // D s_21_1: read-reg s_21_0:i
        let s_21_1: i128 = {
            let value = state.read_register::<i128>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // C s_21_2: const #0s : i
        let s_21_2: i128 = 0;
        // D s_21_3: cmp-gt s_21_1 s_21_2
        let s_21_3: bool = ((s_21_1) > (s_21_2));
        // D s_21_4: write-var gs#257162 <= s_21_3
        fn_state.gs_257162 = s_21_3;
        // N s_21_5: jump b3
        return block_3(state, tracer, fn_state);
    }
}
