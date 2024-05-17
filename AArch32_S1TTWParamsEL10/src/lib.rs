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
use MAIR0_NS_read::*;
use SCTLR_NS_read::*;
use TTBCR_read::*;
use TTBCR2_read::*;
use AArch32_GetVARange::*;
use AArch32_HaveHPDExt::*;
use SCTLR_read__2::*;
use u_get_SCR_EL3_Type_SIF::*;
use HaveTrapLoadStoreMultipleDeviceExt::*;
use u_get_TTBCR_Type_T0SZ::*;
use MAIR1_read::*;
use u_get_SCTLR_Type_UWXN::*;
use u_get_TTBCR_Type_SH1::*;
use u_get_TTBCR_Type_SH0::*;
use u_get_TTBCR_Type_IRGN0::*;
use u_get_TTBCR2_Type_HPD0::*;
use u_get_TTBCR_Type_ORGN0::*;
use u_get_SCTLR_Type_WXN::*;
use MAIR0_read::*;
use u_get_TTBCR_Type_T1SZ::*;
use u_get_SCR_Type_SIF::*;
use u_get_TTBCR_Type_IRGN1::*;
use Mk_MAIRType::*;
use u_get_SCTLR_Type_nTLSMD::*;
use u_get_TTBCR_Type_ORGN1::*;
use u_get_TTBCR_Type_T2E::*;
use u_get_TTBCR2_Type_HPD1::*;
use ELUsingAArch32::*;
use MAIR1_NS_read::*;
use TTBCR_NS_read::*;
use u_get_TTBCR_Type_EAE::*;
use u_get_SCTLR_Type_EE::*;
use TTBCR2_NS_read::*;
use common::*;
pub fn AArch32_S1TTWParamsEL10<T: Tracer>(
    state: &mut State,
    tracer: &T,
    va: u32,
) -> ProductTypeef284266e139aee2 {
    #[derive(Default)]
    struct FunctionState {
        sif: bool,
        ttbcr2: ProductType700c18a878c5601b,
        ga_21220: ProductType700c18a878c5601b,
        walkparams: ProductTypeef284266e139aee2,
        mair: u64,
        ga_21224: ProductType700c18a878c5601b,
        ga_21223: ProductType700c18a878c5601b,
        ga_21219: ProductType700c18a878c5601b,
        ttbcr: ProductType700c18a878c5601b,
        sctlr: ProductType700c18a878c5601b,
        va: u32,
    }
    let fn_state = FunctionState {
        va,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_0_0: const #424u : u32
        let s_0_0: u32 = 424;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call ELUsingAArch32(s_0_1)
        let s_0_2: bool = ELUsingAArch32(state, tracer, s_0_1);
        // N s_0_3: branch s_0_2 b18 b1
        if s_0_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call TTBCR_read(s_1_0)
        let s_1_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_1_0);
        // D s_1_2: write-var ttbcr <= s_1_1
        fn_state.ttbcr = s_1_1;
        // C s_1_3: const #() : ()
        let s_1_3: () = ();
        // S s_1_4: call TTBCR2_read(s_1_3)
        let s_1_4: ProductType700c18a878c5601b = TTBCR2_read(state, tracer, s_1_3);
        // D s_1_5: write-var ttbcr2 <= s_1_4
        fn_state.ttbcr2 = s_1_4;
        // C s_1_6: const #() : ()
        let s_1_6: () = ();
        // S s_1_7: call SCTLR_read__2(s_1_6)
        let s_1_7: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_1_6);
        // D s_1_8: write-var sctlr <= s_1_7
        fn_state.sctlr = s_1_7;
        // C s_1_9: const #() : ()
        let s_1_9: () = ();
        // S s_1_10: call MAIR1_read(s_1_9)
        let s_1_10: ProductType700c18a878c5601b = MAIR1_read(state, tracer, s_1_9);
        // D s_1_11: write-var ga#21223 <= s_1_10
        fn_state.ga_21223 = s_1_10;
        // D s_1_12: read-var ga#21223.0:struct
        let s_1_12: u32 = fn_state.ga_21223._0;
        // C s_1_13: const #() : ()
        let s_1_13: () = ();
        // S s_1_14: call MAIR0_read(s_1_13)
        let s_1_14: ProductType700c18a878c5601b = MAIR0_read(state, tracer, s_1_13);
        // D s_1_15: write-var ga#21224 <= s_1_14
        fn_state.ga_21224 = s_1_14;
        // D s_1_16: read-var ga#21224.0:struct
        let s_1_16: u32 = fn_state.ga_21224._0;
        // D s_1_17: cast zx s_1_12 -> bv
        let s_1_17: Bits = Bits::new(s_1_12 as u128, 32u16);
        // D s_1_18: cast zx s_1_16 -> bv
        let s_1_18: Bits = Bits::new(s_1_16 as u128, 32u16);
        // D s_1_19: cast reint s_1_17 -> u128
        let s_1_19: u128 = (s_1_17.value() as u128);
        // D s_1_20: size-of s_1_17
        let s_1_20: u16 = s_1_17.length();
        // D s_1_21: cast reint s_1_18 -> u128
        let s_1_21: u128 = (s_1_18.value() as u128);
        // D s_1_22: size-of s_1_18
        let s_1_22: u16 = s_1_18.length();
        // D s_1_23: lsl s_1_19 s_1_22
        let s_1_23: u128 = s_1_19 << s_1_22;
        // D s_1_24: or s_1_23 s_1_21
        let s_1_24: u128 = ((s_1_23) | (s_1_21));
        // D s_1_25: add s_1_20 s_1_22
        let s_1_25: u16 = (s_1_20 + s_1_22);
        // D s_1_26: create-bits s_1_24 s_1_25
        let s_1_26: Bits = Bits::new(s_1_24, s_1_25);
        // D s_1_27: cast reint s_1_26 -> u64
        let s_1_27: u64 = (s_1_26.value() as u64);
        // D s_1_28: write-var mair <= s_1_27
        fn_state.mair = s_1_27;
        // C s_1_29: const #424u : u32
        let s_1_29: u32 = 424;
        // D s_1_30: read-reg s_1_29:u8
        let s_1_30: u8 = {
            let value = state.read_register::<u8>(s_1_29 as isize);
            tracer.read_register(s_1_29 as isize, value);
            value
        };
        // C s_1_31: const #2u : u8
        let s_1_31: u8 = 2;
        // D s_1_32: cmp-lt s_1_30 s_1_31
        let s_1_32: bool = ((s_1_30) < (s_1_31));
        // N s_1_33: branch s_1_32 b17 b2
        if s_1_32 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var sif <= s_2_0
        fn_state.sif = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_4_0: read-var ttbcr:struct
        let s_4_0: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_4_1: call _get_TTBCR_Type_EAE(s_4_0)
        let s_4_1: bool = u_get_TTBCR_Type_EAE(state, tracer, s_4_0);
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 1u16);
        // C s_4_3: const #1u : u8
        let s_4_3: bool = true;
        // C s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 1u16);
        // D s_4_5: cmp-eq s_4_2 s_4_4
        let s_4_5: bool = ((s_4_2) == (s_4_4));
        // N s_4_6: assert s_4_5
        let s_4_6: () = assert!(s_4_5);
        // D s_4_7: read-var ttbcr:struct
        let s_4_7: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_4_8: call _get_TTBCR_Type_T0SZ(s_4_7)
        let s_4_8: u8 = u_get_TTBCR_Type_T0SZ(state, tracer, s_4_7);
        // D s_4_9: write-var walkparams.32 <= s_4_8
        fn_state.walkparams._32 = s_4_8;
        // D s_4_10: read-var ttbcr:struct
        let s_4_10: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_4_11: call _get_TTBCR_Type_T1SZ(s_4_10)
        let s_4_11: u8 = u_get_TTBCR_Type_T1SZ(state, tracer, s_4_10);
        // D s_4_12: write-var walkparams.33 <= s_4_11
        fn_state.walkparams._33 = s_4_11;
        // D s_4_13: read-var sctlr:struct
        let s_4_13: ProductType700c18a878c5601b = fn_state.sctlr;
        // D s_4_14: call _get_SCTLR_Type_EE(s_4_13)
        let s_4_14: bool = u_get_SCTLR_Type_EE(state, tracer, s_4_13);
        // D s_4_15: write-var walkparams.9 <= s_4_14
        fn_state.walkparams._9 = s_4_14;
        // D s_4_16: read-var sctlr:struct
        let s_4_16: ProductType700c18a878c5601b = fn_state.sctlr;
        // D s_4_17: call _get_SCTLR_Type_WXN(s_4_16)
        let s_4_17: bool = u_get_SCTLR_Type_WXN(state, tracer, s_4_16);
        // D s_4_18: write-var walkparams.39 <= s_4_17
        fn_state.walkparams._39 = s_4_17;
        // D s_4_19: read-var sctlr:struct
        let s_4_19: ProductType700c18a878c5601b = fn_state.sctlr;
        // D s_4_20: call _get_SCTLR_Type_UWXN(s_4_19)
        let s_4_20: bool = u_get_SCTLR_Type_UWXN(state, tracer, s_4_19);
        // D s_4_21: write-var walkparams.38 <= s_4_20
        fn_state.walkparams._38 = s_4_20;
        // C s_4_22: const #() : ()
        let s_4_22: () = ();
        // S s_4_23: call HaveTrapLoadStoreMultipleDeviceExt(s_4_22)
        let s_4_23: bool = HaveTrapLoadStoreMultipleDeviceExt(state, tracer, s_4_22);
        // N s_4_24: branch s_4_23 b16 b5
        if s_4_23 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var walkparams.21 <= s_5_0
        fn_state.walkparams._21 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_6_0: read-var mair:u64
        let s_6_0: u64 = fn_state.mair;
        // D s_6_1: call Mk_MAIRType(s_6_0)
        let s_6_1: ProductType5c790c8ef59cc8b2 = Mk_MAIRType(state, tracer, s_6_0);
        // D s_6_2: write-var walkparams.17 <= s_6_1
        fn_state.walkparams._17 = s_6_1;
        // D s_6_3: read-var sif:u8
        let s_6_3: bool = fn_state.sif;
        // D s_6_4: write-var walkparams.30 <= s_6_3
        fn_state.walkparams._30 = s_6_3;
        // D s_6_5: read-var walkparams.32:struct
        let s_6_5: u8 = fn_state.walkparams._32;
        // D s_6_6: read-var walkparams.33:struct
        let s_6_6: u8 = fn_state.walkparams._33;
        // D s_6_7: read-var va:u32
        let s_6_7: u32 = fn_state.va;
        // D s_6_8: call AArch32_GetVARange(s_6_7, s_6_5, s_6_6)
        let s_6_8: u32 = AArch32_GetVARange(state, tracer, s_6_7, s_6_5, s_6_6);
        // C s_6_9: const #0u : u32
        let s_6_9: u32 = 0;
        // D s_6_10: cmp-eq s_6_8 s_6_9
        let s_6_10: bool = ((s_6_8) == (s_6_9));
        // N s_6_11: branch s_6_10 b12 b7
        if s_6_10 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_7_0: read-var ttbcr:struct
        let s_7_0: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_7_1: call _get_TTBCR_Type_SH1(s_7_0)
        let s_7_1: u8 = u_get_TTBCR_Type_SH1(state, tracer, s_7_0);
        // D s_7_2: write-var walkparams.29 <= s_7_1
        fn_state.walkparams._29 = s_7_1;
        // D s_7_3: read-var ttbcr:struct
        let s_7_3: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_7_4: call _get_TTBCR_Type_IRGN1(s_7_3)
        let s_7_4: u8 = u_get_TTBCR_Type_IRGN1(state, tracer, s_7_3);
        // D s_7_5: write-var walkparams.16 <= s_7_4
        fn_state.walkparams._16 = s_7_4;
        // D s_7_6: read-var ttbcr:struct
        let s_7_6: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_7_7: call _get_TTBCR_Type_ORGN1(s_7_6)
        let s_7_7: u8 = u_get_TTBCR_Type_ORGN1(state, tracer, s_7_6);
        // D s_7_8: write-var walkparams.23 <= s_7_7
        fn_state.walkparams._23 = s_7_7;
        // C s_7_9: const #() : ()
        let s_7_9: () = ();
        // S s_7_10: call AArch32_HaveHPDExt(s_7_9)
        let s_7_10: bool = AArch32_HaveHPDExt(state, tracer, s_7_9);
        // N s_7_11: branch s_7_10 b11 b8
        if s_7_10 {
            return block_11(state, tracer, fn_state);
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
        // D s_8_1: write-var walkparams.15 <= s_8_0
        fn_state.walkparams._15 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_10_0: read-var walkparams:struct
        let s_10_0: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_11_0: read-var ttbcr:struct
        let s_11_0: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_11_1: call _get_TTBCR_Type_T2E(s_11_0)
        let s_11_1: bool = u_get_TTBCR_Type_T2E(state, tracer, s_11_0);
        // D s_11_2: read-var ttbcr2:struct
        let s_11_2: ProductType700c18a878c5601b = fn_state.ttbcr2;
        // D s_11_3: call _get_TTBCR2_Type_HPD1(s_11_2)
        let s_11_3: bool = u_get_TTBCR2_Type_HPD1(state, tracer, s_11_2);
        // D s_11_4: cast zx s_11_1 -> bv
        let s_11_4: Bits = Bits::new(s_11_1 as u128, 1u16);
        // D s_11_5: cast zx s_11_3 -> bv
        let s_11_5: Bits = Bits::new(s_11_3 as u128, 1u16);
        // D s_11_6: and s_11_4 s_11_5
        let s_11_6: Bits = ((s_11_4) & (s_11_5));
        // D s_11_7: cast reint s_11_6 -> u8
        let s_11_7: bool = ((s_11_6.value()) != 0);
        // D s_11_8: write-var walkparams.15 <= s_11_7
        fn_state.walkparams._15 = s_11_7;
        // N s_11_9: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_12_0: read-var ttbcr:struct
        let s_12_0: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_12_1: call _get_TTBCR_Type_SH0(s_12_0)
        let s_12_1: u8 = u_get_TTBCR_Type_SH0(state, tracer, s_12_0);
        // D s_12_2: write-var walkparams.29 <= s_12_1
        fn_state.walkparams._29 = s_12_1;
        // D s_12_3: read-var ttbcr:struct
        let s_12_3: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_12_4: call _get_TTBCR_Type_IRGN0(s_12_3)
        let s_12_4: u8 = u_get_TTBCR_Type_IRGN0(state, tracer, s_12_3);
        // D s_12_5: write-var walkparams.16 <= s_12_4
        fn_state.walkparams._16 = s_12_4;
        // D s_12_6: read-var ttbcr:struct
        let s_12_6: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_12_7: call _get_TTBCR_Type_ORGN0(s_12_6)
        let s_12_7: u8 = u_get_TTBCR_Type_ORGN0(state, tracer, s_12_6);
        // D s_12_8: write-var walkparams.23 <= s_12_7
        fn_state.walkparams._23 = s_12_7;
        // C s_12_9: const #() : ()
        let s_12_9: () = ();
        // S s_12_10: call AArch32_HaveHPDExt(s_12_9)
        let s_12_10: bool = AArch32_HaveHPDExt(state, tracer, s_12_9);
        // N s_12_11: branch s_12_10 b15 b13
        if s_12_10 {
            return block_15(state, tracer, fn_state);
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
        // N s_14_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_15_0: read-var ttbcr:struct
        let s_15_0: ProductType700c18a878c5601b = fn_state.ttbcr;
        // D s_15_1: call _get_TTBCR_Type_T2E(s_15_0)
        let s_15_1: bool = u_get_TTBCR_Type_T2E(state, tracer, s_15_0);
        // D s_15_2: read-var ttbcr2:struct
        let s_15_2: ProductType700c18a878c5601b = fn_state.ttbcr2;
        // D s_15_3: call _get_TTBCR2_Type_HPD0(s_15_2)
        let s_15_3: bool = u_get_TTBCR2_Type_HPD0(state, tracer, s_15_2);
        // D s_15_4: cast zx s_15_1 -> bv
        let s_15_4: Bits = Bits::new(s_15_1 as u128, 1u16);
        // D s_15_5: cast zx s_15_3 -> bv
        let s_15_5: Bits = Bits::new(s_15_3 as u128, 1u16);
        // D s_15_6: and s_15_4 s_15_5
        let s_15_6: Bits = ((s_15_4) & (s_15_5));
        // D s_15_7: cast reint s_15_6 -> u8
        let s_15_7: bool = ((s_15_6.value()) != 0);
        // D s_15_8: write-var walkparams.15 <= s_15_7
        fn_state.walkparams._15 = s_15_7;
        // N s_15_9: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_16_0: read-var sctlr:struct
        let s_16_0: ProductType700c18a878c5601b = fn_state.sctlr;
        // D s_16_1: call _get_SCTLR_Type_nTLSMD(s_16_0)
        let s_16_1: bool = u_get_SCTLR_Type_nTLSMD(state, tracer, s_16_0);
        // D s_16_2: write-var walkparams.21 <= s_16_1
        fn_state.walkparams._21 = s_16_1;
        // N s_16_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_17_0: const #90704u : u32
        let s_17_0: u32 = 90704;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_SCR_EL3_Type_SIF(s_17_1)
        let s_17_2: bool = u_get_SCR_EL3_Type_SIF(state, tracer, s_17_1);
        // D s_17_3: write-var sif <= s_17_2
        fn_state.sif = s_17_2;
        // N s_17_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call TTBCR_NS_read(s_18_0)
        let s_18_1: ProductType700c18a878c5601b = TTBCR_NS_read(state, tracer, s_18_0);
        // D s_18_2: write-var ttbcr <= s_18_1
        fn_state.ttbcr = s_18_1;
        // C s_18_3: const #() : ()
        let s_18_3: () = ();
        // S s_18_4: call TTBCR2_NS_read(s_18_3)
        let s_18_4: ProductType700c18a878c5601b = TTBCR2_NS_read(state, tracer, s_18_3);
        // D s_18_5: write-var ttbcr2 <= s_18_4
        fn_state.ttbcr2 = s_18_4;
        // C s_18_6: const #() : ()
        let s_18_6: () = ();
        // S s_18_7: call SCTLR_NS_read(s_18_6)
        let s_18_7: ProductType700c18a878c5601b = SCTLR_NS_read(state, tracer, s_18_6);
        // D s_18_8: write-var sctlr <= s_18_7
        fn_state.sctlr = s_18_7;
        // C s_18_9: const #() : ()
        let s_18_9: () = ();
        // S s_18_10: call MAIR1_NS_read(s_18_9)
        let s_18_10: ProductType700c18a878c5601b = MAIR1_NS_read(state, tracer, s_18_9);
        // D s_18_11: write-var ga#21219 <= s_18_10
        fn_state.ga_21219 = s_18_10;
        // D s_18_12: read-var ga#21219.0:struct
        let s_18_12: u32 = fn_state.ga_21219._0;
        // C s_18_13: const #() : ()
        let s_18_13: () = ();
        // S s_18_14: call MAIR0_NS_read(s_18_13)
        let s_18_14: ProductType700c18a878c5601b = MAIR0_NS_read(state, tracer, s_18_13);
        // D s_18_15: write-var ga#21220 <= s_18_14
        fn_state.ga_21220 = s_18_14;
        // D s_18_16: read-var ga#21220.0:struct
        let s_18_16: u32 = fn_state.ga_21220._0;
        // D s_18_17: cast zx s_18_12 -> bv
        let s_18_17: Bits = Bits::new(s_18_12 as u128, 32u16);
        // D s_18_18: cast zx s_18_16 -> bv
        let s_18_18: Bits = Bits::new(s_18_16 as u128, 32u16);
        // D s_18_19: cast reint s_18_17 -> u128
        let s_18_19: u128 = (s_18_17.value() as u128);
        // D s_18_20: size-of s_18_17
        let s_18_20: u16 = s_18_17.length();
        // D s_18_21: cast reint s_18_18 -> u128
        let s_18_21: u128 = (s_18_18.value() as u128);
        // D s_18_22: size-of s_18_18
        let s_18_22: u16 = s_18_18.length();
        // D s_18_23: lsl s_18_19 s_18_22
        let s_18_23: u128 = s_18_19 << s_18_22;
        // D s_18_24: or s_18_23 s_18_21
        let s_18_24: u128 = ((s_18_23) | (s_18_21));
        // D s_18_25: add s_18_20 s_18_22
        let s_18_25: u16 = (s_18_20 + s_18_22);
        // D s_18_26: create-bits s_18_24 s_18_25
        let s_18_26: Bits = Bits::new(s_18_24, s_18_25);
        // D s_18_27: cast reint s_18_26 -> u64
        let s_18_27: u64 = (s_18_26.value() as u64);
        // D s_18_28: write-var mair <= s_18_27
        fn_state.mair = s_18_27;
        // C s_18_29: const #20920u : u32
        let s_18_29: u32 = 20920;
        // D s_18_30: read-reg s_18_29:struct
        let s_18_30: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_18_29 as isize);
            tracer.read_register(s_18_29 as isize, value);
            value
        };
        // D s_18_31: call _get_SCR_Type_SIF(s_18_30)
        let s_18_31: bool = u_get_SCR_Type_SIF(state, tracer, s_18_30);
        // D s_18_32: write-var sif <= s_18_31
        fn_state.sif = s_18_31;
        // N s_18_33: jump b4
        return block_4(state, tracer, fn_state);
    }
}
