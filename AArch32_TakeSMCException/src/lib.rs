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
use AArch32_ITAdvance::*;
use SSAdvance::*;
use AArch32_EnterMonitorMode::*;
use ELUsingAArch32::*;
use NextInstrAddr::*;
use common::*;
pub fn AArch32_TakeSMCException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_31791: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_31792: bool,
        gs_31791: (),
    }
    let fn_state = FunctionState {
        gs_31791,
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#31792 <= s_1_0
        fn_state.gs_31792 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#31792:u8
        let s_2_0: bool = fn_state.gs_31792;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call AArch32_ITAdvance(s_2_2)
        let s_2_3: () = AArch32_ITAdvance(state, tracer, s_2_2);
        // C s_2_4: const #() : ()
        let s_2_4: () = ();
        // S s_2_5: call SSAdvance(s_2_4)
        let s_2_5: () = SSAdvance(state, tracer, s_2_4);
        // C s_2_6: const #32s : i64
        let s_2_6: i64 = 32;
        // C s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // S s_2_8: call NextInstrAddr(s_2_7)
        let s_2_8: Bits = NextInstrAddr(state, tracer, s_2_7);
        // S s_2_9: cast reint s_2_8 -> u32
        let s_2_9: u32 = (s_2_8.value() as u32);
        // C s_2_10: const #8u : u8
        let s_2_10: u8 = 8;
        // C s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 8u16);
        // C s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (s_2_11.value() as i128);
        // C s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // C s_2_14: const #0s : i64
        let s_2_14: i64 = 0;
        // C s_2_15: cast zx s_2_14 -> i
        let s_2_15: i128 = (i128::try_from(s_2_14).unwrap());
        // C s_2_16: cast zx s_2_13 -> i
        let s_2_16: i128 = (i128::try_from(s_2_13).unwrap());
        // S s_2_17: call AArch32_EnterMonitorMode(s_2_9, s_2_15, s_2_16)
        let s_2_17: () = AArch32_EnterMonitorMode(state, tracer, s_2_9, s_2_15, s_2_16);
        // N s_2_18: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #424u : u32
        let s_3_0: u32 = 424;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call ELUsingAArch32(s_3_1)
        let s_3_2: bool = ELUsingAArch32(state, tracer, s_3_1);
        // D s_3_3: write-var gs#31792 <= s_3_2
        fn_state.gs_31792 = s_3_2;
        // N s_3_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
