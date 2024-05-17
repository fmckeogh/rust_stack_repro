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
use CheckLDST64BEnabled::*;
use SP_read::*;
use MemStore64B::*;
use CreateAccDescLS64::*;
use SPESampleGeneralPurposeLoadStore::*;
use BigEndian::*;
use X_read::*;
use AArch64_SetLSInstructionSyndrome::*;
use CheckSPAlignment::*;
use SetLoadStoreType::*;
use reverse_endianness::*;
use common::*;
pub fn execute_aarch64_instrs_memory_atomicops_st_acc_st64b<T: Tracer>(
    state: &mut State,
    tracer: &T,
    lst: u8,
    memop: u32,
    n: i64,
    t: i64,
    tagchecked: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        data: u64,
        address: u64,
        value_name: u64,
        accdesc: ProductType9878976b5bcce9c9,
        i: i64,
        lst: u8,
        memop: u32,
        n: i64,
        t: i64,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        lst,
        memop,
        n,
        t,
        tagchecked,
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
        // S s_0_1: call CheckLDST64BEnabled(s_0_0)
        let s_0_1: () = CheckLDST64BEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var memop:u32
        let s_1_0: u32 = fn_state.memop;
        // D s_1_1: read-var tagchecked:u8
        let s_1_1: bool = fn_state.tagchecked;
        // D s_1_2: call CreateAccDescLS64(s_1_0, s_1_1)
        let s_1_2: ProductType9878976b5bcce9c9 = CreateAccDescLS64(
            state,
            tracer,
            s_1_0,
            s_1_1,
        );
        // D s_1_3: write-var accdesc <= s_1_2
        fn_state.accdesc = s_1_2;
        // C s_1_4: const #0s : i64
        let s_1_4: i64 = 0;
        // D s_1_5: write-var i <= s_1_4
        fn_state.i = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var i:i64
        let s_2_0: i64 = fn_state.i;
        // C s_2_1: const #7s : i64
        let s_2_1: i64 = 7;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var t:i64
        let s_3_0: i64 = fn_state.t;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var i:i64
        let s_3_2: i64 = fn_state.i;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: add s_3_1 s_3_3
        let s_3_4: i128 = (s_3_1 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // C s_3_6: const #64s : i64
        let s_3_6: i64 = 64;
        // D s_3_7: cast zx s_3_5 -> i
        let s_3_7: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_8: call X_read(s_3_7, s_3_6)
        let s_3_8: Bits = X_read(state, tracer, s_3_7, s_3_6);
        // D s_3_9: cast reint s_3_8 -> u64
        let s_3_9: u64 = (s_3_8.value() as u64);
        // D s_3_10: write-var value_name <= s_3_9
        fn_state.value_name = s_3_9;
        // D s_3_11: read-var accdesc.1:struct
        let s_3_11: u32 = fn_state.accdesc._1;
        // D s_3_12: call BigEndian(s_3_11)
        let s_3_12: bool = BigEndian(state, tracer, s_3_11);
        // N s_3_13: branch s_3_12 b6 b4
        if s_3_12 {
            return block_6(state, tracer, fn_state);
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
        // C s_5_0: const #64s : i
        let s_5_0: i128 = 64;
        // D s_5_1: read-var i:i64
        let s_5_1: i64 = fn_state.i;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // C s_5_5: const #63s : i
        let s_5_5: i128 = 63;
        // D s_5_6: cast zx s_5_4 -> i
        let s_5_6: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_7: add s_5_5 s_5_6
        let s_5_7: i128 = (s_5_5 + s_5_6);
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // C s_5_9: const #64s : i
        let s_5_9: i128 = 64;
        // D s_5_10: read-var i:i64
        let s_5_10: i64 = fn_state.i;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: mul s_5_9 s_5_11
        let s_5_12: i128 = ((s_5_9) * (s_5_11));
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: read-var data:u512
        let s_5_14: u64 = fn_state.data;
        // D s_5_15: cast zx s_5_14 -> bv
        let s_5_15: Bits = Bits::new(s_5_14 as u128, 512u16);
        // D s_5_16: cast zx s_5_8 -> i
        let s_5_16: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_17: cast zx s_5_13 -> i
        let s_5_17: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_18: read-var value_name:u64
        let s_5_18: u64 = fn_state.value_name;
        // D s_5_19: cast zx s_5_18 -> bv
        let s_5_19: Bits = Bits::new(s_5_18 as u128, 64u16);
        // D s_5_20: sub s_5_16 s_5_17
        let s_5_20: i128 = ((s_5_16) - (s_5_17));
        // C s_5_21: const #1u : u64
        let s_5_21: u64 = 1;
        // C s_5_22: cast zx s_5_21 -> bv
        let s_5_22: Bits = Bits::new(s_5_21 as u128, 64u16);
        // D s_5_23: lsl s_5_22 s_5_20
        let s_5_23: Bits = s_5_22 << s_5_20;
        // D s_5_24: sub s_5_23 s_5_22
        let s_5_24: Bits = ((s_5_23) - (s_5_22));
        // D s_5_25: and s_5_19 s_5_24
        let s_5_25: Bits = ((s_5_19) & (s_5_24));
        // D s_5_26: lsl s_5_25 s_5_17
        let s_5_26: Bits = s_5_25 << s_5_17;
        // D s_5_27: lsl s_5_24 s_5_17
        let s_5_27: Bits = s_5_24 << s_5_17;
        // D s_5_28: cmpl s_5_27
        let s_5_28: Bits = !s_5_27;
        // D s_5_29: and s_5_15 s_5_28
        let s_5_29: Bits = ((s_5_15) & (s_5_28));
        // D s_5_30: or s_5_29 s_5_26
        let s_5_30: Bits = ((s_5_29) | (s_5_26));
        // D s_5_31: cast reint s_5_30 -> u512
        let s_5_31: u64 = (s_5_30.value() as u64);
        // D s_5_32: write-var data <= s_5_31
        fn_state.data = s_5_31;
        // D s_5_33: read-var i:i64
        let s_5_33: i64 = fn_state.i;
        // C s_5_34: const #1s : i64
        let s_5_34: i64 = 1;
        // D s_5_35: add s_5_33 s_5_34
        let s_5_35: i64 = (s_5_33 + s_5_34);
        // D s_5_36: write-var i <= s_5_35
        fn_state.i = s_5_35;
        // N s_5_37: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var value_name:u64
        let s_6_0: u64 = fn_state.value_name;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 64u16);
        // D s_6_2: call reverse_endianness(s_6_1)
        let s_6_2: Bits = reverse_endianness(state, tracer, s_6_1);
        // D s_6_3: cast reint s_6_2 -> u64
        let s_6_3: u64 = (s_6_2.value() as u64);
        // D s_6_4: write-var value_name <= s_6_3
        fn_state.value_name = s_6_3;
        // N s_6_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #31s : i
        let s_7_0: i128 = 31;
        // D s_7_1: read-var n:i64
        let s_7_1: i64 = fn_state.n;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_0
        let s_7_3: bool = ((s_7_2) == (s_7_0));
        // N s_7_4: branch s_7_3 b13 b8
        if s_7_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // D s_8_1: read-var n:i64
        let s_8_1: i64 = fn_state.n;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: call X_read(s_8_2, s_8_0)
        let s_8_3: Bits = X_read(state, tracer, s_8_2, s_8_0);
        // D s_8_4: cast reint s_8_3 -> u64
        let s_8_4: u64 = (s_8_3.value() as u64);
        // D s_8_5: write-var address <= s_8_4
        fn_state.address = s_8_4;
        // N s_8_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var lst:u8
        let s_9_0: u8 = fn_state.lst;
        // D s_9_1: call SetLoadStoreType(s_9_0)
        let s_9_1: () = SetLoadStoreType(state, tracer, s_9_0);
        // C s_9_2: const #64s : i
        let s_9_2: i128 = 64;
        // D s_9_3: read-var t:i64
        let s_9_3: i64 = fn_state.t;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // C s_9_5: const #0u : u8
        let s_9_5: bool = false;
        // C s_9_6: const #1u : u8
        let s_9_6: bool = true;
        // C s_9_7: const #0u : u8
        let s_9_7: bool = false;
        // D s_9_8: call AArch64_SetLSInstructionSyndrome(s_9_2, s_9_5, s_9_4, s_9_6, s_9_7)
        let s_9_8: () = AArch64_SetLSInstructionSyndrome(
            state,
            tracer,
            s_9_2,
            s_9_5,
            s_9_4,
            s_9_6,
            s_9_7,
        );
        // D s_9_9: read-var address:u64
        let s_9_9: u64 = fn_state.address;
        // D s_9_10: read-var data:u512
        let s_9_10: u64 = fn_state.data;
        // D s_9_11: read-var accdesc:struct
        let s_9_11: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_9_12: call MemStore64B(s_9_9, s_9_10, s_9_11)
        let s_9_12: () = MemStore64B(state, tracer, s_9_9, s_9_10, s_9_11);
        // N s_9_13: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #22416u : u32
        let s_10_0: u32 = 22416;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: bool = {
            let value = state.read_register::<bool>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // N s_10_2: branch s_10_1 b12 b11
        if s_10_1 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
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
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call SPESampleGeneralPurposeLoadStore(s_12_0)
        let s_12_1: () = SPESampleGeneralPurposeLoadStore(state, tracer, s_12_0);
        // N s_12_2: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call CheckSPAlignment(s_13_0)
        let s_13_1: () = CheckSPAlignment(state, tracer, s_13_0);
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call SP_read(s_14_0)
        let s_14_1: u64 = SP_read(state, tracer, s_14_0);
        // D s_14_2: write-var address <= s_14_1
        fn_state.address = s_14_1;
        // N s_14_3: jump b9
        return block_9(state, tracer, fn_state);
    }
}
