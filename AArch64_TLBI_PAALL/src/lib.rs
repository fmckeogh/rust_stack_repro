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
use HaveRME::*;
use TLBI::*;
use common::*;
pub fn AArch64_TLBI_PAALL<T: Tracer>(
    state: &mut State,
    tracer: &T,
    shareability: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_27249: bool,
        r: ProductTypefb7b2cabacce34a2,
        shareability: u32,
    }
    let fn_state = FunctionState {
        shareability,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveRME(s_0_0)
        let s_0_1: bool = HaveRME(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b5 b1
        if s_0_1 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#27249 <= s_1_0
        fn_state.gs_27249 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#27249:u8
        let s_2_0: bool = fn_state.gs_27249;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #23u : u32
        let s_2_2: u32 = 23;
        // D s_2_3: write-var r.9 <= s_2_2
        fn_state.r._9 = s_2_2;
        // C s_2_4: const #0u : u32
        let s_2_4: u32 = 0;
        // D s_2_5: write-var r.8 <= s_2_4
        fn_state.r._8 = s_2_4;
        // C s_2_6: const #0u : u32
        let s_2_6: u32 = 0;
        // D s_2_7: write-var r.2 <= s_2_6
        fn_state.r._2 = s_2_6;
        // D s_2_8: read-var r:struct
        let s_2_8: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_2_9: read-var shareability:u32
        let s_2_9: u32 = fn_state.shareability;
        // D s_2_10: call TLBI(s_2_8, s_2_9)
        let s_2_10: () = TLBI(state, tracer, s_2_8, s_2_9);
        // D s_2_11: read-var shareability:u32
        let s_2_11: u32 = fn_state.shareability;
        // C s_2_12: const #0u : u32
        let s_2_12: u32 = 0;
        // D s_2_13: cmp-eq s_2_11 s_2_12
        let s_2_13: bool = ((s_2_11) == (s_2_12));
        // N s_2_14: branch s_2_13 b4 b3
        if s_2_13 {
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
        // C s_5_0: const #16975u : u32
        let s_5_0: u32 = 16975;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 2u16);
        // C s_5_3: const #424u : u32
        let s_5_3: u32 = 424;
        // D s_5_4: read-reg s_5_3:u8
        let s_5_4: u8 = {
            let value = state.read_register::<u8>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cmp-eq s_5_2 s_5_5
        let s_5_6: bool = ((s_5_2) == (s_5_5));
        // D s_5_7: write-var gs#27249 <= s_5_6
        fn_state.gs_27249 = s_5_6;
        // N s_5_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
