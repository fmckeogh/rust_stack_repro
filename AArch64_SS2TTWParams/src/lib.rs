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
use u_get_VTCR_EL2_Type_ORGN0::*;
use u_get_HCR_EL2_Type_FWB::*;
use HaveAccessFlagUpdateExt::*;
use u_get_VTTBR_EL2_Type_SKL::*;
use u_get_VSTCR_EL2_Type_SA::*;
use HaveFeatCMOW::*;
use u_get_VSTCR_EL2_Type_SW::*;
use u_get_VTCR_EL2_Type_NSW::*;
use u_get_VTCR_EL2_Type_SH0::*;
use Zeros::*;
use HaveS2PIExt::*;
use u_get_VTCR_EL2_Type_HAFT::*;
use u_get_VTCR_EL2_Type_HD::*;
use u_get_VTCR_EL2_Type_T0SZ::*;
use VTTBR_EL2_read::*;
use u_get_VSTCR_EL2_Type_SL0::*;
use u_get_VTCR_EL2_Type_S2PIE::*;
use IsHCRXEL2Enabled::*;
use Have52BitIPAAndPASpaceExt::*;
use u_get_VTCR_EL2_Type_D128::*;
use u_get_VTCR_EL2_Type_TG0::*;
use HaveTHExt::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCR_EL2_Type_VM::*;
use u_get_VTCR_EL2_Type_PS::*;
use u_get_VTCR_EL2_Type_NSA::*;
use u_get_HCRX_EL2_Type_CMOW::*;
use u_get_VSTCR_EL2_Type_TG0::*;
use u_get_VTCR_EL2_Type_SL2::*;
use u_get_VTCR_EL2_Type_TL0::*;
use u_get_SCTLR_EL2_Type_EE::*;
use u_get_VTCR_EL2_Type_AssuredOnly::*;
use HaveDirtyBitModifierExt::*;
use u_get_VSTCR_EL2_Type_T0SZ::*;
use u_get_VSTCR_EL2_Type_SL2::*;
use Unreachable::*;
use u_get_VSTTBR_EL2_Type_SKL::*;
use u_get_VTCR_EL2_Type_TL1::*;
use u_get_HCR_EL2_Type_DC::*;
use Have128BitDescriptorExt::*;
use u_get_VTCR_EL2_Type_HA::*;
use HaveAccessFlagUpdateForTableExt::*;
use AArch64_S2DecodeTG0::*;
use u_get_VTCR_EL2_Type_IRGN0::*;
use u_get_VTCR_EL2_Type_SL0::*;
use Mk_S2PIRType::*;
use u_get_VTCR_EL2_Type_DS::*;
use u_get_HCR_EL2_Type_PTW::*;
use HaveStage2MemAttrControl::*;
use common::*;
pub fn AArch64_SS2TTWParams<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ipaspace: u32,
    s1aarch64: bool,
) -> ProductTypeb05ce25a107f0c5e {
    #[derive(Default)]
    struct FunctionState {
        gs_18359: bool,
        gs_18322: bool,
        gs_18355: bool,
        gs_18327: bool,
        walkparams: ProductTypeb05ce25a107f0c5e,
        gs_18326: bool,
        gs_18323: bool,
        gs_18324: bool,
        ipaspace: u32,
        s1aarch64: bool,
    }
    let fn_state = FunctionState {
        ipaspace,
        s1aarch64,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call Have128BitDescriptorExt(s_0_0)
        let s_0_1: bool = Have128BitDescriptorExt(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b78 b1
        if s_0_1 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var walkparams.2 <= s_1_0
        fn_state.walkparams._2 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_2_0: read-var ipaspace:u32
        let s_2_0: u32 = fn_state.ipaspace;
        // C s_2_1: const #1u : u32
        let s_2_1: u32 = 1;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b69 b3
        if s_2_2 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_3_0: read-var ipaspace:u32
        let s_3_0: u32 = fn_state.ipaspace;
        // C s_3_1: const #0u : u32
        let s_3_1: u32 = 0;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b60 b4
        if s_3_2 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call Unreachable(s_4_0)
        let s_4_1: () = Unreachable(state, tracer, s_4_0);
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_5_0: const #11168u : u32
        let s_5_0: u32 = 11168;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_VSTCR_EL2_Type_SW(s_5_1)
        let s_5_2: bool = u_get_VSTCR_EL2_Type_SW(state, tracer, s_5_1);
        // D s_5_3: write-var walkparams.24 <= s_5_2
        fn_state.walkparams._24 = s_5_2;
        // C s_5_4: const #15328u : u32
        let s_5_4: u32 = 15328;
        // D s_5_5: read-reg s_5_4:struct
        let s_5_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_4 as isize);
            tracer.read_register(s_5_4 as isize, value);
            value
        };
        // D s_5_6: call _get_VTCR_EL2_Type_NSW(s_5_5)
        let s_5_6: bool = u_get_VTCR_EL2_Type_NSW(state, tracer, s_5_5);
        // D s_5_7: write-var walkparams.12 <= s_5_6
        fn_state.walkparams._12 = s_5_6;
        // C s_5_8: const #11168u : u32
        let s_5_8: u32 = 11168;
        // D s_5_9: read-reg s_5_8:struct
        let s_5_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_8 as isize);
            tracer.read_register(s_5_8 as isize, value);
            value
        };
        // D s_5_10: call _get_VSTCR_EL2_Type_SA(s_5_9)
        let s_5_10: bool = u_get_VSTCR_EL2_Type_SA(state, tracer, s_5_9);
        // D s_5_11: write-var walkparams.19 <= s_5_10
        fn_state.walkparams._19 = s_5_10;
        // C s_5_12: const #15328u : u32
        let s_5_12: u32 = 15328;
        // D s_5_13: read-reg s_5_12:struct
        let s_5_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_12 as isize);
            tracer.read_register(s_5_12 as isize, value);
            value
        };
        // D s_5_14: call _get_VTCR_EL2_Type_NSA(s_5_13)
        let s_5_14: bool = u_get_VTCR_EL2_Type_NSA(state, tracer, s_5_13);
        // D s_5_15: write-var walkparams.11 <= s_5_14
        fn_state.walkparams._11 = s_5_14;
        // C s_5_16: const #102552u : u32
        let s_5_16: u32 = 102552;
        // D s_5_17: read-reg s_5_16:struct
        let s_5_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_16 as isize);
            tracer.read_register(s_5_16 as isize, value);
            value
        };
        // D s_5_18: call _get_HCR_EL2_Type_VM(s_5_17)
        let s_5_18: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_5_17);
        // C s_5_19: const #102552u : u32
        let s_5_19: u32 = 102552;
        // D s_5_20: read-reg s_5_19:struct
        let s_5_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_19 as isize);
            tracer.read_register(s_5_19 as isize, value);
            value
        };
        // D s_5_21: call _get_HCR_EL2_Type_DC(s_5_20)
        let s_5_21: bool = u_get_HCR_EL2_Type_DC(state, tracer, s_5_20);
        // D s_5_22: cast zx s_5_18 -> bv
        let s_5_22: Bits = Bits::new(s_5_18 as u128, 1u16);
        // D s_5_23: cast zx s_5_21 -> bv
        let s_5_23: Bits = Bits::new(s_5_21 as u128, 1u16);
        // D s_5_24: or s_5_22 s_5_23
        let s_5_24: Bits = ((s_5_22) | (s_5_23));
        // D s_5_25: cast reint s_5_24 -> u8
        let s_5_25: bool = ((s_5_24.value()) != 0);
        // D s_5_26: write-var walkparams.30 <= s_5_25
        fn_state.walkparams._30 = s_5_25;
        // C s_5_27: const #15328u : u32
        let s_5_27: u32 = 15328;
        // D s_5_28: read-reg s_5_27:struct
        let s_5_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_27 as isize);
            tracer.read_register(s_5_27 as isize, value);
            value
        };
        // D s_5_29: call _get_VTCR_EL2_Type_PS(s_5_28)
        let s_5_29: u8 = u_get_VTCR_EL2_Type_PS(state, tracer, s_5_28);
        // D s_5_30: write-var walkparams.14 <= s_5_29
        fn_state.walkparams._14 = s_5_29;
        // C s_5_31: const #15328u : u32
        let s_5_31: u32 = 15328;
        // D s_5_32: read-reg s_5_31:struct
        let s_5_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_31 as isize);
            tracer.read_register(s_5_31 as isize, value);
            value
        };
        // D s_5_33: call _get_VTCR_EL2_Type_IRGN0(s_5_32)
        let s_5_33: u8 = u_get_VTCR_EL2_Type_IRGN0(state, tracer, s_5_32);
        // D s_5_34: write-var walkparams.10 <= s_5_33
        fn_state.walkparams._10 = s_5_33;
        // C s_5_35: const #15328u : u32
        let s_5_35: u32 = 15328;
        // D s_5_36: read-reg s_5_35:struct
        let s_5_36: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_35 as isize);
            tracer.read_register(s_5_35 as isize, value);
            value
        };
        // D s_5_37: call _get_VTCR_EL2_Type_ORGN0(s_5_36)
        let s_5_37: u8 = u_get_VTCR_EL2_Type_ORGN0(state, tracer, s_5_36);
        // D s_5_38: write-var walkparams.13 <= s_5_37
        fn_state.walkparams._13 = s_5_37;
        // C s_5_39: const #15328u : u32
        let s_5_39: u32 = 15328;
        // D s_5_40: read-reg s_5_39:struct
        let s_5_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_39 as isize);
            tracer.read_register(s_5_39 as isize, value);
            value
        };
        // D s_5_41: call _get_VTCR_EL2_Type_SH0(s_5_40)
        let s_5_41: u8 = u_get_VTCR_EL2_Type_SH0(state, tracer, s_5_40);
        // D s_5_42: write-var walkparams.20 <= s_5_41
        fn_state.walkparams._20 = s_5_41;
        // C s_5_43: const #20784u : u32
        let s_5_43: u32 = 20784;
        // D s_5_44: read-reg s_5_43:struct
        let s_5_44: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_43 as isize);
            tracer.read_register(s_5_43 as isize, value);
            value
        };
        // D s_5_45: call _get_SCTLR_EL2_Type_EE(s_5_44)
        let s_5_45: bool = u_get_SCTLR_EL2_Type_EE(state, tracer, s_5_44);
        // D s_5_46: write-var walkparams.4 <= s_5_45
        fn_state.walkparams._4 = s_5_45;
        // C s_5_47: const #102552u : u32
        let s_5_47: u32 = 102552;
        // D s_5_48: read-reg s_5_47:struct
        let s_5_48: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_47 as isize);
            tracer.read_register(s_5_47 as isize, value);
            value
        };
        // D s_5_49: call _get_HCR_EL2_Type_TGE(s_5_48)
        let s_5_49: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_5_48);
        // D s_5_50: cast zx s_5_49 -> bv
        let s_5_50: Bits = Bits::new(s_5_49 as u128, 1u16);
        // C s_5_51: const #0u : u8
        let s_5_51: bool = false;
        // C s_5_52: cast zx s_5_51 -> bv
        let s_5_52: Bits = Bits::new(s_5_51 as u128, 1u16);
        // D s_5_53: cmp-eq s_5_50 s_5_52
        let s_5_53: bool = ((s_5_50) == (s_5_52));
        // N s_5_54: branch s_5_53 b59 b6
        if s_5_53 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var walkparams.15 <= s_6_0
        fn_state.walkparams._15 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call HaveStage2MemAttrControl(s_7_0)
        let s_7_1: bool = HaveStage2MemAttrControl(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b58 b8
        if s_7_1 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var walkparams.6 <= s_8_0
        fn_state.walkparams._6 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call HaveAccessFlagUpdateExt(s_9_0)
        let s_9_1: bool = HaveAccessFlagUpdateExt(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b57 b10
        if s_9_1 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var walkparams.7 <= s_10_0
        fn_state.walkparams._7 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HaveDirtyBitModifierExt(s_11_0)
        let s_11_1: bool = HaveDirtyBitModifierExt(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b56 b12
        if s_11_1 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var walkparams.9 <= s_12_0
        fn_state.walkparams._9 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_13_0: read-var walkparams.26:struct
        let s_13_0: u32 = fn_state.walkparams._26;
        // C s_13_1: const #0u : u32
        let s_13_1: u32 = 0;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b55 b14
        if s_13_2 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_14_0: read-var walkparams.26:struct
        let s_14_0: u32 = fn_state.walkparams._26;
        // C s_14_1: const #1u : u32
        let s_14_1: u32 = 1;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // D s_14_3: write-var gs#18322 <= s_14_2
        fn_state.gs_18322 = s_14_2;
        // N s_14_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_15_0: read-var gs#18322:u8
        let s_15_0: bool = fn_state.gs_18322;
        // N s_15_1: branch s_15_0 b54 b16
        if s_15_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#18323 <= s_16_0
        fn_state.gs_18323 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_17_0: read-var gs#18323:u8
        let s_17_0: bool = fn_state.gs_18323;
        // N s_17_1: branch s_17_0 b53 b18
        if s_17_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var walkparams.3 <= s_18_0
        fn_state.walkparams._3 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call HaveFeatCMOW(s_19_0)
        let s_19_1: bool = HaveFeatCMOW(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b52 b20
        if s_19_1 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#18324 <= s_20_0
        fn_state.gs_18324 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_21_0: read-var gs#18324:u8
        let s_21_0: bool = fn_state.gs_18324;
        // N s_21_1: branch s_21_0 b51 b22
        if s_21_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var walkparams.1 <= s_22_0
        fn_state.walkparams._1 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_23_0: read-var walkparams.2:struct
        let s_23_0: bool = fn_state.walkparams._2;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // C s_23_2: const #1u : u8
        let s_23_2: bool = true;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // N s_23_5: branch s_23_4 b50 b24
        if s_23_4 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HaveS2PIExt(s_24_0)
        let s_24_1: bool = HaveS2PIExt(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b49 b25
        if s_24_1 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var walkparams.17 <= s_25_0
        fn_state.walkparams._17 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call HaveS2PIExt(s_27_0)
        let s_27_1: bool = HaveS2PIExt(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b48 b28
        if s_27_1 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_28_0: const #64s : i
        let s_28_0: i128 = 64;
        // S s_28_1: call Zeros(s_28_0)
        let s_28_1: Bits = Zeros(state, tracer, s_28_0);
        // S s_28_2: cast reint s_28_1 -> u64
        let s_28_2: u64 = (s_28_1.value() as u64);
        // S s_28_3: call Mk_S2PIRType(s_28_2)
        let s_28_3: ProductType5c790c8ef59cc8b2 = Mk_S2PIRType(state, tracer, s_28_2);
        // D s_28_4: write-var walkparams.18 <= s_28_3
        fn_state.walkparams._18 = s_28_3;
        // N s_28_5: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call HaveTHExt(s_29_0)
        let s_29_1: bool = HaveTHExt(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b47 b30
        if s_29_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#18326 <= s_30_0
        fn_state.gs_18326 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_31_0: read-var gs#18326:u8
        let s_31_0: bool = fn_state.gs_18326;
        // N s_31_1: branch s_31_0 b46 b32
        if s_31_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var walkparams.0 <= s_32_0
        fn_state.walkparams._0 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call HaveTHExt(s_33_0)
        let s_33_1: bool = HaveTHExt(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b45 b34
        if s_33_1 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var walkparams.27 <= s_34_0
        fn_state.walkparams._27 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call HaveTHExt(s_35_0)
        let s_35_1: bool = HaveTHExt(state, tracer, s_35_0);
        // N s_35_2: branch s_35_1 b44 b36
        if s_35_1 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var walkparams.28 <= s_36_0
        fn_state.walkparams._28 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call HaveAccessFlagUpdateForTableExt(s_37_0)
        let s_37_1: bool = HaveAccessFlagUpdateForTableExt(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b43 b38
        if s_37_1 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#18327 <= s_38_0
        fn_state.gs_18327 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_39_0: read-var gs#18327:u8
        let s_39_0: bool = fn_state.gs_18327;
        // N s_39_1: branch s_39_0 b42 b40
        if s_39_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var walkparams.8 <= s_40_0
        fn_state.walkparams._8 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var walkparams.5 <= s_41_0
        fn_state.walkparams._5 = s_41_0;
        // D s_41_2: read-var walkparams:struct
        let s_41_2: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // N s_41_3: return s_41_2
        return s_41_2;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_42_0: const #15328u : u32
        let s_42_0: u32 = 15328;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_VTCR_EL2_Type_HAFT(s_42_1)
        let s_42_2: bool = u_get_VTCR_EL2_Type_HAFT(state, tracer, s_42_1);
        // D s_42_3: write-var walkparams.8 <= s_42_2
        fn_state.walkparams._8 = s_42_2;
        // N s_42_4: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_43_0: read-var walkparams.7:struct
        let s_43_0: bool = fn_state.walkparams._7;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 1u16);
        // C s_43_2: const #1u : u8
        let s_43_2: bool = true;
        // C s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: write-var gs#18327 <= s_43_4
        fn_state.gs_18327 = s_43_4;
        // N s_43_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_44_0: const #15328u : u32
        let s_44_0: u32 = 15328;
        // D s_44_1: read-reg s_44_0:struct
        let s_44_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call _get_VTCR_EL2_Type_TL1(s_44_1)
        let s_44_2: bool = u_get_VTCR_EL2_Type_TL1(state, tracer, s_44_1);
        // D s_44_3: write-var walkparams.28 <= s_44_2
        fn_state.walkparams._28 = s_44_2;
        // N s_44_4: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_45_0: const #15328u : u32
        let s_45_0: u32 = 15328;
        // D s_45_1: read-reg s_45_0:struct
        let s_45_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call _get_VTCR_EL2_Type_TL0(s_45_1)
        let s_45_2: bool = u_get_VTCR_EL2_Type_TL0(state, tracer, s_45_1);
        // D s_45_3: write-var walkparams.27 <= s_45_2
        fn_state.walkparams._27 = s_45_2;
        // N s_45_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_46_0: const #15328u : u32
        let s_46_0: u32 = 15328;
        // D s_46_1: read-reg s_46_0:struct
        let s_46_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call _get_VTCR_EL2_Type_AssuredOnly(s_46_1)
        let s_46_2: bool = u_get_VTCR_EL2_Type_AssuredOnly(state, tracer, s_46_1);
        // D s_46_3: write-var walkparams.0 <= s_46_2
        fn_state.walkparams._0 = s_46_2;
        // N s_46_4: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_47_0: read-var walkparams.2:struct
        let s_47_0: bool = fn_state.walkparams._2;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 1u16);
        // C s_47_2: const #1u : u8
        let s_47_2: bool = true;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // D s_47_4: cmp-ne s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) != (s_47_3));
        // D s_47_5: write-var gs#18326 <= s_47_4
        fn_state.gs_18326 = s_47_4;
        // N s_47_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_48_0: const #11936u : u32
        let s_48_0: u32 = 11936;
        // D s_48_1: read-reg s_48_0:u64
        let s_48_1: u64 = {
            let value = state.read_register::<u64>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: call Mk_S2PIRType(s_48_1)
        let s_48_2: ProductType5c790c8ef59cc8b2 = Mk_S2PIRType(state, tracer, s_48_1);
        // D s_48_3: write-var walkparams.18 <= s_48_2
        fn_state.walkparams._18 = s_48_2;
        // N s_48_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_49_0: const #15328u : u32
        let s_49_0: u32 = 15328;
        // D s_49_1: read-reg s_49_0:struct
        let s_49_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: call _get_VTCR_EL2_Type_S2PIE(s_49_1)
        let s_49_2: bool = u_get_VTCR_EL2_Type_S2PIE(state, tracer, s_49_1);
        // D s_49_3: write-var walkparams.17 <= s_49_2
        fn_state.walkparams._17 = s_49_2;
        // N s_49_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_50_0: const #1u : u8
        let s_50_0: bool = true;
        // D s_50_1: write-var walkparams.17 <= s_50_0
        fn_state.walkparams._17 = s_50_0;
        // N s_50_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_51_0: const #22528u : u32
        let s_51_0: u32 = 22528;
        // D s_51_1: read-reg s_51_0:struct
        let s_51_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call _get_HCRX_EL2_Type_CMOW(s_51_1)
        let s_51_2: bool = u_get_HCRX_EL2_Type_CMOW(state, tracer, s_51_1);
        // D s_51_3: write-var walkparams.1 <= s_51_2
        fn_state.walkparams._1 = s_51_2;
        // N s_51_4: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call IsHCRXEL2Enabled(s_52_0)
        let s_52_1: bool = IsHCRXEL2Enabled(state, tracer, s_52_0);
        // D s_52_2: write-var gs#18324 <= s_52_1
        fn_state.gs_18324 = s_52_1;
        // N s_52_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_53_0: const #15328u : u32
        let s_53_0: u32 = 15328;
        // D s_53_1: read-reg s_53_0:struct
        let s_53_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // D s_53_2: call _get_VTCR_EL2_Type_DS(s_53_1)
        let s_53_2: bool = u_get_VTCR_EL2_Type_DS(state, tracer, s_53_1);
        // D s_53_3: write-var walkparams.3 <= s_53_2
        fn_state.walkparams._3 = s_53_2;
        // N s_53_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call Have52BitIPAAndPASpaceExt(s_54_0)
        let s_54_1: bool = Have52BitIPAAndPASpaceExt(state, tracer, s_54_0);
        // D s_54_2: write-var gs#18323 <= s_54_1
        fn_state.gs_18323 = s_54_1;
        // N s_54_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#18322 <= s_55_0
        fn_state.gs_18322 = s_55_0;
        // N s_55_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_56_0: const #15328u : u32
        let s_56_0: u32 = 15328;
        // D s_56_1: read-reg s_56_0:struct
        let s_56_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: call _get_VTCR_EL2_Type_HD(s_56_1)
        let s_56_2: bool = u_get_VTCR_EL2_Type_HD(state, tracer, s_56_1);
        // D s_56_3: write-var walkparams.9 <= s_56_2
        fn_state.walkparams._9 = s_56_2;
        // N s_56_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_57_0: const #15328u : u32
        let s_57_0: u32 = 15328;
        // D s_57_1: read-reg s_57_0:struct
        let s_57_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: call _get_VTCR_EL2_Type_HA(s_57_1)
        let s_57_2: bool = u_get_VTCR_EL2_Type_HA(state, tracer, s_57_1);
        // D s_57_3: write-var walkparams.7 <= s_57_2
        fn_state.walkparams._7 = s_57_2;
        // N s_57_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_58_0: const #102552u : u32
        let s_58_0: u32 = 102552;
        // D s_58_1: read-reg s_58_0:struct
        let s_58_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call _get_HCR_EL2_Type_FWB(s_58_1)
        let s_58_2: bool = u_get_HCR_EL2_Type_FWB(state, tracer, s_58_1);
        // D s_58_3: write-var walkparams.6 <= s_58_2
        fn_state.walkparams._6 = s_58_2;
        // N s_58_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_59_0: const #102552u : u32
        let s_59_0: u32 = 102552;
        // D s_59_1: read-reg s_59_0:struct
        let s_59_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call _get_HCR_EL2_Type_PTW(s_59_1)
        let s_59_2: bool = u_get_HCR_EL2_Type_PTW(state, tracer, s_59_1);
        // D s_59_3: write-var walkparams.15 <= s_59_2
        fn_state.walkparams._15 = s_59_2;
        // N s_59_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_60_0: const #15328u : u32
        let s_60_0: u32 = 15328;
        // D s_60_1: read-reg s_60_0:struct
        let s_60_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call _get_VTCR_EL2_Type_TG0(s_60_1)
        let s_60_2: u8 = u_get_VTCR_EL2_Type_TG0(state, tracer, s_60_1);
        // D s_60_3: call AArch64_S2DecodeTG0(s_60_2)
        let s_60_3: u32 = AArch64_S2DecodeTG0(state, tracer, s_60_2);
        // D s_60_4: write-var walkparams.26 <= s_60_3
        fn_state.walkparams._26 = s_60_3;
        // C s_60_5: const #15328u : u32
        let s_60_5: u32 = 15328;
        // D s_60_6: read-reg s_60_5:struct
        let s_60_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_5 as isize);
            tracer.read_register(s_60_5 as isize, value);
            value
        };
        // D s_60_7: call _get_VTCR_EL2_Type_T0SZ(s_60_6)
        let s_60_7: u8 = u_get_VTCR_EL2_Type_T0SZ(state, tracer, s_60_6);
        // D s_60_8: write-var walkparams.29 <= s_60_7
        fn_state.walkparams._29 = s_60_7;
        // D s_60_9: read-var walkparams.2:struct
        let s_60_9: bool = fn_state.walkparams._2;
        // D s_60_10: cast zx s_60_9 -> bv
        let s_60_10: Bits = Bits::new(s_60_9 as u128, 1u16);
        // C s_60_11: const #1u : u8
        let s_60_11: bool = true;
        // C s_60_12: cast zx s_60_11 -> bv
        let s_60_12: Bits = Bits::new(s_60_11 as u128, 1u16);
        // D s_60_13: cmp-eq s_60_10 s_60_12
        let s_60_13: bool = ((s_60_10) == (s_60_12));
        // N s_60_14: branch s_60_13 b68 b61
        if s_60_13 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_61_0: const #15328u : u32
        let s_61_0: u32 = 15328;
        // D s_61_1: read-reg s_61_0:struct
        let s_61_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call _get_VTCR_EL2_Type_SL0(s_61_1)
        let s_61_2: u8 = u_get_VTCR_EL2_Type_SL0(state, tracer, s_61_1);
        // D s_61_3: write-var walkparams.22 <= s_61_2
        fn_state.walkparams._22 = s_61_2;
        // N s_61_4: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_62_0: read-var walkparams.26:struct
        let s_62_0: u32 = fn_state.walkparams._26;
        // C s_62_1: const #0u : u32
        let s_62_1: u32 = 0;
        // D s_62_2: cmp-eq s_62_0 s_62_1
        let s_62_2: bool = ((s_62_0) == (s_62_1));
        // N s_62_3: branch s_62_2 b67 b63
        if s_62_2 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#18355 <= s_63_0
        fn_state.gs_18355 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_64_0: read-var gs#18355:u8
        let s_64_0: bool = fn_state.gs_18355;
        // N s_64_1: branch s_64_0 b66 b65
        if s_64_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var walkparams.23 <= s_65_0
        fn_state.walkparams._23 = s_65_0;
        // N s_65_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_66_0: const #15328u : u32
        let s_66_0: u32 = 15328;
        // D s_66_1: read-reg s_66_0:struct
        let s_66_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_0 as isize);
            tracer.read_register(s_66_0 as isize, value);
            value
        };
        // D s_66_2: call _get_VTCR_EL2_Type_SL2(s_66_1)
        let s_66_2: bool = u_get_VTCR_EL2_Type_SL2(state, tracer, s_66_1);
        // C s_66_3: const #15328u : u32
        let s_66_3: u32 = 15328;
        // D s_66_4: read-reg s_66_3:struct
        let s_66_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_3 as isize);
            tracer.read_register(s_66_3 as isize, value);
            value
        };
        // D s_66_5: call _get_VTCR_EL2_Type_DS(s_66_4)
        let s_66_5: bool = u_get_VTCR_EL2_Type_DS(state, tracer, s_66_4);
        // D s_66_6: cast zx s_66_2 -> bv
        let s_66_6: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_7: cast zx s_66_5 -> bv
        let s_66_7: Bits = Bits::new(s_66_5 as u128, 1u16);
        // D s_66_8: and s_66_6 s_66_7
        let s_66_8: Bits = ((s_66_6) & (s_66_7));
        // D s_66_9: cast reint s_66_8 -> u8
        let s_66_9: bool = ((s_66_8.value()) != 0);
        // D s_66_10: write-var walkparams.23 <= s_66_9
        fn_state.walkparams._23 = s_66_9;
        // N s_66_11: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call Have52BitIPAAndPASpaceExt(s_67_0)
        let s_67_1: bool = Have52BitIPAAndPASpaceExt(state, tracer, s_67_0);
        // D s_67_2: write-var gs#18355 <= s_67_1
        fn_state.gs_18355 = s_67_1;
        // N s_67_3: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_68_0: const #() : ()
        let s_68_0: () = ();
        // S s_68_1: call VTTBR_EL2_read(s_68_0)
        let s_68_1: ProductType782ac6922b48c20d = VTTBR_EL2_read(state, tracer, s_68_0);
        // S s_68_2: call _get_VTTBR_EL2_Type_SKL(s_68_1)
        let s_68_2: u8 = u_get_VTTBR_EL2_Type_SKL(state, tracer, s_68_1);
        // D s_68_3: write-var walkparams.21 <= s_68_2
        fn_state.walkparams._21 = s_68_2;
        // N s_68_4: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_69_0: const #11168u : u32
        let s_69_0: u32 = 11168;
        // D s_69_1: read-reg s_69_0:struct
        let s_69_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: call _get_VSTCR_EL2_Type_TG0(s_69_1)
        let s_69_2: u8 = u_get_VSTCR_EL2_Type_TG0(state, tracer, s_69_1);
        // D s_69_3: call AArch64_S2DecodeTG0(s_69_2)
        let s_69_3: u32 = AArch64_S2DecodeTG0(state, tracer, s_69_2);
        // D s_69_4: write-var walkparams.26 <= s_69_3
        fn_state.walkparams._26 = s_69_3;
        // C s_69_5: const #11168u : u32
        let s_69_5: u32 = 11168;
        // D s_69_6: read-reg s_69_5:struct
        let s_69_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_5 as isize);
            tracer.read_register(s_69_5 as isize, value);
            value
        };
        // D s_69_7: call _get_VSTCR_EL2_Type_T0SZ(s_69_6)
        let s_69_7: u8 = u_get_VSTCR_EL2_Type_T0SZ(state, tracer, s_69_6);
        // D s_69_8: write-var walkparams.29 <= s_69_7
        fn_state.walkparams._29 = s_69_7;
        // D s_69_9: read-var walkparams.2:struct
        let s_69_9: bool = fn_state.walkparams._2;
        // D s_69_10: cast zx s_69_9 -> bv
        let s_69_10: Bits = Bits::new(s_69_9 as u128, 1u16);
        // C s_69_11: const #1u : u8
        let s_69_11: bool = true;
        // C s_69_12: cast zx s_69_11 -> bv
        let s_69_12: Bits = Bits::new(s_69_11 as u128, 1u16);
        // D s_69_13: cmp-eq s_69_10 s_69_12
        let s_69_13: bool = ((s_69_10) == (s_69_12));
        // N s_69_14: branch s_69_13 b77 b70
        if s_69_13 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_70_0: const #11168u : u32
        let s_70_0: u32 = 11168;
        // D s_70_1: read-reg s_70_0:struct
        let s_70_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: call _get_VSTCR_EL2_Type_SL0(s_70_1)
        let s_70_2: u8 = u_get_VSTCR_EL2_Type_SL0(state, tracer, s_70_1);
        // D s_70_3: write-var walkparams.22 <= s_70_2
        fn_state.walkparams._22 = s_70_2;
        // N s_70_4: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_71_0: read-var walkparams.26:struct
        let s_71_0: u32 = fn_state.walkparams._26;
        // C s_71_1: const #0u : u32
        let s_71_1: u32 = 0;
        // D s_71_2: cmp-eq s_71_0 s_71_1
        let s_71_2: bool = ((s_71_0) == (s_71_1));
        // N s_71_3: branch s_71_2 b76 b72
        if s_71_2 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var gs#18359 <= s_72_0
        fn_state.gs_18359 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_73_0: read-var gs#18359:u8
        let s_73_0: bool = fn_state.gs_18359;
        // N s_73_1: branch s_73_0 b75 b74
        if s_73_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var walkparams.23 <= s_74_0
        fn_state.walkparams._23 = s_74_0;
        // N s_74_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_75_0: const #11168u : u32
        let s_75_0: u32 = 11168;
        // D s_75_1: read-reg s_75_0:struct
        let s_75_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // D s_75_2: call _get_VSTCR_EL2_Type_SL2(s_75_1)
        let s_75_2: bool = u_get_VSTCR_EL2_Type_SL2(state, tracer, s_75_1);
        // C s_75_3: const #15328u : u32
        let s_75_3: u32 = 15328;
        // D s_75_4: read-reg s_75_3:struct
        let s_75_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_75_3 as isize);
            tracer.read_register(s_75_3 as isize, value);
            value
        };
        // D s_75_5: call _get_VTCR_EL2_Type_DS(s_75_4)
        let s_75_5: bool = u_get_VTCR_EL2_Type_DS(state, tracer, s_75_4);
        // D s_75_6: cast zx s_75_2 -> bv
        let s_75_6: Bits = Bits::new(s_75_2 as u128, 1u16);
        // D s_75_7: cast zx s_75_5 -> bv
        let s_75_7: Bits = Bits::new(s_75_5 as u128, 1u16);
        // D s_75_8: and s_75_6 s_75_7
        let s_75_8: Bits = ((s_75_6) & (s_75_7));
        // D s_75_9: cast reint s_75_8 -> u8
        let s_75_9: bool = ((s_75_8.value()) != 0);
        // D s_75_10: write-var walkparams.23 <= s_75_9
        fn_state.walkparams._23 = s_75_9;
        // N s_75_11: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call Have52BitIPAAndPASpaceExt(s_76_0)
        let s_76_1: bool = Have52BitIPAAndPASpaceExt(state, tracer, s_76_0);
        // D s_76_2: write-var gs#18359 <= s_76_1
        fn_state.gs_18359 = s_76_1;
        // N s_76_3: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_77_0: const #22824u : u32
        let s_77_0: u32 = 22824;
        // D s_77_1: read-reg s_77_0:struct
        let s_77_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call _get_VSTTBR_EL2_Type_SKL(s_77_1)
        let s_77_2: u8 = u_get_VSTTBR_EL2_Type_SKL(state, tracer, s_77_1);
        // D s_77_3: write-var walkparams.21 <= s_77_2
        fn_state.walkparams._21 = s_77_2;
        // N s_77_4: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_78_0: const #15328u : u32
        let s_78_0: u32 = 15328;
        // D s_78_1: read-reg s_78_0:struct
        let s_78_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call _get_VTCR_EL2_Type_D128(s_78_1)
        let s_78_2: bool = u_get_VTCR_EL2_Type_D128(state, tracer, s_78_1);
        // D s_78_3: write-var walkparams.2 <= s_78_2
        fn_state.walkparams._2 = s_78_2;
        // N s_78_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
