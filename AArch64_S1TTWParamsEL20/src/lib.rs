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
use HaveRME::*;
use HaveFeatCMOW::*;
use u_get_TCR2_EL2_Type_PIE::*;
use AArch64_S1DecodeTG1::*;
use AArch64_HaveHPDExt::*;
use u_get_TCR_EL2_Type_T0SZ::*;
use u_get_TTBR1_EL2_Type_SKL::*;
use HaveS1PIExt::*;
use HaveMTE4Ext::*;
use u_get_SCR_EL3_Type_SIF::*;
use u_get_SCTLR_EL2_Type_WXN::*;
use HavePACExt::*;
use HaveSecureEL2Ext::*;
use u_get_TCR_EL2_Type_HPD1::*;
use u_get_SCTLR2_EL2_Type_EMEC::*;
use Mk_MAIRType::*;
use u_get_SCTLR_EL2_Type_EE::*;
use HaveFeatMEC::*;
use u_get_SCTLR_EL2_Type_CMOW::*;
use u_get_TCR_EL2_Type_E0PD1::*;
use u_get_TTBR0_EL2_Type_SKL::*;
use u_get_TCR_EL2_Type_IRGN0::*;
use Have128BitDescriptorExt::*;
use u_get_TCR_EL2_Type_TG1::*;
use HaveE0PDExt::*;
use u_get_TCR_EL2_Type_ORGN1::*;
use Mk_S1PIRType::*;
use IsSCTLR2EL2Enabled::*;
use u_get_TCR_EL2_Type_TBID1::*;
use HaveAccessFlagUpdateExt::*;
use u_get_TCR_EL2_Type_SH0::*;
use u_get_SCTLR_EL2_Type_EPAN::*;
use u_get_TCR_EL2_Type_SH1::*;
use HaveTME::*;
use u_get_TCR2_EL2_Type_DisCH1::*;
use u_get_TCR_EL2_Type_TBI0::*;
use u_get_SCTLR_EL2_Type_nTLSMD::*;
use u_get_TCR2_EL2_Type_PnCH::*;
use HaveTrapLoadStoreMultipleDeviceExt::*;
use Have52BitIPAAndPASpaceExt::*;
use u_get_TCR_EL2_Type_MTX1::*;
use TTBR0_EL2_read::*;
use HaveSVE::*;
use u_get_TCR_EL2_Type_MTX0::*;
use u_get_TCR_EL2_Type_E0PD0::*;
use HaveTHExt::*;
use u_get_TCR2_EL2_Type_AMEC1::*;
use u_get_TCR2_EL2_Type_D128::*;
use u_get_TCR2_EL2_Type_AMEC0::*;
use u_get_HCR_EL2_Type_E2H::*;
use AArch64_S1DecodeTG0::*;
use u_get_TCR_EL2_Type_HPD0::*;
use IsTCR2EL2Enabled::*;
use HaveDirtyBitModifierExt::*;
use HaveAIEExt::*;
use u_get_TCR_EL2_Type_TBI1::*;
use HavePAN3Ext::*;
use u_get_TCR2_EL2_Type_AIE::*;
use u_get_TCR_EL2_Type_NFD0::*;
use u_get_TCR_EL2_Type_NFD1::*;
use u_get_TCR2_EL2_Type_DisCH0::*;
use HaveAccessFlagUpdateForTableExt::*;
use u_get_TCR_EL2_Type_T1SZ::*;
use u_get_TCR_EL2_Type_IPS::*;
use u_get_TCR_EL2_Type_TBID0::*;
use u_get_TCR_EL2_Type_IRGN1::*;
use u_get_TCR2_EL2_Type_HAFT::*;
use u_get_TCR_EL2_Type_TG0::*;
use u_get_TCR_EL2_Type_ORGN0::*;
use common::*;
pub fn AArch64_S1TTWParamsEL20<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ss: u32,
    varange: u32,
) -> ProductTypeef284266e139aee2 {
    #[derive(Default)]
    struct FunctionState {
        ga_10332: i64,
        ga_10317: u64,
        gs_13785: bool,
        gs_13786: bool,
        gs_13771: bool,
        gs_13788: bool,
        gs_13789: bool,
        gs_13782: bool,
        gs_13772: bool,
        ga_10331: u64,
        ga_10322: u64,
        gs_13784: bool,
        gs_13783: bool,
        gs_13828: bool,
        walkparams: ProductTypeef284266e139aee2,
        gs_13773: bool,
        gs_13798: bool,
        gs_13774: bool,
        gs_13815: bool,
        ga_10323: i64,
        ga_10318: i64,
        gs_13781: bool,
        gs_13787: bool,
        ss: u32,
        varange: u32,
    }
    let fn_state = FunctionState {
        ss,
        varange,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call Have128BitDescriptorExt(s_0_0)
        let s_0_1: bool = Have128BitDescriptorExt(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b159 b1
        if s_0_1 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#13771 <= s_1_0
        fn_state.gs_13771 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_2_0: read-var gs#13771:u8
        let s_2_0: bool = fn_state.gs_13771;
        // N s_2_1: branch s_2_0 b158 b3
        if s_2_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var walkparams.3 <= s_3_0
        fn_state.walkparams._3 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_4_0: read-var varange:u32
        let s_4_0: u32 = fn_state.varange;
        // C s_4_1: const #0u : u32
        let s_4_1: u32 = 0;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // N s_4_3: branch s_4_2 b133 b5
        if s_4_2 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_5_0: const #12816u : u32
        let s_5_0: u32 = 12816;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_TCR_EL2_Type_TG1(s_5_1)
        let s_5_2: u8 = u_get_TCR_EL2_Type_TG1(state, tracer, s_5_1);
        // D s_5_3: call AArch64_S1DecodeTG1(s_5_2)
        let s_5_3: u32 = AArch64_S1DecodeTG1(state, tracer, s_5_2);
        // D s_5_4: write-var walkparams.36 <= s_5_3
        fn_state.walkparams._36 = s_5_3;
        // C s_5_5: const #12816u : u32
        let s_5_5: u32 = 12816;
        // D s_5_6: read-reg s_5_5:struct
        let s_5_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_5 as isize);
            tracer.read_register(s_5_5 as isize, value);
            value
        };
        // D s_5_7: call _get_TCR_EL2_Type_T1SZ(s_5_6)
        let s_5_7: u8 = u_get_TCR_EL2_Type_T1SZ(state, tracer, s_5_6);
        // D s_5_8: write-var walkparams.37 <= s_5_7
        fn_state.walkparams._37 = s_5_7;
        // C s_5_9: const #12816u : u32
        let s_5_9: u32 = 12816;
        // D s_5_10: read-reg s_5_9:struct
        let s_5_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_9 as isize);
            tracer.read_register(s_5_9 as isize, value);
            value
        };
        // D s_5_11: call _get_TCR_EL2_Type_IRGN1(s_5_10)
        let s_5_11: u8 = u_get_TCR_EL2_Type_IRGN1(state, tracer, s_5_10);
        // D s_5_12: write-var walkparams.16 <= s_5_11
        fn_state.walkparams._16 = s_5_11;
        // C s_5_13: const #12816u : u32
        let s_5_13: u32 = 12816;
        // D s_5_14: read-reg s_5_13:struct
        let s_5_14: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_13 as isize);
            tracer.read_register(s_5_13 as isize, value);
            value
        };
        // D s_5_15: call _get_TCR_EL2_Type_ORGN1(s_5_14)
        let s_5_15: u8 = u_get_TCR_EL2_Type_ORGN1(state, tracer, s_5_14);
        // D s_5_16: write-var walkparams.23 <= s_5_15
        fn_state.walkparams._23 = s_5_15;
        // C s_5_17: const #12816u : u32
        let s_5_17: u32 = 12816;
        // D s_5_18: read-reg s_5_17:struct
        let s_5_18: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_17 as isize);
            tracer.read_register(s_5_17 as isize, value);
            value
        };
        // D s_5_19: call _get_TCR_EL2_Type_SH1(s_5_18)
        let s_5_19: u8 = u_get_TCR_EL2_Type_SH1(state, tracer, s_5_18);
        // D s_5_20: write-var walkparams.29 <= s_5_19
        fn_state.walkparams._29 = s_5_19;
        // C s_5_21: const #12816u : u32
        let s_5_21: u32 = 12816;
        // D s_5_22: read-reg s_5_21:struct
        let s_5_22: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_21 as isize);
            tracer.read_register(s_5_21 as isize, value);
            value
        };
        // D s_5_23: call _get_TCR_EL2_Type_TBI1(s_5_22)
        let s_5_23: bool = u_get_TCR_EL2_Type_TBI1(state, tracer, s_5_22);
        // D s_5_24: write-var walkparams.34 <= s_5_23
        fn_state.walkparams._34 = s_5_23;
        // C s_5_25: const #() : ()
        let s_5_25: () = ();
        // S s_5_26: call HaveSVE(s_5_25)
        let s_5_26: bool = HaveSVE(state, tracer, s_5_25);
        // N s_5_27: branch s_5_26 b132 b6
        if s_5_26 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HaveTME(s_6_0)
        let s_6_1: bool = HaveTME(state, tracer, s_6_0);
        // D s_6_2: write-var gs#13815 <= s_6_1
        fn_state.gs_13815 = s_6_1;
        // N s_6_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_7_0: read-var gs#13815:u8
        let s_7_0: bool = fn_state.gs_13815;
        // N s_7_1: branch s_7_0 b131 b8
        if s_7_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var walkparams.20 <= s_8_0
        fn_state.walkparams._20 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call HavePACExt(s_9_0)
        let s_9_1: bool = HavePACExt(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b130 b10
        if s_9_1 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var walkparams.35 <= s_10_0
        fn_state.walkparams._35 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HaveE0PDExt(s_11_0)
        let s_11_1: bool = HaveE0PDExt(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b129 b12
        if s_11_1 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var walkparams.8 <= s_12_0
        fn_state.walkparams._8 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call AArch64_HaveHPDExt(s_13_0)
        let s_13_1: bool = AArch64_HaveHPDExt(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b128 b14
        if s_13_1 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var walkparams.15 <= s_14_0
        fn_state.walkparams._15 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HaveMTE4Ext(s_15_0)
        let s_15_1: bool = HaveMTE4Ext(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b127 b16
        if s_15_1 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var walkparams.19 <= s_16_0
        fn_state.walkparams._19 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_17_0: read-var walkparams.3:struct
        let s_17_0: bool = fn_state.walkparams._3;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 1u16);
        // C s_17_2: const #1u : u8
        let s_17_2: bool = true;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // N s_17_5: branch s_17_4 b126 b18
        if s_17_4 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_18_0: const #0u : u8
        let s_18_0: u8 = 0;
        // D s_18_1: write-var walkparams.31 <= s_18_0
        fn_state.walkparams._31 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_19_0: read-var walkparams.3:struct
        let s_19_0: bool = fn_state.walkparams._3;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #1u : u8
        let s_19_2: bool = true;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // N s_19_5: branch s_19_4 b125 b20
        if s_19_4 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var walkparams.6 <= s_20_0
        fn_state.walkparams._6 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_22_0: const #10544u : u32
        let s_22_0: u32 = 10544;
        // D s_22_1: read-reg s_22_0:u64
        let s_22_1: u64 = {
            let value = state.read_register::<u64>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call Mk_MAIRType(s_22_1)
        let s_22_2: ProductType5c790c8ef59cc8b2 = Mk_MAIRType(state, tracer, s_22_1);
        // D s_22_3: write-var walkparams.17 <= s_22_2
        fn_state.walkparams._17 = s_22_2;
        // C s_22_4: const #() : ()
        let s_22_4: () = ();
        // S s_22_5: call HaveAIEExt(s_22_4)
        let s_22_5: bool = HaveAIEExt(state, tracer, s_22_4);
        // N s_22_6: branch s_22_5 b124 b23
        if s_22_5 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HaveAIEExt(s_24_0)
        let s_24_1: bool = HaveAIEExt(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b123 b25
        if s_24_1 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#13772 <= s_25_0
        fn_state.gs_13772 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_26_0: read-var gs#13772:u8
        let s_26_0: bool = fn_state.gs_13772;
        // N s_26_1: branch s_26_0 b122 b27
        if s_26_0 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var walkparams.0 <= s_27_0
        fn_state.walkparams._0 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_28_0: const #20784u : u32
        let s_28_0: u32 = 20784;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_SCTLR_EL2_Type_WXN(s_28_1)
        let s_28_2: bool = u_get_SCTLR_EL2_Type_WXN(state, tracer, s_28_1);
        // D s_28_3: write-var walkparams.39 <= s_28_2
        fn_state.walkparams._39 = s_28_2;
        // C s_28_4: const #12816u : u32
        let s_28_4: u32 = 12816;
        // D s_28_5: read-reg s_28_4:struct
        let s_28_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_4 as isize);
            tracer.read_register(s_28_4 as isize, value);
            value
        };
        // D s_28_6: call _get_TCR_EL2_Type_IPS(s_28_5)
        let s_28_6: u8 = u_get_TCR_EL2_Type_IPS(state, tracer, s_28_5);
        // D s_28_7: write-var walkparams.28 <= s_28_6
        fn_state.walkparams._28 = s_28_6;
        // C s_28_8: const #20784u : u32
        let s_28_8: u32 = 20784;
        // D s_28_9: read-reg s_28_8:struct
        let s_28_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_8 as isize);
            tracer.read_register(s_28_8 as isize, value);
            value
        };
        // D s_28_10: call _get_SCTLR_EL2_Type_EE(s_28_9)
        let s_28_10: bool = u_get_SCTLR_EL2_Type_EE(state, tracer, s_28_9);
        // D s_28_11: write-var walkparams.9 <= s_28_10
        fn_state.walkparams._9 = s_28_10;
        // C s_28_12: const #424u : u32
        let s_28_12: u32 = 424;
        // D s_28_13: read-reg s_28_12:u8
        let s_28_13: u8 = {
            let value = state.read_register::<u8>(s_28_12 as isize);
            tracer.read_register(s_28_12 as isize, value);
            value
        };
        // C s_28_14: const #2u : u8
        let s_28_14: u8 = 2;
        // D s_28_15: cmp-lt s_28_13 s_28_14
        let s_28_15: bool = ((s_28_13) < (s_28_14));
        // N s_28_16: branch s_28_15 b118 b29
        if s_28_15 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#13774 <= s_29_0
        fn_state.gs_13774 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_30_0: read-var gs#13774:u8
        let s_30_0: bool = fn_state.gs_13774;
        // N s_30_1: branch s_30_0 b117 b31
        if s_30_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var walkparams.30 <= s_31_0
        fn_state.walkparams._30 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call HaveTrapLoadStoreMultipleDeviceExt(s_32_0)
        let s_32_1: bool = HaveTrapLoadStoreMultipleDeviceExt(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b116 b33
        if s_32_1 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var walkparams.21 <= s_33_0
        fn_state.walkparams._21 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call HaveFeatCMOW(s_34_0)
        let s_34_1: bool = HaveFeatCMOW(state, tracer, s_34_0);
        // N s_34_2: branch s_34_1 b115 b35
        if s_34_1 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var walkparams.2 <= s_35_0
        fn_state.walkparams._2 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call HaveAccessFlagUpdateExt(s_36_0)
        let s_36_1: bool = HaveAccessFlagUpdateExt(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b111 b37
        if s_36_1 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var walkparams.12 <= s_37_0
        fn_state.walkparams._12 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call HaveDirtyBitModifierExt(s_38_0)
        let s_38_1: bool = HaveDirtyBitModifierExt(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b107 b39
        if s_38_1 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var walkparams.14 <= s_39_0
        fn_state.walkparams._14 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_40_0: read-var walkparams.36:struct
        let s_40_0: u32 = fn_state.walkparams._36;
        // C s_40_1: const #0u : u32
        let s_40_1: u32 = 0;
        // D s_40_2: cmp-eq s_40_0 s_40_1
        let s_40_2: bool = ((s_40_0) == (s_40_1));
        // N s_40_3: branch s_40_2 b106 b41
        if s_40_2 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_41_0: read-var walkparams.36:struct
        let s_41_0: u32 = fn_state.walkparams._36;
        // C s_41_1: const #1u : u32
        let s_41_1: u32 = 1;
        // D s_41_2: cmp-eq s_41_0 s_41_1
        let s_41_2: bool = ((s_41_0) == (s_41_1));
        // D s_41_3: write-var gs#13781 <= s_41_2
        fn_state.gs_13781 = s_41_2;
        // N s_41_4: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_42_0: read-var gs#13781:u8
        let s_42_0: bool = fn_state.gs_13781;
        // N s_42_1: branch s_42_0 b105 b43
        if s_42_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#13782 <= s_43_0
        fn_state.gs_13782 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_44_0: read-var gs#13782:u8
        let s_44_0: bool = fn_state.gs_13782;
        // N s_44_1: branch s_44_0 b101 b45
        if s_44_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var walkparams.7 <= s_45_0
        fn_state.walkparams._7 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_46_0: read-var walkparams.3:struct
        let s_46_0: bool = fn_state.walkparams._3;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // N s_46_5: branch s_46_4 b100 b47
        if s_46_4 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call HaveS1PIExt(s_47_0)
        let s_47_1: bool = HaveS1PIExt(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b99 b48
        if s_47_1 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#13798 <= s_48_0
        fn_state.gs_13798 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_49_0: read-var gs#13798:u8
        let s_49_0: bool = fn_state.gs_13798;
        // N s_49_1: branch s_49_0 b98 b50
        if s_49_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var walkparams.24 <= s_50_0
        fn_state.walkparams._24 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_51_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call HaveS1PIExt(s_52_0)
        let s_52_1: bool = HaveS1PIExt(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b97 b53
        if s_52_1 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_53_0: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call HavePAN3Ext(s_54_0)
        let s_54_1: bool = HavePAN3Ext(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b93 b55
        if s_54_1 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var walkparams.11 <= s_55_0
        fn_state.walkparams._11 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call HaveTHExt(s_56_0)
        let s_56_1: bool = HaveTHExt(state, tracer, s_56_0);
        // N s_56_2: branch s_56_1 b92 b57
        if s_56_1 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#13783 <= s_57_0
        fn_state.gs_13783 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_58_0: read-var gs#13783:u8
        let s_58_0: bool = fn_state.gs_13783;
        // N s_58_1: branch s_58_0 b91 b59
        if s_58_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#13784 <= s_59_0
        fn_state.gs_13784 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_60_0: read-var gs#13784:u8
        let s_60_0: bool = fn_state.gs_13784;
        // N s_60_1: branch s_60_0 b90 b61
        if s_60_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var walkparams.27 <= s_61_0
        fn_state.walkparams._27 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call HaveAccessFlagUpdateForTableExt(s_62_0)
        let s_62_1: bool = HaveAccessFlagUpdateForTableExt(state, tracer, s_62_0);
        // N s_62_2: branch s_62_1 b89 b63
        if s_62_1 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#13785 <= s_63_0
        fn_state.gs_13785 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_64_0: read-var gs#13785:u8
        let s_64_0: bool = fn_state.gs_13785;
        // N s_64_1: branch s_64_0 b88 b65
        if s_64_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#13786 <= s_65_0
        fn_state.gs_13786 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_66_0: read-var gs#13786:u8
        let s_66_0: bool = fn_state.gs_13786;
        // N s_66_1: branch s_66_0 b87 b67
        if s_66_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var walkparams.13 <= s_67_0
        fn_state.walkparams._13 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_68_0: const #() : ()
        let s_68_0: () = ();
        // S s_68_1: call HaveFeatMEC(s_68_0)
        let s_68_1: bool = HaveFeatMEC(state, tracer, s_68_0);
        // N s_68_2: branch s_68_1 b86 b69
        if s_68_1 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#13787 <= s_69_0
        fn_state.gs_13787 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_70_0: read-var gs#13787:u8
        let s_70_0: bool = fn_state.gs_13787;
        // N s_70_1: branch s_70_0 b85 b71
        if s_70_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var walkparams.10 <= s_71_0
        fn_state.walkparams._10 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call HaveFeatMEC(s_72_0)
        let s_72_1: bool = HaveFeatMEC(state, tracer, s_72_0);
        // N s_72_2: branch s_72_1 b84 b73
        if s_72_1 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#13788 <= s_73_0
        fn_state.gs_13788 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_74_0: read-var gs#13788:u8
        let s_74_0: bool = fn_state.gs_13788;
        // N s_74_1: branch s_74_0 b83 b75
        if s_74_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#13789 <= s_75_0
        fn_state.gs_13789 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_76_0: read-var gs#13789:u8
        let s_76_0: bool = fn_state.gs_13789;
        // N s_76_1: branch s_76_0 b79 b77
        if s_76_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var walkparams.1 <= s_77_0
        fn_state.walkparams._1 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_78_0: read-var walkparams:struct
        let s_78_0: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // N s_78_1: return s_78_0
        return s_78_0;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_79_0: read-var varange:u32
        let s_79_0: u32 = fn_state.varange;
        // C s_79_1: const #0u : u32
        let s_79_1: u32 = 0;
        // D s_79_2: cmp-eq s_79_0 s_79_1
        let s_79_2: bool = ((s_79_0) == (s_79_1));
        // N s_79_3: branch s_79_2 b82 b80
        if s_79_2 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_80_0: const #15752u : u32
        let s_80_0: u32 = 15752;
        // D s_80_1: read-reg s_80_0:struct
        let s_80_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // D s_80_2: call _get_TCR2_EL2_Type_AMEC1(s_80_1)
        let s_80_2: bool = u_get_TCR2_EL2_Type_AMEC1(state, tracer, s_80_1);
        // D s_80_3: write-var walkparams.1 <= s_80_2
        fn_state.walkparams._1 = s_80_2;
        // N s_80_4: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_81_0: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_82_0: const #15752u : u32
        let s_82_0: u32 = 15752;
        // D s_82_1: read-reg s_82_0:struct
        let s_82_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call _get_TCR2_EL2_Type_AMEC0(s_82_1)
        let s_82_2: bool = u_get_TCR2_EL2_Type_AMEC0(state, tracer, s_82_1);
        // D s_82_3: write-var walkparams.1 <= s_82_2
        fn_state.walkparams._1 = s_82_2;
        // N s_82_4: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call IsTCR2EL2Enabled(s_83_0)
        let s_83_1: bool = IsTCR2EL2Enabled(state, tracer, s_83_0);
        // D s_83_2: write-var gs#13789 <= s_83_1
        fn_state.gs_13789 = s_83_1;
        // N s_83_3: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_84_0: read-var ss:u32
        let s_84_0: u32 = fn_state.ss;
        // C s_84_1: const #2u : u32
        let s_84_1: u32 = 2;
        // D s_84_2: cmp-eq s_84_0 s_84_1
        let s_84_2: bool = ((s_84_0) == (s_84_1));
        // D s_84_3: write-var gs#13788 <= s_84_2
        fn_state.gs_13788 = s_84_2;
        // N s_84_4: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_85_0: const #102680u : u32
        let s_85_0: u32 = 102680;
        // D s_85_1: read-reg s_85_0:struct
        let s_85_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call _get_SCTLR2_EL2_Type_EMEC(s_85_1)
        let s_85_2: bool = u_get_SCTLR2_EL2_Type_EMEC(state, tracer, s_85_1);
        // D s_85_3: write-var walkparams.10 <= s_85_2
        fn_state.walkparams._10 = s_85_2;
        // N s_85_4: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_86_0: const #() : ()
        let s_86_0: () = ();
        // S s_86_1: call IsSCTLR2EL2Enabled(s_86_0)
        let s_86_1: bool = IsSCTLR2EL2Enabled(state, tracer, s_86_0);
        // D s_86_2: write-var gs#13787 <= s_86_1
        fn_state.gs_13787 = s_86_1;
        // N s_86_3: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_87_0: const #15752u : u32
        let s_87_0: u32 = 15752;
        // D s_87_1: read-reg s_87_0:struct
        let s_87_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_87_0 as isize);
            tracer.read_register(s_87_0 as isize, value);
            value
        };
        // D s_87_2: call _get_TCR2_EL2_Type_HAFT(s_87_1)
        let s_87_2: bool = u_get_TCR2_EL2_Type_HAFT(state, tracer, s_87_1);
        // D s_87_3: write-var walkparams.13 <= s_87_2
        fn_state.walkparams._13 = s_87_2;
        // N s_87_4: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_88_0: const #() : ()
        let s_88_0: () = ();
        // S s_88_1: call IsTCR2EL2Enabled(s_88_0)
        let s_88_1: bool = IsTCR2EL2Enabled(state, tracer, s_88_0);
        // D s_88_2: write-var gs#13786 <= s_88_1
        fn_state.gs_13786 = s_88_1;
        // N s_88_3: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_89_0: read-var walkparams.12:struct
        let s_89_0: bool = fn_state.walkparams._12;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #1u : u8
        let s_89_2: bool = true;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // D s_89_5: write-var gs#13785 <= s_89_4
        fn_state.gs_13785 = s_89_4;
        // N s_89_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_90_0: const #15752u : u32
        let s_90_0: u32 = 15752;
        // D s_90_1: read-reg s_90_0:struct
        let s_90_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // D s_90_2: call _get_TCR2_EL2_Type_PnCH(s_90_1)
        let s_90_2: bool = u_get_TCR2_EL2_Type_PnCH(state, tracer, s_90_1);
        // D s_90_3: write-var walkparams.27 <= s_90_2
        fn_state.walkparams._27 = s_90_2;
        // N s_90_4: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_91_0: const #() : ()
        let s_91_0: () = ();
        // S s_91_1: call IsTCR2EL2Enabled(s_91_0)
        let s_91_1: bool = IsTCR2EL2Enabled(state, tracer, s_91_0);
        // D s_91_2: write-var gs#13784 <= s_91_1
        fn_state.gs_13784 = s_91_1;
        // N s_91_3: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_92_0: read-var walkparams.3:struct
        let s_92_0: bool = fn_state.walkparams._3;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #0u : u8
        let s_92_2: bool = false;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#13783 <= s_92_4
        fn_state.gs_13783 = s_92_4;
        // N s_92_6: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_93_0: read-var walkparams.24:struct
        let s_93_0: bool = fn_state.walkparams._24;
        // D s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 1u16);
        // C s_93_2: const #0u : u8
        let s_93_2: bool = false;
        // C s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // D s_93_4: cmp-eq s_93_1 s_93_3
        let s_93_4: bool = ((s_93_1) == (s_93_3));
        // N s_93_5: branch s_93_4 b96 b94
        if s_93_4 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_94_0: const #1u : u8
        let s_94_0: bool = true;
        // D s_94_1: write-var walkparams.11 <= s_94_0
        fn_state.walkparams._11 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_95_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_96_0: const #20784u : u32
        let s_96_0: u32 = 20784;
        // D s_96_1: read-reg s_96_0:struct
        let s_96_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // D s_96_2: call _get_SCTLR_EL2_Type_EPAN(s_96_1)
        let s_96_2: bool = u_get_SCTLR_EL2_Type_EPAN(state, tracer, s_96_1);
        // D s_96_3: write-var walkparams.11 <= s_96_2
        fn_state.walkparams._11 = s_96_2;
        // N s_96_4: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_97_0: const #89560u : u32
        let s_97_0: u32 = 89560;
        // D s_97_1: read-reg s_97_0:u64
        let s_97_1: u64 = {
            let value = state.read_register::<u64>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // D s_97_2: call Mk_S1PIRType(s_97_1)
        let s_97_2: ProductType5c790c8ef59cc8b2 = Mk_S1PIRType(state, tracer, s_97_1);
        // D s_97_3: write-var walkparams.25 <= s_97_2
        fn_state.walkparams._25 = s_97_2;
        // C s_97_4: const #102360u : u32
        let s_97_4: u32 = 102360;
        // D s_97_5: read-reg s_97_4:u64
        let s_97_5: u64 = {
            let value = state.read_register::<u64>(s_97_4 as isize);
            tracer.read_register(s_97_4 as isize, value);
            value
        };
        // D s_97_6: call Mk_S1PIRType(s_97_5)
        let s_97_6: ProductType5c790c8ef59cc8b2 = Mk_S1PIRType(state, tracer, s_97_5);
        // D s_97_7: write-var walkparams.26 <= s_97_6
        fn_state.walkparams._26 = s_97_6;
        // N s_97_8: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_98_0: const #15752u : u32
        let s_98_0: u32 = 15752;
        // D s_98_1: read-reg s_98_0:struct
        let s_98_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // D s_98_2: call _get_TCR2_EL2_Type_PIE(s_98_1)
        let s_98_2: bool = u_get_TCR2_EL2_Type_PIE(state, tracer, s_98_1);
        // D s_98_3: write-var walkparams.24 <= s_98_2
        fn_state.walkparams._24 = s_98_2;
        // N s_98_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_99_0: const #() : ()
        let s_99_0: () = ();
        // S s_99_1: call IsTCR2EL2Enabled(s_99_0)
        let s_99_1: bool = IsTCR2EL2Enabled(state, tracer, s_99_0);
        // D s_99_2: write-var gs#13798 <= s_99_1
        fn_state.gs_13798 = s_99_1;
        // N s_99_3: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_100_0: const #1u : u8
        let s_100_0: bool = true;
        // D s_100_1: write-var walkparams.24 <= s_100_0
        fn_state.walkparams._24 = s_100_0;
        // N s_100_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_101_0: const #12816u : u32
        let s_101_0: u32 = 12816;
        // D s_101_1: read-reg s_101_0:u64
        let s_101_1: u64 = {
            let value = state.read_register::<u64>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: write-var ga#10331 <= s_101_1
        fn_state.ga_10331 = s_101_1;
        // C s_101_3: const #102552u : u32
        let s_101_3: u32 = 102552;
        // D s_101_4: read-reg s_101_3:struct
        let s_101_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_101_3 as isize);
            tracer.read_register(s_101_3 as isize, value);
            value
        };
        // D s_101_5: call _get_HCR_EL2_Type_E2H(s_101_4)
        let s_101_5: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_101_4);
        // D s_101_6: cast zx s_101_5 -> bv
        let s_101_6: Bits = Bits::new(s_101_5 as u128, 1u16);
        // C s_101_7: const #0u : u8
        let s_101_7: bool = false;
        // C s_101_8: cast zx s_101_7 -> bv
        let s_101_8: Bits = Bits::new(s_101_7 as u128, 1u16);
        // D s_101_9: cmp-eq s_101_6 s_101_8
        let s_101_9: bool = ((s_101_6) == (s_101_8));
        // N s_101_10: branch s_101_9 b104 b102
        if s_101_9 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_102_0: const #59s : i64
        let s_102_0: i64 = 59;
        // D s_102_1: write-var ga#10332 <= s_102_0
        fn_state.ga_10332 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_103_0: const #1s : i
        let s_103_0: i128 = 1;
        // D s_103_1: read-var ga#10331:u64
        let s_103_1: u64 = fn_state.ga_10331;
        // D s_103_2: cast zx s_103_1 -> bv
        let s_103_2: Bits = Bits::new(s_103_1 as u128, 64u16);
        // D s_103_3: read-var ga#10332:i64
        let s_103_3: i64 = fn_state.ga_10332;
        // D s_103_4: cast zx s_103_3 -> i
        let s_103_4: i128 = (i128::try_from(s_103_3).unwrap());
        // D s_103_5: bit-extract s_103_2 s_103_4 s_103_0
        let s_103_5: Bits = (Bits::new(
            ((s_103_2) >> (s_103_4)).value(),
            u16::try_from(s_103_0).unwrap(),
        ));
        // D s_103_6: cast reint s_103_5 -> u8
        let s_103_6: bool = ((s_103_5.value()) != 0);
        // D s_103_7: write-var walkparams.7 <= s_103_6
        fn_state.walkparams._7 = s_103_6;
        // N s_103_8: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_104_0: const #32s : i64
        let s_104_0: i64 = 32;
        // D s_104_1: write-var ga#10332 <= s_104_0
        fn_state.ga_10332 = s_104_0;
        // N s_104_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_105_0: const #() : ()
        let s_105_0: () = ();
        // S s_105_1: call Have52BitIPAAndPASpaceExt(s_105_0)
        let s_105_1: bool = Have52BitIPAAndPASpaceExt(state, tracer, s_105_0);
        // D s_105_2: write-var gs#13782 <= s_105_1
        fn_state.gs_13782 = s_105_1;
        // N s_105_3: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_106_0: const #1u : u8
        let s_106_0: bool = true;
        // D s_106_1: write-var gs#13781 <= s_106_0
        fn_state.gs_13781 = s_106_0;
        // N s_106_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_107_0: const #12816u : u32
        let s_107_0: u32 = 12816;
        // D s_107_1: read-reg s_107_0:u64
        let s_107_1: u64 = {
            let value = state.read_register::<u64>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: write-var ga#10322 <= s_107_1
        fn_state.ga_10322 = s_107_1;
        // C s_107_3: const #102552u : u32
        let s_107_3: u32 = 102552;
        // D s_107_4: read-reg s_107_3:struct
        let s_107_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_107_3 as isize);
            tracer.read_register(s_107_3 as isize, value);
            value
        };
        // D s_107_5: call _get_HCR_EL2_Type_E2H(s_107_4)
        let s_107_5: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_107_4);
        // D s_107_6: cast zx s_107_5 -> bv
        let s_107_6: Bits = Bits::new(s_107_5 as u128, 1u16);
        // C s_107_7: const #0u : u8
        let s_107_7: bool = false;
        // C s_107_8: cast zx s_107_7 -> bv
        let s_107_8: Bits = Bits::new(s_107_7 as u128, 1u16);
        // D s_107_9: cmp-eq s_107_6 s_107_8
        let s_107_9: bool = ((s_107_6) == (s_107_8));
        // N s_107_10: branch s_107_9 b110 b108
        if s_107_9 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_108_0: const #40s : i64
        let s_108_0: i64 = 40;
        // D s_108_1: write-var ga#10323 <= s_108_0
        fn_state.ga_10323 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_109_0: const #1s : i
        let s_109_0: i128 = 1;
        // D s_109_1: read-var ga#10322:u64
        let s_109_1: u64 = fn_state.ga_10322;
        // D s_109_2: cast zx s_109_1 -> bv
        let s_109_2: Bits = Bits::new(s_109_1 as u128, 64u16);
        // D s_109_3: read-var ga#10323:i64
        let s_109_3: i64 = fn_state.ga_10323;
        // D s_109_4: cast zx s_109_3 -> i
        let s_109_4: i128 = (i128::try_from(s_109_3).unwrap());
        // D s_109_5: bit-extract s_109_2 s_109_4 s_109_0
        let s_109_5: Bits = (Bits::new(
            ((s_109_2) >> (s_109_4)).value(),
            u16::try_from(s_109_0).unwrap(),
        ));
        // D s_109_6: cast reint s_109_5 -> u8
        let s_109_6: bool = ((s_109_5.value()) != 0);
        // D s_109_7: write-var walkparams.14 <= s_109_6
        fn_state.walkparams._14 = s_109_6;
        // N s_109_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_110_0: const #22s : i64
        let s_110_0: i64 = 22;
        // D s_110_1: write-var ga#10323 <= s_110_0
        fn_state.ga_10323 = s_110_0;
        // N s_110_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_111_0: const #12816u : u32
        let s_111_0: u32 = 12816;
        // D s_111_1: read-reg s_111_0:u64
        let s_111_1: u64 = {
            let value = state.read_register::<u64>(s_111_0 as isize);
            tracer.read_register(s_111_0 as isize, value);
            value
        };
        // D s_111_2: write-var ga#10317 <= s_111_1
        fn_state.ga_10317 = s_111_1;
        // C s_111_3: const #102552u : u32
        let s_111_3: u32 = 102552;
        // D s_111_4: read-reg s_111_3:struct
        let s_111_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_111_3 as isize);
            tracer.read_register(s_111_3 as isize, value);
            value
        };
        // D s_111_5: call _get_HCR_EL2_Type_E2H(s_111_4)
        let s_111_5: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_111_4);
        // D s_111_6: cast zx s_111_5 -> bv
        let s_111_6: Bits = Bits::new(s_111_5 as u128, 1u16);
        // C s_111_7: const #0u : u8
        let s_111_7: bool = false;
        // C s_111_8: cast zx s_111_7 -> bv
        let s_111_8: Bits = Bits::new(s_111_7 as u128, 1u16);
        // D s_111_9: cmp-eq s_111_6 s_111_8
        let s_111_9: bool = ((s_111_6) == (s_111_8));
        // N s_111_10: branch s_111_9 b114 b112
        if s_111_9 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_112_0: const #39s : i64
        let s_112_0: i64 = 39;
        // D s_112_1: write-var ga#10318 <= s_112_0
        fn_state.ga_10318 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_113_0: const #1s : i
        let s_113_0: i128 = 1;
        // D s_113_1: read-var ga#10317:u64
        let s_113_1: u64 = fn_state.ga_10317;
        // D s_113_2: cast zx s_113_1 -> bv
        let s_113_2: Bits = Bits::new(s_113_1 as u128, 64u16);
        // D s_113_3: read-var ga#10318:i64
        let s_113_3: i64 = fn_state.ga_10318;
        // D s_113_4: cast zx s_113_3 -> i
        let s_113_4: i128 = (i128::try_from(s_113_3).unwrap());
        // D s_113_5: bit-extract s_113_2 s_113_4 s_113_0
        let s_113_5: Bits = (Bits::new(
            ((s_113_2) >> (s_113_4)).value(),
            u16::try_from(s_113_0).unwrap(),
        ));
        // D s_113_6: cast reint s_113_5 -> u8
        let s_113_6: bool = ((s_113_5.value()) != 0);
        // D s_113_7: write-var walkparams.12 <= s_113_6
        fn_state.walkparams._12 = s_113_6;
        // N s_113_8: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_114_0: const #21s : i64
        let s_114_0: i64 = 21;
        // D s_114_1: write-var ga#10318 <= s_114_0
        fn_state.ga_10318 = s_114_0;
        // N s_114_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_115_0: const #20784u : u32
        let s_115_0: u32 = 20784;
        // D s_115_1: read-reg s_115_0:struct
        let s_115_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_115_0 as isize);
            tracer.read_register(s_115_0 as isize, value);
            value
        };
        // D s_115_2: call _get_SCTLR_EL2_Type_CMOW(s_115_1)
        let s_115_2: bool = u_get_SCTLR_EL2_Type_CMOW(state, tracer, s_115_1);
        // D s_115_3: write-var walkparams.2 <= s_115_2
        fn_state.walkparams._2 = s_115_2;
        // N s_115_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_116_0: const #20784u : u32
        let s_116_0: u32 = 20784;
        // D s_116_1: read-reg s_116_0:struct
        let s_116_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // D s_116_2: call _get_SCTLR_EL2_Type_nTLSMD(s_116_1)
        let s_116_2: bool = u_get_SCTLR_EL2_Type_nTLSMD(state, tracer, s_116_1);
        // D s_116_3: write-var walkparams.21 <= s_116_2
        fn_state.walkparams._21 = s_116_2;
        // N s_116_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_117_0: const #90704u : u32
        let s_117_0: u32 = 90704;
        // D s_117_1: read-reg s_117_0:struct
        let s_117_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_117_0 as isize);
            tracer.read_register(s_117_0 as isize, value);
            value
        };
        // D s_117_2: call _get_SCR_EL3_Type_SIF(s_117_1)
        let s_117_2: bool = u_get_SCR_EL3_Type_SIF(state, tracer, s_117_1);
        // D s_117_3: write-var walkparams.30 <= s_117_2
        fn_state.walkparams._30 = s_117_2;
        // N s_117_4: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call HaveRME(s_118_0)
        let s_118_1: bool = HaveRME(state, tracer, s_118_0);
        // S s_118_2: not s_118_1
        let s_118_2: bool = !s_118_1;
        // N s_118_3: branch s_118_2 b121 b119
        if s_118_2 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_119_0: const #() : ()
        let s_119_0: () = ();
        // S s_119_1: call HaveSecureEL2Ext(s_119_0)
        let s_119_1: bool = HaveSecureEL2Ext(state, tracer, s_119_0);
        // D s_119_2: write-var gs#13773 <= s_119_1
        fn_state.gs_13773 = s_119_1;
        // N s_119_3: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_120_0: read-var gs#13773:u8
        let s_120_0: bool = fn_state.gs_13773;
        // D s_120_1: write-var gs#13774 <= s_120_0
        fn_state.gs_13774 = s_120_0;
        // N s_120_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_121_0: const #1u : u8
        let s_121_0: bool = true;
        // D s_121_1: write-var gs#13773 <= s_121_0
        fn_state.gs_13773 = s_121_0;
        // N s_121_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_122_0: const #15752u : u32
        let s_122_0: u32 = 15752;
        // D s_122_1: read-reg s_122_0:struct
        let s_122_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_122_0 as isize);
            tracer.read_register(s_122_0 as isize, value);
            value
        };
        // D s_122_2: call _get_TCR2_EL2_Type_AIE(s_122_1)
        let s_122_2: bool = u_get_TCR2_EL2_Type_AIE(state, tracer, s_122_1);
        // D s_122_3: write-var walkparams.0 <= s_122_2
        fn_state.walkparams._0 = s_122_2;
        // N s_122_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_123_0: const #() : ()
        let s_123_0: () = ();
        // S s_123_1: call IsTCR2EL2Enabled(s_123_0)
        let s_123_1: bool = IsTCR2EL2Enabled(state, tracer, s_123_0);
        // D s_123_2: write-var gs#13772 <= s_123_1
        fn_state.gs_13772 = s_123_1;
        // N s_123_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_124_0: const #20928u : u32
        let s_124_0: u32 = 20928;
        // D s_124_1: read-reg s_124_0:u64
        let s_124_1: u64 = {
            let value = state.read_register::<u64>(s_124_0 as isize);
            tracer.read_register(s_124_0 as isize, value);
            value
        };
        // D s_124_2: call Mk_MAIRType(s_124_1)
        let s_124_2: ProductType5c790c8ef59cc8b2 = Mk_MAIRType(state, tracer, s_124_1);
        // D s_124_3: write-var walkparams.18 <= s_124_2
        fn_state.walkparams._18 = s_124_2;
        // N s_124_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_125_0: const #15752u : u32
        let s_125_0: u32 = 15752;
        // D s_125_1: read-reg s_125_0:struct
        let s_125_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_125_0 as isize);
            tracer.read_register(s_125_0 as isize, value);
            value
        };
        // D s_125_2: call _get_TCR2_EL2_Type_DisCH1(s_125_1)
        let s_125_2: bool = u_get_TCR2_EL2_Type_DisCH1(state, tracer, s_125_1);
        // D s_125_3: write-var walkparams.6 <= s_125_2
        fn_state.walkparams._6 = s_125_2;
        // N s_125_4: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_126_0: const #18432u : u32
        let s_126_0: u32 = 18432;
        // D s_126_1: read-reg s_126_0:struct
        let s_126_1: ProductType782ac6922b48c20d = {
            let value = state
                .read_register::<ProductType782ac6922b48c20d>(s_126_0 as isize);
            tracer.read_register(s_126_0 as isize, value);
            value
        };
        // D s_126_2: call _get_TTBR1_EL2_Type_SKL(s_126_1)
        let s_126_2: u8 = u_get_TTBR1_EL2_Type_SKL(state, tracer, s_126_1);
        // D s_126_3: write-var walkparams.31 <= s_126_2
        fn_state.walkparams._31 = s_126_2;
        // N s_126_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_127_0: const #12816u : u32
        let s_127_0: u32 = 12816;
        // D s_127_1: read-reg s_127_0:struct
        let s_127_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_127_0 as isize);
            tracer.read_register(s_127_0 as isize, value);
            value
        };
        // D s_127_2: call _get_TCR_EL2_Type_MTX1(s_127_1)
        let s_127_2: bool = u_get_TCR_EL2_Type_MTX1(state, tracer, s_127_1);
        // D s_127_3: write-var walkparams.19 <= s_127_2
        fn_state.walkparams._19 = s_127_2;
        // N s_127_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_128_0: const #12816u : u32
        let s_128_0: u32 = 12816;
        // D s_128_1: read-reg s_128_0:struct
        let s_128_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_128_0 as isize);
            tracer.read_register(s_128_0 as isize, value);
            value
        };
        // D s_128_2: call _get_TCR_EL2_Type_HPD1(s_128_1)
        let s_128_2: bool = u_get_TCR_EL2_Type_HPD1(state, tracer, s_128_1);
        // D s_128_3: write-var walkparams.15 <= s_128_2
        fn_state.walkparams._15 = s_128_2;
        // N s_128_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_129_0: const #12816u : u32
        let s_129_0: u32 = 12816;
        // D s_129_1: read-reg s_129_0:struct
        let s_129_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_129_0 as isize);
            tracer.read_register(s_129_0 as isize, value);
            value
        };
        // D s_129_2: call _get_TCR_EL2_Type_E0PD1(s_129_1)
        let s_129_2: bool = u_get_TCR_EL2_Type_E0PD1(state, tracer, s_129_1);
        // D s_129_3: write-var walkparams.8 <= s_129_2
        fn_state.walkparams._8 = s_129_2;
        // N s_129_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_130_0: const #12816u : u32
        let s_130_0: u32 = 12816;
        // D s_130_1: read-reg s_130_0:struct
        let s_130_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_130_0 as isize);
            tracer.read_register(s_130_0 as isize, value);
            value
        };
        // D s_130_2: call _get_TCR_EL2_Type_TBID1(s_130_1)
        let s_130_2: bool = u_get_TCR_EL2_Type_TBID1(state, tracer, s_130_1);
        // D s_130_3: write-var walkparams.35 <= s_130_2
        fn_state.walkparams._35 = s_130_2;
        // N s_130_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_131_0: const #12816u : u32
        let s_131_0: u32 = 12816;
        // D s_131_1: read-reg s_131_0:struct
        let s_131_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_131_0 as isize);
            tracer.read_register(s_131_0 as isize, value);
            value
        };
        // D s_131_2: call _get_TCR_EL2_Type_NFD1(s_131_1)
        let s_131_2: bool = u_get_TCR_EL2_Type_NFD1(state, tracer, s_131_1);
        // D s_131_3: write-var walkparams.20 <= s_131_2
        fn_state.walkparams._20 = s_131_2;
        // N s_131_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_132_0: const #1u : u8
        let s_132_0: bool = true;
        // D s_132_1: write-var gs#13815 <= s_132_0
        fn_state.gs_13815 = s_132_0;
        // N s_132_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_133_0: const #12816u : u32
        let s_133_0: u32 = 12816;
        // D s_133_1: read-reg s_133_0:struct
        let s_133_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_133_0 as isize);
            tracer.read_register(s_133_0 as isize, value);
            value
        };
        // D s_133_2: call _get_TCR_EL2_Type_TG0(s_133_1)
        let s_133_2: u8 = u_get_TCR_EL2_Type_TG0(state, tracer, s_133_1);
        // D s_133_3: call AArch64_S1DecodeTG0(s_133_2)
        let s_133_3: u32 = AArch64_S1DecodeTG0(state, tracer, s_133_2);
        // D s_133_4: write-var walkparams.36 <= s_133_3
        fn_state.walkparams._36 = s_133_3;
        // C s_133_5: const #12816u : u32
        let s_133_5: u32 = 12816;
        // D s_133_6: read-reg s_133_5:struct
        let s_133_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_133_5 as isize);
            tracer.read_register(s_133_5 as isize, value);
            value
        };
        // D s_133_7: call _get_TCR_EL2_Type_T0SZ(s_133_6)
        let s_133_7: u8 = u_get_TCR_EL2_Type_T0SZ(state, tracer, s_133_6);
        // D s_133_8: write-var walkparams.37 <= s_133_7
        fn_state.walkparams._37 = s_133_7;
        // C s_133_9: const #12816u : u32
        let s_133_9: u32 = 12816;
        // D s_133_10: read-reg s_133_9:struct
        let s_133_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_133_9 as isize);
            tracer.read_register(s_133_9 as isize, value);
            value
        };
        // D s_133_11: call _get_TCR_EL2_Type_IRGN0(s_133_10)
        let s_133_11: u8 = u_get_TCR_EL2_Type_IRGN0(state, tracer, s_133_10);
        // D s_133_12: write-var walkparams.16 <= s_133_11
        fn_state.walkparams._16 = s_133_11;
        // C s_133_13: const #12816u : u32
        let s_133_13: u32 = 12816;
        // D s_133_14: read-reg s_133_13:struct
        let s_133_14: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_133_13 as isize);
            tracer.read_register(s_133_13 as isize, value);
            value
        };
        // D s_133_15: call _get_TCR_EL2_Type_ORGN0(s_133_14)
        let s_133_15: u8 = u_get_TCR_EL2_Type_ORGN0(state, tracer, s_133_14);
        // D s_133_16: write-var walkparams.23 <= s_133_15
        fn_state.walkparams._23 = s_133_15;
        // C s_133_17: const #12816u : u32
        let s_133_17: u32 = 12816;
        // D s_133_18: read-reg s_133_17:struct
        let s_133_18: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_133_17 as isize);
            tracer.read_register(s_133_17 as isize, value);
            value
        };
        // D s_133_19: call _get_TCR_EL2_Type_SH0(s_133_18)
        let s_133_19: u8 = u_get_TCR_EL2_Type_SH0(state, tracer, s_133_18);
        // D s_133_20: write-var walkparams.29 <= s_133_19
        fn_state.walkparams._29 = s_133_19;
        // C s_133_21: const #12816u : u32
        let s_133_21: u32 = 12816;
        // D s_133_22: read-reg s_133_21:struct
        let s_133_22: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_133_21 as isize);
            tracer.read_register(s_133_21 as isize, value);
            value
        };
        // D s_133_23: call _get_TCR_EL2_Type_TBI0(s_133_22)
        let s_133_23: bool = u_get_TCR_EL2_Type_TBI0(state, tracer, s_133_22);
        // D s_133_24: write-var walkparams.34 <= s_133_23
        fn_state.walkparams._34 = s_133_23;
        // C s_133_25: const #() : ()
        let s_133_25: () = ();
        // S s_133_26: call HaveSVE(s_133_25)
        let s_133_26: bool = HaveSVE(state, tracer, s_133_25);
        // N s_133_27: branch s_133_26 b157 b134
        if s_133_26 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_134_0: const #() : ()
        let s_134_0: () = ();
        // S s_134_1: call HaveTME(s_134_0)
        let s_134_1: bool = HaveTME(state, tracer, s_134_0);
        // D s_134_2: write-var gs#13828 <= s_134_1
        fn_state.gs_13828 = s_134_1;
        // N s_134_3: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_135_0: read-var gs#13828:u8
        let s_135_0: bool = fn_state.gs_13828;
        // N s_135_1: branch s_135_0 b156 b136
        if s_135_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var walkparams.20 <= s_136_0
        fn_state.walkparams._20 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_137_0: const #() : ()
        let s_137_0: () = ();
        // S s_137_1: call HavePACExt(s_137_0)
        let s_137_1: bool = HavePACExt(state, tracer, s_137_0);
        // N s_137_2: branch s_137_1 b155 b138
        if s_137_1 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_138_0: const #0u : u8
        let s_138_0: bool = false;
        // D s_138_1: write-var walkparams.35 <= s_138_0
        fn_state.walkparams._35 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_139_0: const #() : ()
        let s_139_0: () = ();
        // S s_139_1: call HaveE0PDExt(s_139_0)
        let s_139_1: bool = HaveE0PDExt(state, tracer, s_139_0);
        // N s_139_2: branch s_139_1 b154 b140
        if s_139_1 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_140_0: const #0u : u8
        let s_140_0: bool = false;
        // D s_140_1: write-var walkparams.8 <= s_140_0
        fn_state.walkparams._8 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_141_0: const #() : ()
        let s_141_0: () = ();
        // S s_141_1: call AArch64_HaveHPDExt(s_141_0)
        let s_141_1: bool = AArch64_HaveHPDExt(state, tracer, s_141_0);
        // N s_141_2: branch s_141_1 b153 b142
        if s_141_1 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_142_0: const #0u : u8
        let s_142_0: bool = false;
        // D s_142_1: write-var walkparams.15 <= s_142_0
        fn_state.walkparams._15 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_143_0: const #() : ()
        let s_143_0: () = ();
        // S s_143_1: call HaveMTE4Ext(s_143_0)
        let s_143_1: bool = HaveMTE4Ext(state, tracer, s_143_0);
        // N s_143_2: branch s_143_1 b152 b144
        if s_143_1 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_144_0: const #0u : u8
        let s_144_0: bool = false;
        // D s_144_1: write-var walkparams.19 <= s_144_0
        fn_state.walkparams._19 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_145_0: read-var walkparams.3:struct
        let s_145_0: bool = fn_state.walkparams._3;
        // D s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 1u16);
        // C s_145_2: const #1u : u8
        let s_145_2: bool = true;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 1u16);
        // D s_145_4: cmp-eq s_145_1 s_145_3
        let s_145_4: bool = ((s_145_1) == (s_145_3));
        // N s_145_5: branch s_145_4 b151 b146
        if s_145_4 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_146_0: const #0u : u8
        let s_146_0: u8 = 0;
        // D s_146_1: write-var walkparams.31 <= s_146_0
        fn_state.walkparams._31 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_147_0: read-var walkparams.3:struct
        let s_147_0: bool = fn_state.walkparams._3;
        // D s_147_1: cast zx s_147_0 -> bv
        let s_147_1: Bits = Bits::new(s_147_0 as u128, 1u16);
        // C s_147_2: const #1u : u8
        let s_147_2: bool = true;
        // C s_147_3: cast zx s_147_2 -> bv
        let s_147_3: Bits = Bits::new(s_147_2 as u128, 1u16);
        // D s_147_4: cmp-eq s_147_1 s_147_3
        let s_147_4: bool = ((s_147_1) == (s_147_3));
        // N s_147_5: branch s_147_4 b150 b148
        if s_147_4 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_148_0: const #0u : u8
        let s_148_0: bool = false;
        // D s_148_1: write-var walkparams.6 <= s_148_0
        fn_state.walkparams._6 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_149_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_150_0: const #15752u : u32
        let s_150_0: u32 = 15752;
        // D s_150_1: read-reg s_150_0:struct
        let s_150_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_150_0 as isize);
            tracer.read_register(s_150_0 as isize, value);
            value
        };
        // D s_150_2: call _get_TCR2_EL2_Type_DisCH0(s_150_1)
        let s_150_2: bool = u_get_TCR2_EL2_Type_DisCH0(state, tracer, s_150_1);
        // D s_150_3: write-var walkparams.6 <= s_150_2
        fn_state.walkparams._6 = s_150_2;
        // N s_150_4: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_151_0: const #() : ()
        let s_151_0: () = ();
        // S s_151_1: call TTBR0_EL2_read(s_151_0)
        let s_151_1: ProductType782ac6922b48c20d = TTBR0_EL2_read(
            state,
            tracer,
            s_151_0,
        );
        // S s_151_2: call _get_TTBR0_EL2_Type_SKL(s_151_1)
        let s_151_2: u8 = u_get_TTBR0_EL2_Type_SKL(state, tracer, s_151_1);
        // D s_151_3: write-var walkparams.31 <= s_151_2
        fn_state.walkparams._31 = s_151_2;
        // N s_151_4: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_152_0: const #12816u : u32
        let s_152_0: u32 = 12816;
        // D s_152_1: read-reg s_152_0:struct
        let s_152_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_152_0 as isize);
            tracer.read_register(s_152_0 as isize, value);
            value
        };
        // D s_152_2: call _get_TCR_EL2_Type_MTX0(s_152_1)
        let s_152_2: bool = u_get_TCR_EL2_Type_MTX0(state, tracer, s_152_1);
        // D s_152_3: write-var walkparams.19 <= s_152_2
        fn_state.walkparams._19 = s_152_2;
        // N s_152_4: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_153_0: const #12816u : u32
        let s_153_0: u32 = 12816;
        // D s_153_1: read-reg s_153_0:struct
        let s_153_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_153_0 as isize);
            tracer.read_register(s_153_0 as isize, value);
            value
        };
        // D s_153_2: call _get_TCR_EL2_Type_HPD0(s_153_1)
        let s_153_2: bool = u_get_TCR_EL2_Type_HPD0(state, tracer, s_153_1);
        // D s_153_3: write-var walkparams.15 <= s_153_2
        fn_state.walkparams._15 = s_153_2;
        // N s_153_4: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_154_0: const #12816u : u32
        let s_154_0: u32 = 12816;
        // D s_154_1: read-reg s_154_0:struct
        let s_154_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_154_0 as isize);
            tracer.read_register(s_154_0 as isize, value);
            value
        };
        // D s_154_2: call _get_TCR_EL2_Type_E0PD0(s_154_1)
        let s_154_2: bool = u_get_TCR_EL2_Type_E0PD0(state, tracer, s_154_1);
        // D s_154_3: write-var walkparams.8 <= s_154_2
        fn_state.walkparams._8 = s_154_2;
        // N s_154_4: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_155_0: const #12816u : u32
        let s_155_0: u32 = 12816;
        // D s_155_1: read-reg s_155_0:struct
        let s_155_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_155_0 as isize);
            tracer.read_register(s_155_0 as isize, value);
            value
        };
        // D s_155_2: call _get_TCR_EL2_Type_TBID0(s_155_1)
        let s_155_2: bool = u_get_TCR_EL2_Type_TBID0(state, tracer, s_155_1);
        // D s_155_3: write-var walkparams.35 <= s_155_2
        fn_state.walkparams._35 = s_155_2;
        // N s_155_4: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_156_0: const #12816u : u32
        let s_156_0: u32 = 12816;
        // D s_156_1: read-reg s_156_0:struct
        let s_156_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_156_0 as isize);
            tracer.read_register(s_156_0 as isize, value);
            value
        };
        // D s_156_2: call _get_TCR_EL2_Type_NFD0(s_156_1)
        let s_156_2: bool = u_get_TCR_EL2_Type_NFD0(state, tracer, s_156_1);
        // D s_156_3: write-var walkparams.20 <= s_156_2
        fn_state.walkparams._20 = s_156_2;
        // N s_156_4: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_157_0: const #1u : u8
        let s_157_0: bool = true;
        // D s_157_1: write-var gs#13828 <= s_157_0
        fn_state.gs_13828 = s_157_0;
        // N s_157_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_158_0: const #15752u : u32
        let s_158_0: u32 = 15752;
        // D s_158_1: read-reg s_158_0:struct
        let s_158_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_158_0 as isize);
            tracer.read_register(s_158_0 as isize, value);
            value
        };
        // D s_158_2: call _get_TCR2_EL2_Type_D128(s_158_1)
        let s_158_2: bool = u_get_TCR2_EL2_Type_D128(state, tracer, s_158_1);
        // D s_158_3: write-var walkparams.3 <= s_158_2
        fn_state.walkparams._3 = s_158_2;
        // N s_158_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_159_0: const #() : ()
        let s_159_0: () = ();
        // S s_159_1: call IsTCR2EL2Enabled(s_159_0)
        let s_159_1: bool = IsTCR2EL2Enabled(state, tracer, s_159_0);
        // D s_159_2: write-var gs#13771 <= s_159_1
        fn_state.gs_13771 = s_159_1;
        // N s_159_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
