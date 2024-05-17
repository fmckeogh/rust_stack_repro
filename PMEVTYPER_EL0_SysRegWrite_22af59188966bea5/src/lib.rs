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
use u_get_PMUSERENR_EL0_Type_UEN::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HCR_EL2_Type_E2H::*;
use Halted::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use ConstrainUnpredictableProcedure::*;
use u_get_MDCR_EL3_Type_TPM::*;
use u_get_HDFGWTR_EL2_Type_PMEVTYPERn_EL0::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use AArch64_GetNumEventCountersAccessible::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use Mk_PMEVTYPER_EL0_Type::*;
use EDSCR_read::*;
use common::*;
pub fn PMEVTYPER_EL0_SysRegWrite_22af59188966bea5<T: Tracer>(
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
        gs_89234: bool,
        gs_89211: bool,
        gs_89233: bool,
        gs_89231: bool,
        gs_89235: bool,
        gs_89246: bool,
        gs_89228: bool,
        gs_89212: bool,
        u__MDCR_EL3_TPM: bool,
        gs_89217: bool,
        gs_89227: bool,
        gs_89218: bool,
        gs_89239: bool,
        gs_89247: bool,
        gs_89240: bool,
        gs_89216: bool,
        u__HDFGWTR_EL2_PMEVTYPERn_EL0: bool,
        gs_89224: bool,
        gs_89220: bool,
        gs_89243: bool,
        u__PSTATE_EL: u8,
        gs_89237: bool,
        gs_89225: bool,
        gs_89223: bool,
        u__MDCR_EL2_TPM: bool,
        gs_89222: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        gs_89232: bool,
        gs_89221: bool,
        u__SCR_EL3_FGTEn: bool,
        u__PMUSERENR_EL0_EN: bool,
        gs_89250: bool,
        gs_89242: bool,
        gs_89210: bool,
        gs_89244: bool,
        gs_89213: bool,
        gs_89219: bool,
        gs_89236: bool,
        gs_89238: bool,
        gs_89241: bool,
        gs_89251: bool,
        gs_89209: bool,
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
        // C s_0_23: const #17360u : u32
        let s_0_23: u32 = 17360;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HDFGWTR_EL2_Type_PMEVTYPERn_EL0(s_0_24)
        let s_0_25: bool = u_get_HDFGWTR_EL2_Type_PMEVTYPERn_EL0(state, tracer, s_0_24);
        // D s_0_26: write-var __HDFGWTR_EL2_PMEVTYPERn_EL0 <= s_0_25
        fn_state.u__HDFGWTR_EL2_PMEVTYPERn_EL0 = s_0_25;
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
        // D s_0_31: read-var __PSTATE_EL:u8
        let s_0_31: u8 = fn_state.u__PSTATE_EL;
        // D s_0_32: cast zx s_0_31 -> bv
        let s_0_32: Bits = Bits::new(s_0_31 as u128, 2u16);
        // C s_0_33: const #448u : u32
        let s_0_33: u32 = 448;
        // D s_0_34: read-reg s_0_33:u8
        let s_0_34: u8 = {
            let value = state.read_register::<u8>(s_0_33 as isize);
            tracer.read_register(s_0_33 as isize, value);
            value
        };
        // D s_0_35: cast zx s_0_34 -> bv
        let s_0_35: Bits = Bits::new(s_0_34 as u128, 2u16);
        // D s_0_36: cmp-eq s_0_32 s_0_35
        let s_0_36: bool = ((s_0_32) == (s_0_35));
        // N s_0_37: branch s_0_36 b82 b1
        if s_0_36 {
            return block_82(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b31 b2
        if s_1_5 {
            return block_31(state, tracer, fn_state);
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
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // D s_5_4: call Mk_PMEVTYPER_EL0_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_PMEVTYPER_EL0_Type(
            state,
            tracer,
            s_5_3,
        );
        // C s_5_5: const #14s : i
        let s_5_5: i128 = 14;
        // C s_5_6: const #11208u : u32
        let s_5_6: u32 = 11208;
        // D s_5_7: read-reg s_5_6:[struct; 32]
        let s_5_7: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 32usize]>(s_5_6 as isize);
            tracer.read_register(s_5_6 as isize, value);
            value
        };
        // D s_5_8: mutate-element s_5_7[s_5_5] <= s_5_4
        let s_5_8: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let mut local = s_5_7.clone();
            local[(s_5_5) as usize] = s_5_4;
            local
        };
        // D s_5_9: cast cvt s_5_8 -> [struct; 0]
        let s_5_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_5_8,
        );
        // D s_5_10: cast cvt s_5_9 -> [struct; 32]
        let s_5_10: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_5_9);
            buf
        };
        // C s_5_11: const #11208u : u32
        let s_5_11: u32 = 11208;
        // N s_5_12: write-reg s_5_11 <= s_5_10
        let s_5_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_5_11 as isize, s_5_10);
            tracer.write_register(s_5_11 as isize, s_5_10);
        };
        // N s_5_13: return
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
        // N s_6_2: branch s_6_1 b30 b7
        if s_6_1 {
            return block_30(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#89209 <= s_7_0
        fn_state.gs_89209 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#89209:u8
        let s_8_0: bool = fn_state.gs_89209;
        // N s_8_1: branch s_8_0 b29 b9
        if s_8_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#89210 <= s_9_0
        fn_state.gs_89210 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#89210:u8
        let s_10_0: bool = fn_state.gs_89210;
        // N s_10_1: branch s_10_0 b28 b11
        if s_10_0 {
            return block_28(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#89211 <= s_11_0
        fn_state.gs_89211 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#89211:u8
        let s_12_0: bool = fn_state.gs_89211;
        // N s_12_1: branch s_12_0 b27 b13
        if s_12_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#89212 <= s_13_0
        fn_state.gs_89212 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#89212:u8
        let s_14_0: bool = fn_state.gs_89212;
        // N s_14_1: branch s_14_0 b26 b15
        if s_14_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #424u : u32
        let s_15_0: u32 = 424;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // D s_15_3: cmp-lt s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) < (s_15_2));
        // N s_15_4: branch s_15_3 b25 b16
        if s_15_3 {
            return block_25(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#89213 <= s_16_0
        fn_state.gs_89213 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#89213:u8
        let s_17_0: bool = fn_state.gs_89213;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #64s : i64
        let s_18_0: i64 = 64;
        // D s_18_1: read-var t:i
        let s_18_1: i128 = fn_state.t;
        // D s_18_2: call X_read(s_18_1, s_18_0)
        let s_18_2: Bits = X_read(state, tracer, s_18_1, s_18_0);
        // D s_18_3: cast reint s_18_2 -> u64
        let s_18_3: u64 = (s_18_2.value() as u64);
        // D s_18_4: call Mk_PMEVTYPER_EL0_Type(s_18_3)
        let s_18_4: ProductType5c790c8ef59cc8b2 = Mk_PMEVTYPER_EL0_Type(
            state,
            tracer,
            s_18_3,
        );
        // C s_18_5: const #14s : i
        let s_18_5: i128 = 14;
        // C s_18_6: const #11208u : u32
        let s_18_6: u32 = 11208;
        // D s_18_7: read-reg s_18_6:[struct; 32]
        let s_18_7: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_18_6 as isize);
            tracer.read_register(s_18_6 as isize, value);
            value
        };
        // D s_18_8: mutate-element s_18_7[s_18_5] <= s_18_4
        let s_18_8: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let mut local = s_18_7.clone();
            local[(s_18_5) as usize] = s_18_4;
            local
        };
        // D s_18_9: cast cvt s_18_8 -> [struct; 0]
        let s_18_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_18_8,
        );
        // D s_18_10: cast cvt s_18_9 -> [struct; 32]
        let s_18_10: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_18_9);
            buf
        };
        // C s_18_11: const #11208u : u32
        let s_18_11: u32 = 11208;
        // N s_18_12: write-reg s_18_11 <= s_18_10
        let s_18_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_18_11 as isize, s_18_10);
            tracer.write_register(s_18_11 as isize, s_18_10);
        };
        // N s_18_13: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call Halted(s_19_0)
        let s_19_1: bool = Halted(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b24 b20
        if s_19_1 {
            return block_24(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#89216 <= s_20_0
        fn_state.gs_89216 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#89216:u8
        let s_21_0: bool = fn_state.gs_89216;
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
        // C s_22_0: const #24u : u8
        let s_22_0: u8 = 24;
        // C s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 8u16);
        // C s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (s_22_1.value() as i128);
        // C s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #424u : u32
        let s_22_5: u32 = 424;
        // D s_22_6: read-reg s_22_5:u8
        let s_22_6: u8 = {
            let value = state.read_register::<u8>(s_22_5 as isize);
            tracer.read_register(s_22_5 as isize, value);
            value
        };
        // D s_22_7: call AArch64_SystemAccessTrap(s_22_6, s_22_4)
        let s_22_7: () = AArch64_SystemAccessTrap(state, tracer, s_22_6, s_22_4);
        // N s_22_8: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var __EDSCR_SDD:u8
        let s_24_0: bool = fn_state.u__EDSCR_SDD;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#89216 <= s_24_4
        fn_state.gs_89216 = s_24_4;
        // N s_24_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __MDCR_EL3_TPM:u8
        let s_25_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#89213 <= s_25_4
        fn_state.gs_89213 = s_25_4;
        // N s_25_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __MDCR_EL3_TPM:u8
        let s_27_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #1u : u8
        let s_27_2: bool = true;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#89212 <= s_27_4
        fn_state.gs_89212 = s_27_4;
        // N s_27_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_28_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_28_1: call __IMPDEF_boolean(s_28_0)
        let s_28_1: bool = u__IMPDEF_boolean(state, tracer, s_28_0);
        // D s_28_2: write-var gs#89211 <= s_28_1
        fn_state.gs_89211 = s_28_1;
        // N s_28_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __EDSCR_SDD:u8
        let s_29_0: bool = fn_state.u__EDSCR_SDD;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#89210 <= s_29_4
        fn_state.gs_89210 = s_29_4;
        // N s_29_6: jump b10
        return block_10(state, tracer, fn_state);
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
        // C s_30_2: const #2u : u8
        let s_30_2: u8 = 2;
        // D s_30_3: cmp-lt s_30_1 s_30_2
        let s_30_3: bool = ((s_30_1) < (s_30_2));
        // D s_30_4: write-var gs#89209 <= s_30_3
        fn_state.gs_89209 = s_30_3;
        // N s_30_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call Halted(s_31_0)
        let s_31_1: bool = Halted(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b81 b32
        if s_31_1 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#89217 <= s_32_0
        fn_state.gs_89217 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#89217:u8
        let s_33_0: bool = fn_state.gs_89217;
        // N s_33_1: branch s_33_0 b80 b34
        if s_33_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#89218 <= s_34_0
        fn_state.gs_89218 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#89218:u8
        let s_35_0: bool = fn_state.gs_89218;
        // N s_35_1: branch s_35_0 b79 b36
        if s_35_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#89219 <= s_36_0
        fn_state.gs_89219 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#89219:u8
        let s_37_0: bool = fn_state.gs_89219;
        // N s_37_1: branch s_37_0 b78 b38
        if s_37_0 {
            return block_78(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#89220 <= s_38_0
        fn_state.gs_89220 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#89220:u8
        let s_39_0: bool = fn_state.gs_89220;
        // N s_39_1: branch s_39_0 b77 b40
        if s_39_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EL2Enabled(s_40_0)
        let s_40_1: bool = EL2Enabled(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b76 b41
        if s_40_1 {
            return block_76(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#89221 <= s_41_0
        fn_state.gs_89221 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#89221:u8
        let s_42_0: bool = fn_state.gs_89221;
        // N s_42_1: branch s_42_0 b72 b43
        if s_42_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#89223 <= s_43_0
        fn_state.gs_89223 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#89223:u8
        let s_44_0: bool = fn_state.gs_89223;
        // N s_44_1: branch s_44_0 b71 b45
        if s_44_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#89224 <= s_45_0
        fn_state.gs_89224 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#89224:u8
        let s_46_0: bool = fn_state.gs_89224;
        // N s_46_1: branch s_46_0 b70 b47
        if s_46_0 {
            return block_70(state, tracer, fn_state);
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
        // N s_47_2: branch s_47_1 b69 b48
        if s_47_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#89225 <= s_48_0
        fn_state.gs_89225 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#89225:u8
        let s_49_0: bool = fn_state.gs_89225;
        // N s_49_1: branch s_49_0 b68 b50
        if s_49_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call EL2Enabled(s_50_0)
        let s_50_1: bool = EL2Enabled(state, tracer, s_50_0);
        // N s_50_2: branch s_50_1 b67 b51
        if s_50_1 {
            return block_67(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#89227 <= s_51_0
        fn_state.gs_89227 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#89227:u8
        let s_52_0: bool = fn_state.gs_89227;
        // N s_52_1: branch s_52_0 b64 b53
        if s_52_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #424u : u32
        let s_53_0: u32 = 424;
        // D s_53_1: read-reg s_53_0:u8
        let s_53_1: u8 = {
            let value = state.read_register::<u8>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // C s_53_2: const #2u : u8
        let s_53_2: u8 = 2;
        // D s_53_3: cmp-lt s_53_1 s_53_2
        let s_53_3: bool = ((s_53_1) < (s_53_2));
        // N s_53_4: branch s_53_3 b63 b54
        if s_53_3 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#89228 <= s_54_0
        fn_state.gs_89228 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#89228:u8
        let s_55_0: bool = fn_state.gs_89228;
        // N s_55_1: branch s_55_0 b57 b56
        if s_55_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #64s : i64
        let s_56_0: i64 = 64;
        // D s_56_1: read-var t:i
        let s_56_1: i128 = fn_state.t;
        // D s_56_2: call X_read(s_56_1, s_56_0)
        let s_56_2: Bits = X_read(state, tracer, s_56_1, s_56_0);
        // D s_56_3: cast reint s_56_2 -> u64
        let s_56_3: u64 = (s_56_2.value() as u64);
        // D s_56_4: call Mk_PMEVTYPER_EL0_Type(s_56_3)
        let s_56_4: ProductType5c790c8ef59cc8b2 = Mk_PMEVTYPER_EL0_Type(
            state,
            tracer,
            s_56_3,
        );
        // C s_56_5: const #14s : i
        let s_56_5: i128 = 14;
        // C s_56_6: const #11208u : u32
        let s_56_6: u32 = 11208;
        // D s_56_7: read-reg s_56_6:[struct; 32]
        let s_56_7: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_56_6 as isize);
            tracer.read_register(s_56_6 as isize, value);
            value
        };
        // D s_56_8: mutate-element s_56_7[s_56_5] <= s_56_4
        let s_56_8: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let mut local = s_56_7.clone();
            local[(s_56_5) as usize] = s_56_4;
            local
        };
        // D s_56_9: cast cvt s_56_8 -> [struct; 0]
        let s_56_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_56_8,
        );
        // D s_56_10: cast cvt s_56_9 -> [struct; 32]
        let s_56_10: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_56_9);
            buf
        };
        // C s_56_11: const #11208u : u32
        let s_56_11: u32 = 11208;
        // N s_56_12: write-reg s_56_11 <= s_56_10
        let s_56_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_56_11 as isize, s_56_10);
            tracer.write_register(s_56_11 as isize, s_56_10);
        };
        // N s_56_13: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call Halted(s_57_0)
        let s_57_1: bool = Halted(state, tracer, s_57_0);
        // N s_57_2: branch s_57_1 b62 b58
        if s_57_1 {
            return block_62(state, tracer, fn_state);
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
        // D s_58_1: write-var gs#89231 <= s_58_0
        fn_state.gs_89231 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#89231:u8
        let s_59_0: bool = fn_state.gs_89231;
        // N s_59_1: branch s_59_0 b61 b60
        if s_59_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #24u : u8
        let s_60_0: u8 = 24;
        // C s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 8u16);
        // C s_60_2: cast zx s_60_1 -> i
        let s_60_2: i128 = (s_60_1.value() as i128);
        // C s_60_3: cast reint s_60_2 -> i64
        let s_60_3: i64 = (s_60_2 as i64);
        // C s_60_4: cast zx s_60_3 -> i
        let s_60_4: i128 = (i128::try_from(s_60_3).unwrap());
        // C s_60_5: const #424u : u32
        let s_60_5: u32 = 424;
        // D s_60_6: read-reg s_60_5:u8
        let s_60_6: u8 = {
            let value = state.read_register::<u8>(s_60_5 as isize);
            tracer.read_register(s_60_5 as isize, value);
            value
        };
        // D s_60_7: call AArch64_SystemAccessTrap(s_60_6, s_60_4)
        let s_60_7: () = AArch64_SystemAccessTrap(state, tracer, s_60_6, s_60_4);
        // N s_60_8: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_61_0: panic
        panic!("{:?}", ());
        // N s_61_1: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var __EDSCR_SDD:u8
        let s_62_0: bool = fn_state.u__EDSCR_SDD;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#89231 <= s_62_4
        fn_state.gs_89231 = s_62_4;
        // N s_62_6: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var __MDCR_EL3_TPM:u8
        let s_63_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #1u : u8
        let s_63_2: bool = true;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#89228 <= s_63_4
        fn_state.gs_89228 = s_63_4;
        // N s_63_6: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #146u : u32
        let s_64_0: u32 = 146;
        // S s_64_1: call IsFeatureImplemented(s_64_0)
        let s_64_1: bool = IsFeatureImplemented(state, tracer, s_64_0);
        // S s_64_2: not s_64_1
        let s_64_2: bool = !s_64_1;
        // N s_64_3: branch s_64_2 b66 b65
        if s_64_2 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #24u : u8
        let s_65_0: u8 = 24;
        // C s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 8u16);
        // C s_65_2: cast zx s_65_1 -> i
        let s_65_2: i128 = (s_65_1.value() as i128);
        // C s_65_3: cast reint s_65_2 -> i64
        let s_65_3: i64 = (s_65_2 as i64);
        // C s_65_4: cast zx s_65_3 -> i
        let s_65_4: i128 = (i128::try_from(s_65_3).unwrap());
        // C s_65_5: const #432u : u32
        let s_65_5: u32 = 432;
        // D s_65_6: read-reg s_65_5:u8
        let s_65_6: u8 = {
            let value = state.read_register::<u8>(s_65_5 as isize);
            tracer.read_register(s_65_5 as isize, value);
            value
        };
        // D s_65_7: call AArch64_SystemAccessTrap(s_65_6, s_65_4)
        let s_65_7: () = AArch64_SystemAccessTrap(state, tracer, s_65_6, s_65_4);
        // N s_65_8: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #72u : u32
        let s_66_0: u32 = 72;
        // S s_66_1: call ConstrainUnpredictableProcedure(s_66_0)
        let s_66_1: () = ConstrainUnpredictableProcedure(state, tracer, s_66_0);
        // N s_66_2: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call AArch64_GetNumEventCountersAccessible(s_67_0)
        let s_67_1: i128 = AArch64_GetNumEventCountersAccessible(state, tracer, s_67_0);
        // C s_67_2: const #14s : i
        let s_67_2: i128 = 14;
        // S s_67_3: cmp-ge s_67_2 s_67_1
        let s_67_3: bool = ((s_67_2) >= (s_67_1));
        // D s_67_4: write-var gs#89227 <= s_67_3
        fn_state.gs_89227 = s_67_3;
        // N s_67_5: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #24u : u8
        let s_68_0: u8 = 24;
        // C s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 8u16);
        // C s_68_2: cast zx s_68_1 -> i
        let s_68_2: i128 = (s_68_1.value() as i128);
        // C s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: cast zx s_68_3 -> i
        let s_68_4: i128 = (i128::try_from(s_68_3).unwrap());
        // C s_68_5: const #432u : u32
        let s_68_5: u32 = 432;
        // D s_68_6: read-reg s_68_5:u8
        let s_68_6: u8 = {
            let value = state.read_register::<u8>(s_68_5 as isize);
            tracer.read_register(s_68_5 as isize, value);
            value
        };
        // D s_68_7: call AArch64_SystemAccessTrap(s_68_6, s_68_4)
        let s_68_7: () = AArch64_SystemAccessTrap(state, tracer, s_68_6, s_68_4);
        // N s_68_8: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __MDCR_EL2_TPM:u8
        let s_69_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#89225 <= s_69_4
        fn_state.gs_89225 = s_69_4;
        // N s_69_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #24u : u8
        let s_70_0: u8 = 24;
        // C s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 8u16);
        // C s_70_2: cast zx s_70_1 -> i
        let s_70_2: i128 = (s_70_1.value() as i128);
        // C s_70_3: cast reint s_70_2 -> i64
        let s_70_3: i64 = (s_70_2 as i64);
        // C s_70_4: cast zx s_70_3 -> i
        let s_70_4: i128 = (i128::try_from(s_70_3).unwrap());
        // C s_70_5: const #432u : u32
        let s_70_5: u32 = 432;
        // D s_70_6: read-reg s_70_5:u8
        let s_70_6: u8 = {
            let value = state.read_register::<u8>(s_70_5 as isize);
            tracer.read_register(s_70_5 as isize, value);
            value
        };
        // D s_70_7: call AArch64_SystemAccessTrap(s_70_6, s_70_4)
        let s_70_7: () = AArch64_SystemAccessTrap(state, tracer, s_70_6, s_70_4);
        // N s_70_8: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __HDFGWTR_EL2_PMEVTYPERn_EL0:u8
        let s_71_0: bool = fn_state.u__HDFGWTR_EL2_PMEVTYPERn_EL0;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #1u : u8
        let s_71_2: bool = true;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#89224 <= s_71_4
        fn_state.gs_89224 = s_71_4;
        // N s_71_6: jump b46
        return block_46(state, tracer, fn_state);
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
        // D s_72_4: not s_72_3
        let s_72_4: bool = !s_72_3;
        // N s_72_5: branch s_72_4 b75 b73
        if s_72_4 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __SCR_EL3_FGTEn:u8
        let s_73_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#89222 <= s_73_4
        fn_state.gs_89222 = s_73_4;
        // N s_73_6: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#89222:u8
        let s_74_0: bool = fn_state.gs_89222;
        // D s_74_1: write-var gs#89223 <= s_74_0
        fn_state.gs_89223 = s_74_0;
        // N s_74_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#89222 <= s_75_0
        fn_state.gs_89222 = s_75_0;
        // N s_75_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #146u : u32
        let s_76_0: u32 = 146;
        // S s_76_1: call IsFeatureImplemented(s_76_0)
        let s_76_1: bool = IsFeatureImplemented(state, tracer, s_76_0);
        // D s_76_2: write-var gs#89221 <= s_76_1
        fn_state.gs_89221 = s_76_1;
        // N s_76_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_77_0: panic
        panic!("{:?}", ());
        // N s_77_1: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var __MDCR_EL3_TPM:u8
        let s_78_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #1u : u8
        let s_78_2: bool = true;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#89220 <= s_78_4
        fn_state.gs_89220 = s_78_4;
        // N s_78_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_79_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_79_1: call __IMPDEF_boolean(s_79_0)
        let s_79_1: bool = u__IMPDEF_boolean(state, tracer, s_79_0);
        // D s_79_2: write-var gs#89219 <= s_79_1
        fn_state.gs_89219 = s_79_1;
        // N s_79_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __EDSCR_SDD:u8
        let s_80_0: bool = fn_state.u__EDSCR_SDD;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#89218 <= s_80_4
        fn_state.gs_89218 = s_80_4;
        // N s_80_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #424u : u32
        let s_81_0: u32 = 424;
        // D s_81_1: read-reg s_81_0:u8
        let s_81_1: u8 = {
            let value = state.read_register::<u8>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // C s_81_2: const #2u : u8
        let s_81_2: u8 = 2;
        // D s_81_3: cmp-lt s_81_1 s_81_2
        let s_81_3: bool = ((s_81_1) < (s_81_2));
        // D s_81_4: write-var gs#89217 <= s_81_3
        fn_state.gs_89217 = s_81_3;
        // N s_81_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call Halted(s_82_0)
        let s_82_1: bool = Halted(state, tracer, s_82_0);
        // N s_82_2: branch s_82_1 b151 b83
        if s_82_1 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#89232 <= s_83_0
        fn_state.gs_89232 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#89232:u8
        let s_84_0: bool = fn_state.gs_89232;
        // N s_84_1: branch s_84_0 b150 b85
        if s_84_0 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#89233 <= s_85_0
        fn_state.gs_89233 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#89233:u8
        let s_86_0: bool = fn_state.gs_89233;
        // N s_86_1: branch s_86_0 b149 b87
        if s_86_0 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#89234 <= s_87_0
        fn_state.gs_89234 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#89234:u8
        let s_88_0: bool = fn_state.gs_89234;
        // N s_88_1: branch s_88_0 b148 b89
        if s_88_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#89235 <= s_89_0
        fn_state.gs_89235 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#89235:u8
        let s_90_0: bool = fn_state.gs_89235;
        // N s_90_1: branch s_90_0 b147 b91
        if s_90_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #204u : u32
        let s_91_0: u32 = 204;
        // S s_91_1: call IsFeatureImplemented(s_91_0)
        let s_91_1: bool = IsFeatureImplemented(state, tracer, s_91_0);
        // N s_91_2: branch s_91_1 b146 b92
        if s_91_1 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#89236 <= s_92_0
        fn_state.gs_89236 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#89236:u8
        let s_93_0: bool = fn_state.gs_89236;
        // N s_93_1: branch s_93_0 b145 b94
        if s_93_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #204u : u32
        let s_94_0: u32 = 204;
        // S s_94_1: call IsFeatureImplemented(s_94_0)
        let s_94_1: bool = IsFeatureImplemented(state, tracer, s_94_0);
        // S s_94_2: not s_94_1
        let s_94_2: bool = !s_94_1;
        // N s_94_3: branch s_94_2 b144 b95
        if s_94_2 {
            return block_144(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#89237 <= s_95_0
        fn_state.gs_89237 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#89237:u8
        let s_96_0: bool = fn_state.gs_89237;
        // D s_96_1: write-var gs#89238 <= s_96_0
        fn_state.gs_89238 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#89238:u8
        let s_97_0: bool = fn_state.gs_89238;
        // N s_97_1: branch s_97_0 b138 b98
        if s_97_0 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call EL2Enabled(s_98_0)
        let s_98_1: bool = EL2Enabled(state, tracer, s_98_0);
        // N s_98_2: branch s_98_1 b137 b99
        if s_98_1 {
            return block_137(state, tracer, fn_state);
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
        // D s_99_1: write-var gs#89239 <= s_99_0
        fn_state.gs_89239 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#89239:u8
        let s_100_0: bool = fn_state.gs_89239;
        // N s_100_1: branch s_100_0 b136 b101
        if s_100_0 {
            return block_136(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#89240 <= s_101_0
        fn_state.gs_89240 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#89240:u8
        let s_102_0: bool = fn_state.gs_89240;
        // N s_102_1: branch s_102_0 b132 b103
        if s_102_0 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#89242 <= s_103_0
        fn_state.gs_89242 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#89242:u8
        let s_104_0: bool = fn_state.gs_89242;
        // N s_104_1: branch s_104_0 b131 b105
        if s_104_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#89243 <= s_105_0
        fn_state.gs_89243 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#89243:u8
        let s_106_0: bool = fn_state.gs_89243;
        // N s_106_1: branch s_106_0 b130 b107
        if s_106_0 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #() : ()
        let s_107_0: () = ();
        // S s_107_1: call EL2Enabled(s_107_0)
        let s_107_1: bool = EL2Enabled(state, tracer, s_107_0);
        // N s_107_2: branch s_107_1 b129 b108
        if s_107_1 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #0u : u8
        let s_108_0: bool = false;
        // D s_108_1: write-var gs#89244 <= s_108_0
        fn_state.gs_89244 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#89244:u8
        let s_109_0: bool = fn_state.gs_89244;
        // N s_109_1: branch s_109_0 b128 b110
        if s_109_0 {
            return block_128(state, tracer, fn_state);
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
        // N s_110_2: branch s_110_1 b127 b111
        if s_110_1 {
            return block_127(state, tracer, fn_state);
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
        // D s_111_1: write-var gs#89246 <= s_111_0
        fn_state.gs_89246 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#89246:u8
        let s_112_0: bool = fn_state.gs_89246;
        // N s_112_1: branch s_112_0 b124 b113
        if s_112_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #424u : u32
        let s_113_0: u32 = 424;
        // D s_113_1: read-reg s_113_0:u8
        let s_113_1: u8 = {
            let value = state.read_register::<u8>(s_113_0 as isize);
            tracer.read_register(s_113_0 as isize, value);
            value
        };
        // C s_113_2: const #2u : u8
        let s_113_2: u8 = 2;
        // D s_113_3: cmp-lt s_113_1 s_113_2
        let s_113_3: bool = ((s_113_1) < (s_113_2));
        // N s_113_4: branch s_113_3 b123 b114
        if s_113_3 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #0u : u8
        let s_114_0: bool = false;
        // D s_114_1: write-var gs#89247 <= s_114_0
        fn_state.gs_89247 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#89247:u8
        let s_115_0: bool = fn_state.gs_89247;
        // N s_115_1: branch s_115_0 b117 b116
        if s_115_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #64s : i64
        let s_116_0: i64 = 64;
        // D s_116_1: read-var t:i
        let s_116_1: i128 = fn_state.t;
        // D s_116_2: call X_read(s_116_1, s_116_0)
        let s_116_2: Bits = X_read(state, tracer, s_116_1, s_116_0);
        // D s_116_3: cast reint s_116_2 -> u64
        let s_116_3: u64 = (s_116_2.value() as u64);
        // D s_116_4: call Mk_PMEVTYPER_EL0_Type(s_116_3)
        let s_116_4: ProductType5c790c8ef59cc8b2 = Mk_PMEVTYPER_EL0_Type(
            state,
            tracer,
            s_116_3,
        );
        // C s_116_5: const #14s : i
        let s_116_5: i128 = 14;
        // C s_116_6: const #11208u : u32
        let s_116_6: u32 = 11208;
        // D s_116_7: read-reg s_116_6:[struct; 32]
        let s_116_7: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_116_6 as isize);
            tracer.read_register(s_116_6 as isize, value);
            value
        };
        // D s_116_8: mutate-element s_116_7[s_116_5] <= s_116_4
        let s_116_8: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let mut local = s_116_7.clone();
            local[(s_116_5) as usize] = s_116_4;
            local
        };
        // D s_116_9: cast cvt s_116_8 -> [struct; 0]
        let s_116_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_116_8,
        );
        // D s_116_10: cast cvt s_116_9 -> [struct; 32]
        let s_116_10: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_116_9);
            buf
        };
        // C s_116_11: const #11208u : u32
        let s_116_11: u32 = 11208;
        // N s_116_12: write-reg s_116_11 <= s_116_10
        let s_116_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_116_11 as isize, s_116_10);
            tracer.write_register(s_116_11 as isize, s_116_10);
        };
        // N s_116_13: return
        return;
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #() : ()
        let s_117_0: () = ();
        // S s_117_1: call Halted(s_117_0)
        let s_117_1: bool = Halted(state, tracer, s_117_0);
        // N s_117_2: branch s_117_1 b122 b118
        if s_117_1 {
            return block_122(state, tracer, fn_state);
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
        // D s_118_1: write-var gs#89250 <= s_118_0
        fn_state.gs_89250 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#89250:u8
        let s_119_0: bool = fn_state.gs_89250;
        // N s_119_1: branch s_119_0 b121 b120
        if s_119_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #24u : u8
        let s_120_0: u8 = 24;
        // C s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 8u16);
        // C s_120_2: cast zx s_120_1 -> i
        let s_120_2: i128 = (s_120_1.value() as i128);
        // C s_120_3: cast reint s_120_2 -> i64
        let s_120_3: i64 = (s_120_2 as i64);
        // C s_120_4: cast zx s_120_3 -> i
        let s_120_4: i128 = (i128::try_from(s_120_3).unwrap());
        // C s_120_5: const #424u : u32
        let s_120_5: u32 = 424;
        // D s_120_6: read-reg s_120_5:u8
        let s_120_6: u8 = {
            let value = state.read_register::<u8>(s_120_5 as isize);
            tracer.read_register(s_120_5 as isize, value);
            value
        };
        // D s_120_7: call AArch64_SystemAccessTrap(s_120_6, s_120_4)
        let s_120_7: () = AArch64_SystemAccessTrap(state, tracer, s_120_6, s_120_4);
        // N s_120_8: return
        return;
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_121_0: panic
        panic!("{:?}", ());
        // N s_121_1: return
        return;
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var __EDSCR_SDD:u8
        let s_122_0: bool = fn_state.u__EDSCR_SDD;
        // D s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 1u16);
        // C s_122_2: const #1u : u8
        let s_122_2: bool = true;
        // C s_122_3: cast zx s_122_2 -> bv
        let s_122_3: Bits = Bits::new(s_122_2 as u128, 1u16);
        // D s_122_4: cmp-eq s_122_1 s_122_3
        let s_122_4: bool = ((s_122_1) == (s_122_3));
        // D s_122_5: write-var gs#89250 <= s_122_4
        fn_state.gs_89250 = s_122_4;
        // N s_122_6: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var __MDCR_EL3_TPM:u8
        let s_123_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 1u16);
        // C s_123_2: const #1u : u8
        let s_123_2: bool = true;
        // C s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 1u16);
        // D s_123_4: cmp-eq s_123_1 s_123_3
        let s_123_4: bool = ((s_123_1) == (s_123_3));
        // D s_123_5: write-var gs#89247 <= s_123_4
        fn_state.gs_89247 = s_123_4;
        // N s_123_6: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #146u : u32
        let s_124_0: u32 = 146;
        // S s_124_1: call IsFeatureImplemented(s_124_0)
        let s_124_1: bool = IsFeatureImplemented(state, tracer, s_124_0);
        // S s_124_2: not s_124_1
        let s_124_2: bool = !s_124_1;
        // N s_124_3: branch s_124_2 b126 b125
        if s_124_2 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #24u : u8
        let s_125_0: u8 = 24;
        // C s_125_1: cast zx s_125_0 -> bv
        let s_125_1: Bits = Bits::new(s_125_0 as u128, 8u16);
        // C s_125_2: cast zx s_125_1 -> i
        let s_125_2: i128 = (s_125_1.value() as i128);
        // C s_125_3: cast reint s_125_2 -> i64
        let s_125_3: i64 = (s_125_2 as i64);
        // C s_125_4: cast zx s_125_3 -> i
        let s_125_4: i128 = (i128::try_from(s_125_3).unwrap());
        // C s_125_5: const #432u : u32
        let s_125_5: u32 = 432;
        // D s_125_6: read-reg s_125_5:u8
        let s_125_6: u8 = {
            let value = state.read_register::<u8>(s_125_5 as isize);
            tracer.read_register(s_125_5 as isize, value);
            value
        };
        // D s_125_7: call AArch64_SystemAccessTrap(s_125_6, s_125_4)
        let s_125_7: () = AArch64_SystemAccessTrap(state, tracer, s_125_6, s_125_4);
        // N s_125_8: return
        return;
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #72u : u32
        let s_126_0: u32 = 72;
        // S s_126_1: call ConstrainUnpredictableProcedure(s_126_0)
        let s_126_1: () = ConstrainUnpredictableProcedure(state, tracer, s_126_0);
        // N s_126_2: return
        return;
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #() : ()
        let s_127_0: () = ();
        // S s_127_1: call AArch64_GetNumEventCountersAccessible(s_127_0)
        let s_127_1: i128 = AArch64_GetNumEventCountersAccessible(
            state,
            tracer,
            s_127_0,
        );
        // C s_127_2: const #14s : i
        let s_127_2: i128 = 14;
        // S s_127_3: cmp-ge s_127_2 s_127_1
        let s_127_3: bool = ((s_127_2) >= (s_127_1));
        // D s_127_4: write-var gs#89246 <= s_127_3
        fn_state.gs_89246 = s_127_3;
        // N s_127_5: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #24u : u8
        let s_128_0: u8 = 24;
        // C s_128_1: cast zx s_128_0 -> bv
        let s_128_1: Bits = Bits::new(s_128_0 as u128, 8u16);
        // C s_128_2: cast zx s_128_1 -> i
        let s_128_2: i128 = (s_128_1.value() as i128);
        // C s_128_3: cast reint s_128_2 -> i64
        let s_128_3: i64 = (s_128_2 as i64);
        // C s_128_4: cast zx s_128_3 -> i
        let s_128_4: i128 = (i128::try_from(s_128_3).unwrap());
        // C s_128_5: const #432u : u32
        let s_128_5: u32 = 432;
        // D s_128_6: read-reg s_128_5:u8
        let s_128_6: u8 = {
            let value = state.read_register::<u8>(s_128_5 as isize);
            tracer.read_register(s_128_5 as isize, value);
            value
        };
        // D s_128_7: call AArch64_SystemAccessTrap(s_128_6, s_128_4)
        let s_128_7: () = AArch64_SystemAccessTrap(state, tracer, s_128_6, s_128_4);
        // N s_128_8: return
        return;
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var __MDCR_EL2_TPM:u8
        let s_129_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 1u16);
        // C s_129_2: const #1u : u8
        let s_129_2: bool = true;
        // C s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 1u16);
        // D s_129_4: cmp-eq s_129_1 s_129_3
        let s_129_4: bool = ((s_129_1) == (s_129_3));
        // D s_129_5: write-var gs#89244 <= s_129_4
        fn_state.gs_89244 = s_129_4;
        // N s_129_6: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #24u : u8
        let s_130_0: u8 = 24;
        // C s_130_1: cast zx s_130_0 -> bv
        let s_130_1: Bits = Bits::new(s_130_0 as u128, 8u16);
        // C s_130_2: cast zx s_130_1 -> i
        let s_130_2: i128 = (s_130_1.value() as i128);
        // C s_130_3: cast reint s_130_2 -> i64
        let s_130_3: i64 = (s_130_2 as i64);
        // C s_130_4: cast zx s_130_3 -> i
        let s_130_4: i128 = (i128::try_from(s_130_3).unwrap());
        // C s_130_5: const #432u : u32
        let s_130_5: u32 = 432;
        // D s_130_6: read-reg s_130_5:u8
        let s_130_6: u8 = {
            let value = state.read_register::<u8>(s_130_5 as isize);
            tracer.read_register(s_130_5 as isize, value);
            value
        };
        // D s_130_7: call AArch64_SystemAccessTrap(s_130_6, s_130_4)
        let s_130_7: () = AArch64_SystemAccessTrap(state, tracer, s_130_6, s_130_4);
        // N s_130_8: return
        return;
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var __HDFGWTR_EL2_PMEVTYPERn_EL0:u8
        let s_131_0: bool = fn_state.u__HDFGWTR_EL2_PMEVTYPERn_EL0;
        // D s_131_1: cast zx s_131_0 -> bv
        let s_131_1: Bits = Bits::new(s_131_0 as u128, 1u16);
        // C s_131_2: const #1u : u8
        let s_131_2: bool = true;
        // C s_131_3: cast zx s_131_2 -> bv
        let s_131_3: Bits = Bits::new(s_131_2 as u128, 1u16);
        // D s_131_4: cmp-eq s_131_1 s_131_3
        let s_131_4: bool = ((s_131_1) == (s_131_3));
        // D s_131_5: write-var gs#89243 <= s_131_4
        fn_state.gs_89243 = s_131_4;
        // N s_131_6: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #424u : u32
        let s_132_0: u32 = 424;
        // D s_132_1: read-reg s_132_0:u8
        let s_132_1: u8 = {
            let value = state.read_register::<u8>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // C s_132_2: const #2u : u8
        let s_132_2: u8 = 2;
        // D s_132_3: cmp-lt s_132_1 s_132_2
        let s_132_3: bool = ((s_132_1) < (s_132_2));
        // D s_132_4: not s_132_3
        let s_132_4: bool = !s_132_3;
        // N s_132_5: branch s_132_4 b135 b133
        if s_132_4 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var __SCR_EL3_FGTEn:u8
        let s_133_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_133_1: cast zx s_133_0 -> bv
        let s_133_1: Bits = Bits::new(s_133_0 as u128, 1u16);
        // C s_133_2: const #1u : u8
        let s_133_2: bool = true;
        // C s_133_3: cast zx s_133_2 -> bv
        let s_133_3: Bits = Bits::new(s_133_2 as u128, 1u16);
        // D s_133_4: cmp-eq s_133_1 s_133_3
        let s_133_4: bool = ((s_133_1) == (s_133_3));
        // D s_133_5: write-var gs#89241 <= s_133_4
        fn_state.gs_89241 = s_133_4;
        // N s_133_6: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#89241:u8
        let s_134_0: bool = fn_state.gs_89241;
        // D s_134_1: write-var gs#89242 <= s_134_0
        fn_state.gs_89242 = s_134_0;
        // N s_134_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #1u : u8
        let s_135_0: bool = true;
        // D s_135_1: write-var gs#89241 <= s_135_0
        fn_state.gs_89241 = s_135_0;
        // N s_135_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #146u : u32
        let s_136_0: u32 = 146;
        // S s_136_1: call IsFeatureImplemented(s_136_0)
        let s_136_1: bool = IsFeatureImplemented(state, tracer, s_136_0);
        // D s_136_2: write-var gs#89240 <= s_136_1
        fn_state.gs_89240 = s_136_1;
        // N s_136_3: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #102552u : u32
        let s_137_0: u32 = 102552;
        // D s_137_1: read-reg s_137_0:struct
        let s_137_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_137_0 as isize);
            tracer.read_register(s_137_0 as isize, value);
            value
        };
        // D s_137_2: call _get_HCR_EL2_Type_E2H(s_137_1)
        let s_137_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_137_1);
        // C s_137_3: const #102552u : u32
        let s_137_3: u32 = 102552;
        // D s_137_4: read-reg s_137_3:struct
        let s_137_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_137_3 as isize);
            tracer.read_register(s_137_3 as isize, value);
            value
        };
        // D s_137_5: call _get_HCR_EL2_Type_TGE(s_137_4)
        let s_137_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_137_4);
        // D s_137_6: cast zx s_137_2 -> bv
        let s_137_6: Bits = Bits::new(s_137_2 as u128, 1u16);
        // D s_137_7: cast zx s_137_5 -> bv
        let s_137_7: Bits = Bits::new(s_137_5 as u128, 1u16);
        // D s_137_8: cast reint s_137_6 -> u128
        let s_137_8: u128 = (s_137_6.value() as u128);
        // D s_137_9: size-of s_137_6
        let s_137_9: u16 = s_137_6.length();
        // D s_137_10: cast reint s_137_7 -> u128
        let s_137_10: u128 = (s_137_7.value() as u128);
        // D s_137_11: size-of s_137_7
        let s_137_11: u16 = s_137_7.length();
        // D s_137_12: lsl s_137_8 s_137_11
        let s_137_12: u128 = s_137_8 << s_137_11;
        // D s_137_13: or s_137_12 s_137_10
        let s_137_13: u128 = ((s_137_12) | (s_137_10));
        // D s_137_14: add s_137_9 s_137_11
        let s_137_14: u16 = (s_137_9 + s_137_11);
        // D s_137_15: create-bits s_137_13 s_137_14
        let s_137_15: Bits = Bits::new(s_137_13, s_137_14);
        // D s_137_16: cast reint s_137_15 -> u8
        let s_137_16: u8 = (s_137_15.value() as u8);
        // D s_137_17: cast zx s_137_16 -> bv
        let s_137_17: Bits = Bits::new(s_137_16 as u128, 2u16);
        // C s_137_18: const #3u : u8
        let s_137_18: u8 = 3;
        // C s_137_19: cast zx s_137_18 -> bv
        let s_137_19: Bits = Bits::new(s_137_18 as u128, 2u16);
        // D s_137_20: cmp-ne s_137_17 s_137_19
        let s_137_20: bool = ((s_137_17) != (s_137_19));
        // D s_137_21: write-var gs#89239 <= s_137_20
        fn_state.gs_89239 = s_137_20;
        // N s_137_22: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #() : ()
        let s_138_0: () = ();
        // S s_138_1: call EL2Enabled(s_138_0)
        let s_138_1: bool = EL2Enabled(state, tracer, s_138_0);
        // N s_138_2: branch s_138_1 b143 b139
        if s_138_1 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #0u : u8
        let s_139_0: bool = false;
        // D s_139_1: write-var gs#89251 <= s_139_0
        fn_state.gs_89251 = s_139_0;
        // N s_139_2: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var gs#89251:u8
        let s_140_0: bool = fn_state.gs_89251;
        // N s_140_1: branch s_140_0 b142 b141
        if s_140_0 {
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
        // C s_141_5: const #440u : u32
        let s_141_5: u32 = 440;
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
        // C s_142_0: const #24u : u8
        let s_142_0: u8 = 24;
        // C s_142_1: cast zx s_142_0 -> bv
        let s_142_1: Bits = Bits::new(s_142_0 as u128, 8u16);
        // C s_142_2: cast zx s_142_1 -> i
        let s_142_2: i128 = (s_142_1.value() as i128);
        // C s_142_3: cast reint s_142_2 -> i64
        let s_142_3: i64 = (s_142_2 as i64);
        // C s_142_4: cast zx s_142_3 -> i
        let s_142_4: i128 = (i128::try_from(s_142_3).unwrap());
        // C s_142_5: const #432u : u32
        let s_142_5: u32 = 432;
        // D s_142_6: read-reg s_142_5:u8
        let s_142_6: u8 = {
            let value = state.read_register::<u8>(s_142_5 as isize);
            tracer.read_register(s_142_5 as isize, value);
            value
        };
        // D s_142_7: call AArch64_SystemAccessTrap(s_142_6, s_142_4)
        let s_142_7: () = AArch64_SystemAccessTrap(state, tracer, s_142_6, s_142_4);
        // N s_142_8: return
        return;
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var __HCR_EL2_TGE:u8
        let s_143_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_143_1: cast zx s_143_0 -> bv
        let s_143_1: Bits = Bits::new(s_143_0 as u128, 1u16);
        // C s_143_2: const #1u : u8
        let s_143_2: bool = true;
        // C s_143_3: cast zx s_143_2 -> bv
        let s_143_3: Bits = Bits::new(s_143_2 as u128, 1u16);
        // D s_143_4: cmp-eq s_143_1 s_143_3
        let s_143_4: bool = ((s_143_1) == (s_143_3));
        // D s_143_5: write-var gs#89251 <= s_143_4
        fn_state.gs_89251 = s_143_4;
        // N s_143_6: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var __PMUSERENR_EL0_EN:u8
        let s_144_0: bool = fn_state.u__PMUSERENR_EL0_EN;
        // D s_144_1: cast zx s_144_0 -> bv
        let s_144_1: Bits = Bits::new(s_144_0 as u128, 1u16);
        // C s_144_2: const #0u : u8
        let s_144_2: bool = false;
        // C s_144_3: cast zx s_144_2 -> bv
        let s_144_3: Bits = Bits::new(s_144_2 as u128, 1u16);
        // D s_144_4: cmp-eq s_144_1 s_144_3
        let s_144_4: bool = ((s_144_1) == (s_144_3));
        // D s_144_5: write-var gs#89237 <= s_144_4
        fn_state.gs_89237 = s_144_4;
        // N s_144_6: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #1u : u8
        let s_145_0: bool = true;
        // D s_145_1: write-var gs#89238 <= s_145_0
        fn_state.gs_89238 = s_145_0;
        // N s_145_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #102624u : u32
        let s_146_0: u32 = 102624;
        // D s_146_1: read-reg s_146_0:struct
        let s_146_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_146_0 as isize);
            tracer.read_register(s_146_0 as isize, value);
            value
        };
        // D s_146_2: call _get_PMUSERENR_EL0_Type_UEN(s_146_1)
        let s_146_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_146_1);
        // C s_146_3: const #102624u : u32
        let s_146_3: u32 = 102624;
        // D s_146_4: read-reg s_146_3:struct
        let s_146_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_146_3 as isize);
            tracer.read_register(s_146_3 as isize, value);
            value
        };
        // D s_146_5: call _get_PMUSERENR_EL0_Type_EN(s_146_4)
        let s_146_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_146_4);
        // D s_146_6: cast zx s_146_2 -> bv
        let s_146_6: Bits = Bits::new(s_146_2 as u128, 1u16);
        // D s_146_7: cast zx s_146_5 -> bv
        let s_146_7: Bits = Bits::new(s_146_5 as u128, 1u16);
        // D s_146_8: cast reint s_146_6 -> u128
        let s_146_8: u128 = (s_146_6.value() as u128);
        // D s_146_9: size-of s_146_6
        let s_146_9: u16 = s_146_6.length();
        // D s_146_10: cast reint s_146_7 -> u128
        let s_146_10: u128 = (s_146_7.value() as u128);
        // D s_146_11: size-of s_146_7
        let s_146_11: u16 = s_146_7.length();
        // D s_146_12: lsl s_146_8 s_146_11
        let s_146_12: u128 = s_146_8 << s_146_11;
        // D s_146_13: or s_146_12 s_146_10
        let s_146_13: u128 = ((s_146_12) | (s_146_10));
        // D s_146_14: add s_146_9 s_146_11
        let s_146_14: u16 = (s_146_9 + s_146_11);
        // D s_146_15: create-bits s_146_13 s_146_14
        let s_146_15: Bits = Bits::new(s_146_13, s_146_14);
        // D s_146_16: cast reint s_146_15 -> u8
        let s_146_16: u8 = (s_146_15.value() as u8);
        // D s_146_17: cast zx s_146_16 -> bv
        let s_146_17: Bits = Bits::new(s_146_16 as u128, 2u16);
        // C s_146_18: const #0u : u8
        let s_146_18: u8 = 0;
        // C s_146_19: cast zx s_146_18 -> bv
        let s_146_19: Bits = Bits::new(s_146_18 as u128, 2u16);
        // D s_146_20: cmp-eq s_146_17 s_146_19
        let s_146_20: bool = ((s_146_17) == (s_146_19));
        // D s_146_21: write-var gs#89236 <= s_146_20
        fn_state.gs_89236 = s_146_20;
        // N s_146_22: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_147_0: panic
        panic!("{:?}", ());
        // N s_147_1: return
        return;
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var __MDCR_EL3_TPM:u8
        let s_148_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_148_1: cast zx s_148_0 -> bv
        let s_148_1: Bits = Bits::new(s_148_0 as u128, 1u16);
        // C s_148_2: const #1u : u8
        let s_148_2: bool = true;
        // C s_148_3: cast zx s_148_2 -> bv
        let s_148_3: Bits = Bits::new(s_148_2 as u128, 1u16);
        // D s_148_4: cmp-eq s_148_1 s_148_3
        let s_148_4: bool = ((s_148_1) == (s_148_3));
        // D s_148_5: write-var gs#89235 <= s_148_4
        fn_state.gs_89235 = s_148_4;
        // N s_148_6: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_149_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_149_1: call __IMPDEF_boolean(s_149_0)
        let s_149_1: bool = u__IMPDEF_boolean(state, tracer, s_149_0);
        // D s_149_2: write-var gs#89234 <= s_149_1
        fn_state.gs_89234 = s_149_1;
        // N s_149_3: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var __EDSCR_SDD:u8
        let s_150_0: bool = fn_state.u__EDSCR_SDD;
        // D s_150_1: cast zx s_150_0 -> bv
        let s_150_1: Bits = Bits::new(s_150_0 as u128, 1u16);
        // C s_150_2: const #1u : u8
        let s_150_2: bool = true;
        // C s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 1u16);
        // D s_150_4: cmp-eq s_150_1 s_150_3
        let s_150_4: bool = ((s_150_1) == (s_150_3));
        // D s_150_5: write-var gs#89233 <= s_150_4
        fn_state.gs_89233 = s_150_4;
        // N s_150_6: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #424u : u32
        let s_151_0: u32 = 424;
        // D s_151_1: read-reg s_151_0:u8
        let s_151_1: u8 = {
            let value = state.read_register::<u8>(s_151_0 as isize);
            tracer.read_register(s_151_0 as isize, value);
            value
        };
        // C s_151_2: const #2u : u8
        let s_151_2: u8 = 2;
        // D s_151_3: cmp-lt s_151_1 s_151_2
        let s_151_3: bool = ((s_151_1) < (s_151_2));
        // D s_151_4: write-var gs#89232 <= s_151_3
        fn_state.gs_89232 = s_151_3;
        // N s_151_5: jump b84
        return block_84(state, tracer, fn_state);
    }
}
