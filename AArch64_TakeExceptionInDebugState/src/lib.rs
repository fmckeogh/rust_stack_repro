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
use EDSCR_write::*;
use FailTransaction::*;
use UpdateEDSCRFields::*;
use HaveGCS::*;
use HaveTME::*;
use Mk_DSPSR_EL0_Type::*;
use HaveSME::*;
use u__UNKNOWN_bits::*;
use SCTLR_read__1::*;
use ResetSVEState::*;
use EndOfInstruction::*;
use u_get_SCR_EL3_Type_NMEA::*;
use u_get_EDSCR_Type_SDD::*;
use AArch64_MaybeZeroRegisterUppers::*;
use MaybeZeroSVEUppers::*;
use ConstrainUnpredictableBool::*;
use SCTLR_read::*;
use u__UNKNOWN_bit::*;
use u_get_SCTLRType_SPAN::*;
use HaveDoubleFaultExt::*;
use u_get_SCTLRType_IESB::*;
use HaveBTIExt::*;
use ELIsInHost::*;
use HavePANExt::*;
use AArch64_ReportException::*;
use ELR_set__1::*;
use SPSR_set::*;
use HaveUAOExt::*;
use HaveIESB::*;
use HaveSSBSExt::*;
use HaveMTEExt::*;
use SynchronizeErrors::*;
use UsingAArch32::*;
use SynchronizeContext::*;
use u_get_SCR_EL3_Type_EA::*;
use ELUsingAArch32::*;
use EDSCR_read::*;
use u_update_EDSCR_Type_ERR::*;
use common::*;
pub fn AArch64_TakeExceptionInDebugState<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_el: u8,
    exception_in: ProductTypeb7f99f96751e17c4,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_6096: bool,
        gs_6113: bool,
        from_32: bool,
        gs_6137: bool,
        gs_6116: bool,
        gs_6114: bool,
        cause: u32,
        gs_6117: bool,
        gs_6136: bool,
        gs_6100: bool,
        gs_6095: bool,
        sync_errors: bool,
        except: ProductTypeb7f99f96751e17c4,
        ga_4038: u32,
        gs_6139: bool,
        gs_6138: bool,
        gs_6094: bool,
        target_el: u8,
        exception_in: ProductTypeb7f99f96751e17c4,
    }
    let fn_state = FunctionState {
        target_el,
        exception_in,
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
        // N s_0_3: branch s_0_2 b87 b1
        if s_0_2 {
            return block_87(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#6094 <= s_1_0
        fn_state.gs_6094 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#6094:u8
        let s_2_0: bool = fn_state.gs_6094;
        // N s_2_1: branch s_2_0 b86 b3
        if s_2_0 {
            return block_86(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#6095 <= s_3_0
        fn_state.gs_6095 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#6095:u8
        let s_4_0: bool = fn_state.gs_6095;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // D s_4_2: read-var target_el:u8
        let s_4_2: u8 = fn_state.target_el;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // C s_4_4: const #424u : u32
        let s_4_4: u32 = 424;
        // D s_4_5: read-reg s_4_4:u8
        let s_4_5: u8 = {
            let value = state.read_register::<u8>(s_4_4 as isize);
            tracer.read_register(s_4_4 as isize, value);
            value
        };
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 2u16);
        // D s_4_7: cmp-ne s_4_3 s_4_6
        let s_4_7: bool = ((s_4_3) != (s_4_6));
        // N s_4_8: branch s_4_7 b85 b5
        if s_4_7 {
            return block_85(state, tracer, fn_state);
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
        // S s_5_1: call EDSCR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_5_0);
        // S s_5_2: call _get_EDSCR_Type_SDD(s_5_1)
        let s_5_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_5_1);
        // S s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // C s_5_4: const #0u : u8
        let s_5_4: bool = false;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // S s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // D s_5_7: write-var gs#6096 <= s_5_6
        fn_state.gs_6096 = s_5_6;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#6096:u8
        let s_6_0: bool = fn_state.gs_6096;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // D s_6_2: read-var exception_in:struct
        let s_6_2: ProductTypeb7f99f96751e17c4 = fn_state.exception_in;
        // D s_6_3: write-var except <= s_6_2
        fn_state.except = s_6_2;
        // C s_6_4: const #() : ()
        let s_6_4: () = ();
        // S s_6_5: call HaveIESB(s_6_4)
        let s_6_5: bool = HaveIESB(state, tracer, s_6_4);
        // N s_6_6: branch s_6_5 b72 b7
        if s_6_5 {
            return block_72(state, tracer, fn_state);
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
        // D s_7_1: write-var sync_errors <= s_7_0
        fn_state.sync_errors = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HaveTME(s_8_0)
        let s_8_1: bool = HaveTME(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b71 b9
        if s_8_1 {
            return block_71(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#6100 <= s_9_0
        fn_state.gs_6100 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#6100:u8
        let s_10_0: bool = fn_state.gs_6100;
        // N s_10_1: branch s_10_0 b61 b11
        if s_10_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call SynchronizeContext(s_12_0)
        let s_12_1: () = SynchronizeContext(state, tracer, s_12_0);
        // C s_12_2: const #() : ()
        let s_12_2: () = ();
        // S s_12_3: call UsingAArch32(s_12_2)
        let s_12_3: bool = UsingAArch32(state, tracer, s_12_2);
        // D s_12_4: write-var from_32 <= s_12_3
        fn_state.from_32 = s_12_3;
        // D s_12_5: read-var from_32:u8
        let s_12_5: bool = fn_state.from_32;
        // N s_12_6: branch s_12_5 b60 b13
        if s_12_5 {
            return block_60(state, tracer, fn_state);
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
        // D s_14_0: read-var from_32:u8
        let s_14_0: bool = fn_state.from_32;
        // N s_14_1: branch s_14_0 b59 b15
        if s_14_0 {
            return block_59(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#6116 <= s_15_0
        fn_state.gs_6116 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#6116:u8
        let s_16_0: bool = fn_state.gs_6116;
        // N s_16_1: branch s_16_0 b58 b17
        if s_16_0 {
            return block_58(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#6117 <= s_17_0
        fn_state.gs_6117 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#6117:u8
        let s_18_0: bool = fn_state.gs_6117;
        // N s_18_1: branch s_18_0 b57 b19
        if s_18_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var target_el:u8
        let s_19_0: u8 = fn_state.target_el;
        // D s_19_1: call MaybeZeroSVEUppers(s_19_0)
        let s_19_1: () = MaybeZeroSVEUppers(state, tracer, s_19_0);
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var except:struct
        let s_20_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_20_1: read-var target_el:u8
        let s_20_1: u8 = fn_state.target_el;
        // D s_20_2: call AArch64_ReportException(s_20_0, s_20_1)
        let s_20_2: () = AArch64_ReportException(state, tracer, s_20_0, s_20_1);
        // C s_20_3: const #() : ()
        let s_20_3: () = ();
        // S s_20_4: call HaveGCS(s_20_3)
        let s_20_4: bool = HaveGCS(state, tracer, s_20_3);
        // N s_20_5: branch s_20_4 b56 b21
        if s_20_4 {
            return block_56(state, tracer, fn_state);
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
        // D s_22_0: read-var target_el:u8
        let s_22_0: u8 = fn_state.target_el;
        // C s_22_1: const #16975u : u32
        let s_22_1: u32 = 16975;
        // N s_22_2: write-reg s_22_1 <= s_22_0
        let s_22_2: () = {
            state.write_register::<u8>(s_22_1 as isize, s_22_0);
            tracer.write_register(s_22_1 as isize, s_22_0);
        };
        // C s_22_3: const #0u : u8
        let s_22_3: bool = false;
        // C s_22_4: const #16999u : u32
        let s_22_4: u32 = 16999;
        // N s_22_5: write-reg s_22_4 <= s_22_3
        let s_22_5: () = {
            state.write_register::<bool>(s_22_4 as isize, s_22_3);
            tracer.write_register(s_22_4 as isize, s_22_3);
        };
        // C s_22_6: const #1u : u8
        let s_22_6: bool = true;
        // C s_22_7: const #16990u : u32
        let s_22_7: u32 = 16990;
        // N s_22_8: write-reg s_22_7 <= s_22_6
        let s_22_8: () = {
            state.write_register::<bool>(s_22_7 as isize, s_22_6);
            tracer.write_register(s_22_7 as isize, s_22_6);
        };
        // C s_22_9: const #64s : i64
        let s_22_9: i64 = 64;
        // C s_22_10: cast zx s_22_9 -> i
        let s_22_10: i128 = (i128::try_from(s_22_9).unwrap());
        // S s_22_11: call __UNKNOWN_bits(s_22_10)
        let s_22_11: Bits = u__UNKNOWN_bits(state, tracer, s_22_10);
        // S s_22_12: cast reint s_22_11 -> u64
        let s_22_12: u64 = (s_22_11.value() as u64);
        // C s_22_13: const #64s : i
        let s_22_13: i128 = 64;
        // S s_22_14: cast zx s_22_12 -> bv
        let s_22_14: Bits = Bits::new(s_22_12 as u128, 64u16);
        // S s_22_15: call SPSR_set(s_22_13, s_22_14)
        let s_22_15: () = SPSR_set(state, tracer, s_22_13, s_22_14);
        // C s_22_16: const #64s : i64
        let s_22_16: i64 = 64;
        // C s_22_17: cast zx s_22_16 -> i
        let s_22_17: i128 = (i128::try_from(s_22_16).unwrap());
        // S s_22_18: call __UNKNOWN_bits(s_22_17)
        let s_22_18: Bits = u__UNKNOWN_bits(state, tracer, s_22_17);
        // S s_22_19: cast reint s_22_18 -> u64
        let s_22_19: u64 = (s_22_18.value() as u64);
        // S s_22_20: call ELR_set__1(s_22_19)
        let s_22_20: () = ELR_set__1(state, tracer, s_22_19);
        // C s_22_21: const #5s : i64
        let s_22_21: i64 = 5;
        // C s_22_22: cast zx s_22_21 -> i
        let s_22_22: i128 = (i128::try_from(s_22_21).unwrap());
        // S s_22_23: call __UNKNOWN_bits(s_22_22)
        let s_22_23: Bits = u__UNKNOWN_bits(state, tracer, s_22_22);
        // S s_22_24: cast reint s_22_23 -> u8
        let s_22_24: u8 = (s_22_23.value() as u8);
        // C s_22_25: const #4s : i
        let s_22_25: i128 = 4;
        // S s_22_26: cast zx s_22_24 -> bv
        let s_22_26: Bits = Bits::new(s_22_24 as u128, 5u16);
        // C s_22_27: const #1s : i64
        let s_22_27: i64 = 1;
        // C s_22_28: cast zx s_22_27 -> i
        let s_22_28: i128 = (i128::try_from(s_22_27).unwrap());
        // C s_22_29: const #0s : i
        let s_22_29: i128 = 0;
        // C s_22_30: add s_22_29 s_22_28
        let s_22_30: i128 = (s_22_29 + s_22_28);
        // D s_22_31: bit-extract s_22_26 s_22_25 s_22_30
        let s_22_31: Bits = (Bits::new(
            ((s_22_26) >> (s_22_25)).value(),
            u16::try_from(s_22_30).unwrap(),
        ));
        // D s_22_32: cast reint s_22_31 -> u8
        let s_22_32: bool = ((s_22_31.value()) != 0);
        // C s_22_33: const #16991u : u32
        let s_22_33: u32 = 16991;
        // N s_22_34: write-reg s_22_33 <= s_22_32
        let s_22_34: () = {
            state.write_register::<bool>(s_22_33 as isize, s_22_32);
            tracer.write_register(s_22_33 as isize, s_22_32);
        };
        // C s_22_35: const #3s : i
        let s_22_35: i128 = 3;
        // S s_22_36: cast zx s_22_24 -> bv
        let s_22_36: Bits = Bits::new(s_22_24 as u128, 5u16);
        // C s_22_37: const #1s : i64
        let s_22_37: i64 = 1;
        // C s_22_38: cast zx s_22_37 -> i
        let s_22_38: i128 = (i128::try_from(s_22_37).unwrap());
        // C s_22_39: const #0s : i
        let s_22_39: i128 = 0;
        // C s_22_40: add s_22_39 s_22_38
        let s_22_40: i128 = (s_22_39 + s_22_38);
        // D s_22_41: bit-extract s_22_36 s_22_35 s_22_40
        let s_22_41: Bits = (Bits::new(
            ((s_22_36) >> (s_22_35)).value(),
            u16::try_from(s_22_40).unwrap(),
        ));
        // D s_22_42: cast reint s_22_41 -> u8
        let s_22_42: bool = ((s_22_41.value()) != 0);
        // C s_22_43: const #16972u : u32
        let s_22_43: u32 = 16972;
        // N s_22_44: write-reg s_22_43 <= s_22_42
        let s_22_44: () = {
            state.write_register::<bool>(s_22_43 as isize, s_22_42);
            tracer.write_register(s_22_43 as isize, s_22_42);
        };
        // C s_22_45: const #2s : i
        let s_22_45: i128 = 2;
        // S s_22_46: cast zx s_22_24 -> bv
        let s_22_46: Bits = Bits::new(s_22_24 as u128, 5u16);
        // C s_22_47: const #1s : i64
        let s_22_47: i64 = 1;
        // C s_22_48: cast zx s_22_47 -> i
        let s_22_48: i128 = (i128::try_from(s_22_47).unwrap());
        // C s_22_49: const #0s : i
        let s_22_49: i128 = 0;
        // C s_22_50: add s_22_49 s_22_48
        let s_22_50: i128 = (s_22_49 + s_22_48);
        // D s_22_51: bit-extract s_22_46 s_22_45 s_22_50
        let s_22_51: Bits = (Bits::new(
            ((s_22_46) >> (s_22_45)).value(),
            u16::try_from(s_22_50).unwrap(),
        ));
        // D s_22_52: cast reint s_22_51 -> u8
        let s_22_52: bool = ((s_22_51.value()) != 0);
        // C s_22_53: const #16968u : u32
        let s_22_53: u32 = 16968;
        // N s_22_54: write-reg s_22_53 <= s_22_52
        let s_22_54: () = {
            state.write_register::<bool>(s_22_53 as isize, s_22_52);
            tracer.write_register(s_22_53 as isize, s_22_52);
        };
        // C s_22_55: const #1s : i
        let s_22_55: i128 = 1;
        // S s_22_56: cast zx s_22_24 -> bv
        let s_22_56: Bits = Bits::new(s_22_24 as u128, 5u16);
        // C s_22_57: const #1s : i64
        let s_22_57: i64 = 1;
        // C s_22_58: cast zx s_22_57 -> i
        let s_22_58: i128 = (i128::try_from(s_22_57).unwrap());
        // C s_22_59: const #0s : i
        let s_22_59: i128 = 0;
        // C s_22_60: add s_22_59 s_22_58
        let s_22_60: i128 = (s_22_59 + s_22_58);
        // D s_22_61: bit-extract s_22_56 s_22_55 s_22_60
        let s_22_61: Bits = (Bits::new(
            ((s_22_56) >> (s_22_55)).value(),
            u16::try_from(s_22_60).unwrap(),
        ));
        // D s_22_62: cast reint s_22_61 -> u8
        let s_22_62: bool = ((s_22_61.value()) != 0);
        // C s_22_63: const #16979u : u32
        let s_22_63: u32 = 16979;
        // N s_22_64: write-reg s_22_63 <= s_22_62
        let s_22_64: () = {
            state.write_register::<bool>(s_22_63 as isize, s_22_62);
            tracer.write_register(s_22_63 as isize, s_22_62);
        };
        // C s_22_65: const #0s : i
        let s_22_65: i128 = 0;
        // S s_22_66: cast zx s_22_24 -> bv
        let s_22_66: Bits = Bits::new(s_22_24 as u128, 5u16);
        // C s_22_67: const #1s : i64
        let s_22_67: i64 = 1;
        // C s_22_68: cast zx s_22_67 -> i
        let s_22_68: i128 = (i128::try_from(s_22_67).unwrap());
        // C s_22_69: const #0s : i
        let s_22_69: i128 = 0;
        // C s_22_70: add s_22_69 s_22_68
        let s_22_70: i128 = (s_22_69 + s_22_68);
        // D s_22_71: bit-extract s_22_66 s_22_65 s_22_70
        let s_22_71: Bits = (Bits::new(
            ((s_22_66) >> (s_22_65)).value(),
            u16::try_from(s_22_70).unwrap(),
        ));
        // D s_22_72: cast reint s_22_71 -> u8
        let s_22_72: bool = ((s_22_71.value()) != 0);
        // C s_22_73: const #16977u : u32
        let s_22_73: u32 = 16977;
        // N s_22_74: write-reg s_22_73 <= s_22_72
        let s_22_74: () = {
            state.write_register::<bool>(s_22_73 as isize, s_22_72);
            tracer.write_register(s_22_73 as isize, s_22_72);
        };
        // C s_22_75: const #0u : u8
        let s_22_75: bool = false;
        // C s_22_76: const #16980u : u32
        let s_22_76: u32 = 16980;
        // N s_22_77: write-reg s_22_76 <= s_22_75
        let s_22_77: () = {
            state.write_register::<bool>(s_22_76 as isize, s_22_75);
            tracer.write_register(s_22_76 as isize, s_22_75);
        };
        // D s_22_78: read-var from_32:u8
        let s_22_78: bool = fn_state.from_32;
        // N s_22_79: branch s_22_78 b55 b23
        if s_22_78 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HavePANExt(s_24_0)
        let s_24_1: bool = HavePANExt(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b48 b25
        if s_24_1 {
            return block_48(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#6138 <= s_25_0
        fn_state.gs_6138 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#6138:u8
        let s_26_0: bool = fn_state.gs_6138;
        // N s_26_1: branch s_26_0 b47 b27
        if s_26_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#6139 <= s_27_0
        fn_state.gs_6139 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#6139:u8
        let s_28_0: bool = fn_state.gs_6139;
        // N s_28_1: branch s_28_0 b46 b29
        if s_28_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call HaveUAOExt(s_30_0)
        let s_30_1: bool = HaveUAOExt(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b45 b31
        if s_30_1 {
            return block_45(state, tracer, fn_state);
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
        // S s_32_1: call HaveBTIExt(s_32_0)
        let s_32_1: bool = HaveBTIExt(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b44 b33
        if s_32_1 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call HaveSSBSExt(s_34_0)
        let s_34_1: bool = HaveSSBSExt(state, tracer, s_34_0);
        // N s_34_2: branch s_34_1 b43 b35
        if s_34_1 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call HaveMTEExt(s_36_0)
        let s_36_1: bool = HaveMTEExt(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b42 b37
        if s_36_1 {
            return block_42(state, tracer, fn_state);
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
        // C s_38_0: const #64s : i64
        let s_38_0: i64 = 64;
        // C s_38_1: cast zx s_38_0 -> i
        let s_38_1: i128 = (i128::try_from(s_38_0).unwrap());
        // S s_38_2: call __UNKNOWN_bits(s_38_1)
        let s_38_2: Bits = u__UNKNOWN_bits(state, tracer, s_38_1);
        // S s_38_3: cast reint s_38_2 -> u64
        let s_38_3: u64 = (s_38_2.value() as u64);
        // C s_38_4: const #13360u : u32
        let s_38_4: u32 = 13360;
        // N s_38_5: write-reg s_38_4 <= s_38_3
        let s_38_5: () = {
            state.write_register::<u64>(s_38_4 as isize, s_38_3);
            tracer.write_register(s_38_4 as isize, s_38_3);
        };
        // C s_38_6: const #64s : i64
        let s_38_6: i64 = 64;
        // C s_38_7: cast zx s_38_6 -> i
        let s_38_7: i128 = (i128::try_from(s_38_6).unwrap());
        // S s_38_8: call __UNKNOWN_bits(s_38_7)
        let s_38_8: Bits = u__UNKNOWN_bits(state, tracer, s_38_7);
        // S s_38_9: cast reint s_38_8 -> u64
        let s_38_9: u64 = (s_38_8.value() as u64);
        // S s_38_10: call Mk_DSPSR_EL0_Type(s_38_9)
        let s_38_10: ProductType5c790c8ef59cc8b2 = Mk_DSPSR_EL0_Type(
            state,
            tracer,
            s_38_9,
        );
        // C s_38_11: const #102584u : u32
        let s_38_11: u32 = 102584;
        // N s_38_12: write-reg s_38_11 <= s_38_10
        let s_38_12: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_38_11 as isize, s_38_10);
            tracer.write_register(s_38_11 as isize, s_38_10);
        };
        // C s_38_13: const #() : ()
        let s_38_13: () = ();
        // S s_38_14: call EDSCR_read(s_38_13)
        let s_38_14: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_38_13);
        // C s_38_15: const #1u : u8
        let s_38_15: bool = true;
        // S s_38_16: call _update_EDSCR_Type_ERR(s_38_14, s_38_15)
        let s_38_16: ProductType700c18a878c5601b = u_update_EDSCR_Type_ERR(
            state,
            tracer,
            s_38_14,
            s_38_15,
        );
        // S s_38_17: call EDSCR_write(s_38_16)
        let s_38_17: () = EDSCR_write(state, tracer, s_38_16);
        // C s_38_18: const #() : ()
        let s_38_18: () = ();
        // S s_38_19: call UpdateEDSCRFields(s_38_18)
        let s_38_19: () = UpdateEDSCRFields(state, tracer, s_38_18);
        // D s_38_20: read-var sync_errors:u8
        let s_38_20: bool = fn_state.sync_errors;
        // N s_38_21: branch s_38_20 b41 b39
        if s_38_20 {
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
        // N s_39_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EndOfInstruction(s_40_0)
        let s_40_1: () = EndOfInstruction(state, tracer, s_40_0);
        // N s_40_2: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call SynchronizeErrors(s_41_0)
        let s_41_1: () = SynchronizeErrors(state, tracer, s_41_0);
        // N s_41_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // C s_42_1: const #16994u : u32
        let s_42_1: u32 = 16994;
        // N s_42_2: write-reg s_42_1 <= s_42_0
        let s_42_2: () = {
            state.write_register::<bool>(s_42_1 as isize, s_42_0);
            tracer.write_register(s_42_1 as isize, s_42_0);
        };
        // N s_42_3: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call __UNKNOWN_bit(s_43_0)
        let s_43_1: bool = u__UNKNOWN_bit(state, tracer, s_43_0);
        // C s_43_2: const #16992u : u32
        let s_43_2: u32 = 16992;
        // N s_43_3: write-reg s_43_2 <= s_43_1
        let s_43_3: () = {
            state.write_register::<bool>(s_43_2 as isize, s_43_1);
            tracer.write_register(s_43_2 as isize, s_43_1);
        };
        // N s_43_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: u8 = 0;
        // C s_44_1: const #16970u : u32
        let s_44_1: u32 = 16970;
        // N s_44_2: write-reg s_44_1 <= s_44_0
        let s_44_2: () = {
            state.write_register::<u8>(s_44_1 as isize, s_44_0);
            tracer.write_register(s_44_1 as isize, s_44_0);
        };
        // N s_44_3: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // C s_45_1: const #16995u : u32
        let s_45_1: u32 = 16995;
        // N s_45_2: write-reg s_45_1 <= s_45_0
        let s_45_2: () = {
            state.write_register::<bool>(s_45_1 as isize, s_45_0);
            tracer.write_register(s_45_1 as isize, s_45_0);
        };
        // N s_45_3: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // C s_46_1: const #16985u : u32
        let s_46_1: u32 = 16985;
        // N s_46_2: write-reg s_46_1 <= s_46_0
        let s_46_2: () = {
            state.write_register::<bool>(s_46_1 as isize, s_46_0);
            tracer.write_register(s_46_1 as isize, s_46_0);
        };
        // N s_46_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call SCTLR_read__1(s_47_0)
        let s_47_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_47_0);
        // S s_47_2: call _get_SCTLRType_SPAN(s_47_1)
        let s_47_2: bool = u_get_SCTLRType_SPAN(state, tracer, s_47_1);
        // S s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // C s_47_4: const #0u : u8
        let s_47_4: bool = false;
        // C s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 1u16);
        // S s_47_6: cmp-eq s_47_3 s_47_5
        let s_47_6: bool = ((s_47_3) == (s_47_5));
        // D s_47_7: write-var gs#6139 <= s_47_6
        fn_state.gs_6139 = s_47_6;
        // N s_47_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #16975u : u32
        let s_48_0: u32 = 16975;
        // D s_48_1: read-reg s_48_0:u8
        let s_48_1: u8 = {
            let value = state.read_register::<u8>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: cast zx s_48_1 -> bv
        let s_48_2: Bits = Bits::new(s_48_1 as u128, 2u16);
        // C s_48_3: const #440u : u32
        let s_48_3: u32 = 440;
        // D s_48_4: read-reg s_48_3:u8
        let s_48_4: u8 = {
            let value = state.read_register::<u8>(s_48_3 as isize);
            tracer.read_register(s_48_3 as isize, value);
            value
        };
        // D s_48_5: cast zx s_48_4 -> bv
        let s_48_5: Bits = Bits::new(s_48_4 as u128, 2u16);
        // D s_48_6: cmp-eq s_48_2 s_48_5
        let s_48_6: bool = ((s_48_2) == (s_48_5));
        // N s_48_7: branch s_48_6 b54 b49
        if s_48_6 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #16975u : u32
        let s_49_0: u32 = 16975;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: cast zx s_49_1 -> bv
        let s_49_2: Bits = Bits::new(s_49_1 as u128, 2u16);
        // C s_49_3: const #432u : u32
        let s_49_3: u32 = 432;
        // D s_49_4: read-reg s_49_3:u8
        let s_49_4: u8 = {
            let value = state.read_register::<u8>(s_49_3 as isize);
            tracer.read_register(s_49_3 as isize, value);
            value
        };
        // D s_49_5: cast zx s_49_4 -> bv
        let s_49_5: Bits = Bits::new(s_49_4 as u128, 2u16);
        // D s_49_6: cmp-eq s_49_2 s_49_5
        let s_49_6: bool = ((s_49_2) == (s_49_5));
        // N s_49_7: branch s_49_6 b53 b50
        if s_49_6 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#6136 <= s_50_0
        fn_state.gs_6136 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#6136:u8
        let s_51_0: bool = fn_state.gs_6136;
        // D s_51_1: write-var gs#6137 <= s_51_0
        fn_state.gs_6137 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#6137:u8
        let s_52_0: bool = fn_state.gs_6137;
        // D s_52_1: write-var gs#6138 <= s_52_0
        fn_state.gs_6138 = s_52_0;
        // N s_52_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #448u : u32
        let s_53_0: u32 = 448;
        // D s_53_1: read-reg s_53_0:u8
        let s_53_1: u8 = {
            let value = state.read_register::<u8>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // D s_53_2: call ELIsInHost(s_53_1)
        let s_53_2: bool = ELIsInHost(state, tracer, s_53_1);
        // D s_53_3: write-var gs#6136 <= s_53_2
        fn_state.gs_6136 = s_53_2;
        // N s_53_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#6137 <= s_54_0
        fn_state.gs_6137 = s_54_0;
        // N s_54_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: u8 = 0;
        // C s_55_1: const #16981u : u32
        let s_55_1: u32 = 16981;
        // N s_55_2: write-reg s_55_1 <= s_55_0
        let s_55_2: () = {
            state.write_register::<u8>(s_55_1 as isize, s_55_0);
            tracer.write_register(s_55_1 as isize, s_55_0);
        };
        // C s_55_3: const #0u : u8
        let s_55_3: bool = false;
        // C s_55_4: const #16993u : u32
        let s_55_4: u32 = 16993;
        // N s_55_5: write-reg s_55_4 <= s_55_3
        let s_55_5: () = {
            state.write_register::<bool>(s_55_4 as isize, s_55_3);
            tracer.write_register(s_55_4 as isize, s_55_3);
        };
        // N s_55_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // C s_56_1: const #16976u : u32
        let s_56_1: u32 = 16976;
        // N s_56_2: write-reg s_56_1 <= s_56_0
        let s_56_2: () = {
            state.write_register::<bool>(s_56_1 as isize, s_56_0);
            tracer.write_register(s_56_1 as isize, s_56_0);
        };
        // N s_56_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call ResetSVEState(s_57_0)
        let s_57_1: () = ResetSVEState(state, tracer, s_57_0);
        // N s_57_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #16989u : u32
        let s_58_0: u32 = 16989;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: bool = {
            let value = state.read_register::<bool>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: cast zx s_58_1 -> bv
        let s_58_2: Bits = Bits::new(s_58_1 as u128, 1u16);
        // C s_58_3: const #1u : u8
        let s_58_3: bool = true;
        // C s_58_4: cast zx s_58_3 -> bv
        let s_58_4: Bits = Bits::new(s_58_3 as u128, 1u16);
        // D s_58_5: cmp-eq s_58_2 s_58_4
        let s_58_5: bool = ((s_58_2) == (s_58_4));
        // D s_58_6: write-var gs#6117 <= s_58_5
        fn_state.gs_6117 = s_58_5;
        // N s_58_7: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call HaveSME(s_59_0)
        let s_59_1: bool = HaveSME(state, tracer, s_59_0);
        // D s_59_2: write-var gs#6116 <= s_59_1
        fn_state.gs_6116 = s_59_1;
        // N s_59_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call AArch64_MaybeZeroRegisterUppers(s_60_0)
        let s_60_1: () = AArch64_MaybeZeroRegisterUppers(state, tracer, s_60_0);
        // N s_60_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var except.1:struct
        let s_61_0: u32 = fn_state.except._1;
        // D s_61_1: write-var ga#4038 <= s_61_0
        fn_state.ga_4038 = s_61_0;
        // C s_61_2: const #29u : u32
        let s_61_2: u32 = 29;
        // D s_61_3: read-var ga#4038:u32
        let s_61_3: u32 = fn_state.ga_4038;
        // D s_61_4: cmp-eq s_61_2 s_61_3
        let s_61_4: bool = ((s_61_2) == (s_61_3));
        // D s_61_5: not s_61_4
        let s_61_5: bool = !s_61_4;
        // N s_61_6: branch s_61_5 b64 b62
        if s_61_5 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #1u : u32
        let s_62_0: u32 = 1;
        // D s_62_1: write-var cause <= s_62_0
        fn_state.cause = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var cause:u32
        let s_63_0: u32 = fn_state.cause;
        // C s_63_1: const #0u : u8
        let s_63_1: bool = false;
        // D s_63_2: call FailTransaction(s_63_0, s_63_1)
        let s_63_2: () = FailTransaction(state, tracer, s_63_0, s_63_1);
        // N s_63_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #25u : u32
        let s_64_0: u32 = 25;
        // D s_64_1: read-var ga#4038:u32
        let s_64_1: u32 = fn_state.ga_4038;
        // D s_64_2: cmp-eq s_64_0 s_64_1
        let s_64_2: bool = ((s_64_0) == (s_64_1));
        // D s_64_3: not s_64_2
        let s_64_3: bool = !s_64_2;
        // N s_64_4: branch s_64_3 b66 b65
        if s_64_3 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u32
        let s_65_0: u32 = 1;
        // D s_65_1: write-var cause <= s_65_0
        fn_state.cause = s_65_0;
        // N s_65_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #27u : u32
        let s_66_0: u32 = 27;
        // D s_66_1: read-var ga#4038:u32
        let s_66_1: u32 = fn_state.ga_4038;
        // D s_66_2: cmp-eq s_66_0 s_66_1
        let s_66_2: bool = ((s_66_0) == (s_66_1));
        // D s_66_3: not s_66_2
        let s_66_3: bool = !s_66_2;
        // N s_66_4: branch s_66_3 b68 b67
        if s_66_3 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u32
        let s_67_0: u32 = 1;
        // D s_67_1: write-var cause <= s_67_0
        fn_state.cause = s_67_0;
        // N s_67_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #26u : u32
        let s_68_0: u32 = 26;
        // D s_68_1: read-var ga#4038:u32
        let s_68_1: u32 = fn_state.ga_4038;
        // D s_68_2: cmp-eq s_68_0 s_68_1
        let s_68_2: bool = ((s_68_0) == (s_68_1));
        // D s_68_3: not s_68_2
        let s_68_3: bool = !s_68_2;
        // N s_68_4: branch s_68_3 b70 b69
        if s_68_3 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u32
        let s_69_0: u32 = 1;
        // D s_69_1: write-var cause <= s_69_0
        fn_state.cause = s_69_0;
        // N s_69_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #2u : u32
        let s_70_0: u32 = 2;
        // D s_70_1: write-var cause <= s_70_0
        fn_state.cause = s_70_0;
        // N s_70_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #100180u : u32
        let s_71_0: u32 = 100180;
        // D s_71_1: read-reg s_71_0:i
        let s_71_1: i128 = {
            let value = state.read_register::<i128>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // C s_71_2: const #0s : i
        let s_71_2: i128 = 0;
        // D s_71_3: cmp-gt s_71_1 s_71_2
        let s_71_3: bool = ((s_71_1) > (s_71_2));
        // D s_71_4: write-var gs#6100 <= s_71_3
        fn_state.gs_6100 = s_71_3;
        // N s_71_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var target_el:u8
        let s_72_0: u8 = fn_state.target_el;
        // D s_72_1: call SCTLR_read(s_72_0)
        let s_72_1: ProductType5c790c8ef59cc8b2 = SCTLR_read(state, tracer, s_72_0);
        // D s_72_2: call _get_SCTLRType_IESB(s_72_1)
        let s_72_2: bool = u_get_SCTLRType_IESB(state, tracer, s_72_1);
        // D s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // C s_72_4: const #1u : u8
        let s_72_4: bool = true;
        // C s_72_5: cast zx s_72_4 -> bv
        let s_72_5: Bits = Bits::new(s_72_4 as u128, 1u16);
        // D s_72_6: cmp-eq s_72_3 s_72_5
        let s_72_6: bool = ((s_72_3) == (s_72_5));
        // D s_72_7: write-var sync_errors <= s_72_6
        fn_state.sync_errors = s_72_6;
        // C s_72_8: const #() : ()
        let s_72_8: () = ();
        // S s_72_9: call HaveDoubleFaultExt(s_72_8)
        let s_72_9: bool = HaveDoubleFaultExt(state, tracer, s_72_8);
        // N s_72_10: branch s_72_9 b78 b73
        if s_72_9 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_73_0: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #59u : u32
        let s_74_0: u32 = 59;
        // S s_74_1: call ConstrainUnpredictableBool(s_74_0)
        let s_74_1: bool = ConstrainUnpredictableBool(state, tracer, s_74_0);
        // S s_74_2: not s_74_1
        let s_74_2: bool = !s_74_1;
        // N s_74_3: branch s_74_2 b77 b75
        if s_74_2 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_75_0: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_76_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var sync_errors <= s_77_0
        fn_state.sync_errors = s_77_0;
        // N s_77_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var sync_errors:u8
        let s_78_0: bool = fn_state.sync_errors;
        // N s_78_1: branch s_78_0 b84 b79
        if s_78_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #90704u : u32
        let s_79_0: u32 = 90704;
        // D s_79_1: read-reg s_79_0:struct
        let s_79_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // D s_79_2: call _get_SCR_EL3_Type_EA(s_79_1)
        let s_79_2: bool = u_get_SCR_EL3_Type_EA(state, tracer, s_79_1);
        // C s_79_3: const #90704u : u32
        let s_79_3: u32 = 90704;
        // D s_79_4: read-reg s_79_3:struct
        let s_79_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_3 as isize);
            tracer.read_register(s_79_3 as isize, value);
            value
        };
        // D s_79_5: call _get_SCR_EL3_Type_NMEA(s_79_4)
        let s_79_5: bool = u_get_SCR_EL3_Type_NMEA(state, tracer, s_79_4);
        // D s_79_6: cast zx s_79_2 -> bv
        let s_79_6: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_7: cast zx s_79_5 -> bv
        let s_79_7: Bits = Bits::new(s_79_5 as u128, 1u16);
        // D s_79_8: cast reint s_79_6 -> u128
        let s_79_8: u128 = (s_79_6.value() as u128);
        // D s_79_9: size-of s_79_6
        let s_79_9: u16 = s_79_6.length();
        // D s_79_10: cast reint s_79_7 -> u128
        let s_79_10: u128 = (s_79_7.value() as u128);
        // D s_79_11: size-of s_79_7
        let s_79_11: u16 = s_79_7.length();
        // D s_79_12: lsl s_79_8 s_79_11
        let s_79_12: u128 = s_79_8 << s_79_11;
        // D s_79_13: or s_79_12 s_79_10
        let s_79_13: u128 = ((s_79_12) | (s_79_10));
        // D s_79_14: add s_79_9 s_79_11
        let s_79_14: u16 = (s_79_9 + s_79_11);
        // D s_79_15: create-bits s_79_13 s_79_14
        let s_79_15: Bits = Bits::new(s_79_13, s_79_14);
        // D s_79_16: cast reint s_79_15 -> u8
        let s_79_16: u8 = (s_79_15.value() as u8);
        // D s_79_17: cast zx s_79_16 -> bv
        let s_79_17: Bits = Bits::new(s_79_16 as u128, 2u16);
        // C s_79_18: const #3u : u8
        let s_79_18: u8 = 3;
        // C s_79_19: cast zx s_79_18 -> bv
        let s_79_19: Bits = Bits::new(s_79_18 as u128, 2u16);
        // D s_79_20: cmp-eq s_79_17 s_79_19
        let s_79_20: bool = ((s_79_17) == (s_79_19));
        // N s_79_21: branch s_79_20 b83 b80
        if s_79_20 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var gs#6113 <= s_80_0
        fn_state.gs_6113 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#6113:u8
        let s_81_0: bool = fn_state.gs_6113;
        // D s_81_1: write-var gs#6114 <= s_81_0
        fn_state.gs_6114 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#6114:u8
        let s_82_0: bool = fn_state.gs_6114;
        // D s_82_1: write-var sync_errors <= s_82_0
        fn_state.sync_errors = s_82_0;
        // N s_82_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var target_el:u8
        let s_83_0: u8 = fn_state.target_el;
        // D s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 2u16);
        // C s_83_2: const #424u : u32
        let s_83_2: u32 = 424;
        // D s_83_3: read-reg s_83_2:u8
        let s_83_3: u8 = {
            let value = state.read_register::<u8>(s_83_2 as isize);
            tracer.read_register(s_83_2 as isize, value);
            value
        };
        // D s_83_4: cast zx s_83_3 -> bv
        let s_83_4: Bits = Bits::new(s_83_3 as u128, 2u16);
        // D s_83_5: cmp-eq s_83_1 s_83_4
        let s_83_5: bool = ((s_83_1) == (s_83_4));
        // D s_83_6: write-var gs#6113 <= s_83_5
        fn_state.gs_6113 = s_83_5;
        // N s_83_7: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #1u : u8
        let s_84_0: bool = true;
        // D s_84_1: write-var gs#6114 <= s_84_0
        fn_state.gs_6114 = s_84_0;
        // N s_84_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #1u : u8
        let s_85_0: bool = true;
        // D s_85_1: write-var gs#6096 <= s_85_0
        fn_state.gs_6096 = s_85_0;
        // N s_85_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var target_el:u8
        let s_86_0: u8 = fn_state.target_el;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 2u16);
        // D s_86_2: cast zx s_86_1 -> i
        let s_86_2: i128 = (s_86_1.value() as i128);
        // D s_86_3: cast reint s_86_2 -> i64
        let s_86_3: i64 = (s_86_2 as i64);
        // C s_86_4: const #16975u : u32
        let s_86_4: u32 = 16975;
        // D s_86_5: read-reg s_86_4:u8
        let s_86_5: u8 = {
            let value = state.read_register::<u8>(s_86_4 as isize);
            tracer.read_register(s_86_4 as isize, value);
            value
        };
        // D s_86_6: cast zx s_86_5 -> bv
        let s_86_6: Bits = Bits::new(s_86_5 as u128, 2u16);
        // D s_86_7: cast zx s_86_6 -> i
        let s_86_7: i128 = (s_86_6.value() as i128);
        // D s_86_8: cast reint s_86_7 -> i64
        let s_86_8: i64 = (s_86_7 as i64);
        // D s_86_9: cast zx s_86_3 -> i
        let s_86_9: i128 = (i128::try_from(s_86_3).unwrap());
        // D s_86_10: cast zx s_86_8 -> i
        let s_86_10: i128 = (i128::try_from(s_86_8).unwrap());
        // D s_86_11: cmp-ge s_86_9 s_86_10
        let s_86_11: bool = ((s_86_9) >= (s_86_10));
        // D s_86_12: write-var gs#6095 <= s_86_11
        fn_state.gs_6095 = s_86_11;
        // N s_86_13: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var target_el:u8
        let s_87_0: u8 = fn_state.target_el;
        // D s_87_1: call ELUsingAArch32(s_87_0)
        let s_87_1: bool = ELUsingAArch32(state, tracer, s_87_0);
        // D s_87_2: not s_87_1
        let s_87_2: bool = !s_87_1;
        // D s_87_3: write-var gs#6094 <= s_87_2
        fn_state.gs_6094 = s_87_2;
        // N s_87_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
