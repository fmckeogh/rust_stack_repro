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
use common::*;
pub fn u__GIC_InterruptID<T: Tracer>(state: &mut State, tracer: &T, intid: u32) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_331296: u32,
        intid: u32,
    }
    let fn_state = FunctionState {
        intid,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #5u : u32
        let s_0_0: u32 = 5;
        // D s_0_1: read-var intid:u32
        let s_0_1: u32 = fn_state.intid;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
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
    ) -> u32 {
        // C s_1_0: const #13u : u32
        let s_1_0: u32 = 13;
        // C s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 32u16);
        // C s_1_2: const #16s : i64
        let s_1_2: i64 = 16;
        // C s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // C s_1_4: cast cvt s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 128);
        // C s_1_5: add s_1_1 s_1_4
        let s_1_5: Bits = (s_1_1 + s_1_4);
        // C s_1_6: cast reint s_1_5 -> u32
        let s_1_6: u32 = (s_1_5.value() as u32);
        // D s_1_7: write-var gs#331296 <= s_1_6
        fn_state.gs_331296 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var gs#331296:u32
        let s_2_0: u32 = fn_state.gs_331296;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_3_0: const #6u : u32
        let s_3_0: u32 = 6;
        // D s_3_1: read-var intid:u32
        let s_3_1: u32 = fn_state.intid;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_4_0: const #10u : u32
        let s_4_0: u32 = 10;
        // C s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 32u16);
        // C s_4_2: const #16s : i64
        let s_4_2: i64 = 16;
        // C s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // C s_4_4: cast cvt s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 128);
        // C s_4_5: add s_4_1 s_4_4
        let s_4_5: Bits = (s_4_1 + s_4_4);
        // C s_4_6: cast reint s_4_5 -> u32
        let s_4_6: u32 = (s_4_5.value() as u32);
        // D s_4_7: write-var gs#331296 <= s_4_6
        fn_state.gs_331296 = s_4_6;
        // N s_4_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_5_0: const #9u : u32
        let s_5_0: u32 = 9;
        // D s_5_1: read-var intid:u32
        let s_5_1: u32 = fn_state.intid;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_6_0: const #11u : u32
        let s_6_0: u32 = 11;
        // C s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 32u16);
        // C s_6_2: const #16s : i64
        let s_6_2: i64 = 16;
        // C s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // C s_6_4: cast cvt s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 128);
        // C s_6_5: add s_6_1 s_6_4
        let s_6_5: Bits = (s_6_1 + s_6_4);
        // C s_6_6: cast reint s_6_5 -> u32
        let s_6_6: u32 = (s_6_5.value() as u32);
        // D s_6_7: write-var gs#331296 <= s_6_6
        fn_state.gs_331296 = s_6_6;
        // N s_6_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #1384u : u32
        let s_7_0: u32 = 1384;
        // D s_7_1: read-reg s_7_0:u32
        let s_7_1: u32 = {
            let value = state.read_register::<u32>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: write-var gs#331296 <= s_7_1
        fn_state.gs_331296 = s_7_1;
        // N s_7_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
