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
use ELUsingAArch32::*;
use UsingAArch32::*;
use AArch32_ITAdvance::*;
use SSAdvance::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use NextInstrAddr::*;
use common::*;
pub fn AArch64_CallSecureMonitor<T: Tracer>(
    state: &mut State,
    tracer: &T,
    immediate: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_24620: bool,
        immediate: u16,
    }
    let fn_state = FunctionState {
        immediate,
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
        // D s_1_1: write-var gs#24620 <= s_1_0
        fn_state.gs_24620 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#24620:u8
        let s_2_0: bool = fn_state.gs_24620;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call UsingAArch32(s_2_2)
        let s_2_3: bool = UsingAArch32(state, tracer, s_2_2);
        // N s_2_4: branch s_2_3 b5 b3
        if s_2_3 {
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
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call SSAdvance(s_4_0)
        let s_4_1: () = SSAdvance(state, tracer, s_4_0);
        // C s_4_2: const #64s : i64
        let s_4_2: i64 = 64;
        // C s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // S s_4_4: call NextInstrAddr(s_4_3)
        let s_4_4: Bits = NextInstrAddr(state, tracer, s_4_3);
        // S s_4_5: cast reint s_4_4 -> u64
        let s_4_5: u64 = (s_4_4.value() as u64);
        // C s_4_6: const #0u : u8
        let s_4_6: u8 = 0;
        // C s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 4u16);
        // C s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (s_4_7.value() as i128);
        // C s_4_9: cast reint s_4_8 -> i64
        let s_4_9: i64 = (s_4_8 as i64);
        // C s_4_10: const #14u : u32
        let s_4_10: u32 = 14;
        // S s_4_11: call ExceptionSyndrome(s_4_10)
        let s_4_11: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_4_10,
        );
        // C s_4_12: cast zx s_4_9 -> i
        let s_4_12: i128 = (i128::try_from(s_4_9).unwrap());
        // C s_4_13: const #424u : u32
        let s_4_13: u32 = 424;
        // D s_4_14: read-reg s_4_13:u8
        let s_4_14: u8 = {
            let value = state.read_register::<u8>(s_4_13 as isize);
            tracer.read_register(s_4_13 as isize, value);
            value
        };
        // D s_4_15: call AArch64_TakeException(s_4_14, s_4_11, s_4_5, s_4_12)
        let s_4_15: () = AArch64_TakeException(
            state,
            tracer,
            s_4_14,
            s_4_11,
            s_4_5,
            s_4_12,
        );
        // N s_4_16: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call AArch32_ITAdvance(s_5_0)
        let s_5_1: () = AArch32_ITAdvance(state, tracer, s_5_0);
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
        // D s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // D s_6_4: write-var gs#24620 <= s_6_3
        fn_state.gs_24620 = s_6_3;
        // N s_6_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
