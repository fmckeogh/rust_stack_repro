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
use SCTLR_read__1::*;
use u_get_HCR_EL2_Type_TWE::*;
use u_get_HCR_EL2_Type_TWI::*;
use u_get_SCR_EL3_Type_TWE::*;
use AArch64_WFxTrap::*;
use u_get_SCTLRType_nTWI::*;
use u_get_SCTLRType_nTWE::*;
use u_get_SCR_EL3_Type_TWI::*;
use common::*;
pub fn AArch64_CheckForWFxTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_el: u8,
    wfxtype: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        trap: bool,
        ga_4405: bool,
        gs_6611: bool,
        ga_4407: bool,
        ga_4406: bool,
        is_wfe: bool,
        target_el: u8,
        wfxtype: u32,
    }
    let fn_state = FunctionState {
        target_el,
        wfxtype,
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
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // D s_0_4: read-var wfxtype:u32
        let s_0_4: u32 = fn_state.wfxtype;
        // C s_0_5: const #0u : u32
        let s_0_5: u32 = 0;
        // D s_0_6: cmp-eq s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) == (s_0_5));
        // N s_0_7: branch s_0_6 b21 b1
        if s_0_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var wfxtype:u32
        let s_1_0: u32 = fn_state.wfxtype;
        // C s_1_1: const #2u : u32
        let s_1_1: u32 = 2;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // D s_1_3: write-var gs#6611 <= s_1_2
        fn_state.gs_6611 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#6611:u8
        let s_2_0: bool = fn_state.gs_6611;
        // D s_2_1: write-var is_wfe <= s_2_0
        fn_state.is_wfe = s_2_0;
        // D s_2_2: read-var target_el:u8
        let s_2_2: u8 = fn_state.target_el;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // C s_2_4: const #440u : u32
        let s_2_4: u32 = 440;
        // D s_2_5: read-reg s_2_4:u8
        let s_2_5: u8 = {
            let value = state.read_register::<u8>(s_2_4 as isize);
            tracer.read_register(s_2_4 as isize, value);
            value
        };
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 2u16);
        // D s_2_7: cmp-eq s_2_3 s_2_6
        let s_2_7: bool = ((s_2_3) == (s_2_6));
        // D s_2_8: not s_2_7
        let s_2_8: bool = !s_2_7;
        // N s_2_9: branch s_2_8 b10 b3
        if s_2_8 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var is_wfe:u8
        let s_3_0: bool = fn_state.is_wfe;
        // N s_3_1: branch s_3_0 b9 b4
        if s_3_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call SCTLR_read__1(s_4_0)
        let s_4_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_4_0);
        // S s_4_2: call _get_SCTLRType_nTWI(s_4_1)
        let s_4_2: bool = u_get_SCTLRType_nTWI(state, tracer, s_4_1);
        // D s_4_3: write-var ga#4405 <= s_4_2
        fn_state.ga_4405 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#4405:u8
        let s_5_0: bool = fn_state.ga_4405;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: write-var trap <= s_5_4
        fn_state.trap = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var trap:u8
        let s_6_0: bool = fn_state.trap;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var wfxtype:u32
        let s_8_0: u32 = fn_state.wfxtype;
        // D s_8_1: read-var target_el:u8
        let s_8_1: u8 = fn_state.target_el;
        // D s_8_2: call AArch64_WFxTrap(s_8_0, s_8_1)
        let s_8_2: () = AArch64_WFxTrap(state, tracer, s_8_0, s_8_1);
        // N s_8_3: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call SCTLR_read__1(s_9_0)
        let s_9_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_9_0);
        // S s_9_2: call _get_SCTLRType_nTWE(s_9_1)
        let s_9_2: bool = u_get_SCTLRType_nTWE(state, tracer, s_9_1);
        // D s_9_3: write-var ga#4405 <= s_9_2
        fn_state.ga_4405 = s_9_2;
        // N s_9_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var target_el:u8
        let s_10_0: u8 = fn_state.target_el;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #432u : u32
        let s_10_2: u32 = 432;
        // D s_10_3: read-reg s_10_2:u8
        let s_10_3: u8 = {
            let value = state.read_register::<u8>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 2u16);
        // D s_10_5: cmp-eq s_10_1 s_10_4
        let s_10_5: bool = ((s_10_1) == (s_10_4));
        // D s_10_6: not s_10_5
        let s_10_6: bool = !s_10_5;
        // N s_10_7: branch s_10_6 b15 b11
        if s_10_6 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var is_wfe:u8
        let s_11_0: bool = fn_state.is_wfe;
        // N s_11_1: branch s_11_0 b14 b12
        if s_11_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #102552u : u32
        let s_12_0: u32 = 102552;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_HCR_EL2_Type_TWI(s_12_1)
        let s_12_2: bool = u_get_HCR_EL2_Type_TWI(state, tracer, s_12_1);
        // D s_12_3: write-var ga#4406 <= s_12_2
        fn_state.ga_4406 = s_12_2;
        // N s_12_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var ga#4406:u8
        let s_13_0: bool = fn_state.ga_4406;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #1u : u8
        let s_13_2: bool = true;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: write-var trap <= s_13_4
        fn_state.trap = s_13_4;
        // N s_13_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #102552u : u32
        let s_14_0: u32 = 102552;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_HCR_EL2_Type_TWE(s_14_1)
        let s_14_2: bool = u_get_HCR_EL2_Type_TWE(state, tracer, s_14_1);
        // D s_14_3: write-var ga#4406 <= s_14_2
        fn_state.ga_4406 = s_14_2;
        // N s_14_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var target_el:u8
        let s_15_0: u8 = fn_state.target_el;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 2u16);
        // C s_15_2: const #424u : u32
        let s_15_2: u32 = 424;
        // D s_15_3: read-reg s_15_2:u8
        let s_15_3: u8 = {
            let value = state.read_register::<u8>(s_15_2 as isize);
            tracer.read_register(s_15_2 as isize, value);
            value
        };
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 2u16);
        // D s_15_5: cmp-eq s_15_1 s_15_4
        let s_15_5: bool = ((s_15_1) == (s_15_4));
        // D s_15_6: not s_15_5
        let s_15_6: bool = !s_15_5;
        // N s_15_7: branch s_15_6 b20 b16
        if s_15_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var is_wfe:u8
        let s_16_0: bool = fn_state.is_wfe;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #90704u : u32
        let s_17_0: u32 = 90704;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_SCR_EL3_Type_TWI(s_17_1)
        let s_17_2: bool = u_get_SCR_EL3_Type_TWI(state, tracer, s_17_1);
        // D s_17_3: write-var ga#4407 <= s_17_2
        fn_state.ga_4407 = s_17_2;
        // N s_17_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var ga#4407:u8
        let s_18_0: bool = fn_state.ga_4407;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: write-var trap <= s_18_4
        fn_state.trap = s_18_4;
        // N s_18_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #90704u : u32
        let s_19_0: u32 = 90704;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_SCR_EL3_Type_TWE(s_19_1)
        let s_19_2: bool = u_get_SCR_EL3_Type_TWE(state, tracer, s_19_1);
        // D s_19_3: write-var ga#4407 <= s_19_2
        fn_state.ga_4407 = s_19_2;
        // N s_19_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#6611 <= s_21_0
        fn_state.gs_6611 = s_21_0;
        // N s_21_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
