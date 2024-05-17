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
use Bit::*;
use X_read::*;
use SP_read::*;
use AArch64_AllocationTagFromAddress::*;
use common::*;
pub fn execute_aarch64_instrs_integer_tags_mcinserttagmask<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u64,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
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
        // N s_0_4: branch s_0_3 b3 b1
        if s_0_3 {
            return block_3(state, tracer, fn_state);
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
        // D s_2_1: read-var m:i64
        let s_2_1: i64 = fn_state.m;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: read-var address:u64
        let s_2_5: u64 = fn_state.address;
        // D s_2_6: call AArch64_AllocationTagFromAddress(s_2_5)
        let s_2_6: u8 = AArch64_AllocationTagFromAddress(state, tracer, s_2_5);
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // C s_2_10: const #1u : u8
        let s_2_10: bool = true;
        // S s_2_11: call Bit(s_2_10)
        let s_2_11: bool = Bit(state, tracer, s_2_10);
        // D s_2_12: cast zx s_2_4 -> bv
        let s_2_12: Bits = Bits::new(s_2_4 as u128, 64u16);
        // D s_2_13: cast zx s_2_9 -> i
        let s_2_13: i128 = (i128::try_from(s_2_9).unwrap());
        // C s_2_14: const #1u : u64
        let s_2_14: u64 = 1;
        // D s_2_15: bit-insert s_2_12 s_2_12 s_2_13 s_2_14
        let s_2_15: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_2_14 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_2_12.length(),
            );
            (s_2_12 & mask) | (s_2_12 << s_2_13)
        };
        // D s_2_16: cast reint s_2_15 -> u64
        let s_2_16: u64 = (s_2_15.value() as u64);
        // C s_2_17: const #64s : i64
        let s_2_17: i64 = 64;
        // D s_2_18: read-var d:i64
        let s_2_18: i64 = fn_state.d;
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (i128::try_from(s_2_18).unwrap());
        // D s_2_20: cast zx s_2_16 -> bv
        let s_2_20: Bits = Bits::new(s_2_16 as u128, 64u16);
        // D s_2_21: call X_set(s_2_19, s_2_17, s_2_20)
        let s_2_21: () = X_set(state, tracer, s_2_19, s_2_17, s_2_20);
        // N s_2_22: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call SP_read(s_3_0)
        let s_3_1: u64 = SP_read(state, tracer, s_3_0);
        // D s_3_2: write-var address <= s_3_1
        fn_state.address = s_3_1;
        // N s_3_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
