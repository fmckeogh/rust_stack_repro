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
use AArch64_ReportException::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_HCR_EL2_Type_NV1::*;
use BRBEException::*;
use u_get_HCR_EL2_Type_NV2::*;
use Halted::*;
use IsInHost::*;
use FailTransaction::*;
use u_get_SCTLRType_IESB::*;
use InsertIESBBeforeException::*;
use u_get_SCR_EL3_Type_EA::*;
use AArch64_TakeExceptionInDebugState::*;
use HaveGCS::*;
use HaveTME::*;
use HaveNV2Ext::*;
use HaveSME::*;
use SCTLR_read__1::*;
use ResetSVEState::*;
use EndOfInstruction::*;
use u_get_SCR_EL3_Type_NMEA::*;
use HaveDoubleFaultExt::*;
use EL2Enabled::*;
use TakeUnmaskedPhysicalSErrorInterrupts::*;
use AArch64_MaybeZeroRegisterUppers::*;
use MaybeZeroSVEUppers::*;
use HaveNVExt::*;
use ConstrainUnpredictableBool::*;
use SCTLR_read::*;
use HaveBRBExt::*;
use u_get_SCTLRType_DSSBS::*;
use HaveFeatNMI::*;
use u_get_SCTLRType_SPAN::*;
use HaveBTIExt::*;
use ELIsInHost::*;
use HavePANExt::*;
use VBAR_read__1::*;
use GetCurrentEXLOCKEN::*;
use ELR_set__1::*;
use GetPSRFromPSTATE::*;
use SPSR_set::*;
use VBAR_read::*;
use HaveUAOExt::*;
use HaveIESB::*;
use HaveSSBSExt::*;
use HaveMTEExt::*;
use integer_subrange::*;
use UsingAArch32::*;
use SynchronizeContext::*;
use CheckExceptionCatch::*;
use SynchronizeErrors::*;
use BranchTo::*;
use u_get_SCTLRType_SPINTMASK::*;
use ELUsingAArch32::*;
use common::*;
pub fn AArch64_TakeException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_el: u8,
    exception_in: ProductTypeb7f99f96751e17c4,
    preferred_exception_return: u64,
    vect_offset_in: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_6447: bool,
        from_32: bool,
        lower_32: bool,
        gs_6444: bool,
        ga_4212: i128,
        gs_6390: bool,
        gs_6449: bool,
        gs_6448: bool,
        gs_6401: bool,
        gs_6397: bool,
        gs_6459: bool,
        gs_6394: bool,
        gs_6395: bool,
        zero_btype: bool,
        gs_6446: bool,
        gs_6363: bool,
        except: ProductTypeb7f99f96751e17c4,
        gs_6398: bool,
        gs_6443: bool,
        gs_6384: bool,
        vect_offset: i128,
        gs_6458: bool,
        gs_6357: bool,
        gs_6358: bool,
        vect_offsetshadow_76: i128,
        gs_6450: bool,
        gs_6389: bool,
        ga_4197: u32,
        gs_6374: bool,
        gs_6416: bool,
        gs_6381: bool,
        spsr: u64,
        cause: u32,
        gs_6442: bool,
        gs_6451: bool,
        gs_6445: bool,
        gs_6414: bool,
        gs_6413: bool,
        sync_errors: bool,
        gs_6383: bool,
        gs_6457: bool,
        gs_6380: bool,
        gs_6415: bool,
        gs_6399: bool,
        gs_6396: bool,
        target_el: u8,
        exception_in: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u64,
        vect_offset_in: i128,
    }
    let fn_state = FunctionState {
        target_el,
        exception_in,
        preferred_exception_return,
        vect_offset_in,
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
        // N s_0_3: branch s_0_2 b204 b1
        if s_0_2 {
            return block_204(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#6357 <= s_1_0
        fn_state.gs_6357 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#6357:u8
        let s_2_0: bool = fn_state.gs_6357;
        // N s_2_1: branch s_2_0 b203 b3
        if s_2_0 {
            return block_203(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#6358 <= s_3_0
        fn_state.gs_6358 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#6358:u8
        let s_4_0: bool = fn_state.gs_6358;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #() : ()
        let s_4_2: () = ();
        // S s_4_3: call Halted(s_4_2)
        let s_4_3: bool = Halted(state, tracer, s_4_2);
        // N s_4_4: branch s_4_3 b202 b5
        if s_4_3 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var exception_in:struct
        let s_5_0: ProductTypeb7f99f96751e17c4 = fn_state.exception_in;
        // D s_5_1: write-var except <= s_5_0
        fn_state.except = s_5_0;
        // C s_5_2: const #() : ()
        let s_5_2: () = ();
        // S s_5_3: call HaveIESB(s_5_2)
        let s_5_3: bool = HaveIESB(state, tracer, s_5_2);
        // N s_5_4: branch s_5_3 b186 b6
        if s_5_3 {
            return block_186(state, tracer, fn_state);
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
        // D s_6_1: write-var sync_errors <= s_6_0
        fn_state.sync_errors = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call HaveTME(s_7_0)
        let s_7_1: bool = HaveTME(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b185 b8
        if s_7_1 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#6363 <= s_8_0
        fn_state.gs_6363 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#6363:u8
        let s_9_0: bool = fn_state.gs_6363;
        // N s_9_1: branch s_9_0 b175 b10
        if s_9_0 {
            return block_175(state, tracer, fn_state);
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
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call SynchronizeContext(s_11_0)
        let s_11_1: () = SynchronizeContext(state, tracer, s_11_0);
        // C s_11_2: const #() : ()
        let s_11_2: () = ();
        // S s_11_3: call UsingAArch32(s_11_2)
        let s_11_3: bool = UsingAArch32(state, tracer, s_11_2);
        // D s_11_4: write-var from_32 <= s_11_3
        fn_state.from_32 = s_11_3;
        // D s_11_5: read-var from_32:u8
        let s_11_5: bool = fn_state.from_32;
        // N s_11_6: branch s_11_5 b174 b12
        if s_11_5 {
            return block_174(state, tracer, fn_state);
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
        // D s_13_0: read-var from_32:u8
        let s_13_0: bool = fn_state.from_32;
        // N s_13_1: branch s_13_0 b173 b14
        if s_13_0 {
            return block_173(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#6383 <= s_14_0
        fn_state.gs_6383 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#6383:u8
        let s_15_0: bool = fn_state.gs_6383;
        // N s_15_1: branch s_15_0 b172 b16
        if s_15_0 {
            return block_172(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#6384 <= s_16_0
        fn_state.gs_6384 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#6384:u8
        let s_17_0: bool = fn_state.gs_6384;
        // N s_17_1: branch s_17_0 b171 b18
        if s_17_0 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var target_el:u8
        let s_18_0: u8 = fn_state.target_el;
        // D s_18_1: call MaybeZeroSVEUppers(s_18_0)
        let s_18_1: () = MaybeZeroSVEUppers(state, tracer, s_18_0);
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var vect_offset_in:i
        let s_19_0: i128 = fn_state.vect_offset_in;
        // D s_19_1: write-var vect_offset <= s_19_0
        fn_state.vect_offset = s_19_0;
        // D s_19_2: read-var target_el:u8
        let s_19_2: u8 = fn_state.target_el;
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 2u16);
        // D s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (s_19_3.value() as i128);
        // D s_19_5: cast reint s_19_4 -> i64
        let s_19_5: i64 = (s_19_4 as i64);
        // C s_19_6: const #16975u : u32
        let s_19_6: u32 = 16975;
        // D s_19_7: read-reg s_19_6:u8
        let s_19_7: u8 = {
            let value = state.read_register::<u8>(s_19_6 as isize);
            tracer.read_register(s_19_6 as isize, value);
            value
        };
        // D s_19_8: cast zx s_19_7 -> bv
        let s_19_8: Bits = Bits::new(s_19_7 as u128, 2u16);
        // D s_19_9: cast zx s_19_8 -> i
        let s_19_9: i128 = (s_19_8.value() as i128);
        // D s_19_10: cast reint s_19_9 -> i64
        let s_19_10: i64 = (s_19_9 as i64);
        // D s_19_11: cast zx s_19_5 -> i
        let s_19_11: i128 = (i128::try_from(s_19_5).unwrap());
        // D s_19_12: cast zx s_19_10 -> i
        let s_19_12: i128 = (i128::try_from(s_19_10).unwrap());
        // D s_19_13: cmp-gt s_19_11 s_19_12
        let s_19_13: bool = ((s_19_11) > (s_19_12));
        // N s_19_14: branch s_19_13 b154 b20
        if s_19_13 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #16990u : u32
        let s_20_0: u32 = 16990;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: bool = {
            let value = state.read_register::<bool>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: cast zx s_20_1 -> bv
        let s_20_2: Bits = Bits::new(s_20_1 as u128, 1u16);
        // C s_20_3: const #1u : u8
        let s_20_3: bool = true;
        // C s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 1u16);
        // D s_20_5: cmp-eq s_20_2 s_20_4
        let s_20_5: bool = ((s_20_2) == (s_20_4));
        // N s_20_6: branch s_20_5 b153 b21
        if s_20_5 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var vect_offset:i
        let s_22_0: i128 = fn_state.vect_offset;
        // D s_22_1: write-var vect_offsetshadow#76 <= s_22_0
        fn_state.vect_offsetshadow_76 = s_22_0;
        // C s_22_2: const #64s : i64
        let s_22_2: i64 = 64;
        // C s_22_3: const #1u : u32
        let s_22_3: u32 = 1;
        // S s_22_4: call GetPSRFromPSTATE(s_22_3, s_22_2)
        let s_22_4: Bits = GetPSRFromPSTATE(state, tracer, s_22_3, s_22_2);
        // S s_22_5: cast reint s_22_4 -> u64
        let s_22_5: u64 = (s_22_4.value() as u64);
        // D s_22_6: write-var spsr <= s_22_5
        fn_state.spsr = s_22_5;
        // C s_22_7: const #16975u : u32
        let s_22_7: u32 = 16975;
        // D s_22_8: read-reg s_22_7:u8
        let s_22_8: u8 = {
            let value = state.read_register::<u8>(s_22_7 as isize);
            tracer.read_register(s_22_7 as isize, value);
            value
        };
        // D s_22_9: cast zx s_22_8 -> bv
        let s_22_9: Bits = Bits::new(s_22_8 as u128, 2u16);
        // C s_22_10: const #440u : u32
        let s_22_10: u32 = 440;
        // D s_22_11: read-reg s_22_10:u8
        let s_22_11: u8 = {
            let value = state.read_register::<u8>(s_22_10 as isize);
            tracer.read_register(s_22_10 as isize, value);
            value
        };
        // D s_22_12: cast zx s_22_11 -> bv
        let s_22_12: Bits = Bits::new(s_22_11 as u128, 2u16);
        // D s_22_13: cmp-eq s_22_9 s_22_12
        let s_22_13: bool = ((s_22_9) == (s_22_12));
        // N s_22_14: branch s_22_13 b152 b23
        if s_22_13 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#6394 <= s_23_0
        fn_state.gs_6394 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#6394:u8
        let s_24_0: bool = fn_state.gs_6394;
        // N s_24_1: branch s_24_0 b151 b25
        if s_24_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#6395 <= s_25_0
        fn_state.gs_6395 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#6395:u8
        let s_26_0: bool = fn_state.gs_6395;
        // N s_26_1: branch s_26_0 b136 b27
        if s_26_0 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call HaveBTIExt(s_28_0)
        let s_28_1: bool = HaveBTIExt(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b135 b29
        if s_28_1 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#6396 <= s_29_0
        fn_state.gs_6396 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#6396:u8
        let s_30_0: bool = fn_state.gs_6396;
        // N s_30_1: branch s_30_0 b98 b31
        if s_30_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call HaveNV2Ext(s_32_0)
        let s_32_1: bool = HaveNV2Ext(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b97 b33
        if s_32_1 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#6397 <= s_33_0
        fn_state.gs_6397 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#6397:u8
        let s_34_0: bool = fn_state.gs_6397;
        // N s_34_1: branch s_34_0 b96 b35
        if s_34_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#6398 <= s_35_0
        fn_state.gs_6398 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#6398:u8
        let s_36_0: bool = fn_state.gs_6398;
        // N s_36_1: branch s_36_0 b95 b37
        if s_36_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var except.1:struct
        let s_38_0: u32 = fn_state.except._1;
        // C s_38_1: const #31u : u32
        let s_38_1: u32 = 31;
        // D s_38_2: cmp-eq s_38_0 s_38_1
        let s_38_2: bool = ((s_38_0) == (s_38_1));
        // N s_38_3: branch s_38_2 b94 b39
        if s_38_2 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var except.1:struct
        let s_39_0: u32 = fn_state.except._1;
        // C s_39_1: const #40u : u32
        let s_39_1: u32 = 40;
        // D s_39_2: cmp-eq s_39_0 s_39_1
        let s_39_2: bool = ((s_39_0) == (s_39_1));
        // D s_39_3: write-var gs#6399 <= s_39_2
        fn_state.gs_6399 = s_39_2;
        // N s_39_4: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#6399:u8
        let s_40_0: bool = fn_state.gs_6399;
        // D s_40_1: not s_40_0
        let s_40_1: bool = !s_40_0;
        // N s_40_2: branch s_40_1 b93 b41
        if s_40_1 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call HaveBRBExt(s_42_0)
        let s_42_1: bool = HaveBRBExt(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b92 b43
        if s_42_1 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call HaveGCS(s_44_0)
        let s_44_1: bool = HaveGCS(state, tracer, s_44_0);
        // N s_44_2: branch s_44_1 b87 b45
        if s_44_1 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var target_el:u8
        let s_46_0: u8 = fn_state.target_el;
        // C s_46_1: const #16975u : u32
        let s_46_1: u32 = 16975;
        // N s_46_2: write-reg s_46_1 <= s_46_0
        let s_46_2: () = {
            state.write_register::<u8>(s_46_1 as isize, s_46_0);
            tracer.write_register(s_46_1 as isize, s_46_0);
        };
        // C s_46_3: const #0u : u8
        let s_46_3: bool = false;
        // C s_46_4: const #16999u : u32
        let s_46_4: u32 = 16999;
        // N s_46_5: write-reg s_46_4 <= s_46_3
        let s_46_5: () = {
            state.write_register::<bool>(s_46_4 as isize, s_46_3);
            tracer.write_register(s_46_4 as isize, s_46_3);
        };
        // C s_46_6: const #1u : u8
        let s_46_6: bool = true;
        // C s_46_7: const #16990u : u32
        let s_46_7: u32 = 16990;
        // N s_46_8: write-reg s_46_7 <= s_46_6
        let s_46_8: () = {
            state.write_register::<bool>(s_46_7 as isize, s_46_6);
            tracer.write_register(s_46_7 as isize, s_46_6);
        };
        // C s_46_9: const #64s : i
        let s_46_9: i128 = 64;
        // D s_46_10: read-var spsr:u64
        let s_46_10: u64 = fn_state.spsr;
        // D s_46_11: cast zx s_46_10 -> bv
        let s_46_11: Bits = Bits::new(s_46_10 as u128, 64u16);
        // D s_46_12: call SPSR_set(s_46_9, s_46_11)
        let s_46_12: () = SPSR_set(state, tracer, s_46_9, s_46_11);
        // D s_46_13: read-var preferred_exception_return:u64
        let s_46_13: u64 = fn_state.preferred_exception_return;
        // D s_46_14: call ELR_set__1(s_46_13)
        let s_46_14: () = ELR_set__1(state, tracer, s_46_13);
        // C s_46_15: const #0u : u8
        let s_46_15: bool = false;
        // C s_46_16: const #16991u : u32
        let s_46_16: u32 = 16991;
        // N s_46_17: write-reg s_46_16 <= s_46_15
        let s_46_17: () = {
            state.write_register::<bool>(s_46_16 as isize, s_46_15);
            tracer.write_register(s_46_16 as isize, s_46_15);
        };
        // C s_46_18: const #() : ()
        let s_46_18: () = ();
        // S s_46_19: call HaveFeatNMI(s_46_18)
        let s_46_19: bool = HaveFeatNMI(state, tracer, s_46_18);
        // N s_46_20: branch s_46_19 b86 b47
        if s_46_19 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#6401 <= s_47_0
        fn_state.gs_6401 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#6401:u8
        let s_48_0: bool = fn_state.gs_6401;
        // N s_48_1: branch s_48_0 b85 b49
        if s_48_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_49_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #15u : u8
        let s_50_0: u8 = 15;
        // C s_50_1: const #3s : i
        let s_50_1: i128 = 3;
        // C s_50_2: cast zx s_50_0 -> bv
        let s_50_2: Bits = Bits::new(s_50_0 as u128, 4u16);
        // C s_50_3: const #1s : i64
        let s_50_3: i64 = 1;
        // C s_50_4: cast zx s_50_3 -> i
        let s_50_4: i128 = (i128::try_from(s_50_3).unwrap());
        // C s_50_5: const #0s : i
        let s_50_5: i128 = 0;
        // C s_50_6: add s_50_5 s_50_4
        let s_50_6: i128 = (s_50_5 + s_50_4);
        // D s_50_7: bit-extract s_50_2 s_50_1 s_50_6
        let s_50_7: Bits = (Bits::new(
            ((s_50_2) >> (s_50_1)).value(),
            u16::try_from(s_50_6).unwrap(),
        ));
        // D s_50_8: cast reint s_50_7 -> u8
        let s_50_8: bool = ((s_50_7.value()) != 0);
        // C s_50_9: const #16972u : u32
        let s_50_9: u32 = 16972;
        // N s_50_10: write-reg s_50_9 <= s_50_8
        let s_50_10: () = {
            state.write_register::<bool>(s_50_9 as isize, s_50_8);
            tracer.write_register(s_50_9 as isize, s_50_8);
        };
        // C s_50_11: const #2s : i
        let s_50_11: i128 = 2;
        // C s_50_12: cast zx s_50_0 -> bv
        let s_50_12: Bits = Bits::new(s_50_0 as u128, 4u16);
        // C s_50_13: const #1s : i64
        let s_50_13: i64 = 1;
        // C s_50_14: cast zx s_50_13 -> i
        let s_50_14: i128 = (i128::try_from(s_50_13).unwrap());
        // C s_50_15: const #0s : i
        let s_50_15: i128 = 0;
        // C s_50_16: add s_50_15 s_50_14
        let s_50_16: i128 = (s_50_15 + s_50_14);
        // D s_50_17: bit-extract s_50_12 s_50_11 s_50_16
        let s_50_17: Bits = (Bits::new(
            ((s_50_12) >> (s_50_11)).value(),
            u16::try_from(s_50_16).unwrap(),
        ));
        // D s_50_18: cast reint s_50_17 -> u8
        let s_50_18: bool = ((s_50_17.value()) != 0);
        // C s_50_19: const #16968u : u32
        let s_50_19: u32 = 16968;
        // N s_50_20: write-reg s_50_19 <= s_50_18
        let s_50_20: () = {
            state.write_register::<bool>(s_50_19 as isize, s_50_18);
            tracer.write_register(s_50_19 as isize, s_50_18);
        };
        // C s_50_21: const #1s : i
        let s_50_21: i128 = 1;
        // C s_50_22: cast zx s_50_0 -> bv
        let s_50_22: Bits = Bits::new(s_50_0 as u128, 4u16);
        // C s_50_23: const #1s : i64
        let s_50_23: i64 = 1;
        // C s_50_24: cast zx s_50_23 -> i
        let s_50_24: i128 = (i128::try_from(s_50_23).unwrap());
        // C s_50_25: const #0s : i
        let s_50_25: i128 = 0;
        // C s_50_26: add s_50_25 s_50_24
        let s_50_26: i128 = (s_50_25 + s_50_24);
        // D s_50_27: bit-extract s_50_22 s_50_21 s_50_26
        let s_50_27: Bits = (Bits::new(
            ((s_50_22) >> (s_50_21)).value(),
            u16::try_from(s_50_26).unwrap(),
        ));
        // D s_50_28: cast reint s_50_27 -> u8
        let s_50_28: bool = ((s_50_27.value()) != 0);
        // C s_50_29: const #16979u : u32
        let s_50_29: u32 = 16979;
        // N s_50_30: write-reg s_50_29 <= s_50_28
        let s_50_30: () = {
            state.write_register::<bool>(s_50_29 as isize, s_50_28);
            tracer.write_register(s_50_29 as isize, s_50_28);
        };
        // C s_50_31: const #0s : i
        let s_50_31: i128 = 0;
        // C s_50_32: cast zx s_50_0 -> bv
        let s_50_32: Bits = Bits::new(s_50_0 as u128, 4u16);
        // C s_50_33: const #1s : i64
        let s_50_33: i64 = 1;
        // C s_50_34: cast zx s_50_33 -> i
        let s_50_34: i128 = (i128::try_from(s_50_33).unwrap());
        // C s_50_35: const #0s : i
        let s_50_35: i128 = 0;
        // C s_50_36: add s_50_35 s_50_34
        let s_50_36: i128 = (s_50_35 + s_50_34);
        // D s_50_37: bit-extract s_50_32 s_50_31 s_50_36
        let s_50_37: Bits = (Bits::new(
            ((s_50_32) >> (s_50_31)).value(),
            u16::try_from(s_50_36).unwrap(),
        ));
        // D s_50_38: cast reint s_50_37 -> u8
        let s_50_38: bool = ((s_50_37.value()) != 0);
        // C s_50_39: const #16977u : u32
        let s_50_39: u32 = 16977;
        // N s_50_40: write-reg s_50_39 <= s_50_38
        let s_50_40: () = {
            state.write_register::<bool>(s_50_39 as isize, s_50_38);
            tracer.write_register(s_50_39 as isize, s_50_38);
        };
        // C s_50_41: const #0u : u8
        let s_50_41: bool = false;
        // C s_50_42: const #16980u : u32
        let s_50_42: u32 = 16980;
        // N s_50_43: write-reg s_50_42 <= s_50_41
        let s_50_43: () = {
            state.write_register::<bool>(s_50_42 as isize, s_50_41);
            tracer.write_register(s_50_42 as isize, s_50_41);
        };
        // D s_50_44: read-var from_32:u8
        let s_50_44: bool = fn_state.from_32;
        // N s_50_45: branch s_50_44 b84 b51
        if s_50_44 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_51_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call HavePANExt(s_52_0)
        let s_52_1: bool = HavePANExt(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b77 b53
        if s_52_1 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#6415 <= s_53_0
        fn_state.gs_6415 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#6415:u8
        let s_54_0: bool = fn_state.gs_6415;
        // N s_54_1: branch s_54_0 b76 b55
        if s_54_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#6416 <= s_55_0
        fn_state.gs_6416 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#6416:u8
        let s_56_0: bool = fn_state.gs_6416;
        // N s_56_1: branch s_56_0 b75 b57
        if s_56_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_57_0: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call HaveUAOExt(s_58_0)
        let s_58_1: bool = HaveUAOExt(state, tracer, s_58_0);
        // N s_58_2: branch s_58_1 b74 b59
        if s_58_1 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call HaveBTIExt(s_60_0)
        let s_60_1: bool = HaveBTIExt(state, tracer, s_60_0);
        // N s_60_2: branch s_60_1 b73 b61
        if s_60_1 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_61_0: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call HaveSSBSExt(s_62_0)
        let s_62_1: bool = HaveSSBSExt(state, tracer, s_62_0);
        // N s_62_2: branch s_62_1 b72 b63
        if s_62_1 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_63_0: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call HaveMTEExt(s_64_0)
        let s_64_1: bool = HaveMTEExt(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b71 b65
        if s_64_1 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_65_0: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // C s_66_1: const #() : ()
        let s_66_1: () = ();
        // S s_66_2: call VBAR_read__1(s_66_1)
        let s_66_2: u64 = VBAR_read__1(state, tracer, s_66_1);
        // C s_66_3: const #11s : i
        let s_66_3: i128 = 11;
        // S s_66_4: cast zx s_66_2 -> bv
        let s_66_4: Bits = Bits::new(s_66_2 as u128, 64u16);
        // C s_66_5: const #1s : i64
        let s_66_5: i64 = 1;
        // C s_66_6: cast zx s_66_5 -> i
        let s_66_6: i128 = (i128::try_from(s_66_5).unwrap());
        // C s_66_7: const #52s : i
        let s_66_7: i128 = 52;
        // C s_66_8: add s_66_7 s_66_6
        let s_66_8: i128 = (s_66_7 + s_66_6);
        // D s_66_9: bit-extract s_66_4 s_66_3 s_66_8
        let s_66_9: Bits = (Bits::new(
            ((s_66_4) >> (s_66_3)).value(),
            u16::try_from(s_66_8).unwrap(),
        ));
        // D s_66_10: cast reint s_66_9 -> u53
        let s_66_10: u64 = (s_66_9.value() as u64);
        // C s_66_11: const #10s : i
        let s_66_11: i128 = 10;
        // C s_66_12: const #0s : i
        let s_66_12: i128 = 0;
        // D s_66_13: read-var vect_offsetshadow#76:i
        let s_66_13: i128 = fn_state.vect_offsetshadow_76;
        // D s_66_14: call integer_subrange(s_66_13, s_66_11, s_66_12)
        let s_66_14: Bits = integer_subrange(state, tracer, s_66_13, s_66_11, s_66_12);
        // D s_66_15: cast reint s_66_14 -> u11
        let s_66_15: u16 = (s_66_14.value() as u16);
        // D s_66_16: cast zx s_66_10 -> bv
        let s_66_16: Bits = Bits::new(s_66_10 as u128, 53u16);
        // D s_66_17: cast zx s_66_15 -> bv
        let s_66_17: Bits = Bits::new(s_66_15 as u128, 11u16);
        // D s_66_18: cast reint s_66_16 -> u128
        let s_66_18: u128 = (s_66_16.value() as u128);
        // D s_66_19: size-of s_66_16
        let s_66_19: u16 = s_66_16.length();
        // D s_66_20: cast reint s_66_17 -> u128
        let s_66_20: u128 = (s_66_17.value() as u128);
        // D s_66_21: size-of s_66_17
        let s_66_21: u16 = s_66_17.length();
        // D s_66_22: lsl s_66_18 s_66_21
        let s_66_22: u128 = s_66_18 << s_66_21;
        // D s_66_23: or s_66_22 s_66_20
        let s_66_23: u128 = ((s_66_22) | (s_66_20));
        // D s_66_24: add s_66_19 s_66_21
        let s_66_24: u16 = (s_66_19 + s_66_21);
        // D s_66_25: create-bits s_66_23 s_66_24
        let s_66_25: Bits = Bits::new(s_66_23, s_66_24);
        // D s_66_26: cast reint s_66_25 -> u64
        let s_66_26: u64 = (s_66_25.value() as u64);
        // D s_66_27: cast zx s_66_26 -> bv
        let s_66_27: Bits = Bits::new(s_66_26 as u128, 64u16);
        // C s_66_28: const #7u : u32
        let s_66_28: u32 = 7;
        // D s_66_29: call BranchTo(s_66_27, s_66_28, s_66_0)
        let s_66_29: () = BranchTo(state, tracer, s_66_27, s_66_28, s_66_0);
        // C s_66_30: const #1u : u8
        let s_66_30: bool = true;
        // S s_66_31: call CheckExceptionCatch(s_66_30)
        let s_66_31: () = CheckExceptionCatch(state, tracer, s_66_30);
        // N s_66_32: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var sync_errors:u8
        let s_67_0: bool = fn_state.sync_errors;
        // N s_67_1: branch s_67_0 b70 b68
        if s_67_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_68_0: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call EndOfInstruction(s_69_0)
        let s_69_1: () = EndOfInstruction(state, tracer, s_69_0);
        // N s_69_2: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call SynchronizeErrors(s_70_0)
        let s_70_1: () = SynchronizeErrors(state, tracer, s_70_0);
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // S s_70_3: call TakeUnmaskedPhysicalSErrorInterrupts(s_70_2)
        let s_70_3: () = TakeUnmaskedPhysicalSErrorInterrupts(state, tracer, s_70_2);
        // N s_70_4: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // C s_71_1: const #16994u : u32
        let s_71_1: u32 = 16994;
        // N s_71_2: write-reg s_71_1 <= s_71_0
        let s_71_2: () = {
            state.write_register::<bool>(s_71_1 as isize, s_71_0);
            tracer.write_register(s_71_1 as isize, s_71_0);
        };
        // N s_71_3: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call SCTLR_read__1(s_72_0)
        let s_72_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_72_0);
        // S s_72_2: call _get_SCTLRType_DSSBS(s_72_1)
        let s_72_2: bool = u_get_SCTLRType_DSSBS(state, tracer, s_72_1);
        // C s_72_3: const #16992u : u32
        let s_72_3: u32 = 16992;
        // N s_72_4: write-reg s_72_3 <= s_72_2
        let s_72_4: () = {
            state.write_register::<bool>(s_72_3 as isize, s_72_2);
            tracer.write_register(s_72_3 as isize, s_72_2);
        };
        // N s_72_5: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: u8 = 0;
        // C s_73_1: const #16970u : u32
        let s_73_1: u32 = 16970;
        // N s_73_2: write-reg s_73_1 <= s_73_0
        let s_73_2: () = {
            state.write_register::<u8>(s_73_1 as isize, s_73_0);
            tracer.write_register(s_73_1 as isize, s_73_0);
        };
        // N s_73_3: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // C s_74_1: const #16995u : u32
        let s_74_1: u32 = 16995;
        // N s_74_2: write-reg s_74_1 <= s_74_0
        let s_74_2: () = {
            state.write_register::<bool>(s_74_1 as isize, s_74_0);
            tracer.write_register(s_74_1 as isize, s_74_0);
        };
        // N s_74_3: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // C s_75_1: const #16985u : u32
        let s_75_1: u32 = 16985;
        // N s_75_2: write-reg s_75_1 <= s_75_0
        let s_75_2: () = {
            state.write_register::<bool>(s_75_1 as isize, s_75_0);
            tracer.write_register(s_75_1 as isize, s_75_0);
        };
        // N s_75_3: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call SCTLR_read__1(s_76_0)
        let s_76_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_76_0);
        // S s_76_2: call _get_SCTLRType_SPAN(s_76_1)
        let s_76_2: bool = u_get_SCTLRType_SPAN(state, tracer, s_76_1);
        // S s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // C s_76_4: const #0u : u8
        let s_76_4: bool = false;
        // C s_76_5: cast zx s_76_4 -> bv
        let s_76_5: Bits = Bits::new(s_76_4 as u128, 1u16);
        // S s_76_6: cmp-eq s_76_3 s_76_5
        let s_76_6: bool = ((s_76_3) == (s_76_5));
        // D s_76_7: write-var gs#6416 <= s_76_6
        fn_state.gs_6416 = s_76_6;
        // N s_76_8: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #16975u : u32
        let s_77_0: u32 = 16975;
        // D s_77_1: read-reg s_77_0:u8
        let s_77_1: u8 = {
            let value = state.read_register::<u8>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: cast zx s_77_1 -> bv
        let s_77_2: Bits = Bits::new(s_77_1 as u128, 2u16);
        // C s_77_3: const #440u : u32
        let s_77_3: u32 = 440;
        // D s_77_4: read-reg s_77_3:u8
        let s_77_4: u8 = {
            let value = state.read_register::<u8>(s_77_3 as isize);
            tracer.read_register(s_77_3 as isize, value);
            value
        };
        // D s_77_5: cast zx s_77_4 -> bv
        let s_77_5: Bits = Bits::new(s_77_4 as u128, 2u16);
        // D s_77_6: cmp-eq s_77_2 s_77_5
        let s_77_6: bool = ((s_77_2) == (s_77_5));
        // N s_77_7: branch s_77_6 b83 b78
        if s_77_6 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #16975u : u32
        let s_78_0: u32 = 16975;
        // D s_78_1: read-reg s_78_0:u8
        let s_78_1: u8 = {
            let value = state.read_register::<u8>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: cast zx s_78_1 -> bv
        let s_78_2: Bits = Bits::new(s_78_1 as u128, 2u16);
        // C s_78_3: const #432u : u32
        let s_78_3: u32 = 432;
        // D s_78_4: read-reg s_78_3:u8
        let s_78_4: u8 = {
            let value = state.read_register::<u8>(s_78_3 as isize);
            tracer.read_register(s_78_3 as isize, value);
            value
        };
        // D s_78_5: cast zx s_78_4 -> bv
        let s_78_5: Bits = Bits::new(s_78_4 as u128, 2u16);
        // D s_78_6: cmp-eq s_78_2 s_78_5
        let s_78_6: bool = ((s_78_2) == (s_78_5));
        // N s_78_7: branch s_78_6 b82 b79
        if s_78_6 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#6413 <= s_79_0
        fn_state.gs_6413 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#6413:u8
        let s_80_0: bool = fn_state.gs_6413;
        // D s_80_1: write-var gs#6414 <= s_80_0
        fn_state.gs_6414 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#6414:u8
        let s_81_0: bool = fn_state.gs_6414;
        // D s_81_1: write-var gs#6415 <= s_81_0
        fn_state.gs_6415 = s_81_0;
        // N s_81_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #448u : u32
        let s_82_0: u32 = 448;
        // D s_82_1: read-reg s_82_0:u8
        let s_82_1: u8 = {
            let value = state.read_register::<u8>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call ELIsInHost(s_82_1)
        let s_82_2: bool = ELIsInHost(state, tracer, s_82_1);
        // D s_82_3: write-var gs#6413 <= s_82_2
        fn_state.gs_6413 = s_82_2;
        // N s_82_4: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #1u : u8
        let s_83_0: bool = true;
        // D s_83_1: write-var gs#6414 <= s_83_0
        fn_state.gs_6414 = s_83_0;
        // N s_83_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0u : u8
        let s_84_0: u8 = 0;
        // C s_84_1: const #16981u : u32
        let s_84_1: u32 = 16981;
        // N s_84_2: write-reg s_84_1 <= s_84_0
        let s_84_2: () = {
            state.write_register::<u8>(s_84_1 as isize, s_84_0);
            tracer.write_register(s_84_1 as isize, s_84_0);
        };
        // C s_84_3: const #0u : u8
        let s_84_3: bool = false;
        // C s_84_4: const #16993u : u32
        let s_84_4: u32 = 16993;
        // N s_84_5: write-reg s_84_4 <= s_84_3
        let s_84_5: () = {
            state.write_register::<bool>(s_84_4 as isize, s_84_3);
            tracer.write_register(s_84_4 as isize, s_84_3);
        };
        // N s_84_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #() : ()
        let s_85_0: () = ();
        // S s_85_1: call SCTLR_read__1(s_85_0)
        let s_85_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_85_0);
        // S s_85_2: call _get_SCTLRType_SPINTMASK(s_85_1)
        let s_85_2: bool = u_get_SCTLRType_SPINTMASK(state, tracer, s_85_1);
        // S s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // S s_85_4: not s_85_3
        let s_85_4: Bits = !s_85_3;
        // S s_85_5: cast reint s_85_4 -> u8
        let s_85_5: bool = ((s_85_4.value()) != 0);
        // C s_85_6: const #16969u : u32
        let s_85_6: u32 = 16969;
        // N s_85_7: write-reg s_85_6 <= s_85_5
        let s_85_7: () = {
            state.write_register::<bool>(s_85_6 as isize, s_85_5);
            tracer.write_register(s_85_6 as isize, s_85_5);
        };
        // N s_85_8: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var target_el:u8
        let s_86_0: u8 = fn_state.target_el;
        // D s_86_1: call ELUsingAArch32(s_86_0)
        let s_86_1: bool = ELUsingAArch32(state, tracer, s_86_0);
        // D s_86_2: not s_86_1
        let s_86_2: bool = !s_86_1;
        // D s_86_3: write-var gs#6401 <= s_86_2
        fn_state.gs_6401 = s_86_2;
        // N s_86_4: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #16975u : u32
        let s_87_0: u32 = 16975;
        // D s_87_1: read-reg s_87_0:u8
        let s_87_1: u8 = {
            let value = state.read_register::<u8>(s_87_0 as isize);
            tracer.read_register(s_87_0 as isize, value);
            value
        };
        // D s_87_2: cast zx s_87_1 -> bv
        let s_87_2: Bits = Bits::new(s_87_1 as u128, 2u16);
        // D s_87_3: read-var target_el:u8
        let s_87_3: u8 = fn_state.target_el;
        // D s_87_4: cast zx s_87_3 -> bv
        let s_87_4: Bits = Bits::new(s_87_3 as u128, 2u16);
        // D s_87_5: cmp-eq s_87_2 s_87_4
        let s_87_5: bool = ((s_87_2) == (s_87_4));
        // N s_87_6: branch s_87_5 b89 b88
        if s_87_5 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // C s_88_1: const #16976u : u32
        let s_88_1: u32 = 16976;
        // N s_88_2: write-reg s_88_1 <= s_88_0
        let s_88_2: () = {
            state.write_register::<bool>(s_88_1 as isize, s_88_0);
            tracer.write_register(s_88_1 as isize, s_88_0);
        };
        // N s_88_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #() : ()
        let s_89_0: () = ();
        // S s_89_1: call GetCurrentEXLOCKEN(s_89_0)
        let s_89_1: bool = GetCurrentEXLOCKEN(state, tracer, s_89_0);
        // N s_89_2: branch s_89_1 b91 b90
        if s_89_1 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // C s_90_1: const #16976u : u32
        let s_90_1: u32 = 16976;
        // N s_90_2: write-reg s_90_1 <= s_90_0
        let s_90_2: () = {
            state.write_register::<bool>(s_90_1 as isize, s_90_0);
            tracer.write_register(s_90_1 as isize, s_90_0);
        };
        // N s_90_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #1u : u8
        let s_91_0: bool = true;
        // C s_91_1: const #16976u : u32
        let s_91_1: u32 = 16976;
        // N s_91_2: write-reg s_91_1 <= s_91_0
        let s_91_2: () = {
            state.write_register::<bool>(s_91_1 as isize, s_91_0);
            tracer.write_register(s_91_1 as isize, s_91_0);
        };
        // N s_91_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var target_el:u8
        let s_92_0: u8 = fn_state.target_el;
        // D s_92_1: call VBAR_read(s_92_0)
        let s_92_1: u64 = VBAR_read(state, tracer, s_92_0);
        // C s_92_2: const #11s : i
        let s_92_2: i128 = 11;
        // D s_92_3: cast zx s_92_1 -> bv
        let s_92_3: Bits = Bits::new(s_92_1 as u128, 64u16);
        // C s_92_4: const #1s : i64
        let s_92_4: i64 = 1;
        // C s_92_5: cast zx s_92_4 -> i
        let s_92_5: i128 = (i128::try_from(s_92_4).unwrap());
        // C s_92_6: const #52s : i
        let s_92_6: i128 = 52;
        // C s_92_7: add s_92_6 s_92_5
        let s_92_7: i128 = (s_92_6 + s_92_5);
        // D s_92_8: bit-extract s_92_3 s_92_2 s_92_7
        let s_92_8: Bits = (Bits::new(
            ((s_92_3) >> (s_92_2)).value(),
            u16::try_from(s_92_7).unwrap(),
        ));
        // D s_92_9: cast reint s_92_8 -> u53
        let s_92_9: u64 = (s_92_8.value() as u64);
        // C s_92_10: const #10s : i
        let s_92_10: i128 = 10;
        // C s_92_11: const #0s : i
        let s_92_11: i128 = 0;
        // D s_92_12: read-var vect_offsetshadow#76:i
        let s_92_12: i128 = fn_state.vect_offsetshadow_76;
        // D s_92_13: call integer_subrange(s_92_12, s_92_10, s_92_11)
        let s_92_13: Bits = integer_subrange(state, tracer, s_92_12, s_92_10, s_92_11);
        // D s_92_14: cast reint s_92_13 -> u11
        let s_92_14: u16 = (s_92_13.value() as u16);
        // D s_92_15: cast zx s_92_9 -> bv
        let s_92_15: Bits = Bits::new(s_92_9 as u128, 53u16);
        // D s_92_16: cast zx s_92_14 -> bv
        let s_92_16: Bits = Bits::new(s_92_14 as u128, 11u16);
        // D s_92_17: cast reint s_92_15 -> u128
        let s_92_17: u128 = (s_92_15.value() as u128);
        // D s_92_18: size-of s_92_15
        let s_92_18: u16 = s_92_15.length();
        // D s_92_19: cast reint s_92_16 -> u128
        let s_92_19: u128 = (s_92_16.value() as u128);
        // D s_92_20: size-of s_92_16
        let s_92_20: u16 = s_92_16.length();
        // D s_92_21: lsl s_92_17 s_92_20
        let s_92_21: u128 = s_92_17 << s_92_20;
        // D s_92_22: or s_92_21 s_92_19
        let s_92_22: u128 = ((s_92_21) | (s_92_19));
        // D s_92_23: add s_92_18 s_92_20
        let s_92_23: u16 = (s_92_18 + s_92_20);
        // D s_92_24: create-bits s_92_22 s_92_23
        let s_92_24: Bits = Bits::new(s_92_22, s_92_23);
        // D s_92_25: cast reint s_92_24 -> u64
        let s_92_25: u64 = (s_92_24.value() as u64);
        // D s_92_26: read-var except.8:struct
        let s_92_26: bool = fn_state.except._8;
        // D s_92_27: read-var except:struct
        let s_92_27: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_92_28: read-var preferred_exception_return:u64
        let s_92_28: u64 = fn_state.preferred_exception_return;
        // D s_92_29: read-var target_el:u8
        let s_92_29: u8 = fn_state.target_el;
        // D s_92_30: call BRBEException(s_92_27, s_92_28, s_92_25, s_92_29, s_92_26)
        let s_92_30: () = BRBEException(
            state,
            tracer,
            s_92_27,
            s_92_28,
            s_92_25,
            s_92_29,
            s_92_26,
        );
        // N s_92_31: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var except:struct
        let s_93_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_93_1: read-var target_el:u8
        let s_93_1: u8 = fn_state.target_el;
        // D s_93_2: call AArch64_ReportException(s_93_0, s_93_1)
        let s_93_2: () = AArch64_ReportException(state, tracer, s_93_0, s_93_1);
        // N s_93_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #1u : u8
        let s_94_0: bool = true;
        // D s_94_1: write-var gs#6399 <= s_94_0
        fn_state.gs_6399 = s_94_0;
        // N s_94_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #19u : u32
        let s_95_0: u32 = 19;
        // D s_95_1: write-var except.1 <= s_95_0
        fn_state.except._1 = s_95_0;
        // N s_95_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var target_el:u8
        let s_96_0: u8 = fn_state.target_el;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 2u16);
        // C s_96_2: const #424u : u32
        let s_96_2: u32 = 424;
        // D s_96_3: read-reg s_96_2:u8
        let s_96_3: u8 = {
            let value = state.read_register::<u8>(s_96_2 as isize);
            tracer.read_register(s_96_2 as isize, value);
            value
        };
        // D s_96_4: cast zx s_96_3 -> bv
        let s_96_4: Bits = Bits::new(s_96_3 as u128, 2u16);
        // D s_96_5: cmp-eq s_96_1 s_96_4
        let s_96_5: bool = ((s_96_1) == (s_96_4));
        // D s_96_6: write-var gs#6398 <= s_96_5
        fn_state.gs_6398 = s_96_5;
        // N s_96_7: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var except.1:struct
        let s_97_0: u32 = fn_state.except._1;
        // C s_97_1: const #20u : u32
        let s_97_1: u32 = 20;
        // D s_97_2: cmp-eq s_97_0 s_97_1
        let s_97_2: bool = ((s_97_0) == (s_97_1));
        // D s_97_3: write-var gs#6397 <= s_97_2
        fn_state.gs_6397 = s_97_2;
        // N s_97_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var except.1:struct
        let s_98_0: u32 = fn_state.except._1;
        // C s_98_1: const #24u : u32
        let s_98_1: u32 = 24;
        // D s_98_2: cmp-eq s_98_0 s_98_1
        let s_98_2: bool = ((s_98_0) == (s_98_1));
        // N s_98_3: branch s_98_2 b134 b99
        if s_98_2 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var except.1:struct
        let s_99_0: u32 = fn_state.except._1;
        // C s_99_1: const #31u : u32
        let s_99_1: u32 = 31;
        // D s_99_2: cmp-eq s_99_0 s_99_1
        let s_99_2: bool = ((s_99_0) == (s_99_1));
        // N s_99_3: branch s_99_2 b133 b100
        if s_99_2 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var except.1:struct
        let s_100_0: u32 = fn_state.except._1;
        // C s_100_1: const #40u : u32
        let s_100_1: u32 = 40;
        // D s_100_2: cmp-eq s_100_0 s_100_1
        let s_100_2: bool = ((s_100_0) == (s_100_1));
        // N s_100_3: branch s_100_2 b132 b101
        if s_100_2 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var except.1:struct
        let s_101_0: u32 = fn_state.except._1;
        // C s_101_1: const #26u : u32
        let s_101_1: u32 = 26;
        // D s_101_2: cmp-eq s_101_0 s_101_1
        let s_101_2: bool = ((s_101_0) == (s_101_1));
        // N s_101_3: branch s_101_2 b131 b102
        if s_101_2 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var except.1:struct
        let s_102_0: u32 = fn_state.except._1;
        // C s_102_1: const #18u : u32
        let s_102_1: u32 = 18;
        // D s_102_2: cmp-eq s_102_0 s_102_1
        let s_102_2: bool = ((s_102_0) == (s_102_1));
        // N s_102_3: branch s_102_2 b130 b103
        if s_102_2 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var except.1:struct
        let s_103_0: u32 = fn_state.except._1;
        // C s_103_1: const #17u : u32
        let s_103_1: u32 = 17;
        // D s_103_2: cmp-eq s_103_0 s_103_1
        let s_103_2: bool = ((s_103_0) == (s_103_1));
        // N s_103_3: branch s_103_2 b129 b104
        if s_103_2 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var except.1:struct
        let s_104_0: u32 = fn_state.except._1;
        // C s_104_1: const #25u : u32
        let s_104_1: u32 = 25;
        // D s_104_2: cmp-eq s_104_0 s_104_1
        let s_104_2: bool = ((s_104_0) == (s_104_1));
        // N s_104_3: branch s_104_2 b128 b105
        if s_104_2 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var except.1:struct
        let s_105_0: u32 = fn_state.except._1;
        // C s_105_1: const #30u : u32
        let s_105_1: u32 = 30;
        // D s_105_2: cmp-eq s_105_0 s_105_1
        let s_105_2: bool = ((s_105_0) == (s_105_1));
        // N s_105_3: branch s_105_2 b127 b106
        if s_105_2 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var except.1:struct
        let s_106_0: u32 = fn_state.except._1;
        // C s_106_1: const #29u : u32
        let s_106_1: u32 = 29;
        // D s_106_2: cmp-eq s_106_0 s_106_1
        let s_106_2: bool = ((s_106_0) == (s_106_1));
        // N s_106_3: branch s_106_2 b126 b107
        if s_106_2 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var except.1:struct
        let s_107_0: u32 = fn_state.except._1;
        // C s_107_1: const #11u : u32
        let s_107_1: u32 = 11;
        // D s_107_2: cmp-eq s_107_0 s_107_1
        let s_107_2: bool = ((s_107_0) == (s_107_1));
        // N s_107_3: branch s_107_2 b125 b108
        if s_107_2 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var except.1:struct
        let s_108_0: u32 = fn_state.except._1;
        // C s_108_1: const #36u : u32
        let s_108_1: u32 = 36;
        // D s_108_2: cmp-eq s_108_0 s_108_1
        let s_108_2: bool = ((s_108_0) == (s_108_1));
        // D s_108_3: write-var gs#6442 <= s_108_2
        fn_state.gs_6442 = s_108_2;
        // N s_108_4: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#6442:u8
        let s_109_0: bool = fn_state.gs_6442;
        // D s_109_1: write-var gs#6443 <= s_109_0
        fn_state.gs_6443 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#6443:u8
        let s_110_0: bool = fn_state.gs_6443;
        // D s_110_1: write-var gs#6444 <= s_110_0
        fn_state.gs_6444 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#6444:u8
        let s_111_0: bool = fn_state.gs_6444;
        // D s_111_1: write-var gs#6445 <= s_111_0
        fn_state.gs_6445 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#6445:u8
        let s_112_0: bool = fn_state.gs_6445;
        // D s_112_1: write-var gs#6446 <= s_112_0
        fn_state.gs_6446 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#6446:u8
        let s_113_0: bool = fn_state.gs_6446;
        // D s_113_1: write-var gs#6447 <= s_113_0
        fn_state.gs_6447 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#6447:u8
        let s_114_0: bool = fn_state.gs_6447;
        // D s_114_1: write-var gs#6448 <= s_114_0
        fn_state.gs_6448 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#6448:u8
        let s_115_0: bool = fn_state.gs_6448;
        // D s_115_1: write-var gs#6449 <= s_115_0
        fn_state.gs_6449 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#6449:u8
        let s_116_0: bool = fn_state.gs_6449;
        // D s_116_1: write-var gs#6450 <= s_116_0
        fn_state.gs_6450 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#6450:u8
        let s_117_0: bool = fn_state.gs_6450;
        // D s_117_1: write-var gs#6451 <= s_117_0
        fn_state.gs_6451 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#6451:u8
        let s_118_0: bool = fn_state.gs_6451;
        // N s_118_1: branch s_118_0 b124 b119
        if s_118_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #61u : u32
        let s_119_0: u32 = 61;
        // S s_119_1: call ConstrainUnpredictableBool(s_119_0)
        let s_119_1: bool = ConstrainUnpredictableBool(state, tracer, s_119_0);
        // D s_119_2: write-var zero_btype <= s_119_1
        fn_state.zero_btype = s_119_1;
        // N s_119_3: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var zero_btype:u8
        let s_120_0: bool = fn_state.zero_btype;
        // N s_120_1: branch s_120_0 b123 b121
        if s_120_0 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_121_0: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_122_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #10s : i
        let s_123_0: i128 = 10;
        // D s_123_1: read-var spsr:u64
        let s_123_1: u64 = fn_state.spsr;
        // D s_123_2: cast zx s_123_1 -> bv
        let s_123_2: Bits = Bits::new(s_123_1 as u128, 64u16);
        // C s_123_3: const #0u : u8
        let s_123_3: u8 = 0;
        // C s_123_4: cast zx s_123_3 -> bv
        let s_123_4: Bits = Bits::new(s_123_3 as u128, 2u16);
        // C s_123_5: const #1s : i
        let s_123_5: i128 = 1;
        // C s_123_6: const #1u : u64
        let s_123_6: u64 = 1;
        // C s_123_7: cast zx s_123_6 -> bv
        let s_123_7: Bits = Bits::new(s_123_6 as u128, 64u16);
        // C s_123_8: lsl s_123_7 s_123_5
        let s_123_8: Bits = s_123_7 << s_123_5;
        // C s_123_9: sub s_123_8 s_123_7
        let s_123_9: Bits = ((s_123_8) - (s_123_7));
        // C s_123_10: and s_123_4 s_123_9
        let s_123_10: Bits = ((s_123_4) & (s_123_9));
        // C s_123_11: lsl s_123_10 s_123_0
        let s_123_11: Bits = s_123_10 << s_123_0;
        // C s_123_12: lsl s_123_9 s_123_0
        let s_123_12: Bits = s_123_9 << s_123_0;
        // C s_123_13: cmpl s_123_12
        let s_123_13: Bits = !s_123_12;
        // D s_123_14: and s_123_2 s_123_13
        let s_123_14: Bits = ((s_123_2) & (s_123_13));
        // D s_123_15: or s_123_14 s_123_11
        let s_123_15: Bits = ((s_123_14) | (s_123_11));
        // D s_123_16: cast reint s_123_15 -> u64
        let s_123_16: u64 = (s_123_15.value() as u64);
        // D s_123_17: write-var spsr <= s_123_16
        fn_state.spsr = s_123_16;
        // N s_123_18: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #0u : u8
        let s_124_0: bool = false;
        // D s_124_1: write-var zero_btype <= s_124_0
        fn_state.zero_btype = s_124_0;
        // N s_124_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #1u : u8
        let s_125_0: bool = true;
        // D s_125_1: write-var gs#6442 <= s_125_0
        fn_state.gs_6442 = s_125_0;
        // N s_125_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #1u : u8
        let s_126_0: bool = true;
        // D s_126_1: write-var gs#6443 <= s_126_0
        fn_state.gs_6443 = s_126_0;
        // N s_126_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #1u : u8
        let s_127_0: bool = true;
        // D s_127_1: write-var gs#6444 <= s_127_0
        fn_state.gs_6444 = s_127_0;
        // N s_127_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #1u : u8
        let s_128_0: bool = true;
        // D s_128_1: write-var gs#6445 <= s_128_0
        fn_state.gs_6445 = s_128_0;
        // N s_128_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #1u : u8
        let s_129_0: bool = true;
        // D s_129_1: write-var gs#6446 <= s_129_0
        fn_state.gs_6446 = s_129_0;
        // N s_129_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #1u : u8
        let s_130_0: bool = true;
        // D s_130_1: write-var gs#6447 <= s_130_0
        fn_state.gs_6447 = s_130_0;
        // N s_130_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #1u : u8
        let s_131_0: bool = true;
        // D s_131_1: write-var gs#6448 <= s_131_0
        fn_state.gs_6448 = s_131_0;
        // N s_131_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #1u : u8
        let s_132_0: bool = true;
        // D s_132_1: write-var gs#6449 <= s_132_0
        fn_state.gs_6449 = s_132_0;
        // N s_132_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #1u : u8
        let s_133_0: bool = true;
        // D s_133_1: write-var gs#6450 <= s_133_0
        fn_state.gs_6450 = s_133_0;
        // N s_133_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #1u : u8
        let s_134_0: bool = true;
        // D s_134_1: write-var gs#6451 <= s_134_0
        fn_state.gs_6451 = s_134_0;
        // N s_134_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #() : ()
        let s_135_0: () = ();
        // S s_135_1: call UsingAArch32(s_135_0)
        let s_135_1: bool = UsingAArch32(state, tracer, s_135_0);
        // S s_135_2: not s_135_1
        let s_135_2: bool = !s_135_1;
        // D s_135_3: write-var gs#6396 <= s_135_2
        fn_state.gs_6396 = s_135_2;
        // N s_135_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #() : ()
        let s_136_0: () = ();
        // S s_136_1: call HaveNV2Ext(s_136_0)
        let s_136_1: bool = HaveNV2Ext(state, tracer, s_136_0);
        // N s_136_2: branch s_136_1 b147 b137
        if s_136_1 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #0u : u8
        let s_137_0: bool = false;
        // D s_137_1: write-var gs#6458 <= s_137_0
        fn_state.gs_6458 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#6458:u8
        let s_138_0: bool = fn_state.gs_6458;
        // N s_138_1: branch s_138_0 b146 b139
        if s_138_0 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #() : ()
        let s_139_0: () = ();
        // S s_139_1: call HaveNVExt(s_139_0)
        let s_139_1: bool = HaveNVExt(state, tracer, s_139_0);
        // N s_139_2: branch s_139_1 b145 b140
        if s_139_1 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #0u : u8
        let s_140_0: bool = false;
        // D s_140_1: write-var gs#6459 <= s_140_0
        fn_state.gs_6459 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#6459:u8
        let s_141_0: bool = fn_state.gs_6459;
        // N s_141_1: branch s_141_0 b144 b142
        if s_141_0 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_142_0: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_143_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #2s : i
        let s_144_0: i128 = 2;
        // D s_144_1: read-var spsr:u64
        let s_144_1: u64 = fn_state.spsr;
        // D s_144_2: cast zx s_144_1 -> bv
        let s_144_2: Bits = Bits::new(s_144_1 as u128, 64u16);
        // C s_144_3: const #2u : u8
        let s_144_3: u8 = 2;
        // C s_144_4: cast zx s_144_3 -> bv
        let s_144_4: Bits = Bits::new(s_144_3 as u128, 2u16);
        // C s_144_5: const #1s : i
        let s_144_5: i128 = 1;
        // C s_144_6: const #1u : u64
        let s_144_6: u64 = 1;
        // C s_144_7: cast zx s_144_6 -> bv
        let s_144_7: Bits = Bits::new(s_144_6 as u128, 64u16);
        // C s_144_8: lsl s_144_7 s_144_5
        let s_144_8: Bits = s_144_7 << s_144_5;
        // C s_144_9: sub s_144_8 s_144_7
        let s_144_9: Bits = ((s_144_8) - (s_144_7));
        // C s_144_10: and s_144_4 s_144_9
        let s_144_10: Bits = ((s_144_4) & (s_144_9));
        // C s_144_11: lsl s_144_10 s_144_0
        let s_144_11: Bits = s_144_10 << s_144_0;
        // C s_144_12: lsl s_144_9 s_144_0
        let s_144_12: Bits = s_144_9 << s_144_0;
        // C s_144_13: cmpl s_144_12
        let s_144_13: Bits = !s_144_12;
        // D s_144_14: and s_144_2 s_144_13
        let s_144_14: Bits = ((s_144_2) & (s_144_13));
        // D s_144_15: or s_144_14 s_144_11
        let s_144_15: Bits = ((s_144_14) | (s_144_11));
        // D s_144_16: cast reint s_144_15 -> u64
        let s_144_16: u64 = (s_144_15.value() as u64);
        // D s_144_17: write-var spsr <= s_144_16
        fn_state.spsr = s_144_16;
        // N s_144_18: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #102552u : u32
        let s_145_0: u32 = 102552;
        // D s_145_1: read-reg s_145_0:struct
        let s_145_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_145_0 as isize);
            tracer.read_register(s_145_0 as isize, value);
            value
        };
        // D s_145_2: call _get_HCR_EL2_Type_NV(s_145_1)
        let s_145_2: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_145_1);
        // C s_145_3: const #102552u : u32
        let s_145_3: u32 = 102552;
        // D s_145_4: read-reg s_145_3:struct
        let s_145_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_145_3 as isize);
            tracer.read_register(s_145_3 as isize, value);
            value
        };
        // D s_145_5: call _get_HCR_EL2_Type_NV1(s_145_4)
        let s_145_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_145_4);
        // D s_145_6: cast zx s_145_2 -> bv
        let s_145_6: Bits = Bits::new(s_145_2 as u128, 1u16);
        // D s_145_7: cast zx s_145_5 -> bv
        let s_145_7: Bits = Bits::new(s_145_5 as u128, 1u16);
        // D s_145_8: cast reint s_145_6 -> u128
        let s_145_8: u128 = (s_145_6.value() as u128);
        // D s_145_9: size-of s_145_6
        let s_145_9: u16 = s_145_6.length();
        // D s_145_10: cast reint s_145_7 -> u128
        let s_145_10: u128 = (s_145_7.value() as u128);
        // D s_145_11: size-of s_145_7
        let s_145_11: u16 = s_145_7.length();
        // D s_145_12: lsl s_145_8 s_145_11
        let s_145_12: u128 = s_145_8 << s_145_11;
        // D s_145_13: or s_145_12 s_145_10
        let s_145_13: u128 = ((s_145_12) | (s_145_10));
        // D s_145_14: add s_145_9 s_145_11
        let s_145_14: u16 = (s_145_9 + s_145_11);
        // D s_145_15: create-bits s_145_13 s_145_14
        let s_145_15: Bits = Bits::new(s_145_13, s_145_14);
        // D s_145_16: cast reint s_145_15 -> u8
        let s_145_16: u8 = (s_145_15.value() as u8);
        // D s_145_17: cast zx s_145_16 -> bv
        let s_145_17: Bits = Bits::new(s_145_16 as u128, 2u16);
        // C s_145_18: const #2u : u8
        let s_145_18: u8 = 2;
        // C s_145_19: cast zx s_145_18 -> bv
        let s_145_19: Bits = Bits::new(s_145_18 as u128, 2u16);
        // D s_145_20: cmp-eq s_145_17 s_145_19
        let s_145_20: bool = ((s_145_17) == (s_145_19));
        // D s_145_21: write-var gs#6459 <= s_145_20
        fn_state.gs_6459 = s_145_20;
        // N s_145_22: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #2s : i
        let s_146_0: i128 = 2;
        // D s_146_1: read-var spsr:u64
        let s_146_1: u64 = fn_state.spsr;
        // D s_146_2: cast zx s_146_1 -> bv
        let s_146_2: Bits = Bits::new(s_146_1 as u128, 64u16);
        // C s_146_3: const #2u : u8
        let s_146_3: u8 = 2;
        // C s_146_4: cast zx s_146_3 -> bv
        let s_146_4: Bits = Bits::new(s_146_3 as u128, 2u16);
        // C s_146_5: const #1s : i
        let s_146_5: i128 = 1;
        // C s_146_6: const #1u : u64
        let s_146_6: u64 = 1;
        // C s_146_7: cast zx s_146_6 -> bv
        let s_146_7: Bits = Bits::new(s_146_6 as u128, 64u16);
        // C s_146_8: lsl s_146_7 s_146_5
        let s_146_8: Bits = s_146_7 << s_146_5;
        // C s_146_9: sub s_146_8 s_146_7
        let s_146_9: Bits = ((s_146_8) - (s_146_7));
        // C s_146_10: and s_146_4 s_146_9
        let s_146_10: Bits = ((s_146_4) & (s_146_9));
        // C s_146_11: lsl s_146_10 s_146_0
        let s_146_11: Bits = s_146_10 << s_146_0;
        // C s_146_12: lsl s_146_9 s_146_0
        let s_146_12: Bits = s_146_9 << s_146_0;
        // C s_146_13: cmpl s_146_12
        let s_146_13: Bits = !s_146_12;
        // D s_146_14: and s_146_2 s_146_13
        let s_146_14: Bits = ((s_146_2) & (s_146_13));
        // D s_146_15: or s_146_14 s_146_11
        let s_146_15: Bits = ((s_146_14) | (s_146_11));
        // D s_146_16: cast reint s_146_15 -> u64
        let s_146_16: u64 = (s_146_15.value() as u64);
        // D s_146_17: write-var spsr <= s_146_16
        fn_state.spsr = s_146_16;
        // N s_146_18: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #102552u : u32
        let s_147_0: u32 = 102552;
        // D s_147_1: read-reg s_147_0:struct
        let s_147_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_147_0 as isize);
            tracer.read_register(s_147_0 as isize, value);
            value
        };
        // D s_147_2: call _get_HCR_EL2_Type_NV(s_147_1)
        let s_147_2: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_147_1);
        // C s_147_3: const #102552u : u32
        let s_147_3: u32 = 102552;
        // D s_147_4: read-reg s_147_3:struct
        let s_147_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_147_3 as isize);
            tracer.read_register(s_147_3 as isize, value);
            value
        };
        // D s_147_5: call _get_HCR_EL2_Type_NV1(s_147_4)
        let s_147_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_147_4);
        // C s_147_6: const #102552u : u32
        let s_147_6: u32 = 102552;
        // D s_147_7: read-reg s_147_6:struct
        let s_147_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_147_6 as isize);
            tracer.read_register(s_147_6 as isize, value);
            value
        };
        // D s_147_8: call _get_HCR_EL2_Type_NV2(s_147_7)
        let s_147_8: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_147_7);
        // D s_147_9: cast zx s_147_5 -> bv
        let s_147_9: Bits = Bits::new(s_147_5 as u128, 1u16);
        // D s_147_10: cast zx s_147_8 -> bv
        let s_147_10: Bits = Bits::new(s_147_8 as u128, 1u16);
        // D s_147_11: cast reint s_147_9 -> u128
        let s_147_11: u128 = (s_147_9.value() as u128);
        // D s_147_12: size-of s_147_9
        let s_147_12: u16 = s_147_9.length();
        // D s_147_13: cast reint s_147_10 -> u128
        let s_147_13: u128 = (s_147_10.value() as u128);
        // D s_147_14: size-of s_147_10
        let s_147_14: u16 = s_147_10.length();
        // D s_147_15: lsl s_147_11 s_147_14
        let s_147_15: u128 = s_147_11 << s_147_14;
        // D s_147_16: or s_147_15 s_147_13
        let s_147_16: u128 = ((s_147_15) | (s_147_13));
        // D s_147_17: add s_147_12 s_147_14
        let s_147_17: u16 = (s_147_12 + s_147_14);
        // D s_147_18: create-bits s_147_16 s_147_17
        let s_147_18: Bits = Bits::new(s_147_16, s_147_17);
        // D s_147_19: cast reint s_147_18 -> u8
        let s_147_19: u8 = (s_147_18.value() as u8);
        // D s_147_20: cast zx s_147_2 -> bv
        let s_147_20: Bits = Bits::new(s_147_2 as u128, 1u16);
        // D s_147_21: cast zx s_147_19 -> bv
        let s_147_21: Bits = Bits::new(s_147_19 as u128, 2u16);
        // D s_147_22: cast reint s_147_20 -> u128
        let s_147_22: u128 = (s_147_20.value() as u128);
        // D s_147_23: size-of s_147_20
        let s_147_23: u16 = s_147_20.length();
        // D s_147_24: cast reint s_147_21 -> u128
        let s_147_24: u128 = (s_147_21.value() as u128);
        // D s_147_25: size-of s_147_21
        let s_147_25: u16 = s_147_21.length();
        // D s_147_26: lsl s_147_22 s_147_25
        let s_147_26: u128 = s_147_22 << s_147_25;
        // D s_147_27: or s_147_26 s_147_24
        let s_147_27: u128 = ((s_147_26) | (s_147_24));
        // D s_147_28: add s_147_23 s_147_25
        let s_147_28: u16 = (s_147_23 + s_147_25);
        // D s_147_29: create-bits s_147_27 s_147_28
        let s_147_29: Bits = Bits::new(s_147_27, s_147_28);
        // D s_147_30: cast reint s_147_29 -> u8
        let s_147_30: u8 = (s_147_29.value() as u8);
        // D s_147_31: cast zx s_147_30 -> bv
        let s_147_31: Bits = Bits::new(s_147_30 as u128, 3u16);
        // C s_147_32: const #4u : u8
        let s_147_32: u8 = 4;
        // C s_147_33: cast zx s_147_32 -> bv
        let s_147_33: Bits = Bits::new(s_147_32 as u128, 3u16);
        // D s_147_34: cmp-eq s_147_31 s_147_33
        let s_147_34: bool = ((s_147_31) == (s_147_33));
        // N s_147_35: branch s_147_34 b150 b148
        if s_147_34 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #102552u : u32
        let s_148_0: u32 = 102552;
        // D s_148_1: read-reg s_148_0:struct
        let s_148_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_148_0 as isize);
            tracer.read_register(s_148_0 as isize, value);
            value
        };
        // D s_148_2: call _get_HCR_EL2_Type_NV(s_148_1)
        let s_148_2: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_148_1);
        // C s_148_3: const #102552u : u32
        let s_148_3: u32 = 102552;
        // D s_148_4: read-reg s_148_3:struct
        let s_148_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_148_3 as isize);
            tracer.read_register(s_148_3 as isize, value);
            value
        };
        // D s_148_5: call _get_HCR_EL2_Type_NV1(s_148_4)
        let s_148_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_148_4);
        // C s_148_6: const #102552u : u32
        let s_148_6: u32 = 102552;
        // D s_148_7: read-reg s_148_6:struct
        let s_148_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_148_6 as isize);
            tracer.read_register(s_148_6 as isize, value);
            value
        };
        // D s_148_8: call _get_HCR_EL2_Type_NV2(s_148_7)
        let s_148_8: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_148_7);
        // D s_148_9: cast zx s_148_5 -> bv
        let s_148_9: Bits = Bits::new(s_148_5 as u128, 1u16);
        // D s_148_10: cast zx s_148_8 -> bv
        let s_148_10: Bits = Bits::new(s_148_8 as u128, 1u16);
        // D s_148_11: cast reint s_148_9 -> u128
        let s_148_11: u128 = (s_148_9.value() as u128);
        // D s_148_12: size-of s_148_9
        let s_148_12: u16 = s_148_9.length();
        // D s_148_13: cast reint s_148_10 -> u128
        let s_148_13: u128 = (s_148_10.value() as u128);
        // D s_148_14: size-of s_148_10
        let s_148_14: u16 = s_148_10.length();
        // D s_148_15: lsl s_148_11 s_148_14
        let s_148_15: u128 = s_148_11 << s_148_14;
        // D s_148_16: or s_148_15 s_148_13
        let s_148_16: u128 = ((s_148_15) | (s_148_13));
        // D s_148_17: add s_148_12 s_148_14
        let s_148_17: u16 = (s_148_12 + s_148_14);
        // D s_148_18: create-bits s_148_16 s_148_17
        let s_148_18: Bits = Bits::new(s_148_16, s_148_17);
        // D s_148_19: cast reint s_148_18 -> u8
        let s_148_19: u8 = (s_148_18.value() as u8);
        // D s_148_20: cast zx s_148_2 -> bv
        let s_148_20: Bits = Bits::new(s_148_2 as u128, 1u16);
        // D s_148_21: cast zx s_148_19 -> bv
        let s_148_21: Bits = Bits::new(s_148_19 as u128, 2u16);
        // D s_148_22: cast reint s_148_20 -> u128
        let s_148_22: u128 = (s_148_20.value() as u128);
        // D s_148_23: size-of s_148_20
        let s_148_23: u16 = s_148_20.length();
        // D s_148_24: cast reint s_148_21 -> u128
        let s_148_24: u128 = (s_148_21.value() as u128);
        // D s_148_25: size-of s_148_21
        let s_148_25: u16 = s_148_21.length();
        // D s_148_26: lsl s_148_22 s_148_25
        let s_148_26: u128 = s_148_22 << s_148_25;
        // D s_148_27: or s_148_26 s_148_24
        let s_148_27: u128 = ((s_148_26) | (s_148_24));
        // D s_148_28: add s_148_23 s_148_25
        let s_148_28: u16 = (s_148_23 + s_148_25);
        // D s_148_29: create-bits s_148_27 s_148_28
        let s_148_29: Bits = Bits::new(s_148_27, s_148_28);
        // D s_148_30: cast reint s_148_29 -> u8
        let s_148_30: u8 = (s_148_29.value() as u8);
        // D s_148_31: cast zx s_148_30 -> bv
        let s_148_31: Bits = Bits::new(s_148_30 as u128, 3u16);
        // C s_148_32: const #7u : u8
        let s_148_32: u8 = 7;
        // C s_148_33: cast zx s_148_32 -> bv
        let s_148_33: Bits = Bits::new(s_148_32 as u128, 3u16);
        // D s_148_34: cmp-eq s_148_31 s_148_33
        let s_148_34: bool = ((s_148_31) == (s_148_33));
        // D s_148_35: write-var gs#6457 <= s_148_34
        fn_state.gs_6457 = s_148_34;
        // N s_148_36: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#6457:u8
        let s_149_0: bool = fn_state.gs_6457;
        // D s_149_1: write-var gs#6458 <= s_149_0
        fn_state.gs_6458 = s_149_0;
        // N s_149_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #1u : u8
        let s_150_0: bool = true;
        // D s_150_1: write-var gs#6457 <= s_150_0
        fn_state.gs_6457 = s_150_0;
        // N s_150_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #() : ()
        let s_151_0: () = ();
        // S s_151_1: call EL2Enabled(s_151_0)
        let s_151_1: bool = EL2Enabled(state, tracer, s_151_0);
        // D s_151_2: write-var gs#6395 <= s_151_1
        fn_state.gs_6395 = s_151_1;
        // N s_151_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var target_el:u8
        let s_152_0: u8 = fn_state.target_el;
        // D s_152_1: cast zx s_152_0 -> bv
        let s_152_1: Bits = Bits::new(s_152_0 as u128, 2u16);
        // C s_152_2: const #440u : u32
        let s_152_2: u32 = 440;
        // D s_152_3: read-reg s_152_2:u8
        let s_152_3: u8 = {
            let value = state.read_register::<u8>(s_152_2 as isize);
            tracer.read_register(s_152_2 as isize, value);
            value
        };
        // D s_152_4: cast zx s_152_3 -> bv
        let s_152_4: Bits = Bits::new(s_152_3 as u128, 2u16);
        // D s_152_5: cmp-eq s_152_1 s_152_4
        let s_152_5: bool = ((s_152_1) == (s_152_4));
        // D s_152_6: write-var gs#6394 <= s_152_5
        fn_state.gs_6394 = s_152_5;
        // N s_152_7: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #512u : u12
        let s_153_0: u16 = 512;
        // C s_153_1: cast zx s_153_0 -> bv
        let s_153_1: Bits = Bits::new(s_153_0 as u128, 12u16);
        // C s_153_2: cast zx s_153_1 -> i
        let s_153_2: i128 = (s_153_1.value() as i128);
        // C s_153_3: cast reint s_153_2 -> i64
        let s_153_3: i64 = (s_153_2 as i64);
        // C s_153_4: cast zx s_153_3 -> i
        let s_153_4: i128 = (i128::try_from(s_153_3).unwrap());
        // D s_153_5: read-var vect_offset:i
        let s_153_5: i128 = fn_state.vect_offset;
        // D s_153_6: add s_153_5 s_153_4
        let s_153_6: i128 = (s_153_5 + s_153_4);
        // D s_153_7: write-var vect_offset <= s_153_6
        fn_state.vect_offset = s_153_6;
        // N s_153_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var target_el:u8
        let s_154_0: u8 = fn_state.target_el;
        // D s_154_1: cast zx s_154_0 -> bv
        let s_154_1: Bits = Bits::new(s_154_0 as u128, 2u16);
        // C s_154_2: const #424u : u32
        let s_154_2: u32 = 424;
        // D s_154_3: read-reg s_154_2:u8
        let s_154_3: u8 = {
            let value = state.read_register::<u8>(s_154_2 as isize);
            tracer.read_register(s_154_2 as isize, value);
            value
        };
        // D s_154_4: cast zx s_154_3 -> bv
        let s_154_4: Bits = Bits::new(s_154_3 as u128, 2u16);
        // D s_154_5: cmp-eq s_154_1 s_154_4
        let s_154_5: bool = ((s_154_1) == (s_154_4));
        // N s_154_6: branch s_154_5 b168 b155
        if s_154_5 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #() : ()
        let s_155_0: () = ();
        // S s_155_1: call IsInHost(s_155_0)
        let s_155_1: bool = IsInHost(state, tracer, s_155_0);
        // N s_155_2: branch s_155_1 b167 b156
        if s_155_1 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #0u : u8
        let s_156_0: bool = false;
        // D s_156_1: write-var gs#6389 <= s_156_0
        fn_state.gs_6389 = s_156_0;
        // N s_156_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var gs#6389:u8
        let s_157_0: bool = fn_state.gs_6389;
        // N s_157_1: branch s_157_0 b166 b158
        if s_157_0 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #0u : u8
        let s_158_0: bool = false;
        // D s_158_1: write-var gs#6390 <= s_158_0
        fn_state.gs_6390 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#6390:u8
        let s_159_0: bool = fn_state.gs_6390;
        // N s_159_1: branch s_159_0 b165 b160
        if s_159_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #1s : i
        let s_160_0: i128 = 1;
        // D s_160_1: read-var target_el:u8
        let s_160_1: u8 = fn_state.target_el;
        // D s_160_2: cast zx s_160_1 -> bv
        let s_160_2: Bits = Bits::new(s_160_1 as u128, 2u16);
        // C s_160_3: cast cvt s_160_0 -> bv
        let s_160_3: Bits = Bits::new(s_160_0 as u128, 128);
        // D s_160_4: sub s_160_2 s_160_3
        let s_160_4: Bits = ((s_160_2) - (s_160_3));
        // D s_160_5: cast reint s_160_4 -> u8
        let s_160_5: u8 = (s_160_4.value() as u8);
        // D s_160_6: call ELUsingAArch32(s_160_5)
        let s_160_6: bool = ELUsingAArch32(state, tracer, s_160_5);
        // D s_160_7: write-var lower_32 <= s_160_6
        fn_state.lower_32 = s_160_6;
        // N s_160_8: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var lower_32:u8
        let s_161_0: bool = fn_state.lower_32;
        // N s_161_1: branch s_161_0 b164 b162
        if s_161_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_162(state, tracer, fn_state);
        };
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #1024u : u12
        let s_162_0: u16 = 1024;
        // C s_162_1: cast zx s_162_0 -> bv
        let s_162_1: Bits = Bits::new(s_162_0 as u128, 12u16);
        // C s_162_2: cast zx s_162_1 -> i
        let s_162_2: i128 = (s_162_1.value() as i128);
        // D s_162_3: write-var ga#4212 <= s_162_2
        fn_state.ga_4212 = s_162_2;
        // N s_162_4: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_163_0: read-var vect_offset:i
        let s_163_0: i128 = fn_state.vect_offset;
        // D s_163_1: read-var ga#4212:i
        let s_163_1: i128 = fn_state.ga_4212;
        // D s_163_2: add s_163_0 s_163_1
        let s_163_2: i128 = (s_163_0 + s_163_1);
        // D s_163_3: write-var vect_offset <= s_163_2
        fn_state.vect_offset = s_163_2;
        // N s_163_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #1536u : u12
        let s_164_0: u16 = 1536;
        // C s_164_1: cast zx s_164_0 -> bv
        let s_164_1: Bits = Bits::new(s_164_0 as u128, 12u16);
        // C s_164_2: cast zx s_164_1 -> i
        let s_164_2: i128 = (s_164_1.value() as i128);
        // D s_164_3: write-var ga#4212 <= s_164_2
        fn_state.ga_4212 = s_164_2;
        // N s_164_4: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #448u : u32
        let s_165_0: u32 = 448;
        // D s_165_1: read-reg s_165_0:u8
        let s_165_1: u8 = {
            let value = state.read_register::<u8>(s_165_0 as isize);
            tracer.read_register(s_165_0 as isize, value);
            value
        };
        // D s_165_2: call ELUsingAArch32(s_165_1)
        let s_165_2: bool = ELUsingAArch32(state, tracer, s_165_1);
        // D s_165_3: write-var lower_32 <= s_165_2
        fn_state.lower_32 = s_165_2;
        // N s_165_4: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var target_el:u8
        let s_166_0: u8 = fn_state.target_el;
        // D s_166_1: cast zx s_166_0 -> bv
        let s_166_1: Bits = Bits::new(s_166_0 as u128, 2u16);
        // C s_166_2: const #432u : u32
        let s_166_2: u32 = 432;
        // D s_166_3: read-reg s_166_2:u8
        let s_166_3: u8 = {
            let value = state.read_register::<u8>(s_166_2 as isize);
            tracer.read_register(s_166_2 as isize, value);
            value
        };
        // D s_166_4: cast zx s_166_3 -> bv
        let s_166_4: Bits = Bits::new(s_166_3 as u128, 2u16);
        // D s_166_5: cmp-eq s_166_1 s_166_4
        let s_166_5: bool = ((s_166_1) == (s_166_4));
        // D s_166_6: write-var gs#6390 <= s_166_5
        fn_state.gs_6390 = s_166_5;
        // N s_166_7: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #16975u : u32
        let s_167_0: u32 = 16975;
        // D s_167_1: read-reg s_167_0:u8
        let s_167_1: u8 = {
            let value = state.read_register::<u8>(s_167_0 as isize);
            tracer.read_register(s_167_0 as isize, value);
            value
        };
        // D s_167_2: cast zx s_167_1 -> bv
        let s_167_2: Bits = Bits::new(s_167_1 as u128, 2u16);
        // C s_167_3: const #448u : u32
        let s_167_3: u32 = 448;
        // D s_167_4: read-reg s_167_3:u8
        let s_167_4: u8 = {
            let value = state.read_register::<u8>(s_167_3 as isize);
            tracer.read_register(s_167_3 as isize, value);
            value
        };
        // D s_167_5: cast zx s_167_4 -> bv
        let s_167_5: Bits = Bits::new(s_167_4 as u128, 2u16);
        // D s_167_6: cmp-eq s_167_2 s_167_5
        let s_167_6: bool = ((s_167_2) == (s_167_5));
        // D s_167_7: write-var gs#6389 <= s_167_6
        fn_state.gs_6389 = s_167_6;
        // N s_167_8: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #() : ()
        let s_168_0: () = ();
        // S s_168_1: call EL2Enabled(s_168_0)
        let s_168_1: bool = EL2Enabled(state, tracer, s_168_0);
        // N s_168_2: branch s_168_1 b170 b169
        if s_168_1 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #440u : u32
        let s_169_0: u32 = 440;
        // D s_169_1: read-reg s_169_0:u8
        let s_169_1: u8 = {
            let value = state.read_register::<u8>(s_169_0 as isize);
            tracer.read_register(s_169_0 as isize, value);
            value
        };
        // D s_169_2: call ELUsingAArch32(s_169_1)
        let s_169_2: bool = ELUsingAArch32(state, tracer, s_169_1);
        // D s_169_3: write-var lower_32 <= s_169_2
        fn_state.lower_32 = s_169_2;
        // N s_169_4: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #432u : u32
        let s_170_0: u32 = 432;
        // D s_170_1: read-reg s_170_0:u8
        let s_170_1: u8 = {
            let value = state.read_register::<u8>(s_170_0 as isize);
            tracer.read_register(s_170_0 as isize, value);
            value
        };
        // D s_170_2: call ELUsingAArch32(s_170_1)
        let s_170_2: bool = ELUsingAArch32(state, tracer, s_170_1);
        // D s_170_3: write-var lower_32 <= s_170_2
        fn_state.lower_32 = s_170_2;
        // N s_170_4: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #() : ()
        let s_171_0: () = ();
        // S s_171_1: call ResetSVEState(s_171_0)
        let s_171_1: () = ResetSVEState(state, tracer, s_171_0);
        // N s_171_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #16989u : u32
        let s_172_0: u32 = 16989;
        // D s_172_1: read-reg s_172_0:u8
        let s_172_1: bool = {
            let value = state.read_register::<bool>(s_172_0 as isize);
            tracer.read_register(s_172_0 as isize, value);
            value
        };
        // D s_172_2: cast zx s_172_1 -> bv
        let s_172_2: Bits = Bits::new(s_172_1 as u128, 1u16);
        // C s_172_3: const #1u : u8
        let s_172_3: bool = true;
        // C s_172_4: cast zx s_172_3 -> bv
        let s_172_4: Bits = Bits::new(s_172_3 as u128, 1u16);
        // D s_172_5: cmp-eq s_172_2 s_172_4
        let s_172_5: bool = ((s_172_2) == (s_172_4));
        // D s_172_6: write-var gs#6384 <= s_172_5
        fn_state.gs_6384 = s_172_5;
        // N s_172_7: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #() : ()
        let s_173_0: () = ();
        // S s_173_1: call HaveSME(s_173_0)
        let s_173_1: bool = HaveSME(state, tracer, s_173_0);
        // D s_173_2: write-var gs#6383 <= s_173_1
        fn_state.gs_6383 = s_173_1;
        // N s_173_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #() : ()
        let s_174_0: () = ();
        // S s_174_1: call AArch64_MaybeZeroRegisterUppers(s_174_0)
        let s_174_1: () = AArch64_MaybeZeroRegisterUppers(state, tracer, s_174_0);
        // N s_174_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var except.1:struct
        let s_175_0: u32 = fn_state.except._1;
        // D s_175_1: write-var ga#4197 <= s_175_0
        fn_state.ga_4197 = s_175_0;
        // C s_175_2: const #29u : u32
        let s_175_2: u32 = 29;
        // D s_175_3: read-var ga#4197:u32
        let s_175_3: u32 = fn_state.ga_4197;
        // D s_175_4: cmp-eq s_175_2 s_175_3
        let s_175_4: bool = ((s_175_2) == (s_175_3));
        // D s_175_5: not s_175_4
        let s_175_5: bool = !s_175_4;
        // N s_175_6: branch s_175_5 b178 b176
        if s_175_5 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_176(state, tracer, fn_state);
        };
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #1u : u32
        let s_176_0: u32 = 1;
        // D s_176_1: write-var cause <= s_176_0
        fn_state.cause = s_176_0;
        // N s_176_2: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_177_0: read-var cause:u32
        let s_177_0: u32 = fn_state.cause;
        // C s_177_1: const #0u : u8
        let s_177_1: bool = false;
        // D s_177_2: call FailTransaction(s_177_0, s_177_1)
        let s_177_2: () = FailTransaction(state, tracer, s_177_0, s_177_1);
        // N s_177_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #25u : u32
        let s_178_0: u32 = 25;
        // D s_178_1: read-var ga#4197:u32
        let s_178_1: u32 = fn_state.ga_4197;
        // D s_178_2: cmp-eq s_178_0 s_178_1
        let s_178_2: bool = ((s_178_0) == (s_178_1));
        // D s_178_3: not s_178_2
        let s_178_3: bool = !s_178_2;
        // N s_178_4: branch s_178_3 b180 b179
        if s_178_3 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_179(state, tracer, fn_state);
        };
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #1u : u32
        let s_179_0: u32 = 1;
        // D s_179_1: write-var cause <= s_179_0
        fn_state.cause = s_179_0;
        // N s_179_2: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #27u : u32
        let s_180_0: u32 = 27;
        // D s_180_1: read-var ga#4197:u32
        let s_180_1: u32 = fn_state.ga_4197;
        // D s_180_2: cmp-eq s_180_0 s_180_1
        let s_180_2: bool = ((s_180_0) == (s_180_1));
        // D s_180_3: not s_180_2
        let s_180_3: bool = !s_180_2;
        // N s_180_4: branch s_180_3 b182 b181
        if s_180_3 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_181(state, tracer, fn_state);
        };
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #1u : u32
        let s_181_0: u32 = 1;
        // D s_181_1: write-var cause <= s_181_0
        fn_state.cause = s_181_0;
        // N s_181_2: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #26u : u32
        let s_182_0: u32 = 26;
        // D s_182_1: read-var ga#4197:u32
        let s_182_1: u32 = fn_state.ga_4197;
        // D s_182_2: cmp-eq s_182_0 s_182_1
        let s_182_2: bool = ((s_182_0) == (s_182_1));
        // D s_182_3: not s_182_2
        let s_182_3: bool = !s_182_2;
        // N s_182_4: branch s_182_3 b184 b183
        if s_182_3 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #1u : u32
        let s_183_0: u32 = 1;
        // D s_183_1: write-var cause <= s_183_0
        fn_state.cause = s_183_0;
        // N s_183_2: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #2u : u32
        let s_184_0: u32 = 2;
        // D s_184_1: write-var cause <= s_184_0
        fn_state.cause = s_184_0;
        // N s_184_2: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #100180u : u32
        let s_185_0: u32 = 100180;
        // D s_185_1: read-reg s_185_0:i
        let s_185_1: i128 = {
            let value = state.read_register::<i128>(s_185_0 as isize);
            tracer.read_register(s_185_0 as isize, value);
            value
        };
        // C s_185_2: const #0s : i
        let s_185_2: i128 = 0;
        // D s_185_3: cmp-gt s_185_1 s_185_2
        let s_185_3: bool = ((s_185_1) > (s_185_2));
        // D s_185_4: write-var gs#6363 <= s_185_3
        fn_state.gs_6363 = s_185_3;
        // N s_185_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var target_el:u8
        let s_186_0: u8 = fn_state.target_el;
        // D s_186_1: call SCTLR_read(s_186_0)
        let s_186_1: ProductType5c790c8ef59cc8b2 = SCTLR_read(state, tracer, s_186_0);
        // D s_186_2: call _get_SCTLRType_IESB(s_186_1)
        let s_186_2: bool = u_get_SCTLRType_IESB(state, tracer, s_186_1);
        // D s_186_3: cast zx s_186_2 -> bv
        let s_186_3: Bits = Bits::new(s_186_2 as u128, 1u16);
        // C s_186_4: const #1u : u8
        let s_186_4: bool = true;
        // C s_186_5: cast zx s_186_4 -> bv
        let s_186_5: Bits = Bits::new(s_186_4 as u128, 1u16);
        // D s_186_6: cmp-eq s_186_3 s_186_5
        let s_186_6: bool = ((s_186_3) == (s_186_5));
        // D s_186_7: write-var sync_errors <= s_186_6
        fn_state.sync_errors = s_186_6;
        // C s_186_8: const #() : ()
        let s_186_8: () = ();
        // S s_186_9: call HaveDoubleFaultExt(s_186_8)
        let s_186_9: bool = HaveDoubleFaultExt(state, tracer, s_186_8);
        // N s_186_10: branch s_186_9 b195 b187
        if s_186_9 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_187_0: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var sync_errors:u8
        let s_188_0: bool = fn_state.sync_errors;
        // N s_188_1: branch s_188_0 b194 b189
        if s_188_0 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_189(state, tracer, fn_state);
        };
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #0u : u8
        let s_189_0: bool = false;
        // D s_189_1: write-var gs#6374 <= s_189_0
        fn_state.gs_6374 = s_189_0;
        // N s_189_2: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var gs#6374:u8
        let s_190_0: bool = fn_state.gs_6374;
        // N s_190_1: branch s_190_0 b193 b191
        if s_190_0 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_191(state, tracer, fn_state);
        };
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_191_0: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_192_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #() : ()
        let s_193_0: () = ();
        // S s_193_1: call SynchronizeErrors(s_193_0)
        let s_193_1: () = SynchronizeErrors(state, tracer, s_193_0);
        // C s_193_2: const #0u : u8
        let s_193_2: bool = false;
        // C s_193_3: const #0u : u8
        let s_193_3: bool = false;
        // D s_193_4: write-var sync_errors <= s_193_3
        fn_state.sync_errors = s_193_3;
        // S s_193_5: call TakeUnmaskedPhysicalSErrorInterrupts(s_193_2)
        let s_193_5: () = TakeUnmaskedPhysicalSErrorInterrupts(state, tracer, s_193_2);
        // N s_193_6: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var target_el:u8
        let s_194_0: u8 = fn_state.target_el;
        // D s_194_1: call InsertIESBBeforeException(s_194_0)
        let s_194_1: bool = InsertIESBBeforeException(state, tracer, s_194_0);
        // D s_194_2: write-var gs#6374 <= s_194_1
        fn_state.gs_6374 = s_194_1;
        // N s_194_3: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_195_0: read-var sync_errors:u8
        let s_195_0: bool = fn_state.sync_errors;
        // N s_195_1: branch s_195_0 b201 b196
        if s_195_0 {
            return block_201(state, tracer, fn_state);
        } else {
            return block_196(state, tracer, fn_state);
        };
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #90704u : u32
        let s_196_0: u32 = 90704;
        // D s_196_1: read-reg s_196_0:struct
        let s_196_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_196_0 as isize);
            tracer.read_register(s_196_0 as isize, value);
            value
        };
        // D s_196_2: call _get_SCR_EL3_Type_EA(s_196_1)
        let s_196_2: bool = u_get_SCR_EL3_Type_EA(state, tracer, s_196_1);
        // C s_196_3: const #90704u : u32
        let s_196_3: u32 = 90704;
        // D s_196_4: read-reg s_196_3:struct
        let s_196_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_196_3 as isize);
            tracer.read_register(s_196_3 as isize, value);
            value
        };
        // D s_196_5: call _get_SCR_EL3_Type_NMEA(s_196_4)
        let s_196_5: bool = u_get_SCR_EL3_Type_NMEA(state, tracer, s_196_4);
        // D s_196_6: cast zx s_196_2 -> bv
        let s_196_6: Bits = Bits::new(s_196_2 as u128, 1u16);
        // D s_196_7: cast zx s_196_5 -> bv
        let s_196_7: Bits = Bits::new(s_196_5 as u128, 1u16);
        // D s_196_8: cast reint s_196_6 -> u128
        let s_196_8: u128 = (s_196_6.value() as u128);
        // D s_196_9: size-of s_196_6
        let s_196_9: u16 = s_196_6.length();
        // D s_196_10: cast reint s_196_7 -> u128
        let s_196_10: u128 = (s_196_7.value() as u128);
        // D s_196_11: size-of s_196_7
        let s_196_11: u16 = s_196_7.length();
        // D s_196_12: lsl s_196_8 s_196_11
        let s_196_12: u128 = s_196_8 << s_196_11;
        // D s_196_13: or s_196_12 s_196_10
        let s_196_13: u128 = ((s_196_12) | (s_196_10));
        // D s_196_14: add s_196_9 s_196_11
        let s_196_14: u16 = (s_196_9 + s_196_11);
        // D s_196_15: create-bits s_196_13 s_196_14
        let s_196_15: Bits = Bits::new(s_196_13, s_196_14);
        // D s_196_16: cast reint s_196_15 -> u8
        let s_196_16: u8 = (s_196_15.value() as u8);
        // D s_196_17: cast zx s_196_16 -> bv
        let s_196_17: Bits = Bits::new(s_196_16 as u128, 2u16);
        // C s_196_18: const #3u : u8
        let s_196_18: u8 = 3;
        // C s_196_19: cast zx s_196_18 -> bv
        let s_196_19: Bits = Bits::new(s_196_18 as u128, 2u16);
        // D s_196_20: cmp-eq s_196_17 s_196_19
        let s_196_20: bool = ((s_196_17) == (s_196_19));
        // N s_196_21: branch s_196_20 b200 b197
        if s_196_20 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_197(state, tracer, fn_state);
        };
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #0u : u8
        let s_197_0: bool = false;
        // D s_197_1: write-var gs#6380 <= s_197_0
        fn_state.gs_6380 = s_197_0;
        // N s_197_2: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var gs#6380:u8
        let s_198_0: bool = fn_state.gs_6380;
        // D s_198_1: write-var gs#6381 <= s_198_0
        fn_state.gs_6381 = s_198_0;
        // N s_198_2: jump b199
        return block_199(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_199_0: read-var gs#6381:u8
        let s_199_0: bool = fn_state.gs_6381;
        // D s_199_1: write-var sync_errors <= s_199_0
        fn_state.sync_errors = s_199_0;
        // N s_199_2: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var target_el:u8
        let s_200_0: u8 = fn_state.target_el;
        // D s_200_1: cast zx s_200_0 -> bv
        let s_200_1: Bits = Bits::new(s_200_0 as u128, 2u16);
        // C s_200_2: const #424u : u32
        let s_200_2: u32 = 424;
        // D s_200_3: read-reg s_200_2:u8
        let s_200_3: u8 = {
            let value = state.read_register::<u8>(s_200_2 as isize);
            tracer.read_register(s_200_2 as isize, value);
            value
        };
        // D s_200_4: cast zx s_200_3 -> bv
        let s_200_4: Bits = Bits::new(s_200_3 as u128, 2u16);
        // D s_200_5: cmp-eq s_200_1 s_200_4
        let s_200_5: bool = ((s_200_1) == (s_200_4));
        // D s_200_6: write-var gs#6380 <= s_200_5
        fn_state.gs_6380 = s_200_5;
        // N s_200_7: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #1u : u8
        let s_201_0: bool = true;
        // D s_201_1: write-var gs#6381 <= s_201_0
        fn_state.gs_6381 = s_201_0;
        // N s_201_2: jump b199
        return block_199(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var target_el:u8
        let s_202_0: u8 = fn_state.target_el;
        // D s_202_1: read-var exception_in:struct
        let s_202_1: ProductTypeb7f99f96751e17c4 = fn_state.exception_in;
        // D s_202_2: call AArch64_TakeExceptionInDebugState(s_202_0, s_202_1)
        let s_202_2: () = AArch64_TakeExceptionInDebugState(
            state,
            tracer,
            s_202_0,
            s_202_1,
        );
        // N s_202_3: return
        return;
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_203_0: read-var target_el:u8
        let s_203_0: u8 = fn_state.target_el;
        // D s_203_1: cast zx s_203_0 -> bv
        let s_203_1: Bits = Bits::new(s_203_0 as u128, 2u16);
        // D s_203_2: cast zx s_203_1 -> i
        let s_203_2: i128 = (s_203_1.value() as i128);
        // D s_203_3: cast reint s_203_2 -> i64
        let s_203_3: i64 = (s_203_2 as i64);
        // C s_203_4: const #16975u : u32
        let s_203_4: u32 = 16975;
        // D s_203_5: read-reg s_203_4:u8
        let s_203_5: u8 = {
            let value = state.read_register::<u8>(s_203_4 as isize);
            tracer.read_register(s_203_4 as isize, value);
            value
        };
        // D s_203_6: cast zx s_203_5 -> bv
        let s_203_6: Bits = Bits::new(s_203_5 as u128, 2u16);
        // D s_203_7: cast zx s_203_6 -> i
        let s_203_7: i128 = (s_203_6.value() as i128);
        // D s_203_8: cast reint s_203_7 -> i64
        let s_203_8: i64 = (s_203_7 as i64);
        // D s_203_9: cast zx s_203_3 -> i
        let s_203_9: i128 = (i128::try_from(s_203_3).unwrap());
        // D s_203_10: cast zx s_203_8 -> i
        let s_203_10: i128 = (i128::try_from(s_203_8).unwrap());
        // D s_203_11: cmp-ge s_203_9 s_203_10
        let s_203_11: bool = ((s_203_9) >= (s_203_10));
        // D s_203_12: write-var gs#6358 <= s_203_11
        fn_state.gs_6358 = s_203_11;
        // N s_203_13: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_204_0: read-var target_el:u8
        let s_204_0: u8 = fn_state.target_el;
        // D s_204_1: call ELUsingAArch32(s_204_0)
        let s_204_1: bool = ELUsingAArch32(state, tracer, s_204_0);
        // D s_204_2: not s_204_1
        let s_204_2: bool = !s_204_1;
        // D s_204_3: write-var gs#6357 <= s_204_2
        fn_state.gs_6357 = s_204_2;
        // N s_204_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
