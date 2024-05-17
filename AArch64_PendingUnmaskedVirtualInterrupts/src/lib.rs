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
use u_get_HCR_EL2_Type_VF::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_HCRX_EL2_Type_VFNMI::*;
use u_get_HCR_EL2_Type_VSE::*;
use HaveFeatNMI::*;
use u_get_SCTLRType_NMI::*;
use u_get_HCRX_EL2_Type_VINMI::*;
use u_get_HCR_EL2_Type_FMO::*;
use SCTLR_read__1::*;
use IsHCRXEL2Enabled::*;
use Bit::*;
use u_get_HCR_EL2_Type_IMO::*;
use u_get_HCR_EL2_Type_AMO::*;
use u_get_SCTLRType_SPINTMASK::*;
use u_get_HCR_EL2_Type_VI::*;
use common::*;
pub fn AArch64_PendingUnmaskedVirtualInterrupts<T: Tracer>(
    state: &mut State,
    tracer: &T,
    mask_in: u8,
) -> ProductTyped8f896a024a4e2cb {
    #[derive(Default)]
    struct FunctionState {
        pending: u8,
        gs_327471: bool,
        mask: u8,
        allintmaskshadow_7998: bool,
        gs_327477: bool,
        gs_327470: bool,
        gs_327472: bool,
        gs_327474: bool,
        gs_327476: bool,
        gs_327478: bool,
        mask_in: u8,
    }
    let fn_state = FunctionState {
        mask_in,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_0_0: read-var mask_in:u8
        let s_0_0: u8 = fn_state.mask_in;
        // D s_0_1: write-var mask <= s_0_0
        fn_state.mask = s_0_0;
        // C s_0_2: const #16975u : u32
        let s_0_2: u32 = 16975;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // C s_0_5: const #448u : u32
        let s_0_5: u32 = 448;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: u8 = {
            let value = state.read_register::<u8>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // D s_0_8: cmp-eq s_0_4 s_0_7
        let s_0_8: bool = ((s_0_4) == (s_0_7));
        // N s_0_9: branch s_0_8 b45 b1
        if s_0_8 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #440u : u32
        let s_1_3: u32 = 440;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // D s_1_7: write-var gs#327470 <= s_1_6
        fn_state.gs_327470 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_2_0: read-var gs#327470:u8
        let s_2_0: bool = fn_state.gs_327470;
        // N s_2_1: branch s_2_0 b44 b3
        if s_2_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#327471 <= s_3_0
        fn_state.gs_327471 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_4_0: read-var gs#327471:u8
        let s_4_0: bool = fn_state.gs_327471;
        // N s_4_1: branch s_4_0 b43 b5
        if s_4_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#327472 <= s_5_0
        fn_state.gs_327472 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_6_0: read-var gs#327472:u8
        let s_6_0: bool = fn_state.gs_327472;
        // N s_6_1: branch s_6_0 b9 b7
        if s_6_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_7_0: const #0u : u8
        let s_7_0: u8 = 0;
        // D s_7_1: write-var pending <= s_7_0
        fn_state.pending = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_8_0: read-var mask:u8
        let s_8_0: u8 = fn_state.mask;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 3u16);
        // D s_8_2: not s_8_1
        let s_8_2: Bits = !s_8_1;
        // D s_8_3: cast reint s_8_2 -> u8
        let s_8_3: u8 = (s_8_2.value() as u8);
        // D s_8_4: read-var pending:u8
        let s_8_4: u8 = fn_state.pending;
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 3u16);
        // D s_8_6: cast zx s_8_3 -> bv
        let s_8_6: Bits = Bits::new(s_8_3 as u128, 3u16);
        // D s_8_7: and s_8_5 s_8_6
        let s_8_7: Bits = ((s_8_5) & (s_8_6));
        // D s_8_8: cast reint s_8_7 -> u8
        let s_8_8: u8 = (s_8_7.value() as u8);
        // C s_8_9: const #2s : i
        let s_8_9: i128 = 2;
        // D s_8_10: cast zx s_8_8 -> bv
        let s_8_10: Bits = Bits::new(s_8_8 as u128, 3u16);
        // C s_8_11: const #1u : u64
        let s_8_11: u64 = 1;
        // D s_8_12: bit-extract s_8_10 s_8_9 s_8_11
        let s_8_12: Bits = (Bits::new(
            ((s_8_10) >> (s_8_9)).value(),
            u16::try_from(s_8_11).unwrap(),
        ));
        // D s_8_13: cast reint s_8_12 -> u8
        let s_8_13: bool = ((s_8_12.value()) != 0);
        // C s_8_14: const #0s : i
        let s_8_14: i128 = 0;
        // C s_8_15: const #0u : u64
        let s_8_15: u64 = 0;
        // D s_8_16: cast zx s_8_13 -> u64
        let s_8_16: u64 = (s_8_13 as u64);
        // C s_8_17: const #1u : u64
        let s_8_17: u64 = 1;
        // D s_8_18: and s_8_16 s_8_17
        let s_8_18: u64 = ((s_8_16) & (s_8_17));
        // D s_8_19: cmp-eq s_8_18 s_8_17
        let s_8_19: bool = ((s_8_18) == (s_8_17));
        // D s_8_20: lsl s_8_16 s_8_14
        let s_8_20: u64 = s_8_16 << s_8_14;
        // D s_8_21: or s_8_15 s_8_20
        let s_8_21: u64 = ((s_8_15) | (s_8_20));
        // D s_8_22: cmpl s_8_20
        let s_8_22: u64 = !s_8_20;
        // D s_8_23: and s_8_15 s_8_22
        let s_8_23: u64 = ((s_8_15) & (s_8_22));
        // D s_8_24: select s_8_19 s_8_21 s_8_23
        let s_8_24: u64 = if s_8_19 { s_8_21 } else { s_8_23 };
        // D s_8_25: cast trunc s_8_24 -> u8
        let s_8_25: bool = ((s_8_24) != 0);
        // D s_8_26: cast zx s_8_25 -> bv
        let s_8_26: Bits = Bits::new(s_8_25 as u128, 1u16);
        // C s_8_27: const #1u : u8
        let s_8_27: bool = true;
        // C s_8_28: cast zx s_8_27 -> bv
        let s_8_28: Bits = Bits::new(s_8_27 as u128, 1u16);
        // D s_8_29: cmp-eq s_8_26 s_8_28
        let s_8_29: bool = ((s_8_26) == (s_8_28));
        // C s_8_30: const #1s : i
        let s_8_30: i128 = 1;
        // D s_8_31: cast zx s_8_8 -> bv
        let s_8_31: Bits = Bits::new(s_8_8 as u128, 3u16);
        // C s_8_32: const #1u : u64
        let s_8_32: u64 = 1;
        // D s_8_33: bit-extract s_8_31 s_8_30 s_8_32
        let s_8_33: Bits = (Bits::new(
            ((s_8_31) >> (s_8_30)).value(),
            u16::try_from(s_8_32).unwrap(),
        ));
        // D s_8_34: cast reint s_8_33 -> u8
        let s_8_34: bool = ((s_8_33.value()) != 0);
        // C s_8_35: const #0s : i
        let s_8_35: i128 = 0;
        // C s_8_36: const #0u : u64
        let s_8_36: u64 = 0;
        // D s_8_37: cast zx s_8_34 -> u64
        let s_8_37: u64 = (s_8_34 as u64);
        // C s_8_38: const #1u : u64
        let s_8_38: u64 = 1;
        // D s_8_39: and s_8_37 s_8_38
        let s_8_39: u64 = ((s_8_37) & (s_8_38));
        // D s_8_40: cmp-eq s_8_39 s_8_38
        let s_8_40: bool = ((s_8_39) == (s_8_38));
        // D s_8_41: lsl s_8_37 s_8_35
        let s_8_41: u64 = s_8_37 << s_8_35;
        // D s_8_42: or s_8_36 s_8_41
        let s_8_42: u64 = ((s_8_36) | (s_8_41));
        // D s_8_43: cmpl s_8_41
        let s_8_43: u64 = !s_8_41;
        // D s_8_44: and s_8_36 s_8_43
        let s_8_44: u64 = ((s_8_36) & (s_8_43));
        // D s_8_45: select s_8_40 s_8_42 s_8_44
        let s_8_45: u64 = if s_8_40 { s_8_42 } else { s_8_44 };
        // D s_8_46: cast trunc s_8_45 -> u8
        let s_8_46: bool = ((s_8_45) != 0);
        // D s_8_47: cast zx s_8_46 -> bv
        let s_8_47: Bits = Bits::new(s_8_46 as u128, 1u16);
        // C s_8_48: const #1u : u8
        let s_8_48: bool = true;
        // C s_8_49: cast zx s_8_48 -> bv
        let s_8_49: Bits = Bits::new(s_8_48 as u128, 1u16);
        // D s_8_50: cmp-eq s_8_47 s_8_49
        let s_8_50: bool = ((s_8_47) == (s_8_49));
        // C s_8_51: const #0s : i
        let s_8_51: i128 = 0;
        // D s_8_52: cast zx s_8_8 -> bv
        let s_8_52: Bits = Bits::new(s_8_8 as u128, 3u16);
        // C s_8_53: const #1u : u64
        let s_8_53: u64 = 1;
        // D s_8_54: bit-extract s_8_52 s_8_51 s_8_53
        let s_8_54: Bits = (Bits::new(
            ((s_8_52) >> (s_8_51)).value(),
            u16::try_from(s_8_53).unwrap(),
        ));
        // D s_8_55: cast reint s_8_54 -> u8
        let s_8_55: bool = ((s_8_54.value()) != 0);
        // C s_8_56: const #0s : i
        let s_8_56: i128 = 0;
        // C s_8_57: const #0u : u64
        let s_8_57: u64 = 0;
        // D s_8_58: cast zx s_8_55 -> u64
        let s_8_58: u64 = (s_8_55 as u64);
        // C s_8_59: const #1u : u64
        let s_8_59: u64 = 1;
        // D s_8_60: and s_8_58 s_8_59
        let s_8_60: u64 = ((s_8_58) & (s_8_59));
        // D s_8_61: cmp-eq s_8_60 s_8_59
        let s_8_61: bool = ((s_8_60) == (s_8_59));
        // D s_8_62: lsl s_8_58 s_8_56
        let s_8_62: u64 = s_8_58 << s_8_56;
        // D s_8_63: or s_8_57 s_8_62
        let s_8_63: u64 = ((s_8_57) | (s_8_62));
        // D s_8_64: cmpl s_8_62
        let s_8_64: u64 = !s_8_62;
        // D s_8_65: and s_8_57 s_8_64
        let s_8_65: u64 = ((s_8_57) & (s_8_64));
        // D s_8_66: select s_8_61 s_8_63 s_8_65
        let s_8_66: u64 = if s_8_61 { s_8_63 } else { s_8_65 };
        // D s_8_67: cast trunc s_8_66 -> u8
        let s_8_67: bool = ((s_8_66) != 0);
        // D s_8_68: cast zx s_8_67 -> bv
        let s_8_68: Bits = Bits::new(s_8_67 as u128, 1u16);
        // C s_8_69: const #1u : u8
        let s_8_69: bool = true;
        // C s_8_70: cast zx s_8_69 -> bv
        let s_8_70: Bits = Bits::new(s_8_69 as u128, 1u16);
        // D s_8_71: cmp-eq s_8_68 s_8_70
        let s_8_71: bool = ((s_8_68) == (s_8_70));
        // D s_8_72: create-product struct = ["s_8_29", "s_8_50", "s_8_71"]
        let s_8_72: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_8_29,
            _1: s_8_50,
            _2: s_8_71,
        };
        // N s_8_73: return s_8_72
        return s_8_72;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_9_0: const #102552u : u32
        let s_9_0: u32 = 102552;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_HCR_EL2_Type_VSE(s_9_1)
        let s_9_2: bool = u_get_HCR_EL2_Type_VSE(state, tracer, s_9_1);
        // C s_9_3: const #102552u : u32
        let s_9_3: u32 = 102552;
        // D s_9_4: read-reg s_9_3:struct
        let s_9_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_3 as isize);
            tracer.read_register(s_9_3 as isize, value);
            value
        };
        // D s_9_5: call _get_HCR_EL2_Type_VI(s_9_4)
        let s_9_5: bool = u_get_HCR_EL2_Type_VI(state, tracer, s_9_4);
        // C s_9_6: const #102552u : u32
        let s_9_6: u32 = 102552;
        // D s_9_7: read-reg s_9_6:struct
        let s_9_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_6 as isize);
            tracer.read_register(s_9_6 as isize, value);
            value
        };
        // D s_9_8: call _get_HCR_EL2_Type_VF(s_9_7)
        let s_9_8: bool = u_get_HCR_EL2_Type_VF(state, tracer, s_9_7);
        // D s_9_9: cast zx s_9_5 -> bv
        let s_9_9: Bits = Bits::new(s_9_5 as u128, 1u16);
        // D s_9_10: cast zx s_9_8 -> bv
        let s_9_10: Bits = Bits::new(s_9_8 as u128, 1u16);
        // D s_9_11: cast reint s_9_9 -> u128
        let s_9_11: u128 = (s_9_9.value() as u128);
        // D s_9_12: size-of s_9_9
        let s_9_12: u16 = s_9_9.length();
        // D s_9_13: cast reint s_9_10 -> u128
        let s_9_13: u128 = (s_9_10.value() as u128);
        // D s_9_14: size-of s_9_10
        let s_9_14: u16 = s_9_10.length();
        // D s_9_15: lsl s_9_11 s_9_14
        let s_9_15: u128 = s_9_11 << s_9_14;
        // D s_9_16: or s_9_15 s_9_13
        let s_9_16: u128 = ((s_9_15) | (s_9_13));
        // D s_9_17: add s_9_12 s_9_14
        let s_9_17: u16 = (s_9_12 + s_9_14);
        // D s_9_18: create-bits s_9_16 s_9_17
        let s_9_18: Bits = Bits::new(s_9_16, s_9_17);
        // D s_9_19: cast reint s_9_18 -> u8
        let s_9_19: u8 = (s_9_18.value() as u8);
        // D s_9_20: cast zx s_9_2 -> bv
        let s_9_20: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_21: cast zx s_9_19 -> bv
        let s_9_21: Bits = Bits::new(s_9_19 as u128, 2u16);
        // D s_9_22: cast reint s_9_20 -> u128
        let s_9_22: u128 = (s_9_20.value() as u128);
        // D s_9_23: size-of s_9_20
        let s_9_23: u16 = s_9_20.length();
        // D s_9_24: cast reint s_9_21 -> u128
        let s_9_24: u128 = (s_9_21.value() as u128);
        // D s_9_25: size-of s_9_21
        let s_9_25: u16 = s_9_21.length();
        // D s_9_26: lsl s_9_22 s_9_25
        let s_9_26: u128 = s_9_22 << s_9_25;
        // D s_9_27: or s_9_26 s_9_24
        let s_9_27: u128 = ((s_9_26) | (s_9_24));
        // D s_9_28: add s_9_23 s_9_25
        let s_9_28: u16 = (s_9_23 + s_9_25);
        // D s_9_29: create-bits s_9_27 s_9_28
        let s_9_29: Bits = Bits::new(s_9_27, s_9_28);
        // D s_9_30: cast reint s_9_29 -> u8
        let s_9_30: u8 = (s_9_29.value() as u8);
        // C s_9_31: const #102552u : u32
        let s_9_31: u32 = 102552;
        // D s_9_32: read-reg s_9_31:struct
        let s_9_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_31 as isize);
            tracer.read_register(s_9_31 as isize, value);
            value
        };
        // D s_9_33: call _get_HCR_EL2_Type_AMO(s_9_32)
        let s_9_33: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_9_32);
        // C s_9_34: const #102552u : u32
        let s_9_34: u32 = 102552;
        // D s_9_35: read-reg s_9_34:struct
        let s_9_35: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_34 as isize);
            tracer.read_register(s_9_34 as isize, value);
            value
        };
        // D s_9_36: call _get_HCR_EL2_Type_IMO(s_9_35)
        let s_9_36: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_9_35);
        // C s_9_37: const #102552u : u32
        let s_9_37: u32 = 102552;
        // D s_9_38: read-reg s_9_37:struct
        let s_9_38: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_37 as isize);
            tracer.read_register(s_9_37 as isize, value);
            value
        };
        // D s_9_39: call _get_HCR_EL2_Type_FMO(s_9_38)
        let s_9_39: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_9_38);
        // D s_9_40: cast zx s_9_36 -> bv
        let s_9_40: Bits = Bits::new(s_9_36 as u128, 1u16);
        // D s_9_41: cast zx s_9_39 -> bv
        let s_9_41: Bits = Bits::new(s_9_39 as u128, 1u16);
        // D s_9_42: cast reint s_9_40 -> u128
        let s_9_42: u128 = (s_9_40.value() as u128);
        // D s_9_43: size-of s_9_40
        let s_9_43: u16 = s_9_40.length();
        // D s_9_44: cast reint s_9_41 -> u128
        let s_9_44: u128 = (s_9_41.value() as u128);
        // D s_9_45: size-of s_9_41
        let s_9_45: u16 = s_9_41.length();
        // D s_9_46: lsl s_9_42 s_9_45
        let s_9_46: u128 = s_9_42 << s_9_45;
        // D s_9_47: or s_9_46 s_9_44
        let s_9_47: u128 = ((s_9_46) | (s_9_44));
        // D s_9_48: add s_9_43 s_9_45
        let s_9_48: u16 = (s_9_43 + s_9_45);
        // D s_9_49: create-bits s_9_47 s_9_48
        let s_9_49: Bits = Bits::new(s_9_47, s_9_48);
        // D s_9_50: cast reint s_9_49 -> u8
        let s_9_50: u8 = (s_9_49.value() as u8);
        // D s_9_51: cast zx s_9_33 -> bv
        let s_9_51: Bits = Bits::new(s_9_33 as u128, 1u16);
        // D s_9_52: cast zx s_9_50 -> bv
        let s_9_52: Bits = Bits::new(s_9_50 as u128, 2u16);
        // D s_9_53: cast reint s_9_51 -> u128
        let s_9_53: u128 = (s_9_51.value() as u128);
        // D s_9_54: size-of s_9_51
        let s_9_54: u16 = s_9_51.length();
        // D s_9_55: cast reint s_9_52 -> u128
        let s_9_55: u128 = (s_9_52.value() as u128);
        // D s_9_56: size-of s_9_52
        let s_9_56: u16 = s_9_52.length();
        // D s_9_57: lsl s_9_53 s_9_56
        let s_9_57: u128 = s_9_53 << s_9_56;
        // D s_9_58: or s_9_57 s_9_55
        let s_9_58: u128 = ((s_9_57) | (s_9_55));
        // D s_9_59: add s_9_54 s_9_56
        let s_9_59: u16 = (s_9_54 + s_9_56);
        // D s_9_60: create-bits s_9_58 s_9_59
        let s_9_60: Bits = Bits::new(s_9_58, s_9_59);
        // D s_9_61: cast reint s_9_60 -> u8
        let s_9_61: u8 = (s_9_60.value() as u8);
        // D s_9_62: cast zx s_9_30 -> bv
        let s_9_62: Bits = Bits::new(s_9_30 as u128, 3u16);
        // D s_9_63: cast zx s_9_61 -> bv
        let s_9_63: Bits = Bits::new(s_9_61 as u128, 3u16);
        // D s_9_64: and s_9_62 s_9_63
        let s_9_64: Bits = ((s_9_62) & (s_9_63));
        // D s_9_65: cast reint s_9_64 -> u8
        let s_9_65: u8 = (s_9_64.value() as u8);
        // D s_9_66: write-var pending <= s_9_65
        fn_state.pending = s_9_65;
        // C s_9_67: const #() : ()
        let s_9_67: () = ();
        // S s_9_68: call HaveFeatNMI(s_9_67)
        let s_9_68: bool = HaveFeatNMI(state, tracer, s_9_67);
        // N s_9_69: branch s_9_68 b42 b10
        if s_9_68 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#327474 <= s_10_0
        fn_state.gs_327474 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_11_0: read-var gs#327474:u8
        let s_11_0: bool = fn_state.gs_327474;
        // N s_11_1: branch s_11_0 b14 b12
        if s_11_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_13_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_14_0: const #16969u : u32
        let s_14_0: u32 = 16969;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: bool = {
            let value = state.read_register::<bool>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // C s_14_2: const #16990u : u32
        let s_14_2: u32 = 16990;
        // D s_14_3: read-reg s_14_2:u8
        let s_14_3: bool = {
            let value = state.read_register::<bool>(s_14_2 as isize);
            tracer.read_register(s_14_2 as isize, value);
            value
        };
        // C s_14_4: const #() : ()
        let s_14_4: () = ();
        // S s_14_5: call SCTLR_read__1(s_14_4)
        let s_14_5: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_14_4);
        // S s_14_6: call _get_SCTLRType_SPINTMASK(s_14_5)
        let s_14_6: bool = u_get_SCTLRType_SPINTMASK(state, tracer, s_14_5);
        // D s_14_7: cast zx s_14_3 -> bv
        let s_14_7: Bits = Bits::new(s_14_3 as u128, 1u16);
        // S s_14_8: cast zx s_14_6 -> bv
        let s_14_8: Bits = Bits::new(s_14_6 as u128, 1u16);
        // D s_14_9: and s_14_7 s_14_8
        let s_14_9: Bits = ((s_14_7) & (s_14_8));
        // D s_14_10: cast reint s_14_9 -> u8
        let s_14_10: bool = ((s_14_9.value()) != 0);
        // D s_14_11: cast zx s_14_1 -> bv
        let s_14_11: Bits = Bits::new(s_14_1 as u128, 1u16);
        // D s_14_12: cast zx s_14_10 -> bv
        let s_14_12: Bits = Bits::new(s_14_10 as u128, 1u16);
        // D s_14_13: or s_14_11 s_14_12
        let s_14_13: Bits = ((s_14_11) | (s_14_12));
        // D s_14_14: cast reint s_14_13 -> u8
        let s_14_14: bool = ((s_14_13.value()) != 0);
        // D s_14_15: write-var allintmaskshadow#7998 <= s_14_14
        fn_state.allintmaskshadow_7998 = s_14_14;
        // C s_14_16: const #() : ()
        let s_14_16: () = ();
        // S s_14_17: call IsHCRXEL2Enabled(s_14_16)
        let s_14_17: bool = IsHCRXEL2Enabled(state, tracer, s_14_16);
        // N s_14_18: branch s_14_17 b38 b15
        if s_14_17 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#327477 <= s_15_0
        fn_state.gs_327477 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_16_0: read-var gs#327477:u8
        let s_16_0: bool = fn_state.gs_327477;
        // N s_16_1: branch s_16_0 b31 b17
        if s_16_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_18_0: const #16975u : u32
        let s_18_0: u32 = 16975;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 2u16);
        // C s_18_3: const #440u : u32
        let s_18_3: u32 = 440;
        // D s_18_4: read-reg s_18_3:u8
        let s_18_4: u8 = {
            let value = state.read_register::<u8>(s_18_3 as isize);
            tracer.read_register(s_18_3 as isize, value);
            value
        };
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 2u16);
        // D s_18_6: cmp-eq s_18_2 s_18_5
        let s_18_6: bool = ((s_18_2) == (s_18_5));
        // N s_18_7: branch s_18_6 b30 b19
        if s_18_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#327478 <= s_19_0
        fn_state.gs_327478 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_20_0: read-var gs#327478:u8
        let s_20_0: bool = fn_state.gs_327478;
        // N s_20_1: branch s_20_0 b23 b21
        if s_20_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_22_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_23_0: const #102552u : u32
        let s_23_0: u32 = 102552;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_HCR_EL2_Type_FMO(s_23_1)
        let s_23_2: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // N s_23_7: branch s_23_6 b29 b24
        if s_23_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_25_0: const #102552u : u32
        let s_25_0: u32 = 102552;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_HCR_EL2_Type_IMO(s_25_1)
        let s_25_2: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_25_1);
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #1u : u8
        let s_25_4: bool = true;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // D s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // N s_25_7: branch s_25_6 b28 b26
        if s_25_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_27_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // S s_28_1: call Bit(s_28_0)
        let s_28_1: bool = Bit(state, tracer, s_28_0);
        // C s_28_2: const #1s : i
        let s_28_2: i128 = 1;
        // D s_28_3: read-var mask:u8
        let s_28_3: u8 = fn_state.mask;
        // D s_28_4: cast zx s_28_3 -> bv
        let s_28_4: Bits = Bits::new(s_28_3 as u128, 3u16);
        // C s_28_5: const #1u : u64
        let s_28_5: u64 = 1;
        // D s_28_6: bit-insert s_28_4 s_28_4 s_28_2 s_28_5
        let s_28_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_28_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_28_4.length(),
            );
            (s_28_4 & mask) | (s_28_4 << s_28_2)
        };
        // D s_28_7: cast reint s_28_6 -> u8
        let s_28_7: u8 = (s_28_6.value() as u8);
        // D s_28_8: write-var mask <= s_28_7
        fn_state.mask = s_28_7;
        // N s_28_9: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // S s_29_1: call Bit(s_29_0)
        let s_29_1: bool = Bit(state, tracer, s_29_0);
        // C s_29_2: const #0s : i
        let s_29_2: i128 = 0;
        // D s_29_3: read-var mask:u8
        let s_29_3: u8 = fn_state.mask;
        // D s_29_4: cast zx s_29_3 -> bv
        let s_29_4: Bits = Bits::new(s_29_3 as u128, 3u16);
        // C s_29_5: const #1u : u64
        let s_29_5: u64 = 1;
        // D s_29_6: bit-insert s_29_4 s_29_4 s_29_2 s_29_5
        let s_29_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_29_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_29_4.length(),
            );
            (s_29_4 & mask) | (s_29_4 << s_29_2)
        };
        // D s_29_7: cast reint s_29_6 -> u8
        let s_29_7: u8 = (s_29_6.value() as u8);
        // D s_29_8: write-var mask <= s_29_7
        fn_state.mask = s_29_7;
        // N s_29_9: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_30_0: read-var allintmaskshadow#7998:u8
        let s_30_0: bool = fn_state.allintmaskshadow_7998;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#327478 <= s_30_4
        fn_state.gs_327478 = s_30_4;
        // N s_30_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_31_0: const #22528u : u32
        let s_31_0: u32 = 22528;
        // D s_31_1: read-reg s_31_0:struct
        let s_31_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call _get_HCRX_EL2_Type_VFNMI(s_31_1)
        let s_31_2: bool = u_get_HCRX_EL2_Type_VFNMI(state, tracer, s_31_1);
        // D s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // C s_31_4: const #1u : u8
        let s_31_4: bool = true;
        // C s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 1u16);
        // D s_31_6: cmp-eq s_31_3 s_31_5
        let s_31_6: bool = ((s_31_3) == (s_31_5));
        // N s_31_7: branch s_31_6 b37 b32
        if s_31_6 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_32_0: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_33_0: const #22528u : u32
        let s_33_0: u32 = 22528;
        // D s_33_1: read-reg s_33_0:struct
        let s_33_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call _get_HCRX_EL2_Type_VINMI(s_33_1)
        let s_33_2: bool = u_get_HCRX_EL2_Type_VINMI(state, tracer, s_33_1);
        // D s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // C s_33_4: const #1u : u8
        let s_33_4: bool = true;
        // C s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 1u16);
        // D s_33_6: cmp-eq s_33_3 s_33_5
        let s_33_6: bool = ((s_33_3) == (s_33_5));
        // N s_33_7: branch s_33_6 b36 b34
        if s_33_6 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_34_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_35_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // S s_36_1: call Bit(s_36_0)
        let s_36_1: bool = Bit(state, tracer, s_36_0);
        // C s_36_2: const #1s : i
        let s_36_2: i128 = 1;
        // D s_36_3: read-var mask:u8
        let s_36_3: u8 = fn_state.mask;
        // D s_36_4: cast zx s_36_3 -> bv
        let s_36_4: Bits = Bits::new(s_36_3 as u128, 3u16);
        // C s_36_5: const #1u : u64
        let s_36_5: u64 = 1;
        // D s_36_6: bit-insert s_36_4 s_36_4 s_36_2 s_36_5
        let s_36_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_36_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_36_4.length(),
            );
            (s_36_4 & mask) | (s_36_4 << s_36_2)
        };
        // D s_36_7: cast reint s_36_6 -> u8
        let s_36_7: u8 = (s_36_6.value() as u8);
        // D s_36_8: write-var mask <= s_36_7
        fn_state.mask = s_36_7;
        // N s_36_9: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // S s_37_1: call Bit(s_37_0)
        let s_37_1: bool = Bit(state, tracer, s_37_0);
        // C s_37_2: const #0s : i
        let s_37_2: i128 = 0;
        // D s_37_3: read-var mask:u8
        let s_37_3: u8 = fn_state.mask;
        // D s_37_4: cast zx s_37_3 -> bv
        let s_37_4: Bits = Bits::new(s_37_3 as u128, 3u16);
        // C s_37_5: const #1u : u64
        let s_37_5: u64 = 1;
        // D s_37_6: bit-insert s_37_4 s_37_4 s_37_2 s_37_5
        let s_37_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_37_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_37_4.length(),
            );
            (s_37_4 & mask) | (s_37_4 << s_37_2)
        };
        // D s_37_7: cast reint s_37_6 -> u8
        let s_37_7: u8 = (s_37_6.value() as u8);
        // D s_37_8: write-var mask <= s_37_7
        fn_state.mask = s_37_7;
        // N s_37_9: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_38_0: const #16975u : u32
        let s_38_0: u32 = 16975;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: cast zx s_38_1 -> bv
        let s_38_2: Bits = Bits::new(s_38_1 as u128, 2u16);
        // C s_38_3: const #448u : u32
        let s_38_3: u32 = 448;
        // D s_38_4: read-reg s_38_3:u8
        let s_38_4: u8 = {
            let value = state.read_register::<u8>(s_38_3 as isize);
            tracer.read_register(s_38_3 as isize, value);
            value
        };
        // D s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 2u16);
        // D s_38_6: cmp-eq s_38_2 s_38_5
        let s_38_6: bool = ((s_38_2) == (s_38_5));
        // N s_38_7: branch s_38_6 b41 b39
        if s_38_6 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_39_0: read-var allintmaskshadow#7998:u8
        let s_39_0: bool = fn_state.allintmaskshadow_7998;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #0u : u8
        let s_39_2: bool = false;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#327476 <= s_39_4
        fn_state.gs_327476 = s_39_4;
        // N s_39_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_40_0: read-var gs#327476:u8
        let s_40_0: bool = fn_state.gs_327476;
        // D s_40_1: write-var gs#327477 <= s_40_0
        fn_state.gs_327477 = s_40_0;
        // N s_40_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#327476 <= s_41_0
        fn_state.gs_327476 = s_41_0;
        // N s_41_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call SCTLR_read__1(s_42_0)
        let s_42_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_42_0);
        // S s_42_2: call _get_SCTLRType_NMI(s_42_1)
        let s_42_2: bool = u_get_SCTLRType_NMI(state, tracer, s_42_1);
        // S s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // C s_42_4: const #1u : u8
        let s_42_4: bool = true;
        // C s_42_5: cast zx s_42_4 -> bv
        let s_42_5: Bits = Bits::new(s_42_4 as u128, 1u16);
        // S s_42_6: cmp-eq s_42_3 s_42_5
        let s_42_6: bool = ((s_42_3) == (s_42_5));
        // D s_42_7: write-var gs#327474 <= s_42_6
        fn_state.gs_327474 = s_42_6;
        // N s_42_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
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
        // D s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // C s_43_4: const #0u : u8
        let s_43_4: bool = false;
        // C s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 1u16);
        // D s_43_6: cmp-eq s_43_3 s_43_5
        let s_43_6: bool = ((s_43_3) == (s_43_5));
        // D s_43_7: write-var gs#327472 <= s_43_6
        fn_state.gs_327472 = s_43_6;
        // N s_43_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call EL2Enabled(s_44_0)
        let s_44_1: bool = EL2Enabled(state, tracer, s_44_0);
        // D s_44_2: write-var gs#327471 <= s_44_1
        fn_state.gs_327471 = s_44_1;
        // N s_44_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#327470 <= s_45_0
        fn_state.gs_327470 = s_45_0;
        // N s_45_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
