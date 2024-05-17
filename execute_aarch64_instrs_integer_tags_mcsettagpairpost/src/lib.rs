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
use Mem_set::*;
use AArch64_MemTag_set::*;
use SP_set::*;
use CreateAccDescLDGSTG::*;
use SP_read::*;
use AArch64_Abort::*;
use IsAligned__1::*;
use X_read::*;
use Zeros::*;
use CheckSPAlignment::*;
use AlignmentFault::*;
use AArch64_AllocationTagFromAddress::*;
use common::*;
pub fn execute_aarch64_instrs_integer_tags_mcsettagpairpost<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    offset: u64,
    postindex: bool,
    t: i64,
    writeback: bool,
    zero_data: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        tag: u8,
        data: u64,
        address: u64,
        accdesc: ProductType9878976b5bcce9c9,
        n: i64,
        offset: u64,
        postindex: bool,
        t: i64,
        writeback: bool,
        zero_data: bool,
    }
    let fn_state = FunctionState {
        n,
        offset,
        postindex,
        t,
        writeback,
        zero_data,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #31s : i
        let s_0_0: i128 = 31;
        // D s_0_1: read-var t:i64
        let s_0_1: i64 = fn_state.t;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b26 b1
        if s_0_3 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: read-var t:i64
        let s_1_1: i64 = fn_state.t;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call X_read(s_1_2, s_1_0)
        let s_1_3: Bits = X_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u64
        let s_1_4: u64 = (s_1_3.value() as u64);
        // D s_1_5: write-var data <= s_1_4
        fn_state.data = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var data:u64
        let s_2_0: u64 = fn_state.data;
        // D s_2_1: call AArch64_AllocationTagFromAddress(s_2_0)
        let s_2_1: u8 = AArch64_AllocationTagFromAddress(state, tracer, s_2_0);
        // D s_2_2: write-var tag <= s_2_1
        fn_state.tag = s_2_1;
        // C s_2_3: const #31s : i
        let s_2_3: i128 = 31;
        // D s_2_4: read-var n:i64
        let s_2_4: i64 = fn_state.n;
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: cmp-eq s_2_5 s_2_3
        let s_2_6: bool = ((s_2_5) == (s_2_3));
        // N s_2_7: branch s_2_6 b24 b3
        if s_2_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: read-var n:i64
        let s_3_1: i64 = fn_state.n;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: call X_read(s_3_2, s_3_0)
        let s_3_3: Bits = X_read(state, tracer, s_3_2, s_3_0);
        // D s_3_4: cast reint s_3_3 -> u64
        let s_3_4: u64 = (s_3_3.value() as u64);
        // D s_3_5: write-var address <= s_3_4
        fn_state.address = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var postindex:u8
        let s_4_0: bool = fn_state.postindex;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b23 b5
        if s_4_1 {
            return block_23(state, tracer, fn_state);
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
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // S s_6_1: call CreateAccDescLDGSTG(s_6_0)
        let s_6_1: ProductType9878976b5bcce9c9 = CreateAccDescLDGSTG(
            state,
            tracer,
            s_6_0,
        );
        // D s_6_2: write-var accdesc <= s_6_1
        fn_state.accdesc = s_6_1;
        // D s_6_3: read-var zero_data:u8
        let s_6_3: bool = fn_state.zero_data;
        // N s_6_4: branch s_6_3 b18 b7
        if s_6_3 {
            return block_18(state, tracer, fn_state);
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
        // D s_8_0: read-var address:u64
        let s_8_0: u64 = fn_state.address;
        // D s_8_1: read-var accdesc:struct
        let s_8_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_2: read-var tag:u8
        let s_8_2: u8 = fn_state.tag;
        // D s_8_3: call AArch64_MemTag_set(s_8_0, s_8_1, s_8_2)
        let s_8_3: () = AArch64_MemTag_set(state, tracer, s_8_0, s_8_1, s_8_2);
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var address:u64
        let s_9_0: u64 = fn_state.address;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 64u16);
        // C s_9_2: const #1344u : u32
        let s_9_2: u32 = 1344;
        // D s_9_3: read-reg s_9_2:i64
        let s_9_3: i64 = {
            let value = state.read_register::<i64>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cast cvt s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 128);
        // D s_9_6: add s_9_1 s_9_5
        let s_9_6: Bits = (s_9_1 + s_9_5);
        // D s_9_7: cast reint s_9_6 -> u64
        let s_9_7: u64 = (s_9_6.value() as u64);
        // D s_9_8: read-var accdesc:struct
        let s_9_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_9_9: read-var tag:u8
        let s_9_9: u8 = fn_state.tag;
        // D s_9_10: call AArch64_MemTag_set(s_9_7, s_9_8, s_9_9)
        let s_9_10: () = AArch64_MemTag_set(state, tracer, s_9_7, s_9_8, s_9_9);
        // N s_9_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var writeback:u8
        let s_10_0: bool = fn_state.writeback;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
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
        // D s_12_0: read-var postindex:u8
        let s_12_0: bool = fn_state.postindex;
        // N s_12_1: branch s_12_0 b17 b13
        if s_12_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #31s : i
        let s_14_0: i128 = 31;
        // D s_14_1: read-var n:i64
        let s_14_1: i64 = fn_state.n;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: cmp-eq s_14_2 s_14_0
        let s_14_3: bool = ((s_14_2) == (s_14_0));
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
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // D s_15_1: read-var n:i64
        let s_15_1: i64 = fn_state.n;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: read-var address:u64
        let s_15_3: u64 = fn_state.address;
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 64u16);
        // D s_15_5: call X_set(s_15_2, s_15_0, s_15_4)
        let s_15_5: () = X_set(state, tracer, s_15_2, s_15_0, s_15_4);
        // N s_15_6: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var address:u64
        let s_16_0: u64 = fn_state.address;
        // D s_16_1: call SP_set(s_16_0)
        let s_16_1: () = SP_set(state, tracer, s_16_0);
        // N s_16_2: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var address:u64
        let s_17_0: u64 = fn_state.address;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 64u16);
        // D s_17_2: read-var offset:u64
        let s_17_2: u64 = fn_state.offset;
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 64u16);
        // D s_17_4: add s_17_1 s_17_3
        let s_17_4: Bits = (s_17_1 + s_17_3);
        // D s_17_5: cast reint s_17_4 -> u64
        let s_17_5: u64 = (s_17_4.value() as u64);
        // D s_17_6: write-var address <= s_17_5
        fn_state.address = s_17_5;
        // N s_17_7: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var address:u64
        let s_18_0: u64 = fn_state.address;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 64u16);
        // C s_18_2: const #1344u : u32
        let s_18_2: u32 = 1344;
        // D s_18_3: read-reg s_18_2:i64
        let s_18_3: i64 = {
            let value = state.read_register::<i64>(s_18_2 as isize);
            tracer.read_register(s_18_2 as isize, value);
            value
        };
        // D s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_5: call IsAligned__1(s_18_1, s_18_4)
        let s_18_5: bool = IsAligned__1(state, tracer, s_18_1, s_18_4);
        // D s_18_6: not s_18_5
        let s_18_6: bool = !s_18_5;
        // N s_18_7: branch s_18_6 b22 b19
        if s_18_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #8s : i
        let s_20_0: i128 = 8;
        // C s_20_1: const #1344u : u32
        let s_20_1: u32 = 1344;
        // D s_20_2: read-reg s_20_1:i64
        let s_20_2: i64 = {
            let value = state.read_register::<i64>(s_20_1 as isize);
            tracer.read_register(s_20_1 as isize, value);
            value
        };
        // D s_20_3: cast zx s_20_2 -> i
        let s_20_3: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_4: mul s_20_3 s_20_0
        let s_20_4: i128 = ((s_20_3) * (s_20_0));
        // D s_20_5: cast reint s_20_4 -> i64
        let s_20_5: i64 = (s_20_4 as i64);
        // D s_20_6: cast zx s_20_5 -> i
        let s_20_6: i128 = (i128::try_from(s_20_5).unwrap());
        // D s_20_7: call Zeros(s_20_6)
        let s_20_7: Bits = Zeros(state, tracer, s_20_6);
        // D s_20_8: cast reint s_20_7 -> u128
        let s_20_8: u128 = (s_20_7.value() as u128);
        // C s_20_9: const #1344u : u32
        let s_20_9: u32 = 1344;
        // D s_20_10: read-reg s_20_9:i64
        let s_20_10: i64 = {
            let value = state.read_register::<i64>(s_20_9 as isize);
            tracer.read_register(s_20_9 as isize, value);
            value
        };
        // D s_20_11: cast zx s_20_10 -> i
        let s_20_11: i128 = (i128::try_from(s_20_10).unwrap());
        // D s_20_12: cast zx s_20_8 -> bv
        let s_20_12: Bits = Bits::new(s_20_8 as u128, 128u16);
        // D s_20_13: read-var address:u64
        let s_20_13: u64 = fn_state.address;
        // D s_20_14: read-var accdesc:struct
        let s_20_14: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_20_15: call Mem_set(s_20_13, s_20_11, s_20_14, s_20_12)
        let s_20_15: () = Mem_set(state, tracer, s_20_13, s_20_11, s_20_14, s_20_12);
        // N s_20_16: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var address:u64
        let s_21_0: u64 = fn_state.address;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 64u16);
        // C s_21_2: const #1344u : u32
        let s_21_2: u32 = 1344;
        // D s_21_3: read-reg s_21_2:i64
        let s_21_3: i64 = {
            let value = state.read_register::<i64>(s_21_2 as isize);
            tracer.read_register(s_21_2 as isize, value);
            value
        };
        // D s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_5: cast cvt s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 128);
        // D s_21_6: add s_21_1 s_21_5
        let s_21_6: Bits = (s_21_1 + s_21_5);
        // D s_21_7: cast reint s_21_6 -> u64
        let s_21_7: u64 = (s_21_6.value() as u64);
        // C s_21_8: const #8s : i
        let s_21_8: i128 = 8;
        // C s_21_9: const #1344u : u32
        let s_21_9: u32 = 1344;
        // D s_21_10: read-reg s_21_9:i64
        let s_21_10: i64 = {
            let value = state.read_register::<i64>(s_21_9 as isize);
            tracer.read_register(s_21_9 as isize, value);
            value
        };
        // D s_21_11: cast zx s_21_10 -> i
        let s_21_11: i128 = (i128::try_from(s_21_10).unwrap());
        // D s_21_12: mul s_21_11 s_21_8
        let s_21_12: i128 = ((s_21_11) * (s_21_8));
        // D s_21_13: cast reint s_21_12 -> i64
        let s_21_13: i64 = (s_21_12 as i64);
        // D s_21_14: cast zx s_21_13 -> i
        let s_21_14: i128 = (i128::try_from(s_21_13).unwrap());
        // D s_21_15: call Zeros(s_21_14)
        let s_21_15: Bits = Zeros(state, tracer, s_21_14);
        // D s_21_16: cast reint s_21_15 -> u128
        let s_21_16: u128 = (s_21_15.value() as u128);
        // C s_21_17: const #1344u : u32
        let s_21_17: u32 = 1344;
        // D s_21_18: read-reg s_21_17:i64
        let s_21_18: i64 = {
            let value = state.read_register::<i64>(s_21_17 as isize);
            tracer.read_register(s_21_17 as isize, value);
            value
        };
        // D s_21_19: cast zx s_21_18 -> i
        let s_21_19: i128 = (i128::try_from(s_21_18).unwrap());
        // D s_21_20: cast zx s_21_16 -> bv
        let s_21_20: Bits = Bits::new(s_21_16 as u128, 128u16);
        // D s_21_21: read-var accdesc:struct
        let s_21_21: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_21_22: call Mem_set(s_21_7, s_21_19, s_21_21, s_21_20)
        let s_21_22: () = Mem_set(state, tracer, s_21_7, s_21_19, s_21_21, s_21_20);
        // N s_21_23: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var accdesc:struct
        let s_22_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_22_1: call AlignmentFault(s_22_0)
        let s_22_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_22_0);
        // D s_22_2: read-var address:u64
        let s_22_2: u64 = fn_state.address;
        // D s_22_3: call AArch64_Abort(s_22_2, s_22_1)
        let s_22_3: () = AArch64_Abort(state, tracer, s_22_2, s_22_1);
        // N s_22_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var address:u64
        let s_23_0: u64 = fn_state.address;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 64u16);
        // D s_23_2: read-var offset:u64
        let s_23_2: u64 = fn_state.offset;
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 64u16);
        // D s_23_4: add s_23_1 s_23_3
        let s_23_4: Bits = (s_23_1 + s_23_3);
        // D s_23_5: cast reint s_23_4 -> u64
        let s_23_5: u64 = (s_23_4.value() as u64);
        // D s_23_6: write-var address <= s_23_5
        fn_state.address = s_23_5;
        // N s_23_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call CheckSPAlignment(s_24_0)
        let s_24_1: () = CheckSPAlignment(state, tracer, s_24_0);
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call SP_read(s_25_0)
        let s_25_1: u64 = SP_read(state, tracer, s_25_0);
        // D s_25_2: write-var address <= s_25_1
        fn_state.address = s_25_1;
        // N s_25_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call SP_read(s_26_0)
        let s_26_1: u64 = SP_read(state, tracer, s_26_0);
        // D s_26_2: write-var data <= s_26_1
        fn_state.data = s_26_1;
        // N s_26_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
