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
use u_get_HSCTLR_Type_EE::*;
use Halted::*;
use Mk_DSPSR_Type::*;
use Mk_SPSR_EL1_Type::*;
use SPSR_svc_write::*;
use u_get_SCTLR_EL1_Type_IESB::*;
use HSR_write::*;
use Mk_SPSR_svc_Type::*;
use UpdateEDSCRFields::*;
use u_get_SCTLR_Type_SPAN::*;
use Mk_DSPSR_EL0_Type::*;
use u__UNKNOWN_bits::*;
use DSPSR_write::*;
use SCTLR_read__2::*;
use Mk_ESR_EL1_Type::*;
use AArch32_WriteMode::*;
use u_get_SCTLR_EL1_Type_SPAN::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use HSCTLR_read::*;
use AArch64_MaybeZeroRegisterUppers::*;
use MaybeZeroSVEUppers::*;
use DLR_write::*;
use ConstrainUnpredictableBool::*;
use HCR_read::*;
use ELR_hyp_write::*;
use SPSR_hyp_write::*;
use HavePANExt::*;
use u_get_HCR_Type_TGE::*;
use Mk_SPSR_hyp_Type::*;
use HaveUAOExt::*;
use ELUsingAArch32::*;
use HaveIESB::*;
use SynchronizeErrors::*;
use u_get_SCTLR_Type_EE::*;
use Mk_HSR_Type::*;
use common::*;
pub fn execute_aarch32_instrs_DCPS1_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_323926: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_323930: bool,
        gs_323928: bool,
        gs_323936: bool,
        gs_323937: bool,
        gs_323960: bool,
        gs_323927: bool,
        tgeshadow_7909: bool,
        gs_323926: (),
    }
    let fn_state = FunctionState {
        gs_323926,
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
        // S s_0_1: call Halted(s_0_0)
        let s_0_1: bool = Halted(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b49 b1
        if s_0_2 {
            return block_49(state, tracer, fn_state);
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
        // S s_1_1: call EL2Enabled(s_1_0)
        let s_1_1: bool = EL2Enabled(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b48 b2
        if s_1_1 {
            return block_48(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#323927 <= s_2_0
        fn_state.gs_323927 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#323927:u8
        let s_3_0: bool = fn_state.gs_323927;
        // N s_3_1: branch s_3_0 b42 b4
        if s_3_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
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
        // C s_5_3: const #448u : u32
        let s_5_3: u32 = 448;
        // D s_5_4: read-reg s_5_3:u8
        let s_5_4: u8 = {
            let value = state.read_register::<u8>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cmp-ne s_5_2 s_5_5
        let s_5_6: bool = ((s_5_2) != (s_5_5));
        // N s_5_7: branch s_5_6 b41 b6
        if s_5_6 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #440u : u32
        let s_6_0: u32 = 440;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call ELUsingAArch32(s_6_1)
        let s_6_2: bool = ELUsingAArch32(state, tracer, s_6_1);
        // D s_6_3: write-var gs#323928 <= s_6_2
        fn_state.gs_323928 = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#323928:u8
        let s_7_0: bool = fn_state.gs_323928;
        // N s_7_1: branch s_7_0 b28 b8
        if s_7_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call AArch64_MaybeZeroRegisterUppers(s_8_0)
        let s_8_1: () = AArch64_MaybeZeroRegisterUppers(state, tracer, s_8_0);
        // C s_8_2: const #440u : u32
        let s_8_2: u32 = 440;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: call MaybeZeroSVEUppers(s_8_3)
        let s_8_4: () = MaybeZeroSVEUppers(state, tracer, s_8_3);
        // C s_8_5: const #0u : u8
        let s_8_5: bool = false;
        // C s_8_6: const #16999u : u32
        let s_8_6: u32 = 16999;
        // N s_8_7: write-reg s_8_6 <= s_8_5
        let s_8_7: () = {
            state.write_register::<bool>(s_8_6 as isize, s_8_5);
            tracer.write_register(s_8_6 as isize, s_8_5);
        };
        // C s_8_8: const #1u : u8
        let s_8_8: bool = true;
        // C s_8_9: const #16990u : u32
        let s_8_9: u32 = 16990;
        // N s_8_10: write-reg s_8_9 <= s_8_8
        let s_8_10: () = {
            state.write_register::<bool>(s_8_9 as isize, s_8_8);
            tracer.write_register(s_8_9 as isize, s_8_8);
        };
        // C s_8_11: const #440u : u32
        let s_8_11: u32 = 440;
        // D s_8_12: read-reg s_8_11:u8
        let s_8_12: u8 = {
            let value = state.read_register::<u8>(s_8_11 as isize);
            tracer.read_register(s_8_11 as isize, value);
            value
        };
        // C s_8_13: const #16975u : u32
        let s_8_13: u32 = 16975;
        // N s_8_14: write-reg s_8_13 <= s_8_12
        let s_8_14: () = {
            state.write_register::<u8>(s_8_13 as isize, s_8_12);
            tracer.write_register(s_8_13 as isize, s_8_12);
        };
        // C s_8_15: const #() : ()
        let s_8_15: () = ();
        // S s_8_16: call HavePANExt(s_8_15)
        let s_8_16: bool = HavePANExt(state, tracer, s_8_15);
        // N s_8_17: branch s_8_16 b27 b9
        if s_8_16 {
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
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#323930 <= s_9_0
        fn_state.gs_323930 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#323930:u8
        let s_10_0: bool = fn_state.gs_323930;
        // N s_10_1: branch s_10_0 b26 b11
        if s_10_0 {
            return block_26(state, tracer, fn_state);
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
        // S s_12_1: call HaveUAOExt(s_12_0)
        let s_12_1: bool = HaveUAOExt(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b25 b13
        if s_12_1 {
            return block_25(state, tracer, fn_state);
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
        // C s_14_0: const #64s : i64
        let s_14_0: i64 = 64;
        // C s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // S s_14_2: call __UNKNOWN_bits(s_14_1)
        let s_14_2: Bits = u__UNKNOWN_bits(state, tracer, s_14_1);
        // S s_14_3: cast reint s_14_2 -> u64
        let s_14_3: u64 = (s_14_2.value() as u64);
        // C s_14_4: const #18312u : u32
        let s_14_4: u32 = 18312;
        // N s_14_5: write-reg s_14_4 <= s_14_3
        let s_14_5: () = {
            state.write_register::<u64>(s_14_4 as isize, s_14_3);
            tracer.write_register(s_14_4 as isize, s_14_3);
        };
        // C s_14_6: const #64s : i64
        let s_14_6: i64 = 64;
        // C s_14_7: cast zx s_14_6 -> i
        let s_14_7: i128 = (i128::try_from(s_14_6).unwrap());
        // S s_14_8: call __UNKNOWN_bits(s_14_7)
        let s_14_8: Bits = u__UNKNOWN_bits(state, tracer, s_14_7);
        // S s_14_9: cast reint s_14_8 -> u64
        let s_14_9: u64 = (s_14_8.value() as u64);
        // S s_14_10: call Mk_ESR_EL1_Type(s_14_9)
        let s_14_10: ProductType5c790c8ef59cc8b2 = Mk_ESR_EL1_Type(
            state,
            tracer,
            s_14_9,
        );
        // C s_14_11: const #22952u : u32
        let s_14_11: u32 = 22952;
        // N s_14_12: write-reg s_14_11 <= s_14_10
        let s_14_12: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_11 as isize, s_14_10);
            tracer.write_register(s_14_11 as isize, s_14_10);
        };
        // C s_14_13: const #64s : i64
        let s_14_13: i64 = 64;
        // C s_14_14: cast zx s_14_13 -> i
        let s_14_14: i128 = (i128::try_from(s_14_13).unwrap());
        // S s_14_15: call __UNKNOWN_bits(s_14_14)
        let s_14_15: Bits = u__UNKNOWN_bits(state, tracer, s_14_14);
        // S s_14_16: cast reint s_14_15 -> u64
        let s_14_16: u64 = (s_14_15.value() as u64);
        // S s_14_17: call Mk_SPSR_EL1_Type(s_14_16)
        let s_14_17: ProductType5c790c8ef59cc8b2 = Mk_SPSR_EL1_Type(
            state,
            tracer,
            s_14_16,
        );
        // C s_14_18: const #90648u : u32
        let s_14_18: u32 = 90648;
        // N s_14_19: write-reg s_14_18 <= s_14_17
        let s_14_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_18 as isize, s_14_17);
            tracer.write_register(s_14_18 as isize, s_14_17);
        };
        // C s_14_20: const #64s : i64
        let s_14_20: i64 = 64;
        // C s_14_21: cast zx s_14_20 -> i
        let s_14_21: i128 = (i128::try_from(s_14_20).unwrap());
        // S s_14_22: call __UNKNOWN_bits(s_14_21)
        let s_14_22: Bits = u__UNKNOWN_bits(state, tracer, s_14_21);
        // S s_14_23: cast reint s_14_22 -> u64
        let s_14_23: u64 = (s_14_22.value() as u64);
        // C s_14_24: const #13360u : u32
        let s_14_24: u32 = 13360;
        // N s_14_25: write-reg s_14_24 <= s_14_23
        let s_14_25: () = {
            state.write_register::<u64>(s_14_24 as isize, s_14_23);
            tracer.write_register(s_14_24 as isize, s_14_23);
        };
        // C s_14_26: const #64s : i64
        let s_14_26: i64 = 64;
        // C s_14_27: cast zx s_14_26 -> i
        let s_14_27: i128 = (i128::try_from(s_14_26).unwrap());
        // S s_14_28: call __UNKNOWN_bits(s_14_27)
        let s_14_28: Bits = u__UNKNOWN_bits(state, tracer, s_14_27);
        // S s_14_29: cast reint s_14_28 -> u64
        let s_14_29: u64 = (s_14_28.value() as u64);
        // S s_14_30: call Mk_DSPSR_EL0_Type(s_14_29)
        let s_14_30: ProductType5c790c8ef59cc8b2 = Mk_DSPSR_EL0_Type(
            state,
            tracer,
            s_14_29,
        );
        // C s_14_31: const #102584u : u32
        let s_14_31: u32 = 102584;
        // N s_14_32: write-reg s_14_31 <= s_14_30
        let s_14_32: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_31 as isize, s_14_30);
            tracer.write_register(s_14_31 as isize, s_14_30);
        };
        // C s_14_33: const #() : ()
        let s_14_33: () = ();
        // S s_14_34: call HaveIESB(s_14_33)
        let s_14_34: bool = HaveIESB(state, tracer, s_14_33);
        // N s_14_35: branch s_14_34 b24 b15
        if s_14_34 {
            return block_24(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#323936 <= s_15_0
        fn_state.gs_323936 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#323936:u8
        let s_16_0: bool = fn_state.gs_323936;
        // N s_16_1: branch s_16_0 b23 b17
        if s_16_0 {
            return block_23(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#323937 <= s_17_0
        fn_state.gs_323937 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#323937:u8
        let s_18_0: bool = fn_state.gs_323937;
        // N s_18_1: branch s_18_0 b22 b19
        if s_18_0 {
            return block_22(state, tracer, fn_state);
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
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call UpdateEDSCRFields(s_21_0)
        let s_21_1: () = UpdateEDSCRFields(state, tracer, s_21_0);
        // N s_21_2: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call SynchronizeErrors(s_22_0)
        let s_22_1: () = SynchronizeErrors(state, tracer, s_22_0);
        // N s_22_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #59u : u32
        let s_23_0: u32 = 59;
        // S s_23_1: call ConstrainUnpredictableBool(s_23_0)
        let s_23_1: bool = ConstrainUnpredictableBool(state, tracer, s_23_0);
        // S s_23_2: not s_23_1
        let s_23_2: bool = !s_23_1;
        // D s_23_3: write-var gs#323937 <= s_23_2
        fn_state.gs_323937 = s_23_2;
        // N s_23_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #90272u : u32
        let s_24_0: u32 = 90272;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_SCTLR_EL1_Type_IESB(s_24_1)
        let s_24_2: bool = u_get_SCTLR_EL1_Type_IESB(state, tracer, s_24_1);
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // D s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // D s_24_7: write-var gs#323936 <= s_24_6
        fn_state.gs_323936 = s_24_6;
        // N s_24_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // C s_25_1: const #16995u : u32
        let s_25_1: u32 = 16995;
        // N s_25_2: write-reg s_25_1 <= s_25_0
        let s_25_2: () = {
            state.write_register::<bool>(s_25_1 as isize, s_25_0);
            tracer.write_register(s_25_1 as isize, s_25_0);
        };
        // N s_25_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // C s_26_1: const #16985u : u32
        let s_26_1: u32 = 16985;
        // N s_26_2: write-reg s_26_1 <= s_26_0
        let s_26_2: () = {
            state.write_register::<bool>(s_26_1 as isize, s_26_0);
            tracer.write_register(s_26_1 as isize, s_26_0);
        };
        // N s_26_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #90272u : u32
        let s_27_0: u32 = 90272;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_SCTLR_EL1_Type_SPAN(s_27_1)
        let s_27_2: bool = u_get_SCTLR_EL1_Type_SPAN(state, tracer, s_27_1);
        // D s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // C s_27_4: const #0u : u8
        let s_27_4: bool = false;
        // C s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 1u16);
        // D s_27_6: cmp-eq s_27_3 s_27_5
        let s_27_6: bool = ((s_27_3) == (s_27_5));
        // D s_27_7: write-var gs#323930 <= s_27_6
        fn_state.gs_323930 = s_27_6;
        // N s_27_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #16983u : u32
        let s_28_0: u32 = 16983;
        // D s_28_1: read-reg s_28_0:u8
        let s_28_1: u8 = {
            let value = state.read_register::<u8>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: cast zx s_28_1 -> bv
        let s_28_2: Bits = Bits::new(s_28_1 as u128, 5u16);
        // C s_28_3: const #384u : u32
        let s_28_3: u32 = 384;
        // D s_28_4: read-reg s_28_3:u8
        let s_28_4: u8 = {
            let value = state.read_register::<u8>(s_28_3 as isize);
            tracer.read_register(s_28_3 as isize, value);
            value
        };
        // D s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 5u16);
        // D s_28_6: cmp-eq s_28_2 s_28_5
        let s_28_6: bool = ((s_28_2) == (s_28_5));
        // N s_28_7: branch s_28_6 b40 b29
        if s_28_6 {
            return block_40(state, tracer, fn_state);
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
        // C s_30_0: const #16975u : u32
        let s_30_0: u32 = 16975;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: cast zx s_30_1 -> bv
        let s_30_2: Bits = Bits::new(s_30_1 as u128, 2u16);
        // C s_30_3: const #432u : u32
        let s_30_3: u32 = 432;
        // D s_30_4: read-reg s_30_3:u8
        let s_30_4: u8 = {
            let value = state.read_register::<u8>(s_30_3 as isize);
            tracer.read_register(s_30_3 as isize, value);
            value
        };
        // D s_30_5: cast zx s_30_4 -> bv
        let s_30_5: Bits = Bits::new(s_30_4 as u128, 2u16);
        // D s_30_6: cmp-ne s_30_2 s_30_5
        let s_30_6: bool = ((s_30_2) != (s_30_5));
        // N s_30_7: branch s_30_6 b33 b31
        if s_30_6 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call HSCTLR_read(s_31_0)
        let s_31_1: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_31_0);
        // S s_31_2: call _get_HSCTLR_Type_EE(s_31_1)
        let s_31_2: bool = u_get_HSCTLR_Type_EE(state, tracer, s_31_1);
        // C s_31_3: const #16974u : u32
        let s_31_3: u32 = 16974;
        // N s_31_4: write-reg s_31_3 <= s_31_2
        let s_31_4: () = {
            state.write_register::<bool>(s_31_3 as isize, s_31_2);
            tracer.write_register(s_31_3 as isize, s_31_2);
        };
        // C s_31_5: const #32s : i64
        let s_31_5: i64 = 32;
        // C s_31_6: cast zx s_31_5 -> i
        let s_31_6: i128 = (i128::try_from(s_31_5).unwrap());
        // S s_31_7: call __UNKNOWN_bits(s_31_6)
        let s_31_7: Bits = u__UNKNOWN_bits(state, tracer, s_31_6);
        // S s_31_8: cast reint s_31_7 -> u32
        let s_31_8: u32 = (s_31_7.value() as u32);
        // S s_31_9: call ELR_hyp_write(s_31_8)
        let s_31_9: () = ELR_hyp_write(state, tracer, s_31_8);
        // C s_31_10: const #32s : i64
        let s_31_10: i64 = 32;
        // C s_31_11: cast zx s_31_10 -> i
        let s_31_11: i128 = (i128::try_from(s_31_10).unwrap());
        // S s_31_12: call __UNKNOWN_bits(s_31_11)
        let s_31_12: Bits = u__UNKNOWN_bits(state, tracer, s_31_11);
        // S s_31_13: cast reint s_31_12 -> u32
        let s_31_13: u32 = (s_31_12.value() as u32);
        // S s_31_14: call Mk_HSR_Type(s_31_13)
        let s_31_14: ProductType700c18a878c5601b = Mk_HSR_Type(state, tracer, s_31_13);
        // S s_31_15: call HSR_write(s_31_14)
        let s_31_15: () = HSR_write(state, tracer, s_31_14);
        // C s_31_16: const #32s : i64
        let s_31_16: i64 = 32;
        // C s_31_17: cast zx s_31_16 -> i
        let s_31_17: i128 = (i128::try_from(s_31_16).unwrap());
        // S s_31_18: call __UNKNOWN_bits(s_31_17)
        let s_31_18: Bits = u__UNKNOWN_bits(state, tracer, s_31_17);
        // S s_31_19: cast reint s_31_18 -> u32
        let s_31_19: u32 = (s_31_18.value() as u32);
        // S s_31_20: call Mk_SPSR_hyp_Type(s_31_19)
        let s_31_20: ProductType700c18a878c5601b = Mk_SPSR_hyp_Type(
            state,
            tracer,
            s_31_19,
        );
        // S s_31_21: call SPSR_hyp_write(s_31_20)
        let s_31_21: () = SPSR_hyp_write(state, tracer, s_31_20);
        // N s_31_22: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #32s : i64
        let s_32_0: i64 = 32;
        // C s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // S s_32_2: call __UNKNOWN_bits(s_32_1)
        let s_32_2: Bits = u__UNKNOWN_bits(state, tracer, s_32_1);
        // S s_32_3: cast reint s_32_2 -> u32
        let s_32_3: u32 = (s_32_2.value() as u32);
        // S s_32_4: call DLR_write(s_32_3)
        let s_32_4: () = DLR_write(state, tracer, s_32_3);
        // C s_32_5: const #32s : i64
        let s_32_5: i64 = 32;
        // C s_32_6: cast zx s_32_5 -> i
        let s_32_6: i128 = (i128::try_from(s_32_5).unwrap());
        // S s_32_7: call __UNKNOWN_bits(s_32_6)
        let s_32_7: Bits = u__UNKNOWN_bits(state, tracer, s_32_6);
        // S s_32_8: cast reint s_32_7 -> u32
        let s_32_8: u32 = (s_32_7.value() as u32);
        // S s_32_9: call Mk_DSPSR_Type(s_32_8)
        let s_32_9: ProductType700c18a878c5601b = Mk_DSPSR_Type(state, tracer, s_32_8);
        // S s_32_10: call DSPSR_write(s_32_9)
        let s_32_10: () = DSPSR_write(state, tracer, s_32_9);
        // N s_32_11: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #376u : u32
        let s_33_0: u32 = 376;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call AArch32_WriteMode(s_33_1)
        let s_33_2: () = AArch32_WriteMode(state, tracer, s_33_1);
        // C s_33_3: const #() : ()
        let s_33_3: () = ();
        // S s_33_4: call SCTLR_read__2(s_33_3)
        let s_33_4: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_33_3);
        // S s_33_5: call _get_SCTLR_Type_EE(s_33_4)
        let s_33_5: bool = u_get_SCTLR_Type_EE(state, tracer, s_33_4);
        // C s_33_6: const #16974u : u32
        let s_33_6: u32 = 16974;
        // N s_33_7: write-reg s_33_6 <= s_33_5
        let s_33_7: () = {
            state.write_register::<bool>(s_33_6 as isize, s_33_5);
            tracer.write_register(s_33_6 as isize, s_33_5);
        };
        // C s_33_8: const #() : ()
        let s_33_8: () = ();
        // S s_33_9: call HavePANExt(s_33_8)
        let s_33_9: bool = HavePANExt(state, tracer, s_33_8);
        // N s_33_10: branch s_33_9 b39 b34
        if s_33_9 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#323960 <= s_34_0
        fn_state.gs_323960 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#323960:u8
        let s_35_0: bool = fn_state.gs_323960;
        // N s_35_1: branch s_35_0 b38 b36
        if s_35_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #32s : i64
        let s_37_0: i64 = 32;
        // C s_37_1: cast zx s_37_0 -> i
        let s_37_1: i128 = (i128::try_from(s_37_0).unwrap());
        // S s_37_2: call __UNKNOWN_bits(s_37_1)
        let s_37_2: Bits = u__UNKNOWN_bits(state, tracer, s_37_1);
        // C s_37_3: const #32s : i64
        let s_37_3: i64 = 32;
        // C s_37_4: cast zx s_37_3 -> i
        let s_37_4: i128 = (i128::try_from(s_37_3).unwrap());
        // S s_37_5: call __UNKNOWN_bits(s_37_4)
        let s_37_5: Bits = u__UNKNOWN_bits(state, tracer, s_37_4);
        // S s_37_6: cast reint s_37_5 -> u32
        let s_37_6: u32 = (s_37_5.value() as u32);
        // S s_37_7: call Mk_SPSR_svc_Type(s_37_6)
        let s_37_7: ProductType700c18a878c5601b = Mk_SPSR_svc_Type(
            state,
            tracer,
            s_37_6,
        );
        // S s_37_8: call SPSR_svc_write(s_37_7)
        let s_37_8: () = SPSR_svc_write(state, tracer, s_37_7);
        // N s_37_9: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // C s_38_1: const #16985u : u32
        let s_38_1: u32 = 16985;
        // N s_38_2: write-reg s_38_1 <= s_38_0
        let s_38_2: () = {
            state.write_register::<bool>(s_38_1 as isize, s_38_0);
            tracer.write_register(s_38_1 as isize, s_38_0);
        };
        // N s_38_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call SCTLR_read__2(s_39_0)
        let s_39_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_39_0);
        // S s_39_2: call _get_SCTLR_Type_SPAN(s_39_1)
        let s_39_2: bool = u_get_SCTLR_Type_SPAN(state, tracer, s_39_1);
        // S s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #0u : u8
        let s_39_4: bool = false;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // S s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // D s_39_7: write-var gs#323960 <= s_39_6
        fn_state.gs_323960 = s_39_6;
        // N s_39_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #20920u : u32
        let s_40_0: u32 = 20920;
        // D s_40_1: read-reg s_40_0:struct
        let s_40_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // C s_40_2: const #20920u : u32
        let s_40_2: u32 = 20920;
        // N s_40_3: write-reg s_40_2 <= s_40_1
        let s_40_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_40_2 as isize, s_40_1);
            tracer.write_register(s_40_2 as isize, s_40_1);
        };
        // N s_40_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#323928 <= s_41_0
        fn_state.gs_323928 = s_41_0;
        // N s_41_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #432u : u32
        let s_42_0: u32 = 432;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: u8 = {
            let value = state.read_register::<u8>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call ELUsingAArch32(s_42_1)
        let s_42_2: bool = ELUsingAArch32(state, tracer, s_42_1);
        // N s_42_3: branch s_42_2 b47 b43
        if s_42_2 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #102552u : u32
        let s_43_0: u32 = 102552;
        // D s_43_1: read-reg s_43_0:struct
        let s_43_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call _get_HCR_EL2_Type_TGE(s_43_1)
        let s_43_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_43_1);
        // D s_43_3: write-var tgeshadow#7909 <= s_43_2
        fn_state.tgeshadow_7909 = s_43_2;
        // N s_43_4: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var tgeshadow#7909:u8
        let s_44_0: bool = fn_state.tgeshadow_7909;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #1u : u8
        let s_44_2: bool = true;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // N s_44_5: branch s_44_4 b46 b45
        if s_44_4 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: panic
        panic!("{:?}", ());
        // N s_46_1: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call HCR_read(s_47_0)
        let s_47_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_47_0);
        // S s_47_2: call _get_HCR_Type_TGE(s_47_1)
        let s_47_2: bool = u_get_HCR_Type_TGE(state, tracer, s_47_1);
        // D s_47_3: write-var tgeshadow#7909 <= s_47_2
        fn_state.tgeshadow_7909 = s_47_2;
        // N s_47_4: jump b44
        return block_44(state, tracer, fn_state);
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
        // C s_48_3: const #448u : u32
        let s_48_3: u32 = 448;
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
        // D s_48_7: write-var gs#323927 <= s_48_6
        fn_state.gs_323927 = s_48_6;
        // N s_48_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_49_0: panic
        panic!("{:?}", ());
        // N s_49_1: return
        return;
    }
}
