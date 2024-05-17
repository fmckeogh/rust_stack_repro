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
use CheckSVEEnabled::*;
use neq_int::*;
use X_read::*;
use common::*;
pub fn execute_CTERMNE_RR__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    esize: i64,
    m: i64,
    n: i64,
    op: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_2777: i64,
        element2: i128,
        element1: i128,
        term: bool,
        esize: i64,
        m: i64,
        n: i64,
        op: u32,
    }
    let fn_state = FunctionState {
        esize,
        m,
        n,
        op,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#2777 <= s_0_2
        fn_state.esizeshadow_2777 = s_0_2;
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
        // D s_1_0: read-var esizeshadow#2777:i64
        let s_1_0: i64 = fn_state.esizeshadow_2777;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call X_read(s_1_4, s_1_2)
        let s_1_5: Bits = X_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: read-var esizeshadow#2777:i64
        let s_1_6: i64 = fn_state.esizeshadow_2777;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: read-var m:i64
        let s_1_9: i64 = fn_state.m;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: call X_read(s_1_10, s_1_8)
        let s_1_11: Bits = X_read(state, tracer, s_1_10, s_1_8);
        // D s_1_12: cast zx s_1_5 -> i
        let s_1_12: i128 = (s_1_5.value() as i128);
        // D s_1_13: write-var element1 <= s_1_12
        fn_state.element1 = s_1_12;
        // D s_1_14: cast zx s_1_11 -> i
        let s_1_14: i128 = (s_1_11.value() as i128);
        // D s_1_15: write-var element2 <= s_1_14
        fn_state.element2 = s_1_14;
        // C s_1_16: const #0u : u32
        let s_1_16: u32 = 0;
        // D s_1_17: read-var op:u32
        let s_1_17: u32 = fn_state.op;
        // D s_1_18: cmp-eq s_1_16 s_1_17
        let s_1_18: bool = ((s_1_16) == (s_1_17));
        // D s_1_19: not s_1_18
        let s_1_19: bool = !s_1_18;
        // N s_1_20: branch s_1_19 b6 b2
        if s_1_19 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var element1:i
        let s_2_0: i128 = fn_state.element1;
        // D s_2_1: read-var element2:i
        let s_2_1: i128 = fn_state.element2;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // D s_2_3: write-var term <= s_2_2
        fn_state.term = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var term:u8
        let s_3_0: bool = fn_state.term;
        // N s_3_1: branch s_3_0 b5 b4
        if s_3_0 {
            return block_5(state, tracer, fn_state);
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
        // C s_4_1: const #16984u : u32
        let s_4_1: u32 = 16984;
        // N s_4_2: write-reg s_4_1 <= s_4_0
        let s_4_2: () = {
            state.write_register::<bool>(s_4_1 as isize, s_4_0);
            tracer.write_register(s_4_1 as isize, s_4_0);
        };
        // C s_4_3: const #16971u : u32
        let s_4_3: u32 = 16971;
        // D s_4_4: read-reg s_4_3:u8
        let s_4_4: bool = {
            let value = state.read_register::<bool>(s_4_3 as isize);
            tracer.read_register(s_4_3 as isize, value);
            value
        };
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // D s_4_6: not s_4_5
        let s_4_6: Bits = !s_4_5;
        // D s_4_7: cast reint s_4_6 -> u8
        let s_4_7: bool = ((s_4_6.value()) != 0);
        // C s_4_8: const #16996u : u32
        let s_4_8: u32 = 16996;
        // N s_4_9: write-reg s_4_8 <= s_4_7
        let s_4_9: () = {
            state.write_register::<bool>(s_4_8 as isize, s_4_7);
            tracer.write_register(s_4_8 as isize, s_4_7);
        };
        // N s_4_10: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // C s_5_1: const #16984u : u32
        let s_5_1: u32 = 16984;
        // N s_5_2: write-reg s_5_1 <= s_5_0
        let s_5_2: () = {
            state.write_register::<bool>(s_5_1 as isize, s_5_0);
            tracer.write_register(s_5_1 as isize, s_5_0);
        };
        // C s_5_3: const #0u : u8
        let s_5_3: bool = false;
        // C s_5_4: const #16996u : u32
        let s_5_4: u32 = 16996;
        // N s_5_5: write-reg s_5_4 <= s_5_3
        let s_5_5: () = {
            state.write_register::<bool>(s_5_4 as isize, s_5_3);
            tracer.write_register(s_5_4 as isize, s_5_3);
        };
        // N s_5_6: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: read-var op:u32
        let s_6_1: u32 = fn_state.op;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // D s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // N s_6_4: branch s_6_3 b8 b7
        if s_6_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var element1:i
        let s_7_0: i128 = fn_state.element1;
        // D s_7_1: read-var element2:i
        let s_7_1: i128 = fn_state.element2;
        // D s_7_2: call neq_int(s_7_0, s_7_1)
        let s_7_2: bool = neq_int(state, tracer, s_7_0, s_7_1);
        // D s_7_3: write-var term <= s_7_2
        fn_state.term = s_7_2;
        // N s_7_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b3
        return block_3(state, tracer, fn_state);
    }
}
