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
use u_get_SCR_EL3_Type_EA::*;
use u_get_SCR_EL3_Type_NMEA::*;
use FailTransaction::*;
use IllegalExceptionReturn::*;
use ClearExclusiveLocal::*;
use ProcessorID::*;
use HaveBRBExt::*;
use HaveDoubleFaultExt::*;
use u_get_SCTLRType_IESB::*;
use SendEventLocal::*;
use HaveTME::*;
use HaveSME::*;
use BRBEExceptionReturn::*;
use AArch64_BranchAddr::*;
use u__UNKNOWN_bits::*;
use CheckExceptionCatch::*;
use HaveIESB::*;
use SetPSTATEFromPSR__1::*;
use ResetSVEState::*;
use SCTLR_read__1::*;
use UsingAArch32::*;
use SynchronizeContext::*;
use Bit::*;
use SynchronizeErrors::*;
use BranchTo::*;
use BranchToAddr::*;
use TakeUnmaskedPhysicalSErrorInterrupts::*;
use common::*;
pub fn AArch64_ExceptionReturn<T: Tracer>(
    state: &mut State,
    tracer: &T,
    new_pc_in: u64,
    spsr: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_26380: bool,
        gs_26385: bool,
        gs_26395: bool,
        sync_errors: bool,
        source_el: u8,
        gs_26393: bool,
        new_pc: u64,
        gs_26386: bool,
        new_pc_in: u64,
        spsr: u64,
    }
    let fn_state = FunctionState {
        new_pc_in,
        spsr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var new_pc_in:u64
        let s_0_0: u64 = fn_state.new_pc_in;
        // D s_0_1: write-var new_pc <= s_0_0
        fn_state.new_pc = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call HaveTME(s_0_2)
        let s_0_3: bool = HaveTME(state, tracer, s_0_2);
        // N s_0_4: branch s_0_3 b43 b1
        if s_0_3 {
            return block_43(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#26380 <= s_1_0
        fn_state.gs_26380 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#26380:u8
        let s_2_0: bool = fn_state.gs_26380;
        // N s_2_1: branch s_2_0 b42 b3
        if s_2_0 {
            return block_42(state, tracer, fn_state);
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
        // S s_4_1: call HaveIESB(s_4_0)
        let s_4_1: bool = HaveIESB(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b29 b5
        if s_4_1 {
            return block_29(state, tracer, fn_state);
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
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call SynchronizeContext(s_6_0)
        let s_6_1: () = SynchronizeContext(state, tracer, s_6_0);
        // C s_6_2: const #0u : u8
        let s_6_2: bool = false;
        // C s_6_3: const #22760u : u32
        let s_6_3: u32 = 22760;
        // N s_6_4: write-reg s_6_3 <= s_6_2
        let s_6_4: () = {
            state.write_register::<bool>(s_6_3 as isize, s_6_2);
            tracer.write_register(s_6_3 as isize, s_6_2);
        };
        // C s_6_5: const #16975u : u32
        let s_6_5: u32 = 16975;
        // D s_6_6: read-reg s_6_5:u8
        let s_6_6: u8 = {
            let value = state.read_register::<u8>(s_6_5 as isize);
            tracer.read_register(s_6_5 as isize, value);
            value
        };
        // D s_6_7: write-var source_el <= s_6_6
        fn_state.source_el = s_6_6;
        // D s_6_8: read-var spsr:u64
        let s_6_8: u64 = fn_state.spsr;
        // D s_6_9: cast zx s_6_8 -> bv
        let s_6_9: Bits = Bits::new(s_6_8 as u128, 64u16);
        // D s_6_10: call IllegalExceptionReturn(s_6_9)
        let s_6_10: bool = IllegalExceptionReturn(state, tracer, s_6_9);
        // D s_6_11: read-var spsr:u64
        let s_6_11: u64 = fn_state.spsr;
        // D s_6_12: cast zx s_6_11 -> bv
        let s_6_12: Bits = Bits::new(s_6_11 as u128, 64u16);
        // D s_6_13: call SetPSTATEFromPSR__1(s_6_12, s_6_10)
        let s_6_13: () = SetPSTATEFromPSR__1(state, tracer, s_6_12, s_6_10);
        // C s_6_14: const #() : ()
        let s_6_14: () = ();
        // S s_6_15: call ProcessorID(s_6_14)
        let s_6_15: i128 = ProcessorID(state, tracer, s_6_14);
        // S s_6_16: call ClearExclusiveLocal(s_6_15)
        let s_6_16: () = ClearExclusiveLocal(state, tracer, s_6_15);
        // C s_6_17: const #() : ()
        let s_6_17: () = ();
        // S s_6_18: call SendEventLocal(s_6_17)
        let s_6_18: () = SendEventLocal(state, tracer, s_6_17);
        // N s_6_19: branch s_6_10 b28 b7
        if s_6_10 {
            return block_28(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#26393 <= s_7_0
        fn_state.gs_26393 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#26393:u8
        let s_8_0: bool = fn_state.gs_26393;
        // N s_8_1: branch s_8_0 b27 b9
        if s_8_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call UsingAArch32(s_9_0)
        let s_9_1: bool = UsingAArch32(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b24 b10
        if s_9_1 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #16975u : u32
        let s_10_0: u32 = 16975;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: read-var new_pc:u64
        let s_10_2: u64 = fn_state.new_pc;
        // D s_10_3: call AArch64_BranchAddr(s_10_2, s_10_1)
        let s_10_3: u64 = AArch64_BranchAddr(state, tracer, s_10_2, s_10_1);
        // D s_10_4: write-var new_pc <= s_10_3
        fn_state.new_pc = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HaveBRBExt(s_11_0)
        let s_11_1: bool = HaveBRBExt(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b23 b12
        if s_11_1 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call UsingAArch32(s_13_0)
        let s_13_1: bool = UsingAArch32(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b16 b14
        if s_13_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var new_pc:u64
        let s_14_0: u64 = fn_state.new_pc;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 64u16);
        // C s_14_2: const #2u : u32
        let s_14_2: u32 = 2;
        // D s_14_3: call BranchToAddr(s_14_1, s_14_2)
        let s_14_3: () = BranchToAddr(state, tracer, s_14_1, s_14_2);
        // N s_14_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // S s_15_1: call CheckExceptionCatch(s_15_0)
        let s_15_1: () = CheckExceptionCatch(state, tracer, s_15_0);
        // N s_15_2: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call HaveSME(s_16_0)
        let s_16_1: bool = HaveSME(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b22 b17
        if s_16_1 {
            return block_22(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#26395 <= s_17_0
        fn_state.gs_26395 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#26395:u8
        let s_18_0: bool = fn_state.gs_26395;
        // N s_18_1: branch s_18_0 b21 b19
        if s_18_0 {
            return block_21(state, tracer, fn_state);
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
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // C s_20_1: const #0s : i
        let s_20_1: i128 = 0;
        // D s_20_2: read-var new_pc:u64
        let s_20_2: u64 = fn_state.new_pc;
        // D s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 64u16);
        // C s_20_4: const #1s : i64
        let s_20_4: i64 = 1;
        // C s_20_5: cast zx s_20_4 -> i
        let s_20_5: i128 = (i128::try_from(s_20_4).unwrap());
        // C s_20_6: const #31s : i
        let s_20_6: i128 = 31;
        // C s_20_7: add s_20_6 s_20_5
        let s_20_7: i128 = (s_20_6 + s_20_5);
        // D s_20_8: bit-extract s_20_3 s_20_1 s_20_7
        let s_20_8: Bits = (Bits::new(
            ((s_20_3) >> (s_20_1)).value(),
            u16::try_from(s_20_7).unwrap(),
        ));
        // D s_20_9: cast reint s_20_8 -> u32
        let s_20_9: u32 = (s_20_8.value() as u32);
        // D s_20_10: cast zx s_20_9 -> bv
        let s_20_10: Bits = Bits::new(s_20_9 as u128, 32u16);
        // C s_20_11: const #2u : u32
        let s_20_11: u32 = 2;
        // D s_20_12: call BranchTo(s_20_10, s_20_11, s_20_0)
        let s_20_12: () = BranchTo(state, tracer, s_20_10, s_20_11, s_20_0);
        // N s_20_13: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call ResetSVEState(s_21_0)
        let s_21_1: () = ResetSVEState(state, tracer, s_21_0);
        // N s_21_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #16989u : u32
        let s_22_0: u32 = 16989;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: bool = {
            let value = state.read_register::<bool>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 1u16);
        // C s_22_3: const #1u : u8
        let s_22_3: bool = true;
        // C s_22_4: cast zx s_22_3 -> bv
        let s_22_4: Bits = Bits::new(s_22_3 as u128, 1u16);
        // D s_22_5: cmp-eq s_22_2 s_22_4
        let s_22_5: bool = ((s_22_2) == (s_22_4));
        // D s_22_6: write-var gs#26395 <= s_22_5
        fn_state.gs_26395 = s_22_5;
        // N s_22_7: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var new_pc:u64
        let s_23_0: u64 = fn_state.new_pc;
        // D s_23_1: read-var source_el:u8
        let s_23_1: u8 = fn_state.source_el;
        // D s_23_2: call BRBEExceptionReturn(s_23_0, s_23_1)
        let s_23_2: () = BRBEExceptionReturn(state, tracer, s_23_0, s_23_1);
        // N s_23_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #16993u : u32
        let s_24_0: u32 = 16993;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: bool = {
            let value = state.read_register::<bool>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 1u16);
        // C s_24_3: const #1u : u8
        let s_24_3: bool = true;
        // C s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 1u16);
        // D s_24_5: cmp-eq s_24_2 s_24_4
        let s_24_5: bool = ((s_24_2) == (s_24_4));
        // N s_24_6: branch s_24_5 b26 b25
        if s_24_5 {
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
        // C s_25_0: const #0s : i
        let s_25_0: i128 = 0;
        // D s_25_1: read-var new_pc:u64
        let s_25_1: u64 = fn_state.new_pc;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 64u16);
        // C s_25_3: const #0u : u8
        let s_25_3: u8 = 0;
        // C s_25_4: cast zx s_25_3 -> bv
        let s_25_4: Bits = Bits::new(s_25_3 as u128, 2u16);
        // C s_25_5: const #1s : i
        let s_25_5: i128 = 1;
        // C s_25_6: const #1u : u64
        let s_25_6: u64 = 1;
        // C s_25_7: cast zx s_25_6 -> bv
        let s_25_7: Bits = Bits::new(s_25_6 as u128, 64u16);
        // C s_25_8: lsl s_25_7 s_25_5
        let s_25_8: Bits = s_25_7 << s_25_5;
        // C s_25_9: sub s_25_8 s_25_7
        let s_25_9: Bits = ((s_25_8) - (s_25_7));
        // C s_25_10: and s_25_4 s_25_9
        let s_25_10: Bits = ((s_25_4) & (s_25_9));
        // C s_25_11: lsl s_25_10 s_25_0
        let s_25_11: Bits = s_25_10 << s_25_0;
        // C s_25_12: lsl s_25_9 s_25_0
        let s_25_12: Bits = s_25_9 << s_25_0;
        // C s_25_13: cmpl s_25_12
        let s_25_13: Bits = !s_25_12;
        // D s_25_14: and s_25_2 s_25_13
        let s_25_14: Bits = ((s_25_2) & (s_25_13));
        // D s_25_15: or s_25_14 s_25_11
        let s_25_15: Bits = ((s_25_14) | (s_25_11));
        // D s_25_16: cast reint s_25_15 -> u64
        let s_25_16: u64 = (s_25_15.value() as u64);
        // D s_25_17: write-var new_pc <= s_25_16
        fn_state.new_pc = s_25_16;
        // N s_25_18: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // S s_26_1: call Bit(s_26_0)
        let s_26_1: bool = Bit(state, tracer, s_26_0);
        // C s_26_2: const #0s : i
        let s_26_2: i128 = 0;
        // D s_26_3: read-var new_pc:u64
        let s_26_3: u64 = fn_state.new_pc;
        // D s_26_4: cast zx s_26_3 -> bv
        let s_26_4: Bits = Bits::new(s_26_3 as u128, 64u16);
        // C s_26_5: const #1u : u64
        let s_26_5: u64 = 1;
        // D s_26_6: bit-insert s_26_4 s_26_4 s_26_2 s_26_5
        let s_26_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_26_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_26_4.length(),
            );
            (s_26_4 & mask) | (s_26_4 << s_26_2)
        };
        // D s_26_7: cast reint s_26_6 -> u64
        let s_26_7: u64 = (s_26_6.value() as u64);
        // D s_26_8: write-var new_pc <= s_26_7
        fn_state.new_pc = s_26_7;
        // N s_26_9: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #32s : i64
        let s_27_0: i64 = 32;
        // C s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // S s_27_2: call __UNKNOWN_bits(s_27_1)
        let s_27_2: Bits = u__UNKNOWN_bits(state, tracer, s_27_1);
        // S s_27_3: cast reint s_27_2 -> u32
        let s_27_3: u32 = (s_27_2.value() as u32);
        // C s_27_4: const #32s : i
        let s_27_4: i128 = 32;
        // D s_27_5: read-var new_pc:u64
        let s_27_5: u64 = fn_state.new_pc;
        // D s_27_6: cast zx s_27_5 -> bv
        let s_27_6: Bits = Bits::new(s_27_5 as u128, 64u16);
        // S s_27_7: cast zx s_27_3 -> bv
        let s_27_7: Bits = Bits::new(s_27_3 as u128, 32u16);
        // C s_27_8: const #31s : i
        let s_27_8: i128 = 31;
        // C s_27_9: const #1u : u64
        let s_27_9: u64 = 1;
        // C s_27_10: cast zx s_27_9 -> bv
        let s_27_10: Bits = Bits::new(s_27_9 as u128, 64u16);
        // C s_27_11: lsl s_27_10 s_27_8
        let s_27_11: Bits = s_27_10 << s_27_8;
        // C s_27_12: sub s_27_11 s_27_10
        let s_27_12: Bits = ((s_27_11) - (s_27_10));
        // S s_27_13: and s_27_7 s_27_12
        let s_27_13: Bits = ((s_27_7) & (s_27_12));
        // S s_27_14: lsl s_27_13 s_27_4
        let s_27_14: Bits = s_27_13 << s_27_4;
        // C s_27_15: lsl s_27_12 s_27_4
        let s_27_15: Bits = s_27_12 << s_27_4;
        // C s_27_16: cmpl s_27_15
        let s_27_16: Bits = !s_27_15;
        // D s_27_17: and s_27_6 s_27_16
        let s_27_17: Bits = ((s_27_6) & (s_27_16));
        // D s_27_18: or s_27_17 s_27_14
        let s_27_18: Bits = ((s_27_17) | (s_27_14));
        // D s_27_19: cast reint s_27_18 -> u64
        let s_27_19: u64 = (s_27_18.value() as u64);
        // D s_27_20: write-var new_pc <= s_27_19
        fn_state.new_pc = s_27_19;
        // C s_27_21: const #2s : i64
        let s_27_21: i64 = 2;
        // C s_27_22: cast zx s_27_21 -> i
        let s_27_22: i128 = (i128::try_from(s_27_21).unwrap());
        // S s_27_23: call __UNKNOWN_bits(s_27_22)
        let s_27_23: Bits = u__UNKNOWN_bits(state, tracer, s_27_22);
        // S s_27_24: cast reint s_27_23 -> u8
        let s_27_24: u8 = (s_27_23.value() as u8);
        // C s_27_25: const #0s : i
        let s_27_25: i128 = 0;
        // D s_27_26: read-var new_pc:u64
        let s_27_26: u64 = fn_state.new_pc;
        // D s_27_27: cast zx s_27_26 -> bv
        let s_27_27: Bits = Bits::new(s_27_26 as u128, 64u16);
        // S s_27_28: cast zx s_27_24 -> bv
        let s_27_28: Bits = Bits::new(s_27_24 as u128, 2u16);
        // C s_27_29: const #1s : i
        let s_27_29: i128 = 1;
        // C s_27_30: const #1u : u64
        let s_27_30: u64 = 1;
        // C s_27_31: cast zx s_27_30 -> bv
        let s_27_31: Bits = Bits::new(s_27_30 as u128, 64u16);
        // C s_27_32: lsl s_27_31 s_27_29
        let s_27_32: Bits = s_27_31 << s_27_29;
        // C s_27_33: sub s_27_32 s_27_31
        let s_27_33: Bits = ((s_27_32) - (s_27_31));
        // S s_27_34: and s_27_28 s_27_33
        let s_27_34: Bits = ((s_27_28) & (s_27_33));
        // S s_27_35: lsl s_27_34 s_27_25
        let s_27_35: Bits = s_27_34 << s_27_25;
        // C s_27_36: lsl s_27_33 s_27_25
        let s_27_36: Bits = s_27_33 << s_27_25;
        // C s_27_37: cmpl s_27_36
        let s_27_37: Bits = !s_27_36;
        // D s_27_38: and s_27_27 s_27_37
        let s_27_38: Bits = ((s_27_27) & (s_27_37));
        // D s_27_39: or s_27_38 s_27_35
        let s_27_39: Bits = ((s_27_38) | (s_27_35));
        // D s_27_40: cast reint s_27_39 -> u64
        let s_27_40: u64 = (s_27_39.value() as u64);
        // D s_27_41: write-var new_pc <= s_27_40
        fn_state.new_pc = s_27_40;
        // N s_27_42: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #4s : i
        let s_28_0: i128 = 4;
        // D s_28_1: read-var spsr:u64
        let s_28_1: u64 = fn_state.spsr;
        // D s_28_2: cast zx s_28_1 -> bv
        let s_28_2: Bits = Bits::new(s_28_1 as u128, 64u16);
        // C s_28_3: const #1u : u64
        let s_28_3: u64 = 1;
        // D s_28_4: bit-extract s_28_2 s_28_0 s_28_3
        let s_28_4: Bits = (Bits::new(
            ((s_28_2) >> (s_28_0)).value(),
            u16::try_from(s_28_3).unwrap(),
        ));
        // D s_28_5: cast reint s_28_4 -> u8
        let s_28_5: bool = ((s_28_4.value()) != 0);
        // C s_28_6: const #0s : i
        let s_28_6: i128 = 0;
        // C s_28_7: const #0u : u64
        let s_28_7: u64 = 0;
        // D s_28_8: cast zx s_28_5 -> u64
        let s_28_8: u64 = (s_28_5 as u64);
        // C s_28_9: const #1u : u64
        let s_28_9: u64 = 1;
        // D s_28_10: and s_28_8 s_28_9
        let s_28_10: u64 = ((s_28_8) & (s_28_9));
        // D s_28_11: cmp-eq s_28_10 s_28_9
        let s_28_11: bool = ((s_28_10) == (s_28_9));
        // D s_28_12: lsl s_28_8 s_28_6
        let s_28_12: u64 = s_28_8 << s_28_6;
        // D s_28_13: or s_28_7 s_28_12
        let s_28_13: u64 = ((s_28_7) | (s_28_12));
        // D s_28_14: cmpl s_28_12
        let s_28_14: u64 = !s_28_12;
        // D s_28_15: and s_28_7 s_28_14
        let s_28_15: u64 = ((s_28_7) & (s_28_14));
        // D s_28_16: select s_28_11 s_28_13 s_28_15
        let s_28_16: u64 = if s_28_11 { s_28_13 } else { s_28_15 };
        // D s_28_17: cast trunc s_28_16 -> u8
        let s_28_17: bool = ((s_28_16) != 0);
        // D s_28_18: cast zx s_28_17 -> bv
        let s_28_18: Bits = Bits::new(s_28_17 as u128, 1u16);
        // C s_28_19: const #1u : u8
        let s_28_19: bool = true;
        // C s_28_20: cast zx s_28_19 -> bv
        let s_28_20: Bits = Bits::new(s_28_19 as u128, 1u16);
        // D s_28_21: cmp-eq s_28_18 s_28_20
        let s_28_21: bool = ((s_28_18) == (s_28_20));
        // D s_28_22: write-var gs#26393 <= s_28_21
        fn_state.gs_26393 = s_28_21;
        // N s_28_23: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call SCTLR_read__1(s_29_0)
        let s_29_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_29_0);
        // S s_29_2: call _get_SCTLRType_IESB(s_29_1)
        let s_29_2: bool = u_get_SCTLRType_IESB(state, tracer, s_29_1);
        // S s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // C s_29_4: const #1u : u8
        let s_29_4: bool = true;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 1u16);
        // S s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // D s_29_7: write-var sync_errors <= s_29_6
        fn_state.sync_errors = s_29_6;
        // C s_29_8: const #() : ()
        let s_29_8: () = ();
        // S s_29_9: call HaveDoubleFaultExt(s_29_8)
        let s_29_9: bool = HaveDoubleFaultExt(state, tracer, s_29_8);
        // N s_29_10: branch s_29_9 b35 b30
        if s_29_9 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var sync_errors:u8
        let s_31_0: bool = fn_state.sync_errors;
        // N s_31_1: branch s_31_0 b34 b32
        if s_31_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call SynchronizeErrors(s_34_0)
        let s_34_1: () = SynchronizeErrors(state, tracer, s_34_0);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // S s_34_3: call TakeUnmaskedPhysicalSErrorInterrupts(s_34_2)
        let s_34_3: () = TakeUnmaskedPhysicalSErrorInterrupts(state, tracer, s_34_2);
        // N s_34_4: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var sync_errors:u8
        let s_35_0: bool = fn_state.sync_errors;
        // N s_35_1: branch s_35_0 b41 b36
        if s_35_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #90704u : u32
        let s_36_0: u32 = 90704;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_SCR_EL3_Type_EA(s_36_1)
        let s_36_2: bool = u_get_SCR_EL3_Type_EA(state, tracer, s_36_1);
        // C s_36_3: const #90704u : u32
        let s_36_3: u32 = 90704;
        // D s_36_4: read-reg s_36_3:struct
        let s_36_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_3 as isize);
            tracer.read_register(s_36_3 as isize, value);
            value
        };
        // D s_36_5: call _get_SCR_EL3_Type_NMEA(s_36_4)
        let s_36_5: bool = u_get_SCR_EL3_Type_NMEA(state, tracer, s_36_4);
        // D s_36_6: cast zx s_36_2 -> bv
        let s_36_6: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_7: cast zx s_36_5 -> bv
        let s_36_7: Bits = Bits::new(s_36_5 as u128, 1u16);
        // D s_36_8: cast reint s_36_6 -> u128
        let s_36_8: u128 = (s_36_6.value() as u128);
        // D s_36_9: size-of s_36_6
        let s_36_9: u16 = s_36_6.length();
        // D s_36_10: cast reint s_36_7 -> u128
        let s_36_10: u128 = (s_36_7.value() as u128);
        // D s_36_11: size-of s_36_7
        let s_36_11: u16 = s_36_7.length();
        // D s_36_12: lsl s_36_8 s_36_11
        let s_36_12: u128 = s_36_8 << s_36_11;
        // D s_36_13: or s_36_12 s_36_10
        let s_36_13: u128 = ((s_36_12) | (s_36_10));
        // D s_36_14: add s_36_9 s_36_11
        let s_36_14: u16 = (s_36_9 + s_36_11);
        // D s_36_15: create-bits s_36_13 s_36_14
        let s_36_15: Bits = Bits::new(s_36_13, s_36_14);
        // D s_36_16: cast reint s_36_15 -> u8
        let s_36_16: u8 = (s_36_15.value() as u8);
        // D s_36_17: cast zx s_36_16 -> bv
        let s_36_17: Bits = Bits::new(s_36_16 as u128, 2u16);
        // C s_36_18: const #3u : u8
        let s_36_18: u8 = 3;
        // C s_36_19: cast zx s_36_18 -> bv
        let s_36_19: Bits = Bits::new(s_36_18 as u128, 2u16);
        // D s_36_20: cmp-eq s_36_17 s_36_19
        let s_36_20: bool = ((s_36_17) == (s_36_19));
        // N s_36_21: branch s_36_20 b40 b37
        if s_36_20 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#26385 <= s_37_0
        fn_state.gs_26385 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#26385:u8
        let s_38_0: bool = fn_state.gs_26385;
        // D s_38_1: write-var gs#26386 <= s_38_0
        fn_state.gs_26386 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#26386:u8
        let s_39_0: bool = fn_state.gs_26386;
        // D s_39_1: write-var sync_errors <= s_39_0
        fn_state.sync_errors = s_39_0;
        // N s_39_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #16975u : u32
        let s_40_0: u32 = 16975;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: cast zx s_40_1 -> bv
        let s_40_2: Bits = Bits::new(s_40_1 as u128, 2u16);
        // C s_40_3: const #424u : u32
        let s_40_3: u32 = 424;
        // D s_40_4: read-reg s_40_3:u8
        let s_40_4: u8 = {
            let value = state.read_register::<u8>(s_40_3 as isize);
            tracer.read_register(s_40_3 as isize, value);
            value
        };
        // D s_40_5: cast zx s_40_4 -> bv
        let s_40_5: Bits = Bits::new(s_40_4 as u128, 2u16);
        // D s_40_6: cmp-eq s_40_2 s_40_5
        let s_40_6: bool = ((s_40_2) == (s_40_5));
        // D s_40_7: write-var gs#26385 <= s_40_6
        fn_state.gs_26385 = s_40_6;
        // N s_40_8: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#26386 <= s_41_0
        fn_state.gs_26386 = s_41_0;
        // N s_41_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #2u : u32
        let s_42_0: u32 = 2;
        // C s_42_1: const #0u : u8
        let s_42_1: bool = false;
        // S s_42_2: call FailTransaction(s_42_0, s_42_1)
        let s_42_2: () = FailTransaction(state, tracer, s_42_0, s_42_1);
        // N s_42_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #100180u : u32
        let s_43_0: u32 = 100180;
        // D s_43_1: read-reg s_43_0:i
        let s_43_1: i128 = {
            let value = state.read_register::<i128>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // C s_43_2: const #0s : i
        let s_43_2: i128 = 0;
        // D s_43_3: cmp-gt s_43_1 s_43_2
        let s_43_3: bool = ((s_43_1) > (s_43_2));
        // D s_43_4: write-var gs#26380 <= s_43_3
        fn_state.gs_26380 = s_43_3;
        // N s_43_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
