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
use WaitForInterrupt::*;
use HaveFeatWFxT::*;
use IsInHost::*;
use FailTransaction::*;
use EndOfInstruction::*;
use AArch64_CheckForWFxTrap::*;
use LocalTimeoutEvent::*;
use EL2Enabled::*;
use HaveTME::*;
use InterruptPending::*;
use common::*;
pub fn Hint_WFI<T: Tracer>(
    state: &mut State,
    tracer: &T,
    localtimeout: i128,
    wfxtype: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_6712: bool,
        gs_6713: bool,
        gs_6711: bool,
        gs_6714: bool,
        gs_6710: bool,
        gs_6709: bool,
        gs_6707: bool,
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
        // S s_0_1: call HaveTME(s_0_0)
        let s_0_1: bool = HaveTME(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b35 b1
        if s_0_1 {
            return block_35(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#6707 <= s_1_0
        fn_state.gs_6707 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#6707:u8
        let s_2_0: bool = fn_state.gs_6707;
        // N s_2_1: branch s_2_0 b34 b3
        if s_2_0 {
            return block_34(state, tracer, fn_state);
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
        // S s_4_1: call InterruptPending(s_4_0)
        let s_4_1: bool = InterruptPending(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b33 b5
        if s_4_1 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call HaveFeatWFxT(s_5_0)
        let s_5_1: bool = HaveFeatWFxT(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b32 b6
        if s_5_1 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#6709 <= s_6_0
        fn_state.gs_6709 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#6709:u8
        let s_7_0: bool = fn_state.gs_6709;
        // D s_7_1: write-var gs#6710 <= s_7_0
        fn_state.gs_6710 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#6710:u8
        let s_8_0: bool = fn_state.gs_6710;
        // N s_8_1: branch s_8_0 b31 b9
        if s_8_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #16975u : u32
        let s_9_0: u32 = 16975;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 2u16);
        // C s_9_3: const #448u : u32
        let s_9_3: u32 = 448;
        // D s_9_4: read-reg s_9_3:u8
        let s_9_4: u8 = {
            let value = state.read_register::<u8>(s_9_3 as isize);
            tracer.read_register(s_9_3 as isize, value);
            value
        };
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 2u16);
        // D s_9_6: cmp-eq s_9_2 s_9_5
        let s_9_6: bool = ((s_9_2) == (s_9_5));
        // N s_9_7: branch s_9_6 b30 b10
        if s_9_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #16975u : u32
        let s_11_0: u32 = 16975;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 2u16);
        // C s_11_3: const #448u : u32
        let s_11_3: u32 = 448;
        // D s_11_4: read-reg s_11_3:u8
        let s_11_4: u8 = {
            let value = state.read_register::<u8>(s_11_3 as isize);
            tracer.read_register(s_11_3 as isize, value);
            value
        };
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 2u16);
        // D s_11_6: cmp-eq s_11_2 s_11_5
        let s_11_6: bool = ((s_11_2) == (s_11_5));
        // N s_11_7: branch s_11_6 b29 b12
        if s_11_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #16975u : u32
        let s_12_0: u32 = 16975;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 2u16);
        // C s_12_3: const #440u : u32
        let s_12_3: u32 = 440;
        // D s_12_4: read-reg s_12_3:u8
        let s_12_4: u8 = {
            let value = state.read_register::<u8>(s_12_3 as isize);
            tracer.read_register(s_12_3 as isize, value);
            value
        };
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 2u16);
        // D s_12_6: cmp-eq s_12_2 s_12_5
        let s_12_6: bool = ((s_12_2) == (s_12_5));
        // D s_12_7: write-var gs#6711 <= s_12_6
        fn_state.gs_6711 = s_12_6;
        // N s_12_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#6711:u8
        let s_13_0: bool = fn_state.gs_6711;
        // N s_13_1: branch s_13_0 b28 b14
        if s_13_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#6712 <= s_14_0
        fn_state.gs_6712 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#6712:u8
        let s_15_0: bool = fn_state.gs_6712;
        // N s_15_1: branch s_15_0 b27 b16
        if s_15_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#6713 <= s_16_0
        fn_state.gs_6713 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#6713:u8
        let s_17_0: bool = fn_state.gs_6713;
        // N s_17_1: branch s_17_0 b26 b18
        if s_17_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #424u : u32
        let s_19_0: u32 = 424;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // C s_19_2: const #2u : u8
        let s_19_2: u8 = 2;
        // D s_19_3: cmp-lt s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) < (s_19_2));
        // N s_19_4: branch s_19_3 b25 b20
        if s_19_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#6714 <= s_20_0
        fn_state.gs_6714 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#6714:u8
        let s_21_0: bool = fn_state.gs_6714;
        // N s_21_1: branch s_21_0 b24 b22
        if s_21_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var localtimeout:i
        let s_23_0: i128 = fn_state.localtimeout;
        // D s_23_1: call WaitForInterrupt(s_23_0)
        let s_23_1: () = WaitForInterrupt(state, tracer, s_23_0);
        // N s_23_2: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #424u : u32
        let s_24_0: u32 = 424;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: read-var wfxtype:u32
        let s_24_2: u32 = fn_state.wfxtype;
        // D s_24_3: call AArch64_CheckForWFxTrap(s_24_1, s_24_2)
        let s_24_3: () = AArch64_CheckForWFxTrap(state, tracer, s_24_1, s_24_2);
        // N s_24_4: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #16975u : u32
        let s_25_0: u32 = 16975;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 2u16);
        // C s_25_3: const #424u : u32
        let s_25_3: u32 = 424;
        // D s_25_4: read-reg s_25_3:u8
        let s_25_4: u8 = {
            let value = state.read_register::<u8>(s_25_3 as isize);
            tracer.read_register(s_25_3 as isize, value);
            value
        };
        // D s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 2u16);
        // D s_25_6: cmp-ne s_25_2 s_25_5
        let s_25_6: bool = ((s_25_2) != (s_25_5));
        // D s_25_7: write-var gs#6714 <= s_25_6
        fn_state.gs_6714 = s_25_6;
        // N s_25_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #432u : u32
        let s_26_0: u32 = 432;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: u8 = {
            let value = state.read_register::<u8>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: read-var wfxtype:u32
        let s_26_2: u32 = fn_state.wfxtype;
        // D s_26_3: call AArch64_CheckForWFxTrap(s_26_1, s_26_2)
        let s_26_3: () = AArch64_CheckForWFxTrap(state, tracer, s_26_1, s_26_2);
        // N s_26_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call IsInHost(s_27_0)
        let s_27_1: bool = IsInHost(state, tracer, s_27_0);
        // S s_27_2: not s_27_1
        let s_27_2: bool = !s_27_1;
        // D s_27_3: write-var gs#6713 <= s_27_2
        fn_state.gs_6713 = s_27_2;
        // N s_27_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EL2Enabled(s_28_0)
        let s_28_1: bool = EL2Enabled(state, tracer, s_28_0);
        // D s_28_2: write-var gs#6712 <= s_28_1
        fn_state.gs_6712 = s_28_1;
        // N s_28_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#6711 <= s_29_0
        fn_state.gs_6711 = s_29_0;
        // N s_29_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #440u : u32
        let s_30_0: u32 = 440;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: read-var wfxtype:u32
        let s_30_2: u32 = fn_state.wfxtype;
        // D s_30_3: call AArch64_CheckForWFxTrap(s_30_1, s_30_2)
        let s_30_3: () = AArch64_CheckForWFxTrap(state, tracer, s_30_1, s_30_2);
        // N s_30_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call EndOfInstruction(s_31_0)
        let s_31_1: () = EndOfInstruction(state, tracer, s_31_0);
        // N s_31_2: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var localtimeout:i
        let s_32_0: i128 = fn_state.localtimeout;
        // D s_32_1: call LocalTimeoutEvent(s_32_0)
        let s_32_1: bool = LocalTimeoutEvent(state, tracer, s_32_0);
        // D s_32_2: write-var gs#6709 <= s_32_1
        fn_state.gs_6709 = s_32_1;
        // N s_32_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#6710 <= s_33_0
        fn_state.gs_6710 = s_33_0;
        // N s_33_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #2u : u32
        let s_34_0: u32 = 2;
        // C s_34_1: const #0u : u8
        let s_34_1: bool = false;
        // S s_34_2: call FailTransaction(s_34_0, s_34_1)
        let s_34_2: () = FailTransaction(state, tracer, s_34_0, s_34_1);
        // N s_34_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #100180u : u32
        let s_35_0: u32 = 100180;
        // D s_35_1: read-reg s_35_0:i
        let s_35_1: i128 = {
            let value = state.read_register::<i128>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // C s_35_2: const #0s : i
        let s_35_2: i128 = 0;
        // D s_35_3: cmp-gt s_35_1 s_35_2
        let s_35_3: bool = ((s_35_1) > (s_35_2));
        // D s_35_4: write-var gs#6707 <= s_35_3
        fn_state.gs_6707 = s_35_3;
        // N s_35_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
