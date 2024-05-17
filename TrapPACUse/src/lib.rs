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
use ThisInstrAddr::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use common::*;
pub fn TrapPACUse<T: Tracer>(state: &mut State, tracer: &T, target_el: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_14959: bool,
        gs_14958: bool,
        target_el: u8,
    }
    let fn_state = FunctionState {
        target_el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var target_el:u8
        let s_0_0: u8 = fn_state.target_el;
        // C s_0_1: const #2u : u8
        let s_0_1: u8 = 2;
        // D s_0_2: cmp-lt s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) < (s_0_1));
        // N s_0_3: branch s_0_2 b6 b1
        if s_0_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#14958 <= s_1_0
        fn_state.gs_14958 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#14958:u8
        let s_2_0: bool = fn_state.gs_14958;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#14959 <= s_3_0
        fn_state.gs_14959 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#14959:u8
        let s_4_0: bool = fn_state.gs_14959;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #64s : i64
        let s_4_2: i64 = 64;
        // C s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // S s_4_4: call ThisInstrAddr(s_4_3)
        let s_4_4: Bits = ThisInstrAddr(state, tracer, s_4_3);
        // S s_4_5: cast reint s_4_4 -> u64
        let s_4_5: u64 = (s_4_4.value() as u64);
        // C s_4_6: const #0s : i64
        let s_4_6: i64 = 0;
        // C s_4_7: const #10u : u32
        let s_4_7: u32 = 10;
        // S s_4_8: call ExceptionSyndrome(s_4_7)
        let s_4_8: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_4_7);
        // C s_4_9: cast zx s_4_6 -> i
        let s_4_9: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_10: read-var target_el:u8
        let s_4_10: u8 = fn_state.target_el;
        // D s_4_11: call AArch64_TakeException(s_4_10, s_4_8, s_4_5, s_4_9)
        let s_4_11: () = AArch64_TakeException(
            state,
            tracer,
            s_4_10,
            s_4_8,
            s_4_5,
            s_4_9,
        );
        // N s_4_12: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var target_el:u8
        let s_5_0: u8 = fn_state.target_el;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #16975u : u32
        let s_5_4: u32 = 16975;
        // D s_5_5: read-reg s_5_4:u8
        let s_5_5: u8 = {
            let value = state.read_register::<u8>(s_5_4 as isize);
            tracer.read_register(s_5_4 as isize, value);
            value
        };
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 2u16);
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (s_5_6.value() as i128);
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // D s_5_9: cast zx s_5_3 -> i
        let s_5_9: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_10: cast zx s_5_8 -> i
        let s_5_10: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_11: cmp-ge s_5_9 s_5_10
        let s_5_11: bool = ((s_5_9) >= (s_5_10));
        // D s_5_12: write-var gs#14959 <= s_5_11
        fn_state.gs_14959 = s_5_11;
        // N s_5_13: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var target_el:u8
        let s_6_0: u8 = fn_state.target_el;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #448u : u32
        let s_6_2: u32 = 448;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-ne s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) != (s_6_4));
        // D s_6_6: write-var gs#14958 <= s_6_5
        fn_state.gs_14958 = s_6_5;
        // N s_6_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
