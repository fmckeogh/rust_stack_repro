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
use CurrentInstrSet::*;
use AArch32_EnterMonitorMode::*;
use ELUsingAArch32::*;
use common::*;
pub fn AArch32_TakeMonitorTrapException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_31832: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_24832: i64,
        gs_31833: bool,
        vect_offset: i64,
        preferred_exception_return: u32,
        gs_31832: (),
    }
    let fn_state = FunctionState {
        gs_31832,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #424u : u32
        let s_0_0: u32 = 424;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // N s_0_4: branch s_0_3 b6 b1
        if s_0_3 {
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
        // D s_1_1: write-var gs#31833 <= s_1_0
        fn_state.gs_31833 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#31833:u8
        let s_2_0: bool = fn_state.gs_31833;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #32s : i64
        let s_2_2: i64 = 32;
        // C s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // S s_2_4: call ThisInstrAddr(s_2_3)
        let s_2_4: Bits = ThisInstrAddr(state, tracer, s_2_3);
        // S s_2_5: cast reint s_2_4 -> u32
        let s_2_5: u32 = (s_2_4.value() as u32);
        // D s_2_6: write-var preferred_exception_return <= s_2_5
        fn_state.preferred_exception_return = s_2_5;
        // C s_2_7: const #4u : u8
        let s_2_7: u8 = 4;
        // C s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 8u16);
        // C s_2_9: cast zx s_2_8 -> i
        let s_2_9: i128 = (s_2_8.value() as i128);
        // C s_2_10: cast reint s_2_9 -> i64
        let s_2_10: i64 = (s_2_9 as i64);
        // D s_2_11: write-var vect_offset <= s_2_10
        fn_state.vect_offset = s_2_10;
        // C s_2_12: const #() : ()
        let s_2_12: () = ();
        // S s_2_13: call CurrentInstrSet(s_2_12)
        let s_2_13: u32 = CurrentInstrSet(state, tracer, s_2_12);
        // C s_2_14: const #1u : u32
        let s_2_14: u32 = 1;
        // S s_2_15: cmp-eq s_2_13 s_2_14
        let s_2_15: bool = ((s_2_13) == (s_2_14));
        // N s_2_16: branch s_2_15 b5 b3
        if s_2_15 {
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
        // C s_3_0: const #2s : i64
        let s_3_0: i64 = 2;
        // D s_3_1: write-var ga#24832 <= s_3_0
        fn_state.ga_24832 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#24832:i64
        let s_4_0: i64 = fn_state.ga_24832;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var vect_offset:i64
        let s_4_2: i64 = fn_state.vect_offset;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var preferred_exception_return:u32
        let s_4_4: u32 = fn_state.preferred_exception_return;
        // D s_4_5: call AArch32_EnterMonitorMode(s_4_4, s_4_1, s_4_3)
        let s_4_5: () = AArch32_EnterMonitorMode(state, tracer, s_4_4, s_4_1, s_4_3);
        // N s_4_6: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #4s : i64
        let s_5_0: i64 = 4;
        // D s_5_1: write-var ga#24832 <= s_5_0
        fn_state.ga_24832 = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #424u : u32
        let s_6_0: u32 = 424;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call ELUsingAArch32(s_6_1)
        let s_6_2: bool = ELUsingAArch32(state, tracer, s_6_1);
        // D s_6_3: write-var gs#31833 <= s_6_2
        fn_state.gs_31833 = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
