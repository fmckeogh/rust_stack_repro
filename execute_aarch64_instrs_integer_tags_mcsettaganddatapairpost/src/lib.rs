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
use CheckSPAlignment::*;
use AlignmentFault::*;
use AArch64_AllocationTagFromAddress::*;
use common::*;
pub fn execute_aarch64_instrs_integer_tags_mcsettaganddatapairpost<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    offset: u64,
    postindex: bool,
    t: i64,
    t2: i64,
    writeback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        data2: u64,
        address: u64,
        accdesc: ProductType9878976b5bcce9c9,
        data1: u64,
        n: i64,
        offset: u64,
        postindex: bool,
        t: i64,
        t2: i64,
        writeback: bool,
    }
    let fn_state = FunctionState {
        n,
        offset,
        postindex,
        t,
        t2,
        writeback,
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
        // D s_0_1: read-var n:i64
        let s_0_1: i64 = fn_state.n;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b19 b1
        if s_0_3 {
            return block_19(state, tracer, fn_state);
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
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call X_read(s_1_2, s_1_0)
        let s_1_3: Bits = X_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u64
        let s_1_4: u64 = (s_1_3.value() as u64);
        // D s_1_5: write-var address <= s_1_4
        fn_state.address = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var t:i64
        let s_2_1: i64 = fn_state.t;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: write-var data1 <= s_2_4
        fn_state.data1 = s_2_4;
        // C s_2_6: const #64s : i64
        let s_2_6: i64 = 64;
        // D s_2_7: read-var t2:i64
        let s_2_7: i64 = fn_state.t2;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: call X_read(s_2_8, s_2_6)
        let s_2_9: Bits = X_read(state, tracer, s_2_8, s_2_6);
        // D s_2_10: cast reint s_2_9 -> u64
        let s_2_10: u64 = (s_2_9.value() as u64);
        // D s_2_11: write-var data2 <= s_2_10
        fn_state.data2 = s_2_10;
        // D s_2_12: read-var postindex:u8
        let s_2_12: bool = fn_state.postindex;
        // D s_2_13: not s_2_12
        let s_2_13: bool = !s_2_12;
        // N s_2_14: branch s_2_13 b18 b3
        if s_2_13 {
            return block_18(state, tracer, fn_state);
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
        // C s_4_0: const #1u : u32
        let s_4_0: u32 = 1;
        // S s_4_1: call CreateAccDescLDGSTG(s_4_0)
        let s_4_1: ProductType9878976b5bcce9c9 = CreateAccDescLDGSTG(
            state,
            tracer,
            s_4_0,
        );
        // D s_4_2: write-var accdesc <= s_4_1
        fn_state.accdesc = s_4_1;
        // D s_4_3: read-var address:u64
        let s_4_3: u64 = fn_state.address;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 64u16);
        // C s_4_5: const #1344u : u32
        let s_4_5: u32 = 1344;
        // D s_4_6: read-reg s_4_5:i64
        let s_4_6: i64 = {
            let value = state.read_register::<i64>(s_4_5 as isize);
            tracer.read_register(s_4_5 as isize, value);
            value
        };
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: call IsAligned__1(s_4_4, s_4_7)
        let s_4_8: bool = IsAligned__1(state, tracer, s_4_4, s_4_7);
        // D s_4_9: not s_4_8
        let s_4_9: bool = !s_4_8;
        // N s_4_10: branch s_4_9 b17 b5
        if s_4_9 {
            return block_17(state, tracer, fn_state);
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
        // C s_6_0: const #8s : i
        let s_6_0: i128 = 8;
        // D s_6_1: read-var data1:u64
        let s_6_1: u64 = fn_state.data1;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 64u16);
        // D s_6_3: read-var address:u64
        let s_6_3: u64 = fn_state.address;
        // D s_6_4: read-var accdesc:struct
        let s_6_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_6_5: call Mem_set(s_6_3, s_6_0, s_6_4, s_6_2)
        let s_6_5: () = Mem_set(state, tracer, s_6_3, s_6_0, s_6_4, s_6_2);
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #8s : i
        let s_7_0: i128 = 8;
        // D s_7_1: read-var address:u64
        let s_7_1: u64 = fn_state.address;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 64u16);
        // C s_7_3: cast cvt s_7_0 -> bv
        let s_7_3: Bits = Bits::new(s_7_0 as u128, 128);
        // D s_7_4: add s_7_2 s_7_3
        let s_7_4: Bits = (s_7_2 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> u64
        let s_7_5: u64 = (s_7_4.value() as u64);
        // C s_7_6: const #8s : i
        let s_7_6: i128 = 8;
        // D s_7_7: read-var data2:u64
        let s_7_7: u64 = fn_state.data2;
        // D s_7_8: cast zx s_7_7 -> bv
        let s_7_8: Bits = Bits::new(s_7_7 as u128, 64u16);
        // D s_7_9: read-var accdesc:struct
        let s_7_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_7_10: call Mem_set(s_7_5, s_7_6, s_7_9, s_7_8)
        let s_7_10: () = Mem_set(state, tracer, s_7_5, s_7_6, s_7_9, s_7_8);
        // N s_7_11: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var address:u64
        let s_8_0: u64 = fn_state.address;
        // D s_8_1: call AArch64_AllocationTagFromAddress(s_8_0)
        let s_8_1: u8 = AArch64_AllocationTagFromAddress(state, tracer, s_8_0);
        // D s_8_2: read-var address:u64
        let s_8_2: u64 = fn_state.address;
        // D s_8_3: read-var accdesc:struct
        let s_8_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_4: call AArch64_MemTag_set(s_8_2, s_8_3, s_8_1)
        let s_8_4: () = AArch64_MemTag_set(state, tracer, s_8_2, s_8_3, s_8_1);
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var writeback:u8
        let s_9_0: bool = fn_state.writeback;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var postindex:u8
        let s_11_0: bool = fn_state.postindex;
        // N s_11_1: branch s_11_0 b16 b12
        if s_11_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #31s : i
        let s_13_0: i128 = 31;
        // D s_13_1: read-var n:i64
        let s_13_1: i64 = fn_state.n;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: cmp-eq s_13_2 s_13_0
        let s_13_3: bool = ((s_13_2) == (s_13_0));
        // N s_13_4: branch s_13_3 b15 b14
        if s_13_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #64s : i64
        let s_14_0: i64 = 64;
        // D s_14_1: read-var n:i64
        let s_14_1: i64 = fn_state.n;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: read-var address:u64
        let s_14_3: u64 = fn_state.address;
        // D s_14_4: cast zx s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 64u16);
        // D s_14_5: call X_set(s_14_2, s_14_0, s_14_4)
        let s_14_5: () = X_set(state, tracer, s_14_2, s_14_0, s_14_4);
        // N s_14_6: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var address:u64
        let s_15_0: u64 = fn_state.address;
        // D s_15_1: call SP_set(s_15_0)
        let s_15_1: () = SP_set(state, tracer, s_15_0);
        // N s_15_2: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var address:u64
        let s_16_0: u64 = fn_state.address;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 64u16);
        // D s_16_2: read-var offset:u64
        let s_16_2: u64 = fn_state.offset;
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 64u16);
        // D s_16_4: add s_16_1 s_16_3
        let s_16_4: Bits = (s_16_1 + s_16_3);
        // D s_16_5: cast reint s_16_4 -> u64
        let s_16_5: u64 = (s_16_4.value() as u64);
        // D s_16_6: write-var address <= s_16_5
        fn_state.address = s_16_5;
        // N s_16_7: jump b13
        return block_13(state, tracer, fn_state);
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
        // D s_17_2: read-var address:u64
        let s_17_2: u64 = fn_state.address;
        // D s_17_3: call AArch64_Abort(s_17_2, s_17_1)
        let s_17_3: () = AArch64_Abort(state, tracer, s_17_2, s_17_1);
        // N s_17_4: jump b6
        return block_6(state, tracer, fn_state);
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
        // D s_18_2: read-var offset:u64
        let s_18_2: u64 = fn_state.offset;
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 64u16);
        // D s_18_4: add s_18_1 s_18_3
        let s_18_4: Bits = (s_18_1 + s_18_3);
        // D s_18_5: cast reint s_18_4 -> u64
        let s_18_5: u64 = (s_18_4.value() as u64);
        // D s_18_6: write-var address <= s_18_5
        fn_state.address = s_18_5;
        // N s_18_7: jump b4
        return block_4(state, tracer, fn_state);
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
        // D s_20_2: write-var address <= s_20_1
        fn_state.address = s_20_1;
        // N s_20_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
