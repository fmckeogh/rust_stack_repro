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
use MAIR0_S_read::*;
use u_get_TTBCR_Type_T1SZ::*;
use u_get_SCR_Type_SIF::*;
use u_get_TTBCR_Type_IRGN1::*;
use Mk_MAIRType::*;
use u_get_SCTLR_Type_nTLSMD::*;
use AArch32_GetVARange::*;
use u_get_TTBCR_Type_ORGN1::*;
use AArch32_HaveHPDExt::*;
use u_get_TTBCR_Type_T2E::*;
use MAIR1_S_read::*;
use u_get_TTBCR2_Type_HPD1::*;
use HaveTrapLoadStoreMultipleDeviceExt::*;
use u_get_SCTLR_Type_UWXN::*;
use u_get_TTBCR_Type_T0SZ::*;
use u_get_TTBCR_Type_ORGN0::*;
use u_get_TTBCR_Type_SH1::*;
use u_get_TTBCR_Type_EAE::*;
use u_get_SCTLR_Type_EE::*;
use u_get_TTBCR_Type_SH0::*;
use u_get_TTBCR_Type_IRGN0::*;
use u_get_TTBCR2_Type_HPD0::*;
use u_get_SCTLR_Type_WXN::*;
use common::*;
pub fn AArch32_S1TTWParamsEL30<T: Tracer>(
    state: &mut State,
    tracer: &T,
    va: u32,
) -> ProductTypeef284266e139aee2 {
    #[derive(Default)]
    struct FunctionState {
        ga_21293: ProductType700c18a878c5601b,
        ga_21294: ProductType700c18a878c5601b,
        walkparams: ProductTypeef284266e139aee2,
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
        // C s_0_0: const #15368u : u32
        let s_0_0: u32 = 15368;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_TTBCR_Type_EAE(s_0_1)
        let s_0_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // C s_0_4: const #1u : u8
        let s_0_4: bool = true;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 1u16);
        // D s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // N s_0_7: assert s_0_6
        let s_0_7: () = assert!(s_0_6);
        // C s_0_8: const #15368u : u32
        let s_0_8: u32 = 15368;
        // D s_0_9: read-reg s_0_8:struct
        let s_0_9: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // D s_0_10: call _get_TTBCR_Type_T0SZ(s_0_9)
        let s_0_10: u8 = u_get_TTBCR_Type_T0SZ(state, tracer, s_0_9);
        // D s_0_11: write-var walkparams.32 <= s_0_10
        fn_state.walkparams._32 = s_0_10;
        // C s_0_12: const #15368u : u32
        let s_0_12: u32 = 15368;
        // D s_0_13: read-reg s_0_12:struct
        let s_0_13: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_12 as isize);
            tracer.read_register(s_0_12 as isize, value);
            value
        };
        // D s_0_14: call _get_TTBCR_Type_T1SZ(s_0_13)
        let s_0_14: u8 = u_get_TTBCR_Type_T1SZ(state, tracer, s_0_13);
        // D s_0_15: write-var walkparams.33 <= s_0_14
        fn_state.walkparams._33 = s_0_14;
        // C s_0_16: const #16456u : u32
        let s_0_16: u32 = 16456;
        // D s_0_17: read-reg s_0_16:struct
        let s_0_17: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_16 as isize);
            tracer.read_register(s_0_16 as isize, value);
            value
        };
        // D s_0_18: call _get_SCTLR_Type_EE(s_0_17)
        let s_0_18: bool = u_get_SCTLR_Type_EE(state, tracer, s_0_17);
        // D s_0_19: write-var walkparams.9 <= s_0_18
        fn_state.walkparams._9 = s_0_18;
        // C s_0_20: const #16456u : u32
        let s_0_20: u32 = 16456;
        // D s_0_21: read-reg s_0_20:struct
        let s_0_21: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_20 as isize);
            tracer.read_register(s_0_20 as isize, value);
            value
        };
        // D s_0_22: call _get_SCTLR_Type_WXN(s_0_21)
        let s_0_22: bool = u_get_SCTLR_Type_WXN(state, tracer, s_0_21);
        // D s_0_23: write-var walkparams.39 <= s_0_22
        fn_state.walkparams._39 = s_0_22;
        // C s_0_24: const #16456u : u32
        let s_0_24: u32 = 16456;
        // D s_0_25: read-reg s_0_24:struct
        let s_0_25: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_24 as isize);
            tracer.read_register(s_0_24 as isize, value);
            value
        };
        // D s_0_26: call _get_SCTLR_Type_UWXN(s_0_25)
        let s_0_26: bool = u_get_SCTLR_Type_UWXN(state, tracer, s_0_25);
        // D s_0_27: write-var walkparams.38 <= s_0_26
        fn_state.walkparams._38 = s_0_26;
        // C s_0_28: const #() : ()
        let s_0_28: () = ();
        // S s_0_29: call HaveTrapLoadStoreMultipleDeviceExt(s_0_28)
        let s_0_29: bool = HaveTrapLoadStoreMultipleDeviceExt(state, tracer, s_0_28);
        // N s_0_30: branch s_0_29 b12 b1
        if s_0_29 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_1_0: const #1u : u8
        let s_1_0: bool = true;
        // D s_1_1: write-var walkparams.21 <= s_1_0
        fn_state.walkparams._21 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call MAIR1_S_read(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = MAIR1_S_read(state, tracer, s_2_0);
        // D s_2_2: write-var ga#21293 <= s_2_1
        fn_state.ga_21293 = s_2_1;
        // D s_2_3: read-var ga#21293.0:struct
        let s_2_3: u32 = fn_state.ga_21293._0;
        // C s_2_4: const #() : ()
        let s_2_4: () = ();
        // S s_2_5: call MAIR0_S_read(s_2_4)
        let s_2_5: ProductType700c18a878c5601b = MAIR0_S_read(state, tracer, s_2_4);
        // D s_2_6: write-var ga#21294 <= s_2_5
        fn_state.ga_21294 = s_2_5;
        // D s_2_7: read-var ga#21294.0:struct
        let s_2_7: u32 = fn_state.ga_21294._0;
        // D s_2_8: cast zx s_2_3 -> bv
        let s_2_8: Bits = Bits::new(s_2_3 as u128, 32u16);
        // D s_2_9: cast zx s_2_7 -> bv
        let s_2_9: Bits = Bits::new(s_2_7 as u128, 32u16);
        // D s_2_10: cast reint s_2_8 -> u128
        let s_2_10: u128 = (s_2_8.value() as u128);
        // D s_2_11: size-of s_2_8
        let s_2_11: u16 = s_2_8.length();
        // D s_2_12: cast reint s_2_9 -> u128
        let s_2_12: u128 = (s_2_9.value() as u128);
        // D s_2_13: size-of s_2_9
        let s_2_13: u16 = s_2_9.length();
        // D s_2_14: lsl s_2_10 s_2_13
        let s_2_14: u128 = s_2_10 << s_2_13;
        // D s_2_15: or s_2_14 s_2_12
        let s_2_15: u128 = ((s_2_14) | (s_2_12));
        // D s_2_16: add s_2_11 s_2_13
        let s_2_16: u16 = (s_2_11 + s_2_13);
        // D s_2_17: create-bits s_2_15 s_2_16
        let s_2_17: Bits = Bits::new(s_2_15, s_2_16);
        // D s_2_18: cast reint s_2_17 -> u64
        let s_2_18: u64 = (s_2_17.value() as u64);
        // D s_2_19: call Mk_MAIRType(s_2_18)
        let s_2_19: ProductType5c790c8ef59cc8b2 = Mk_MAIRType(state, tracer, s_2_18);
        // D s_2_20: write-var walkparams.17 <= s_2_19
        fn_state.walkparams._17 = s_2_19;
        // C s_2_21: const #20920u : u32
        let s_2_21: u32 = 20920;
        // D s_2_22: read-reg s_2_21:struct
        let s_2_22: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_21 as isize);
            tracer.read_register(s_2_21 as isize, value);
            value
        };
        // D s_2_23: call _get_SCR_Type_SIF(s_2_22)
        let s_2_23: bool = u_get_SCR_Type_SIF(state, tracer, s_2_22);
        // D s_2_24: write-var walkparams.30 <= s_2_23
        fn_state.walkparams._30 = s_2_23;
        // D s_2_25: read-var walkparams.32:struct
        let s_2_25: u8 = fn_state.walkparams._32;
        // D s_2_26: read-var walkparams.33:struct
        let s_2_26: u8 = fn_state.walkparams._33;
        // D s_2_27: read-var va:u32
        let s_2_27: u32 = fn_state.va;
        // D s_2_28: call AArch32_GetVARange(s_2_27, s_2_25, s_2_26)
        let s_2_28: u32 = AArch32_GetVARange(state, tracer, s_2_27, s_2_25, s_2_26);
        // C s_2_29: const #0u : u32
        let s_2_29: u32 = 0;
        // D s_2_30: cmp-eq s_2_28 s_2_29
        let s_2_30: bool = ((s_2_28) == (s_2_29));
        // N s_2_31: branch s_2_30 b8 b3
        if s_2_30 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_3_0: const #15368u : u32
        let s_3_0: u32 = 15368;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call _get_TTBCR_Type_SH1(s_3_1)
        let s_3_2: u8 = u_get_TTBCR_Type_SH1(state, tracer, s_3_1);
        // D s_3_3: write-var walkparams.29 <= s_3_2
        fn_state.walkparams._29 = s_3_2;
        // C s_3_4: const #15368u : u32
        let s_3_4: u32 = 15368;
        // D s_3_5: read-reg s_3_4:struct
        let s_3_5: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_3_4 as isize);
            tracer.read_register(s_3_4 as isize, value);
            value
        };
        // D s_3_6: call _get_TTBCR_Type_IRGN1(s_3_5)
        let s_3_6: u8 = u_get_TTBCR_Type_IRGN1(state, tracer, s_3_5);
        // D s_3_7: write-var walkparams.16 <= s_3_6
        fn_state.walkparams._16 = s_3_6;
        // C s_3_8: const #15368u : u32
        let s_3_8: u32 = 15368;
        // D s_3_9: read-reg s_3_8:struct
        let s_3_9: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_3_8 as isize);
            tracer.read_register(s_3_8 as isize, value);
            value
        };
        // D s_3_10: call _get_TTBCR_Type_ORGN1(s_3_9)
        let s_3_10: u8 = u_get_TTBCR_Type_ORGN1(state, tracer, s_3_9);
        // D s_3_11: write-var walkparams.23 <= s_3_10
        fn_state.walkparams._23 = s_3_10;
        // C s_3_12: const #() : ()
        let s_3_12: () = ();
        // S s_3_13: call AArch32_HaveHPDExt(s_3_12)
        let s_3_13: bool = AArch32_HaveHPDExt(state, tracer, s_3_12);
        // N s_3_14: branch s_3_13 b7 b4
        if s_3_13 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var walkparams.15 <= s_4_0
        fn_state.walkparams._15 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_6_0: read-var walkparams:struct
        let s_6_0: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_7_0: const #15368u : u32
        let s_7_0: u32 = 15368;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_TTBCR_Type_T2E(s_7_1)
        let s_7_2: bool = u_get_TTBCR_Type_T2E(state, tracer, s_7_1);
        // C s_7_3: const #20352u : u32
        let s_7_3: u32 = 20352;
        // D s_7_4: read-reg s_7_3:struct
        let s_7_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: call _get_TTBCR2_Type_HPD1(s_7_4)
        let s_7_5: bool = u_get_TTBCR2_Type_HPD1(state, tracer, s_7_4);
        // D s_7_6: cast zx s_7_2 -> bv
        let s_7_6: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_7: cast zx s_7_5 -> bv
        let s_7_7: Bits = Bits::new(s_7_5 as u128, 1u16);
        // D s_7_8: and s_7_6 s_7_7
        let s_7_8: Bits = ((s_7_6) & (s_7_7));
        // D s_7_9: cast reint s_7_8 -> u8
        let s_7_9: bool = ((s_7_8.value()) != 0);
        // D s_7_10: write-var walkparams.15 <= s_7_9
        fn_state.walkparams._15 = s_7_9;
        // N s_7_11: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_8_0: const #15368u : u32
        let s_8_0: u32 = 15368;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_TTBCR_Type_SH0(s_8_1)
        let s_8_2: u8 = u_get_TTBCR_Type_SH0(state, tracer, s_8_1);
        // D s_8_3: write-var walkparams.29 <= s_8_2
        fn_state.walkparams._29 = s_8_2;
        // C s_8_4: const #15368u : u32
        let s_8_4: u32 = 15368;
        // D s_8_5: read-reg s_8_4:struct
        let s_8_5: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_4 as isize);
            tracer.read_register(s_8_4 as isize, value);
            value
        };
        // D s_8_6: call _get_TTBCR_Type_IRGN0(s_8_5)
        let s_8_6: u8 = u_get_TTBCR_Type_IRGN0(state, tracer, s_8_5);
        // D s_8_7: write-var walkparams.16 <= s_8_6
        fn_state.walkparams._16 = s_8_6;
        // C s_8_8: const #15368u : u32
        let s_8_8: u32 = 15368;
        // D s_8_9: read-reg s_8_8:struct
        let s_8_9: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_8 as isize);
            tracer.read_register(s_8_8 as isize, value);
            value
        };
        // D s_8_10: call _get_TTBCR_Type_ORGN0(s_8_9)
        let s_8_10: u8 = u_get_TTBCR_Type_ORGN0(state, tracer, s_8_9);
        // D s_8_11: write-var walkparams.23 <= s_8_10
        fn_state.walkparams._23 = s_8_10;
        // C s_8_12: const #() : ()
        let s_8_12: () = ();
        // S s_8_13: call AArch32_HaveHPDExt(s_8_12)
        let s_8_13: bool = AArch32_HaveHPDExt(state, tracer, s_8_12);
        // N s_8_14: branch s_8_13 b11 b9
        if s_8_13 {
            return block_11(state, tracer, fn_state);
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
        // D s_9_1: write-var walkparams.15 <= s_9_0
        fn_state.walkparams._15 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_10_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_11_0: const #15368u : u32
        let s_11_0: u32 = 15368;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_TTBCR_Type_T2E(s_11_1)
        let s_11_2: bool = u_get_TTBCR_Type_T2E(state, tracer, s_11_1);
        // C s_11_3: const #20352u : u32
        let s_11_3: u32 = 20352;
        // D s_11_4: read-reg s_11_3:struct
        let s_11_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_11_3 as isize);
            tracer.read_register(s_11_3 as isize, value);
            value
        };
        // D s_11_5: call _get_TTBCR2_Type_HPD0(s_11_4)
        let s_11_5: bool = u_get_TTBCR2_Type_HPD0(state, tracer, s_11_4);
        // D s_11_6: cast zx s_11_2 -> bv
        let s_11_6: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_7: cast zx s_11_5 -> bv
        let s_11_7: Bits = Bits::new(s_11_5 as u128, 1u16);
        // D s_11_8: and s_11_6 s_11_7
        let s_11_8: Bits = ((s_11_6) & (s_11_7));
        // D s_11_9: cast reint s_11_8 -> u8
        let s_11_9: bool = ((s_11_8.value()) != 0);
        // D s_11_10: write-var walkparams.15 <= s_11_9
        fn_state.walkparams._15 = s_11_9;
        // N s_11_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_12_0: const #16456u : u32
        let s_12_0: u32 = 16456;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_SCTLR_Type_nTLSMD(s_12_1)
        let s_12_2: bool = u_get_SCTLR_Type_nTLSMD(state, tracer, s_12_1);
        // D s_12_3: write-var walkparams.21 <= s_12_2
        fn_state.walkparams._21 = s_12_2;
        // N s_12_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
