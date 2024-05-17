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
use X_read::*;
use IsZero::*;
use BranchNotTaken::*;
use BranchTo::*;
use common::*;
pub fn execute_aarch64_instrs_branch_conditional_compare<T: Tracer>(
    state: &mut State,
    tracer: &T,
    datasize: i64,
    iszero: bool,
    offset: u64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasize: i64,
        iszero: bool,
        offset: u64,
        t: i64,
    }
    let fn_state = FunctionState {
        datasize,
        iszero,
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
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_4: cast reint s_0_3 -> i64
        let s_0_4: i64 = (s_0_3 as i64);
        // D s_0_5: read-var t:i64
        let s_0_5: i64 = fn_state.t;
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: call X_read(s_0_6, s_0_4)
        let s_0_7: Bits = X_read(state, tracer, s_0_6, s_0_4);
        // D s_0_8: call IsZero(s_0_7)
        let s_0_8: bool = IsZero(state, tracer, s_0_7);
        // D s_0_9: read-var iszero:u8
        let s_0_9: bool = fn_state.iszero;
        // D s_0_10: cmp-eq s_0_8 s_0_9
        let s_0_10: bool = ((s_0_8) == (s_0_9));
        // N s_0_11: branch s_0_10 b2 b1
        if s_0_10 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #5u : u32
        let s_1_0: u32 = 5;
        // C s_1_1: const #1u : u8
        let s_1_1: bool = true;
        // S s_1_2: call BranchNotTaken(s_1_0, s_1_1)
        let s_1_2: () = BranchNotTaken(state, tracer, s_1_0, s_1_1);
        // N s_1_3: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #12744u : u32
        let s_2_0: u32 = 12744;
        // D s_2_1: read-reg s_2_0:u64
        let s_2_1: u64 = {
            let value = state.read_register::<u64>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 64u16);
        // D s_2_3: read-var offset:u64
        let s_2_3: u64 = fn_state.offset;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 64u16);
        // D s_2_5: add s_2_2 s_2_4
        let s_2_5: Bits = (s_2_2 + s_2_4);
        // D s_2_6: cast reint s_2_5 -> u64
        let s_2_6: u64 = (s_2_5.value() as u64);
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 64u16);
        // C s_2_8: const #5u : u32
        let s_2_8: u32 = 5;
        // C s_2_9: const #1u : u8
        let s_2_9: bool = true;
        // D s_2_10: call BranchTo(s_2_7, s_2_8, s_2_9)
        let s_2_10: () = BranchTo(state, tracer, s_2_7, s_2_8, s_2_9);
        // N s_2_11: return
        return;
    }
}
