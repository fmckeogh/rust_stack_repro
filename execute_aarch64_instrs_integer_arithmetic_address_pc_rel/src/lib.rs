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
use set_subrange_zeros::*;
use PC_read::*;
use common::*;
pub fn execute_aarch64_instrs_integer_arithmetic_address_pc_rel<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    imm: u64,
    page: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        base: u64,
        d: i64,
        imm: u64,
        page: bool,
    }
    let fn_state = FunctionState {
        d,
        imm,
        page,
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
        // S s_0_1: call PC_read(s_0_0)
        let s_0_1: u64 = PC_read(state, tracer, s_0_0);
        // D s_0_2: write-var base <= s_0_1
        fn_state.base = s_0_1;
        // D s_0_3: read-var page:u8
        let s_0_3: bool = fn_state.page;
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
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var base:u64
        let s_2_1: u64 = fn_state.base;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 64u16);
        // D s_2_3: read-var imm:u64
        let s_2_3: u64 = fn_state.imm;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 64u16);
        // D s_2_5: add s_2_2 s_2_4
        let s_2_5: Bits = (s_2_2 + s_2_4);
        // D s_2_6: cast reint s_2_5 -> u64
        let s_2_6: u64 = (s_2_5.value() as u64);
        // D s_2_7: read-var d:i64
        let s_2_7: i64 = fn_state.d;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: cast zx s_2_6 -> bv
        let s_2_9: Bits = Bits::new(s_2_6 as u128, 64u16);
        // D s_2_10: call X_set(s_2_8, s_2_0, s_2_9)
        let s_2_10: () = X_set(state, tracer, s_2_8, s_2_0, s_2_9);
        // N s_2_11: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i
        let s_3_0: i128 = 64;
        // C s_3_1: const #11s : i
        let s_3_1: i128 = 11;
        // C s_3_2: const #0s : i
        let s_3_2: i128 = 0;
        // D s_3_3: read-var base:u64
        let s_3_3: u64 = fn_state.base;
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 64u16);
        // D s_3_5: call set_subrange_zeros(s_3_0, s_3_4, s_3_1, s_3_2)
        let s_3_5: Bits = set_subrange_zeros(state, tracer, s_3_0, s_3_4, s_3_1, s_3_2);
        // D s_3_6: cast reint s_3_5 -> u64
        let s_3_6: u64 = (s_3_5.value() as u64);
        // D s_3_7: write-var base <= s_3_6
        fn_state.base = s_3_6;
        // N s_3_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
