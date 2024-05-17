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
use u_get_AMUSERENR_Type_EN::*;
use u_get_HSTR_EL2_Type_T0::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HCR_EL2_Type_E2H::*;
use HCR_read::*;
use Halted::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use AMUSERENR_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HCR_Type_TGE::*;
use u_get_HCPTR_Type_TAM::*;
use u_get_CPTR_EL3_Type_TAM::*;
use R_set::*;
use u_get_HSTR_Type_T0::*;
use HCPTR_read::*;
use ELUsingAArch32::*;
use u_get_HAFGRTR_EL2_Type_AMEVCNTR02_EL0::*;
use u_get_CPTR_EL2_Type_TAM::*;
use u_get_AMUSERENR_EL0_Type_EN::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn AMEVCNTR0_SysRegRead64_bf8f308b8b559115<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_120352: bool,
        gs_120319: bool,
        ga_200435: ProductType5c790c8ef59cc8b2,
        gs_120320: bool,
        u__CPTR_EL3_TAM: bool,
        ga_200473: ProductType5c790c8ef59cc8b2,
        u__CPTR_EL2_TAM: bool,
        gs_120388: bool,
        gs_120412: bool,
        gs_120384: bool,
        ga_200437: ProductType5c790c8ef59cc8b2,
        gs_120321: bool,
        u__HCPTR_TAM: bool,
        gs_120343: bool,
        gs_120369: bool,
        gs_120334: bool,
        gs_120351: bool,
        ga_200465: ProductType5c790c8ef59cc8b2,
        u__HCR_TGE: bool,
        gs_120375: bool,
        ga_200463: ProductType5c790c8ef59cc8b2,
        gs_120372: bool,
        gs_120378: bool,
        ga_200387: ProductType5c790c8ef59cc8b2,
        gs_120366: bool,
        ga_200471: ProductType5c790c8ef59cc8b2,
        gs_120350: bool,
        gs_120383: bool,
        gs_120370: bool,
        gs_120387: bool,
        gs_120410: bool,
        u__PSTATE_EL: u8,
        gs_120380: bool,
        gs_120335: bool,
        gs_120407: bool,
        gs_120390: bool,
        gs_120367: bool,
        gs_120340: bool,
        gs_120386: bool,
        gs_120411: bool,
        u__HCR_EL2_TGE: bool,
        ga_200389: ProductType5c790c8ef59cc8b2,
        gs_120408: bool,
        ga_200469: ProductType72d61775f103f7e0,
        gs_120374: bool,
        ga_200441: ProductType72d61775f103f7e0,
        gs_120396: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_120348: bool,
        gs_120349: bool,
        gs_120393: bool,
        u__AMUSERENR_EL0_EN: bool,
        gs_120368: bool,
        gs_120413: bool,
        gs_120354: bool,
        gs_120355: bool,
        gs_120317: bool,
        ga_200393: ProductType72d61775f103f7e0,
        gs_120391: bool,
        gs_120338: bool,
        gs_120409: bool,
        gs_120337: bool,
        gs_120373: bool,
        gs_120389: bool,
        gs_120323: bool,
        gs_120345: bool,
        gs_120336: bool,
        gs_120339: bool,
        gs_120353: bool,
        gs_120318: bool,
        gs_120371: bool,
        gs_120392: bool,
        u__HAFGRTR_EL2_AMEVCNTR02_EL0: bool,
        gs_120322: bool,
        gs_120379: bool,
        gs_120344: bool,
        u__HSTR_EL2_T0: bool,
        u__AMUSERENR_EN: bool,
        u__HSTR_T0: bool,
        gs_120395: bool,
        gs_120394: bool,
        gs_120385: bool,
        ga_200477: ProductType72d61775f103f7e0,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRm,
        t,
        t2,
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
        // C s_0_3: const #16840u : u32
        let s_0_3: u32 = 16840;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_CPTR_EL3_Type_TAM(s_0_4)
        let s_0_5: bool = u_get_CPTR_EL3_Type_TAM(state, tracer, s_0_4);
        // D s_0_6: write-var __CPTR_EL3_TAM <= s_0_5
        fn_state.u__CPTR_EL3_TAM = s_0_5;
        // C s_0_7: const #90496u : u32
        let s_0_7: u32 = 90496;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_AMUSERENR_EL0_Type_EN(s_0_8)
        let s_0_9: bool = u_get_AMUSERENR_EL0_Type_EN(state, tracer, s_0_8);
        // D s_0_10: write-var __AMUSERENR_EL0_EN <= s_0_9
        fn_state.u__AMUSERENR_EL0_EN = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TGE(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TGE <= s_0_13
        fn_state.u__HCR_EL2_TGE = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call AMUSERENR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = AMUSERENR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_AMUSERENR_Type_EN(s_0_16)
        let s_0_17: bool = u_get_AMUSERENR_Type_EN(state, tracer, s_0_16);
        // D s_0_18: write-var __AMUSERENR_EN <= s_0_17
        fn_state.u__AMUSERENR_EN = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call HCR_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_19);
        // S s_0_21: call _get_HCR_Type_TGE(s_0_20)
        let s_0_21: bool = u_get_HCR_Type_TGE(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_TGE <= s_0_21
        fn_state.u__HCR_TGE = s_0_21;
        // C s_0_23: const #104936u : u32
        let s_0_23: u32 = 104936;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HSTR_EL2_Type_T0(s_0_24)
        let s_0_25: bool = u_get_HSTR_EL2_Type_T0(state, tracer, s_0_24);
        // D s_0_26: write-var __HSTR_EL2_T0 <= s_0_25
        fn_state.u__HSTR_EL2_T0 = s_0_25;
        // C s_0_27: const #() : ()
        let s_0_27: () = ();
        // S s_0_28: call HSTR_read(s_0_27)
        let s_0_28: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_27);
        // S s_0_29: call _get_HSTR_Type_T0(s_0_28)
        let s_0_29: bool = u_get_HSTR_Type_T0(state, tracer, s_0_28);
        // D s_0_30: write-var __HSTR_T0 <= s_0_29
        fn_state.u__HSTR_T0 = s_0_29;
        // C s_0_31: const #11088u : u32
        let s_0_31: u32 = 11088;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_CPTR_EL2_Type_TAM(s_0_32)
        let s_0_33: bool = u_get_CPTR_EL2_Type_TAM(state, tracer, s_0_32);
        // D s_0_34: write-var __CPTR_EL2_TAM <= s_0_33
        fn_state.u__CPTR_EL2_TAM = s_0_33;
        // C s_0_35: const #() : ()
        let s_0_35: () = ();
        // S s_0_36: call HCPTR_read(s_0_35)
        let s_0_36: ProductType700c18a878c5601b = HCPTR_read(state, tracer, s_0_35);
        // S s_0_37: call _get_HCPTR_Type_TAM(s_0_36)
        let s_0_37: bool = u_get_HCPTR_Type_TAM(state, tracer, s_0_36);
        // D s_0_38: write-var __HCPTR_TAM <= s_0_37
        fn_state.u__HCPTR_TAM = s_0_37;
        // C s_0_39: const #90704u : u32
        let s_0_39: u32 = 90704;
        // D s_0_40: read-reg s_0_39:struct
        let s_0_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_39 as isize);
            tracer.read_register(s_0_39 as isize, value);
            value
        };
        // D s_0_41: call _get_SCR_EL3_Type_FGTEn(s_0_40)
        let s_0_41: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_40);
        // D s_0_42: write-var __SCR_EL3_FGTEn <= s_0_41
        fn_state.u__SCR_EL3_FGTEn = s_0_41;
        // C s_0_43: const #21760u : u32
        let s_0_43: u32 = 21760;
        // D s_0_44: read-reg s_0_43:struct
        let s_0_44: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_43 as isize);
            tracer.read_register(s_0_43 as isize, value);
            value
        };
        // D s_0_45: call _get_HAFGRTR_EL2_Type_AMEVCNTR02_EL0(s_0_44)
        let s_0_45: bool = u_get_HAFGRTR_EL2_Type_AMEVCNTR02_EL0(state, tracer, s_0_44);
        // D s_0_46: write-var __HAFGRTR_EL2_AMEVCNTR02_EL0 <= s_0_45
        fn_state.u__HAFGRTR_EL2_AMEVCNTR02_EL0 = s_0_45;
        // D s_0_47: read-var __PSTATE_EL:u8
        let s_0_47: u8 = fn_state.u__PSTATE_EL;
        // D s_0_48: cast zx s_0_47 -> bv
        let s_0_48: Bits = Bits::new(s_0_47 as u128, 2u16);
        // C s_0_49: const #448u : u32
        let s_0_49: u32 = 448;
        // D s_0_50: read-reg s_0_49:u8
        let s_0_50: u8 = {
            let value = state.read_register::<u8>(s_0_49 as isize);
            tracer.read_register(s_0_49 as isize, value);
            value
        };
        // D s_0_51: cast zx s_0_50 -> bv
        let s_0_51: Bits = Bits::new(s_0_50 as u128, 2u16);
        // D s_0_52: cmp-eq s_0_48 s_0_51
        let s_0_52: bool = ((s_0_48) == (s_0_51));
        // N s_0_53: branch s_0_52 b106 b1
        if s_0_52 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var __PSTATE_EL:u8
        let s_1_0: u8 = fn_state.u__PSTATE_EL;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #440u : u32
        let s_1_2: u32 = 440;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // D s_1_5: cmp-eq s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) == (s_1_4));
        // N s_1_6: branch s_1_5 b37 b2
        if s_1_5 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var __PSTATE_EL:u8
        let s_2_0: u8 = fn_state.u__PSTATE_EL;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #432u : u32
        let s_2_2: u32 = 432;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cmp-eq s_2_1 s_2_4
        let s_2_5: bool = ((s_2_1) == (s_2_4));
        // N s_2_6: branch s_2_5 b6 b3
        if s_2_5 {
            return block_6(state, tracer, fn_state);
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
        // C s_3_2: const #424u : u32
        let s_3_2: u32 = 424;
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
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // C s_5_1: const #14624u : u32
        let s_5_1: u32 = 14624;
        // D s_5_2: read-reg s_5_1:[struct; 4]
        let s_5_2: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: read-element s_5_2[s_5_0]
        let s_5_3: ProductType5c790c8ef59cc8b2 = s_5_2[(s_5_0) as usize];
        // D s_5_4: write-var ga#200471 <= s_5_3
        fn_state.ga_200471 = s_5_3;
        // D s_5_5: read-var ga#200471.0:struct
        let s_5_5: u64 = fn_state.ga_200471._0;
        // C s_5_6: const #32s : i
        let s_5_6: i128 = 32;
        // D s_5_7: cast zx s_5_5 -> bv
        let s_5_7: Bits = Bits::new(s_5_5 as u128, 64u16);
        // C s_5_8: const #1s : i64
        let s_5_8: i64 = 1;
        // C s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // C s_5_10: const #31s : i
        let s_5_10: i128 = 31;
        // C s_5_11: add s_5_10 s_5_9
        let s_5_11: i128 = (s_5_10 + s_5_9);
        // D s_5_12: bit-extract s_5_7 s_5_6 s_5_11
        let s_5_12: Bits = (Bits::new(
            ((s_5_7) >> (s_5_6)).value(),
            u16::try_from(s_5_11).unwrap(),
        ));
        // D s_5_13: cast reint s_5_12 -> u32
        let s_5_13: u32 = (s_5_12.value() as u32);
        // C s_5_14: const #2s : i
        let s_5_14: i128 = 2;
        // C s_5_15: const #14624u : u32
        let s_5_15: u32 = 14624;
        // D s_5_16: read-reg s_5_15:[struct; 4]
        let s_5_16: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_5_15 as isize);
            tracer.read_register(s_5_15 as isize, value);
            value
        };
        // D s_5_17: read-element s_5_16[s_5_14]
        let s_5_17: ProductType5c790c8ef59cc8b2 = s_5_16[(s_5_14) as usize];
        // D s_5_18: write-var ga#200473 <= s_5_17
        fn_state.ga_200473 = s_5_17;
        // D s_5_19: read-var ga#200473.0:struct
        let s_5_19: u64 = fn_state.ga_200473._0;
        // C s_5_20: const #0s : i
        let s_5_20: i128 = 0;
        // D s_5_21: cast zx s_5_19 -> bv
        let s_5_21: Bits = Bits::new(s_5_19 as u128, 64u16);
        // C s_5_22: const #1s : i64
        let s_5_22: i64 = 1;
        // C s_5_23: cast zx s_5_22 -> i
        let s_5_23: i128 = (i128::try_from(s_5_22).unwrap());
        // C s_5_24: const #31s : i
        let s_5_24: i128 = 31;
        // C s_5_25: add s_5_24 s_5_23
        let s_5_25: i128 = (s_5_24 + s_5_23);
        // D s_5_26: bit-extract s_5_21 s_5_20 s_5_25
        let s_5_26: Bits = (Bits::new(
            ((s_5_21) >> (s_5_20)).value(),
            u16::try_from(s_5_25).unwrap(),
        ));
        // D s_5_27: cast reint s_5_26 -> u32
        let s_5_27: u32 = (s_5_26.value() as u32);
        // D s_5_28: create-product struct = ["s_5_13", "s_5_27"]
        let s_5_28: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_5_13,
            _1: s_5_27,
        };
        // D s_5_29: write-var ga#200477 <= s_5_28
        fn_state.ga_200477 = s_5_28;
        // D s_5_30: read-var ga#200477.0:struct
        let s_5_30: u32 = fn_state.ga_200477._0;
        // D s_5_31: read-var ga#200477.1:struct
        let s_5_31: u32 = fn_state.ga_200477._1;
        // D s_5_32: read-var t2:i
        let s_5_32: i128 = fn_state.t2;
        // D s_5_33: call R_set(s_5_32, s_5_30)
        let s_5_33: () = R_set(state, tracer, s_5_32, s_5_30);
        // D s_5_34: read-var t:i
        let s_5_34: i128 = fn_state.t;
        // D s_5_35: call R_set(s_5_34, s_5_31)
        let s_5_35: () = R_set(state, tracer, s_5_34, s_5_31);
        // N s_5_36: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call Halted(s_6_0)
        let s_6_1: bool = Halted(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b36 b7
        if s_6_1 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#120317 <= s_7_0
        fn_state.gs_120317 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#120317:u8
        let s_8_0: bool = fn_state.gs_120317;
        // N s_8_1: branch s_8_0 b35 b9
        if s_8_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#120318 <= s_9_0
        fn_state.gs_120318 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#120318:u8
        let s_10_0: bool = fn_state.gs_120318;
        // N s_10_1: branch s_10_0 b34 b11
        if s_10_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#120319 <= s_11_0
        fn_state.gs_120319 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#120319:u8
        let s_12_0: bool = fn_state.gs_120319;
        // N s_12_1: branch s_12_0 b33 b13
        if s_12_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#120320 <= s_13_0
        fn_state.gs_120320 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#120320:u8
        let s_14_0: bool = fn_state.gs_120320;
        // N s_14_1: branch s_14_0 b32 b15
        if s_14_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#120321 <= s_15_0
        fn_state.gs_120321 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#120321:u8
        let s_16_0: bool = fn_state.gs_120321;
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
    ) -> () {
        // C s_17_0: const #424u : u32
        let s_17_0: u32 = 424;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // D s_17_3: cmp-lt s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) < (s_17_2));
        // N s_17_4: branch s_17_3 b30 b18
        if s_17_3 {
            return block_30(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#120322 <= s_18_0
        fn_state.gs_120322 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#120322:u8
        let s_19_0: bool = fn_state.gs_120322;
        // N s_19_1: branch s_19_0 b29 b20
        if s_19_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#120323 <= s_20_0
        fn_state.gs_120323 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#120323:u8
        let s_21_0: bool = fn_state.gs_120323;
        // N s_21_1: branch s_21_0 b23 b22
        if s_21_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #2s : i
        let s_22_0: i128 = 2;
        // C s_22_1: const #14624u : u32
        let s_22_1: u32 = 14624;
        // D s_22_2: read-reg s_22_1:[struct; 4]
        let s_22_2: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_22_1 as isize);
            tracer.read_register(s_22_1 as isize, value);
            value
        };
        // D s_22_3: read-element s_22_2[s_22_0]
        let s_22_3: ProductType5c790c8ef59cc8b2 = s_22_2[(s_22_0) as usize];
        // D s_22_4: write-var ga#200463 <= s_22_3
        fn_state.ga_200463 = s_22_3;
        // D s_22_5: read-var ga#200463.0:struct
        let s_22_5: u64 = fn_state.ga_200463._0;
        // C s_22_6: const #32s : i
        let s_22_6: i128 = 32;
        // D s_22_7: cast zx s_22_5 -> bv
        let s_22_7: Bits = Bits::new(s_22_5 as u128, 64u16);
        // C s_22_8: const #1s : i64
        let s_22_8: i64 = 1;
        // C s_22_9: cast zx s_22_8 -> i
        let s_22_9: i128 = (i128::try_from(s_22_8).unwrap());
        // C s_22_10: const #31s : i
        let s_22_10: i128 = 31;
        // C s_22_11: add s_22_10 s_22_9
        let s_22_11: i128 = (s_22_10 + s_22_9);
        // D s_22_12: bit-extract s_22_7 s_22_6 s_22_11
        let s_22_12: Bits = (Bits::new(
            ((s_22_7) >> (s_22_6)).value(),
            u16::try_from(s_22_11).unwrap(),
        ));
        // D s_22_13: cast reint s_22_12 -> u32
        let s_22_13: u32 = (s_22_12.value() as u32);
        // C s_22_14: const #2s : i
        let s_22_14: i128 = 2;
        // C s_22_15: const #14624u : u32
        let s_22_15: u32 = 14624;
        // D s_22_16: read-reg s_22_15:[struct; 4]
        let s_22_16: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_22_15 as isize);
            tracer.read_register(s_22_15 as isize, value);
            value
        };
        // D s_22_17: read-element s_22_16[s_22_14]
        let s_22_17: ProductType5c790c8ef59cc8b2 = s_22_16[(s_22_14) as usize];
        // D s_22_18: write-var ga#200465 <= s_22_17
        fn_state.ga_200465 = s_22_17;
        // D s_22_19: read-var ga#200465.0:struct
        let s_22_19: u64 = fn_state.ga_200465._0;
        // C s_22_20: const #0s : i
        let s_22_20: i128 = 0;
        // D s_22_21: cast zx s_22_19 -> bv
        let s_22_21: Bits = Bits::new(s_22_19 as u128, 64u16);
        // C s_22_22: const #1s : i64
        let s_22_22: i64 = 1;
        // C s_22_23: cast zx s_22_22 -> i
        let s_22_23: i128 = (i128::try_from(s_22_22).unwrap());
        // C s_22_24: const #31s : i
        let s_22_24: i128 = 31;
        // C s_22_25: add s_22_24 s_22_23
        let s_22_25: i128 = (s_22_24 + s_22_23);
        // D s_22_26: bit-extract s_22_21 s_22_20 s_22_25
        let s_22_26: Bits = (Bits::new(
            ((s_22_21) >> (s_22_20)).value(),
            u16::try_from(s_22_25).unwrap(),
        ));
        // D s_22_27: cast reint s_22_26 -> u32
        let s_22_27: u32 = (s_22_26.value() as u32);
        // D s_22_28: create-product struct = ["s_22_13", "s_22_27"]
        let s_22_28: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_22_13,
            _1: s_22_27,
        };
        // D s_22_29: write-var ga#200469 <= s_22_28
        fn_state.ga_200469 = s_22_28;
        // D s_22_30: read-var ga#200469.0:struct
        let s_22_30: u32 = fn_state.ga_200469._0;
        // D s_22_31: read-var ga#200469.1:struct
        let s_22_31: u32 = fn_state.ga_200469._1;
        // D s_22_32: read-var t2:i
        let s_22_32: i128 = fn_state.t2;
        // D s_22_33: call R_set(s_22_32, s_22_30)
        let s_22_33: () = R_set(state, tracer, s_22_32, s_22_30);
        // D s_22_34: read-var t:i
        let s_22_34: i128 = fn_state.t;
        // D s_22_35: call R_set(s_22_34, s_22_31)
        let s_22_35: () = R_set(state, tracer, s_22_34, s_22_31);
        // N s_22_36: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call Halted(s_23_0)
        let s_23_1: bool = Halted(state, tracer, s_23_0);
        // N s_23_2: branch s_23_1 b28 b24
        if s_23_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#120334 <= s_24_0
        fn_state.gs_120334 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#120334:u8
        let s_25_0: bool = fn_state.gs_120334;
        // N s_25_1: branch s_25_0 b27 b26
        if s_25_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #4u : u8
        let s_26_0: u8 = 4;
        // C s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 8u16);
        // C s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #424u : u32
        let s_26_5: u32 = 424;
        // D s_26_6: read-reg s_26_5:u8
        let s_26_6: u8 = {
            let value = state.read_register::<u8>(s_26_5 as isize);
            tracer.read_register(s_26_5 as isize, value);
            value
        };
        // D s_26_7: call AArch64_AArch32SystemAccessTrap(s_26_6, s_26_4)
        let s_26_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_26_6, s_26_4);
        // N s_26_8: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EDSCR_read(s_28_0)
        let s_28_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_28_0);
        // S s_28_2: call _get_EDSCR_Type_SDD(s_28_1)
        let s_28_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_28_1);
        // S s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // S s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#120334 <= s_28_6
        fn_state.gs_120334 = s_28_6;
        // N s_28_8: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __CPTR_EL3_TAM:u8
        let s_29_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#120323 <= s_29_4
        fn_state.gs_120323 = s_29_4;
        // N s_29_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #424u : u32
        let s_30_0: u32 = 424;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call ELUsingAArch32(s_30_1)
        let s_30_2: bool = ELUsingAArch32(state, tracer, s_30_1);
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // D s_30_4: write-var gs#120322 <= s_30_3
        fn_state.gs_120322 = s_30_3;
        // N s_30_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __CPTR_EL3_TAM:u8
        let s_32_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#120321 <= s_32_4
        fn_state.gs_120321 = s_32_4;
        // N s_32_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #424u : u32
        let s_33_0: u32 = 424;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call ELUsingAArch32(s_33_1)
        let s_33_2: bool = ELUsingAArch32(state, tracer, s_33_1);
        // D s_33_3: not s_33_2
        let s_33_3: bool = !s_33_2;
        // D s_33_4: write-var gs#120320 <= s_33_3
        fn_state.gs_120320 = s_33_3;
        // N s_33_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_34_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_34_1: call __IMPDEF_boolean(s_34_0)
        let s_34_1: bool = u__IMPDEF_boolean(state, tracer, s_34_0);
        // D s_34_2: write-var gs#120319 <= s_34_1
        fn_state.gs_120319 = s_34_1;
        // N s_34_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call EDSCR_read(s_35_0)
        let s_35_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_35_0);
        // S s_35_2: call _get_EDSCR_Type_SDD(s_35_1)
        let s_35_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_35_1);
        // S s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // C s_35_4: const #1u : u8
        let s_35_4: bool = true;
        // C s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 1u16);
        // S s_35_6: cmp-eq s_35_3 s_35_5
        let s_35_6: bool = ((s_35_3) == (s_35_5));
        // D s_35_7: write-var gs#120318 <= s_35_6
        fn_state.gs_120318 = s_35_6;
        // N s_35_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #424u : u32
        let s_36_0: u32 = 424;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // C s_36_2: const #2u : u8
        let s_36_2: u8 = 2;
        // D s_36_3: cmp-lt s_36_1 s_36_2
        let s_36_3: bool = ((s_36_1) < (s_36_2));
        // D s_36_4: write-var gs#120317 <= s_36_3
        fn_state.gs_120317 = s_36_3;
        // N s_36_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call Halted(s_37_0)
        let s_37_1: bool = Halted(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b105 b38
        if s_37_1 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#120335 <= s_38_0
        fn_state.gs_120335 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#120335:u8
        let s_39_0: bool = fn_state.gs_120335;
        // N s_39_1: branch s_39_0 b104 b40
        if s_39_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#120336 <= s_40_0
        fn_state.gs_120336 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#120336:u8
        let s_41_0: bool = fn_state.gs_120336;
        // N s_41_1: branch s_41_0 b103 b42
        if s_41_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#120337 <= s_42_0
        fn_state.gs_120337 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#120337:u8
        let s_43_0: bool = fn_state.gs_120337;
        // N s_43_1: branch s_43_0 b102 b44
        if s_43_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#120338 <= s_44_0
        fn_state.gs_120338 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#120338:u8
        let s_45_0: bool = fn_state.gs_120338;
        // N s_45_1: branch s_45_0 b101 b46
        if s_45_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#120339 <= s_46_0
        fn_state.gs_120339 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#120339:u8
        let s_47_0: bool = fn_state.gs_120339;
        // N s_47_1: branch s_47_0 b100 b48
        if s_47_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EL2Enabled(s_48_0)
        let s_48_1: bool = EL2Enabled(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b99 b49
        if s_48_1 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#120340 <= s_49_0
        fn_state.gs_120340 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#120340:u8
        let s_50_0: bool = fn_state.gs_120340;
        // N s_50_1: branch s_50_0 b98 b51
        if s_50_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#120343 <= s_51_0
        fn_state.gs_120343 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#120343:u8
        let s_52_0: bool = fn_state.gs_120343;
        // N s_52_1: branch s_52_0 b97 b53
        if s_52_0 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#120344 <= s_53_0
        fn_state.gs_120344 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#120344:u8
        let s_54_0: bool = fn_state.gs_120344;
        // N s_54_1: branch s_54_0 b96 b55
        if s_54_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call EL2Enabled(s_55_0)
        let s_55_1: bool = EL2Enabled(state, tracer, s_55_0);
        // N s_55_2: branch s_55_1 b95 b56
        if s_55_1 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#120345 <= s_56_0
        fn_state.gs_120345 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#120345:u8
        let s_57_0: bool = fn_state.gs_120345;
        // N s_57_1: branch s_57_0 b94 b58
        if s_57_0 {
            return block_94(state, tracer, fn_state);
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
        // D s_58_1: write-var gs#120348 <= s_58_0
        fn_state.gs_120348 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#120348:u8
        let s_59_0: bool = fn_state.gs_120348;
        // N s_59_1: branch s_59_0 b93 b60
        if s_59_0 {
            return block_93(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#120349 <= s_60_0
        fn_state.gs_120349 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#120349:u8
        let s_61_0: bool = fn_state.gs_120349;
        // N s_61_1: branch s_61_0 b92 b62
        if s_61_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call EL2Enabled(s_62_0)
        let s_62_1: bool = EL2Enabled(state, tracer, s_62_0);
        // N s_62_2: branch s_62_1 b91 b63
        if s_62_1 {
            return block_91(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#120350 <= s_63_0
        fn_state.gs_120350 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#120350:u8
        let s_64_0: bool = fn_state.gs_120350;
        // N s_64_1: branch s_64_0 b90 b65
        if s_64_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#120351 <= s_65_0
        fn_state.gs_120351 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#120351:u8
        let s_66_0: bool = fn_state.gs_120351;
        // N s_66_1: branch s_66_0 b89 b67
        if s_66_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call EL2Enabled(s_67_0)
        let s_67_1: bool = EL2Enabled(state, tracer, s_67_0);
        // N s_67_2: branch s_67_1 b88 b68
        if s_67_1 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#120352 <= s_68_0
        fn_state.gs_120352 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#120352:u8
        let s_69_0: bool = fn_state.gs_120352;
        // N s_69_1: branch s_69_0 b87 b70
        if s_69_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#120353 <= s_70_0
        fn_state.gs_120353 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#120353:u8
        let s_71_0: bool = fn_state.gs_120353;
        // N s_71_1: branch s_71_0 b86 b72
        if s_71_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #424u : u32
        let s_72_0: u32 = 424;
        // D s_72_1: read-reg s_72_0:u8
        let s_72_1: u8 = {
            let value = state.read_register::<u8>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // C s_72_2: const #2u : u8
        let s_72_2: u8 = 2;
        // D s_72_3: cmp-lt s_72_1 s_72_2
        let s_72_3: bool = ((s_72_1) < (s_72_2));
        // N s_72_4: branch s_72_3 b85 b73
        if s_72_3 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#120354 <= s_73_0
        fn_state.gs_120354 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#120354:u8
        let s_74_0: bool = fn_state.gs_120354;
        // N s_74_1: branch s_74_0 b84 b75
        if s_74_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#120355 <= s_75_0
        fn_state.gs_120355 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#120355:u8
        let s_76_0: bool = fn_state.gs_120355;
        // N s_76_1: branch s_76_0 b78 b77
        if s_76_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #2s : i
        let s_77_0: i128 = 2;
        // C s_77_1: const #14624u : u32
        let s_77_1: u32 = 14624;
        // D s_77_2: read-reg s_77_1:[struct; 4]
        let s_77_2: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_77_1 as isize);
            tracer.read_register(s_77_1 as isize, value);
            value
        };
        // D s_77_3: read-element s_77_2[s_77_0]
        let s_77_3: ProductType5c790c8ef59cc8b2 = s_77_2[(s_77_0) as usize];
        // D s_77_4: write-var ga#200435 <= s_77_3
        fn_state.ga_200435 = s_77_3;
        // D s_77_5: read-var ga#200435.0:struct
        let s_77_5: u64 = fn_state.ga_200435._0;
        // C s_77_6: const #32s : i
        let s_77_6: i128 = 32;
        // D s_77_7: cast zx s_77_5 -> bv
        let s_77_7: Bits = Bits::new(s_77_5 as u128, 64u16);
        // C s_77_8: const #1s : i64
        let s_77_8: i64 = 1;
        // C s_77_9: cast zx s_77_8 -> i
        let s_77_9: i128 = (i128::try_from(s_77_8).unwrap());
        // C s_77_10: const #31s : i
        let s_77_10: i128 = 31;
        // C s_77_11: add s_77_10 s_77_9
        let s_77_11: i128 = (s_77_10 + s_77_9);
        // D s_77_12: bit-extract s_77_7 s_77_6 s_77_11
        let s_77_12: Bits = (Bits::new(
            ((s_77_7) >> (s_77_6)).value(),
            u16::try_from(s_77_11).unwrap(),
        ));
        // D s_77_13: cast reint s_77_12 -> u32
        let s_77_13: u32 = (s_77_12.value() as u32);
        // C s_77_14: const #2s : i
        let s_77_14: i128 = 2;
        // C s_77_15: const #14624u : u32
        let s_77_15: u32 = 14624;
        // D s_77_16: read-reg s_77_15:[struct; 4]
        let s_77_16: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_77_15 as isize);
            tracer.read_register(s_77_15 as isize, value);
            value
        };
        // D s_77_17: read-element s_77_16[s_77_14]
        let s_77_17: ProductType5c790c8ef59cc8b2 = s_77_16[(s_77_14) as usize];
        // D s_77_18: write-var ga#200437 <= s_77_17
        fn_state.ga_200437 = s_77_17;
        // D s_77_19: read-var ga#200437.0:struct
        let s_77_19: u64 = fn_state.ga_200437._0;
        // C s_77_20: const #0s : i
        let s_77_20: i128 = 0;
        // D s_77_21: cast zx s_77_19 -> bv
        let s_77_21: Bits = Bits::new(s_77_19 as u128, 64u16);
        // C s_77_22: const #1s : i64
        let s_77_22: i64 = 1;
        // C s_77_23: cast zx s_77_22 -> i
        let s_77_23: i128 = (i128::try_from(s_77_22).unwrap());
        // C s_77_24: const #31s : i
        let s_77_24: i128 = 31;
        // C s_77_25: add s_77_24 s_77_23
        let s_77_25: i128 = (s_77_24 + s_77_23);
        // D s_77_26: bit-extract s_77_21 s_77_20 s_77_25
        let s_77_26: Bits = (Bits::new(
            ((s_77_21) >> (s_77_20)).value(),
            u16::try_from(s_77_25).unwrap(),
        ));
        // D s_77_27: cast reint s_77_26 -> u32
        let s_77_27: u32 = (s_77_26.value() as u32);
        // D s_77_28: create-product struct = ["s_77_13", "s_77_27"]
        let s_77_28: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_77_13,
            _1: s_77_27,
        };
        // D s_77_29: write-var ga#200441 <= s_77_28
        fn_state.ga_200441 = s_77_28;
        // D s_77_30: read-var ga#200441.0:struct
        let s_77_30: u32 = fn_state.ga_200441._0;
        // D s_77_31: read-var ga#200441.1:struct
        let s_77_31: u32 = fn_state.ga_200441._1;
        // D s_77_32: read-var t2:i
        let s_77_32: i128 = fn_state.t2;
        // D s_77_33: call R_set(s_77_32, s_77_30)
        let s_77_33: () = R_set(state, tracer, s_77_32, s_77_30);
        // D s_77_34: read-var t:i
        let s_77_34: i128 = fn_state.t;
        // D s_77_35: call R_set(s_77_34, s_77_31)
        let s_77_35: () = R_set(state, tracer, s_77_34, s_77_31);
        // N s_77_36: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #() : ()
        let s_78_0: () = ();
        // S s_78_1: call Halted(s_78_0)
        let s_78_1: bool = Halted(state, tracer, s_78_0);
        // N s_78_2: branch s_78_1 b83 b79
        if s_78_1 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#120366 <= s_79_0
        fn_state.gs_120366 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#120366:u8
        let s_80_0: bool = fn_state.gs_120366;
        // N s_80_1: branch s_80_0 b82 b81
        if s_80_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #4u : u8
        let s_81_0: u8 = 4;
        // C s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 8u16);
        // C s_81_2: cast zx s_81_1 -> i
        let s_81_2: i128 = (s_81_1.value() as i128);
        // C s_81_3: cast reint s_81_2 -> i64
        let s_81_3: i64 = (s_81_2 as i64);
        // C s_81_4: cast zx s_81_3 -> i
        let s_81_4: i128 = (i128::try_from(s_81_3).unwrap());
        // C s_81_5: const #424u : u32
        let s_81_5: u32 = 424;
        // D s_81_6: read-reg s_81_5:u8
        let s_81_6: u8 = {
            let value = state.read_register::<u8>(s_81_5 as isize);
            tracer.read_register(s_81_5 as isize, value);
            value
        };
        // D s_81_7: call AArch64_AArch32SystemAccessTrap(s_81_6, s_81_4)
        let s_81_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_81_6, s_81_4);
        // N s_81_8: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_82_0: panic
        panic!("{:?}", ());
        // N s_82_1: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call EDSCR_read(s_83_0)
        let s_83_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_83_0);
        // S s_83_2: call _get_EDSCR_Type_SDD(s_83_1)
        let s_83_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_83_1);
        // S s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 1u16);
        // C s_83_4: const #1u : u8
        let s_83_4: bool = true;
        // C s_83_5: cast zx s_83_4 -> bv
        let s_83_5: Bits = Bits::new(s_83_4 as u128, 1u16);
        // S s_83_6: cmp-eq s_83_3 s_83_5
        let s_83_6: bool = ((s_83_3) == (s_83_5));
        // D s_83_7: write-var gs#120366 <= s_83_6
        fn_state.gs_120366 = s_83_6;
        // N s_83_8: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __CPTR_EL3_TAM:u8
        let s_84_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#120355 <= s_84_4
        fn_state.gs_120355 = s_84_4;
        // N s_84_6: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #424u : u32
        let s_85_0: u32 = 424;
        // D s_85_1: read-reg s_85_0:u8
        let s_85_1: u8 = {
            let value = state.read_register::<u8>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call ELUsingAArch32(s_85_1)
        let s_85_2: bool = ELUsingAArch32(state, tracer, s_85_1);
        // D s_85_3: not s_85_2
        let s_85_3: bool = !s_85_2;
        // D s_85_4: write-var gs#120354 <= s_85_3
        fn_state.gs_120354 = s_85_3;
        // N s_85_5: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #4u : u8
        let s_86_0: u8 = 4;
        // C s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 8u16);
        // C s_86_2: cast zx s_86_1 -> i
        let s_86_2: i128 = (s_86_1.value() as i128);
        // C s_86_3: cast reint s_86_2 -> i64
        let s_86_3: i64 = (s_86_2 as i64);
        // C s_86_4: cast zx s_86_3 -> i
        let s_86_4: i128 = (i128::try_from(s_86_3).unwrap());
        // S s_86_5: call AArch32_TakeHypTrapException(s_86_4)
        let s_86_5: () = AArch32_TakeHypTrapException(state, tracer, s_86_4);
        // N s_86_6: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var __HCPTR_TAM:u8
        let s_87_0: bool = fn_state.u__HCPTR_TAM;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 1u16);
        // C s_87_2: const #1u : u8
        let s_87_2: bool = true;
        // C s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 1u16);
        // D s_87_4: cmp-eq s_87_1 s_87_3
        let s_87_4: bool = ((s_87_1) == (s_87_3));
        // D s_87_5: write-var gs#120353 <= s_87_4
        fn_state.gs_120353 = s_87_4;
        // N s_87_6: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #432u : u32
        let s_88_0: u32 = 432;
        // D s_88_1: read-reg s_88_0:u8
        let s_88_1: u8 = {
            let value = state.read_register::<u8>(s_88_0 as isize);
            tracer.read_register(s_88_0 as isize, value);
            value
        };
        // D s_88_2: call ELUsingAArch32(s_88_1)
        let s_88_2: bool = ELUsingAArch32(state, tracer, s_88_1);
        // D s_88_3: write-var gs#120352 <= s_88_2
        fn_state.gs_120352 = s_88_2;
        // N s_88_4: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #4u : u8
        let s_89_0: u8 = 4;
        // C s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 8u16);
        // C s_89_2: cast zx s_89_1 -> i
        let s_89_2: i128 = (s_89_1.value() as i128);
        // C s_89_3: cast reint s_89_2 -> i64
        let s_89_3: i64 = (s_89_2 as i64);
        // C s_89_4: cast zx s_89_3 -> i
        let s_89_4: i128 = (i128::try_from(s_89_3).unwrap());
        // C s_89_5: const #432u : u32
        let s_89_5: u32 = 432;
        // D s_89_6: read-reg s_89_5:u8
        let s_89_6: u8 = {
            let value = state.read_register::<u8>(s_89_5 as isize);
            tracer.read_register(s_89_5 as isize, value);
            value
        };
        // D s_89_7: call AArch64_AArch32SystemAccessTrap(s_89_6, s_89_4)
        let s_89_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_89_6, s_89_4);
        // N s_89_8: return
        return;
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var __CPTR_EL2_TAM:u8
        let s_90_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 1u16);
        // C s_90_2: const #1u : u8
        let s_90_2: bool = true;
        // C s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // D s_90_5: write-var gs#120351 <= s_90_4
        fn_state.gs_120351 = s_90_4;
        // N s_90_6: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #432u : u32
        let s_91_0: u32 = 432;
        // D s_91_1: read-reg s_91_0:u8
        let s_91_1: u8 = {
            let value = state.read_register::<u8>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // D s_91_2: call ELUsingAArch32(s_91_1)
        let s_91_2: bool = ELUsingAArch32(state, tracer, s_91_1);
        // D s_91_3: not s_91_2
        let s_91_3: bool = !s_91_2;
        // D s_91_4: write-var gs#120350 <= s_91_3
        fn_state.gs_120350 = s_91_3;
        // N s_91_5: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #4u : u8
        let s_92_0: u8 = 4;
        // C s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 8u16);
        // C s_92_2: cast zx s_92_1 -> i
        let s_92_2: i128 = (s_92_1.value() as i128);
        // C s_92_3: cast reint s_92_2 -> i64
        let s_92_3: i64 = (s_92_2 as i64);
        // C s_92_4: cast zx s_92_3 -> i
        let s_92_4: i128 = (i128::try_from(s_92_3).unwrap());
        // S s_92_5: call AArch32_TakeHypTrapException(s_92_4)
        let s_92_5: () = AArch32_TakeHypTrapException(state, tracer, s_92_4);
        // N s_92_6: return
        return;
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var __HSTR_T0:u8
        let s_93_0: bool = fn_state.u__HSTR_T0;
        // D s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 1u16);
        // C s_93_2: const #1u : u8
        let s_93_2: bool = true;
        // C s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // D s_93_4: cmp-eq s_93_1 s_93_3
        let s_93_4: bool = ((s_93_1) == (s_93_3));
        // D s_93_5: write-var gs#120349 <= s_93_4
        fn_state.gs_120349 = s_93_4;
        // N s_93_6: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #1u : u8
        let s_94_0: bool = true;
        // D s_94_1: write-var gs#120348 <= s_94_0
        fn_state.gs_120348 = s_94_0;
        // N s_94_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #432u : u32
        let s_95_0: u32 = 432;
        // D s_95_1: read-reg s_95_0:u8
        let s_95_1: u8 = {
            let value = state.read_register::<u8>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // D s_95_2: call ELUsingAArch32(s_95_1)
        let s_95_2: bool = ELUsingAArch32(state, tracer, s_95_1);
        // D s_95_3: write-var gs#120345 <= s_95_2
        fn_state.gs_120345 = s_95_2;
        // N s_95_4: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #4u : u8
        let s_96_0: u8 = 4;
        // C s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 8u16);
        // C s_96_2: cast zx s_96_1 -> i
        let s_96_2: i128 = (s_96_1.value() as i128);
        // C s_96_3: cast reint s_96_2 -> i64
        let s_96_3: i64 = (s_96_2 as i64);
        // C s_96_4: cast zx s_96_3 -> i
        let s_96_4: i128 = (i128::try_from(s_96_3).unwrap());
        // C s_96_5: const #432u : u32
        let s_96_5: u32 = 432;
        // D s_96_6: read-reg s_96_5:u8
        let s_96_6: u8 = {
            let value = state.read_register::<u8>(s_96_5 as isize);
            tracer.read_register(s_96_5 as isize, value);
            value
        };
        // D s_96_7: call AArch64_AArch32SystemAccessTrap(s_96_6, s_96_4)
        let s_96_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_96_6, s_96_4);
        // N s_96_8: return
        return;
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var __HSTR_EL2_T0:u8
        let s_97_0: bool = fn_state.u__HSTR_EL2_T0;
        // D s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 1u16);
        // C s_97_2: const #1u : u8
        let s_97_2: bool = true;
        // C s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 1u16);
        // D s_97_4: cmp-eq s_97_1 s_97_3
        let s_97_4: bool = ((s_97_1) == (s_97_3));
        // D s_97_5: write-var gs#120344 <= s_97_4
        fn_state.gs_120344 = s_97_4;
        // N s_97_6: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #1u : u8
        let s_98_0: bool = true;
        // D s_98_1: write-var gs#120343 <= s_98_0
        fn_state.gs_120343 = s_98_0;
        // N s_98_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #432u : u32
        let s_99_0: u32 = 432;
        // D s_99_1: read-reg s_99_0:u8
        let s_99_1: u8 = {
            let value = state.read_register::<u8>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // D s_99_2: call ELUsingAArch32(s_99_1)
        let s_99_2: bool = ELUsingAArch32(state, tracer, s_99_1);
        // D s_99_3: not s_99_2
        let s_99_3: bool = !s_99_2;
        // D s_99_4: write-var gs#120340 <= s_99_3
        fn_state.gs_120340 = s_99_3;
        // N s_99_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_100_0: panic
        panic!("{:?}", ());
        // N s_100_1: return
        return;
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var __CPTR_EL3_TAM:u8
        let s_101_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 1u16);
        // C s_101_2: const #1u : u8
        let s_101_2: bool = true;
        // C s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 1u16);
        // D s_101_4: cmp-eq s_101_1 s_101_3
        let s_101_4: bool = ((s_101_1) == (s_101_3));
        // D s_101_5: write-var gs#120339 <= s_101_4
        fn_state.gs_120339 = s_101_4;
        // N s_101_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #424u : u32
        let s_102_0: u32 = 424;
        // D s_102_1: read-reg s_102_0:u8
        let s_102_1: u8 = {
            let value = state.read_register::<u8>(s_102_0 as isize);
            tracer.read_register(s_102_0 as isize, value);
            value
        };
        // D s_102_2: call ELUsingAArch32(s_102_1)
        let s_102_2: bool = ELUsingAArch32(state, tracer, s_102_1);
        // D s_102_3: not s_102_2
        let s_102_3: bool = !s_102_2;
        // D s_102_4: write-var gs#120338 <= s_102_3
        fn_state.gs_120338 = s_102_3;
        // N s_102_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_103_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_103_1: call __IMPDEF_boolean(s_103_0)
        let s_103_1: bool = u__IMPDEF_boolean(state, tracer, s_103_0);
        // D s_103_2: write-var gs#120337 <= s_103_1
        fn_state.gs_120337 = s_103_1;
        // N s_103_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #() : ()
        let s_104_0: () = ();
        // S s_104_1: call EDSCR_read(s_104_0)
        let s_104_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_104_0);
        // S s_104_2: call _get_EDSCR_Type_SDD(s_104_1)
        let s_104_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_104_1);
        // S s_104_3: cast zx s_104_2 -> bv
        let s_104_3: Bits = Bits::new(s_104_2 as u128, 1u16);
        // C s_104_4: const #1u : u8
        let s_104_4: bool = true;
        // C s_104_5: cast zx s_104_4 -> bv
        let s_104_5: Bits = Bits::new(s_104_4 as u128, 1u16);
        // S s_104_6: cmp-eq s_104_3 s_104_5
        let s_104_6: bool = ((s_104_3) == (s_104_5));
        // D s_104_7: write-var gs#120336 <= s_104_6
        fn_state.gs_120336 = s_104_6;
        // N s_104_8: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #424u : u32
        let s_105_0: u32 = 424;
        // D s_105_1: read-reg s_105_0:u8
        let s_105_1: u8 = {
            let value = state.read_register::<u8>(s_105_0 as isize);
            tracer.read_register(s_105_0 as isize, value);
            value
        };
        // C s_105_2: const #2u : u8
        let s_105_2: u8 = 2;
        // D s_105_3: cmp-lt s_105_1 s_105_2
        let s_105_3: bool = ((s_105_1) < (s_105_2));
        // D s_105_4: write-var gs#120335 <= s_105_3
        fn_state.gs_120335 = s_105_3;
        // N s_105_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #() : ()
        let s_106_0: () = ();
        // S s_106_1: call Halted(s_106_0)
        let s_106_1: bool = Halted(state, tracer, s_106_0);
        // N s_106_2: branch s_106_1 b231 b107
        if s_106_1 {
            return block_231(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#120367 <= s_107_0
        fn_state.gs_120367 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#120367:u8
        let s_108_0: bool = fn_state.gs_120367;
        // N s_108_1: branch s_108_0 b230 b109
        if s_108_0 {
            return block_230(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#120368 <= s_109_0
        fn_state.gs_120368 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#120368:u8
        let s_110_0: bool = fn_state.gs_120368;
        // N s_110_1: branch s_110_0 b229 b111
        if s_110_0 {
            return block_229(state, tracer, fn_state);
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
        // D s_111_1: write-var gs#120369 <= s_111_0
        fn_state.gs_120369 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#120369:u8
        let s_112_0: bool = fn_state.gs_120369;
        // N s_112_1: branch s_112_0 b228 b113
        if s_112_0 {
            return block_228(state, tracer, fn_state);
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
        // D s_113_1: write-var gs#120370 <= s_113_0
        fn_state.gs_120370 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#120370:u8
        let s_114_0: bool = fn_state.gs_120370;
        // N s_114_1: branch s_114_0 b227 b115
        if s_114_0 {
            return block_227(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#120371 <= s_115_0
        fn_state.gs_120371 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#120371:u8
        let s_116_0: bool = fn_state.gs_120371;
        // N s_116_1: branch s_116_0 b226 b117
        if s_116_0 {
            return block_226(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #440u : u32
        let s_117_0: u32 = 440;
        // D s_117_1: read-reg s_117_0:u8
        let s_117_1: u8 = {
            let value = state.read_register::<u8>(s_117_0 as isize);
            tracer.read_register(s_117_0 as isize, value);
            value
        };
        // D s_117_2: call ELUsingAArch32(s_117_1)
        let s_117_2: bool = ELUsingAArch32(state, tracer, s_117_1);
        // D s_117_3: not s_117_2
        let s_117_3: bool = !s_117_2;
        // N s_117_4: branch s_117_3 b225 b118
        if s_117_3 {
            return block_225(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #0u : u8
        let s_118_0: bool = false;
        // D s_118_1: write-var gs#120372 <= s_118_0
        fn_state.gs_120372 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#120372:u8
        let s_119_0: bool = fn_state.gs_120372;
        // N s_119_1: branch s_119_0 b216 b120
        if s_119_0 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #440u : u32
        let s_120_0: u32 = 440;
        // D s_120_1: read-reg s_120_0:u8
        let s_120_1: u8 = {
            let value = state.read_register::<u8>(s_120_0 as isize);
            tracer.read_register(s_120_0 as isize, value);
            value
        };
        // D s_120_2: call ELUsingAArch32(s_120_1)
        let s_120_2: bool = ELUsingAArch32(state, tracer, s_120_1);
        // N s_120_3: branch s_120_2 b215 b121
        if s_120_2 {
            return block_215(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#120373 <= s_121_0
        fn_state.gs_120373 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#120373:u8
        let s_122_0: bool = fn_state.gs_120373;
        // N s_122_1: branch s_122_0 b198 b123
        if s_122_0 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #() : ()
        let s_123_0: () = ();
        // S s_123_1: call EL2Enabled(s_123_0)
        let s_123_1: bool = EL2Enabled(state, tracer, s_123_0);
        // N s_123_2: branch s_123_1 b197 b124
        if s_123_1 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #0u : u8
        let s_124_0: bool = false;
        // D s_124_1: write-var gs#120374 <= s_124_0
        fn_state.gs_120374 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#120374:u8
        let s_125_0: bool = fn_state.gs_120374;
        // N s_125_1: branch s_125_0 b196 b126
        if s_125_0 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #0u : u8
        let s_126_0: bool = false;
        // D s_126_1: write-var gs#120375 <= s_126_0
        fn_state.gs_120375 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#120375:u8
        let s_127_0: bool = fn_state.gs_120375;
        // N s_127_1: branch s_127_0 b195 b128
        if s_127_0 {
            return block_195(state, tracer, fn_state);
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
        // D s_128_1: write-var gs#120378 <= s_128_0
        fn_state.gs_120378 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#120378:u8
        let s_129_0: bool = fn_state.gs_120378;
        // N s_129_1: branch s_129_0 b194 b130
        if s_129_0 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #0u : u8
        let s_130_0: bool = false;
        // D s_130_1: write-var gs#120379 <= s_130_0
        fn_state.gs_120379 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#120379:u8
        let s_131_0: bool = fn_state.gs_120379;
        // N s_131_1: branch s_131_0 b193 b132
        if s_131_0 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #() : ()
        let s_132_0: () = ();
        // S s_132_1: call EL2Enabled(s_132_0)
        let s_132_1: bool = EL2Enabled(state, tracer, s_132_0);
        // N s_132_2: branch s_132_1 b192 b133
        if s_132_1 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #0u : u8
        let s_133_0: bool = false;
        // D s_133_1: write-var gs#120380 <= s_133_0
        fn_state.gs_120380 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#120380:u8
        let s_134_0: bool = fn_state.gs_120380;
        // N s_134_1: branch s_134_0 b191 b135
        if s_134_0 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #0u : u8
        let s_135_0: bool = false;
        // D s_135_1: write-var gs#120383 <= s_135_0
        fn_state.gs_120383 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#120383:u8
        let s_136_0: bool = fn_state.gs_120383;
        // N s_136_1: branch s_136_0 b190 b137
        if s_136_0 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #0u : u8
        let s_137_0: bool = false;
        // D s_137_1: write-var gs#120384 <= s_137_0
        fn_state.gs_120384 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#120384:u8
        let s_138_0: bool = fn_state.gs_120384;
        // N s_138_1: branch s_138_0 b189 b139
        if s_138_0 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #() : ()
        let s_139_0: () = ();
        // S s_139_1: call EL2Enabled(s_139_0)
        let s_139_1: bool = EL2Enabled(state, tracer, s_139_0);
        // N s_139_2: branch s_139_1 b188 b140
        if s_139_1 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #0u : u8
        let s_140_0: bool = false;
        // D s_140_1: write-var gs#120385 <= s_140_0
        fn_state.gs_120385 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#120385:u8
        let s_141_0: bool = fn_state.gs_120385;
        // N s_141_1: branch s_141_0 b187 b142
        if s_141_0 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #0u : u8
        let s_142_0: bool = false;
        // D s_142_1: write-var gs#120386 <= s_142_0
        fn_state.gs_120386 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#120386:u8
        let s_143_0: bool = fn_state.gs_120386;
        // N s_143_1: branch s_143_0 b186 b144
        if s_143_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #() : ()
        let s_144_0: () = ();
        // S s_144_1: call EL2Enabled(s_144_0)
        let s_144_1: bool = EL2Enabled(state, tracer, s_144_0);
        // N s_144_2: branch s_144_1 b185 b145
        if s_144_1 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #0u : u8
        let s_145_0: bool = false;
        // D s_145_1: write-var gs#120387 <= s_145_0
        fn_state.gs_120387 = s_145_0;
        // N s_145_2: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var gs#120387:u8
        let s_146_0: bool = fn_state.gs_120387;
        // N s_146_1: branch s_146_0 b184 b147
        if s_146_0 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #0u : u8
        let s_147_0: bool = false;
        // D s_147_1: write-var gs#120388 <= s_147_0
        fn_state.gs_120388 = s_147_0;
        // N s_147_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var gs#120388:u8
        let s_148_0: bool = fn_state.gs_120388;
        // N s_148_1: branch s_148_0 b183 b149
        if s_148_0 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_149(state, tracer, fn_state);
        };
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #() : ()
        let s_149_0: () = ();
        // S s_149_1: call EL2Enabled(s_149_0)
        let s_149_1: bool = EL2Enabled(state, tracer, s_149_0);
        // N s_149_2: branch s_149_1 b182 b150
        if s_149_1 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #0u : u8
        let s_150_0: bool = false;
        // D s_150_1: write-var gs#120389 <= s_150_0
        fn_state.gs_120389 = s_150_0;
        // N s_150_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var gs#120389:u8
        let s_151_0: bool = fn_state.gs_120389;
        // N s_151_1: branch s_151_0 b181 b152
        if s_151_0 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #0u : u8
        let s_152_0: bool = false;
        // D s_152_1: write-var gs#120390 <= s_152_0
        fn_state.gs_120390 = s_152_0;
        // N s_152_2: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var gs#120390:u8
        let s_153_0: bool = fn_state.gs_120390;
        // N s_153_1: branch s_153_0 b180 b154
        if s_153_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #0u : u8
        let s_154_0: bool = false;
        // D s_154_1: write-var gs#120391 <= s_154_0
        fn_state.gs_120391 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var gs#120391:u8
        let s_155_0: bool = fn_state.gs_120391;
        // N s_155_1: branch s_155_0 b176 b156
        if s_155_0 {
            return block_176(state, tracer, fn_state);
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
        // D s_156_1: write-var gs#120393 <= s_156_0
        fn_state.gs_120393 = s_156_0;
        // N s_156_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var gs#120393:u8
        let s_157_0: bool = fn_state.gs_120393;
        // N s_157_1: branch s_157_0 b175 b158
        if s_157_0 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #0u : u8
        let s_158_0: bool = false;
        // D s_158_1: write-var gs#120394 <= s_158_0
        fn_state.gs_120394 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#120394:u8
        let s_159_0: bool = fn_state.gs_120394;
        // N s_159_1: branch s_159_0 b174 b160
        if s_159_0 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #424u : u32
        let s_160_0: u32 = 424;
        // D s_160_1: read-reg s_160_0:u8
        let s_160_1: u8 = {
            let value = state.read_register::<u8>(s_160_0 as isize);
            tracer.read_register(s_160_0 as isize, value);
            value
        };
        // C s_160_2: const #2u : u8
        let s_160_2: u8 = 2;
        // D s_160_3: cmp-lt s_160_1 s_160_2
        let s_160_3: bool = ((s_160_1) < (s_160_2));
        // N s_160_4: branch s_160_3 b173 b161
        if s_160_3 {
            return block_173(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #0u : u8
        let s_161_0: bool = false;
        // D s_161_1: write-var gs#120395 <= s_161_0
        fn_state.gs_120395 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#120395:u8
        let s_162_0: bool = fn_state.gs_120395;
        // N s_162_1: branch s_162_0 b172 b163
        if s_162_0 {
            return block_172(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #0u : u8
        let s_163_0: bool = false;
        // D s_163_1: write-var gs#120396 <= s_163_0
        fn_state.gs_120396 = s_163_0;
        // N s_163_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var gs#120396:u8
        let s_164_0: bool = fn_state.gs_120396;
        // N s_164_1: branch s_164_0 b166 b165
        if s_164_0 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #2s : i
        let s_165_0: i128 = 2;
        // C s_165_1: const #14624u : u32
        let s_165_1: u32 = 14624;
        // D s_165_2: read-reg s_165_1:[struct; 4]
        let s_165_2: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_165_1 as isize);
            tracer.read_register(s_165_1 as isize, value);
            value
        };
        // D s_165_3: read-element s_165_2[s_165_0]
        let s_165_3: ProductType5c790c8ef59cc8b2 = s_165_2[(s_165_0) as usize];
        // D s_165_4: write-var ga#200387 <= s_165_3
        fn_state.ga_200387 = s_165_3;
        // D s_165_5: read-var ga#200387.0:struct
        let s_165_5: u64 = fn_state.ga_200387._0;
        // C s_165_6: const #32s : i
        let s_165_6: i128 = 32;
        // D s_165_7: cast zx s_165_5 -> bv
        let s_165_7: Bits = Bits::new(s_165_5 as u128, 64u16);
        // C s_165_8: const #1s : i64
        let s_165_8: i64 = 1;
        // C s_165_9: cast zx s_165_8 -> i
        let s_165_9: i128 = (i128::try_from(s_165_8).unwrap());
        // C s_165_10: const #31s : i
        let s_165_10: i128 = 31;
        // C s_165_11: add s_165_10 s_165_9
        let s_165_11: i128 = (s_165_10 + s_165_9);
        // D s_165_12: bit-extract s_165_7 s_165_6 s_165_11
        let s_165_12: Bits = (Bits::new(
            ((s_165_7) >> (s_165_6)).value(),
            u16::try_from(s_165_11).unwrap(),
        ));
        // D s_165_13: cast reint s_165_12 -> u32
        let s_165_13: u32 = (s_165_12.value() as u32);
        // C s_165_14: const #2s : i
        let s_165_14: i128 = 2;
        // C s_165_15: const #14624u : u32
        let s_165_15: u32 = 14624;
        // D s_165_16: read-reg s_165_15:[struct; 4]
        let s_165_16: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_165_15 as isize);
            tracer.read_register(s_165_15 as isize, value);
            value
        };
        // D s_165_17: read-element s_165_16[s_165_14]
        let s_165_17: ProductType5c790c8ef59cc8b2 = s_165_16[(s_165_14) as usize];
        // D s_165_18: write-var ga#200389 <= s_165_17
        fn_state.ga_200389 = s_165_17;
        // D s_165_19: read-var ga#200389.0:struct
        let s_165_19: u64 = fn_state.ga_200389._0;
        // C s_165_20: const #0s : i
        let s_165_20: i128 = 0;
        // D s_165_21: cast zx s_165_19 -> bv
        let s_165_21: Bits = Bits::new(s_165_19 as u128, 64u16);
        // C s_165_22: const #1s : i64
        let s_165_22: i64 = 1;
        // C s_165_23: cast zx s_165_22 -> i
        let s_165_23: i128 = (i128::try_from(s_165_22).unwrap());
        // C s_165_24: const #31s : i
        let s_165_24: i128 = 31;
        // C s_165_25: add s_165_24 s_165_23
        let s_165_25: i128 = (s_165_24 + s_165_23);
        // D s_165_26: bit-extract s_165_21 s_165_20 s_165_25
        let s_165_26: Bits = (Bits::new(
            ((s_165_21) >> (s_165_20)).value(),
            u16::try_from(s_165_25).unwrap(),
        ));
        // D s_165_27: cast reint s_165_26 -> u32
        let s_165_27: u32 = (s_165_26.value() as u32);
        // D s_165_28: create-product struct = ["s_165_13", "s_165_27"]
        let s_165_28: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_165_13,
            _1: s_165_27,
        };
        // D s_165_29: write-var ga#200393 <= s_165_28
        fn_state.ga_200393 = s_165_28;
        // D s_165_30: read-var ga#200393.0:struct
        let s_165_30: u32 = fn_state.ga_200393._0;
        // D s_165_31: read-var ga#200393.1:struct
        let s_165_31: u32 = fn_state.ga_200393._1;
        // D s_165_32: read-var t2:i
        let s_165_32: i128 = fn_state.t2;
        // D s_165_33: call R_set(s_165_32, s_165_30)
        let s_165_33: () = R_set(state, tracer, s_165_32, s_165_30);
        // D s_165_34: read-var t:i
        let s_165_34: i128 = fn_state.t;
        // D s_165_35: call R_set(s_165_34, s_165_31)
        let s_165_35: () = R_set(state, tracer, s_165_34, s_165_31);
        // N s_165_36: return
        return;
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #() : ()
        let s_166_0: () = ();
        // S s_166_1: call Halted(s_166_0)
        let s_166_1: bool = Halted(state, tracer, s_166_0);
        // N s_166_2: branch s_166_1 b171 b167
        if s_166_1 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #0u : u8
        let s_167_0: bool = false;
        // D s_167_1: write-var gs#120407 <= s_167_0
        fn_state.gs_120407 = s_167_0;
        // N s_167_2: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var gs#120407:u8
        let s_168_0: bool = fn_state.gs_120407;
        // N s_168_1: branch s_168_0 b170 b169
        if s_168_0 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #4u : u8
        let s_169_0: u8 = 4;
        // C s_169_1: cast zx s_169_0 -> bv
        let s_169_1: Bits = Bits::new(s_169_0 as u128, 8u16);
        // C s_169_2: cast zx s_169_1 -> i
        let s_169_2: i128 = (s_169_1.value() as i128);
        // C s_169_3: cast reint s_169_2 -> i64
        let s_169_3: i64 = (s_169_2 as i64);
        // C s_169_4: cast zx s_169_3 -> i
        let s_169_4: i128 = (i128::try_from(s_169_3).unwrap());
        // C s_169_5: const #424u : u32
        let s_169_5: u32 = 424;
        // D s_169_6: read-reg s_169_5:u8
        let s_169_6: u8 = {
            let value = state.read_register::<u8>(s_169_5 as isize);
            tracer.read_register(s_169_5 as isize, value);
            value
        };
        // D s_169_7: call AArch64_AArch32SystemAccessTrap(s_169_6, s_169_4)
        let s_169_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_169_6,
            s_169_4,
        );
        // N s_169_8: return
        return;
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_170_0: panic
        panic!("{:?}", ());
        // N s_170_1: return
        return;
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #() : ()
        let s_171_0: () = ();
        // S s_171_1: call EDSCR_read(s_171_0)
        let s_171_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_171_0);
        // S s_171_2: call _get_EDSCR_Type_SDD(s_171_1)
        let s_171_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_171_1);
        // S s_171_3: cast zx s_171_2 -> bv
        let s_171_3: Bits = Bits::new(s_171_2 as u128, 1u16);
        // C s_171_4: const #1u : u8
        let s_171_4: bool = true;
        // C s_171_5: cast zx s_171_4 -> bv
        let s_171_5: Bits = Bits::new(s_171_4 as u128, 1u16);
        // S s_171_6: cmp-eq s_171_3 s_171_5
        let s_171_6: bool = ((s_171_3) == (s_171_5));
        // D s_171_7: write-var gs#120407 <= s_171_6
        fn_state.gs_120407 = s_171_6;
        // N s_171_8: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var __CPTR_EL3_TAM:u8
        let s_172_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_172_1: cast zx s_172_0 -> bv
        let s_172_1: Bits = Bits::new(s_172_0 as u128, 1u16);
        // C s_172_2: const #1u : u8
        let s_172_2: bool = true;
        // C s_172_3: cast zx s_172_2 -> bv
        let s_172_3: Bits = Bits::new(s_172_2 as u128, 1u16);
        // D s_172_4: cmp-eq s_172_1 s_172_3
        let s_172_4: bool = ((s_172_1) == (s_172_3));
        // D s_172_5: write-var gs#120396 <= s_172_4
        fn_state.gs_120396 = s_172_4;
        // N s_172_6: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #424u : u32
        let s_173_0: u32 = 424;
        // D s_173_1: read-reg s_173_0:u8
        let s_173_1: u8 = {
            let value = state.read_register::<u8>(s_173_0 as isize);
            tracer.read_register(s_173_0 as isize, value);
            value
        };
        // D s_173_2: call ELUsingAArch32(s_173_1)
        let s_173_2: bool = ELUsingAArch32(state, tracer, s_173_1);
        // D s_173_3: not s_173_2
        let s_173_3: bool = !s_173_2;
        // D s_173_4: write-var gs#120395 <= s_173_3
        fn_state.gs_120395 = s_173_3;
        // N s_173_5: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #4u : u8
        let s_174_0: u8 = 4;
        // C s_174_1: cast zx s_174_0 -> bv
        let s_174_1: Bits = Bits::new(s_174_0 as u128, 8u16);
        // C s_174_2: cast zx s_174_1 -> i
        let s_174_2: i128 = (s_174_1.value() as i128);
        // C s_174_3: cast reint s_174_2 -> i64
        let s_174_3: i64 = (s_174_2 as i64);
        // C s_174_4: cast zx s_174_3 -> i
        let s_174_4: i128 = (i128::try_from(s_174_3).unwrap());
        // C s_174_5: const #432u : u32
        let s_174_5: u32 = 432;
        // D s_174_6: read-reg s_174_5:u8
        let s_174_6: u8 = {
            let value = state.read_register::<u8>(s_174_5 as isize);
            tracer.read_register(s_174_5 as isize, value);
            value
        };
        // D s_174_7: call AArch64_AArch32SystemAccessTrap(s_174_6, s_174_4)
        let s_174_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_174_6,
            s_174_4,
        );
        // N s_174_8: return
        return;
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var __HAFGRTR_EL2_AMEVCNTR02_EL0:u8
        let s_175_0: bool = fn_state.u__HAFGRTR_EL2_AMEVCNTR02_EL0;
        // D s_175_1: cast zx s_175_0 -> bv
        let s_175_1: Bits = Bits::new(s_175_0 as u128, 1u16);
        // C s_175_2: const #1u : u8
        let s_175_2: bool = true;
        // C s_175_3: cast zx s_175_2 -> bv
        let s_175_3: Bits = Bits::new(s_175_2 as u128, 1u16);
        // D s_175_4: cmp-eq s_175_1 s_175_3
        let s_175_4: bool = ((s_175_1) == (s_175_3));
        // D s_175_5: write-var gs#120394 <= s_175_4
        fn_state.gs_120394 = s_175_4;
        // N s_175_6: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #424u : u32
        let s_176_0: u32 = 424;
        // D s_176_1: read-reg s_176_0:u8
        let s_176_1: u8 = {
            let value = state.read_register::<u8>(s_176_0 as isize);
            tracer.read_register(s_176_0 as isize, value);
            value
        };
        // C s_176_2: const #2u : u8
        let s_176_2: u8 = 2;
        // D s_176_3: cmp-lt s_176_1 s_176_2
        let s_176_3: bool = ((s_176_1) < (s_176_2));
        // D s_176_4: not s_176_3
        let s_176_4: bool = !s_176_3;
        // N s_176_5: branch s_176_4 b179 b177
        if s_176_4 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_177_0: read-var __SCR_EL3_FGTEn:u8
        let s_177_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_177_1: cast zx s_177_0 -> bv
        let s_177_1: Bits = Bits::new(s_177_0 as u128, 1u16);
        // C s_177_2: const #1u : u8
        let s_177_2: bool = true;
        // C s_177_3: cast zx s_177_2 -> bv
        let s_177_3: Bits = Bits::new(s_177_2 as u128, 1u16);
        // D s_177_4: cmp-eq s_177_1 s_177_3
        let s_177_4: bool = ((s_177_1) == (s_177_3));
        // D s_177_5: write-var gs#120392 <= s_177_4
        fn_state.gs_120392 = s_177_4;
        // N s_177_6: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var gs#120392:u8
        let s_178_0: bool = fn_state.gs_120392;
        // D s_178_1: write-var gs#120393 <= s_178_0
        fn_state.gs_120393 = s_178_0;
        // N s_178_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #1u : u8
        let s_179_0: bool = true;
        // D s_179_1: write-var gs#120392 <= s_179_0
        fn_state.gs_120392 = s_179_0;
        // N s_179_2: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #146u : u32
        let s_180_0: u32 = 146;
        // S s_180_1: call IsFeatureImplemented(s_180_0)
        let s_180_1: bool = IsFeatureImplemented(state, tracer, s_180_0);
        // D s_180_2: write-var gs#120391 <= s_180_1
        fn_state.gs_120391 = s_180_1;
        // N s_180_3: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #102552u : u32
        let s_181_0: u32 = 102552;
        // D s_181_1: read-reg s_181_0:struct
        let s_181_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_181_0 as isize);
            tracer.read_register(s_181_0 as isize, value);
            value
        };
        // D s_181_2: call _get_HCR_EL2_Type_E2H(s_181_1)
        let s_181_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_181_1);
        // C s_181_3: const #102552u : u32
        let s_181_3: u32 = 102552;
        // D s_181_4: read-reg s_181_3:struct
        let s_181_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_181_3 as isize);
            tracer.read_register(s_181_3 as isize, value);
            value
        };
        // D s_181_5: call _get_HCR_EL2_Type_TGE(s_181_4)
        let s_181_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_181_4);
        // D s_181_6: cast zx s_181_2 -> bv
        let s_181_6: Bits = Bits::new(s_181_2 as u128, 1u16);
        // D s_181_7: cast zx s_181_5 -> bv
        let s_181_7: Bits = Bits::new(s_181_5 as u128, 1u16);
        // D s_181_8: cast reint s_181_6 -> u128
        let s_181_8: u128 = (s_181_6.value() as u128);
        // D s_181_9: size-of s_181_6
        let s_181_9: u16 = s_181_6.length();
        // D s_181_10: cast reint s_181_7 -> u128
        let s_181_10: u128 = (s_181_7.value() as u128);
        // D s_181_11: size-of s_181_7
        let s_181_11: u16 = s_181_7.length();
        // D s_181_12: lsl s_181_8 s_181_11
        let s_181_12: u128 = s_181_8 << s_181_11;
        // D s_181_13: or s_181_12 s_181_10
        let s_181_13: u128 = ((s_181_12) | (s_181_10));
        // D s_181_14: add s_181_9 s_181_11
        let s_181_14: u16 = (s_181_9 + s_181_11);
        // D s_181_15: create-bits s_181_13 s_181_14
        let s_181_15: Bits = Bits::new(s_181_13, s_181_14);
        // D s_181_16: cast reint s_181_15 -> u8
        let s_181_16: u8 = (s_181_15.value() as u8);
        // D s_181_17: cast zx s_181_16 -> bv
        let s_181_17: Bits = Bits::new(s_181_16 as u128, 2u16);
        // C s_181_18: const #3u : u8
        let s_181_18: u8 = 3;
        // C s_181_19: cast zx s_181_18 -> bv
        let s_181_19: Bits = Bits::new(s_181_18 as u128, 2u16);
        // D s_181_20: cmp-ne s_181_17 s_181_19
        let s_181_20: bool = ((s_181_17) != (s_181_19));
        // D s_181_21: write-var gs#120390 <= s_181_20
        fn_state.gs_120390 = s_181_20;
        // N s_181_22: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #440u : u32
        let s_182_0: u32 = 440;
        // D s_182_1: read-reg s_182_0:u8
        let s_182_1: u8 = {
            let value = state.read_register::<u8>(s_182_0 as isize);
            tracer.read_register(s_182_0 as isize, value);
            value
        };
        // D s_182_2: call ELUsingAArch32(s_182_1)
        let s_182_2: bool = ELUsingAArch32(state, tracer, s_182_1);
        // D s_182_3: not s_182_2
        let s_182_3: bool = !s_182_2;
        // D s_182_4: write-var gs#120389 <= s_182_3
        fn_state.gs_120389 = s_182_3;
        // N s_182_5: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #4u : u8
        let s_183_0: u8 = 4;
        // C s_183_1: cast zx s_183_0 -> bv
        let s_183_1: Bits = Bits::new(s_183_0 as u128, 8u16);
        // C s_183_2: cast zx s_183_1 -> i
        let s_183_2: i128 = (s_183_1.value() as i128);
        // C s_183_3: cast reint s_183_2 -> i64
        let s_183_3: i64 = (s_183_2 as i64);
        // C s_183_4: cast zx s_183_3 -> i
        let s_183_4: i128 = (i128::try_from(s_183_3).unwrap());
        // S s_183_5: call AArch32_TakeHypTrapException(s_183_4)
        let s_183_5: () = AArch32_TakeHypTrapException(state, tracer, s_183_4);
        // N s_183_6: return
        return;
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var __HCPTR_TAM:u8
        let s_184_0: bool = fn_state.u__HCPTR_TAM;
        // D s_184_1: cast zx s_184_0 -> bv
        let s_184_1: Bits = Bits::new(s_184_0 as u128, 1u16);
        // C s_184_2: const #1u : u8
        let s_184_2: bool = true;
        // C s_184_3: cast zx s_184_2 -> bv
        let s_184_3: Bits = Bits::new(s_184_2 as u128, 1u16);
        // D s_184_4: cmp-eq s_184_1 s_184_3
        let s_184_4: bool = ((s_184_1) == (s_184_3));
        // D s_184_5: write-var gs#120388 <= s_184_4
        fn_state.gs_120388 = s_184_4;
        // N s_184_6: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #432u : u32
        let s_185_0: u32 = 432;
        // D s_185_1: read-reg s_185_0:u8
        let s_185_1: u8 = {
            let value = state.read_register::<u8>(s_185_0 as isize);
            tracer.read_register(s_185_0 as isize, value);
            value
        };
        // D s_185_2: call ELUsingAArch32(s_185_1)
        let s_185_2: bool = ELUsingAArch32(state, tracer, s_185_1);
        // D s_185_3: write-var gs#120387 <= s_185_2
        fn_state.gs_120387 = s_185_2;
        // N s_185_4: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #4u : u8
        let s_186_0: u8 = 4;
        // C s_186_1: cast zx s_186_0 -> bv
        let s_186_1: Bits = Bits::new(s_186_0 as u128, 8u16);
        // C s_186_2: cast zx s_186_1 -> i
        let s_186_2: i128 = (s_186_1.value() as i128);
        // C s_186_3: cast reint s_186_2 -> i64
        let s_186_3: i64 = (s_186_2 as i64);
        // C s_186_4: cast zx s_186_3 -> i
        let s_186_4: i128 = (i128::try_from(s_186_3).unwrap());
        // C s_186_5: const #432u : u32
        let s_186_5: u32 = 432;
        // D s_186_6: read-reg s_186_5:u8
        let s_186_6: u8 = {
            let value = state.read_register::<u8>(s_186_5 as isize);
            tracer.read_register(s_186_5 as isize, value);
            value
        };
        // D s_186_7: call AArch64_AArch32SystemAccessTrap(s_186_6, s_186_4)
        let s_186_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_186_6,
            s_186_4,
        );
        // N s_186_8: return
        return;
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_187_0: read-var __CPTR_EL2_TAM:u8
        let s_187_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_187_1: cast zx s_187_0 -> bv
        let s_187_1: Bits = Bits::new(s_187_0 as u128, 1u16);
        // C s_187_2: const #1u : u8
        let s_187_2: bool = true;
        // C s_187_3: cast zx s_187_2 -> bv
        let s_187_3: Bits = Bits::new(s_187_2 as u128, 1u16);
        // D s_187_4: cmp-eq s_187_1 s_187_3
        let s_187_4: bool = ((s_187_1) == (s_187_3));
        // D s_187_5: write-var gs#120386 <= s_187_4
        fn_state.gs_120386 = s_187_4;
        // N s_187_6: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #432u : u32
        let s_188_0: u32 = 432;
        // D s_188_1: read-reg s_188_0:u8
        let s_188_1: u8 = {
            let value = state.read_register::<u8>(s_188_0 as isize);
            tracer.read_register(s_188_0 as isize, value);
            value
        };
        // D s_188_2: call ELUsingAArch32(s_188_1)
        let s_188_2: bool = ELUsingAArch32(state, tracer, s_188_1);
        // D s_188_3: not s_188_2
        let s_188_3: bool = !s_188_2;
        // D s_188_4: write-var gs#120385 <= s_188_3
        fn_state.gs_120385 = s_188_3;
        // N s_188_5: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #4u : u8
        let s_189_0: u8 = 4;
        // C s_189_1: cast zx s_189_0 -> bv
        let s_189_1: Bits = Bits::new(s_189_0 as u128, 8u16);
        // C s_189_2: cast zx s_189_1 -> i
        let s_189_2: i128 = (s_189_1.value() as i128);
        // C s_189_3: cast reint s_189_2 -> i64
        let s_189_3: i64 = (s_189_2 as i64);
        // C s_189_4: cast zx s_189_3 -> i
        let s_189_4: i128 = (i128::try_from(s_189_3).unwrap());
        // S s_189_5: call AArch32_TakeHypTrapException(s_189_4)
        let s_189_5: () = AArch32_TakeHypTrapException(state, tracer, s_189_4);
        // N s_189_6: return
        return;
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var __HSTR_T0:u8
        let s_190_0: bool = fn_state.u__HSTR_T0;
        // D s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 1u16);
        // C s_190_2: const #1u : u8
        let s_190_2: bool = true;
        // C s_190_3: cast zx s_190_2 -> bv
        let s_190_3: Bits = Bits::new(s_190_2 as u128, 1u16);
        // D s_190_4: cmp-eq s_190_1 s_190_3
        let s_190_4: bool = ((s_190_1) == (s_190_3));
        // D s_190_5: write-var gs#120384 <= s_190_4
        fn_state.gs_120384 = s_190_4;
        // N s_190_6: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #1u : u8
        let s_191_0: bool = true;
        // D s_191_1: write-var gs#120383 <= s_191_0
        fn_state.gs_120383 = s_191_0;
        // N s_191_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #432u : u32
        let s_192_0: u32 = 432;
        // D s_192_1: read-reg s_192_0:u8
        let s_192_1: u8 = {
            let value = state.read_register::<u8>(s_192_0 as isize);
            tracer.read_register(s_192_0 as isize, value);
            value
        };
        // D s_192_2: call ELUsingAArch32(s_192_1)
        let s_192_2: bool = ELUsingAArch32(state, tracer, s_192_1);
        // D s_192_3: write-var gs#120380 <= s_192_2
        fn_state.gs_120380 = s_192_2;
        // N s_192_4: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #4u : u8
        let s_193_0: u8 = 4;
        // C s_193_1: cast zx s_193_0 -> bv
        let s_193_1: Bits = Bits::new(s_193_0 as u128, 8u16);
        // C s_193_2: cast zx s_193_1 -> i
        let s_193_2: i128 = (s_193_1.value() as i128);
        // C s_193_3: cast reint s_193_2 -> i64
        let s_193_3: i64 = (s_193_2 as i64);
        // C s_193_4: cast zx s_193_3 -> i
        let s_193_4: i128 = (i128::try_from(s_193_3).unwrap());
        // C s_193_5: const #432u : u32
        let s_193_5: u32 = 432;
        // D s_193_6: read-reg s_193_5:u8
        let s_193_6: u8 = {
            let value = state.read_register::<u8>(s_193_5 as isize);
            tracer.read_register(s_193_5 as isize, value);
            value
        };
        // D s_193_7: call AArch64_AArch32SystemAccessTrap(s_193_6, s_193_4)
        let s_193_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_193_6,
            s_193_4,
        );
        // N s_193_8: return
        return;
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var __HSTR_EL2_T0:u8
        let s_194_0: bool = fn_state.u__HSTR_EL2_T0;
        // D s_194_1: cast zx s_194_0 -> bv
        let s_194_1: Bits = Bits::new(s_194_0 as u128, 1u16);
        // C s_194_2: const #1u : u8
        let s_194_2: bool = true;
        // C s_194_3: cast zx s_194_2 -> bv
        let s_194_3: Bits = Bits::new(s_194_2 as u128, 1u16);
        // D s_194_4: cmp-eq s_194_1 s_194_3
        let s_194_4: bool = ((s_194_1) == (s_194_3));
        // D s_194_5: write-var gs#120379 <= s_194_4
        fn_state.gs_120379 = s_194_4;
        // N s_194_6: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #1u : u8
        let s_195_0: bool = true;
        // D s_195_1: write-var gs#120378 <= s_195_0
        fn_state.gs_120378 = s_195_0;
        // N s_195_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #102552u : u32
        let s_196_0: u32 = 102552;
        // D s_196_1: read-reg s_196_0:struct
        let s_196_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_196_0 as isize);
            tracer.read_register(s_196_0 as isize, value);
            value
        };
        // D s_196_2: call _get_HCR_EL2_Type_E2H(s_196_1)
        let s_196_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_196_1);
        // C s_196_3: const #102552u : u32
        let s_196_3: u32 = 102552;
        // D s_196_4: read-reg s_196_3:struct
        let s_196_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_196_3 as isize);
            tracer.read_register(s_196_3 as isize, value);
            value
        };
        // D s_196_5: call _get_HCR_EL2_Type_TGE(s_196_4)
        let s_196_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_196_4);
        // D s_196_6: cast zx s_196_2 -> bv
        let s_196_6: Bits = Bits::new(s_196_2 as u128, 1u16);
        // D s_196_7: cast zx s_196_5 -> bv
        let s_196_7: Bits = Bits::new(s_196_5 as u128, 1u16);
        // D s_196_8: cast reint s_196_6 -> u128
        let s_196_8: u128 = (s_196_6.value() as u128);
        // D s_196_9: size-of s_196_6
        let s_196_9: u16 = s_196_6.length();
        // D s_196_10: cast reint s_196_7 -> u128
        let s_196_10: u128 = (s_196_7.value() as u128);
        // D s_196_11: size-of s_196_7
        let s_196_11: u16 = s_196_7.length();
        // D s_196_12: lsl s_196_8 s_196_11
        let s_196_12: u128 = s_196_8 << s_196_11;
        // D s_196_13: or s_196_12 s_196_10
        let s_196_13: u128 = ((s_196_12) | (s_196_10));
        // D s_196_14: add s_196_9 s_196_11
        let s_196_14: u16 = (s_196_9 + s_196_11);
        // D s_196_15: create-bits s_196_13 s_196_14
        let s_196_15: Bits = Bits::new(s_196_13, s_196_14);
        // D s_196_16: cast reint s_196_15 -> u8
        let s_196_16: u8 = (s_196_15.value() as u8);
        // D s_196_17: cast zx s_196_16 -> bv
        let s_196_17: Bits = Bits::new(s_196_16 as u128, 2u16);
        // C s_196_18: const #3u : u8
        let s_196_18: u8 = 3;
        // C s_196_19: cast zx s_196_18 -> bv
        let s_196_19: Bits = Bits::new(s_196_18 as u128, 2u16);
        // D s_196_20: cmp-ne s_196_17 s_196_19
        let s_196_20: bool = ((s_196_17) != (s_196_19));
        // D s_196_21: write-var gs#120375 <= s_196_20
        fn_state.gs_120375 = s_196_20;
        // N s_196_22: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #432u : u32
        let s_197_0: u32 = 432;
        // D s_197_1: read-reg s_197_0:u8
        let s_197_1: u8 = {
            let value = state.read_register::<u8>(s_197_0 as isize);
            tracer.read_register(s_197_0 as isize, value);
            value
        };
        // D s_197_2: call ELUsingAArch32(s_197_1)
        let s_197_2: bool = ELUsingAArch32(state, tracer, s_197_1);
        // D s_197_3: not s_197_2
        let s_197_3: bool = !s_197_2;
        // D s_197_4: write-var gs#120374 <= s_197_3
        fn_state.gs_120374 = s_197_3;
        // N s_197_5: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #() : ()
        let s_198_0: () = ();
        // S s_198_1: call EL2Enabled(s_198_0)
        let s_198_1: bool = EL2Enabled(state, tracer, s_198_0);
        // N s_198_2: branch s_198_1 b214 b199
        if s_198_1 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_199(state, tracer, fn_state);
        };
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #0u : u8
        let s_199_0: bool = false;
        // D s_199_1: write-var gs#120408 <= s_199_0
        fn_state.gs_120408 = s_199_0;
        // N s_199_2: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var gs#120408:u8
        let s_200_0: bool = fn_state.gs_120408;
        // N s_200_1: branch s_200_0 b213 b201
        if s_200_0 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_201(state, tracer, fn_state);
        };
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #0u : u8
        let s_201_0: bool = false;
        // D s_201_1: write-var gs#120409 <= s_201_0
        fn_state.gs_120409 = s_201_0;
        // N s_201_2: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var gs#120409:u8
        let s_202_0: bool = fn_state.gs_120409;
        // N s_202_1: branch s_202_0 b212 b203
        if s_202_0 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_203(state, tracer, fn_state);
        };
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #() : ()
        let s_203_0: () = ();
        // S s_203_1: call EL2Enabled(s_203_0)
        let s_203_1: bool = EL2Enabled(state, tracer, s_203_0);
        // N s_203_2: branch s_203_1 b211 b204
        if s_203_1 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_204(state, tracer, fn_state);
        };
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #0u : u8
        let s_204_0: bool = false;
        // D s_204_1: write-var gs#120410 <= s_204_0
        fn_state.gs_120410 = s_204_0;
        // N s_204_2: jump b205
        return block_205(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_205_0: read-var gs#120410:u8
        let s_205_0: bool = fn_state.gs_120410;
        // N s_205_1: branch s_205_0 b210 b206
        if s_205_0 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_206(state, tracer, fn_state);
        };
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_206_0: const #0u : u8
        let s_206_0: bool = false;
        // D s_206_1: write-var gs#120411 <= s_206_0
        fn_state.gs_120411 = s_206_0;
        // N s_206_2: jump b207
        return block_207(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_207_0: read-var gs#120411:u8
        let s_207_0: bool = fn_state.gs_120411;
        // N s_207_1: branch s_207_0 b209 b208
        if s_207_0 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_208(state, tracer, fn_state);
        };
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_208_0: panic
        panic!("{:?}", ());
        // N s_208_1: return
        return;
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #0u : u8
        let s_209_0: u8 = 0;
        // C s_209_1: cast zx s_209_0 -> bv
        let s_209_1: Bits = Bits::new(s_209_0 as u128, 8u16);
        // C s_209_2: cast zx s_209_1 -> i
        let s_209_2: i128 = (s_209_1.value() as i128);
        // C s_209_3: cast reint s_209_2 -> i64
        let s_209_3: i64 = (s_209_2 as i64);
        // C s_209_4: cast zx s_209_3 -> i
        let s_209_4: i128 = (i128::try_from(s_209_3).unwrap());
        // S s_209_5: call AArch32_TakeHypTrapException(s_209_4)
        let s_209_5: () = AArch32_TakeHypTrapException(state, tracer, s_209_4);
        // N s_209_6: return
        return;
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_210_0: read-var __HCR_TGE:u8
        let s_210_0: bool = fn_state.u__HCR_TGE;
        // D s_210_1: cast zx s_210_0 -> bv
        let s_210_1: Bits = Bits::new(s_210_0 as u128, 1u16);
        // C s_210_2: const #1u : u8
        let s_210_2: bool = true;
        // C s_210_3: cast zx s_210_2 -> bv
        let s_210_3: Bits = Bits::new(s_210_2 as u128, 1u16);
        // D s_210_4: cmp-eq s_210_1 s_210_3
        let s_210_4: bool = ((s_210_1) == (s_210_3));
        // D s_210_5: write-var gs#120411 <= s_210_4
        fn_state.gs_120411 = s_210_4;
        // N s_210_6: jump b207
        return block_207(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #432u : u32
        let s_211_0: u32 = 432;
        // D s_211_1: read-reg s_211_0:u8
        let s_211_1: u8 = {
            let value = state.read_register::<u8>(s_211_0 as isize);
            tracer.read_register(s_211_0 as isize, value);
            value
        };
        // D s_211_2: call ELUsingAArch32(s_211_1)
        let s_211_2: bool = ELUsingAArch32(state, tracer, s_211_1);
        // D s_211_3: write-var gs#120410 <= s_211_2
        fn_state.gs_120410 = s_211_2;
        // N s_211_4: jump b205
        return block_205(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_212_0: const #4u : u8
        let s_212_0: u8 = 4;
        // C s_212_1: cast zx s_212_0 -> bv
        let s_212_1: Bits = Bits::new(s_212_0 as u128, 8u16);
        // C s_212_2: cast zx s_212_1 -> i
        let s_212_2: i128 = (s_212_1.value() as i128);
        // C s_212_3: cast reint s_212_2 -> i64
        let s_212_3: i64 = (s_212_2 as i64);
        // C s_212_4: cast zx s_212_3 -> i
        let s_212_4: i128 = (i128::try_from(s_212_3).unwrap());
        // C s_212_5: const #432u : u32
        let s_212_5: u32 = 432;
        // D s_212_6: read-reg s_212_5:u8
        let s_212_6: u8 = {
            let value = state.read_register::<u8>(s_212_5 as isize);
            tracer.read_register(s_212_5 as isize, value);
            value
        };
        // D s_212_7: call AArch64_AArch32SystemAccessTrap(s_212_6, s_212_4)
        let s_212_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_212_6,
            s_212_4,
        );
        // N s_212_8: return
        return;
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_213_0: read-var __HCR_EL2_TGE:u8
        let s_213_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_213_1: cast zx s_213_0 -> bv
        let s_213_1: Bits = Bits::new(s_213_0 as u128, 1u16);
        // C s_213_2: const #1u : u8
        let s_213_2: bool = true;
        // C s_213_3: cast zx s_213_2 -> bv
        let s_213_3: Bits = Bits::new(s_213_2 as u128, 1u16);
        // D s_213_4: cmp-eq s_213_1 s_213_3
        let s_213_4: bool = ((s_213_1) == (s_213_3));
        // D s_213_5: write-var gs#120409 <= s_213_4
        fn_state.gs_120409 = s_213_4;
        // N s_213_6: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #432u : u32
        let s_214_0: u32 = 432;
        // D s_214_1: read-reg s_214_0:u8
        let s_214_1: u8 = {
            let value = state.read_register::<u8>(s_214_0 as isize);
            tracer.read_register(s_214_0 as isize, value);
            value
        };
        // D s_214_2: call ELUsingAArch32(s_214_1)
        let s_214_2: bool = ELUsingAArch32(state, tracer, s_214_1);
        // D s_214_3: not s_214_2
        let s_214_3: bool = !s_214_2;
        // D s_214_4: write-var gs#120408 <= s_214_3
        fn_state.gs_120408 = s_214_3;
        // N s_214_5: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_215_0: read-var __AMUSERENR_EN:u8
        let s_215_0: bool = fn_state.u__AMUSERENR_EN;
        // D s_215_1: cast zx s_215_0 -> bv
        let s_215_1: Bits = Bits::new(s_215_0 as u128, 1u16);
        // C s_215_2: const #0u : u8
        let s_215_2: bool = false;
        // C s_215_3: cast zx s_215_2 -> bv
        let s_215_3: Bits = Bits::new(s_215_2 as u128, 1u16);
        // D s_215_4: cmp-eq s_215_1 s_215_3
        let s_215_4: bool = ((s_215_1) == (s_215_3));
        // D s_215_5: write-var gs#120373 <= s_215_4
        fn_state.gs_120373 = s_215_4;
        // N s_215_6: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #() : ()
        let s_216_0: () = ();
        // S s_216_1: call EL2Enabled(s_216_0)
        let s_216_1: bool = EL2Enabled(state, tracer, s_216_0);
        // N s_216_2: branch s_216_1 b224 b217
        if s_216_1 {
            return block_224(state, tracer, fn_state);
        } else {
            return block_217(state, tracer, fn_state);
        };
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #0u : u8
        let s_217_0: bool = false;
        // D s_217_1: write-var gs#120412 <= s_217_0
        fn_state.gs_120412 = s_217_0;
        // N s_217_2: jump b218
        return block_218(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_218_0: read-var gs#120412:u8
        let s_218_0: bool = fn_state.gs_120412;
        // N s_218_1: branch s_218_0 b223 b219
        if s_218_0 {
            return block_223(state, tracer, fn_state);
        } else {
            return block_219(state, tracer, fn_state);
        };
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #0u : u8
        let s_219_0: bool = false;
        // D s_219_1: write-var gs#120413 <= s_219_0
        fn_state.gs_120413 = s_219_0;
        // N s_219_2: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_220_0: read-var gs#120413:u8
        let s_220_0: bool = fn_state.gs_120413;
        // N s_220_1: branch s_220_0 b222 b221
        if s_220_0 {
            return block_222(state, tracer, fn_state);
        } else {
            return block_221(state, tracer, fn_state);
        };
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #4u : u8
        let s_221_0: u8 = 4;
        // C s_221_1: cast zx s_221_0 -> bv
        let s_221_1: Bits = Bits::new(s_221_0 as u128, 8u16);
        // C s_221_2: cast zx s_221_1 -> i
        let s_221_2: i128 = (s_221_1.value() as i128);
        // C s_221_3: cast reint s_221_2 -> i64
        let s_221_3: i64 = (s_221_2 as i64);
        // C s_221_4: cast zx s_221_3 -> i
        let s_221_4: i128 = (i128::try_from(s_221_3).unwrap());
        // C s_221_5: const #440u : u32
        let s_221_5: u32 = 440;
        // D s_221_6: read-reg s_221_5:u8
        let s_221_6: u8 = {
            let value = state.read_register::<u8>(s_221_5 as isize);
            tracer.read_register(s_221_5 as isize, value);
            value
        };
        // D s_221_7: call AArch64_AArch32SystemAccessTrap(s_221_6, s_221_4)
        let s_221_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_221_6,
            s_221_4,
        );
        // N s_221_8: return
        return;
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_222_0: const #4u : u8
        let s_222_0: u8 = 4;
        // C s_222_1: cast zx s_222_0 -> bv
        let s_222_1: Bits = Bits::new(s_222_0 as u128, 8u16);
        // C s_222_2: cast zx s_222_1 -> i
        let s_222_2: i128 = (s_222_1.value() as i128);
        // C s_222_3: cast reint s_222_2 -> i64
        let s_222_3: i64 = (s_222_2 as i64);
        // C s_222_4: cast zx s_222_3 -> i
        let s_222_4: i128 = (i128::try_from(s_222_3).unwrap());
        // C s_222_5: const #432u : u32
        let s_222_5: u32 = 432;
        // D s_222_6: read-reg s_222_5:u8
        let s_222_6: u8 = {
            let value = state.read_register::<u8>(s_222_5 as isize);
            tracer.read_register(s_222_5 as isize, value);
            value
        };
        // D s_222_7: call AArch64_AArch32SystemAccessTrap(s_222_6, s_222_4)
        let s_222_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_222_6,
            s_222_4,
        );
        // N s_222_8: return
        return;
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_223_0: read-var __HCR_EL2_TGE:u8
        let s_223_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_223_1: cast zx s_223_0 -> bv
        let s_223_1: Bits = Bits::new(s_223_0 as u128, 1u16);
        // C s_223_2: const #1u : u8
        let s_223_2: bool = true;
        // C s_223_3: cast zx s_223_2 -> bv
        let s_223_3: Bits = Bits::new(s_223_2 as u128, 1u16);
        // D s_223_4: cmp-eq s_223_1 s_223_3
        let s_223_4: bool = ((s_223_1) == (s_223_3));
        // D s_223_5: write-var gs#120413 <= s_223_4
        fn_state.gs_120413 = s_223_4;
        // N s_223_6: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_224_0: const #432u : u32
        let s_224_0: u32 = 432;
        // D s_224_1: read-reg s_224_0:u8
        let s_224_1: u8 = {
            let value = state.read_register::<u8>(s_224_0 as isize);
            tracer.read_register(s_224_0 as isize, value);
            value
        };
        // D s_224_2: call ELUsingAArch32(s_224_1)
        let s_224_2: bool = ELUsingAArch32(state, tracer, s_224_1);
        // D s_224_3: not s_224_2
        let s_224_3: bool = !s_224_2;
        // D s_224_4: write-var gs#120412 <= s_224_3
        fn_state.gs_120412 = s_224_3;
        // N s_224_5: jump b218
        return block_218(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_225_0: read-var __AMUSERENR_EL0_EN:u8
        let s_225_0: bool = fn_state.u__AMUSERENR_EL0_EN;
        // D s_225_1: cast zx s_225_0 -> bv
        let s_225_1: Bits = Bits::new(s_225_0 as u128, 1u16);
        // C s_225_2: const #0u : u8
        let s_225_2: bool = false;
        // C s_225_3: cast zx s_225_2 -> bv
        let s_225_3: Bits = Bits::new(s_225_2 as u128, 1u16);
        // D s_225_4: cmp-eq s_225_1 s_225_3
        let s_225_4: bool = ((s_225_1) == (s_225_3));
        // D s_225_5: write-var gs#120372 <= s_225_4
        fn_state.gs_120372 = s_225_4;
        // N s_225_6: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_226_0: panic
        panic!("{:?}", ());
        // N s_226_1: return
        return;
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_227_0: read-var __CPTR_EL3_TAM:u8
        let s_227_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_227_1: cast zx s_227_0 -> bv
        let s_227_1: Bits = Bits::new(s_227_0 as u128, 1u16);
        // C s_227_2: const #1u : u8
        let s_227_2: bool = true;
        // C s_227_3: cast zx s_227_2 -> bv
        let s_227_3: Bits = Bits::new(s_227_2 as u128, 1u16);
        // D s_227_4: cmp-eq s_227_1 s_227_3
        let s_227_4: bool = ((s_227_1) == (s_227_3));
        // D s_227_5: write-var gs#120371 <= s_227_4
        fn_state.gs_120371 = s_227_4;
        // N s_227_6: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_228_0: const #424u : u32
        let s_228_0: u32 = 424;
        // D s_228_1: read-reg s_228_0:u8
        let s_228_1: u8 = {
            let value = state.read_register::<u8>(s_228_0 as isize);
            tracer.read_register(s_228_0 as isize, value);
            value
        };
        // D s_228_2: call ELUsingAArch32(s_228_1)
        let s_228_2: bool = ELUsingAArch32(state, tracer, s_228_1);
        // D s_228_3: not s_228_2
        let s_228_3: bool = !s_228_2;
        // D s_228_4: write-var gs#120370 <= s_228_3
        fn_state.gs_120370 = s_228_3;
        // N s_228_5: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_229_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_229_1: call __IMPDEF_boolean(s_229_0)
        let s_229_1: bool = u__IMPDEF_boolean(state, tracer, s_229_0);
        // D s_229_2: write-var gs#120369 <= s_229_1
        fn_state.gs_120369 = s_229_1;
        // N s_229_3: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_230_0: const #() : ()
        let s_230_0: () = ();
        // S s_230_1: call EDSCR_read(s_230_0)
        let s_230_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_230_0);
        // S s_230_2: call _get_EDSCR_Type_SDD(s_230_1)
        let s_230_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_230_1);
        // S s_230_3: cast zx s_230_2 -> bv
        let s_230_3: Bits = Bits::new(s_230_2 as u128, 1u16);
        // C s_230_4: const #1u : u8
        let s_230_4: bool = true;
        // C s_230_5: cast zx s_230_4 -> bv
        let s_230_5: Bits = Bits::new(s_230_4 as u128, 1u16);
        // S s_230_6: cmp-eq s_230_3 s_230_5
        let s_230_6: bool = ((s_230_3) == (s_230_5));
        // D s_230_7: write-var gs#120368 <= s_230_6
        fn_state.gs_120368 = s_230_6;
        // N s_230_8: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_231_0: const #424u : u32
        let s_231_0: u32 = 424;
        // D s_231_1: read-reg s_231_0:u8
        let s_231_1: u8 = {
            let value = state.read_register::<u8>(s_231_0 as isize);
            tracer.read_register(s_231_0 as isize, value);
            value
        };
        // C s_231_2: const #2u : u8
        let s_231_2: u8 = 2;
        // D s_231_3: cmp-lt s_231_1 s_231_2
        let s_231_3: bool = ((s_231_1) < (s_231_2));
        // D s_231_4: write-var gs#120367 <= s_231_3
        fn_state.gs_120367 = s_231_3;
        // N s_231_5: jump b108
        return block_108(state, tracer, fn_state);
    }
}
