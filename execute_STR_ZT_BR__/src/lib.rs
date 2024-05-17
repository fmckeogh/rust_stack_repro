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
use CheckSMEZT0Enabled::*;
use X_read::*;
use Elem_read::*;
use HaveTME::*;
use neq_int::*;
use AlignmentEnforced::*;
use CheckSMEEnabled::*;
use SP_read::*;
use ZT0_read::*;
use CreateAccDescSME::*;
use AArch64_Abort::*;
use CheckSPAlignment::*;
use AArch64_MemSingle_set::*;
use AlignmentFault::*;
use common::*;
pub fn execute_STR_ZT_BR__<T: Tracer>(state: &mut State, tracer: &T, n: i64) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        base: u64,
        table: u64,
        accdesc: ProductType9878976b5bcce9c9,
        aligned: bool,
        gs_289933: bool,
        gs_289945: i64,
        gs_289940: bool,
        n: i64,
    }
    let fn_state = FunctionState {
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckSMEEnabled(s_0_0)
        let s_0_1: () = CheckSMEEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call CheckSMEZT0Enabled(s_1_0)
        let s_1_1: () = CheckSMEZT0Enabled(state, tracer, s_1_0);
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #512s : i64
        let s_2_0: i64 = 512;
        // S s_2_1: call ZT0_read(s_2_0)
        let s_2_1: Bits = ZT0_read(state, tracer, s_2_0);
        // S s_2_2: cast reint s_2_1 -> u512
        let s_2_2: u64 = (s_2_1.value() as u64);
        // D s_2_3: write-var table <= s_2_2
        fn_state.table = s_2_2;
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // C s_2_5: const #0u : u8
        let s_2_5: bool = false;
        // C s_2_6: const #31s : i
        let s_2_6: i128 = 31;
        // D s_2_7: read-var n:i64
        let s_2_7: i64 = fn_state.n;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: call neq_int(s_2_8, s_2_6)
        let s_2_9: bool = neq_int(state, tracer, s_2_8, s_2_6);
        // C s_2_10: const #1u : u32
        let s_2_10: u32 = 1;
        // D s_2_11: call CreateAccDescSME(s_2_10, s_2_5, s_2_4, s_2_9)
        let s_2_11: ProductType9878976b5bcce9c9 = CreateAccDescSME(
            state,
            tracer,
            s_2_10,
            s_2_5,
            s_2_4,
            s_2_9,
        );
        // D s_2_12: write-var accdesc <= s_2_11
        fn_state.accdesc = s_2_11;
        // C s_2_13: const #() : ()
        let s_2_13: () = ();
        // S s_2_14: call HaveTME(s_2_13)
        let s_2_14: bool = HaveTME(state, tracer, s_2_13);
        // N s_2_15: branch s_2_14 b22 b3
        if s_2_14 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#289933 <= s_3_0
        fn_state.gs_289933 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#289933:u8
        let s_4_0: bool = fn_state.gs_289933;
        // N s_4_1: branch s_4_0 b21 b5
        if s_4_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #31s : i
        let s_6_0: i128 = 31;
        // D s_6_1: read-var n:i64
        let s_6_1: i64 = fn_state.n;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_0
        let s_6_3: bool = ((s_6_2) == (s_6_0));
        // N s_6_4: branch s_6_3 b19 b7
        if s_6_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // D s_7_1: read-var n:i64
        let s_7_1: i64 = fn_state.n;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: call X_read(s_7_2, s_7_0)
        let s_7_3: Bits = X_read(state, tracer, s_7_2, s_7_0);
        // D s_7_4: cast reint s_7_3 -> u64
        let s_7_4: u64 = (s_7_3.value() as u64);
        // D s_7_5: write-var base <= s_7_4
        fn_state.base = s_7_4;
        // N s_7_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #16s : i
        let s_8_0: i128 = 16;
        // D s_8_1: read-var base:u64
        let s_8_1: u64 = fn_state.base;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 64u16);
        // D s_8_3: call IsAligned__1(s_8_2, s_8_0)
        let s_8_3: bool = IsAligned__1(state, tracer, s_8_2, s_8_0);
        // D s_8_4: write-var aligned <= s_8_3
        fn_state.aligned = s_8_3;
        // D s_8_5: read-var aligned:u8
        let s_8_5: bool = fn_state.aligned;
        // D s_8_6: not s_8_5
        let s_8_6: bool = !s_8_5;
        // N s_8_7: branch s_8_6 b18 b9
        if s_8_6 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#289940 <= s_9_0
        fn_state.gs_289940 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#289940:u8
        let s_10_0: bool = fn_state.gs_289940;
        // N s_10_1: branch s_10_0 b17 b11
        if s_10_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i64
        let s_12_0: i64 = 0;
        // C s_12_1: const #1s : i
        let s_12_1: i128 = 1;
        // C s_12_2: const #64s : i64
        let s_12_2: i64 = 64;
        // C s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // C s_12_4: sub s_12_3 s_12_1
        let s_12_4: i128 = ((s_12_3) - (s_12_1));
        // C s_12_5: cast reint s_12_4 -> i64
        let s_12_5: i64 = (s_12_4 as i64);
        // D s_12_6: write-var gs#289945 <= s_12_5
        fn_state.gs_289945 = s_12_5;
        // D s_12_7: write-var e <= s_12_0
        fn_state.e = s_12_0;
        // N s_12_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var e:i64
        let s_13_0: i64 = fn_state.e;
        // D s_13_1: read-var gs#289945:i64
        let s_13_1: i64 = fn_state.gs_289945;
        // D s_13_2: cmp-gt s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) > (s_13_1));
        // N s_13_3: branch s_13_2 b16 b14
        if s_13_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var base:u64
        let s_14_0: u64 = fn_state.base;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 64u16);
        // D s_14_2: read-var e:i64
        let s_14_2: i64 = fn_state.e;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: cast cvt s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 128);
        // D s_14_5: add s_14_1 s_14_4
        let s_14_5: Bits = (s_14_1 + s_14_4);
        // D s_14_6: cast reint s_14_5 -> u64
        let s_14_6: u64 = (s_14_5.value() as u64);
        // C s_14_7: const #8s : i64
        let s_14_7: i64 = 8;
        // D s_14_8: read-var table:u512
        let s_14_8: u64 = fn_state.table;
        // D s_14_9: cast zx s_14_8 -> bv
        let s_14_9: Bits = Bits::new(s_14_8 as u128, 512u16);
        // D s_14_10: read-var e:i64
        let s_14_10: i64 = fn_state.e;
        // D s_14_11: cast zx s_14_10 -> i
        let s_14_11: i128 = (i128::try_from(s_14_10).unwrap());
        // C s_14_12: cast zx s_14_7 -> i
        let s_14_12: i128 = (i128::try_from(s_14_7).unwrap());
        // D s_14_13: call Elem_read(s_14_9, s_14_11, s_14_12)
        let s_14_13: Bits = Elem_read(state, tracer, s_14_9, s_14_11, s_14_12);
        // D s_14_14: cast reint s_14_13 -> u8
        let s_14_14: u8 = (s_14_13.value() as u8);
        // C s_14_15: const #1s : i64
        let s_14_15: i64 = 1;
        // D s_14_16: cast zx s_14_14 -> bv
        let s_14_16: Bits = Bits::new(s_14_14 as u128, 8u16);
        // D s_14_17: read-var accdesc:struct
        let s_14_17: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_18: read-var aligned:u8
        let s_14_18: bool = fn_state.aligned;
        // D s_14_19: call AArch64_MemSingle_set(s_14_6, s_14_15, s_14_17, s_14_18, s_14_16)
        let s_14_19: () = AArch64_MemSingle_set(
            state,
            tracer,
            s_14_6,
            s_14_15,
            s_14_17,
            s_14_18,
            s_14_16,
        );
        // N s_14_20: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var e:i64
        let s_15_0: i64 = fn_state.e;
        // C s_15_1: const #1s : i64
        let s_15_1: i64 = 1;
        // D s_15_2: add s_15_0 s_15_1
        let s_15_2: i64 = (s_15_0 + s_15_1);
        // D s_15_3: write-var e <= s_15_2
        fn_state.e = s_15_2;
        // N s_15_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var accdesc:struct
        let s_17_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_17_1: call AlignmentFault(s_17_0)
        let s_17_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_17_0);
        // D s_17_2: read-var base:u64
        let s_17_2: u64 = fn_state.base;
        // D s_17_3: call AArch64_Abort(s_17_2, s_17_1)
        let s_17_3: () = AArch64_Abort(state, tracer, s_17_2, s_17_1);
        // N s_17_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call AlignmentEnforced(s_18_0)
        let s_18_1: bool = AlignmentEnforced(state, tracer, s_18_0);
        // D s_18_2: write-var gs#289940 <= s_18_1
        fn_state.gs_289940 = s_18_1;
        // N s_18_3: jump b10
        return block_10(state, tracer, fn_state);
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
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call SP_read(s_20_0)
        let s_20_1: u64 = SP_read(state, tracer, s_20_0);
        // D s_20_2: write-var base <= s_20_1
        fn_state.base = s_20_1;
        // N s_20_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #2u : u32
        let s_21_0: u32 = 2;
        // C s_21_1: const #0u : u8
        let s_21_1: bool = false;
        // S s_21_2: call FailTransaction(s_21_0, s_21_1)
        let s_21_2: () = FailTransaction(state, tracer, s_21_0, s_21_1);
        // N s_21_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #100180u : u32
        let s_22_0: u32 = 100180;
        // D s_22_1: read-reg s_22_0:i
        let s_22_1: i128 = {
            let value = state.read_register::<i128>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // C s_22_2: const #0s : i
        let s_22_2: i128 = 0;
        // D s_22_3: cmp-gt s_22_1 s_22_2
        let s_22_3: bool = ((s_22_1) > (s_22_2));
        // D s_22_4: write-var gs#289933 <= s_22_3
        fn_state.gs_289933 = s_22_3;
        // N s_22_5: jump b4
        return block_4(state, tracer, fn_state);
    }
}
