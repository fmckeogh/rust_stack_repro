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
use IsSCTLR2EL2Enabled::*;
use u_get_TCR_EL2_Type_TBID::*;
use HaveAccessFlagUpdateExt::*;
use HaveRME::*;
use u_get_TCR_EL2_Type_SH0::*;
use u_get_TCR_EL2_Type_TBI::*;
use u_get_TCR_EL2_Type_T0SZ::*;
use AArch64_HaveHPDExt::*;
use u_get_TCR2_EL2_Type_PIE::*;
use u_get_TCR_EL2_Type_MTX::*;
use HaveS1PIExt::*;
use HaveMTE4Ext::*;
use u_get_TCR2_EL2_Type_PnCH::*;
use Have52BitIPAAndPASpaceExt::*;
use u_get_SCR_EL3_Type_SIF::*;
use u_get_TCR_EL2_Type_PS::*;
use HaveTHExt::*;
use u_get_SCTLR_EL2_Type_WXN::*;
use HavePACExt::*;
use u_get_TCR2_EL2_Type_AMEC0::*;
use AArch64_S1DecodeTG0::*;
use u_get_HCR_EL2_Type_E2H::*;
use HaveSecureEL2Ext::*;
use IsTCR2EL2Enabled::*;
use u_get_SCTLR2_EL2_Type_EMEC::*;
use u_get_SCTLR_EL2_Type_EE::*;
use Mk_MAIRType::*;
use HaveDirtyBitModifierExt::*;
use HaveAIEExt::*;
use HaveFeatMEC::*;
use u_get_TCR_EL2_Type_HPD::*;
use u_get_TCR2_EL2_Type_AIE::*;
use u_get_TCR_EL2_Type_IRGN0::*;
use HaveAccessFlagUpdateForTableExt::*;
use u_get_TCR_EL2_Type_TG0::*;
use u_get_TCR_EL2_Type_ORGN0::*;
use u_get_TCR2_EL2_Type_HAFT::*;
use Mk_S1PIRType::*;
use common::*;
pub fn AArch64_S1TTWParamsEL2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ss: u32,
) -> ProductTypeef284266e139aee2 {
    #[derive(Default)]
    struct FunctionState {
        gs_13742: bool,
        gs_13735: bool,
        gs_13734: bool,
        gs_13741: bool,
        ga_10246: i64,
        ga_10250: u64,
        ga_10245: u64,
        gs_13738: bool,
        gs_13740: bool,
        gs_13737: bool,
        ga_10259: u64,
        gs_13725: bool,
        gs_13726: bool,
        ga_10251: i64,
        walkparams: ProductTypeef284266e139aee2,
        gs_13727: bool,
        gs_13739: bool,
        gs_13736: bool,
        ga_10260: i64,
        ss: u32,
    }
    let fn_state = FunctionState {
        ss,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_0_0: const #12816u : u32
        let s_0_0: u32 = 12816;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_TCR_EL2_Type_TG0(s_0_1)
        let s_0_2: u8 = u_get_TCR_EL2_Type_TG0(state, tracer, s_0_1);
        // D s_0_3: call AArch64_S1DecodeTG0(s_0_2)
        let s_0_3: u32 = AArch64_S1DecodeTG0(state, tracer, s_0_2);
        // D s_0_4: write-var walkparams.36 <= s_0_3
        fn_state.walkparams._36 = s_0_3;
        // C s_0_5: const #12816u : u32
        let s_0_5: u32 = 12816;
        // D s_0_6: read-reg s_0_5:struct
        let s_0_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: call _get_TCR_EL2_Type_T0SZ(s_0_6)
        let s_0_7: u8 = u_get_TCR_EL2_Type_T0SZ(state, tracer, s_0_6);
        // D s_0_8: write-var walkparams.37 <= s_0_7
        fn_state.walkparams._37 = s_0_7;
        // C s_0_9: const #12816u : u32
        let s_0_9: u32 = 12816;
        // D s_0_10: read-reg s_0_9:struct
        let s_0_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: call _get_TCR_EL2_Type_PS(s_0_10)
        let s_0_11: u8 = u_get_TCR_EL2_Type_PS(state, tracer, s_0_10);
        // D s_0_12: write-var walkparams.28 <= s_0_11
        fn_state.walkparams._28 = s_0_11;
        // C s_0_13: const #12816u : u32
        let s_0_13: u32 = 12816;
        // D s_0_14: read-reg s_0_13:struct
        let s_0_14: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: call _get_TCR_EL2_Type_IRGN0(s_0_14)
        let s_0_15: u8 = u_get_TCR_EL2_Type_IRGN0(state, tracer, s_0_14);
        // D s_0_16: write-var walkparams.16 <= s_0_15
        fn_state.walkparams._16 = s_0_15;
        // C s_0_17: const #12816u : u32
        let s_0_17: u32 = 12816;
        // D s_0_18: read-reg s_0_17:struct
        let s_0_18: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: call _get_TCR_EL2_Type_ORGN0(s_0_18)
        let s_0_19: u8 = u_get_TCR_EL2_Type_ORGN0(state, tracer, s_0_18);
        // D s_0_20: write-var walkparams.23 <= s_0_19
        fn_state.walkparams._23 = s_0_19;
        // C s_0_21: const #12816u : u32
        let s_0_21: u32 = 12816;
        // D s_0_22: read-reg s_0_21:struct
        let s_0_22: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // D s_0_23: call _get_TCR_EL2_Type_SH0(s_0_22)
        let s_0_23: u8 = u_get_TCR_EL2_Type_SH0(state, tracer, s_0_22);
        // D s_0_24: write-var walkparams.29 <= s_0_23
        fn_state.walkparams._29 = s_0_23;
        // C s_0_25: const #12816u : u32
        let s_0_25: u32 = 12816;
        // D s_0_26: read-reg s_0_25:struct
        let s_0_26: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // D s_0_27: call _get_TCR_EL2_Type_TBI(s_0_26)
        let s_0_27: bool = u_get_TCR_EL2_Type_TBI(state, tracer, s_0_26);
        // D s_0_28: write-var walkparams.34 <= s_0_27
        fn_state.walkparams._34 = s_0_27;
        // C s_0_29: const #10544u : u32
        let s_0_29: u32 = 10544;
        // D s_0_30: read-reg s_0_29:u64
        let s_0_30: u64 = {
            let value = state.read_register::<u64>(s_0_29 as isize);
            tracer.read_register(s_0_29 as isize, value);
            value
        };
        // D s_0_31: call Mk_MAIRType(s_0_30)
        let s_0_31: ProductType5c790c8ef59cc8b2 = Mk_MAIRType(state, tracer, s_0_30);
        // D s_0_32: write-var walkparams.17 <= s_0_31
        fn_state.walkparams._17 = s_0_31;
        // C s_0_33: const #() : ()
        let s_0_33: () = ();
        // S s_0_34: call HaveAIEExt(s_0_33)
        let s_0_34: bool = HaveAIEExt(state, tracer, s_0_33);
        // N s_0_35: branch s_0_34 b90 b1
        if s_0_34 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call HaveAIEExt(s_2_0)
        let s_2_1: bool = HaveAIEExt(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b89 b3
        if s_2_1 {
            return block_89(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#13725 <= s_3_0
        fn_state.gs_13725 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_4_0: read-var gs#13725:u8
        let s_4_0: bool = fn_state.gs_13725;
        // N s_4_1: branch s_4_0 b88 b5
        if s_4_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var walkparams.0 <= s_5_0
        fn_state.walkparams._0 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_6_0: const #20784u : u32
        let s_6_0: u32 = 20784;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call _get_SCTLR_EL2_Type_WXN(s_6_1)
        let s_6_2: bool = u_get_SCTLR_EL2_Type_WXN(state, tracer, s_6_1);
        // D s_6_3: write-var walkparams.39 <= s_6_2
        fn_state.walkparams._39 = s_6_2;
        // C s_6_4: const #20784u : u32
        let s_6_4: u32 = 20784;
        // D s_6_5: read-reg s_6_4:struct
        let s_6_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_4 as isize);
            tracer.read_register(s_6_4 as isize, value);
            value
        };
        // D s_6_6: call _get_SCTLR_EL2_Type_EE(s_6_5)
        let s_6_6: bool = u_get_SCTLR_EL2_Type_EE(state, tracer, s_6_5);
        // D s_6_7: write-var walkparams.9 <= s_6_6
        fn_state.walkparams._9 = s_6_6;
        // C s_6_8: const #424u : u32
        let s_6_8: u32 = 424;
        // D s_6_9: read-reg s_6_8:u8
        let s_6_9: u8 = {
            let value = state.read_register::<u8>(s_6_8 as isize);
            tracer.read_register(s_6_8 as isize, value);
            value
        };
        // C s_6_10: const #2u : u8
        let s_6_10: u8 = 2;
        // D s_6_11: cmp-lt s_6_9 s_6_10
        let s_6_11: bool = ((s_6_9) < (s_6_10));
        // N s_6_12: branch s_6_11 b84 b7
        if s_6_11 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#13727 <= s_7_0
        fn_state.gs_13727 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_8_0: read-var gs#13727:u8
        let s_8_0: bool = fn_state.gs_13727;
        // N s_8_1: branch s_8_0 b83 b9
        if s_8_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var walkparams.30 <= s_9_0
        fn_state.walkparams._30 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call HavePACExt(s_10_0)
        let s_10_1: bool = HavePACExt(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b82 b11
        if s_10_1 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var walkparams.35 <= s_11_0
        fn_state.walkparams._35 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call AArch64_HaveHPDExt(s_12_0)
        let s_12_1: bool = AArch64_HaveHPDExt(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b81 b13
        if s_12_1 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var walkparams.15 <= s_13_0
        fn_state.walkparams._15 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaveAccessFlagUpdateExt(s_14_0)
        let s_14_1: bool = HaveAccessFlagUpdateExt(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b77 b15
        if s_14_1 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var walkparams.12 <= s_15_0
        fn_state.walkparams._12 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call HaveDirtyBitModifierExt(s_16_0)
        let s_16_1: bool = HaveDirtyBitModifierExt(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b73 b17
        if s_16_1 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var walkparams.14 <= s_17_0
        fn_state.walkparams._14 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_18_0: read-var walkparams.36:struct
        let s_18_0: u32 = fn_state.walkparams._36;
        // C s_18_1: const #0u : u32
        let s_18_1: u32 = 0;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // N s_18_3: branch s_18_2 b72 b19
        if s_18_2 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_19_0: read-var walkparams.36:struct
        let s_19_0: u32 = fn_state.walkparams._36;
        // C s_19_1: const #1u : u32
        let s_19_1: u32 = 1;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // D s_19_3: write-var gs#13734 <= s_19_2
        fn_state.gs_13734 = s_19_2;
        // N s_19_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_20_0: read-var gs#13734:u8
        let s_20_0: bool = fn_state.gs_13734;
        // N s_20_1: branch s_20_0 b71 b21
        if s_20_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#13735 <= s_21_0
        fn_state.gs_13735 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_22_0: read-var gs#13735:u8
        let s_22_0: bool = fn_state.gs_13735;
        // N s_22_1: branch s_22_0 b67 b23
        if s_22_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var walkparams.7 <= s_23_0
        fn_state.walkparams._7 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HaveS1PIExt(s_24_0)
        let s_24_1: bool = HaveS1PIExt(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b66 b25
        if s_24_1 {
            return block_66(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#13736 <= s_25_0
        fn_state.gs_13736 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_26_0: read-var gs#13736:u8
        let s_26_0: bool = fn_state.gs_13736;
        // N s_26_1: branch s_26_0 b65 b27
        if s_26_0 {
            return block_65(state, tracer, fn_state);
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
        // D s_27_1: write-var walkparams.24 <= s_27_0
        fn_state.walkparams._24 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call HaveS1PIExt(s_28_0)
        let s_28_1: bool = HaveS1PIExt(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b64 b29
        if s_28_1 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call HaveMTE4Ext(s_30_0)
        let s_30_1: bool = HaveMTE4Ext(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b63 b31
        if s_30_1 {
            return block_63(state, tracer, fn_state);
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
        // D s_31_1: write-var walkparams.19 <= s_31_0
        fn_state.walkparams._19 = s_31_0;
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
        // S s_32_1: call HaveTHExt(s_32_0)
        let s_32_1: bool = HaveTHExt(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b62 b33
        if s_32_1 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#13737 <= s_33_0
        fn_state.gs_13737 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_34_0: read-var gs#13737:u8
        let s_34_0: bool = fn_state.gs_13737;
        // N s_34_1: branch s_34_0 b61 b35
        if s_34_0 {
            return block_61(state, tracer, fn_state);
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
        // D s_35_1: write-var walkparams.27 <= s_35_0
        fn_state.walkparams._27 = s_35_0;
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
        // S s_36_1: call HaveAccessFlagUpdateForTableExt(s_36_0)
        let s_36_1: bool = HaveAccessFlagUpdateForTableExt(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b60 b37
        if s_36_1 {
            return block_60(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#13738 <= s_37_0
        fn_state.gs_13738 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_38_0: read-var gs#13738:u8
        let s_38_0: bool = fn_state.gs_13738;
        // N s_38_1: branch s_38_0 b59 b39
        if s_38_0 {
            return block_59(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#13739 <= s_39_0
        fn_state.gs_13739 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_40_0: read-var gs#13739:u8
        let s_40_0: bool = fn_state.gs_13739;
        // N s_40_1: branch s_40_0 b58 b41
        if s_40_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var walkparams.13 <= s_41_0
        fn_state.walkparams._13 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call HaveFeatMEC(s_42_0)
        let s_42_1: bool = HaveFeatMEC(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b57 b43
        if s_42_1 {
            return block_57(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#13740 <= s_43_0
        fn_state.gs_13740 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_44_0: read-var gs#13740:u8
        let s_44_0: bool = fn_state.gs_13740;
        // N s_44_1: branch s_44_0 b56 b45
        if s_44_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_45_1: write-var walkparams.10 <= s_45_0
        fn_state.walkparams._10 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call HaveFeatMEC(s_46_0)
        let s_46_1: bool = HaveFeatMEC(state, tracer, s_46_0);
        // N s_46_2: branch s_46_1 b55 b47
        if s_46_1 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#13741 <= s_47_0
        fn_state.gs_13741 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_48_0: read-var gs#13741:u8
        let s_48_0: bool = fn_state.gs_13741;
        // N s_48_1: branch s_48_0 b54 b49
        if s_48_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#13742 <= s_49_0
        fn_state.gs_13742 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_50_0: read-var gs#13742:u8
        let s_50_0: bool = fn_state.gs_13742;
        // N s_50_1: branch s_50_0 b53 b51
        if s_50_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var walkparams.1 <= s_51_0
        fn_state.walkparams._1 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_52_0: read-var walkparams:struct
        let s_52_0: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // N s_52_1: return s_52_0
        return s_52_0;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_53_0: const #15752u : u32
        let s_53_0: u32 = 15752;
        // D s_53_1: read-reg s_53_0:struct
        let s_53_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // D s_53_2: call _get_TCR2_EL2_Type_AMEC0(s_53_1)
        let s_53_2: bool = u_get_TCR2_EL2_Type_AMEC0(state, tracer, s_53_1);
        // D s_53_3: write-var walkparams.1 <= s_53_2
        fn_state.walkparams._1 = s_53_2;
        // N s_53_4: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call IsTCR2EL2Enabled(s_54_0)
        let s_54_1: bool = IsTCR2EL2Enabled(state, tracer, s_54_0);
        // D s_54_2: write-var gs#13742 <= s_54_1
        fn_state.gs_13742 = s_54_1;
        // N s_54_3: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_55_0: read-var ss:u32
        let s_55_0: u32 = fn_state.ss;
        // C s_55_1: const #2u : u32
        let s_55_1: u32 = 2;
        // D s_55_2: cmp-eq s_55_0 s_55_1
        let s_55_2: bool = ((s_55_0) == (s_55_1));
        // D s_55_3: write-var gs#13741 <= s_55_2
        fn_state.gs_13741 = s_55_2;
        // N s_55_4: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_56_0: const #102680u : u32
        let s_56_0: u32 = 102680;
        // D s_56_1: read-reg s_56_0:struct
        let s_56_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: call _get_SCTLR2_EL2_Type_EMEC(s_56_1)
        let s_56_2: bool = u_get_SCTLR2_EL2_Type_EMEC(state, tracer, s_56_1);
        // D s_56_3: write-var walkparams.10 <= s_56_2
        fn_state.walkparams._10 = s_56_2;
        // N s_56_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call IsSCTLR2EL2Enabled(s_57_0)
        let s_57_1: bool = IsSCTLR2EL2Enabled(state, tracer, s_57_0);
        // D s_57_2: write-var gs#13740 <= s_57_1
        fn_state.gs_13740 = s_57_1;
        // N s_57_3: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_58_0: const #15752u : u32
        let s_58_0: u32 = 15752;
        // D s_58_1: read-reg s_58_0:struct
        let s_58_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call _get_TCR2_EL2_Type_HAFT(s_58_1)
        let s_58_2: bool = u_get_TCR2_EL2_Type_HAFT(state, tracer, s_58_1);
        // D s_58_3: write-var walkparams.13 <= s_58_2
        fn_state.walkparams._13 = s_58_2;
        // N s_58_4: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call IsTCR2EL2Enabled(s_59_0)
        let s_59_1: bool = IsTCR2EL2Enabled(state, tracer, s_59_0);
        // D s_59_2: write-var gs#13739 <= s_59_1
        fn_state.gs_13739 = s_59_1;
        // N s_59_3: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_60_0: read-var walkparams.12:struct
        let s_60_0: bool = fn_state.walkparams._12;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #1u : u8
        let s_60_2: bool = true;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#13738 <= s_60_4
        fn_state.gs_13738 = s_60_4;
        // N s_60_6: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_61_0: const #15752u : u32
        let s_61_0: u32 = 15752;
        // D s_61_1: read-reg s_61_0:struct
        let s_61_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call _get_TCR2_EL2_Type_PnCH(s_61_1)
        let s_61_2: bool = u_get_TCR2_EL2_Type_PnCH(state, tracer, s_61_1);
        // D s_61_3: write-var walkparams.27 <= s_61_2
        fn_state.walkparams._27 = s_61_2;
        // N s_61_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call IsTCR2EL2Enabled(s_62_0)
        let s_62_1: bool = IsTCR2EL2Enabled(state, tracer, s_62_0);
        // D s_62_2: write-var gs#13737 <= s_62_1
        fn_state.gs_13737 = s_62_1;
        // N s_62_3: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_63_0: const #12816u : u32
        let s_63_0: u32 = 12816;
        // D s_63_1: read-reg s_63_0:struct
        let s_63_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call _get_TCR_EL2_Type_MTX(s_63_1)
        let s_63_2: bool = u_get_TCR_EL2_Type_MTX(state, tracer, s_63_1);
        // D s_63_3: write-var walkparams.19 <= s_63_2
        fn_state.walkparams._19 = s_63_2;
        // N s_63_4: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_64_0: const #89560u : u32
        let s_64_0: u32 = 89560;
        // D s_64_1: read-reg s_64_0:u64
        let s_64_1: u64 = {
            let value = state.read_register::<u64>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call Mk_S1PIRType(s_64_1)
        let s_64_2: ProductType5c790c8ef59cc8b2 = Mk_S1PIRType(state, tracer, s_64_1);
        // D s_64_3: write-var walkparams.25 <= s_64_2
        fn_state.walkparams._25 = s_64_2;
        // N s_64_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_65_0: const #15752u : u32
        let s_65_0: u32 = 15752;
        // D s_65_1: read-reg s_65_0:struct
        let s_65_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // D s_65_2: call _get_TCR2_EL2_Type_PIE(s_65_1)
        let s_65_2: bool = u_get_TCR2_EL2_Type_PIE(state, tracer, s_65_1);
        // D s_65_3: write-var walkparams.24 <= s_65_2
        fn_state.walkparams._24 = s_65_2;
        // N s_65_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_66_0: const #() : ()
        let s_66_0: () = ();
        // S s_66_1: call IsTCR2EL2Enabled(s_66_0)
        let s_66_1: bool = IsTCR2EL2Enabled(state, tracer, s_66_0);
        // D s_66_2: write-var gs#13736 <= s_66_1
        fn_state.gs_13736 = s_66_1;
        // N s_66_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_67_0: const #12816u : u32
        let s_67_0: u32 = 12816;
        // D s_67_1: read-reg s_67_0:u64
        let s_67_1: u64 = {
            let value = state.read_register::<u64>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // D s_67_2: write-var ga#10259 <= s_67_1
        fn_state.ga_10259 = s_67_1;
        // C s_67_3: const #102552u : u32
        let s_67_3: u32 = 102552;
        // D s_67_4: read-reg s_67_3:struct
        let s_67_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_67_3 as isize);
            tracer.read_register(s_67_3 as isize, value);
            value
        };
        // D s_67_5: call _get_HCR_EL2_Type_E2H(s_67_4)
        let s_67_5: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_67_4);
        // D s_67_6: cast zx s_67_5 -> bv
        let s_67_6: Bits = Bits::new(s_67_5 as u128, 1u16);
        // C s_67_7: const #0u : u8
        let s_67_7: bool = false;
        // C s_67_8: cast zx s_67_7 -> bv
        let s_67_8: Bits = Bits::new(s_67_7 as u128, 1u16);
        // D s_67_9: cmp-eq s_67_6 s_67_8
        let s_67_9: bool = ((s_67_6) == (s_67_8));
        // N s_67_10: branch s_67_9 b70 b68
        if s_67_9 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_68_0: const #59s : i64
        let s_68_0: i64 = 59;
        // D s_68_1: write-var ga#10260 <= s_68_0
        fn_state.ga_10260 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_69_0: const #1s : i
        let s_69_0: i128 = 1;
        // D s_69_1: read-var ga#10259:u64
        let s_69_1: u64 = fn_state.ga_10259;
        // D s_69_2: cast zx s_69_1 -> bv
        let s_69_2: Bits = Bits::new(s_69_1 as u128, 64u16);
        // D s_69_3: read-var ga#10260:i64
        let s_69_3: i64 = fn_state.ga_10260;
        // D s_69_4: cast zx s_69_3 -> i
        let s_69_4: i128 = (i128::try_from(s_69_3).unwrap());
        // D s_69_5: bit-extract s_69_2 s_69_4 s_69_0
        let s_69_5: Bits = (Bits::new(
            ((s_69_2) >> (s_69_4)).value(),
            u16::try_from(s_69_0).unwrap(),
        ));
        // D s_69_6: cast reint s_69_5 -> u8
        let s_69_6: bool = ((s_69_5.value()) != 0);
        // D s_69_7: write-var walkparams.7 <= s_69_6
        fn_state.walkparams._7 = s_69_6;
        // N s_69_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_70_0: const #32s : i64
        let s_70_0: i64 = 32;
        // D s_70_1: write-var ga#10260 <= s_70_0
        fn_state.ga_10260 = s_70_0;
        // N s_70_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call Have52BitIPAAndPASpaceExt(s_71_0)
        let s_71_1: bool = Have52BitIPAAndPASpaceExt(state, tracer, s_71_0);
        // D s_71_2: write-var gs#13735 <= s_71_1
        fn_state.gs_13735 = s_71_1;
        // N s_71_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#13734 <= s_72_0
        fn_state.gs_13734 = s_72_0;
        // N s_72_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_73_0: const #12816u : u32
        let s_73_0: u32 = 12816;
        // D s_73_1: read-reg s_73_0:u64
        let s_73_1: u64 = {
            let value = state.read_register::<u64>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // D s_73_2: write-var ga#10250 <= s_73_1
        fn_state.ga_10250 = s_73_1;
        // C s_73_3: const #102552u : u32
        let s_73_3: u32 = 102552;
        // D s_73_4: read-reg s_73_3:struct
        let s_73_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_3 as isize);
            tracer.read_register(s_73_3 as isize, value);
            value
        };
        // D s_73_5: call _get_HCR_EL2_Type_E2H(s_73_4)
        let s_73_5: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_73_4);
        // D s_73_6: cast zx s_73_5 -> bv
        let s_73_6: Bits = Bits::new(s_73_5 as u128, 1u16);
        // C s_73_7: const #0u : u8
        let s_73_7: bool = false;
        // C s_73_8: cast zx s_73_7 -> bv
        let s_73_8: Bits = Bits::new(s_73_7 as u128, 1u16);
        // D s_73_9: cmp-eq s_73_6 s_73_8
        let s_73_9: bool = ((s_73_6) == (s_73_8));
        // N s_73_10: branch s_73_9 b76 b74
        if s_73_9 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_74_0: const #40s : i64
        let s_74_0: i64 = 40;
        // D s_74_1: write-var ga#10251 <= s_74_0
        fn_state.ga_10251 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_75_0: const #1s : i
        let s_75_0: i128 = 1;
        // D s_75_1: read-var ga#10250:u64
        let s_75_1: u64 = fn_state.ga_10250;
        // D s_75_2: cast zx s_75_1 -> bv
        let s_75_2: Bits = Bits::new(s_75_1 as u128, 64u16);
        // D s_75_3: read-var ga#10251:i64
        let s_75_3: i64 = fn_state.ga_10251;
        // D s_75_4: cast zx s_75_3 -> i
        let s_75_4: i128 = (i128::try_from(s_75_3).unwrap());
        // D s_75_5: bit-extract s_75_2 s_75_4 s_75_0
        let s_75_5: Bits = (Bits::new(
            ((s_75_2) >> (s_75_4)).value(),
            u16::try_from(s_75_0).unwrap(),
        ));
        // D s_75_6: cast reint s_75_5 -> u8
        let s_75_6: bool = ((s_75_5.value()) != 0);
        // D s_75_7: write-var walkparams.14 <= s_75_6
        fn_state.walkparams._14 = s_75_6;
        // N s_75_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_76_0: const #22s : i64
        let s_76_0: i64 = 22;
        // D s_76_1: write-var ga#10251 <= s_76_0
        fn_state.ga_10251 = s_76_0;
        // N s_76_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_77_0: const #12816u : u32
        let s_77_0: u32 = 12816;
        // D s_77_1: read-reg s_77_0:u64
        let s_77_1: u64 = {
            let value = state.read_register::<u64>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: write-var ga#10245 <= s_77_1
        fn_state.ga_10245 = s_77_1;
        // C s_77_3: const #102552u : u32
        let s_77_3: u32 = 102552;
        // D s_77_4: read-reg s_77_3:struct
        let s_77_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_77_3 as isize);
            tracer.read_register(s_77_3 as isize, value);
            value
        };
        // D s_77_5: call _get_HCR_EL2_Type_E2H(s_77_4)
        let s_77_5: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_77_4);
        // D s_77_6: cast zx s_77_5 -> bv
        let s_77_6: Bits = Bits::new(s_77_5 as u128, 1u16);
        // C s_77_7: const #0u : u8
        let s_77_7: bool = false;
        // C s_77_8: cast zx s_77_7 -> bv
        let s_77_8: Bits = Bits::new(s_77_7 as u128, 1u16);
        // D s_77_9: cmp-eq s_77_6 s_77_8
        let s_77_9: bool = ((s_77_6) == (s_77_8));
        // N s_77_10: branch s_77_9 b80 b78
        if s_77_9 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_78_0: const #39s : i64
        let s_78_0: i64 = 39;
        // D s_78_1: write-var ga#10246 <= s_78_0
        fn_state.ga_10246 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_79_0: const #1s : i
        let s_79_0: i128 = 1;
        // D s_79_1: read-var ga#10245:u64
        let s_79_1: u64 = fn_state.ga_10245;
        // D s_79_2: cast zx s_79_1 -> bv
        let s_79_2: Bits = Bits::new(s_79_1 as u128, 64u16);
        // D s_79_3: read-var ga#10246:i64
        let s_79_3: i64 = fn_state.ga_10246;
        // D s_79_4: cast zx s_79_3 -> i
        let s_79_4: i128 = (i128::try_from(s_79_3).unwrap());
        // D s_79_5: bit-extract s_79_2 s_79_4 s_79_0
        let s_79_5: Bits = (Bits::new(
            ((s_79_2) >> (s_79_4)).value(),
            u16::try_from(s_79_0).unwrap(),
        ));
        // D s_79_6: cast reint s_79_5 -> u8
        let s_79_6: bool = ((s_79_5.value()) != 0);
        // D s_79_7: write-var walkparams.12 <= s_79_6
        fn_state.walkparams._12 = s_79_6;
        // N s_79_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_80_0: const #21s : i64
        let s_80_0: i64 = 21;
        // D s_80_1: write-var ga#10246 <= s_80_0
        fn_state.ga_10246 = s_80_0;
        // N s_80_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_81_0: const #12816u : u32
        let s_81_0: u32 = 12816;
        // D s_81_1: read-reg s_81_0:struct
        let s_81_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call _get_TCR_EL2_Type_HPD(s_81_1)
        let s_81_2: bool = u_get_TCR_EL2_Type_HPD(state, tracer, s_81_1);
        // D s_81_3: write-var walkparams.15 <= s_81_2
        fn_state.walkparams._15 = s_81_2;
        // N s_81_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_82_0: const #12816u : u32
        let s_82_0: u32 = 12816;
        // D s_82_1: read-reg s_82_0:struct
        let s_82_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call _get_TCR_EL2_Type_TBID(s_82_1)
        let s_82_2: bool = u_get_TCR_EL2_Type_TBID(state, tracer, s_82_1);
        // D s_82_3: write-var walkparams.35 <= s_82_2
        fn_state.walkparams._35 = s_82_2;
        // N s_82_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_83_0: const #90704u : u32
        let s_83_0: u32 = 90704;
        // D s_83_1: read-reg s_83_0:struct
        let s_83_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: call _get_SCR_EL3_Type_SIF(s_83_1)
        let s_83_2: bool = u_get_SCR_EL3_Type_SIF(state, tracer, s_83_1);
        // D s_83_3: write-var walkparams.30 <= s_83_2
        fn_state.walkparams._30 = s_83_2;
        // N s_83_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call HaveRME(s_84_0)
        let s_84_1: bool = HaveRME(state, tracer, s_84_0);
        // S s_84_2: not s_84_1
        let s_84_2: bool = !s_84_1;
        // N s_84_3: branch s_84_2 b87 b85
        if s_84_2 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_85_0: const #() : ()
        let s_85_0: () = ();
        // S s_85_1: call HaveSecureEL2Ext(s_85_0)
        let s_85_1: bool = HaveSecureEL2Ext(state, tracer, s_85_0);
        // D s_85_2: write-var gs#13726 <= s_85_1
        fn_state.gs_13726 = s_85_1;
        // N s_85_3: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_86_0: read-var gs#13726:u8
        let s_86_0: bool = fn_state.gs_13726;
        // D s_86_1: write-var gs#13727 <= s_86_0
        fn_state.gs_13727 = s_86_0;
        // N s_86_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_87_0: const #1u : u8
        let s_87_0: bool = true;
        // D s_87_1: write-var gs#13726 <= s_87_0
        fn_state.gs_13726 = s_87_0;
        // N s_87_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_88_0: const #15752u : u32
        let s_88_0: u32 = 15752;
        // D s_88_1: read-reg s_88_0:struct
        let s_88_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_88_0 as isize);
            tracer.read_register(s_88_0 as isize, value);
            value
        };
        // D s_88_2: call _get_TCR2_EL2_Type_AIE(s_88_1)
        let s_88_2: bool = u_get_TCR2_EL2_Type_AIE(state, tracer, s_88_1);
        // D s_88_3: write-var walkparams.0 <= s_88_2
        fn_state.walkparams._0 = s_88_2;
        // N s_88_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_89_0: const #() : ()
        let s_89_0: () = ();
        // S s_89_1: call IsTCR2EL2Enabled(s_89_0)
        let s_89_1: bool = IsTCR2EL2Enabled(state, tracer, s_89_0);
        // D s_89_2: write-var gs#13725 <= s_89_1
        fn_state.gs_13725 = s_89_1;
        // N s_89_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_90_0: const #20928u : u32
        let s_90_0: u32 = 20928;
        // D s_90_1: read-reg s_90_0:u64
        let s_90_1: u64 = {
            let value = state.read_register::<u64>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // D s_90_2: call Mk_MAIRType(s_90_1)
        let s_90_2: ProductType5c790c8ef59cc8b2 = Mk_MAIRType(state, tracer, s_90_1);
        // D s_90_3: write-var walkparams.18 <= s_90_2
        fn_state.walkparams._18 = s_90_2;
        // N s_90_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
