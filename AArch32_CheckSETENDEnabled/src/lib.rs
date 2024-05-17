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
use HSCTLR_read::*;
use SCTLR_read__2::*;
use u_get_SCTLR_Type_SED::*;
use SCTLR_read__1::*;
use u_get_HSCTLR_Type_SED::*;
use ELUsingAArch32::*;
use u_get_SCTLRType_SED::*;
use common::*;
pub fn AArch32_CheckSETENDEnabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_31940: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        setend_disabled: bool,
        gs_31940: (),
    }
    let fn_state = FunctionState {
        gs_31940,
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
        // C s_0_3: const #432u : u32
        let s_0_3: u32 = 432;
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
        // N s_0_7: branch s_0_6 b8 b1
        if s_0_6 {
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
        // C s_1_0: const #440u : u32
        let s_1_0: u32 = 440;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call ELUsingAArch32(s_1_1)
        let s_1_2: bool = ELUsingAArch32(state, tracer, s_1_1);
        // N s_1_3: branch s_1_2 b7 b2
        if s_1_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call SCTLR_read__1(s_2_0)
        let s_2_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_2_0);
        // S s_2_2: call _get_SCTLRType_SED(s_2_1)
        let s_2_2: bool = u_get_SCTLRType_SED(state, tracer, s_2_1);
        // D s_2_3: write-var setend_disabled <= s_2_2
        fn_state.setend_disabled = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
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
        // D s_4_0: read-var setend_disabled:u8
        let s_4_0: bool = fn_state.setend_disabled;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b6 b5
        if s_4_4 {
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
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call SCTLR_read__2(s_7_0)
        let s_7_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_7_0);
        // S s_7_2: call _get_SCTLR_Type_SED(s_7_1)
        let s_7_2: bool = u_get_SCTLR_Type_SED(state, tracer, s_7_1);
        // D s_7_3: write-var setend_disabled <= s_7_2
        fn_state.setend_disabled = s_7_2;
        // N s_7_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HSCTLR_read(s_8_0)
        let s_8_1: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_8_0);
        // S s_8_2: call _get_HSCTLR_Type_SED(s_8_1)
        let s_8_2: bool = u_get_HSCTLR_Type_SED(state, tracer, s_8_1);
        // D s_8_3: write-var setend_disabled <= s_8_2
        fn_state.setend_disabled = s_8_2;
        // N s_8_4: jump b4
        return block_4(state, tracer, fn_state);
    }
}
