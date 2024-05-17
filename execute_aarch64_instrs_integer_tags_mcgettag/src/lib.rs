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
use SP_read::*;
use CreateAccDescLDGSTG::*;
use Align_bits::*;
use X_read::*;
use AArch64_AddressWithAllocationTag::*;
use AArch64_MemTag_read::*;
use CheckSPAlignment::*;
use common::*;
pub fn execute_aarch64_instrs_integer_tags_mcgettag<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    offset: u64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        tag: u8,
        address: u64,
        n: i64,
        offset: u64,
        t: i64,
    }
    let fn_state = FunctionState {
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
        // C s_0_0: const #31s : i
        let s_0_0: i128 = 31;
        // D s_0_1: read-var n:i64
        let s_0_1: i64 = fn_state.n;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b4 b1
        if s_0_3 {
            return block_4(state, tracer, fn_state);
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
        // D s_2_0: read-var address:u64
        let s_2_0: u64 = fn_state.address;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 64u16);
        // D s_2_2: read-var offset:u64
        let s_2_2: u64 = fn_state.offset;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 64u16);
        // D s_2_4: add s_2_1 s_2_3
        let s_2_4: Bits = (s_2_1 + s_2_3);
        // D s_2_5: cast reint s_2_4 -> u64
        let s_2_5: u64 = (s_2_4.value() as u64);
        // D s_2_6: write-var address <= s_2_5
        fn_state.address = s_2_5;
        // D s_2_7: read-var address:u64
        let s_2_7: u64 = fn_state.address;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 64u16);
        // C s_2_9: const #1344u : u32
        let s_2_9: u32 = 1344;
        // D s_2_10: read-reg s_2_9:i64
        let s_2_10: i64 = {
            let value = state.read_register::<i64>(s_2_9 as isize);
            tracer.read_register(s_2_9 as isize, value);
            value
        };
        // D s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (i128::try_from(s_2_10).unwrap());
        // D s_2_12: call Align_bits(s_2_8, s_2_11)
        let s_2_12: Bits = Align_bits(state, tracer, s_2_8, s_2_11);
        // D s_2_13: cast reint s_2_12 -> u64
        let s_2_13: u64 = (s_2_12.value() as u64);
        // C s_2_14: const #0u : u32
        let s_2_14: u32 = 0;
        // S s_2_15: call CreateAccDescLDGSTG(s_2_14)
        let s_2_15: ProductType9878976b5bcce9c9 = CreateAccDescLDGSTG(
            state,
            tracer,
            s_2_14,
        );
        // D s_2_16: call AArch64_MemTag_read(s_2_13, s_2_15)
        let s_2_16: u8 = AArch64_MemTag_read(state, tracer, s_2_13, s_2_15);
        // D s_2_17: write-var tag <= s_2_16
        fn_state.tag = s_2_16;
        // N s_2_18: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // C s_3_1: const #64s : i64
        let s_3_1: i64 = 64;
        // D s_3_2: read-var t:i64
        let s_3_2: i64 = fn_state.t;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: call X_read(s_3_3, s_3_1)
        let s_3_4: Bits = X_read(state, tracer, s_3_3, s_3_1);
        // D s_3_5: cast reint s_3_4 -> u64
        let s_3_5: u64 = (s_3_4.value() as u64);
        // D s_3_6: read-var tag:u8
        let s_3_6: u8 = fn_state.tag;
        // D s_3_7: call AArch64_AddressWithAllocationTag(s_3_5, s_3_6)
        let s_3_7: u64 = AArch64_AddressWithAllocationTag(state, tracer, s_3_5, s_3_6);
        // D s_3_8: read-var t:i64
        let s_3_8: i64 = fn_state.t;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cast zx s_3_7 -> bv
        let s_3_10: Bits = Bits::new(s_3_7 as u128, 64u16);
        // D s_3_11: call X_set(s_3_9, s_3_0, s_3_10)
        let s_3_11: () = X_set(state, tracer, s_3_9, s_3_0, s_3_10);
        // N s_3_12: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call CheckSPAlignment(s_4_0)
        let s_4_1: () = CheckSPAlignment(state, tracer, s_4_0);
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call SP_read(s_5_0)
        let s_5_1: u64 = SP_read(state, tracer, s_5_0);
        // D s_5_2: write-var address <= s_5_1
        fn_state.address = s_5_1;
        // N s_5_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
