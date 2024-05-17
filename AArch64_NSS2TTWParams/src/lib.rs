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
use HaveFeatCMOW::*;
use u_get_VTCR_EL2_Type_SH0::*;
use Zeros::*;
use HaveS2PIExt::*;
use u_get_VTCR_EL2_Type_HAFT::*;
use u_get_VTCR_EL2_Type_T0SZ::*;
use u_get_VTCR_EL2_Type_HD::*;
use VTTBR_EL2_read::*;
use u_get_VTCR_EL2_Type_S2PIE::*;
use IsHCRXEL2Enabled::*;
use Have52BitIPAAndPASpaceExt::*;
use u_get_VTCR_EL2_Type_D128::*;
use u_get_VTCR_EL2_Type_TG0::*;
use HaveTHExt::*;
use u_get_HCR_EL2_Type_VM::*;
use u_get_HCRX_EL2_Type_CMOW::*;
use u_get_VTCR_EL2_Type_PS::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_VTCR_EL2_Type_SL2::*;
use u_get_VTCR_EL2_Type_TL0::*;
use u_get_SCTLR_EL2_Type_EE::*;
use u_get_VTCR_EL2_Type_AssuredOnly::*;
use HaveDirtyBitModifierExt::*;
use u_get_VTCR_EL2_Type_TL1::*;
use u_get_HCR_EL2_Type_DC::*;
use Have128BitDescriptorExt::*;
use AArch64_S2DecodeTG0::*;
use u_get_VTCR_EL2_Type_SL0::*;
use HaveAccessFlagUpdateForTableExt::*;
use u_get_VTCR_EL2_Type_IRGN0::*;
use u_get_VTCR_EL2_Type_HA::*;
use Mk_S2PIRType::*;
use u_get_VTCR_EL2_Type_DS::*;
use u_get_HCR_EL2_Type_PTW::*;
use HaveStage2MemAttrControl::*;
use common::*;
pub fn AArch64_NSS2TTWParams<T: Tracer>(
    state: &mut State,
    tracer: &T,
    s1aarch64: bool,
) -> ProductTypeb05ce25a107f0c5e {
    #[derive(Default)]
    struct FunctionState {
        walkparams: ProductTypeb05ce25a107f0c5e,
        gs_18276: bool,
        gs_18277: bool,
        gs_18279: bool,
        gs_18275: bool,
        gs_18280: bool,
        gs_18274: bool,
        s1aarch64: bool,
    }
    let fn_state = FunctionState {
        s1aarch64,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_0_0: const #102552u : u32
        let s_0_0: u32 = 102552;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_HCR_EL2_Type_VM(s_0_1)
        let s_0_2: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_0_1);
        // C s_0_3: const #102552u : u32
        let s_0_3: u32 = 102552;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HCR_EL2_Type_DC(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_DC(state, tracer, s_0_4);
        // D s_0_6: cast zx s_0_2 -> bv
        let s_0_6: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_7: cast zx s_0_5 -> bv
        let s_0_7: Bits = Bits::new(s_0_5 as u128, 1u16);
        // D s_0_8: or s_0_6 s_0_7
        let s_0_8: Bits = ((s_0_6) | (s_0_7));
        // D s_0_9: cast reint s_0_8 -> u8
        let s_0_9: bool = ((s_0_8.value()) != 0);
        // D s_0_10: write-var walkparams.30 <= s_0_9
        fn_state.walkparams._30 = s_0_9;
        // C s_0_11: const #15328u : u32
        let s_0_11: u32 = 15328;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_VTCR_EL2_Type_TG0(s_0_12)
        let s_0_13: u8 = u_get_VTCR_EL2_Type_TG0(state, tracer, s_0_12);
        // D s_0_14: call AArch64_S2DecodeTG0(s_0_13)
        let s_0_14: u32 = AArch64_S2DecodeTG0(state, tracer, s_0_13);
        // D s_0_15: write-var walkparams.26 <= s_0_14
        fn_state.walkparams._26 = s_0_14;
        // C s_0_16: const #15328u : u32
        let s_0_16: u32 = 15328;
        // D s_0_17: read-reg s_0_16:struct
        let s_0_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_16 as isize);
            tracer.read_register(s_0_16 as isize, value);
            value
        };
        // D s_0_18: call _get_VTCR_EL2_Type_T0SZ(s_0_17)
        let s_0_18: u8 = u_get_VTCR_EL2_Type_T0SZ(state, tracer, s_0_17);
        // D s_0_19: write-var walkparams.29 <= s_0_18
        fn_state.walkparams._29 = s_0_18;
        // C s_0_20: const #15328u : u32
        let s_0_20: u32 = 15328;
        // D s_0_21: read-reg s_0_20:struct
        let s_0_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_20 as isize);
            tracer.read_register(s_0_20 as isize, value);
            value
        };
        // D s_0_22: call _get_VTCR_EL2_Type_PS(s_0_21)
        let s_0_22: u8 = u_get_VTCR_EL2_Type_PS(state, tracer, s_0_21);
        // D s_0_23: write-var walkparams.14 <= s_0_22
        fn_state.walkparams._14 = s_0_22;
        // C s_0_24: const #15328u : u32
        let s_0_24: u32 = 15328;
        // D s_0_25: read-reg s_0_24:struct
        let s_0_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_24 as isize);
            tracer.read_register(s_0_24 as isize, value);
            value
        };
        // D s_0_26: call _get_VTCR_EL2_Type_IRGN0(s_0_25)
        let s_0_26: u8 = u_get_VTCR_EL2_Type_IRGN0(state, tracer, s_0_25);
        // D s_0_27: write-var walkparams.10 <= s_0_26
        fn_state.walkparams._10 = s_0_26;
        // C s_0_28: const #15328u : u32
        let s_0_28: u32 = 15328;
        // D s_0_29: read-reg s_0_28:struct
        let s_0_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_28 as isize);
            tracer.read_register(s_0_28 as isize, value);
            value
        };
        // D s_0_30: call _get_VTCR_EL2_Type_ORGN0(s_0_29)
        let s_0_30: u8 = u_get_VTCR_EL2_Type_ORGN0(state, tracer, s_0_29);
        // D s_0_31: write-var walkparams.13 <= s_0_30
        fn_state.walkparams._13 = s_0_30;
        // C s_0_32: const #15328u : u32
        let s_0_32: u32 = 15328;
        // D s_0_33: read-reg s_0_32:struct
        let s_0_33: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_32 as isize);
            tracer.read_register(s_0_32 as isize, value);
            value
        };
        // D s_0_34: call _get_VTCR_EL2_Type_SH0(s_0_33)
        let s_0_34: u8 = u_get_VTCR_EL2_Type_SH0(state, tracer, s_0_33);
        // D s_0_35: write-var walkparams.20 <= s_0_34
        fn_state.walkparams._20 = s_0_34;
        // C s_0_36: const #20784u : u32
        let s_0_36: u32 = 20784;
        // D s_0_37: read-reg s_0_36:struct
        let s_0_37: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_36 as isize);
            tracer.read_register(s_0_36 as isize, value);
            value
        };
        // D s_0_38: call _get_SCTLR_EL2_Type_EE(s_0_37)
        let s_0_38: bool = u_get_SCTLR_EL2_Type_EE(state, tracer, s_0_37);
        // D s_0_39: write-var walkparams.4 <= s_0_38
        fn_state.walkparams._4 = s_0_38;
        // C s_0_40: const #() : ()
        let s_0_40: () = ();
        // S s_0_41: call Have128BitDescriptorExt(s_0_40)
        let s_0_41: bool = Have128BitDescriptorExt(state, tracer, s_0_40);
        // N s_0_42: branch s_0_41 b66 b1
        if s_0_41 {
            return block_66(state, tracer, fn_state);
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
        // D s_2_0: read-var walkparams.2:struct
        let s_2_0: bool = fn_state.walkparams._2;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b65 b3
        if s_2_4 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_3_0: const #15328u : u32
        let s_3_0: u32 = 15328;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call _get_VTCR_EL2_Type_SL0(s_3_1)
        let s_3_2: u8 = u_get_VTCR_EL2_Type_SL0(state, tracer, s_3_1);
        // D s_3_3: write-var walkparams.22 <= s_3_2
        fn_state.walkparams._22 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_4_0: const #102552u : u32
        let s_4_0: u32 = 102552;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_HCR_EL2_Type_TGE(s_4_1)
        let s_4_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_4_1);
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // C s_4_4: const #0u : u8
        let s_4_4: bool = false;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // D s_4_6: cmp-eq s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) == (s_4_5));
        // N s_4_7: branch s_4_6 b64 b5
        if s_4_6 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var walkparams.15 <= s_5_0
        fn_state.walkparams._15 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HaveStage2MemAttrControl(s_6_0)
        let s_6_1: bool = HaveStage2MemAttrControl(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b63 b7
        if s_6_1 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var walkparams.6 <= s_7_0
        fn_state.walkparams._6 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HaveAccessFlagUpdateExt(s_8_0)
        let s_8_1: bool = HaveAccessFlagUpdateExt(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b62 b9
        if s_8_1 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var walkparams.7 <= s_9_0
        fn_state.walkparams._7 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call HaveDirtyBitModifierExt(s_10_0)
        let s_10_1: bool = HaveDirtyBitModifierExt(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b61 b11
        if s_10_1 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var walkparams.9 <= s_11_0
        fn_state.walkparams._9 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_12_0: read-var walkparams.26:struct
        let s_12_0: u32 = fn_state.walkparams._26;
        // C s_12_1: const #0u : u32
        let s_12_1: u32 = 0;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // N s_12_3: branch s_12_2 b60 b13
        if s_12_2 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_13_0: read-var walkparams.26:struct
        let s_13_0: u32 = fn_state.walkparams._26;
        // C s_13_1: const #1u : u32
        let s_13_1: u32 = 1;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: write-var gs#18274 <= s_13_2
        fn_state.gs_18274 = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_14_0: read-var gs#18274:u8
        let s_14_0: bool = fn_state.gs_18274;
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
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#18275 <= s_15_0
        fn_state.gs_18275 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_16_0: read-var gs#18275:u8
        let s_16_0: bool = fn_state.gs_18275;
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
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var walkparams.3 <= s_17_0
        fn_state.walkparams._3 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_18_0: read-var walkparams.26:struct
        let s_18_0: u32 = fn_state.walkparams._26;
        // C s_18_1: const #0u : u32
        let s_18_1: u32 = 0;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // N s_18_3: branch s_18_2 b57 b19
        if s_18_2 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#18276 <= s_19_0
        fn_state.gs_18276 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_20_0: read-var gs#18276:u8
        let s_20_0: bool = fn_state.gs_18276;
        // N s_20_1: branch s_20_0 b56 b21
        if s_20_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var walkparams.23 <= s_21_0
        fn_state.walkparams._23 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call HaveFeatCMOW(s_22_0)
        let s_22_1: bool = HaveFeatCMOW(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b55 b23
        if s_22_1 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#18277 <= s_23_0
        fn_state.gs_18277 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_24_0: read-var gs#18277:u8
        let s_24_0: bool = fn_state.gs_18277;
        // N s_24_1: branch s_24_0 b54 b25
        if s_24_0 {
            return block_54(state, tracer, fn_state);
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
        // D s_25_1: write-var walkparams.1 <= s_25_0
        fn_state.walkparams._1 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_26_0: read-var walkparams.2:struct
        let s_26_0: bool = fn_state.walkparams._2;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // N s_26_5: branch s_26_4 b53 b27
        if s_26_4 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
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
        // N s_27_2: branch s_27_1 b52 b28
        if s_27_1 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var walkparams.17 <= s_28_0
        fn_state.walkparams._17 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call HaveS2PIExt(s_30_0)
        let s_30_1: bool = HaveS2PIExt(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b51 b31
        if s_30_1 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_31_0: const #64s : i
        let s_31_0: i128 = 64;
        // S s_31_1: call Zeros(s_31_0)
        let s_31_1: Bits = Zeros(state, tracer, s_31_0);
        // S s_31_2: cast reint s_31_1 -> u64
        let s_31_2: u64 = (s_31_1.value() as u64);
        // S s_31_3: call Mk_S2PIRType(s_31_2)
        let s_31_3: ProductType5c790c8ef59cc8b2 = Mk_S2PIRType(state, tracer, s_31_2);
        // D s_31_4: write-var walkparams.18 <= s_31_3
        fn_state.walkparams._18 = s_31_3;
        // N s_31_5: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call HaveTHExt(s_32_0)
        let s_32_1: bool = HaveTHExt(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b50 b33
        if s_32_1 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#18279 <= s_33_0
        fn_state.gs_18279 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_34_0: read-var gs#18279:u8
        let s_34_0: bool = fn_state.gs_18279;
        // N s_34_1: branch s_34_0 b49 b35
        if s_34_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var walkparams.0 <= s_35_0
        fn_state.walkparams._0 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call HaveTHExt(s_36_0)
        let s_36_1: bool = HaveTHExt(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b48 b37
        if s_36_1 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var walkparams.27 <= s_37_0
        fn_state.walkparams._27 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call HaveTHExt(s_38_0)
        let s_38_1: bool = HaveTHExt(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b47 b39
        if s_38_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var walkparams.28 <= s_39_0
        fn_state.walkparams._28 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call HaveAccessFlagUpdateForTableExt(s_40_0)
        let s_40_1: bool = HaveAccessFlagUpdateForTableExt(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b46 b41
        if s_40_1 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#18280 <= s_41_0
        fn_state.gs_18280 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_42_0: read-var gs#18280:u8
        let s_42_0: bool = fn_state.gs_18280;
        // N s_42_1: branch s_42_0 b45 b43
        if s_42_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var walkparams.8 <= s_43_0
        fn_state.walkparams._8 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_44_0: read-var walkparams:struct
        let s_44_0: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // N s_44_1: return s_44_0
        return s_44_0;
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
        // D s_45_2: call _get_VTCR_EL2_Type_HAFT(s_45_1)
        let s_45_2: bool = u_get_VTCR_EL2_Type_HAFT(state, tracer, s_45_1);
        // D s_45_3: write-var walkparams.8 <= s_45_2
        fn_state.walkparams._8 = s_45_2;
        // N s_45_4: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_46_0: read-var walkparams.7:struct
        let s_46_0: bool = fn_state.walkparams._7;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#18280 <= s_46_4
        fn_state.gs_18280 = s_46_4;
        // N s_46_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_47_0: const #15328u : u32
        let s_47_0: u32 = 15328;
        // D s_47_1: read-reg s_47_0:struct
        let s_47_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call _get_VTCR_EL2_Type_TL1(s_47_1)
        let s_47_2: bool = u_get_VTCR_EL2_Type_TL1(state, tracer, s_47_1);
        // D s_47_3: write-var walkparams.28 <= s_47_2
        fn_state.walkparams._28 = s_47_2;
        // N s_47_4: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_48_0: const #15328u : u32
        let s_48_0: u32 = 15328;
        // D s_48_1: read-reg s_48_0:struct
        let s_48_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: call _get_VTCR_EL2_Type_TL0(s_48_1)
        let s_48_2: bool = u_get_VTCR_EL2_Type_TL0(state, tracer, s_48_1);
        // D s_48_3: write-var walkparams.27 <= s_48_2
        fn_state.walkparams._27 = s_48_2;
        // N s_48_4: jump b38
        return block_38(state, tracer, fn_state);
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
        // D s_49_2: call _get_VTCR_EL2_Type_AssuredOnly(s_49_1)
        let s_49_2: bool = u_get_VTCR_EL2_Type_AssuredOnly(state, tracer, s_49_1);
        // D s_49_3: write-var walkparams.0 <= s_49_2
        fn_state.walkparams._0 = s_49_2;
        // N s_49_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_50_0: read-var walkparams.2:struct
        let s_50_0: bool = fn_state.walkparams._2;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 1u16);
        // C s_50_2: const #1u : u8
        let s_50_2: bool = true;
        // C s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // D s_50_4: cmp-ne s_50_1 s_50_3
        let s_50_4: bool = ((s_50_1) != (s_50_3));
        // D s_50_5: write-var gs#18279 <= s_50_4
        fn_state.gs_18279 = s_50_4;
        // N s_50_6: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_51_0: const #11936u : u32
        let s_51_0: u32 = 11936;
        // D s_51_1: read-reg s_51_0:u64
        let s_51_1: u64 = {
            let value = state.read_register::<u64>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call Mk_S2PIRType(s_51_1)
        let s_51_2: ProductType5c790c8ef59cc8b2 = Mk_S2PIRType(state, tracer, s_51_1);
        // D s_51_3: write-var walkparams.18 <= s_51_2
        fn_state.walkparams._18 = s_51_2;
        // N s_51_4: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_52_0: const #15328u : u32
        let s_52_0: u32 = 15328;
        // D s_52_1: read-reg s_52_0:struct
        let s_52_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: call _get_VTCR_EL2_Type_S2PIE(s_52_1)
        let s_52_2: bool = u_get_VTCR_EL2_Type_S2PIE(state, tracer, s_52_1);
        // D s_52_3: write-var walkparams.17 <= s_52_2
        fn_state.walkparams._17 = s_52_2;
        // N s_52_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var walkparams.17 <= s_53_0
        fn_state.walkparams._17 = s_53_0;
        // N s_53_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_54_0: const #22528u : u32
        let s_54_0: u32 = 22528;
        // D s_54_1: read-reg s_54_0:struct
        let s_54_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // D s_54_2: call _get_HCRX_EL2_Type_CMOW(s_54_1)
        let s_54_2: bool = u_get_HCRX_EL2_Type_CMOW(state, tracer, s_54_1);
        // D s_54_3: write-var walkparams.1 <= s_54_2
        fn_state.walkparams._1 = s_54_2;
        // N s_54_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call IsHCRXEL2Enabled(s_55_0)
        let s_55_1: bool = IsHCRXEL2Enabled(state, tracer, s_55_0);
        // D s_55_2: write-var gs#18277 <= s_55_1
        fn_state.gs_18277 = s_55_1;
        // N s_55_3: jump b24
        return block_24(state, tracer, fn_state);
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
        // D s_56_2: call _get_VTCR_EL2_Type_SL2(s_56_1)
        let s_56_2: bool = u_get_VTCR_EL2_Type_SL2(state, tracer, s_56_1);
        // C s_56_3: const #15328u : u32
        let s_56_3: u32 = 15328;
        // D s_56_4: read-reg s_56_3:struct
        let s_56_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_3 as isize);
            tracer.read_register(s_56_3 as isize, value);
            value
        };
        // D s_56_5: call _get_VTCR_EL2_Type_DS(s_56_4)
        let s_56_5: bool = u_get_VTCR_EL2_Type_DS(state, tracer, s_56_4);
        // D s_56_6: cast zx s_56_2 -> bv
        let s_56_6: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_7: cast zx s_56_5 -> bv
        let s_56_7: Bits = Bits::new(s_56_5 as u128, 1u16);
        // D s_56_8: and s_56_6 s_56_7
        let s_56_8: Bits = ((s_56_6) & (s_56_7));
        // D s_56_9: cast reint s_56_8 -> u8
        let s_56_9: bool = ((s_56_8.value()) != 0);
        // D s_56_10: write-var walkparams.23 <= s_56_9
        fn_state.walkparams._23 = s_56_9;
        // N s_56_11: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call Have52BitIPAAndPASpaceExt(s_57_0)
        let s_57_1: bool = Have52BitIPAAndPASpaceExt(state, tracer, s_57_0);
        // D s_57_2: write-var gs#18276 <= s_57_1
        fn_state.gs_18276 = s_57_1;
        // N s_57_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_58_0: const #15328u : u32
        let s_58_0: u32 = 15328;
        // D s_58_1: read-reg s_58_0:struct
        let s_58_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call _get_VTCR_EL2_Type_DS(s_58_1)
        let s_58_2: bool = u_get_VTCR_EL2_Type_DS(state, tracer, s_58_1);
        // D s_58_3: write-var walkparams.3 <= s_58_2
        fn_state.walkparams._3 = s_58_2;
        // N s_58_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call Have52BitIPAAndPASpaceExt(s_59_0)
        let s_59_1: bool = Have52BitIPAAndPASpaceExt(state, tracer, s_59_0);
        // D s_59_2: write-var gs#18275 <= s_59_1
        fn_state.gs_18275 = s_59_1;
        // N s_59_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#18274 <= s_60_0
        fn_state.gs_18274 = s_60_0;
        // N s_60_2: jump b14
        return block_14(state, tracer, fn_state);
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
        // D s_61_2: call _get_VTCR_EL2_Type_HD(s_61_1)
        let s_61_2: bool = u_get_VTCR_EL2_Type_HD(state, tracer, s_61_1);
        // D s_61_3: write-var walkparams.9 <= s_61_2
        fn_state.walkparams._9 = s_61_2;
        // N s_61_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_62_0: const #15328u : u32
        let s_62_0: u32 = 15328;
        // D s_62_1: read-reg s_62_0:struct
        let s_62_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call _get_VTCR_EL2_Type_HA(s_62_1)
        let s_62_2: bool = u_get_VTCR_EL2_Type_HA(state, tracer, s_62_1);
        // D s_62_3: write-var walkparams.7 <= s_62_2
        fn_state.walkparams._7 = s_62_2;
        // N s_62_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_63_0: const #102552u : u32
        let s_63_0: u32 = 102552;
        // D s_63_1: read-reg s_63_0:struct
        let s_63_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call _get_HCR_EL2_Type_FWB(s_63_1)
        let s_63_2: bool = u_get_HCR_EL2_Type_FWB(state, tracer, s_63_1);
        // D s_63_3: write-var walkparams.6 <= s_63_2
        fn_state.walkparams._6 = s_63_2;
        // N s_63_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_64_0: const #102552u : u32
        let s_64_0: u32 = 102552;
        // D s_64_1: read-reg s_64_0:struct
        let s_64_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call _get_HCR_EL2_Type_PTW(s_64_1)
        let s_64_2: bool = u_get_HCR_EL2_Type_PTW(state, tracer, s_64_1);
        // D s_64_3: write-var walkparams.15 <= s_64_2
        fn_state.walkparams._15 = s_64_2;
        // N s_64_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call VTTBR_EL2_read(s_65_0)
        let s_65_1: ProductType782ac6922b48c20d = VTTBR_EL2_read(state, tracer, s_65_0);
        // S s_65_2: call _get_VTTBR_EL2_Type_SKL(s_65_1)
        let s_65_2: u8 = u_get_VTTBR_EL2_Type_SKL(state, tracer, s_65_1);
        // D s_65_3: write-var walkparams.21 <= s_65_2
        fn_state.walkparams._21 = s_65_2;
        // N s_65_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // D s_66_2: call _get_VTCR_EL2_Type_D128(s_66_1)
        let s_66_2: bool = u_get_VTCR_EL2_Type_D128(state, tracer, s_66_1);
        // D s_66_3: write-var walkparams.2 <= s_66_2
        fn_state.walkparams._2 = s_66_2;
        // N s_66_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
