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
use Broadcast::*;
use TLBI::*;
use common::*;
pub fn AArch64_TLBI_ALL<T: Tracer>(
    state: &mut State,
    tracer: &T,
    security: u32,
    regime: u32,
    shareability: u32,
    attr: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductTypefb7b2cabacce34a2,
        gs_26499: bool,
        security: u32,
        regime: u32,
        shareability: u32,
        attr: u32,
    }
    let fn_state = FunctionState {
        security,
        regime,
        shareability,
        attr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #424u : u32
        let s_0_3: u32 = 424;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b5 b1
        if s_0_6 {
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
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #432u : u32
        let s_1_3: u32 = 432;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // D s_1_7: write-var gs#26499 <= s_1_6
        fn_state.gs_26499 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#26499:u8
        let s_2_0: bool = fn_state.gs_26499;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #6u : u32
        let s_2_2: u32 = 6;
        // D s_2_3: write-var r.9 <= s_2_2
        fn_state.r._9 = s_2_2;
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // D s_2_5: write-var r.6 <= s_2_4
        fn_state.r._6 = s_2_4;
        // D s_2_6: read-var security:u32
        let s_2_6: u32 = fn_state.security;
        // D s_2_7: write-var r.11 <= s_2_6
        fn_state.r._11 = s_2_6;
        // D s_2_8: read-var regime:u32
        let s_2_8: u32 = fn_state.regime;
        // D s_2_9: write-var r.10 <= s_2_8
        fn_state.r._10 = s_2_8;
        // C s_2_10: const #0u : u32
        let s_2_10: u32 = 0;
        // D s_2_11: write-var r.8 <= s_2_10
        fn_state.r._8 = s_2_10;
        // D s_2_12: read-var attr:u32
        let s_2_12: u32 = fn_state.attr;
        // D s_2_13: write-var r.2 <= s_2_12
        fn_state.r._2 = s_2_12;
        // D s_2_14: read-var r:struct
        let s_2_14: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_2_15: read-var shareability:u32
        let s_2_15: u32 = fn_state.shareability;
        // D s_2_16: call TLBI(s_2_14, s_2_15)
        let s_2_16: () = TLBI(state, tracer, s_2_14, s_2_15);
        // D s_2_17: read-var shareability:u32
        let s_2_17: u32 = fn_state.shareability;
        // C s_2_18: const #0u : u32
        let s_2_18: u32 = 0;
        // D s_2_19: cmp-eq s_2_17 s_2_18
        let s_2_19: bool = ((s_2_17) == (s_2_18));
        // N s_2_20: branch s_2_19 b4 b3
        if s_2_19 {
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
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var shareability:u32
        let s_4_0: u32 = fn_state.shareability;
        // D s_4_1: read-var r:struct
        let s_4_1: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_4_2: call Broadcast(s_4_0, s_4_1)
        let s_4_2: () = Broadcast(state, tracer, s_4_0, s_4_1);
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
        // D s_5_1: write-var gs#26499 <= s_5_0
        fn_state.gs_26499 = s_5_0;
        // N s_5_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
