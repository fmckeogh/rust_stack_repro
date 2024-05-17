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
use X_set::*;
use CheckLDST64BEnabled::*;
use MemLoad64B::*;
use SP_read::*;
use CreateAccDescLS64::*;
use SPESampleGeneralPurposeLoadStore::*;
use BigEndian::*;
use X_read::*;
use AArch64_SetLSInstructionSyndrome::*;
use CheckSPAlignment::*;
use SetLoadStoreType::*;
use reverse_endianness::*;
use common::*;
pub fn execute_aarch64_instrs_memory_atomicops_ld_acc<T: Tracer>(
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
        accdesc: ProductType9878976b5bcce9c9,
        i: i64,
        data: u64,
        address: u64,
        value_name: u64,
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
        // C s_1_4: const #31s : i
        let s_1_4: i128 = 31;
        // D s_1_5: read-var n:i64
        let s_1_5: i64 = fn_state.n;
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: cmp-eq s_1_6 s_1_4
        let s_1_7: bool = ((s_1_6) == (s_1_4));
        // N s_1_8: branch s_1_7 b13 b2
        if s_1_7 {
            return block_13(state, tracer, fn_state);
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
        // D s_2_5: write-var address <= s_2_4
        fn_state.address = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var lst:u8
        let s_3_0: u8 = fn_state.lst;
        // D s_3_1: call SetLoadStoreType(s_3_0)
        let s_3_1: () = SetLoadStoreType(state, tracer, s_3_0);
        // C s_3_2: const #64s : i
        let s_3_2: i128 = 64;
        // D s_3_3: read-var t:i64
        let s_3_3: i64 = fn_state.t;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #0u : u8
        let s_3_5: bool = false;
        // C s_3_6: const #1u : u8
        let s_3_6: bool = true;
        // C s_3_7: const #0u : u8
        let s_3_7: bool = false;
        // D s_3_8: call AArch64_SetLSInstructionSyndrome(s_3_2, s_3_5, s_3_4, s_3_6, s_3_7)
        let s_3_8: () = AArch64_SetLSInstructionSyndrome(
            state,
            tracer,
            s_3_2,
            s_3_5,
            s_3_4,
            s_3_6,
            s_3_7,
        );
        // D s_3_9: read-var address:u64
        let s_3_9: u64 = fn_state.address;
        // D s_3_10: read-var accdesc:struct
        let s_3_10: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_3_11: call MemLoad64B(s_3_9, s_3_10)
        let s_3_11: u64 = MemLoad64B(state, tracer, s_3_9, s_3_10);
        // D s_3_12: write-var data <= s_3_11
        fn_state.data = s_3_11;
        // N s_3_13: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i64
        let s_4_0: i64 = 0;
        // D s_4_1: write-var i <= s_4_0
        fn_state.i = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var i:i64
        let s_5_0: i64 = fn_state.i;
        // C s_5_1: const #7s : i64
        let s_5_1: i64 = 7;
        // D s_5_2: cmp-gt s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) > (s_5_1));
        // N s_5_3: branch s_5_2 b10 b6
        if s_5_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i
        let s_6_0: i128 = 64;
        // D s_6_1: read-var i:i64
        let s_6_1: i64 = fn_state.i;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: mul s_6_0 s_6_2
        let s_6_3: i128 = ((s_6_0) * (s_6_2));
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // C s_6_5: const #64s : i
        let s_6_5: i128 = 64;
        // D s_6_6: read-var data:u512
        let s_6_6: u64 = fn_state.data;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 512u16);
        // D s_6_8: cast zx s_6_4 -> i
        let s_6_8: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_9: bit-extract s_6_7 s_6_8 s_6_5
        let s_6_9: Bits = (Bits::new(
            ((s_6_7) >> (s_6_8)).value(),
            u16::try_from(s_6_5).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u64
        let s_6_10: u64 = (s_6_9.value() as u64);
        // D s_6_11: write-var value_name <= s_6_10
        fn_state.value_name = s_6_10;
        // D s_6_12: read-var accdesc.1:struct
        let s_6_12: u32 = fn_state.accdesc._1;
        // D s_6_13: call BigEndian(s_6_12)
        let s_6_13: bool = BigEndian(state, tracer, s_6_12);
        // N s_6_14: branch s_6_13 b9 b7
        if s_6_13 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var t:i64
        let s_8_0: i64 = fn_state.t;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var i:i64
        let s_8_2: i64 = fn_state.i;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: add s_8_1 s_8_3
        let s_8_4: i128 = (s_8_1 + s_8_3);
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // C s_8_6: const #64s : i64
        let s_8_6: i64 = 64;
        // D s_8_7: cast zx s_8_5 -> i
        let s_8_7: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_8: read-var value_name:u64
        let s_8_8: u64 = fn_state.value_name;
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 64u16);
        // D s_8_10: call X_set(s_8_7, s_8_6, s_8_9)
        let s_8_10: () = X_set(state, tracer, s_8_7, s_8_6, s_8_9);
        // D s_8_11: read-var i:i64
        let s_8_11: i64 = fn_state.i;
        // C s_8_12: const #1s : i64
        let s_8_12: i64 = 1;
        // D s_8_13: add s_8_11 s_8_12
        let s_8_13: i64 = (s_8_11 + s_8_12);
        // D s_8_14: write-var i <= s_8_13
        fn_state.i = s_8_13;
        // N s_8_15: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var value_name:u64
        let s_9_0: u64 = fn_state.value_name;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 64u16);
        // D s_9_2: call reverse_endianness(s_9_1)
        let s_9_2: Bits = reverse_endianness(state, tracer, s_9_1);
        // D s_9_3: cast reint s_9_2 -> u64
        let s_9_3: u64 = (s_9_2.value() as u64);
        // D s_9_4: write-var value_name <= s_9_3
        fn_state.value_name = s_9_3;
        // N s_9_5: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_14_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
