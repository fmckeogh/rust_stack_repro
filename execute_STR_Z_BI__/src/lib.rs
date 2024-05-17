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
use AlignmentEnforced::*;
use SP_read::*;
use CheckSVEEnabled::*;
use AArch64_Abort::*;
use IsAligned__1::*;
use X_read::*;
use Elem_read::*;
use CheckSPAlignment::*;
use Z_read::*;
use AArch64_MemSingle_set::*;
use AlignmentFault::*;
use common::*;
pub fn execute_STR_Z_BI__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    imm: i128,
    n: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_5037: i64,
        e: i64,
        gs_243065: i64,
        base: u64,
        accdesc: ProductType9878976b5bcce9c9,
        elements: i64,
        offset: i128,
        VLshadow_5038: i64,
        aligned: bool,
        src: Bits,
        gs_243060: bool,
        VL: i64,
        imm: i128,
        n: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        VL,
        imm,
        n,
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
        // D s_0_3: write-var VLshadow#5037 <= s_0_2
        fn_state.VLshadow_5037 = s_0_2;
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
        // D s_1_0: read-var VLshadow#5037:i64
        let s_1_0: i64 = fn_state.VLshadow_5037;
        // D s_1_1: write-var VLshadow#5038 <= s_1_0
        fn_state.VLshadow_5038 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#5038:i64
        let s_1_3: i64 = fn_state.VLshadow_5038;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: write-var elements <= s_1_6
        fn_state.elements = s_1_6;
        // D s_1_8: read-var elements:i64
        let s_1_8: i64 = fn_state.elements;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: read-var imm:i
        let s_1_10: i128 = fn_state.imm;
        // D s_1_11: mul s_1_10 s_1_9
        let s_1_11: i128 = ((s_1_10) * (s_1_9));
        // D s_1_12: write-var offset <= s_1_11
        fn_state.offset = s_1_11;
        // C s_1_13: const #31s : i
        let s_1_13: i128 = 31;
        // D s_1_14: read-var n:i64
        let s_1_14: i64 = fn_state.n;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: call neq_int(s_1_15, s_1_13)
        let s_1_16: bool = neq_int(state, tracer, s_1_15, s_1_13);
        // C s_1_17: const #1u : u32
        let s_1_17: u32 = 1;
        // C s_1_18: const #0u : u8
        let s_1_18: bool = false;
        // C s_1_19: const #1u : u8
        let s_1_19: bool = true;
        // D s_1_20: call CreateAccDescSVE(s_1_17, s_1_18, s_1_19, s_1_16)
        let s_1_20: ProductType9878976b5bcce9c9 = CreateAccDescSVE(
            state,
            tracer,
            s_1_17,
            s_1_18,
            s_1_19,
            s_1_16,
        );
        // D s_1_21: write-var accdesc <= s_1_20
        fn_state.accdesc = s_1_20;
        // C s_1_22: const #31s : i
        let s_1_22: i128 = 31;
        // D s_1_23: read-var n:i64
        let s_1_23: i64 = fn_state.n;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cmp-eq s_1_24 s_1_22
        let s_1_25: bool = ((s_1_24) == (s_1_22));
        // N s_1_26: branch s_1_25 b14 b2
        if s_1_25 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: write-var base <= s_2_4
        fn_state.base = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var VLshadow#5038:i64
        let s_3_0: i64 = fn_state.VLshadow_5038;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var t:i64
        let s_3_3: i64 = fn_state.t;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: call Z_read(s_3_4, s_3_5)
        let s_3_6: Bits = Z_read(state, tracer, s_3_4, s_3_5);
        // D s_3_7: write-var src <= s_3_6
        fn_state.src = s_3_6;
        // D s_3_8: read-var base:u64
        let s_3_8: u64 = fn_state.base;
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 64u16);
        // D s_3_10: read-var offset:i
        let s_3_10: i128 = fn_state.offset;
        // D s_3_11: cast cvt s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 128);
        // D s_3_12: add s_3_9 s_3_11
        let s_3_12: Bits = (s_3_9 + s_3_11);
        // D s_3_13: cast reint s_3_12 -> u64
        let s_3_13: u64 = (s_3_12.value() as u64);
        // C s_3_14: const #16s : i
        let s_3_14: i128 = 16;
        // D s_3_15: cast zx s_3_13 -> bv
        let s_3_15: Bits = Bits::new(s_3_13 as u128, 64u16);
        // D s_3_16: call IsAligned__1(s_3_15, s_3_14)
        let s_3_16: bool = IsAligned__1(state, tracer, s_3_15, s_3_14);
        // D s_3_17: write-var aligned <= s_3_16
        fn_state.aligned = s_3_16;
        // D s_3_18: read-var aligned:u8
        let s_3_18: bool = fn_state.aligned;
        // D s_3_19: not s_3_18
        let s_3_19: bool = !s_3_18;
        // N s_3_20: branch s_3_19 b13 b4
        if s_3_19 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#243060 <= s_4_0
        fn_state.gs_243060 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#243060:u8
        let s_5_0: bool = fn_state.gs_243060;
        // N s_5_1: branch s_5_0 b12 b6
        if s_5_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
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
        // D s_7_6: write-var gs#243065 <= s_7_5
        fn_state.gs_243065 = s_7_5;
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
        // D s_8_1: read-var gs#243065:i64
        let s_8_1: i64 = fn_state.gs_243065;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b11 b9
        if s_8_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var offset:i
        let s_9_0: i128 = fn_state.offset;
        // D s_9_1: read-var base:u64
        let s_9_1: u64 = fn_state.base;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 64u16);
        // D s_9_3: cast cvt s_9_0 -> bv
        let s_9_3: Bits = Bits::new(s_9_0 as u128, 128);
        // D s_9_4: add s_9_2 s_9_3
        let s_9_4: Bits = (s_9_2 + s_9_3);
        // D s_9_5: cast reint s_9_4 -> u64
        let s_9_5: u64 = (s_9_4.value() as u64);
        // C s_9_6: const #8s : i64
        let s_9_6: i64 = 8;
        // D s_9_7: read-var e:i64
        let s_9_7: i64 = fn_state.e;
        // D s_9_8: cast zx s_9_7 -> i
        let s_9_8: i128 = (i128::try_from(s_9_7).unwrap());
        // C s_9_9: cast zx s_9_6 -> i
        let s_9_9: i128 = (i128::try_from(s_9_6).unwrap());
        // D s_9_10: read-var src:bv
        let s_9_10: Bits = fn_state.src;
        // D s_9_11: call Elem_read(s_9_10, s_9_8, s_9_9)
        let s_9_11: Bits = Elem_read(state, tracer, s_9_10, s_9_8, s_9_9);
        // D s_9_12: cast reint s_9_11 -> u8
        let s_9_12: u8 = (s_9_11.value() as u8);
        // C s_9_13: const #1s : i64
        let s_9_13: i64 = 1;
        // D s_9_14: cast zx s_9_12 -> bv
        let s_9_14: Bits = Bits::new(s_9_12 as u128, 8u16);
        // D s_9_15: read-var accdesc:struct
        let s_9_15: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_9_16: read-var aligned:u8
        let s_9_16: bool = fn_state.aligned;
        // D s_9_17: call AArch64_MemSingle_set(s_9_5, s_9_13, s_9_15, s_9_16, s_9_14)
        let s_9_17: () = AArch64_MemSingle_set(
            state,
            tracer,
            s_9_5,
            s_9_13,
            s_9_15,
            s_9_16,
            s_9_14,
        );
        // N s_9_18: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1s : i
        let s_10_0: i128 = 1;
        // D s_10_1: read-var offset:i
        let s_10_1: i128 = fn_state.offset;
        // D s_10_2: add s_10_1 s_10_0
        let s_10_2: i128 = (s_10_1 + s_10_0);
        // D s_10_3: write-var offset <= s_10_2
        fn_state.offset = s_10_2;
        // D s_10_4: read-var e:i64
        let s_10_4: i64 = fn_state.e;
        // C s_10_5: const #1s : i64
        let s_10_5: i64 = 1;
        // D s_10_6: add s_10_4 s_10_5
        let s_10_6: i64 = (s_10_4 + s_10_5);
        // D s_10_7: write-var e <= s_10_6
        fn_state.e = s_10_6;
        // N s_10_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var base:u64
        let s_12_0: u64 = fn_state.base;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 64u16);
        // D s_12_2: read-var offset:i
        let s_12_2: i128 = fn_state.offset;
        // D s_12_3: cast cvt s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 128);
        // D s_12_4: add s_12_1 s_12_3
        let s_12_4: Bits = (s_12_1 + s_12_3);
        // D s_12_5: cast reint s_12_4 -> u64
        let s_12_5: u64 = (s_12_4.value() as u64);
        // D s_12_6: read-var accdesc:struct
        let s_12_6: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_12_7: call AlignmentFault(s_12_6)
        let s_12_7: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_12_6);
        // D s_12_8: call AArch64_Abort(s_12_5, s_12_7)
        let s_12_8: () = AArch64_Abort(state, tracer, s_12_5, s_12_7);
        // N s_12_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call AlignmentEnforced(s_13_0)
        let s_13_1: bool = AlignmentEnforced(state, tracer, s_13_0);
        // D s_13_2: write-var gs#243060 <= s_13_1
        fn_state.gs_243060 = s_13_1;
        // N s_13_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call CheckSPAlignment(s_14_0)
        let s_14_1: () = CheckSPAlignment(state, tracer, s_14_0);
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
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
        // N s_15_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
