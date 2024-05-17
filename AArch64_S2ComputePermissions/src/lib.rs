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
use AArch64_S2IndirectBasePermissions::*;
use AArch64_S2OverlayPermissions::*;
use u_get_VTCR_EL2_Type_S2POE::*;
use AArch64_S2DirectBasePermissions::*;
use HaveS2POExt::*;
use common::*;
pub fn AArch64_S2ComputePermissions<T: Tracer>(
    state: &mut State,
    tracer: &T,
    permissions: ProductTypebf05c51f33174538,
    walkparams: ProductTypeb05ce25a107f0c5e,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductType2fc9d3588999ac79 {
    #[derive(Default)]
    struct FunctionState {
        s2overlay_permsshadow_306: ProductType2fc9d3588999ac79,
        s2perms: ProductType2fc9d3588999ac79,
        gs_18769: bool,
        permissions: ProductTypebf05c51f33174538,
        walkparams: ProductTypeb05ce25a107f0c5e,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        permissions,
        walkparams,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_0_0: read-var walkparams.17:struct
        let s_0_0: bool = fn_state.walkparams._17;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // C s_0_2: const #1u : u8
        let s_0_2: bool = true;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_1_0: read-var permissions:struct
        let s_1_0: ProductTypebf05c51f33174538 = fn_state.permissions;
        // D s_1_1: read-var accdesc:struct
        let s_1_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_1_2: call AArch64_S2DirectBasePermissions(s_1_0, s_1_1)
        let s_1_2: ProductType2fc9d3588999ac79 = AArch64_S2DirectBasePermissions(
            state,
            tracer,
            s_1_0,
            s_1_1,
        );
        // D s_1_3: write-var s2perms <= s_1_2
        fn_state.s2perms = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_2_0: read-var s2perms:struct
        let s_2_0: ProductType2fc9d3588999ac79 = fn_state.s2perms;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_3_0: read-var permissions:struct
        let s_3_0: ProductTypebf05c51f33174538 = fn_state.permissions;
        // D s_3_1: read-var accdesc:struct
        let s_3_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_3_2: call AArch64_S2IndirectBasePermissions(s_3_0, s_3_1)
        let s_3_2: ProductType2fc9d3588999ac79 = AArch64_S2IndirectBasePermissions(
            state,
            tracer,
            s_3_0,
            s_3_1,
        );
        // D s_3_3: write-var s2perms <= s_3_2
        fn_state.s2perms = s_3_2;
        // C s_3_4: const #() : ()
        let s_3_4: () = ();
        // S s_3_5: call HaveS2POExt(s_3_4)
        let s_3_5: bool = HaveS2POExt(state, tracer, s_3_4);
        // N s_3_6: branch s_3_5 b11 b4
        if s_3_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#18769 <= s_4_0
        fn_state.gs_18769 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_5_0: read-var gs#18769:u8
        let s_5_0: bool = fn_state.gs_18769;
        // D s_5_1: write-var s2perms.3 <= s_5_0
        fn_state.s2perms._3 = s_5_0;
        // D s_5_2: read-var s2perms.3:struct
        let s_5_2: bool = fn_state.s2perms._3;
        // N s_5_3: branch s_5_2 b8 b6
        if s_5_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // N s_7_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_8_0: read-var permissions:struct
        let s_8_0: ProductTypebf05c51f33174538 = fn_state.permissions;
        // D s_8_1: read-var accdesc:struct
        let s_8_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_2: call AArch64_S2OverlayPermissions(s_8_0, s_8_1)
        let s_8_2: ProductType2fc9d3588999ac79 = AArch64_S2OverlayPermissions(
            state,
            tracer,
            s_8_0,
            s_8_1,
        );
        // D s_8_3: write-var s2overlay_permsshadow#306 <= s_8_2
        fn_state.s2overlay_permsshadow_306 = s_8_2;
        // D s_8_4: read-var s2overlay_permsshadow#306.0:struct
        let s_8_4: bool = fn_state.s2overlay_permsshadow_306._0;
        // D s_8_5: write-var s2perms.0 <= s_8_4
        fn_state.s2perms._0 = s_8_4;
        // D s_8_6: read-var s2overlay_permsshadow#306.4:struct
        let s_8_6: bool = fn_state.s2overlay_permsshadow_306._4;
        // D s_8_7: write-var s2perms.4 <= s_8_6
        fn_state.s2perms._4 = s_8_6;
        // D s_8_8: read-var s2overlay_permsshadow#306.7:struct
        let s_8_8: bool = fn_state.s2overlay_permsshadow_306._7;
        // D s_8_9: write-var s2perms.7 <= s_8_8
        fn_state.s2perms._7 = s_8_8;
        // D s_8_10: read-var s2overlay_permsshadow#306.2:struct
        let s_8_10: bool = fn_state.s2overlay_permsshadow_306._2;
        // D s_8_11: write-var s2perms.2 <= s_8_10
        fn_state.s2perms._2 = s_8_10;
        // D s_8_12: read-var s2overlay_permsshadow#306.6:struct
        let s_8_12: bool = fn_state.s2overlay_permsshadow_306._6;
        // D s_8_13: write-var s2perms.6 <= s_8_12
        fn_state.s2perms._6 = s_8_12;
        // D s_8_14: read-var s2overlay_permsshadow#306.1:struct
        let s_8_14: bool = fn_state.s2overlay_permsshadow_306._1;
        // D s_8_15: write-var s2perms.1 <= s_8_14
        fn_state.s2perms._1 = s_8_14;
        // D s_8_16: read-var s2overlay_permsshadow#306.5:struct
        let s_8_16: bool = fn_state.s2overlay_permsshadow_306._5;
        // D s_8_17: write-var s2perms.5 <= s_8_16
        fn_state.s2perms._5 = s_8_16;
        // D s_8_18: read-var s2perms.13:struct
        let s_8_18: bool = fn_state.s2perms._13;
        // D s_8_19: read-var s2perms.15:struct
        let s_8_19: bool = fn_state.s2perms._15;
        // D s_8_20: read-var s2perms.14:struct
        let s_8_20: bool = fn_state.s2perms._14;
        // D s_8_21: cast zx s_8_19 -> bv
        let s_8_21: Bits = Bits::new(s_8_19 as u128, 1u16);
        // D s_8_22: cast zx s_8_20 -> bv
        let s_8_22: Bits = Bits::new(s_8_20 as u128, 1u16);
        // D s_8_23: cast reint s_8_21 -> u128
        let s_8_23: u128 = (s_8_21.value() as u128);
        // D s_8_24: size-of s_8_21
        let s_8_24: u16 = s_8_21.length();
        // D s_8_25: cast reint s_8_22 -> u128
        let s_8_25: u128 = (s_8_22.value() as u128);
        // D s_8_26: size-of s_8_22
        let s_8_26: u16 = s_8_22.length();
        // D s_8_27: lsl s_8_23 s_8_26
        let s_8_27: u128 = s_8_23 << s_8_26;
        // D s_8_28: or s_8_27 s_8_25
        let s_8_28: u128 = ((s_8_27) | (s_8_25));
        // D s_8_29: add s_8_24 s_8_26
        let s_8_29: u16 = (s_8_24 + s_8_26);
        // D s_8_30: create-bits s_8_28 s_8_29
        let s_8_30: Bits = Bits::new(s_8_28, s_8_29);
        // D s_8_31: cast reint s_8_30 -> u8
        let s_8_31: u8 = (s_8_30.value() as u8);
        // D s_8_32: cast zx s_8_18 -> bv
        let s_8_32: Bits = Bits::new(s_8_18 as u128, 1u16);
        // D s_8_33: cast zx s_8_31 -> bv
        let s_8_33: Bits = Bits::new(s_8_31 as u128, 2u16);
        // D s_8_34: cast reint s_8_32 -> u128
        let s_8_34: u128 = (s_8_32.value() as u128);
        // D s_8_35: size-of s_8_32
        let s_8_35: u16 = s_8_32.length();
        // D s_8_36: cast reint s_8_33 -> u128
        let s_8_36: u128 = (s_8_33.value() as u128);
        // D s_8_37: size-of s_8_33
        let s_8_37: u16 = s_8_33.length();
        // D s_8_38: lsl s_8_34 s_8_37
        let s_8_38: u128 = s_8_34 << s_8_37;
        // D s_8_39: or s_8_38 s_8_36
        let s_8_39: u128 = ((s_8_38) | (s_8_36));
        // D s_8_40: add s_8_35 s_8_37
        let s_8_40: u16 = (s_8_35 + s_8_37);
        // D s_8_41: create-bits s_8_39 s_8_40
        let s_8_41: Bits = Bits::new(s_8_39, s_8_40);
        // D s_8_42: cast reint s_8_41 -> u8
        let s_8_42: u8 = (s_8_41.value() as u8);
        // D s_8_43: read-var s2perms.4:struct
        let s_8_43: bool = fn_state.s2perms._4;
        // D s_8_44: read-var s2perms.6:struct
        let s_8_44: bool = fn_state.s2perms._6;
        // D s_8_45: read-var s2perms.5:struct
        let s_8_45: bool = fn_state.s2perms._5;
        // D s_8_46: cast zx s_8_44 -> bv
        let s_8_46: Bits = Bits::new(s_8_44 as u128, 1u16);
        // D s_8_47: cast zx s_8_45 -> bv
        let s_8_47: Bits = Bits::new(s_8_45 as u128, 1u16);
        // D s_8_48: cast reint s_8_46 -> u128
        let s_8_48: u128 = (s_8_46.value() as u128);
        // D s_8_49: size-of s_8_46
        let s_8_49: u16 = s_8_46.length();
        // D s_8_50: cast reint s_8_47 -> u128
        let s_8_50: u128 = (s_8_47.value() as u128);
        // D s_8_51: size-of s_8_47
        let s_8_51: u16 = s_8_47.length();
        // D s_8_52: lsl s_8_48 s_8_51
        let s_8_52: u128 = s_8_48 << s_8_51;
        // D s_8_53: or s_8_52 s_8_50
        let s_8_53: u128 = ((s_8_52) | (s_8_50));
        // D s_8_54: add s_8_49 s_8_51
        let s_8_54: u16 = (s_8_49 + s_8_51);
        // D s_8_55: create-bits s_8_53 s_8_54
        let s_8_55: Bits = Bits::new(s_8_53, s_8_54);
        // D s_8_56: cast reint s_8_55 -> u8
        let s_8_56: u8 = (s_8_55.value() as u8);
        // D s_8_57: cast zx s_8_43 -> bv
        let s_8_57: Bits = Bits::new(s_8_43 as u128, 1u16);
        // D s_8_58: cast zx s_8_56 -> bv
        let s_8_58: Bits = Bits::new(s_8_56 as u128, 2u16);
        // D s_8_59: cast reint s_8_57 -> u128
        let s_8_59: u128 = (s_8_57.value() as u128);
        // D s_8_60: size-of s_8_57
        let s_8_60: u16 = s_8_57.length();
        // D s_8_61: cast reint s_8_58 -> u128
        let s_8_61: u128 = (s_8_58.value() as u128);
        // D s_8_62: size-of s_8_58
        let s_8_62: u16 = s_8_58.length();
        // D s_8_63: lsl s_8_59 s_8_62
        let s_8_63: u128 = s_8_59 << s_8_62;
        // D s_8_64: or s_8_63 s_8_61
        let s_8_64: u128 = ((s_8_63) | (s_8_61));
        // D s_8_65: add s_8_60 s_8_62
        let s_8_65: u16 = (s_8_60 + s_8_62);
        // D s_8_66: create-bits s_8_64 s_8_65
        let s_8_66: Bits = Bits::new(s_8_64, s_8_65);
        // D s_8_67: cast reint s_8_66 -> u8
        let s_8_67: u8 = (s_8_66.value() as u8);
        // D s_8_68: cast zx s_8_42 -> bv
        let s_8_68: Bits = Bits::new(s_8_42 as u128, 3u16);
        // D s_8_69: cast zx s_8_67 -> bv
        let s_8_69: Bits = Bits::new(s_8_67 as u128, 3u16);
        // D s_8_70: and s_8_68 s_8_69
        let s_8_70: Bits = ((s_8_68) & (s_8_69));
        // D s_8_71: cast reint s_8_70 -> u8
        let s_8_71: u8 = (s_8_70.value() as u8);
        // D s_8_72: cast zx s_8_71 -> bv
        let s_8_72: Bits = Bits::new(s_8_71 as u128, 3u16);
        // C s_8_73: const #3u : u8
        let s_8_73: u8 = 3;
        // C s_8_74: cast zx s_8_73 -> bv
        let s_8_74: Bits = Bits::new(s_8_73 as u128, 3u16);
        // D s_8_75: cmp-eq s_8_72 s_8_74
        let s_8_75: bool = ((s_8_72) == (s_8_74));
        // N s_8_76: branch s_8_75 b10 b9
        if s_8_75 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var s2perms.11 <= s_9_0
        fn_state.s2perms._11 = s_9_0;
        // C s_9_2: const #0u : u8
        let s_9_2: bool = false;
        // D s_9_3: write-var s2perms.12 <= s_9_2
        fn_state.s2perms._12 = s_9_2;
        // N s_9_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // D s_10_0: read-var s2perms.11:struct
        let s_10_0: bool = fn_state.s2perms._11;
        // D s_10_1: read-var s2overlay_permsshadow#306.11:struct
        let s_10_1: bool = fn_state.s2overlay_permsshadow_306._11;
        // D s_10_2: cast zx s_10_0 -> bv
        let s_10_2: Bits = Bits::new(s_10_0 as u128, 1u16);
        // D s_10_3: cast zx s_10_1 -> bv
        let s_10_3: Bits = Bits::new(s_10_1 as u128, 1u16);
        // D s_10_4: or s_10_2 s_10_3
        let s_10_4: Bits = ((s_10_2) | (s_10_3));
        // D s_10_5: cast reint s_10_4 -> u8
        let s_10_5: bool = ((s_10_4.value()) != 0);
        // D s_10_6: write-var s2perms.11 <= s_10_5
        fn_state.s2perms._11 = s_10_5;
        // D s_10_7: read-var s2perms.12:struct
        let s_10_7: bool = fn_state.s2perms._12;
        // D s_10_8: read-var s2overlay_permsshadow#306.12:struct
        let s_10_8: bool = fn_state.s2overlay_permsshadow_306._12;
        // D s_10_9: cast zx s_10_7 -> bv
        let s_10_9: Bits = Bits::new(s_10_7 as u128, 1u16);
        // D s_10_10: cast zx s_10_8 -> bv
        let s_10_10: Bits = Bits::new(s_10_8 as u128, 1u16);
        // D s_10_11: or s_10_9 s_10_10
        let s_10_11: Bits = ((s_10_9) | (s_10_10));
        // D s_10_12: cast reint s_10_11 -> u8
        let s_10_12: bool = ((s_10_11.value()) != 0);
        // D s_10_13: write-var s2perms.12 <= s_10_12
        fn_state.s2perms._12 = s_10_12;
        // N s_10_14: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType2fc9d3588999ac79 {
        // C s_11_0: const #15328u : u32
        let s_11_0: u32 = 15328;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_VTCR_EL2_Type_S2POE(s_11_1)
        let s_11_2: bool = u_get_VTCR_EL2_Type_S2POE(state, tracer, s_11_1);
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #1u : u8
        let s_11_4: bool = true;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // D s_11_7: write-var gs#18769 <= s_11_6
        fn_state.gs_18769 = s_11_6;
        // N s_11_8: jump b5
        return block_5(state, tracer, fn_state);
    }
}
