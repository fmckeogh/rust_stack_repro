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
use WaitForEvent::*;
use HaveFeatWFxT::*;
use IsInHost::*;
use IsEventRegisterSet::*;
use HaveTWEDExt::*;
use u_get_HCR_EL2_Type_TWE::*;
use WFETrapDelay::*;
use SCTLR_read__1::*;
use EndOfInstruction::*;
use WaitForEventUntilDelay::*;
use AArch64_CheckForWFxTrap::*;
use u_get_SCR_EL3_Type_TWE::*;
use AArch64_WFxTrap::*;
use LocalTimeoutEvent::*;
use EL2Enabled::*;
use u_get_SCTLRType_nTWE::*;
use ClearEventRegister::*;
use common::*;
pub fn Hint_WFE<T: Tracer>(
    state: &mut State,
    tracer: &T,
    localtimeout: i128,
    wfxtype: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        trap: bool,
        gs_6660: bool,
        gs_6656: bool,
        gs_6668: bool,
        gs_6654: bool,
        ga_4462: ProductType6e608f0222d797fa,
        gs_6659: bool,
        gs_6657: bool,
        gs_6655: bool,
        target_el: u8,
        gs_6658: bool,
        localtimeout: i128,
        wfxtype: u32,
    }
    let fn_state = FunctionState {
        localtimeout,
        wfxtype,
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
        // S s_0_1: call IsEventRegisterSet(s_0_0)
        let s_0_1: bool = IsEventRegisterSet(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b47 b1
        if s_0_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveFeatWFxT(s_1_0)
        let s_1_1: bool = HaveFeatWFxT(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b46 b2
        if s_1_1 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#6654 <= s_2_0
        fn_state.gs_6654 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#6654:u8
        let s_3_0: bool = fn_state.gs_6654;
        // N s_3_1: branch s_3_0 b45 b4
        if s_3_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var trap <= s_4_0
        fn_state.trap = s_4_0;
        // C s_4_2: const #16975u : u32
        let s_4_2: u32 = 16975;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // C s_4_5: const #448u : u32
        let s_4_5: u32 = 448;
        // D s_4_6: read-reg s_4_5:u8
        let s_4_6: u8 = {
            let value = state.read_register::<u8>(s_4_5 as isize);
            tracer.read_register(s_4_5 as isize, value);
            value
        };
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 2u16);
        // D s_4_8: cmp-eq s_4_4 s_4_7
        let s_4_8: bool = ((s_4_4) == (s_4_7));
        // N s_4_9: branch s_4_8 b42 b5
        if s_4_8 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var trap:u8
        let s_6_0: bool = fn_state.trap;
        // D s_6_1: not s_6_0
        let s_6_1: bool = !s_6_0;
        // N s_6_2: branch s_6_1 b38 b7
        if s_6_1 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#6656 <= s_7_0
        fn_state.gs_6656 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#6656:u8
        let s_8_0: bool = fn_state.gs_6656;
        // N s_8_1: branch s_8_0 b37 b9
        if s_8_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#6657 <= s_9_0
        fn_state.gs_6657 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#6657:u8
        let s_10_0: bool = fn_state.gs_6657;
        // N s_10_1: branch s_10_0 b36 b11
        if s_10_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#6658 <= s_11_0
        fn_state.gs_6658 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#6658:u8
        let s_12_0: bool = fn_state.gs_6658;
        // N s_12_1: branch s_12_0 b33 b13
        if s_12_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var trap:u8
        let s_14_0: bool = fn_state.trap;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b32 b15
        if s_14_1 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#6659 <= s_15_0
        fn_state.gs_6659 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#6659:u8
        let s_16_0: bool = fn_state.gs_6659;
        // N s_16_1: branch s_16_0 b31 b17
        if s_16_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#6660 <= s_17_0
        fn_state.gs_6660 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#6660:u8
        let s_18_0: bool = fn_state.gs_6660;
        // N s_18_1: branch s_18_0 b28 b19
        if s_18_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var trap:u8
        let s_20_0: bool = fn_state.trap;
        // N s_20_1: branch s_20_0 b27 b21
        if s_20_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#6668 <= s_21_0
        fn_state.gs_6668 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#6668:u8
        let s_22_0: bool = fn_state.gs_6668;
        // N s_22_1: branch s_22_0 b24 b23
        if s_22_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var localtimeout:i
        let s_23_0: i128 = fn_state.localtimeout;
        // D s_23_1: call WaitForEvent(s_23_0)
        let s_23_1: () = WaitForEvent(state, tracer, s_23_0);
        // N s_23_2: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var target_el:u8
        let s_24_0: u8 = fn_state.target_el;
        // D s_24_1: call WFETrapDelay(s_24_0)
        let s_24_1: ProductType6e608f0222d797fa = WFETrapDelay(state, tracer, s_24_0);
        // D s_24_2: write-var ga#4462 <= s_24_1
        fn_state.ga_4462 = s_24_1;
        // D s_24_3: read-var ga#4462.0:struct
        let s_24_3: bool = fn_state.ga_4462._0;
        // D s_24_4: read-var ga#4462.1:struct
        let s_24_4: i128 = fn_state.ga_4462._1;
        // D s_24_5: call WaitForEventUntilDelay(s_24_3, s_24_4)
        let s_24_5: bool = WaitForEventUntilDelay(state, tracer, s_24_3, s_24_4);
        // D s_24_6: not s_24_5
        let s_24_6: bool = !s_24_5;
        // N s_24_7: branch s_24_6 b26 b25
        if s_24_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var wfxtype:u32
        let s_26_0: u32 = fn_state.wfxtype;
        // D s_26_1: read-var target_el:u8
        let s_26_1: u8 = fn_state.target_el;
        // D s_26_2: call AArch64_WFxTrap(s_26_0, s_26_1)
        let s_26_2: () = AArch64_WFxTrap(state, tracer, s_26_0, s_26_1);
        // N s_26_3: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #16975u : u32
        let s_27_0: u32 = 16975;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 2u16);
        // C s_27_3: const #424u : u32
        let s_27_3: u32 = 424;
        // D s_27_4: read-reg s_27_3:u8
        let s_27_4: u8 = {
            let value = state.read_register::<u8>(s_27_3 as isize);
            tracer.read_register(s_27_3 as isize, value);
            value
        };
        // D s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 2u16);
        // D s_27_6: cmp-ne s_27_2 s_27_5
        let s_27_6: bool = ((s_27_2) != (s_27_5));
        // D s_27_7: write-var gs#6668 <= s_27_6
        fn_state.gs_6668 = s_27_6;
        // N s_27_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call HaveTWEDExt(s_28_0)
        let s_28_1: bool = HaveTWEDExt(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b30 b29
        if s_28_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #424u : u32
        let s_29_0: u32 = 424;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: read-var wfxtype:u32
        let s_29_2: u32 = fn_state.wfxtype;
        // D s_29_3: call AArch64_CheckForWFxTrap(s_29_1, s_29_2)
        let s_29_3: () = AArch64_CheckForWFxTrap(state, tracer, s_29_1, s_29_2);
        // N s_29_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #90704u : u32
        let s_30_0: u32 = 90704;
        // D s_30_1: read-reg s_30_0:struct
        let s_30_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call _get_SCR_EL3_Type_TWE(s_30_1)
        let s_30_2: bool = u_get_SCR_EL3_Type_TWE(state, tracer, s_30_1);
        // D s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // C s_30_4: const #1u : u8
        let s_30_4: bool = true;
        // C s_30_5: cast zx s_30_4 -> bv
        let s_30_5: Bits = Bits::new(s_30_4 as u128, 1u16);
        // D s_30_6: cmp-eq s_30_3 s_30_5
        let s_30_6: bool = ((s_30_3) == (s_30_5));
        // D s_30_7: write-var trap <= s_30_6
        fn_state.trap = s_30_6;
        // C s_30_8: const #424u : u32
        let s_30_8: u32 = 424;
        // D s_30_9: read-reg s_30_8:u8
        let s_30_9: u8 = {
            let value = state.read_register::<u8>(s_30_8 as isize);
            tracer.read_register(s_30_8 as isize, value);
            value
        };
        // D s_30_10: write-var target_el <= s_30_9
        fn_state.target_el = s_30_9;
        // N s_30_11: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #16975u : u32
        let s_31_0: u32 = 16975;
        // D s_31_1: read-reg s_31_0:u8
        let s_31_1: u8 = {
            let value = state.read_register::<u8>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: cast zx s_31_1 -> bv
        let s_31_2: Bits = Bits::new(s_31_1 as u128, 2u16);
        // C s_31_3: const #424u : u32
        let s_31_3: u32 = 424;
        // D s_31_4: read-reg s_31_3:u8
        let s_31_4: u8 = {
            let value = state.read_register::<u8>(s_31_3 as isize);
            tracer.read_register(s_31_3 as isize, value);
            value
        };
        // D s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 2u16);
        // D s_31_6: cmp-ne s_31_2 s_31_5
        let s_31_6: bool = ((s_31_2) != (s_31_5));
        // D s_31_7: write-var gs#6660 <= s_31_6
        fn_state.gs_6660 = s_31_6;
        // N s_31_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #424u : u32
        let s_32_0: u32 = 424;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // C s_32_2: const #2u : u8
        let s_32_2: u8 = 2;
        // D s_32_3: cmp-lt s_32_1 s_32_2
        let s_32_3: bool = ((s_32_1) < (s_32_2));
        // D s_32_4: write-var gs#6659 <= s_32_3
        fn_state.gs_6659 = s_32_3;
        // N s_32_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call HaveTWEDExt(s_33_0)
        let s_33_1: bool = HaveTWEDExt(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b35 b34
        if s_33_1 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #432u : u32
        let s_34_0: u32 = 432;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: read-var wfxtype:u32
        let s_34_2: u32 = fn_state.wfxtype;
        // D s_34_3: call AArch64_CheckForWFxTrap(s_34_1, s_34_2)
        let s_34_3: () = AArch64_CheckForWFxTrap(state, tracer, s_34_1, s_34_2);
        // N s_34_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #102552u : u32
        let s_35_0: u32 = 102552;
        // D s_35_1: read-reg s_35_0:struct
        let s_35_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call _get_HCR_EL2_Type_TWE(s_35_1)
        let s_35_2: bool = u_get_HCR_EL2_Type_TWE(state, tracer, s_35_1);
        // D s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // C s_35_4: const #1u : u8
        let s_35_4: bool = true;
        // C s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 1u16);
        // D s_35_6: cmp-eq s_35_3 s_35_5
        let s_35_6: bool = ((s_35_3) == (s_35_5));
        // D s_35_7: write-var trap <= s_35_6
        fn_state.trap = s_35_6;
        // C s_35_8: const #432u : u32
        let s_35_8: u32 = 432;
        // D s_35_9: read-reg s_35_8:u8
        let s_35_9: u8 = {
            let value = state.read_register::<u8>(s_35_8 as isize);
            tracer.read_register(s_35_8 as isize, value);
            value
        };
        // D s_35_10: write-var target_el <= s_35_9
        fn_state.target_el = s_35_9;
        // N s_35_11: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call IsInHost(s_36_0)
        let s_36_1: bool = IsInHost(state, tracer, s_36_0);
        // S s_36_2: not s_36_1
        let s_36_2: bool = !s_36_1;
        // D s_36_3: write-var gs#6658 <= s_36_2
        fn_state.gs_6658 = s_36_2;
        // N s_36_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call EL2Enabled(s_37_0)
        let s_37_1: bool = EL2Enabled(state, tracer, s_37_0);
        // D s_37_2: write-var gs#6657 <= s_37_1
        fn_state.gs_6657 = s_37_1;
        // N s_37_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #16975u : u32
        let s_38_0: u32 = 16975;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: cast zx s_38_1 -> bv
        let s_38_2: Bits = Bits::new(s_38_1 as u128, 2u16);
        // C s_38_3: const #448u : u32
        let s_38_3: u32 = 448;
        // D s_38_4: read-reg s_38_3:u8
        let s_38_4: u8 = {
            let value = state.read_register::<u8>(s_38_3 as isize);
            tracer.read_register(s_38_3 as isize, value);
            value
        };
        // D s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 2u16);
        // D s_38_6: cmp-eq s_38_2 s_38_5
        let s_38_6: bool = ((s_38_2) == (s_38_5));
        // N s_38_7: branch s_38_6 b41 b39
        if s_38_6 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #16975u : u32
        let s_39_0: u32 = 16975;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: u8 = {
            let value = state.read_register::<u8>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: cast zx s_39_1 -> bv
        let s_39_2: Bits = Bits::new(s_39_1 as u128, 2u16);
        // C s_39_3: const #440u : u32
        let s_39_3: u32 = 440;
        // D s_39_4: read-reg s_39_3:u8
        let s_39_4: u8 = {
            let value = state.read_register::<u8>(s_39_3 as isize);
            tracer.read_register(s_39_3 as isize, value);
            value
        };
        // D s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 2u16);
        // D s_39_6: cmp-eq s_39_2 s_39_5
        let s_39_6: bool = ((s_39_2) == (s_39_5));
        // D s_39_7: write-var gs#6655 <= s_39_6
        fn_state.gs_6655 = s_39_6;
        // N s_39_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#6655:u8
        let s_40_0: bool = fn_state.gs_6655;
        // D s_40_1: write-var gs#6656 <= s_40_0
        fn_state.gs_6656 = s_40_0;
        // N s_40_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#6655 <= s_41_0
        fn_state.gs_6655 = s_41_0;
        // N s_41_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call HaveTWEDExt(s_42_0)
        let s_42_1: bool = HaveTWEDExt(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b44 b43
        if s_42_1 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #440u : u32
        let s_43_0: u32 = 440;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: read-var wfxtype:u32
        let s_43_2: u32 = fn_state.wfxtype;
        // D s_43_3: call AArch64_CheckForWFxTrap(s_43_1, s_43_2)
        let s_43_3: () = AArch64_CheckForWFxTrap(state, tracer, s_43_1, s_43_2);
        // N s_43_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call SCTLR_read__1(s_44_0)
        let s_44_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_44_0);
        // S s_44_2: call _get_SCTLRType_nTWE(s_44_1)
        let s_44_2: bool = u_get_SCTLRType_nTWE(state, tracer, s_44_1);
        // S s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // C s_44_4: const #0u : u8
        let s_44_4: bool = false;
        // C s_44_5: cast zx s_44_4 -> bv
        let s_44_5: Bits = Bits::new(s_44_4 as u128, 1u16);
        // S s_44_6: cmp-eq s_44_3 s_44_5
        let s_44_6: bool = ((s_44_3) == (s_44_5));
        // D s_44_7: write-var trap <= s_44_6
        fn_state.trap = s_44_6;
        // C s_44_8: const #440u : u32
        let s_44_8: u32 = 440;
        // D s_44_9: read-reg s_44_8:u8
        let s_44_9: u8 = {
            let value = state.read_register::<u8>(s_44_8 as isize);
            tracer.read_register(s_44_8 as isize, value);
            value
        };
        // D s_44_10: write-var target_el <= s_44_9
        fn_state.target_el = s_44_9;
        // N s_44_11: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call EndOfInstruction(s_45_0)
        let s_45_1: () = EndOfInstruction(state, tracer, s_45_0);
        // N s_45_2: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var localtimeout:i
        let s_46_0: i128 = fn_state.localtimeout;
        // D s_46_1: call LocalTimeoutEvent(s_46_0)
        let s_46_1: bool = LocalTimeoutEvent(state, tracer, s_46_0);
        // D s_46_2: write-var gs#6654 <= s_46_1
        fn_state.gs_6654 = s_46_1;
        // N s_46_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call ClearEventRegister(s_47_0)
        let s_47_1: () = ClearEventRegister(state, tracer, s_47_0);
        // N s_47_2: return
        return;
    }
}
