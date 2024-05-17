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
use u_get_MDSCR_EL1_Type_SS::*;
use ELUsingAArch32::*;
use DebugTarget::*;
use common::*;
pub fn SSAdvance<T: Tracer>(state: &mut State, tracer: &T, gs_24616: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_24618: bool,
        gs_24617: bool,
        gs_24616: (),
    }
    let fn_state = FunctionState {
        gs_24616,
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
        // S s_0_1: call DebugTarget(s_0_0)
        let s_0_1: u8 = DebugTarget(state, tracer, s_0_0);
        // S s_0_2: call ELUsingAArch32(s_0_1)
        let s_0_2: bool = ELUsingAArch32(state, tracer, s_0_1);
        // S s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b8 b1
        if s_0_3 {
            return block_8(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#24617 <= s_1_0
        fn_state.gs_24617 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#24617:u8
        let s_2_0: bool = fn_state.gs_24617;
        // N s_2_1: branch s_2_0 b7 b3
        if s_2_0 {
            return block_7(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#24618 <= s_3_0
        fn_state.gs_24618 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#24618:u8
        let s_4_0: bool = fn_state.gs_24618;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // C s_6_1: const #16991u : u32
        let s_6_1: u32 = 16991;
        // N s_6_2: write-reg s_6_1 <= s_6_0
        let s_6_2: () = {
            state.write_register::<bool>(s_6_1 as isize, s_6_0);
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
        // C s_7_0: const #16991u : u32
        let s_7_0: u32 = 16991;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: bool = {
            let value = state.read_register::<bool>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 1u16);
        // C s_7_3: const #1u : u8
        let s_7_3: bool = true;
        // C s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 1u16);
        // D s_7_5: cmp-eq s_7_2 s_7_4
        let s_7_5: bool = ((s_7_2) == (s_7_4));
        // D s_7_6: write-var gs#24618 <= s_7_5
        fn_state.gs_24618 = s_7_5;
        // N s_7_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #104648u : u32
        let s_8_0: u32 = 104648;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_MDSCR_EL1_Type_SS(s_8_1)
        let s_8_2: bool = u_get_MDSCR_EL1_Type_SS(state, tracer, s_8_1);
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #1u : u8
        let s_8_4: bool = true;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // D s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // D s_8_7: write-var gs#24617 <= s_8_6
        fn_state.gs_24617 = s_8_6;
        // N s_8_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
