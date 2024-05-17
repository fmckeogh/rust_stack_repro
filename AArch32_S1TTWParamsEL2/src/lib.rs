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
use HTCR_read::*;
use u_get_HTCR_Type_T0SZ::*;
use u_get_HTCR_Type_IRGN0::*;
use u_get_HSCTLR_Type_nTLSMD::*;
use HMAIR1_read::*;
use Mk_MAIRType::*;
use u_get_HTCR_Type_SH0::*;
use HMAIR0_read::*;
use u_get_HSCTLR_Type_WXN::*;
use AArch32_HaveHPDExt::*;
use u_get_HTCR_Type_HPD::*;
use HaveTrapLoadStoreMultipleDeviceExt::*;
use u_get_HTCR_Type_ORGN0::*;
use HSCTLR_read::*;
use common::*;
pub fn AArch32_S1TTWParamsEL2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_27632: (),
) -> ProductTypeef284266e139aee2 {
    #[derive(Default)]
    struct FunctionState {
        ga_21285: ProductType700c18a878c5601b,
        walkparams: ProductTypeef284266e139aee2,
        ga_21286: ProductType700c18a878c5601b,
        gs_27632: (),
    }
    let fn_state = FunctionState {
        gs_27632,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // D s_0_1: write-var walkparams.36 <= s_0_0
        fn_state.walkparams._36 = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call HTCR_read(s_0_2)
        let s_0_3: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_0_2);
        // S s_0_4: call _get_HTCR_Type_T0SZ(s_0_3)
        let s_0_4: u8 = u_get_HTCR_Type_T0SZ(state, tracer, s_0_3);
        // D s_0_5: write-var walkparams.32 <= s_0_4
        fn_state.walkparams._32 = s_0_4;
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call HTCR_read(s_0_6)
        let s_0_7: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_0_6);
        // S s_0_8: call _get_HTCR_Type_SH0(s_0_7)
        let s_0_8: u8 = u_get_HTCR_Type_SH0(state, tracer, s_0_7);
        // D s_0_9: write-var walkparams.16 <= s_0_8
        fn_state.walkparams._16 = s_0_8;
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call HTCR_read(s_0_10)
        let s_0_11: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_0_10);
        // S s_0_12: call _get_HTCR_Type_IRGN0(s_0_11)
        let s_0_12: u8 = u_get_HTCR_Type_IRGN0(state, tracer, s_0_11);
        // D s_0_13: write-var walkparams.23 <= s_0_12
        fn_state.walkparams._23 = s_0_12;
        // C s_0_14: const #() : ()
        let s_0_14: () = ();
        // S s_0_15: call HTCR_read(s_0_14)
        let s_0_15: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_0_14);
        // S s_0_16: call _get_HTCR_Type_ORGN0(s_0_15)
        let s_0_16: u8 = u_get_HTCR_Type_ORGN0(state, tracer, s_0_15);
        // D s_0_17: write-var walkparams.29 <= s_0_16
        fn_state.walkparams._29 = s_0_16;
        // C s_0_18: const #() : ()
        let s_0_18: () = ();
        // S s_0_19: call AArch32_HaveHPDExt(s_0_18)
        let s_0_19: bool = AArch32_HaveHPDExt(state, tracer, s_0_18);
        // N s_0_20: branch s_0_19 b6 b1
        if s_0_19 {
            return block_6(state, tracer, fn_state);
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
        // D s_1_1: write-var walkparams.15 <= s_1_0
        fn_state.walkparams._15 = s_1_0;
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
        // S s_2_1: call HSCTLR_read(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_2_0);
        // S s_2_2: call _get_HSCTLR_Type_EE(s_2_1)
        let s_2_2: bool = u_get_HSCTLR_Type_EE(state, tracer, s_2_1);
        // D s_2_3: write-var walkparams.9 <= s_2_2
        fn_state.walkparams._9 = s_2_2;
        // C s_2_4: const #() : ()
        let s_2_4: () = ();
        // S s_2_5: call HSCTLR_read(s_2_4)
        let s_2_5: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_2_4);
        // S s_2_6: call _get_HSCTLR_Type_WXN(s_2_5)
        let s_2_6: bool = u_get_HSCTLR_Type_WXN(state, tracer, s_2_5);
        // D s_2_7: write-var walkparams.39 <= s_2_6
        fn_state.walkparams._39 = s_2_6;
        // C s_2_8: const #() : ()
        let s_2_8: () = ();
        // S s_2_9: call HaveTrapLoadStoreMultipleDeviceExt(s_2_8)
        let s_2_9: bool = HaveTrapLoadStoreMultipleDeviceExt(state, tracer, s_2_8);
        // N s_2_10: branch s_2_9 b5 b3
        if s_2_9 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var walkparams.21 <= s_3_0
        fn_state.walkparams._21 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call HMAIR1_read(s_4_0)
        let s_4_1: ProductType700c18a878c5601b = HMAIR1_read(state, tracer, s_4_0);
        // D s_4_2: write-var ga#21285 <= s_4_1
        fn_state.ga_21285 = s_4_1;
        // D s_4_3: read-var ga#21285.0:struct
        let s_4_3: u32 = fn_state.ga_21285._0;
        // C s_4_4: const #() : ()
        let s_4_4: () = ();
        // S s_4_5: call HMAIR0_read(s_4_4)
        let s_4_5: ProductType700c18a878c5601b = HMAIR0_read(state, tracer, s_4_4);
        // D s_4_6: write-var ga#21286 <= s_4_5
        fn_state.ga_21286 = s_4_5;
        // D s_4_7: read-var ga#21286.0:struct
        let s_4_7: u32 = fn_state.ga_21286._0;
        // D s_4_8: cast zx s_4_3 -> bv
        let s_4_8: Bits = Bits::new(s_4_3 as u128, 32u16);
        // D s_4_9: cast zx s_4_7 -> bv
        let s_4_9: Bits = Bits::new(s_4_7 as u128, 32u16);
        // D s_4_10: cast reint s_4_8 -> u128
        let s_4_10: u128 = (s_4_8.value() as u128);
        // D s_4_11: size-of s_4_8
        let s_4_11: u16 = s_4_8.length();
        // D s_4_12: cast reint s_4_9 -> u128
        let s_4_12: u128 = (s_4_9.value() as u128);
        // D s_4_13: size-of s_4_9
        let s_4_13: u16 = s_4_9.length();
        // D s_4_14: lsl s_4_10 s_4_13
        let s_4_14: u128 = s_4_10 << s_4_13;
        // D s_4_15: or s_4_14 s_4_12
        let s_4_15: u128 = ((s_4_14) | (s_4_12));
        // D s_4_16: add s_4_11 s_4_13
        let s_4_16: u16 = (s_4_11 + s_4_13);
        // D s_4_17: create-bits s_4_15 s_4_16
        let s_4_17: Bits = Bits::new(s_4_15, s_4_16);
        // D s_4_18: cast reint s_4_17 -> u64
        let s_4_18: u64 = (s_4_17.value() as u64);
        // D s_4_19: call Mk_MAIRType(s_4_18)
        let s_4_19: ProductType5c790c8ef59cc8b2 = Mk_MAIRType(state, tracer, s_4_18);
        // D s_4_20: write-var walkparams.17 <= s_4_19
        fn_state.walkparams._17 = s_4_19;
        // D s_4_21: read-var walkparams:struct
        let s_4_21: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // N s_4_22: return s_4_21
        return s_4_21;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call HSCTLR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_5_0);
        // S s_5_2: call _get_HSCTLR_Type_nTLSMD(s_5_1)
        let s_5_2: bool = u_get_HSCTLR_Type_nTLSMD(state, tracer, s_5_1);
        // D s_5_3: write-var walkparams.21 <= s_5_2
        fn_state.walkparams._21 = s_5_2;
        // N s_5_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HTCR_read(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = HTCR_read(state, tracer, s_6_0);
        // S s_6_2: call _get_HTCR_Type_HPD(s_6_1)
        let s_6_2: bool = u_get_HTCR_Type_HPD(state, tracer, s_6_1);
        // D s_6_3: write-var walkparams.15 <= s_6_2
        fn_state.walkparams._15 = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
