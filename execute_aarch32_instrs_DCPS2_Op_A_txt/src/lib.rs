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
use u_get_SCTLR_EL2_Type_SPAN::*;
use u_get_SCTLR_EL2_Type_IESB::*;
use HSR_write::*;
use UpdateEDSCRFields::*;
use Mk_DSPSR_EL0_Type::*;
use u__UNKNOWN_bits::*;
use DSPSR_write::*;
use AArch32_WriteMode::*;
use Mk_SPSR_EL2_Type::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use HSCTLR_read::*;
use AArch64_MaybeZeroRegisterUppers::*;
use MaybeZeroSVEUppers::*;
use Mk_ESR_EL2_Type::*;
use u_get_HCR_EL2_Type_E2H::*;
use DLR_write::*;
use ConstrainUnpredictableBool::*;
use ELR_hyp_write::*;
use SPSR_hyp_write::*;
use HavePANExt::*;
use Mk_SPSR_hyp_Type::*;
use HaveUAOExt::*;
use ELUsingAArch32::*;
use HaveIESB::*;
use SynchronizeErrors::*;
use Mk_HSR_Type::*;
use common::*;
pub fn execute_aarch32_instrs_DCPS2_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_323976: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_323987: bool,
        gs_323979: bool,
        gs_323980: bool,
        gs_323977: bool,
        gs_323988: bool,
        gs_323981: bool,
        gs_323976: (),
    }
    let fn_state = FunctionState {
        gs_323976,
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
        // N s_0_3: branch s_0_2 b32 b1
        if s_0_2 {
            return block_32(state, tracer, fn_state);
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
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // D s_1_3: write-var gs#323977 <= s_1_2
        fn_state.gs_323977 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#323977:u8
        let s_2_0: bool = fn_state.gs_323977;
        // N s_2_1: branch s_2_0 b31 b3
        if s_2_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #432u : u32
        let s_3_0: u32 = 432;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call ELUsingAArch32(s_3_1)
        let s_3_2: bool = ELUsingAArch32(state, tracer, s_3_1);
        // N s_3_3: branch s_3_2 b30 b4
        if s_3_2 {
            return block_30(state, tracer, fn_state);
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
        // S s_4_1: call AArch64_MaybeZeroRegisterUppers(s_4_0)
        let s_4_1: () = AArch64_MaybeZeroRegisterUppers(state, tracer, s_4_0);
        // C s_4_2: const #432u : u32
        let s_4_2: u32 = 432;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: call MaybeZeroSVEUppers(s_4_3)
        let s_4_4: () = MaybeZeroSVEUppers(state, tracer, s_4_3);
        // C s_4_5: const #0u : u8
        let s_4_5: bool = false;
        // C s_4_6: const #16999u : u32
        let s_4_6: u32 = 16999;
        // N s_4_7: write-reg s_4_6 <= s_4_5
        let s_4_7: () = {
            state.write_register::<bool>(s_4_6 as isize, s_4_5);
            tracer.write_register(s_4_6 as isize, s_4_5);
        };
        // C s_4_8: const #1u : u8
        let s_4_8: bool = true;
        // C s_4_9: const #16990u : u32
        let s_4_9: u32 = 16990;
        // N s_4_10: write-reg s_4_9 <= s_4_8
        let s_4_10: () = {
            state.write_register::<bool>(s_4_9 as isize, s_4_8);
            tracer.write_register(s_4_9 as isize, s_4_8);
        };
        // C s_4_11: const #432u : u32
        let s_4_11: u32 = 432;
        // D s_4_12: read-reg s_4_11:u8
        let s_4_12: u8 = {
            let value = state.read_register::<u8>(s_4_11 as isize);
            tracer.read_register(s_4_11 as isize, value);
            value
        };
        // C s_4_13: const #16975u : u32
        let s_4_13: u32 = 16975;
        // N s_4_14: write-reg s_4_13 <= s_4_12
        let s_4_14: () = {
            state.write_register::<u8>(s_4_13 as isize, s_4_12);
            tracer.write_register(s_4_13 as isize, s_4_12);
        };
        // C s_4_15: const #() : ()
        let s_4_15: () = ();
        // S s_4_16: call HavePANExt(s_4_15)
        let s_4_16: bool = HavePANExt(state, tracer, s_4_15);
        // N s_4_17: branch s_4_16 b29 b5
        if s_4_16 {
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
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#323979 <= s_5_0
        fn_state.gs_323979 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#323979:u8
        let s_6_0: bool = fn_state.gs_323979;
        // N s_6_1: branch s_6_0 b28 b7
        if s_6_0 {
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
        // D s_7_1: write-var gs#323980 <= s_7_0
        fn_state.gs_323980 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#323980:u8
        let s_8_0: bool = fn_state.gs_323980;
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
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#323981 <= s_9_0
        fn_state.gs_323981 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#323981:u8
        let s_10_0: bool = fn_state.gs_323981;
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
        // C s_14_4: const #17224u : u32
        let s_14_4: u32 = 17224;
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
        // S s_14_10: call Mk_ESR_EL2_Type(s_14_9)
        let s_14_10: ProductType5c790c8ef59cc8b2 = Mk_ESR_EL2_Type(
            state,
            tracer,
            s_14_9,
        );
        // C s_14_11: const #90688u : u32
        let s_14_11: u32 = 90688;
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
        // S s_14_17: call Mk_SPSR_EL2_Type(s_14_16)
        let s_14_17: ProductType5c790c8ef59cc8b2 = Mk_SPSR_EL2_Type(
            state,
            tracer,
            s_14_16,
        );
        // C s_14_18: const #15736u : u32
        let s_14_18: u32 = 15736;
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
        // D s_15_1: write-var gs#323987 <= s_15_0
        fn_state.gs_323987 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#323987:u8
        let s_16_0: bool = fn_state.gs_323987;
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
        // D s_17_1: write-var gs#323988 <= s_17_0
        fn_state.gs_323988 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#323988:u8
        let s_18_0: bool = fn_state.gs_323988;
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
        // D s_23_3: write-var gs#323988 <= s_23_2
        fn_state.gs_323988 = s_23_2;
        // N s_23_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #20784u : u32
        let s_24_0: u32 = 20784;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_SCTLR_EL2_Type_IESB(s_24_1)
        let s_24_2: bool = u_get_SCTLR_EL2_Type_IESB(state, tracer, s_24_1);
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // D s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // D s_24_7: write-var gs#323987 <= s_24_6
        fn_state.gs_323987 = s_24_6;
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
        // C s_27_0: const #102552u : u32
        let s_27_0: u32 = 102552;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_HCR_EL2_Type_TGE(s_27_1)
        let s_27_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_27_1);
        // D s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // C s_27_4: const #1u : u8
        let s_27_4: bool = true;
        // C s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 1u16);
        // D s_27_6: cmp-eq s_27_3 s_27_5
        let s_27_6: bool = ((s_27_3) == (s_27_5));
        // D s_27_7: write-var gs#323981 <= s_27_6
        fn_state.gs_323981 = s_27_6;
        // N s_27_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #102552u : u32
        let s_28_0: u32 = 102552;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_HCR_EL2_Type_E2H(s_28_1)
        let s_28_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_28_1);
        // D s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // D s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#323980 <= s_28_6
        fn_state.gs_323980 = s_28_6;
        // N s_28_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #20784u : u32
        let s_29_0: u32 = 20784;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call _get_SCTLR_EL2_Type_SPAN(s_29_1)
        let s_29_2: bool = u_get_SCTLR_EL2_Type_SPAN(state, tracer, s_29_1);
        // D s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // C s_29_4: const #0u : u8
        let s_29_4: bool = false;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 1u16);
        // D s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // D s_29_7: write-var gs#323979 <= s_29_6
        fn_state.gs_323979 = s_29_6;
        // N s_29_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #400u : u32
        let s_30_0: u32 = 400;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call AArch32_WriteMode(s_30_1)
        let s_30_2: () = AArch32_WriteMode(state, tracer, s_30_1);
        // C s_30_3: const #() : ()
        let s_30_3: () = ();
        // S s_30_4: call HSCTLR_read(s_30_3)
        let s_30_4: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_30_3);
        // S s_30_5: call _get_HSCTLR_Type_EE(s_30_4)
        let s_30_5: bool = u_get_HSCTLR_Type_EE(state, tracer, s_30_4);
        // C s_30_6: const #16974u : u32
        let s_30_6: u32 = 16974;
        // N s_30_7: write-reg s_30_6 <= s_30_5
        let s_30_7: () = {
            state.write_register::<bool>(s_30_6 as isize, s_30_5);
            tracer.write_register(s_30_6 as isize, s_30_5);
        };
        // C s_30_8: const #32s : i64
        let s_30_8: i64 = 32;
        // C s_30_9: cast zx s_30_8 -> i
        let s_30_9: i128 = (i128::try_from(s_30_8).unwrap());
        // S s_30_10: call __UNKNOWN_bits(s_30_9)
        let s_30_10: Bits = u__UNKNOWN_bits(state, tracer, s_30_9);
        // S s_30_11: cast reint s_30_10 -> u32
        let s_30_11: u32 = (s_30_10.value() as u32);
        // S s_30_12: call ELR_hyp_write(s_30_11)
        let s_30_12: () = ELR_hyp_write(state, tracer, s_30_11);
        // C s_30_13: const #32s : i64
        let s_30_13: i64 = 32;
        // C s_30_14: cast zx s_30_13 -> i
        let s_30_14: i128 = (i128::try_from(s_30_13).unwrap());
        // S s_30_15: call __UNKNOWN_bits(s_30_14)
        let s_30_15: Bits = u__UNKNOWN_bits(state, tracer, s_30_14);
        // S s_30_16: cast reint s_30_15 -> u32
        let s_30_16: u32 = (s_30_15.value() as u32);
        // S s_30_17: call Mk_HSR_Type(s_30_16)
        let s_30_17: ProductType700c18a878c5601b = Mk_HSR_Type(state, tracer, s_30_16);
        // S s_30_18: call HSR_write(s_30_17)
        let s_30_18: () = HSR_write(state, tracer, s_30_17);
        // C s_30_19: const #32s : i64
        let s_30_19: i64 = 32;
        // C s_30_20: cast zx s_30_19 -> i
        let s_30_20: i128 = (i128::try_from(s_30_19).unwrap());
        // S s_30_21: call __UNKNOWN_bits(s_30_20)
        let s_30_21: Bits = u__UNKNOWN_bits(state, tracer, s_30_20);
        // S s_30_22: cast reint s_30_21 -> u32
        let s_30_22: u32 = (s_30_21.value() as u32);
        // S s_30_23: call Mk_SPSR_hyp_Type(s_30_22)
        let s_30_23: ProductType700c18a878c5601b = Mk_SPSR_hyp_Type(
            state,
            tracer,
            s_30_22,
        );
        // S s_30_24: call SPSR_hyp_write(s_30_23)
        let s_30_24: () = SPSR_hyp_write(state, tracer, s_30_23);
        // C s_30_25: const #32s : i64
        let s_30_25: i64 = 32;
        // C s_30_26: cast zx s_30_25 -> i
        let s_30_26: i128 = (i128::try_from(s_30_25).unwrap());
        // S s_30_27: call __UNKNOWN_bits(s_30_26)
        let s_30_27: Bits = u__UNKNOWN_bits(state, tracer, s_30_26);
        // S s_30_28: cast reint s_30_27 -> u32
        let s_30_28: u32 = (s_30_27.value() as u32);
        // S s_30_29: call DLR_write(s_30_28)
        let s_30_29: () = DLR_write(state, tracer, s_30_28);
        // C s_30_30: const #32s : i64
        let s_30_30: i64 = 32;
        // C s_30_31: cast zx s_30_30 -> i
        let s_30_31: i128 = (i128::try_from(s_30_30).unwrap());
        // S s_30_32: call __UNKNOWN_bits(s_30_31)
        let s_30_32: Bits = u__UNKNOWN_bits(state, tracer, s_30_31);
        // S s_30_33: cast reint s_30_32 -> u32
        let s_30_33: u32 = (s_30_32.value() as u32);
        // S s_30_34: call Mk_DSPSR_Type(s_30_33)
        let s_30_34: ProductType700c18a878c5601b = Mk_DSPSR_Type(state, tracer, s_30_33);
        // S s_30_35: call DSPSR_write(s_30_34)
        let s_30_35: () = DSPSR_write(state, tracer, s_30_34);
        // N s_30_36: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#323977 <= s_32_0
        fn_state.gs_323977 = s_32_0;
        // N s_32_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
