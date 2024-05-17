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
use ELFromM32::*;
use common::*;
pub fn AArch32_WriteMode<T: Tracer>(state: &mut State, tracer: &T, mode: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_4839: bool,
        ga_3231: ProductTypea5cc8de4daab131c,
        mode: u8,
    }
    let fn_state = FunctionState {
        mode,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var mode:u8
        let s_0_0: u8 = fn_state.mode;
        // D s_0_1: call ELFromM32(s_0_0)
        let s_0_1: ProductTypea5cc8de4daab131c = ELFromM32(state, tracer, s_0_0);
        // D s_0_2: write-var ga#3231 <= s_0_1
        fn_state.ga_3231 = s_0_1;
        // D s_0_3: read-var ga#3231.0:struct
        let s_0_3: bool = fn_state.ga_3231._0;
        // D s_0_4: read-var ga#3231.1:struct
        let s_0_4: u8 = fn_state.ga_3231._1;
        // N s_0_5: assert s_0_3
        let s_0_5: () = assert!(s_0_3);
        // D s_0_6: read-var mode:u8
        let s_0_6: u8 = fn_state.mode;
        // C s_0_7: const #16983u : u32
        let s_0_7: u32 = 16983;
        // N s_0_8: write-reg s_0_7 <= s_0_6
        let s_0_8: () = {
            state.write_register::<u8>(s_0_7 as isize, s_0_6);
            tracer.write_register(s_0_7 as isize, s_0_6);
        };
        // C s_0_9: const #16975u : u32
        let s_0_9: u32 = 16975;
        // N s_0_10: write-reg s_0_9 <= s_0_4
        let s_0_10: () = {
            state.write_register::<u8>(s_0_9 as isize, s_0_4);
            tracer.write_register(s_0_9 as isize, s_0_4);
        };
        // C s_0_11: const #1u : u8
        let s_0_11: bool = true;
        // C s_0_12: const #16999u : u32
        let s_0_12: u32 = 16999;
        // N s_0_13: write-reg s_0_12 <= s_0_11
        let s_0_13: () = {
            state.write_register::<bool>(s_0_12 as isize, s_0_11);
            tracer.write_register(s_0_12 as isize, s_0_11);
        };
        // D s_0_14: read-var mode:u8
        let s_0_14: u8 = fn_state.mode;
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 5u16);
        // C s_0_16: const #352u : u32
        let s_0_16: u32 = 352;
        // D s_0_17: read-reg s_0_16:u8
        let s_0_17: u8 = {
            let value = state.read_register::<u8>(s_0_16 as isize);
            tracer.read_register(s_0_16 as isize, value);
            value
        };
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 5u16);
        // D s_0_19: cmp-eq s_0_15 s_0_18
        let s_0_19: bool = ((s_0_15) == (s_0_18));
        // N s_0_20: branch s_0_19 b5 b1
        if s_0_19 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var mode:u8
        let s_1_0: u8 = fn_state.mode;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // C s_1_2: const #416u : u32
        let s_1_2: u32 = 416;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 5u16);
        // D s_1_5: cmp-eq s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) == (s_1_4));
        // D s_1_6: write-var gs#4839 <= s_1_5
        fn_state.gs_4839 = s_1_5;
        // N s_1_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#4839:u8
        let s_2_0: bool = fn_state.gs_4839;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // C s_3_1: const #16990u : u32
        let s_3_1: u32 = 16990;
        // N s_3_2: write-reg s_3_1 <= s_3_0
        let s_3_2: () = {
            state.write_register::<bool>(s_3_1 as isize, s_3_0);
            tracer.write_register(s_3_1 as isize, s_3_0);
        };
        // N s_3_3: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // C s_4_1: const #16990u : u32
        let s_4_1: u32 = 16990;
        // N s_4_2: write-reg s_4_1 <= s_4_0
        let s_4_2: () = {
            state.write_register::<bool>(s_4_1 as isize, s_4_0);
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
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var gs#4839 <= s_5_0
        fn_state.gs_4839 = s_5_0;
        // N s_5_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
