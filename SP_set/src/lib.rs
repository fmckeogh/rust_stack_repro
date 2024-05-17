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
pub fn SP_set<T: Tracer>(state: &mut State, tracer: &T, value_name: u64) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_2594: u8,
        value_name: u64,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16990u : u32
        let s_0_0: u32 = 16990;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: bool = {
            let value = state.read_register::<bool>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 1u16);
        // C s_0_3: const #0u : u8
        let s_0_3: bool = false;
        // C s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 1u16);
        // D s_0_5: cmp-eq s_0_2 s_0_4
        let s_0_5: bool = ((s_0_2) == (s_0_4));
        // N s_0_6: branch s_0_5 b10 b1
        if s_0_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: write-var ga#2594 <= s_1_1
        fn_state.ga_2594 = s_1_1;
        // D s_1_3: read-var ga#2594:u8
        let s_1_3: u8 = fn_state.ga_2594;
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // C s_1_5: const #448u : u32
        let s_1_5: u32 = 448;
        // D s_1_6: read-reg s_1_5:u8
        let s_1_6: u8 = {
            let value = state.read_register::<u8>(s_1_5 as isize);
            tracer.read_register(s_1_5 as isize, value);
            value
        };
        // D s_1_7: cast zx s_1_6 -> bv
        let s_1_7: Bits = Bits::new(s_1_6 as u128, 2u16);
        // D s_1_8: cmp-eq s_1_4 s_1_7
        let s_1_8: bool = ((s_1_4) == (s_1_7));
        // D s_1_9: not s_1_8
        let s_1_9: bool = !s_1_8;
        // N s_1_10: branch s_1_9 b3 b2
        if s_1_9 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var value_name:u64
        let s_2_0: u64 = fn_state.value_name;
        // C s_2_1: const #90128u : u32
        let s_2_1: u32 = 90128;
        // N s_2_2: write-reg s_2_1 <= s_2_0
        let s_2_2: () = {
            state.write_register::<u64>(s_2_1 as isize, s_2_0);
            tracer.write_register(s_2_1 as isize, s_2_0);
        };
        // N s_2_3: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#2594:u8
        let s_3_0: u8 = fn_state.ga_2594;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #440u : u32
        let s_3_2: u32 = 440;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // D s_3_6: not s_3_5
        let s_3_6: bool = !s_3_5;
        // N s_3_7: branch s_3_6 b5 b4
        if s_3_6 {
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
        // D s_4_0: read-var value_name:u64
        let s_4_0: u64 = fn_state.value_name;
        // C s_4_1: const #10184u : u32
        let s_4_1: u32 = 10184;
        // N s_4_2: write-reg s_4_1 <= s_4_0
        let s_4_2: () = {
            state.write_register::<u64>(s_4_1 as isize, s_4_0);
            tracer.write_register(s_4_1 as isize, s_4_0);
        };
        // N s_4_3: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#2594:u8
        let s_5_0: u8 = fn_state.ga_2594;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #432u : u32
        let s_5_2: u32 = 432;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // D s_5_6: not s_5_5
        let s_5_6: bool = !s_5_5;
        // N s_5_7: branch s_5_6 b7 b6
        if s_5_6 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var value_name:u64
        let s_6_0: u64 = fn_state.value_name;
        // C s_6_1: const #90968u : u32
        let s_6_1: u32 = 90968;
        // N s_6_2: write-reg s_6_1 <= s_6_0
        let s_6_2: () = {
            state.write_register::<u64>(s_6_1 as isize, s_6_0);
            tracer.write_register(s_6_1 as isize, s_6_0);
        };
        // N s_6_3: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#2594:u8
        let s_7_0: u8 = fn_state.ga_2594;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #424u : u32
        let s_7_2: u32 = 424;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // D s_7_6: not s_7_5
        let s_7_6: bool = !s_7_5;
        // N s_7_7: branch s_7_6 b9 b8
        if s_7_6 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var value_name:u64
        let s_8_0: u64 = fn_state.value_name;
        // C s_8_1: const #12080u : u32
        let s_8_1: u32 = 12080;
        // N s_8_2: write-reg s_8_1 <= s_8_0
        let s_8_2: () = {
            state.write_register::<u64>(s_8_1 as isize, s_8_0);
            tracer.write_register(s_8_1 as isize, s_8_0);
        };
        // N s_8_3: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var value_name:u64
        let s_10_0: u64 = fn_state.value_name;
        // C s_10_1: const #90128u : u32
        let s_10_1: u32 = 90128;
        // N s_10_2: write-reg s_10_1 <= s_10_0
        let s_10_2: () = {
            state.write_register::<u64>(s_10_1 as isize, s_10_0);
            tracer.write_register(s_10_1 as isize, s_10_0);
        };
        // N s_10_3: return
        return;
    }
}
