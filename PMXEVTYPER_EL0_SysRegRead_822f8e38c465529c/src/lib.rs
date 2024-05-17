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
use u_get_MDCR_EL2_Type_TPM::*;
use u__get_PMCCFILTR_EL0::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_PMUSERENR_EL0_Type_UEN::*;
use Halted::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use ConstrainUnpredictableProcedure::*;
use u_get_PMSELR_EL0_Type_SEL::*;
use u_get_MDCR_EL3_Type_TPM::*;
use neq_int::*;
use u__get_PMEVTYPER_EL0::*;
use X_set::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use AArch64_GetNumEventCountersAccessible::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HDFGRTR_EL2_Type_PMEVTYPERn_EL0::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use EDSCR_read::*;
use common::*;
pub fn PMXEVTYPER_EL0_SysRegRead_822f8e38c465529c<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_68854: bool,
        gs_68841: bool,
        gs_68882: bool,
        gs_68843: bool,
        ga_83980: ProductType5c790c8ef59cc8b2,
        ga_83890: ProductType5c790c8ef59cc8b2,
        gs_68872: bool,
        gs_68851: bool,
        u__MDCR_EL3_TPM: bool,
        gs_68860: bool,
        ga_83974: ProductType5c790c8ef59cc8b2,
        gs_68871: bool,
        gs_68858: bool,
        ga_83896: ProductType5c790c8ef59cc8b2,
        gs_68852: bool,
        ga_83961: ProductType5c790c8ef59cc8b2,
        gs_68844: bool,
        gs_68866: bool,
        gs_68876: bool,
        gs_68857: bool,
        gs_68845: bool,
        gs_68879: bool,
        gs_68870: bool,
        gs_68873: bool,
        u__PSTATE_EL: u8,
        gs_68861: bool,
        u__MDCR_EL2_TPM: bool,
        gs_68856: bool,
        gs_68888: bool,
        gs_68874: bool,
        gs_68867: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        gs_68853: bool,
        u__SCR_EL3_FGTEn: bool,
        u__PMUSERENR_EL0_EN: bool,
        gs_68869: bool,
        ga_83936: ProductType5c790c8ef59cc8b2,
        gs_68862: bool,
        gs_68881: bool,
        gs_68850: bool,
        ga_83942: ProductType5c790c8ef59cc8b2,
        gs_68842: bool,
        gs_68837: bool,
        gs_68877: bool,
        gs_68855: bool,
        ga_83967: ProductType5c790c8ef59cc8b2,
        gs_68887: bool,
        u__HDFGRTR_EL2_PMEVTYPERn_EL0: bool,
        gs_68849: bool,
        gs_68883: bool,
        gs_68875: bool,
        gs_68868: bool,
        gs_68878: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var __PSTATE_EL <= s_0_1
        fn_state.u__PSTATE_EL = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call EDSCR_read(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_3);
        // S s_0_5: call _get_EDSCR_Type_SDD(s_0_4)
        let s_0_5: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_4);
        // D s_0_6: write-var __EDSCR_SDD <= s_0_5
        fn_state.u__EDSCR_SDD = s_0_5;
        // C s_0_7: const #22712u : u32
        let s_0_7: u32 = 22712;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDCR_EL3_Type_TPM(s_0_8)
        let s_0_9: bool = u_get_MDCR_EL3_Type_TPM(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL3_TPM <= s_0_9
        fn_state.u__MDCR_EL3_TPM = s_0_9;
        // C s_0_11: const #102624u : u32
        let s_0_11: u32 = 102624;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_PMUSERENR_EL0_Type_EN(s_0_12)
        let s_0_13: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_0_12);
        // D s_0_14: write-var __PMUSERENR_EL0_EN <= s_0_13
        fn_state.u__PMUSERENR_EL0_EN = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_TGE(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_TGE <= s_0_17
        fn_state.u__HCR_EL2_TGE = s_0_17;
        // C s_0_19: const #90704u : u32
        let s_0_19: u32 = 90704;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_SCR_EL3_Type_FGTEn(s_0_20)
        let s_0_21: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_20);
        // D s_0_22: write-var __SCR_EL3_FGTEn <= s_0_21
        fn_state.u__SCR_EL3_FGTEn = s_0_21;
        // C s_0_23: const #19144u : u32
        let s_0_23: u32 = 19144;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HDFGRTR_EL2_Type_PMEVTYPERn_EL0(s_0_24)
        let s_0_25: bool = u_get_HDFGRTR_EL2_Type_PMEVTYPERn_EL0(state, tracer, s_0_24);
        // D s_0_26: write-var __HDFGRTR_EL2_PMEVTYPERn_EL0 <= s_0_25
        fn_state.u__HDFGRTR_EL2_PMEVTYPERn_EL0 = s_0_25;
        // C s_0_27: const #104880u : u32
        let s_0_27: u32 = 104880;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_MDCR_EL2_Type_TPM(s_0_28)
        let s_0_29: bool = u_get_MDCR_EL2_Type_TPM(state, tracer, s_0_28);
        // D s_0_30: write-var __MDCR_EL2_TPM <= s_0_29
        fn_state.u__MDCR_EL2_TPM = s_0_29;
        // C s_0_31: const #19136u : u32
        let s_0_31: u32 = 19136;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_PMSELR_EL0_Type_SEL(s_0_32)
        let s_0_33: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_0_32);
        // D s_0_34: cast zx s_0_33 -> bv
        let s_0_34: Bits = Bits::new(s_0_33 as u128, 5u16);
        // D s_0_35: cast zx s_0_34 -> i
        let s_0_35: i128 = (s_0_34.value() as i128);
        // D s_0_36: cast reint s_0_35 -> i64
        let s_0_36: i64 = (s_0_35 as i64);
        // C s_0_37: const #31s : i
        let s_0_37: i128 = 31;
        // D s_0_38: cast zx s_0_36 -> i
        let s_0_38: i128 = (i128::try_from(s_0_36).unwrap());
        // D s_0_39: call neq_int(s_0_38, s_0_37)
        let s_0_39: bool = neq_int(state, tracer, s_0_38, s_0_37);
        // N s_0_40: branch s_0_39 b172 b1
        if s_0_39 {
            return block_172(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#68837 <= s_1_0
        fn_state.gs_68837 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#68837:u8
        let s_2_0: bool = fn_state.gs_68837;
        // N s_2_1: branch s_2_0 b169 b3
        if s_2_0 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var __PSTATE_EL:u8
        let s_3_0: u8 = fn_state.u__PSTATE_EL;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #448u : u32
        let s_3_2: u32 = 448;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // N s_3_6: branch s_3_5 b94 b4
        if s_3_5 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var __PSTATE_EL:u8
        let s_4_0: u8 = fn_state.u__PSTATE_EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #440u : u32
        let s_4_2: u32 = 440;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b38 b5
        if s_4_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var __PSTATE_EL:u8
        let s_5_0: u8 = fn_state.u__PSTATE_EL;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #432u : u32
        let s_5_2: u32 = 432;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // N s_5_6: branch s_5_5 b11 b6
        if s_5_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __PSTATE_EL:u8
        let s_6_0: u8 = fn_state.u__PSTATE_EL;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #424u : u32
        let s_6_2: u32 = 424;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #19136u : u32
        let s_8_0: u32 = 19136;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_PMSELR_EL0_Type_SEL(s_8_1)
        let s_8_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_8_1);
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 5u16);
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (s_8_3.value() as i128);
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // C s_8_6: const #31s : i
        let s_8_6: i128 = 31;
        // D s_8_7: cast zx s_8_5 -> i
        let s_8_7: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_8: cmp-eq s_8_7 s_8_6
        let s_8_8: bool = ((s_8_7) == (s_8_6));
        // N s_8_9: branch s_8_8 b10 b9
        if s_8_8 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // C s_9_1: const #19136u : u32
        let s_9_1: u32 = 19136;
        // D s_9_2: read-reg s_9_1:struct
        let s_9_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_1 as isize);
            tracer.read_register(s_9_1 as isize, value);
            value
        };
        // D s_9_3: call _get_PMSELR_EL0_Type_SEL(s_9_2)
        let s_9_3: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_9_2);
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 5u16);
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (s_9_4.value() as i128);
        // D s_9_6: cast reint s_9_5 -> i64
        let s_9_6: i64 = (s_9_5 as i64);
        // C s_9_7: const #11208u : u32
        let s_9_7: u32 = 11208;
        // D s_9_8: read-reg s_9_7:[struct; 32]
        let s_9_8: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 32usize]>(s_9_7 as isize);
            tracer.read_register(s_9_7 as isize, value);
            value
        };
        // D s_9_9: cast zx s_9_6 -> i
        let s_9_9: i128 = (i128::try_from(s_9_6).unwrap());
        // D s_9_10: read-element s_9_8[s_9_9]
        let s_9_10: ProductType5c790c8ef59cc8b2 = s_9_8[(s_9_9) as usize];
        // D s_9_11: call __get_PMEVTYPER_EL0(s_9_10)
        let s_9_11: ProductType5c790c8ef59cc8b2 = u__get_PMEVTYPER_EL0(
            state,
            tracer,
            s_9_10,
        );
        // D s_9_12: write-var ga#83980 <= s_9_11
        fn_state.ga_83980 = s_9_11;
        // D s_9_13: read-var ga#83980.0:struct
        let s_9_13: u64 = fn_state.ga_83980._0;
        // D s_9_14: cast zx s_9_13 -> bv
        let s_9_14: Bits = Bits::new(s_9_13 as u128, 64u16);
        // D s_9_15: read-var t:i
        let s_9_15: i128 = fn_state.t;
        // D s_9_16: call X_set(s_9_15, s_9_0, s_9_14)
        let s_9_16: () = X_set(state, tracer, s_9_15, s_9_0, s_9_14);
        // N s_9_17: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // C s_10_1: const #15864u : u32
        let s_10_1: u32 = 15864;
        // D s_10_2: read-reg s_10_1:struct
        let s_10_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_1 as isize);
            tracer.read_register(s_10_1 as isize, value);
            value
        };
        // D s_10_3: call __get_PMCCFILTR_EL0(s_10_2)
        let s_10_3: ProductType5c790c8ef59cc8b2 = u__get_PMCCFILTR_EL0(
            state,
            tracer,
            s_10_2,
        );
        // D s_10_4: write-var ga#83974 <= s_10_3
        fn_state.ga_83974 = s_10_3;
        // D s_10_5: read-var ga#83974.0:struct
        let s_10_5: u64 = fn_state.ga_83974._0;
        // D s_10_6: cast zx s_10_5 -> bv
        let s_10_6: Bits = Bits::new(s_10_5 as u128, 64u16);
        // D s_10_7: read-var t:i
        let s_10_7: i128 = fn_state.t;
        // D s_10_8: call X_set(s_10_7, s_10_0, s_10_6)
        let s_10_8: () = X_set(state, tracer, s_10_7, s_10_0, s_10_6);
        // N s_10_9: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call Halted(s_11_0)
        let s_11_1: bool = Halted(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b37 b12
        if s_11_1 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#68841 <= s_12_0
        fn_state.gs_68841 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#68841:u8
        let s_13_0: bool = fn_state.gs_68841;
        // N s_13_1: branch s_13_0 b36 b14
        if s_13_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#68842 <= s_14_0
        fn_state.gs_68842 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#68842:u8
        let s_15_0: bool = fn_state.gs_68842;
        // N s_15_1: branch s_15_0 b35 b16
        if s_15_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#68843 <= s_16_0
        fn_state.gs_68843 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#68843:u8
        let s_17_0: bool = fn_state.gs_68843;
        // N s_17_1: branch s_17_0 b34 b18
        if s_17_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#68844 <= s_18_0
        fn_state.gs_68844 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#68844:u8
        let s_19_0: bool = fn_state.gs_68844;
        // N s_19_1: branch s_19_0 b33 b20
        if s_19_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #424u : u32
        let s_20_0: u32 = 424;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: u8 = {
            let value = state.read_register::<u8>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // C s_20_2: const #2u : u8
        let s_20_2: u8 = 2;
        // D s_20_3: cmp-lt s_20_1 s_20_2
        let s_20_3: bool = ((s_20_1) < (s_20_2));
        // N s_20_4: branch s_20_3 b32 b21
        if s_20_3 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#68845 <= s_21_0
        fn_state.gs_68845 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#68845:u8
        let s_22_0: bool = fn_state.gs_68845;
        // N s_22_1: branch s_22_0 b26 b23
        if s_22_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #19136u : u32
        let s_23_0: u32 = 19136;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_PMSELR_EL0_Type_SEL(s_23_1)
        let s_23_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 5u16);
        // D s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (s_23_3.value() as i128);
        // D s_23_5: cast reint s_23_4 -> i64
        let s_23_5: i64 = (s_23_4 as i64);
        // C s_23_6: const #31s : i
        let s_23_6: i128 = 31;
        // D s_23_7: cast zx s_23_5 -> i
        let s_23_7: i128 = (i128::try_from(s_23_5).unwrap());
        // D s_23_8: cmp-eq s_23_7 s_23_6
        let s_23_8: bool = ((s_23_7) == (s_23_6));
        // N s_23_9: branch s_23_8 b25 b24
        if s_23_8 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #64s : i64
        let s_24_0: i64 = 64;
        // C s_24_1: const #19136u : u32
        let s_24_1: u32 = 19136;
        // D s_24_2: read-reg s_24_1:struct
        let s_24_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_1 as isize);
            tracer.read_register(s_24_1 as isize, value);
            value
        };
        // D s_24_3: call _get_PMSELR_EL0_Type_SEL(s_24_2)
        let s_24_3: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_24_2);
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 5u16);
        // D s_24_5: cast zx s_24_4 -> i
        let s_24_5: i128 = (s_24_4.value() as i128);
        // D s_24_6: cast reint s_24_5 -> i64
        let s_24_6: i64 = (s_24_5 as i64);
        // C s_24_7: const #11208u : u32
        let s_24_7: u32 = 11208;
        // D s_24_8: read-reg s_24_7:[struct; 32]
        let s_24_8: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_24_7 as isize);
            tracer.read_register(s_24_7 as isize, value);
            value
        };
        // D s_24_9: cast zx s_24_6 -> i
        let s_24_9: i128 = (i128::try_from(s_24_6).unwrap());
        // D s_24_10: read-element s_24_8[s_24_9]
        let s_24_10: ProductType5c790c8ef59cc8b2 = s_24_8[(s_24_9) as usize];
        // D s_24_11: call __get_PMEVTYPER_EL0(s_24_10)
        let s_24_11: ProductType5c790c8ef59cc8b2 = u__get_PMEVTYPER_EL0(
            state,
            tracer,
            s_24_10,
        );
        // D s_24_12: write-var ga#83967 <= s_24_11
        fn_state.ga_83967 = s_24_11;
        // D s_24_13: read-var ga#83967.0:struct
        let s_24_13: u64 = fn_state.ga_83967._0;
        // D s_24_14: cast zx s_24_13 -> bv
        let s_24_14: Bits = Bits::new(s_24_13 as u128, 64u16);
        // D s_24_15: read-var t:i
        let s_24_15: i128 = fn_state.t;
        // D s_24_16: call X_set(s_24_15, s_24_0, s_24_14)
        let s_24_16: () = X_set(state, tracer, s_24_15, s_24_0, s_24_14);
        // N s_24_17: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #64s : i64
        let s_25_0: i64 = 64;
        // C s_25_1: const #15864u : u32
        let s_25_1: u32 = 15864;
        // D s_25_2: read-reg s_25_1:struct
        let s_25_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_1 as isize);
            tracer.read_register(s_25_1 as isize, value);
            value
        };
        // D s_25_3: call __get_PMCCFILTR_EL0(s_25_2)
        let s_25_3: ProductType5c790c8ef59cc8b2 = u__get_PMCCFILTR_EL0(
            state,
            tracer,
            s_25_2,
        );
        // D s_25_4: write-var ga#83961 <= s_25_3
        fn_state.ga_83961 = s_25_3;
        // D s_25_5: read-var ga#83961.0:struct
        let s_25_5: u64 = fn_state.ga_83961._0;
        // D s_25_6: cast zx s_25_5 -> bv
        let s_25_6: Bits = Bits::new(s_25_5 as u128, 64u16);
        // D s_25_7: read-var t:i
        let s_25_7: i128 = fn_state.t;
        // D s_25_8: call X_set(s_25_7, s_25_0, s_25_6)
        let s_25_8: () = X_set(state, tracer, s_25_7, s_25_0, s_25_6);
        // N s_25_9: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call Halted(s_26_0)
        let s_26_1: bool = Halted(state, tracer, s_26_0);
        // N s_26_2: branch s_26_1 b31 b27
        if s_26_1 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#68849 <= s_27_0
        fn_state.gs_68849 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#68849:u8
        let s_28_0: bool = fn_state.gs_68849;
        // N s_28_1: branch s_28_0 b30 b29
        if s_28_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #24u : u8
        let s_29_0: u8 = 24;
        // C s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 8u16);
        // C s_29_2: cast zx s_29_1 -> i
        let s_29_2: i128 = (s_29_1.value() as i128);
        // C s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // C s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // C s_29_5: const #424u : u32
        let s_29_5: u32 = 424;
        // D s_29_6: read-reg s_29_5:u8
        let s_29_6: u8 = {
            let value = state.read_register::<u8>(s_29_5 as isize);
            tracer.read_register(s_29_5 as isize, value);
            value
        };
        // D s_29_7: call AArch64_SystemAccessTrap(s_29_6, s_29_4)
        let s_29_7: () = AArch64_SystemAccessTrap(state, tracer, s_29_6, s_29_4);
        // N s_29_8: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var __EDSCR_SDD:u8
        let s_31_0: bool = fn_state.u__EDSCR_SDD;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#68849 <= s_31_4
        fn_state.gs_68849 = s_31_4;
        // N s_31_6: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __MDCR_EL3_TPM:u8
        let s_32_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#68845 <= s_32_4
        fn_state.gs_68845 = s_32_4;
        // N s_32_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: panic
        panic!("{:?}", ());
        // N s_33_1: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var __MDCR_EL3_TPM:u8
        let s_34_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: write-var gs#68844 <= s_34_4
        fn_state.gs_68844 = s_34_4;
        // N s_34_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_35_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_35_1: call __IMPDEF_boolean(s_35_0)
        let s_35_1: bool = u__IMPDEF_boolean(state, tracer, s_35_0);
        // D s_35_2: write-var gs#68843 <= s_35_1
        fn_state.gs_68843 = s_35_1;
        // N s_35_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var __EDSCR_SDD:u8
        let s_36_0: bool = fn_state.u__EDSCR_SDD;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 1u16);
        // C s_36_2: const #1u : u8
        let s_36_2: bool = true;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: write-var gs#68842 <= s_36_4
        fn_state.gs_68842 = s_36_4;
        // N s_36_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #424u : u32
        let s_37_0: u32 = 424;
        // D s_37_1: read-reg s_37_0:u8
        let s_37_1: u8 = {
            let value = state.read_register::<u8>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // C s_37_2: const #2u : u8
        let s_37_2: u8 = 2;
        // D s_37_3: cmp-lt s_37_1 s_37_2
        let s_37_3: bool = ((s_37_1) < (s_37_2));
        // D s_37_4: write-var gs#68841 <= s_37_3
        fn_state.gs_68841 = s_37_3;
        // N s_37_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call Halted(s_38_0)
        let s_38_1: bool = Halted(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b93 b39
        if s_38_1 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#68850 <= s_39_0
        fn_state.gs_68850 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#68850:u8
        let s_40_0: bool = fn_state.gs_68850;
        // N s_40_1: branch s_40_0 b92 b41
        if s_40_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#68851 <= s_41_0
        fn_state.gs_68851 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#68851:u8
        let s_42_0: bool = fn_state.gs_68851;
        // N s_42_1: branch s_42_0 b91 b43
        if s_42_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#68852 <= s_43_0
        fn_state.gs_68852 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#68852:u8
        let s_44_0: bool = fn_state.gs_68852;
        // N s_44_1: branch s_44_0 b90 b45
        if s_44_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#68853 <= s_45_0
        fn_state.gs_68853 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#68853:u8
        let s_46_0: bool = fn_state.gs_68853;
        // N s_46_1: branch s_46_0 b89 b47
        if s_46_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EL2Enabled(s_47_0)
        let s_47_1: bool = EL2Enabled(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b88 b48
        if s_47_1 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#68854 <= s_48_0
        fn_state.gs_68854 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#68854:u8
        let s_49_0: bool = fn_state.gs_68854;
        // N s_49_1: branch s_49_0 b84 b50
        if s_49_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#68856 <= s_50_0
        fn_state.gs_68856 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#68856:u8
        let s_51_0: bool = fn_state.gs_68856;
        // N s_51_1: branch s_51_0 b83 b52
        if s_51_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#68857 <= s_52_0
        fn_state.gs_68857 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#68857:u8
        let s_53_0: bool = fn_state.gs_68857;
        // N s_53_1: branch s_53_0 b82 b54
        if s_53_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call EL2Enabled(s_54_0)
        let s_54_1: bool = EL2Enabled(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b81 b55
        if s_54_1 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#68858 <= s_55_0
        fn_state.gs_68858 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#68858:u8
        let s_56_0: bool = fn_state.gs_68858;
        // N s_56_1: branch s_56_0 b80 b57
        if s_56_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call EL2Enabled(s_57_0)
        let s_57_1: bool = EL2Enabled(state, tracer, s_57_0);
        // N s_57_2: branch s_57_1 b79 b58
        if s_57_1 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#68860 <= s_58_0
        fn_state.gs_68860 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#68860:u8
        let s_59_0: bool = fn_state.gs_68860;
        // N s_59_1: branch s_59_0 b78 b60
        if s_59_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#68861 <= s_60_0
        fn_state.gs_68861 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#68861:u8
        let s_61_0: bool = fn_state.gs_68861;
        // N s_61_1: branch s_61_0 b75 b62
        if s_61_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #424u : u32
        let s_62_0: u32 = 424;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // C s_62_2: const #2u : u8
        let s_62_2: u8 = 2;
        // D s_62_3: cmp-lt s_62_1 s_62_2
        let s_62_3: bool = ((s_62_1) < (s_62_2));
        // N s_62_4: branch s_62_3 b74 b63
        if s_62_3 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#68862 <= s_63_0
        fn_state.gs_68862 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#68862:u8
        let s_64_0: bool = fn_state.gs_68862;
        // N s_64_1: branch s_64_0 b68 b65
        if s_64_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #19136u : u32
        let s_65_0: u32 = 19136;
        // D s_65_1: read-reg s_65_0:struct
        let s_65_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // D s_65_2: call _get_PMSELR_EL0_Type_SEL(s_65_1)
        let s_65_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_65_1);
        // D s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 5u16);
        // D s_65_4: cast zx s_65_3 -> i
        let s_65_4: i128 = (s_65_3.value() as i128);
        // D s_65_5: cast reint s_65_4 -> i64
        let s_65_5: i64 = (s_65_4 as i64);
        // C s_65_6: const #31s : i
        let s_65_6: i128 = 31;
        // D s_65_7: cast zx s_65_5 -> i
        let s_65_7: i128 = (i128::try_from(s_65_5).unwrap());
        // D s_65_8: cmp-eq s_65_7 s_65_6
        let s_65_8: bool = ((s_65_7) == (s_65_6));
        // N s_65_9: branch s_65_8 b67 b66
        if s_65_8 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #64s : i64
        let s_66_0: i64 = 64;
        // C s_66_1: const #19136u : u32
        let s_66_1: u32 = 19136;
        // D s_66_2: read-reg s_66_1:struct
        let s_66_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_1 as isize);
            tracer.read_register(s_66_1 as isize, value);
            value
        };
        // D s_66_3: call _get_PMSELR_EL0_Type_SEL(s_66_2)
        let s_66_3: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_66_2);
        // D s_66_4: cast zx s_66_3 -> bv
        let s_66_4: Bits = Bits::new(s_66_3 as u128, 5u16);
        // D s_66_5: cast zx s_66_4 -> i
        let s_66_5: i128 = (s_66_4.value() as i128);
        // D s_66_6: cast reint s_66_5 -> i64
        let s_66_6: i64 = (s_66_5 as i64);
        // C s_66_7: const #11208u : u32
        let s_66_7: u32 = 11208;
        // D s_66_8: read-reg s_66_7:[struct; 32]
        let s_66_8: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_66_7 as isize);
            tracer.read_register(s_66_7 as isize, value);
            value
        };
        // D s_66_9: cast zx s_66_6 -> i
        let s_66_9: i128 = (i128::try_from(s_66_6).unwrap());
        // D s_66_10: read-element s_66_8[s_66_9]
        let s_66_10: ProductType5c790c8ef59cc8b2 = s_66_8[(s_66_9) as usize];
        // D s_66_11: call __get_PMEVTYPER_EL0(s_66_10)
        let s_66_11: ProductType5c790c8ef59cc8b2 = u__get_PMEVTYPER_EL0(
            state,
            tracer,
            s_66_10,
        );
        // D s_66_12: write-var ga#83942 <= s_66_11
        fn_state.ga_83942 = s_66_11;
        // D s_66_13: read-var ga#83942.0:struct
        let s_66_13: u64 = fn_state.ga_83942._0;
        // D s_66_14: cast zx s_66_13 -> bv
        let s_66_14: Bits = Bits::new(s_66_13 as u128, 64u16);
        // D s_66_15: read-var t:i
        let s_66_15: i128 = fn_state.t;
        // D s_66_16: call X_set(s_66_15, s_66_0, s_66_14)
        let s_66_16: () = X_set(state, tracer, s_66_15, s_66_0, s_66_14);
        // N s_66_17: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #64s : i64
        let s_67_0: i64 = 64;
        // C s_67_1: const #15864u : u32
        let s_67_1: u32 = 15864;
        // D s_67_2: read-reg s_67_1:struct
        let s_67_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_67_1 as isize);
            tracer.read_register(s_67_1 as isize, value);
            value
        };
        // D s_67_3: call __get_PMCCFILTR_EL0(s_67_2)
        let s_67_3: ProductType5c790c8ef59cc8b2 = u__get_PMCCFILTR_EL0(
            state,
            tracer,
            s_67_2,
        );
        // D s_67_4: write-var ga#83936 <= s_67_3
        fn_state.ga_83936 = s_67_3;
        // D s_67_5: read-var ga#83936.0:struct
        let s_67_5: u64 = fn_state.ga_83936._0;
        // D s_67_6: cast zx s_67_5 -> bv
        let s_67_6: Bits = Bits::new(s_67_5 as u128, 64u16);
        // D s_67_7: read-var t:i
        let s_67_7: i128 = fn_state.t;
        // D s_67_8: call X_set(s_67_7, s_67_0, s_67_6)
        let s_67_8: () = X_set(state, tracer, s_67_7, s_67_0, s_67_6);
        // N s_67_9: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #() : ()
        let s_68_0: () = ();
        // S s_68_1: call Halted(s_68_0)
        let s_68_1: bool = Halted(state, tracer, s_68_0);
        // N s_68_2: branch s_68_1 b73 b69
        if s_68_1 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#68866 <= s_69_0
        fn_state.gs_68866 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#68866:u8
        let s_70_0: bool = fn_state.gs_68866;
        // N s_70_1: branch s_70_0 b72 b71
        if s_70_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #24u : u8
        let s_71_0: u8 = 24;
        // C s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 8u16);
        // C s_71_2: cast zx s_71_1 -> i
        let s_71_2: i128 = (s_71_1.value() as i128);
        // C s_71_3: cast reint s_71_2 -> i64
        let s_71_3: i64 = (s_71_2 as i64);
        // C s_71_4: cast zx s_71_3 -> i
        let s_71_4: i128 = (i128::try_from(s_71_3).unwrap());
        // C s_71_5: const #424u : u32
        let s_71_5: u32 = 424;
        // D s_71_6: read-reg s_71_5:u8
        let s_71_6: u8 = {
            let value = state.read_register::<u8>(s_71_5 as isize);
            tracer.read_register(s_71_5 as isize, value);
            value
        };
        // D s_71_7: call AArch64_SystemAccessTrap(s_71_6, s_71_4)
        let s_71_7: () = AArch64_SystemAccessTrap(state, tracer, s_71_6, s_71_4);
        // N s_71_8: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_72_0: panic
        panic!("{:?}", ());
        // N s_72_1: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __EDSCR_SDD:u8
        let s_73_0: bool = fn_state.u__EDSCR_SDD;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#68866 <= s_73_4
        fn_state.gs_68866 = s_73_4;
        // N s_73_6: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var __MDCR_EL3_TPM:u8
        let s_74_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // C s_74_2: const #1u : u8
        let s_74_2: bool = true;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: write-var gs#68862 <= s_74_4
        fn_state.gs_68862 = s_74_4;
        // N s_74_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #146u : u32
        let s_75_0: u32 = 146;
        // S s_75_1: call IsFeatureImplemented(s_75_0)
        let s_75_1: bool = IsFeatureImplemented(state, tracer, s_75_0);
        // S s_75_2: not s_75_1
        let s_75_2: bool = !s_75_1;
        // N s_75_3: branch s_75_2 b77 b76
        if s_75_2 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #24u : u8
        let s_76_0: u8 = 24;
        // C s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 8u16);
        // C s_76_2: cast zx s_76_1 -> i
        let s_76_2: i128 = (s_76_1.value() as i128);
        // C s_76_3: cast reint s_76_2 -> i64
        let s_76_3: i64 = (s_76_2 as i64);
        // C s_76_4: cast zx s_76_3 -> i
        let s_76_4: i128 = (i128::try_from(s_76_3).unwrap());
        // C s_76_5: const #432u : u32
        let s_76_5: u32 = 432;
        // D s_76_6: read-reg s_76_5:u8
        let s_76_6: u8 = {
            let value = state.read_register::<u8>(s_76_5 as isize);
            tracer.read_register(s_76_5 as isize, value);
            value
        };
        // D s_76_7: call AArch64_SystemAccessTrap(s_76_6, s_76_4)
        let s_76_7: () = AArch64_SystemAccessTrap(state, tracer, s_76_6, s_76_4);
        // N s_76_8: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #72u : u32
        let s_77_0: u32 = 72;
        // S s_77_1: call ConstrainUnpredictableProcedure(s_77_0)
        let s_77_1: () = ConstrainUnpredictableProcedure(state, tracer, s_77_0);
        // N s_77_2: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #19136u : u32
        let s_78_0: u32 = 19136;
        // D s_78_1: read-reg s_78_0:struct
        let s_78_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call _get_PMSELR_EL0_Type_SEL(s_78_1)
        let s_78_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_78_1);
        // D s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 5u16);
        // D s_78_4: cast zx s_78_3 -> i
        let s_78_4: i128 = (s_78_3.value() as i128);
        // D s_78_5: cast reint s_78_4 -> i64
        let s_78_5: i64 = (s_78_4 as i64);
        // C s_78_6: const #() : ()
        let s_78_6: () = ();
        // S s_78_7: call AArch64_GetNumEventCountersAccessible(s_78_6)
        let s_78_7: i128 = AArch64_GetNumEventCountersAccessible(state, tracer, s_78_6);
        // D s_78_8: cast zx s_78_5 -> i
        let s_78_8: i128 = (i128::try_from(s_78_5).unwrap());
        // D s_78_9: cmp-ge s_78_8 s_78_7
        let s_78_9: bool = ((s_78_8) >= (s_78_7));
        // D s_78_10: write-var gs#68861 <= s_78_9
        fn_state.gs_68861 = s_78_9;
        // N s_78_11: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #19136u : u32
        let s_79_0: u32 = 19136;
        // D s_79_1: read-reg s_79_0:struct
        let s_79_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // D s_79_2: call _get_PMSELR_EL0_Type_SEL(s_79_1)
        let s_79_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_79_1);
        // D s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 5u16);
        // D s_79_4: cast zx s_79_3 -> i
        let s_79_4: i128 = (s_79_3.value() as i128);
        // D s_79_5: cast reint s_79_4 -> i64
        let s_79_5: i64 = (s_79_4 as i64);
        // C s_79_6: const #31s : i
        let s_79_6: i128 = 31;
        // D s_79_7: cast zx s_79_5 -> i
        let s_79_7: i128 = (i128::try_from(s_79_5).unwrap());
        // D s_79_8: call neq_int(s_79_7, s_79_6)
        let s_79_8: bool = neq_int(state, tracer, s_79_7, s_79_6);
        // D s_79_9: write-var gs#68860 <= s_79_8
        fn_state.gs_68860 = s_79_8;
        // N s_79_10: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #24u : u8
        let s_80_0: u8 = 24;
        // C s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 8u16);
        // C s_80_2: cast zx s_80_1 -> i
        let s_80_2: i128 = (s_80_1.value() as i128);
        // C s_80_3: cast reint s_80_2 -> i64
        let s_80_3: i64 = (s_80_2 as i64);
        // C s_80_4: cast zx s_80_3 -> i
        let s_80_4: i128 = (i128::try_from(s_80_3).unwrap());
        // C s_80_5: const #432u : u32
        let s_80_5: u32 = 432;
        // D s_80_6: read-reg s_80_5:u8
        let s_80_6: u8 = {
            let value = state.read_register::<u8>(s_80_5 as isize);
            tracer.read_register(s_80_5 as isize, value);
            value
        };
        // D s_80_7: call AArch64_SystemAccessTrap(s_80_6, s_80_4)
        let s_80_7: () = AArch64_SystemAccessTrap(state, tracer, s_80_6, s_80_4);
        // N s_80_8: return
        return;
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var __MDCR_EL2_TPM:u8
        let s_81_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 1u16);
        // C s_81_2: const #1u : u8
        let s_81_2: bool = true;
        // C s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // D s_81_4: cmp-eq s_81_1 s_81_3
        let s_81_4: bool = ((s_81_1) == (s_81_3));
        // D s_81_5: write-var gs#68858 <= s_81_4
        fn_state.gs_68858 = s_81_4;
        // N s_81_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #24u : u8
        let s_82_0: u8 = 24;
        // C s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 8u16);
        // C s_82_2: cast zx s_82_1 -> i
        let s_82_2: i128 = (s_82_1.value() as i128);
        // C s_82_3: cast reint s_82_2 -> i64
        let s_82_3: i64 = (s_82_2 as i64);
        // C s_82_4: cast zx s_82_3 -> i
        let s_82_4: i128 = (i128::try_from(s_82_3).unwrap());
        // C s_82_5: const #432u : u32
        let s_82_5: u32 = 432;
        // D s_82_6: read-reg s_82_5:u8
        let s_82_6: u8 = {
            let value = state.read_register::<u8>(s_82_5 as isize);
            tracer.read_register(s_82_5 as isize, value);
            value
        };
        // D s_82_7: call AArch64_SystemAccessTrap(s_82_6, s_82_4)
        let s_82_7: () = AArch64_SystemAccessTrap(state, tracer, s_82_6, s_82_4);
        // N s_82_8: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var __HDFGRTR_EL2_PMEVTYPERn_EL0:u8
        let s_83_0: bool = fn_state.u__HDFGRTR_EL2_PMEVTYPERn_EL0;
        // D s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 1u16);
        // C s_83_2: const #1u : u8
        let s_83_2: bool = true;
        // C s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 1u16);
        // D s_83_4: cmp-eq s_83_1 s_83_3
        let s_83_4: bool = ((s_83_1) == (s_83_3));
        // D s_83_5: write-var gs#68857 <= s_83_4
        fn_state.gs_68857 = s_83_4;
        // N s_83_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #424u : u32
        let s_84_0: u32 = 424;
        // D s_84_1: read-reg s_84_0:u8
        let s_84_1: u8 = {
            let value = state.read_register::<u8>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // C s_84_2: const #2u : u8
        let s_84_2: u8 = 2;
        // D s_84_3: cmp-lt s_84_1 s_84_2
        let s_84_3: bool = ((s_84_1) < (s_84_2));
        // D s_84_4: not s_84_3
        let s_84_4: bool = !s_84_3;
        // N s_84_5: branch s_84_4 b87 b85
        if s_84_4 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var __SCR_EL3_FGTEn:u8
        let s_85_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // C s_85_2: const #1u : u8
        let s_85_2: bool = true;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // D s_85_5: write-var gs#68855 <= s_85_4
        fn_state.gs_68855 = s_85_4;
        // N s_85_6: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#68855:u8
        let s_86_0: bool = fn_state.gs_68855;
        // D s_86_1: write-var gs#68856 <= s_86_0
        fn_state.gs_68856 = s_86_0;
        // N s_86_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #1u : u8
        let s_87_0: bool = true;
        // D s_87_1: write-var gs#68855 <= s_87_0
        fn_state.gs_68855 = s_87_0;
        // N s_87_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #146u : u32
        let s_88_0: u32 = 146;
        // S s_88_1: call IsFeatureImplemented(s_88_0)
        let s_88_1: bool = IsFeatureImplemented(state, tracer, s_88_0);
        // D s_88_2: write-var gs#68854 <= s_88_1
        fn_state.gs_68854 = s_88_1;
        // N s_88_3: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_89_0: panic
        panic!("{:?}", ());
        // N s_89_1: return
        return;
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var __MDCR_EL3_TPM:u8
        let s_90_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 1u16);
        // C s_90_2: const #1u : u8
        let s_90_2: bool = true;
        // C s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // D s_90_5: write-var gs#68853 <= s_90_4
        fn_state.gs_68853 = s_90_4;
        // N s_90_6: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_91_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_91_1: call __IMPDEF_boolean(s_91_0)
        let s_91_1: bool = u__IMPDEF_boolean(state, tracer, s_91_0);
        // D s_91_2: write-var gs#68852 <= s_91_1
        fn_state.gs_68852 = s_91_1;
        // N s_91_3: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var __EDSCR_SDD:u8
        let s_92_0: bool = fn_state.u__EDSCR_SDD;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#68851 <= s_92_4
        fn_state.gs_68851 = s_92_4;
        // N s_92_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #424u : u32
        let s_93_0: u32 = 424;
        // D s_93_1: read-reg s_93_0:u8
        let s_93_1: u8 = {
            let value = state.read_register::<u8>(s_93_0 as isize);
            tracer.read_register(s_93_0 as isize, value);
            value
        };
        // C s_93_2: const #2u : u8
        let s_93_2: u8 = 2;
        // D s_93_3: cmp-lt s_93_1 s_93_2
        let s_93_3: bool = ((s_93_1) < (s_93_2));
        // D s_93_4: write-var gs#68850 <= s_93_3
        fn_state.gs_68850 = s_93_3;
        // N s_93_5: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #() : ()
        let s_94_0: () = ();
        // S s_94_1: call Halted(s_94_0)
        let s_94_1: bool = Halted(state, tracer, s_94_0);
        // N s_94_2: branch s_94_1 b168 b95
        if s_94_1 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #0u : u8
        let s_95_0: bool = false;
        // D s_95_1: write-var gs#68867 <= s_95_0
        fn_state.gs_68867 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#68867:u8
        let s_96_0: bool = fn_state.gs_68867;
        // N s_96_1: branch s_96_0 b167 b97
        if s_96_0 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#68868 <= s_97_0
        fn_state.gs_68868 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#68868:u8
        let s_98_0: bool = fn_state.gs_68868;
        // N s_98_1: branch s_98_0 b166 b99
        if s_98_0 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#68869 <= s_99_0
        fn_state.gs_68869 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#68869:u8
        let s_100_0: bool = fn_state.gs_68869;
        // N s_100_1: branch s_100_0 b165 b101
        if s_100_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #0u : u8
        let s_101_0: bool = false;
        // D s_101_1: write-var gs#68870 <= s_101_0
        fn_state.gs_68870 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#68870:u8
        let s_102_0: bool = fn_state.gs_68870;
        // N s_102_1: branch s_102_0 b164 b103
        if s_102_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #204u : u32
        let s_103_0: u32 = 204;
        // S s_103_1: call IsFeatureImplemented(s_103_0)
        let s_103_1: bool = IsFeatureImplemented(state, tracer, s_103_0);
        // N s_103_2: branch s_103_1 b163 b104
        if s_103_1 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #0u : u8
        let s_104_0: bool = false;
        // D s_104_1: write-var gs#68871 <= s_104_0
        fn_state.gs_68871 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#68871:u8
        let s_105_0: bool = fn_state.gs_68871;
        // N s_105_1: branch s_105_0 b162 b106
        if s_105_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #204u : u32
        let s_106_0: u32 = 204;
        // S s_106_1: call IsFeatureImplemented(s_106_0)
        let s_106_1: bool = IsFeatureImplemented(state, tracer, s_106_0);
        // S s_106_2: not s_106_1
        let s_106_2: bool = !s_106_1;
        // N s_106_3: branch s_106_2 b161 b107
        if s_106_2 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#68872 <= s_107_0
        fn_state.gs_68872 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#68872:u8
        let s_108_0: bool = fn_state.gs_68872;
        // D s_108_1: write-var gs#68873 <= s_108_0
        fn_state.gs_68873 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#68873:u8
        let s_109_0: bool = fn_state.gs_68873;
        // N s_109_1: branch s_109_0 b155 b110
        if s_109_0 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #() : ()
        let s_110_0: () = ();
        // S s_110_1: call EL2Enabled(s_110_0)
        let s_110_1: bool = EL2Enabled(state, tracer, s_110_0);
        // N s_110_2: branch s_110_1 b154 b111
        if s_110_1 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #0u : u8
        let s_111_0: bool = false;
        // D s_111_1: write-var gs#68874 <= s_111_0
        fn_state.gs_68874 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#68874:u8
        let s_112_0: bool = fn_state.gs_68874;
        // N s_112_1: branch s_112_0 b153 b113
        if s_112_0 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #0u : u8
        let s_113_0: bool = false;
        // D s_113_1: write-var gs#68875 <= s_113_0
        fn_state.gs_68875 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#68875:u8
        let s_114_0: bool = fn_state.gs_68875;
        // N s_114_1: branch s_114_0 b149 b115
        if s_114_0 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #0u : u8
        let s_115_0: bool = false;
        // D s_115_1: write-var gs#68877 <= s_115_0
        fn_state.gs_68877 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#68877:u8
        let s_116_0: bool = fn_state.gs_68877;
        // N s_116_1: branch s_116_0 b148 b117
        if s_116_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #0u : u8
        let s_117_0: bool = false;
        // D s_117_1: write-var gs#68878 <= s_117_0
        fn_state.gs_68878 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#68878:u8
        let s_118_0: bool = fn_state.gs_68878;
        // N s_118_1: branch s_118_0 b147 b119
        if s_118_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #() : ()
        let s_119_0: () = ();
        // S s_119_1: call EL2Enabled(s_119_0)
        let s_119_1: bool = EL2Enabled(state, tracer, s_119_0);
        // N s_119_2: branch s_119_1 b146 b120
        if s_119_1 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #0u : u8
        let s_120_0: bool = false;
        // D s_120_1: write-var gs#68879 <= s_120_0
        fn_state.gs_68879 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#68879:u8
        let s_121_0: bool = fn_state.gs_68879;
        // N s_121_1: branch s_121_0 b145 b122
        if s_121_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #() : ()
        let s_122_0: () = ();
        // S s_122_1: call EL2Enabled(s_122_0)
        let s_122_1: bool = EL2Enabled(state, tracer, s_122_0);
        // N s_122_2: branch s_122_1 b144 b123
        if s_122_1 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #0u : u8
        let s_123_0: bool = false;
        // D s_123_1: write-var gs#68881 <= s_123_0
        fn_state.gs_68881 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#68881:u8
        let s_124_0: bool = fn_state.gs_68881;
        // N s_124_1: branch s_124_0 b143 b125
        if s_124_0 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #0u : u8
        let s_125_0: bool = false;
        // D s_125_1: write-var gs#68882 <= s_125_0
        fn_state.gs_68882 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#68882:u8
        let s_126_0: bool = fn_state.gs_68882;
        // N s_126_1: branch s_126_0 b140 b127
        if s_126_0 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #424u : u32
        let s_127_0: u32 = 424;
        // D s_127_1: read-reg s_127_0:u8
        let s_127_1: u8 = {
            let value = state.read_register::<u8>(s_127_0 as isize);
            tracer.read_register(s_127_0 as isize, value);
            value
        };
        // C s_127_2: const #2u : u8
        let s_127_2: u8 = 2;
        // D s_127_3: cmp-lt s_127_1 s_127_2
        let s_127_3: bool = ((s_127_1) < (s_127_2));
        // N s_127_4: branch s_127_3 b139 b128
        if s_127_3 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #0u : u8
        let s_128_0: bool = false;
        // D s_128_1: write-var gs#68883 <= s_128_0
        fn_state.gs_68883 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#68883:u8
        let s_129_0: bool = fn_state.gs_68883;
        // N s_129_1: branch s_129_0 b133 b130
        if s_129_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #19136u : u32
        let s_130_0: u32 = 19136;
        // D s_130_1: read-reg s_130_0:struct
        let s_130_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_130_0 as isize);
            tracer.read_register(s_130_0 as isize, value);
            value
        };
        // D s_130_2: call _get_PMSELR_EL0_Type_SEL(s_130_1)
        let s_130_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_130_1);
        // D s_130_3: cast zx s_130_2 -> bv
        let s_130_3: Bits = Bits::new(s_130_2 as u128, 5u16);
        // D s_130_4: cast zx s_130_3 -> i
        let s_130_4: i128 = (s_130_3.value() as i128);
        // D s_130_5: cast reint s_130_4 -> i64
        let s_130_5: i64 = (s_130_4 as i64);
        // C s_130_6: const #31s : i
        let s_130_6: i128 = 31;
        // D s_130_7: cast zx s_130_5 -> i
        let s_130_7: i128 = (i128::try_from(s_130_5).unwrap());
        // D s_130_8: cmp-eq s_130_7 s_130_6
        let s_130_8: bool = ((s_130_7) == (s_130_6));
        // N s_130_9: branch s_130_8 b132 b131
        if s_130_8 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #64s : i64
        let s_131_0: i64 = 64;
        // C s_131_1: const #19136u : u32
        let s_131_1: u32 = 19136;
        // D s_131_2: read-reg s_131_1:struct
        let s_131_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_131_1 as isize);
            tracer.read_register(s_131_1 as isize, value);
            value
        };
        // D s_131_3: call _get_PMSELR_EL0_Type_SEL(s_131_2)
        let s_131_3: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_131_2);
        // D s_131_4: cast zx s_131_3 -> bv
        let s_131_4: Bits = Bits::new(s_131_3 as u128, 5u16);
        // D s_131_5: cast zx s_131_4 -> i
        let s_131_5: i128 = (s_131_4.value() as i128);
        // D s_131_6: cast reint s_131_5 -> i64
        let s_131_6: i64 = (s_131_5 as i64);
        // C s_131_7: const #11208u : u32
        let s_131_7: u32 = 11208;
        // D s_131_8: read-reg s_131_7:[struct; 32]
        let s_131_8: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_131_7 as isize);
            tracer.read_register(s_131_7 as isize, value);
            value
        };
        // D s_131_9: cast zx s_131_6 -> i
        let s_131_9: i128 = (i128::try_from(s_131_6).unwrap());
        // D s_131_10: read-element s_131_8[s_131_9]
        let s_131_10: ProductType5c790c8ef59cc8b2 = s_131_8[(s_131_9) as usize];
        // D s_131_11: call __get_PMEVTYPER_EL0(s_131_10)
        let s_131_11: ProductType5c790c8ef59cc8b2 = u__get_PMEVTYPER_EL0(
            state,
            tracer,
            s_131_10,
        );
        // D s_131_12: write-var ga#83896 <= s_131_11
        fn_state.ga_83896 = s_131_11;
        // D s_131_13: read-var ga#83896.0:struct
        let s_131_13: u64 = fn_state.ga_83896._0;
        // D s_131_14: cast zx s_131_13 -> bv
        let s_131_14: Bits = Bits::new(s_131_13 as u128, 64u16);
        // D s_131_15: read-var t:i
        let s_131_15: i128 = fn_state.t;
        // D s_131_16: call X_set(s_131_15, s_131_0, s_131_14)
        let s_131_16: () = X_set(state, tracer, s_131_15, s_131_0, s_131_14);
        // N s_131_17: return
        return;
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #64s : i64
        let s_132_0: i64 = 64;
        // C s_132_1: const #15864u : u32
        let s_132_1: u32 = 15864;
        // D s_132_2: read-reg s_132_1:struct
        let s_132_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_132_1 as isize);
            tracer.read_register(s_132_1 as isize, value);
            value
        };
        // D s_132_3: call __get_PMCCFILTR_EL0(s_132_2)
        let s_132_3: ProductType5c790c8ef59cc8b2 = u__get_PMCCFILTR_EL0(
            state,
            tracer,
            s_132_2,
        );
        // D s_132_4: write-var ga#83890 <= s_132_3
        fn_state.ga_83890 = s_132_3;
        // D s_132_5: read-var ga#83890.0:struct
        let s_132_5: u64 = fn_state.ga_83890._0;
        // D s_132_6: cast zx s_132_5 -> bv
        let s_132_6: Bits = Bits::new(s_132_5 as u128, 64u16);
        // D s_132_7: read-var t:i
        let s_132_7: i128 = fn_state.t;
        // D s_132_8: call X_set(s_132_7, s_132_0, s_132_6)
        let s_132_8: () = X_set(state, tracer, s_132_7, s_132_0, s_132_6);
        // N s_132_9: return
        return;
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #() : ()
        let s_133_0: () = ();
        // S s_133_1: call Halted(s_133_0)
        let s_133_1: bool = Halted(state, tracer, s_133_0);
        // N s_133_2: branch s_133_1 b138 b134
        if s_133_1 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #0u : u8
        let s_134_0: bool = false;
        // D s_134_1: write-var gs#68887 <= s_134_0
        fn_state.gs_68887 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#68887:u8
        let s_135_0: bool = fn_state.gs_68887;
        // N s_135_1: branch s_135_0 b137 b136
        if s_135_0 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #24u : u8
        let s_136_0: u8 = 24;
        // C s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 8u16);
        // C s_136_2: cast zx s_136_1 -> i
        let s_136_2: i128 = (s_136_1.value() as i128);
        // C s_136_3: cast reint s_136_2 -> i64
        let s_136_3: i64 = (s_136_2 as i64);
        // C s_136_4: cast zx s_136_3 -> i
        let s_136_4: i128 = (i128::try_from(s_136_3).unwrap());
        // C s_136_5: const #424u : u32
        let s_136_5: u32 = 424;
        // D s_136_6: read-reg s_136_5:u8
        let s_136_6: u8 = {
            let value = state.read_register::<u8>(s_136_5 as isize);
            tracer.read_register(s_136_5 as isize, value);
            value
        };
        // D s_136_7: call AArch64_SystemAccessTrap(s_136_6, s_136_4)
        let s_136_7: () = AArch64_SystemAccessTrap(state, tracer, s_136_6, s_136_4);
        // N s_136_8: return
        return;
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_137_0: panic
        panic!("{:?}", ());
        // N s_137_1: return
        return;
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var __EDSCR_SDD:u8
        let s_138_0: bool = fn_state.u__EDSCR_SDD;
        // D s_138_1: cast zx s_138_0 -> bv
        let s_138_1: Bits = Bits::new(s_138_0 as u128, 1u16);
        // C s_138_2: const #1u : u8
        let s_138_2: bool = true;
        // C s_138_3: cast zx s_138_2 -> bv
        let s_138_3: Bits = Bits::new(s_138_2 as u128, 1u16);
        // D s_138_4: cmp-eq s_138_1 s_138_3
        let s_138_4: bool = ((s_138_1) == (s_138_3));
        // D s_138_5: write-var gs#68887 <= s_138_4
        fn_state.gs_68887 = s_138_4;
        // N s_138_6: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var __MDCR_EL3_TPM:u8
        let s_139_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_139_1: cast zx s_139_0 -> bv
        let s_139_1: Bits = Bits::new(s_139_0 as u128, 1u16);
        // C s_139_2: const #1u : u8
        let s_139_2: bool = true;
        // C s_139_3: cast zx s_139_2 -> bv
        let s_139_3: Bits = Bits::new(s_139_2 as u128, 1u16);
        // D s_139_4: cmp-eq s_139_1 s_139_3
        let s_139_4: bool = ((s_139_1) == (s_139_3));
        // D s_139_5: write-var gs#68883 <= s_139_4
        fn_state.gs_68883 = s_139_4;
        // N s_139_6: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #146u : u32
        let s_140_0: u32 = 146;
        // S s_140_1: call IsFeatureImplemented(s_140_0)
        let s_140_1: bool = IsFeatureImplemented(state, tracer, s_140_0);
        // S s_140_2: not s_140_1
        let s_140_2: bool = !s_140_1;
        // N s_140_3: branch s_140_2 b142 b141
        if s_140_2 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #24u : u8
        let s_141_0: u8 = 24;
        // C s_141_1: cast zx s_141_0 -> bv
        let s_141_1: Bits = Bits::new(s_141_0 as u128, 8u16);
        // C s_141_2: cast zx s_141_1 -> i
        let s_141_2: i128 = (s_141_1.value() as i128);
        // C s_141_3: cast reint s_141_2 -> i64
        let s_141_3: i64 = (s_141_2 as i64);
        // C s_141_4: cast zx s_141_3 -> i
        let s_141_4: i128 = (i128::try_from(s_141_3).unwrap());
        // C s_141_5: const #432u : u32
        let s_141_5: u32 = 432;
        // D s_141_6: read-reg s_141_5:u8
        let s_141_6: u8 = {
            let value = state.read_register::<u8>(s_141_5 as isize);
            tracer.read_register(s_141_5 as isize, value);
            value
        };
        // D s_141_7: call AArch64_SystemAccessTrap(s_141_6, s_141_4)
        let s_141_7: () = AArch64_SystemAccessTrap(state, tracer, s_141_6, s_141_4);
        // N s_141_8: return
        return;
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #72u : u32
        let s_142_0: u32 = 72;
        // S s_142_1: call ConstrainUnpredictableProcedure(s_142_0)
        let s_142_1: () = ConstrainUnpredictableProcedure(state, tracer, s_142_0);
        // N s_142_2: return
        return;
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #19136u : u32
        let s_143_0: u32 = 19136;
        // D s_143_1: read-reg s_143_0:struct
        let s_143_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_143_0 as isize);
            tracer.read_register(s_143_0 as isize, value);
            value
        };
        // D s_143_2: call _get_PMSELR_EL0_Type_SEL(s_143_1)
        let s_143_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_143_1);
        // D s_143_3: cast zx s_143_2 -> bv
        let s_143_3: Bits = Bits::new(s_143_2 as u128, 5u16);
        // D s_143_4: cast zx s_143_3 -> i
        let s_143_4: i128 = (s_143_3.value() as i128);
        // D s_143_5: cast reint s_143_4 -> i64
        let s_143_5: i64 = (s_143_4 as i64);
        // C s_143_6: const #() : ()
        let s_143_6: () = ();
        // S s_143_7: call AArch64_GetNumEventCountersAccessible(s_143_6)
        let s_143_7: i128 = AArch64_GetNumEventCountersAccessible(
            state,
            tracer,
            s_143_6,
        );
        // D s_143_8: cast zx s_143_5 -> i
        let s_143_8: i128 = (i128::try_from(s_143_5).unwrap());
        // D s_143_9: cmp-ge s_143_8 s_143_7
        let s_143_9: bool = ((s_143_8) >= (s_143_7));
        // D s_143_10: write-var gs#68882 <= s_143_9
        fn_state.gs_68882 = s_143_9;
        // N s_143_11: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #19136u : u32
        let s_144_0: u32 = 19136;
        // D s_144_1: read-reg s_144_0:struct
        let s_144_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_144_0 as isize);
            tracer.read_register(s_144_0 as isize, value);
            value
        };
        // D s_144_2: call _get_PMSELR_EL0_Type_SEL(s_144_1)
        let s_144_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_144_1);
        // D s_144_3: cast zx s_144_2 -> bv
        let s_144_3: Bits = Bits::new(s_144_2 as u128, 5u16);
        // D s_144_4: cast zx s_144_3 -> i
        let s_144_4: i128 = (s_144_3.value() as i128);
        // D s_144_5: cast reint s_144_4 -> i64
        let s_144_5: i64 = (s_144_4 as i64);
        // C s_144_6: const #31s : i
        let s_144_6: i128 = 31;
        // D s_144_7: cast zx s_144_5 -> i
        let s_144_7: i128 = (i128::try_from(s_144_5).unwrap());
        // D s_144_8: call neq_int(s_144_7, s_144_6)
        let s_144_8: bool = neq_int(state, tracer, s_144_7, s_144_6);
        // D s_144_9: write-var gs#68881 <= s_144_8
        fn_state.gs_68881 = s_144_8;
        // N s_144_10: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #24u : u8
        let s_145_0: u8 = 24;
        // C s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 8u16);
        // C s_145_2: cast zx s_145_1 -> i
        let s_145_2: i128 = (s_145_1.value() as i128);
        // C s_145_3: cast reint s_145_2 -> i64
        let s_145_3: i64 = (s_145_2 as i64);
        // C s_145_4: cast zx s_145_3 -> i
        let s_145_4: i128 = (i128::try_from(s_145_3).unwrap());
        // C s_145_5: const #432u : u32
        let s_145_5: u32 = 432;
        // D s_145_6: read-reg s_145_5:u8
        let s_145_6: u8 = {
            let value = state.read_register::<u8>(s_145_5 as isize);
            tracer.read_register(s_145_5 as isize, value);
            value
        };
        // D s_145_7: call AArch64_SystemAccessTrap(s_145_6, s_145_4)
        let s_145_7: () = AArch64_SystemAccessTrap(state, tracer, s_145_6, s_145_4);
        // N s_145_8: return
        return;
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var __MDCR_EL2_TPM:u8
        let s_146_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 1u16);
        // C s_146_2: const #1u : u8
        let s_146_2: bool = true;
        // C s_146_3: cast zx s_146_2 -> bv
        let s_146_3: Bits = Bits::new(s_146_2 as u128, 1u16);
        // D s_146_4: cmp-eq s_146_1 s_146_3
        let s_146_4: bool = ((s_146_1) == (s_146_3));
        // D s_146_5: write-var gs#68879 <= s_146_4
        fn_state.gs_68879 = s_146_4;
        // N s_146_6: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #24u : u8
        let s_147_0: u8 = 24;
        // C s_147_1: cast zx s_147_0 -> bv
        let s_147_1: Bits = Bits::new(s_147_0 as u128, 8u16);
        // C s_147_2: cast zx s_147_1 -> i
        let s_147_2: i128 = (s_147_1.value() as i128);
        // C s_147_3: cast reint s_147_2 -> i64
        let s_147_3: i64 = (s_147_2 as i64);
        // C s_147_4: cast zx s_147_3 -> i
        let s_147_4: i128 = (i128::try_from(s_147_3).unwrap());
        // C s_147_5: const #432u : u32
        let s_147_5: u32 = 432;
        // D s_147_6: read-reg s_147_5:u8
        let s_147_6: u8 = {
            let value = state.read_register::<u8>(s_147_5 as isize);
            tracer.read_register(s_147_5 as isize, value);
            value
        };
        // D s_147_7: call AArch64_SystemAccessTrap(s_147_6, s_147_4)
        let s_147_7: () = AArch64_SystemAccessTrap(state, tracer, s_147_6, s_147_4);
        // N s_147_8: return
        return;
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var __HDFGRTR_EL2_PMEVTYPERn_EL0:u8
        let s_148_0: bool = fn_state.u__HDFGRTR_EL2_PMEVTYPERn_EL0;
        // D s_148_1: cast zx s_148_0 -> bv
        let s_148_1: Bits = Bits::new(s_148_0 as u128, 1u16);
        // C s_148_2: const #1u : u8
        let s_148_2: bool = true;
        // C s_148_3: cast zx s_148_2 -> bv
        let s_148_3: Bits = Bits::new(s_148_2 as u128, 1u16);
        // D s_148_4: cmp-eq s_148_1 s_148_3
        let s_148_4: bool = ((s_148_1) == (s_148_3));
        // D s_148_5: write-var gs#68878 <= s_148_4
        fn_state.gs_68878 = s_148_4;
        // N s_148_6: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #424u : u32
        let s_149_0: u32 = 424;
        // D s_149_1: read-reg s_149_0:u8
        let s_149_1: u8 = {
            let value = state.read_register::<u8>(s_149_0 as isize);
            tracer.read_register(s_149_0 as isize, value);
            value
        };
        // C s_149_2: const #2u : u8
        let s_149_2: u8 = 2;
        // D s_149_3: cmp-lt s_149_1 s_149_2
        let s_149_3: bool = ((s_149_1) < (s_149_2));
        // D s_149_4: not s_149_3
        let s_149_4: bool = !s_149_3;
        // N s_149_5: branch s_149_4 b152 b150
        if s_149_4 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var __SCR_EL3_FGTEn:u8
        let s_150_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_150_1: cast zx s_150_0 -> bv
        let s_150_1: Bits = Bits::new(s_150_0 as u128, 1u16);
        // C s_150_2: const #1u : u8
        let s_150_2: bool = true;
        // C s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 1u16);
        // D s_150_4: cmp-eq s_150_1 s_150_3
        let s_150_4: bool = ((s_150_1) == (s_150_3));
        // D s_150_5: write-var gs#68876 <= s_150_4
        fn_state.gs_68876 = s_150_4;
        // N s_150_6: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var gs#68876:u8
        let s_151_0: bool = fn_state.gs_68876;
        // D s_151_1: write-var gs#68877 <= s_151_0
        fn_state.gs_68877 = s_151_0;
        // N s_151_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #1u : u8
        let s_152_0: bool = true;
        // D s_152_1: write-var gs#68876 <= s_152_0
        fn_state.gs_68876 = s_152_0;
        // N s_152_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #146u : u32
        let s_153_0: u32 = 146;
        // S s_153_1: call IsFeatureImplemented(s_153_0)
        let s_153_1: bool = IsFeatureImplemented(state, tracer, s_153_0);
        // D s_153_2: write-var gs#68875 <= s_153_1
        fn_state.gs_68875 = s_153_1;
        // N s_153_3: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #102552u : u32
        let s_154_0: u32 = 102552;
        // D s_154_1: read-reg s_154_0:struct
        let s_154_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_154_0 as isize);
            tracer.read_register(s_154_0 as isize, value);
            value
        };
        // D s_154_2: call _get_HCR_EL2_Type_E2H(s_154_1)
        let s_154_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_154_1);
        // C s_154_3: const #102552u : u32
        let s_154_3: u32 = 102552;
        // D s_154_4: read-reg s_154_3:struct
        let s_154_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_154_3 as isize);
            tracer.read_register(s_154_3 as isize, value);
            value
        };
        // D s_154_5: call _get_HCR_EL2_Type_TGE(s_154_4)
        let s_154_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_154_4);
        // D s_154_6: cast zx s_154_2 -> bv
        let s_154_6: Bits = Bits::new(s_154_2 as u128, 1u16);
        // D s_154_7: cast zx s_154_5 -> bv
        let s_154_7: Bits = Bits::new(s_154_5 as u128, 1u16);
        // D s_154_8: cast reint s_154_6 -> u128
        let s_154_8: u128 = (s_154_6.value() as u128);
        // D s_154_9: size-of s_154_6
        let s_154_9: u16 = s_154_6.length();
        // D s_154_10: cast reint s_154_7 -> u128
        let s_154_10: u128 = (s_154_7.value() as u128);
        // D s_154_11: size-of s_154_7
        let s_154_11: u16 = s_154_7.length();
        // D s_154_12: lsl s_154_8 s_154_11
        let s_154_12: u128 = s_154_8 << s_154_11;
        // D s_154_13: or s_154_12 s_154_10
        let s_154_13: u128 = ((s_154_12) | (s_154_10));
        // D s_154_14: add s_154_9 s_154_11
        let s_154_14: u16 = (s_154_9 + s_154_11);
        // D s_154_15: create-bits s_154_13 s_154_14
        let s_154_15: Bits = Bits::new(s_154_13, s_154_14);
        // D s_154_16: cast reint s_154_15 -> u8
        let s_154_16: u8 = (s_154_15.value() as u8);
        // D s_154_17: cast zx s_154_16 -> bv
        let s_154_17: Bits = Bits::new(s_154_16 as u128, 2u16);
        // C s_154_18: const #3u : u8
        let s_154_18: u8 = 3;
        // C s_154_19: cast zx s_154_18 -> bv
        let s_154_19: Bits = Bits::new(s_154_18 as u128, 2u16);
        // D s_154_20: cmp-ne s_154_17 s_154_19
        let s_154_20: bool = ((s_154_17) != (s_154_19));
        // D s_154_21: write-var gs#68874 <= s_154_20
        fn_state.gs_68874 = s_154_20;
        // N s_154_22: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #() : ()
        let s_155_0: () = ();
        // S s_155_1: call EL2Enabled(s_155_0)
        let s_155_1: bool = EL2Enabled(state, tracer, s_155_0);
        // N s_155_2: branch s_155_1 b160 b156
        if s_155_1 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #0u : u8
        let s_156_0: bool = false;
        // D s_156_1: write-var gs#68888 <= s_156_0
        fn_state.gs_68888 = s_156_0;
        // N s_156_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var gs#68888:u8
        let s_157_0: bool = fn_state.gs_68888;
        // N s_157_1: branch s_157_0 b159 b158
        if s_157_0 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #24u : u8
        let s_158_0: u8 = 24;
        // C s_158_1: cast zx s_158_0 -> bv
        let s_158_1: Bits = Bits::new(s_158_0 as u128, 8u16);
        // C s_158_2: cast zx s_158_1 -> i
        let s_158_2: i128 = (s_158_1.value() as i128);
        // C s_158_3: cast reint s_158_2 -> i64
        let s_158_3: i64 = (s_158_2 as i64);
        // C s_158_4: cast zx s_158_3 -> i
        let s_158_4: i128 = (i128::try_from(s_158_3).unwrap());
        // C s_158_5: const #440u : u32
        let s_158_5: u32 = 440;
        // D s_158_6: read-reg s_158_5:u8
        let s_158_6: u8 = {
            let value = state.read_register::<u8>(s_158_5 as isize);
            tracer.read_register(s_158_5 as isize, value);
            value
        };
        // D s_158_7: call AArch64_SystemAccessTrap(s_158_6, s_158_4)
        let s_158_7: () = AArch64_SystemAccessTrap(state, tracer, s_158_6, s_158_4);
        // N s_158_8: return
        return;
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #24u : u8
        let s_159_0: u8 = 24;
        // C s_159_1: cast zx s_159_0 -> bv
        let s_159_1: Bits = Bits::new(s_159_0 as u128, 8u16);
        // C s_159_2: cast zx s_159_1 -> i
        let s_159_2: i128 = (s_159_1.value() as i128);
        // C s_159_3: cast reint s_159_2 -> i64
        let s_159_3: i64 = (s_159_2 as i64);
        // C s_159_4: cast zx s_159_3 -> i
        let s_159_4: i128 = (i128::try_from(s_159_3).unwrap());
        // C s_159_5: const #432u : u32
        let s_159_5: u32 = 432;
        // D s_159_6: read-reg s_159_5:u8
        let s_159_6: u8 = {
            let value = state.read_register::<u8>(s_159_5 as isize);
            tracer.read_register(s_159_5 as isize, value);
            value
        };
        // D s_159_7: call AArch64_SystemAccessTrap(s_159_6, s_159_4)
        let s_159_7: () = AArch64_SystemAccessTrap(state, tracer, s_159_6, s_159_4);
        // N s_159_8: return
        return;
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var __HCR_EL2_TGE:u8
        let s_160_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_160_1: cast zx s_160_0 -> bv
        let s_160_1: Bits = Bits::new(s_160_0 as u128, 1u16);
        // C s_160_2: const #1u : u8
        let s_160_2: bool = true;
        // C s_160_3: cast zx s_160_2 -> bv
        let s_160_3: Bits = Bits::new(s_160_2 as u128, 1u16);
        // D s_160_4: cmp-eq s_160_1 s_160_3
        let s_160_4: bool = ((s_160_1) == (s_160_3));
        // D s_160_5: write-var gs#68888 <= s_160_4
        fn_state.gs_68888 = s_160_4;
        // N s_160_6: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var __PMUSERENR_EL0_EN:u8
        let s_161_0: bool = fn_state.u__PMUSERENR_EL0_EN;
        // D s_161_1: cast zx s_161_0 -> bv
        let s_161_1: Bits = Bits::new(s_161_0 as u128, 1u16);
        // C s_161_2: const #0u : u8
        let s_161_2: bool = false;
        // C s_161_3: cast zx s_161_2 -> bv
        let s_161_3: Bits = Bits::new(s_161_2 as u128, 1u16);
        // D s_161_4: cmp-eq s_161_1 s_161_3
        let s_161_4: bool = ((s_161_1) == (s_161_3));
        // D s_161_5: write-var gs#68872 <= s_161_4
        fn_state.gs_68872 = s_161_4;
        // N s_161_6: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #1u : u8
        let s_162_0: bool = true;
        // D s_162_1: write-var gs#68873 <= s_162_0
        fn_state.gs_68873 = s_162_0;
        // N s_162_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #102624u : u32
        let s_163_0: u32 = 102624;
        // D s_163_1: read-reg s_163_0:struct
        let s_163_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_163_0 as isize);
            tracer.read_register(s_163_0 as isize, value);
            value
        };
        // D s_163_2: call _get_PMUSERENR_EL0_Type_UEN(s_163_1)
        let s_163_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_163_1);
        // C s_163_3: const #102624u : u32
        let s_163_3: u32 = 102624;
        // D s_163_4: read-reg s_163_3:struct
        let s_163_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_163_3 as isize);
            tracer.read_register(s_163_3 as isize, value);
            value
        };
        // D s_163_5: call _get_PMUSERENR_EL0_Type_EN(s_163_4)
        let s_163_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_163_4);
        // D s_163_6: cast zx s_163_2 -> bv
        let s_163_6: Bits = Bits::new(s_163_2 as u128, 1u16);
        // D s_163_7: cast zx s_163_5 -> bv
        let s_163_7: Bits = Bits::new(s_163_5 as u128, 1u16);
        // D s_163_8: cast reint s_163_6 -> u128
        let s_163_8: u128 = (s_163_6.value() as u128);
        // D s_163_9: size-of s_163_6
        let s_163_9: u16 = s_163_6.length();
        // D s_163_10: cast reint s_163_7 -> u128
        let s_163_10: u128 = (s_163_7.value() as u128);
        // D s_163_11: size-of s_163_7
        let s_163_11: u16 = s_163_7.length();
        // D s_163_12: lsl s_163_8 s_163_11
        let s_163_12: u128 = s_163_8 << s_163_11;
        // D s_163_13: or s_163_12 s_163_10
        let s_163_13: u128 = ((s_163_12) | (s_163_10));
        // D s_163_14: add s_163_9 s_163_11
        let s_163_14: u16 = (s_163_9 + s_163_11);
        // D s_163_15: create-bits s_163_13 s_163_14
        let s_163_15: Bits = Bits::new(s_163_13, s_163_14);
        // D s_163_16: cast reint s_163_15 -> u8
        let s_163_16: u8 = (s_163_15.value() as u8);
        // D s_163_17: cast zx s_163_16 -> bv
        let s_163_17: Bits = Bits::new(s_163_16 as u128, 2u16);
        // C s_163_18: const #0u : u8
        let s_163_18: u8 = 0;
        // C s_163_19: cast zx s_163_18 -> bv
        let s_163_19: Bits = Bits::new(s_163_18 as u128, 2u16);
        // D s_163_20: cmp-eq s_163_17 s_163_19
        let s_163_20: bool = ((s_163_17) == (s_163_19));
        // D s_163_21: write-var gs#68871 <= s_163_20
        fn_state.gs_68871 = s_163_20;
        // N s_163_22: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_164_0: panic
        panic!("{:?}", ());
        // N s_164_1: return
        return;
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var __MDCR_EL3_TPM:u8
        let s_165_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_165_1: cast zx s_165_0 -> bv
        let s_165_1: Bits = Bits::new(s_165_0 as u128, 1u16);
        // C s_165_2: const #1u : u8
        let s_165_2: bool = true;
        // C s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 1u16);
        // D s_165_4: cmp-eq s_165_1 s_165_3
        let s_165_4: bool = ((s_165_1) == (s_165_3));
        // D s_165_5: write-var gs#68870 <= s_165_4
        fn_state.gs_68870 = s_165_4;
        // N s_165_6: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_166_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_166_1: call __IMPDEF_boolean(s_166_0)
        let s_166_1: bool = u__IMPDEF_boolean(state, tracer, s_166_0);
        // D s_166_2: write-var gs#68869 <= s_166_1
        fn_state.gs_68869 = s_166_1;
        // N s_166_3: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var __EDSCR_SDD:u8
        let s_167_0: bool = fn_state.u__EDSCR_SDD;
        // D s_167_1: cast zx s_167_0 -> bv
        let s_167_1: Bits = Bits::new(s_167_0 as u128, 1u16);
        // C s_167_2: const #1u : u8
        let s_167_2: bool = true;
        // C s_167_3: cast zx s_167_2 -> bv
        let s_167_3: Bits = Bits::new(s_167_2 as u128, 1u16);
        // D s_167_4: cmp-eq s_167_1 s_167_3
        let s_167_4: bool = ((s_167_1) == (s_167_3));
        // D s_167_5: write-var gs#68868 <= s_167_4
        fn_state.gs_68868 = s_167_4;
        // N s_167_6: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #424u : u32
        let s_168_0: u32 = 424;
        // D s_168_1: read-reg s_168_0:u8
        let s_168_1: u8 = {
            let value = state.read_register::<u8>(s_168_0 as isize);
            tracer.read_register(s_168_0 as isize, value);
            value
        };
        // C s_168_2: const #2u : u8
        let s_168_2: u8 = 2;
        // D s_168_3: cmp-lt s_168_1 s_168_2
        let s_168_3: bool = ((s_168_1) < (s_168_2));
        // D s_168_4: write-var gs#68867 <= s_168_3
        fn_state.gs_68867 = s_168_3;
        // N s_168_5: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #146u : u32
        let s_169_0: u32 = 146;
        // S s_169_1: call IsFeatureImplemented(s_169_0)
        let s_169_1: bool = IsFeatureImplemented(state, tracer, s_169_0);
        // N s_169_2: branch s_169_1 b171 b170
        if s_169_1 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #72u : u32
        let s_170_0: u32 = 72;
        // S s_170_1: call ConstrainUnpredictableProcedure(s_170_0)
        let s_170_1: () = ConstrainUnpredictableProcedure(state, tracer, s_170_0);
        // N s_170_2: return
        return;
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_171_0: panic
        panic!("{:?}", ());
        // N s_171_1: return
        return;
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #19136u : u32
        let s_172_0: u32 = 19136;
        // D s_172_1: read-reg s_172_0:struct
        let s_172_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_172_0 as isize);
            tracer.read_register(s_172_0 as isize, value);
            value
        };
        // D s_172_2: call _get_PMSELR_EL0_Type_SEL(s_172_1)
        let s_172_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_172_1);
        // D s_172_3: cast zx s_172_2 -> bv
        let s_172_3: Bits = Bits::new(s_172_2 as u128, 5u16);
        // D s_172_4: cast zx s_172_3 -> i
        let s_172_4: i128 = (s_172_3.value() as i128);
        // D s_172_5: cast reint s_172_4 -> i64
        let s_172_5: i64 = (s_172_4 as i64);
        // C s_172_6: const #31s : i
        let s_172_6: i128 = 31;
        // D s_172_7: cast zx s_172_5 -> i
        let s_172_7: i128 = (i128::try_from(s_172_5).unwrap());
        // D s_172_8: cmp-ge s_172_7 s_172_6
        let s_172_8: bool = ((s_172_7) >= (s_172_6));
        // D s_172_9: write-var gs#68837 <= s_172_8
        fn_state.gs_68837 = s_172_8;
        // N s_172_10: jump b2
        return block_2(state, tracer, fn_state);
    }
}
