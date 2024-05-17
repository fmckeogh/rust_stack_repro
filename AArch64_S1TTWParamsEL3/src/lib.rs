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
use u_get_TCR_EL3_Type_PnCH::*;
use u_get_TCR_EL3_Type_MTX::*;
use u_get_TCR_EL3_Type_HAFT::*;
use HaveAccessFlagUpdateExt::*;
use u_get_TCR_EL3_Type_PIE::*;
use u_get_SCTLR_EL3_Type_EE::*;
use HaveRME::*;
use u_get_TCR_EL3_Type_DisCH0::*;
use u_get_SCTLR_EL3_Type_WXN::*;
use AArch64_HaveHPDExt::*;
use u_get_TCR_EL3_Type_D128::*;
use HaveS1PIExt::*;
use HaveMTE4Ext::*;
use u_get_SCTLR2_EL3_Type_EMEC::*;
use Have52BitIPAAndPASpaceExt::*;
use u_get_SCR_EL3_Type_SIF::*;
use HaveTHExt::*;
use u_get_TCR_EL3_Type_TBI::*;
use u_get_TCR_EL3_Type_TG0::*;
use HavePACExt::*;
use AArch64_S1DecodeTG0::*;
use HaveSecureEL2Ext::*;
use u_get_TCR_EL3_Type_T0SZ::*;
use u_get_TCR_EL3_Type_HPD::*;
use Mk_MAIRType::*;
use u_get_TCR_EL3_Type_SH0::*;
use u_get_TCR_EL3_Type_PS::*;
use HaveAIEExt::*;
use HaveDirtyBitModifierExt::*;
use HaveFeatMEC::*;
use u_get_TCR_EL3_Type_DS::*;
use u_get_TCR_EL3_Type_HD::*;
use u_get_TCR_EL3_Type_HA::*;
use Have128BitDescriptorExt::*;
use u_get_TTBR0_EL3_Type_SKL::*;
use HaveAccessFlagUpdateForTableExt::*;
use u_get_TCR_EL3_Type_TBID::*;
use u_get_TCR_EL3_Type_AIE::*;
use u_get_TCR_EL3_Type_ORGN0::*;
use u_get_TCR_EL3_Type_IRGN0::*;
use Mk_S1PIRType::*;
use common::*;
pub fn AArch64_S1TTWParamsEL3<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_13848: (),
) -> ProductTypeef284266e139aee2 {
    #[derive(Default)]
    struct FunctionState {
        walkparams: ProductTypeef284266e139aee2,
        gs_13853: bool,
        gs_13852: bool,
        gs_13849: bool,
        gs_13851: bool,
        gs_13850: bool,
        gs_13848: (),
    }
    let fn_state = FunctionState {
        gs_13848,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_0_0: const #10736u : u32
        let s_0_0: u32 = 10736;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_TCR_EL3_Type_TG0(s_0_1)
        let s_0_2: u8 = u_get_TCR_EL3_Type_TG0(state, tracer, s_0_1);
        // D s_0_3: call AArch64_S1DecodeTG0(s_0_2)
        let s_0_3: u32 = AArch64_S1DecodeTG0(state, tracer, s_0_2);
        // D s_0_4: write-var walkparams.36 <= s_0_3
        fn_state.walkparams._36 = s_0_3;
        // C s_0_5: const #10736u : u32
        let s_0_5: u32 = 10736;
        // D s_0_6: read-reg s_0_5:struct
        let s_0_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: call _get_TCR_EL3_Type_T0SZ(s_0_6)
        let s_0_7: u8 = u_get_TCR_EL3_Type_T0SZ(state, tracer, s_0_6);
        // D s_0_8: write-var walkparams.37 <= s_0_7
        fn_state.walkparams._37 = s_0_7;
        // C s_0_9: const #10736u : u32
        let s_0_9: u32 = 10736;
        // D s_0_10: read-reg s_0_9:struct
        let s_0_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: call _get_TCR_EL3_Type_PS(s_0_10)
        let s_0_11: u8 = u_get_TCR_EL3_Type_PS(state, tracer, s_0_10);
        // D s_0_12: write-var walkparams.28 <= s_0_11
        fn_state.walkparams._28 = s_0_11;
        // C s_0_13: const #10736u : u32
        let s_0_13: u32 = 10736;
        // D s_0_14: read-reg s_0_13:struct
        let s_0_14: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: call _get_TCR_EL3_Type_IRGN0(s_0_14)
        let s_0_15: u8 = u_get_TCR_EL3_Type_IRGN0(state, tracer, s_0_14);
        // D s_0_16: write-var walkparams.16 <= s_0_15
        fn_state.walkparams._16 = s_0_15;
        // C s_0_17: const #10736u : u32
        let s_0_17: u32 = 10736;
        // D s_0_18: read-reg s_0_17:struct
        let s_0_18: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: call _get_TCR_EL3_Type_ORGN0(s_0_18)
        let s_0_19: u8 = u_get_TCR_EL3_Type_ORGN0(state, tracer, s_0_18);
        // D s_0_20: write-var walkparams.23 <= s_0_19
        fn_state.walkparams._23 = s_0_19;
        // C s_0_21: const #10736u : u32
        let s_0_21: u32 = 10736;
        // D s_0_22: read-reg s_0_21:struct
        let s_0_22: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // D s_0_23: call _get_TCR_EL3_Type_SH0(s_0_22)
        let s_0_23: u8 = u_get_TCR_EL3_Type_SH0(state, tracer, s_0_22);
        // D s_0_24: write-var walkparams.29 <= s_0_23
        fn_state.walkparams._29 = s_0_23;
        // C s_0_25: const #10736u : u32
        let s_0_25: u32 = 10736;
        // D s_0_26: read-reg s_0_25:struct
        let s_0_26: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // D s_0_27: call _get_TCR_EL3_Type_TBI(s_0_26)
        let s_0_27: bool = u_get_TCR_EL3_Type_TBI(state, tracer, s_0_26);
        // D s_0_28: write-var walkparams.34 <= s_0_27
        fn_state.walkparams._34 = s_0_27;
        // C s_0_29: const #17656u : u32
        let s_0_29: u32 = 17656;
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
        // N s_0_35: branch s_0_34 b69 b1
        if s_0_34 {
            return block_69(state, tracer, fn_state);
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
        // N s_2_2: branch s_2_1 b68 b3
        if s_2_1 {
            return block_68(state, tracer, fn_state);
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
        // D s_3_1: write-var walkparams.0 <= s_3_0
        fn_state.walkparams._0 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_4_0: const #17072u : u32
        let s_4_0: u32 = 17072;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_SCTLR_EL3_Type_WXN(s_4_1)
        let s_4_2: bool = u_get_SCTLR_EL3_Type_WXN(state, tracer, s_4_1);
        // D s_4_3: write-var walkparams.39 <= s_4_2
        fn_state.walkparams._39 = s_4_2;
        // C s_4_4: const #17072u : u32
        let s_4_4: u32 = 17072;
        // D s_4_5: read-reg s_4_4:struct
        let s_4_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_4 as isize);
            tracer.read_register(s_4_4 as isize, value);
            value
        };
        // D s_4_6: call _get_SCTLR_EL3_Type_EE(s_4_5)
        let s_4_6: bool = u_get_SCTLR_EL3_Type_EE(state, tracer, s_4_5);
        // D s_4_7: write-var walkparams.9 <= s_4_6
        fn_state.walkparams._9 = s_4_6;
        // C s_4_8: const #() : ()
        let s_4_8: () = ();
        // S s_4_9: call HaveRME(s_4_8)
        let s_4_9: bool = HaveRME(state, tracer, s_4_8);
        // S s_4_10: not s_4_9
        let s_4_10: bool = !s_4_9;
        // N s_4_11: branch s_4_10 b67 b5
        if s_4_10 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call HaveSecureEL2Ext(s_5_0)
        let s_5_1: bool = HaveSecureEL2Ext(state, tracer, s_5_0);
        // D s_5_2: write-var gs#13849 <= s_5_1
        fn_state.gs_13849 = s_5_1;
        // N s_5_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_6_0: read-var gs#13849:u8
        let s_6_0: bool = fn_state.gs_13849;
        // N s_6_1: branch s_6_0 b66 b7
        if s_6_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_7_1: write-var walkparams.30 <= s_7_0
        fn_state.walkparams._30 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HavePACExt(s_8_0)
        let s_8_1: bool = HavePACExt(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b65 b9
        if s_8_1 {
            return block_65(state, tracer, fn_state);
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
        // D s_9_1: write-var walkparams.35 <= s_9_0
        fn_state.walkparams._35 = s_9_0;
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
        // S s_10_1: call AArch64_HaveHPDExt(s_10_0)
        let s_10_1: bool = AArch64_HaveHPDExt(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b64 b11
        if s_10_1 {
            return block_64(state, tracer, fn_state);
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
        // D s_11_1: write-var walkparams.15 <= s_11_0
        fn_state.walkparams._15 = s_11_0;
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
        // S s_12_1: call HaveAccessFlagUpdateExt(s_12_0)
        let s_12_1: bool = HaveAccessFlagUpdateExt(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b63 b13
        if s_12_1 {
            return block_63(state, tracer, fn_state);
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
        // D s_13_1: write-var walkparams.12 <= s_13_0
        fn_state.walkparams._12 = s_13_0;
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
        // S s_14_1: call HaveDirtyBitModifierExt(s_14_0)
        let s_14_1: bool = HaveDirtyBitModifierExt(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b62 b15
        if s_14_1 {
            return block_62(state, tracer, fn_state);
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
        // D s_15_1: write-var walkparams.14 <= s_15_0
        fn_state.walkparams._14 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_16_0: read-var walkparams.36:struct
        let s_16_0: u32 = fn_state.walkparams._36;
        // C s_16_1: const #0u : u32
        let s_16_1: u32 = 0;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // N s_16_3: branch s_16_2 b61 b17
        if s_16_2 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_17_0: read-var walkparams.36:struct
        let s_17_0: u32 = fn_state.walkparams._36;
        // C s_17_1: const #1u : u32
        let s_17_1: u32 = 1;
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // D s_17_3: write-var gs#13850 <= s_17_2
        fn_state.gs_13850 = s_17_2;
        // N s_17_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_18_0: read-var gs#13850:u8
        let s_18_0: bool = fn_state.gs_13850;
        // N s_18_1: branch s_18_0 b60 b19
        if s_18_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#13851 <= s_19_0
        fn_state.gs_13851 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_20_0: read-var gs#13851:u8
        let s_20_0: bool = fn_state.gs_13851;
        // N s_20_1: branch s_20_0 b59 b21
        if s_20_0 {
            return block_59(state, tracer, fn_state);
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
        // D s_21_1: write-var walkparams.7 <= s_21_0
        fn_state.walkparams._7 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call Have128BitDescriptorExt(s_22_0)
        let s_22_1: bool = Have128BitDescriptorExt(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b58 b23
        if s_22_1 {
            return block_58(state, tracer, fn_state);
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
        // D s_23_1: write-var walkparams.3 <= s_23_0
        fn_state.walkparams._3 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_24_0: read-var walkparams.3:struct
        let s_24_0: bool = fn_state.walkparams._3;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // N s_24_5: branch s_24_4 b57 b25
        if s_24_4 {
            return block_57(state, tracer, fn_state);
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
        let s_25_0: u8 = 0;
        // D s_25_1: write-var walkparams.31 <= s_25_0
        fn_state.walkparams._31 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_26_0: read-var walkparams.3:struct
        let s_26_0: bool = fn_state.walkparams._3;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // N s_26_5: branch s_26_4 b56 b27
        if s_26_4 {
            return block_56(state, tracer, fn_state);
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
        // D s_27_1: write-var walkparams.6 <= s_27_0
        fn_state.walkparams._6 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_28_0: read-var walkparams.3:struct
        let s_28_0: bool = fn_state.walkparams._3;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 1u16);
        // C s_28_2: const #1u : u8
        let s_28_2: bool = true;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // N s_28_5: branch s_28_4 b55 b29
        if s_28_4 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call HaveS1PIExt(s_29_0)
        let s_29_1: bool = HaveS1PIExt(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b54 b30
        if s_29_1 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var walkparams.24 <= s_30_0
        fn_state.walkparams._24 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call HaveS1PIExt(s_32_0)
        let s_32_1: bool = HaveS1PIExt(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b53 b33
        if s_32_1 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call HaveMTE4Ext(s_34_0)
        let s_34_1: bool = HaveMTE4Ext(state, tracer, s_34_0);
        // N s_34_2: branch s_34_1 b52 b35
        if s_34_1 {
            return block_52(state, tracer, fn_state);
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
        // D s_35_1: write-var walkparams.19 <= s_35_0
        fn_state.walkparams._19 = s_35_0;
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
        // S s_36_1: call HaveTHExt(s_36_0)
        let s_36_1: bool = HaveTHExt(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b51 b37
        if s_36_1 {
            return block_51(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#13852 <= s_37_0
        fn_state.gs_13852 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_38_0: read-var gs#13852:u8
        let s_38_0: bool = fn_state.gs_13852;
        // N s_38_1: branch s_38_0 b50 b39
        if s_38_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_39_1: write-var walkparams.27 <= s_39_0
        fn_state.walkparams._27 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call HaveAccessFlagUpdateForTableExt(s_40_0)
        let s_40_1: bool = HaveAccessFlagUpdateForTableExt(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b49 b41
        if s_40_1 {
            return block_49(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#13853 <= s_41_0
        fn_state.gs_13853 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_42_0: read-var gs#13853:u8
        let s_42_0: bool = fn_state.gs_13853;
        // N s_42_1: branch s_42_0 b48 b43
        if s_42_0 {
            return block_48(state, tracer, fn_state);
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
        // D s_43_1: write-var walkparams.13 <= s_43_0
        fn_state.walkparams._13 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call HaveFeatMEC(s_44_0)
        let s_44_1: bool = HaveFeatMEC(state, tracer, s_44_0);
        // N s_44_2: branch s_44_1 b47 b45
        if s_44_1 {
            return block_47(state, tracer, fn_state);
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
        // D s_46_0: read-var walkparams:struct
        let s_46_0: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // N s_46_1: return s_46_0
        return s_46_0;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_47_0: const #1464u : u32
        let s_47_0: u32 = 1464;
        // D s_47_1: read-reg s_47_0:struct
        let s_47_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call _get_SCTLR2_EL3_Type_EMEC(s_47_1)
        let s_47_2: bool = u_get_SCTLR2_EL3_Type_EMEC(state, tracer, s_47_1);
        // D s_47_3: write-var walkparams.10 <= s_47_2
        fn_state.walkparams._10 = s_47_2;
        // N s_47_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_48_0: const #10736u : u32
        let s_48_0: u32 = 10736;
        // D s_48_1: read-reg s_48_0:struct
        let s_48_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: call _get_TCR_EL3_Type_HAFT(s_48_1)
        let s_48_2: bool = u_get_TCR_EL3_Type_HAFT(state, tracer, s_48_1);
        // D s_48_3: write-var walkparams.13 <= s_48_2
        fn_state.walkparams._13 = s_48_2;
        // N s_48_4: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_49_0: read-var walkparams.12:struct
        let s_49_0: bool = fn_state.walkparams._12;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 1u16);
        // C s_49_2: const #1u : u8
        let s_49_2: bool = true;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // D s_49_5: write-var gs#13853 <= s_49_4
        fn_state.gs_13853 = s_49_4;
        // N s_49_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_50_0: const #10736u : u32
        let s_50_0: u32 = 10736;
        // D s_50_1: read-reg s_50_0:struct
        let s_50_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call _get_TCR_EL3_Type_PnCH(s_50_1)
        let s_50_2: bool = u_get_TCR_EL3_Type_PnCH(state, tracer, s_50_1);
        // D s_50_3: write-var walkparams.27 <= s_50_2
        fn_state.walkparams._27 = s_50_2;
        // N s_50_4: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_51_0: read-var walkparams.3:struct
        let s_51_0: bool = fn_state.walkparams._3;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #0u : u8
        let s_51_2: bool = false;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#13852 <= s_51_4
        fn_state.gs_13852 = s_51_4;
        // N s_51_6: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_52_0: const #10736u : u32
        let s_52_0: u32 = 10736;
        // D s_52_1: read-reg s_52_0:struct
        let s_52_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: call _get_TCR_EL3_Type_MTX(s_52_1)
        let s_52_2: bool = u_get_TCR_EL3_Type_MTX(state, tracer, s_52_1);
        // D s_52_3: write-var walkparams.19 <= s_52_2
        fn_state.walkparams._19 = s_52_2;
        // N s_52_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_53_0: const #12176u : u32
        let s_53_0: u32 = 12176;
        // D s_53_1: read-reg s_53_0:u64
        let s_53_1: u64 = {
            let value = state.read_register::<u64>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // D s_53_2: call Mk_S1PIRType(s_53_1)
        let s_53_2: ProductType5c790c8ef59cc8b2 = Mk_S1PIRType(state, tracer, s_53_1);
        // D s_53_3: write-var walkparams.25 <= s_53_2
        fn_state.walkparams._25 = s_53_2;
        // N s_53_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_54_0: const #10736u : u32
        let s_54_0: u32 = 10736;
        // D s_54_1: read-reg s_54_0:struct
        let s_54_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // D s_54_2: call _get_TCR_EL3_Type_PIE(s_54_1)
        let s_54_2: bool = u_get_TCR_EL3_Type_PIE(state, tracer, s_54_1);
        // D s_54_3: write-var walkparams.24 <= s_54_2
        fn_state.walkparams._24 = s_54_2;
        // N s_54_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var walkparams.24 <= s_55_0
        fn_state.walkparams._24 = s_55_0;
        // N s_55_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_56_0: const #10736u : u32
        let s_56_0: u32 = 10736;
        // D s_56_1: read-reg s_56_0:struct
        let s_56_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: call _get_TCR_EL3_Type_DisCH0(s_56_1)
        let s_56_2: bool = u_get_TCR_EL3_Type_DisCH0(state, tracer, s_56_1);
        // D s_56_3: write-var walkparams.6 <= s_56_2
        fn_state.walkparams._6 = s_56_2;
        // N s_56_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_57_0: const #23176u : u32
        let s_57_0: u32 = 23176;
        // D s_57_1: read-reg s_57_0:struct
        let s_57_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: call _get_TTBR0_EL3_Type_SKL(s_57_1)
        let s_57_2: u8 = u_get_TTBR0_EL3_Type_SKL(state, tracer, s_57_1);
        // D s_57_3: write-var walkparams.31 <= s_57_2
        fn_state.walkparams._31 = s_57_2;
        // N s_57_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_58_0: const #10736u : u32
        let s_58_0: u32 = 10736;
        // D s_58_1: read-reg s_58_0:struct
        let s_58_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call _get_TCR_EL3_Type_D128(s_58_1)
        let s_58_2: bool = u_get_TCR_EL3_Type_D128(state, tracer, s_58_1);
        // D s_58_3: write-var walkparams.3 <= s_58_2
        fn_state.walkparams._3 = s_58_2;
        // N s_58_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_59_0: const #10736u : u32
        let s_59_0: u32 = 10736;
        // D s_59_1: read-reg s_59_0:struct
        let s_59_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call _get_TCR_EL3_Type_DS(s_59_1)
        let s_59_2: bool = u_get_TCR_EL3_Type_DS(state, tracer, s_59_1);
        // D s_59_3: write-var walkparams.7 <= s_59_2
        fn_state.walkparams._7 = s_59_2;
        // N s_59_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call Have52BitIPAAndPASpaceExt(s_60_0)
        let s_60_1: bool = Have52BitIPAAndPASpaceExt(state, tracer, s_60_0);
        // D s_60_2: write-var gs#13851 <= s_60_1
        fn_state.gs_13851 = s_60_1;
        // N s_60_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#13850 <= s_61_0
        fn_state.gs_13850 = s_61_0;
        // N s_61_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_62_0: const #10736u : u32
        let s_62_0: u32 = 10736;
        // D s_62_1: read-reg s_62_0:struct
        let s_62_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call _get_TCR_EL3_Type_HD(s_62_1)
        let s_62_2: bool = u_get_TCR_EL3_Type_HD(state, tracer, s_62_1);
        // D s_62_3: write-var walkparams.14 <= s_62_2
        fn_state.walkparams._14 = s_62_2;
        // N s_62_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_63_0: const #10736u : u32
        let s_63_0: u32 = 10736;
        // D s_63_1: read-reg s_63_0:struct
        let s_63_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call _get_TCR_EL3_Type_HA(s_63_1)
        let s_63_2: bool = u_get_TCR_EL3_Type_HA(state, tracer, s_63_1);
        // D s_63_3: write-var walkparams.12 <= s_63_2
        fn_state.walkparams._12 = s_63_2;
        // N s_63_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_64_0: const #10736u : u32
        let s_64_0: u32 = 10736;
        // D s_64_1: read-reg s_64_0:struct
        let s_64_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call _get_TCR_EL3_Type_HPD(s_64_1)
        let s_64_2: bool = u_get_TCR_EL3_Type_HPD(state, tracer, s_64_1);
        // D s_64_3: write-var walkparams.15 <= s_64_2
        fn_state.walkparams._15 = s_64_2;
        // N s_64_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_65_0: const #10736u : u32
        let s_65_0: u32 = 10736;
        // D s_65_1: read-reg s_65_0:struct
        let s_65_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // D s_65_2: call _get_TCR_EL3_Type_TBID(s_65_1)
        let s_65_2: bool = u_get_TCR_EL3_Type_TBID(state, tracer, s_65_1);
        // D s_65_3: write-var walkparams.35 <= s_65_2
        fn_state.walkparams._35 = s_65_2;
        // N s_65_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_66_0: const #90704u : u32
        let s_66_0: u32 = 90704;
        // D s_66_1: read-reg s_66_0:struct
        let s_66_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_0 as isize);
            tracer.read_register(s_66_0 as isize, value);
            value
        };
        // D s_66_2: call _get_SCR_EL3_Type_SIF(s_66_1)
        let s_66_2: bool = u_get_SCR_EL3_Type_SIF(state, tracer, s_66_1);
        // D s_66_3: write-var walkparams.30 <= s_66_2
        fn_state.walkparams._30 = s_66_2;
        // N s_66_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#13849 <= s_67_0
        fn_state.gs_13849 = s_67_0;
        // N s_67_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_68_0: const #10736u : u32
        let s_68_0: u32 = 10736;
        // D s_68_1: read-reg s_68_0:struct
        let s_68_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: call _get_TCR_EL3_Type_AIE(s_68_1)
        let s_68_2: bool = u_get_TCR_EL3_Type_AIE(state, tracer, s_68_1);
        // D s_68_3: write-var walkparams.0 <= s_68_2
        fn_state.walkparams._0 = s_68_2;
        // N s_68_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_69_0: const #19104u : u32
        let s_69_0: u32 = 19104;
        // D s_69_1: read-reg s_69_0:u64
        let s_69_1: u64 = {
            let value = state.read_register::<u64>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: call Mk_MAIRType(s_69_1)
        let s_69_2: ProductType5c790c8ef59cc8b2 = Mk_MAIRType(state, tracer, s_69_1);
        // D s_69_3: write-var walkparams.18 <= s_69_2
        fn_state.walkparams._18 = s_69_2;
        // N s_69_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
