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
use u_get_HDFGWTR_EL2_Type_PMCR_EL0::*;
use u_get_HCR_EL2_Type_E2H::*;
use Halted::*;
use IsFeatureImplemented::*;
use X_read::*;
use u_get_MDCR_EL2_Type_TPMCR::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u__set_PMCR_EL0::*;
use u_get_MDCR_EL3_Type_TPM::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use Mk_PMCR_EL0_Type::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use EDSCR_read::*;
use common::*;
pub fn PMCR_EL0_SysRegWrite_2f94c2f617db2cff<T: Tracer>(
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
        gs_87559: bool,
        u__HDFGWTR_EL2_PMCR_EL0: bool,
        gs_87544: bool,
        u__MDCR_EL2_TPMCR: bool,
        gs_87543: bool,
        gs_87564: bool,
        gs_87545: bool,
        gs_87565: bool,
        gs_87573: bool,
        u__MDCR_EL3_TPM: bool,
        gs_87568: bool,
        gs_87561: bool,
        gs_87551: bool,
        gs_87552: bool,
        gs_87539: bool,
        gs_87570: bool,
        gs_87538: bool,
        gs_87547: bool,
        gs_87553: bool,
        u__PSTATE_EL: u8,
        gs_87555: bool,
        gs_87542: bool,
        u__MDCR_EL2_TPM: bool,
        gs_87558: bool,
        gs_87536: bool,
        gs_87567: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        gs_87572: bool,
        gs_87566: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_87540: bool,
        u__PMUSERENR_EL0_EN: bool,
        gs_87546: bool,
        gs_87550: bool,
        gs_87563: bool,
        gs_87557: bool,
        gs_87549: bool,
        gs_87560: bool,
        gs_87556: bool,
        gs_87569: bool,
        gs_87562: bool,
        gs_87537: bool,
        gs_87548: bool,
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
        // D s_0_25: call _get_HDFGWTR_EL2_Type_PMCR_EL0(s_0_24)
        let s_0_25: bool = u_get_HDFGWTR_EL2_Type_PMCR_EL0(state, tracer, s_0_24);
        // D s_0_26: write-var __HDFGWTR_EL2_PMCR_EL0 <= s_0_25
        fn_state.u__HDFGWTR_EL2_PMCR_EL0 = s_0_25;
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
        // C s_0_31: const #104880u : u32
        let s_0_31: u32 = 104880;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_MDCR_EL2_Type_TPMCR(s_0_32)
        let s_0_33: bool = u_get_MDCR_EL2_Type_TPMCR(state, tracer, s_0_32);
        // D s_0_34: write-var __MDCR_EL2_TPMCR <= s_0_33
        fn_state.u__MDCR_EL2_TPMCR = s_0_33;
        // D s_0_35: read-var __PSTATE_EL:u8
        let s_0_35: u8 = fn_state.u__PSTATE_EL;
        // D s_0_36: cast zx s_0_35 -> bv
        let s_0_36: Bits = Bits::new(s_0_35 as u128, 2u16);
        // C s_0_37: const #448u : u32
        let s_0_37: u32 = 448;
        // D s_0_38: read-reg s_0_37:u8
        let s_0_38: u8 = {
            let value = state.read_register::<u8>(s_0_37 as isize);
            tracer.read_register(s_0_37 as isize, value);
            value
        };
        // D s_0_39: cast zx s_0_38 -> bv
        let s_0_39: Bits = Bits::new(s_0_38 as u128, 2u16);
        // D s_0_40: cmp-eq s_0_36 s_0_39
        let s_0_40: bool = ((s_0_36) == (s_0_39));
        // N s_0_41: branch s_0_40 b80 b1
        if s_0_40 {
            return block_80(state, tracer, fn_state);
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
        // D s_5_4: call Mk_PMCR_EL0_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_PMCR_EL0_Type(state, tracer, s_5_3);
        // D s_5_5: call __set_PMCR_EL0(s_5_4)
        let s_5_5: () = u__set_PMCR_EL0(state, tracer, s_5_4);
        // N s_5_6: return
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
        // D s_7_1: write-var gs#87536 <= s_7_0
        fn_state.gs_87536 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#87536:u8
        let s_8_0: bool = fn_state.gs_87536;
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
        // D s_9_1: write-var gs#87537 <= s_9_0
        fn_state.gs_87537 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#87537:u8
        let s_10_0: bool = fn_state.gs_87537;
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
        // D s_11_1: write-var gs#87538 <= s_11_0
        fn_state.gs_87538 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#87538:u8
        let s_12_0: bool = fn_state.gs_87538;
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
        // D s_13_1: write-var gs#87539 <= s_13_0
        fn_state.gs_87539 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#87539:u8
        let s_14_0: bool = fn_state.gs_87539;
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
        // D s_16_1: write-var gs#87540 <= s_16_0
        fn_state.gs_87540 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#87540:u8
        let s_17_0: bool = fn_state.gs_87540;
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
        // D s_18_4: call Mk_PMCR_EL0_Type(s_18_3)
        let s_18_4: ProductType5c790c8ef59cc8b2 = Mk_PMCR_EL0_Type(
            state,
            tracer,
            s_18_3,
        );
        // D s_18_5: call __set_PMCR_EL0(s_18_4)
        let s_18_5: () = u__set_PMCR_EL0(state, tracer, s_18_4);
        // N s_18_6: return
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
        // D s_20_1: write-var gs#87542 <= s_20_0
        fn_state.gs_87542 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#87542:u8
        let s_21_0: bool = fn_state.gs_87542;
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
        // D s_24_5: write-var gs#87542 <= s_24_4
        fn_state.gs_87542 = s_24_4;
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
        // D s_25_5: write-var gs#87540 <= s_25_4
        fn_state.gs_87540 = s_25_4;
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
        // D s_27_5: write-var gs#87539 <= s_27_4
        fn_state.gs_87539 = s_27_4;
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
        // D s_28_2: write-var gs#87538 <= s_28_1
        fn_state.gs_87538 = s_28_1;
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
        // D s_29_5: write-var gs#87537 <= s_29_4
        fn_state.gs_87537 = s_29_4;
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
        // D s_30_4: write-var gs#87536 <= s_30_3
        fn_state.gs_87536 = s_30_3;
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
        // N s_31_2: branch s_31_1 b79 b32
        if s_31_1 {
            return block_79(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#87543 <= s_32_0
        fn_state.gs_87543 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#87543:u8
        let s_33_0: bool = fn_state.gs_87543;
        // N s_33_1: branch s_33_0 b78 b34
        if s_33_0 {
            return block_78(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#87544 <= s_34_0
        fn_state.gs_87544 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#87544:u8
        let s_35_0: bool = fn_state.gs_87544;
        // N s_35_1: branch s_35_0 b77 b36
        if s_35_0 {
            return block_77(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#87545 <= s_36_0
        fn_state.gs_87545 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#87545:u8
        let s_37_0: bool = fn_state.gs_87545;
        // N s_37_1: branch s_37_0 b76 b38
        if s_37_0 {
            return block_76(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#87546 <= s_38_0
        fn_state.gs_87546 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#87546:u8
        let s_39_0: bool = fn_state.gs_87546;
        // N s_39_1: branch s_39_0 b75 b40
        if s_39_0 {
            return block_75(state, tracer, fn_state);
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
        // N s_40_2: branch s_40_1 b74 b41
        if s_40_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#87547 <= s_41_0
        fn_state.gs_87547 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#87547:u8
        let s_42_0: bool = fn_state.gs_87547;
        // N s_42_1: branch s_42_0 b70 b43
        if s_42_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#87549 <= s_43_0
        fn_state.gs_87549 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#87549:u8
        let s_44_0: bool = fn_state.gs_87549;
        // N s_44_1: branch s_44_0 b69 b45
        if s_44_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#87550 <= s_45_0
        fn_state.gs_87550 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#87550:u8
        let s_46_0: bool = fn_state.gs_87550;
        // N s_46_1: branch s_46_0 b68 b47
        if s_46_0 {
            return block_68(state, tracer, fn_state);
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
        // N s_47_2: branch s_47_1 b67 b48
        if s_47_1 {
            return block_67(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#87551 <= s_48_0
        fn_state.gs_87551 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#87551:u8
        let s_49_0: bool = fn_state.gs_87551;
        // N s_49_1: branch s_49_0 b66 b50
        if s_49_0 {
            return block_66(state, tracer, fn_state);
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
        // N s_50_2: branch s_50_1 b65 b51
        if s_50_1 {
            return block_65(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#87552 <= s_51_0
        fn_state.gs_87552 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#87552:u8
        let s_52_0: bool = fn_state.gs_87552;
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
        // D s_54_1: write-var gs#87553 <= s_54_0
        fn_state.gs_87553 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#87553:u8
        let s_55_0: bool = fn_state.gs_87553;
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
        // D s_56_4: call Mk_PMCR_EL0_Type(s_56_3)
        let s_56_4: ProductType5c790c8ef59cc8b2 = Mk_PMCR_EL0_Type(
            state,
            tracer,
            s_56_3,
        );
        // D s_56_5: call __set_PMCR_EL0(s_56_4)
        let s_56_5: () = u__set_PMCR_EL0(state, tracer, s_56_4);
        // N s_56_6: return
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
        // D s_58_1: write-var gs#87555 <= s_58_0
        fn_state.gs_87555 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#87555:u8
        let s_59_0: bool = fn_state.gs_87555;
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
        // D s_62_5: write-var gs#87555 <= s_62_4
        fn_state.gs_87555 = s_62_4;
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
        // D s_63_5: write-var gs#87553 <= s_63_4
        fn_state.gs_87553 = s_63_4;
        // N s_63_6: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #24u : u8
        let s_64_0: u8 = 24;
        // C s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 8u16);
        // C s_64_2: cast zx s_64_1 -> i
        let s_64_2: i128 = (s_64_1.value() as i128);
        // C s_64_3: cast reint s_64_2 -> i64
        let s_64_3: i64 = (s_64_2 as i64);
        // C s_64_4: cast zx s_64_3 -> i
        let s_64_4: i128 = (i128::try_from(s_64_3).unwrap());
        // C s_64_5: const #432u : u32
        let s_64_5: u32 = 432;
        // D s_64_6: read-reg s_64_5:u8
        let s_64_6: u8 = {
            let value = state.read_register::<u8>(s_64_5 as isize);
            tracer.read_register(s_64_5 as isize, value);
            value
        };
        // D s_64_7: call AArch64_SystemAccessTrap(s_64_6, s_64_4)
        let s_64_7: () = AArch64_SystemAccessTrap(state, tracer, s_64_6, s_64_4);
        // N s_64_8: return
        return;
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var __MDCR_EL2_TPMCR:u8
        let s_65_0: bool = fn_state.u__MDCR_EL2_TPMCR;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #1u : u8
        let s_65_2: bool = true;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#87552 <= s_65_4
        fn_state.gs_87552 = s_65_4;
        // N s_65_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #24u : u8
        let s_66_0: u8 = 24;
        // C s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 8u16);
        // C s_66_2: cast zx s_66_1 -> i
        let s_66_2: i128 = (s_66_1.value() as i128);
        // C s_66_3: cast reint s_66_2 -> i64
        let s_66_3: i64 = (s_66_2 as i64);
        // C s_66_4: cast zx s_66_3 -> i
        let s_66_4: i128 = (i128::try_from(s_66_3).unwrap());
        // C s_66_5: const #432u : u32
        let s_66_5: u32 = 432;
        // D s_66_6: read-reg s_66_5:u8
        let s_66_6: u8 = {
            let value = state.read_register::<u8>(s_66_5 as isize);
            tracer.read_register(s_66_5 as isize, value);
            value
        };
        // D s_66_7: call AArch64_SystemAccessTrap(s_66_6, s_66_4)
        let s_66_7: () = AArch64_SystemAccessTrap(state, tracer, s_66_6, s_66_4);
        // N s_66_8: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __MDCR_EL2_TPM:u8
        let s_67_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#87551 <= s_67_4
        fn_state.gs_87551 = s_67_4;
        // N s_67_6: jump b49
        return block_49(state, tracer, fn_state);
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
        // D s_69_0: read-var __HDFGWTR_EL2_PMCR_EL0:u8
        let s_69_0: bool = fn_state.u__HDFGWTR_EL2_PMCR_EL0;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#87550 <= s_69_4
        fn_state.gs_87550 = s_69_4;
        // N s_69_6: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #424u : u32
        let s_70_0: u32 = 424;
        // D s_70_1: read-reg s_70_0:u8
        let s_70_1: u8 = {
            let value = state.read_register::<u8>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // C s_70_2: const #2u : u8
        let s_70_2: u8 = 2;
        // D s_70_3: cmp-lt s_70_1 s_70_2
        let s_70_3: bool = ((s_70_1) < (s_70_2));
        // D s_70_4: not s_70_3
        let s_70_4: bool = !s_70_3;
        // N s_70_5: branch s_70_4 b73 b71
        if s_70_4 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __SCR_EL3_FGTEn:u8
        let s_71_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #1u : u8
        let s_71_2: bool = true;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#87548 <= s_71_4
        fn_state.gs_87548 = s_71_4;
        // N s_71_6: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#87548:u8
        let s_72_0: bool = fn_state.gs_87548;
        // D s_72_1: write-var gs#87549 <= s_72_0
        fn_state.gs_87549 = s_72_0;
        // N s_72_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var gs#87548 <= s_73_0
        fn_state.gs_87548 = s_73_0;
        // N s_73_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #146u : u32
        let s_74_0: u32 = 146;
        // S s_74_1: call IsFeatureImplemented(s_74_0)
        let s_74_1: bool = IsFeatureImplemented(state, tracer, s_74_0);
        // D s_74_2: write-var gs#87547 <= s_74_1
        fn_state.gs_87547 = s_74_1;
        // N s_74_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_75_0: panic
        panic!("{:?}", ());
        // N s_75_1: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var __MDCR_EL3_TPM:u8
        let s_76_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 1u16);
        // C s_76_2: const #1u : u8
        let s_76_2: bool = true;
        // C s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_4: cmp-eq s_76_1 s_76_3
        let s_76_4: bool = ((s_76_1) == (s_76_3));
        // D s_76_5: write-var gs#87546 <= s_76_4
        fn_state.gs_87546 = s_76_4;
        // N s_76_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_77_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_77_1: call __IMPDEF_boolean(s_77_0)
        let s_77_1: bool = u__IMPDEF_boolean(state, tracer, s_77_0);
        // D s_77_2: write-var gs#87545 <= s_77_1
        fn_state.gs_87545 = s_77_1;
        // N s_77_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var __EDSCR_SDD:u8
        let s_78_0: bool = fn_state.u__EDSCR_SDD;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #1u : u8
        let s_78_2: bool = true;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#87544 <= s_78_4
        fn_state.gs_87544 = s_78_4;
        // N s_78_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #424u : u32
        let s_79_0: u32 = 424;
        // D s_79_1: read-reg s_79_0:u8
        let s_79_1: u8 = {
            let value = state.read_register::<u8>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // C s_79_2: const #2u : u8
        let s_79_2: u8 = 2;
        // D s_79_3: cmp-lt s_79_1 s_79_2
        let s_79_3: bool = ((s_79_1) < (s_79_2));
        // D s_79_4: write-var gs#87543 <= s_79_3
        fn_state.gs_87543 = s_79_3;
        // N s_79_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call Halted(s_80_0)
        let s_80_1: bool = Halted(state, tracer, s_80_0);
        // N s_80_2: branch s_80_1 b147 b81
        if s_80_1 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#87556 <= s_81_0
        fn_state.gs_87556 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#87556:u8
        let s_82_0: bool = fn_state.gs_87556;
        // N s_82_1: branch s_82_0 b146 b83
        if s_82_0 {
            return block_146(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#87557 <= s_83_0
        fn_state.gs_87557 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#87557:u8
        let s_84_0: bool = fn_state.gs_87557;
        // N s_84_1: branch s_84_0 b145 b85
        if s_84_0 {
            return block_145(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#87558 <= s_85_0
        fn_state.gs_87558 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#87558:u8
        let s_86_0: bool = fn_state.gs_87558;
        // N s_86_1: branch s_86_0 b144 b87
        if s_86_0 {
            return block_144(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#87559 <= s_87_0
        fn_state.gs_87559 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#87559:u8
        let s_88_0: bool = fn_state.gs_87559;
        // N s_88_1: branch s_88_0 b143 b89
        if s_88_0 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #204u : u32
        let s_89_0: u32 = 204;
        // S s_89_1: call IsFeatureImplemented(s_89_0)
        let s_89_1: bool = IsFeatureImplemented(state, tracer, s_89_0);
        // N s_89_2: branch s_89_1 b142 b90
        if s_89_1 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#87560 <= s_90_0
        fn_state.gs_87560 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#87560:u8
        let s_91_0: bool = fn_state.gs_87560;
        // N s_91_1: branch s_91_0 b141 b92
        if s_91_0 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #204u : u32
        let s_92_0: u32 = 204;
        // S s_92_1: call IsFeatureImplemented(s_92_0)
        let s_92_1: bool = IsFeatureImplemented(state, tracer, s_92_0);
        // S s_92_2: not s_92_1
        let s_92_2: bool = !s_92_1;
        // N s_92_3: branch s_92_2 b140 b93
        if s_92_2 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var gs#87561 <= s_93_0
        fn_state.gs_87561 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#87561:u8
        let s_94_0: bool = fn_state.gs_87561;
        // D s_94_1: write-var gs#87562 <= s_94_0
        fn_state.gs_87562 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#87562:u8
        let s_95_0: bool = fn_state.gs_87562;
        // N s_95_1: branch s_95_0 b134 b96
        if s_95_0 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #() : ()
        let s_96_0: () = ();
        // S s_96_1: call EL2Enabled(s_96_0)
        let s_96_1: bool = EL2Enabled(state, tracer, s_96_0);
        // N s_96_2: branch s_96_1 b133 b97
        if s_96_1 {
            return block_133(state, tracer, fn_state);
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
        // D s_97_1: write-var gs#87563 <= s_97_0
        fn_state.gs_87563 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#87563:u8
        let s_98_0: bool = fn_state.gs_87563;
        // N s_98_1: branch s_98_0 b132 b99
        if s_98_0 {
            return block_132(state, tracer, fn_state);
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
        // D s_99_1: write-var gs#87564 <= s_99_0
        fn_state.gs_87564 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#87564:u8
        let s_100_0: bool = fn_state.gs_87564;
        // N s_100_1: branch s_100_0 b128 b101
        if s_100_0 {
            return block_128(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#87566 <= s_101_0
        fn_state.gs_87566 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#87566:u8
        let s_102_0: bool = fn_state.gs_87566;
        // N s_102_1: branch s_102_0 b127 b103
        if s_102_0 {
            return block_127(state, tracer, fn_state);
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
        // D s_103_1: write-var gs#87567 <= s_103_0
        fn_state.gs_87567 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#87567:u8
        let s_104_0: bool = fn_state.gs_87567;
        // N s_104_1: branch s_104_0 b126 b105
        if s_104_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #() : ()
        let s_105_0: () = ();
        // S s_105_1: call EL2Enabled(s_105_0)
        let s_105_1: bool = EL2Enabled(state, tracer, s_105_0);
        // N s_105_2: branch s_105_1 b125 b106
        if s_105_1 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #0u : u8
        let s_106_0: bool = false;
        // D s_106_1: write-var gs#87568 <= s_106_0
        fn_state.gs_87568 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#87568:u8
        let s_107_0: bool = fn_state.gs_87568;
        // N s_107_1: branch s_107_0 b124 b108
        if s_107_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #() : ()
        let s_108_0: () = ();
        // S s_108_1: call EL2Enabled(s_108_0)
        let s_108_1: bool = EL2Enabled(state, tracer, s_108_0);
        // N s_108_2: branch s_108_1 b123 b109
        if s_108_1 {
            return block_123(state, tracer, fn_state);
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
        // D s_109_1: write-var gs#87569 <= s_109_0
        fn_state.gs_87569 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#87569:u8
        let s_110_0: bool = fn_state.gs_87569;
        // N s_110_1: branch s_110_0 b122 b111
        if s_110_0 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #424u : u32
        let s_111_0: u32 = 424;
        // D s_111_1: read-reg s_111_0:u8
        let s_111_1: u8 = {
            let value = state.read_register::<u8>(s_111_0 as isize);
            tracer.read_register(s_111_0 as isize, value);
            value
        };
        // C s_111_2: const #2u : u8
        let s_111_2: u8 = 2;
        // D s_111_3: cmp-lt s_111_1 s_111_2
        let s_111_3: bool = ((s_111_1) < (s_111_2));
        // N s_111_4: branch s_111_3 b121 b112
        if s_111_3 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // D s_112_1: write-var gs#87570 <= s_112_0
        fn_state.gs_87570 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#87570:u8
        let s_113_0: bool = fn_state.gs_87570;
        // N s_113_1: branch s_113_0 b115 b114
        if s_113_0 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #64s : i64
        let s_114_0: i64 = 64;
        // D s_114_1: read-var t:i
        let s_114_1: i128 = fn_state.t;
        // D s_114_2: call X_read(s_114_1, s_114_0)
        let s_114_2: Bits = X_read(state, tracer, s_114_1, s_114_0);
        // D s_114_3: cast reint s_114_2 -> u64
        let s_114_3: u64 = (s_114_2.value() as u64);
        // D s_114_4: call Mk_PMCR_EL0_Type(s_114_3)
        let s_114_4: ProductType5c790c8ef59cc8b2 = Mk_PMCR_EL0_Type(
            state,
            tracer,
            s_114_3,
        );
        // D s_114_5: call __set_PMCR_EL0(s_114_4)
        let s_114_5: () = u__set_PMCR_EL0(state, tracer, s_114_4);
        // N s_114_6: return
        return;
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #() : ()
        let s_115_0: () = ();
        // S s_115_1: call Halted(s_115_0)
        let s_115_1: bool = Halted(state, tracer, s_115_0);
        // N s_115_2: branch s_115_1 b120 b116
        if s_115_1 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #0u : u8
        let s_116_0: bool = false;
        // D s_116_1: write-var gs#87572 <= s_116_0
        fn_state.gs_87572 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#87572:u8
        let s_117_0: bool = fn_state.gs_87572;
        // N s_117_1: branch s_117_0 b119 b118
        if s_117_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #24u : u8
        let s_118_0: u8 = 24;
        // C s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 8u16);
        // C s_118_2: cast zx s_118_1 -> i
        let s_118_2: i128 = (s_118_1.value() as i128);
        // C s_118_3: cast reint s_118_2 -> i64
        let s_118_3: i64 = (s_118_2 as i64);
        // C s_118_4: cast zx s_118_3 -> i
        let s_118_4: i128 = (i128::try_from(s_118_3).unwrap());
        // C s_118_5: const #424u : u32
        let s_118_5: u32 = 424;
        // D s_118_6: read-reg s_118_5:u8
        let s_118_6: u8 = {
            let value = state.read_register::<u8>(s_118_5 as isize);
            tracer.read_register(s_118_5 as isize, value);
            value
        };
        // D s_118_7: call AArch64_SystemAccessTrap(s_118_6, s_118_4)
        let s_118_7: () = AArch64_SystemAccessTrap(state, tracer, s_118_6, s_118_4);
        // N s_118_8: return
        return;
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_119_0: panic
        panic!("{:?}", ());
        // N s_119_1: return
        return;
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var __EDSCR_SDD:u8
        let s_120_0: bool = fn_state.u__EDSCR_SDD;
        // D s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 1u16);
        // C s_120_2: const #1u : u8
        let s_120_2: bool = true;
        // C s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 1u16);
        // D s_120_4: cmp-eq s_120_1 s_120_3
        let s_120_4: bool = ((s_120_1) == (s_120_3));
        // D s_120_5: write-var gs#87572 <= s_120_4
        fn_state.gs_87572 = s_120_4;
        // N s_120_6: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var __MDCR_EL3_TPM:u8
        let s_121_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 1u16);
        // C s_121_2: const #1u : u8
        let s_121_2: bool = true;
        // C s_121_3: cast zx s_121_2 -> bv
        let s_121_3: Bits = Bits::new(s_121_2 as u128, 1u16);
        // D s_121_4: cmp-eq s_121_1 s_121_3
        let s_121_4: bool = ((s_121_1) == (s_121_3));
        // D s_121_5: write-var gs#87570 <= s_121_4
        fn_state.gs_87570 = s_121_4;
        // N s_121_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #24u : u8
        let s_122_0: u8 = 24;
        // C s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 8u16);
        // C s_122_2: cast zx s_122_1 -> i
        let s_122_2: i128 = (s_122_1.value() as i128);
        // C s_122_3: cast reint s_122_2 -> i64
        let s_122_3: i64 = (s_122_2 as i64);
        // C s_122_4: cast zx s_122_3 -> i
        let s_122_4: i128 = (i128::try_from(s_122_3).unwrap());
        // C s_122_5: const #432u : u32
        let s_122_5: u32 = 432;
        // D s_122_6: read-reg s_122_5:u8
        let s_122_6: u8 = {
            let value = state.read_register::<u8>(s_122_5 as isize);
            tracer.read_register(s_122_5 as isize, value);
            value
        };
        // D s_122_7: call AArch64_SystemAccessTrap(s_122_6, s_122_4)
        let s_122_7: () = AArch64_SystemAccessTrap(state, tracer, s_122_6, s_122_4);
        // N s_122_8: return
        return;
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var __MDCR_EL2_TPMCR:u8
        let s_123_0: bool = fn_state.u__MDCR_EL2_TPMCR;
        // D s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 1u16);
        // C s_123_2: const #1u : u8
        let s_123_2: bool = true;
        // C s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 1u16);
        // D s_123_4: cmp-eq s_123_1 s_123_3
        let s_123_4: bool = ((s_123_1) == (s_123_3));
        // D s_123_5: write-var gs#87569 <= s_123_4
        fn_state.gs_87569 = s_123_4;
        // N s_123_6: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #24u : u8
        let s_124_0: u8 = 24;
        // C s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 8u16);
        // C s_124_2: cast zx s_124_1 -> i
        let s_124_2: i128 = (s_124_1.value() as i128);
        // C s_124_3: cast reint s_124_2 -> i64
        let s_124_3: i64 = (s_124_2 as i64);
        // C s_124_4: cast zx s_124_3 -> i
        let s_124_4: i128 = (i128::try_from(s_124_3).unwrap());
        // C s_124_5: const #432u : u32
        let s_124_5: u32 = 432;
        // D s_124_6: read-reg s_124_5:u8
        let s_124_6: u8 = {
            let value = state.read_register::<u8>(s_124_5 as isize);
            tracer.read_register(s_124_5 as isize, value);
            value
        };
        // D s_124_7: call AArch64_SystemAccessTrap(s_124_6, s_124_4)
        let s_124_7: () = AArch64_SystemAccessTrap(state, tracer, s_124_6, s_124_4);
        // N s_124_8: return
        return;
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var __MDCR_EL2_TPM:u8
        let s_125_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_125_1: cast zx s_125_0 -> bv
        let s_125_1: Bits = Bits::new(s_125_0 as u128, 1u16);
        // C s_125_2: const #1u : u8
        let s_125_2: bool = true;
        // C s_125_3: cast zx s_125_2 -> bv
        let s_125_3: Bits = Bits::new(s_125_2 as u128, 1u16);
        // D s_125_4: cmp-eq s_125_1 s_125_3
        let s_125_4: bool = ((s_125_1) == (s_125_3));
        // D s_125_5: write-var gs#87568 <= s_125_4
        fn_state.gs_87568 = s_125_4;
        // N s_125_6: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #24u : u8
        let s_126_0: u8 = 24;
        // C s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 8u16);
        // C s_126_2: cast zx s_126_1 -> i
        let s_126_2: i128 = (s_126_1.value() as i128);
        // C s_126_3: cast reint s_126_2 -> i64
        let s_126_3: i64 = (s_126_2 as i64);
        // C s_126_4: cast zx s_126_3 -> i
        let s_126_4: i128 = (i128::try_from(s_126_3).unwrap());
        // C s_126_5: const #432u : u32
        let s_126_5: u32 = 432;
        // D s_126_6: read-reg s_126_5:u8
        let s_126_6: u8 = {
            let value = state.read_register::<u8>(s_126_5 as isize);
            tracer.read_register(s_126_5 as isize, value);
            value
        };
        // D s_126_7: call AArch64_SystemAccessTrap(s_126_6, s_126_4)
        let s_126_7: () = AArch64_SystemAccessTrap(state, tracer, s_126_6, s_126_4);
        // N s_126_8: return
        return;
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var __HDFGWTR_EL2_PMCR_EL0:u8
        let s_127_0: bool = fn_state.u__HDFGWTR_EL2_PMCR_EL0;
        // D s_127_1: cast zx s_127_0 -> bv
        let s_127_1: Bits = Bits::new(s_127_0 as u128, 1u16);
        // C s_127_2: const #1u : u8
        let s_127_2: bool = true;
        // C s_127_3: cast zx s_127_2 -> bv
        let s_127_3: Bits = Bits::new(s_127_2 as u128, 1u16);
        // D s_127_4: cmp-eq s_127_1 s_127_3
        let s_127_4: bool = ((s_127_1) == (s_127_3));
        // D s_127_5: write-var gs#87567 <= s_127_4
        fn_state.gs_87567 = s_127_4;
        // N s_127_6: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #424u : u32
        let s_128_0: u32 = 424;
        // D s_128_1: read-reg s_128_0:u8
        let s_128_1: u8 = {
            let value = state.read_register::<u8>(s_128_0 as isize);
            tracer.read_register(s_128_0 as isize, value);
            value
        };
        // C s_128_2: const #2u : u8
        let s_128_2: u8 = 2;
        // D s_128_3: cmp-lt s_128_1 s_128_2
        let s_128_3: bool = ((s_128_1) < (s_128_2));
        // D s_128_4: not s_128_3
        let s_128_4: bool = !s_128_3;
        // N s_128_5: branch s_128_4 b131 b129
        if s_128_4 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var __SCR_EL3_FGTEn:u8
        let s_129_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 1u16);
        // C s_129_2: const #1u : u8
        let s_129_2: bool = true;
        // C s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 1u16);
        // D s_129_4: cmp-eq s_129_1 s_129_3
        let s_129_4: bool = ((s_129_1) == (s_129_3));
        // D s_129_5: write-var gs#87565 <= s_129_4
        fn_state.gs_87565 = s_129_4;
        // N s_129_6: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#87565:u8
        let s_130_0: bool = fn_state.gs_87565;
        // D s_130_1: write-var gs#87566 <= s_130_0
        fn_state.gs_87566 = s_130_0;
        // N s_130_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #1u : u8
        let s_131_0: bool = true;
        // D s_131_1: write-var gs#87565 <= s_131_0
        fn_state.gs_87565 = s_131_0;
        // N s_131_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #146u : u32
        let s_132_0: u32 = 146;
        // S s_132_1: call IsFeatureImplemented(s_132_0)
        let s_132_1: bool = IsFeatureImplemented(state, tracer, s_132_0);
        // D s_132_2: write-var gs#87564 <= s_132_1
        fn_state.gs_87564 = s_132_1;
        // N s_132_3: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #102552u : u32
        let s_133_0: u32 = 102552;
        // D s_133_1: read-reg s_133_0:struct
        let s_133_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_133_0 as isize);
            tracer.read_register(s_133_0 as isize, value);
            value
        };
        // D s_133_2: call _get_HCR_EL2_Type_E2H(s_133_1)
        let s_133_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_133_1);
        // C s_133_3: const #102552u : u32
        let s_133_3: u32 = 102552;
        // D s_133_4: read-reg s_133_3:struct
        let s_133_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_133_3 as isize);
            tracer.read_register(s_133_3 as isize, value);
            value
        };
        // D s_133_5: call _get_HCR_EL2_Type_TGE(s_133_4)
        let s_133_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_133_4);
        // D s_133_6: cast zx s_133_2 -> bv
        let s_133_6: Bits = Bits::new(s_133_2 as u128, 1u16);
        // D s_133_7: cast zx s_133_5 -> bv
        let s_133_7: Bits = Bits::new(s_133_5 as u128, 1u16);
        // D s_133_8: cast reint s_133_6 -> u128
        let s_133_8: u128 = (s_133_6.value() as u128);
        // D s_133_9: size-of s_133_6
        let s_133_9: u16 = s_133_6.length();
        // D s_133_10: cast reint s_133_7 -> u128
        let s_133_10: u128 = (s_133_7.value() as u128);
        // D s_133_11: size-of s_133_7
        let s_133_11: u16 = s_133_7.length();
        // D s_133_12: lsl s_133_8 s_133_11
        let s_133_12: u128 = s_133_8 << s_133_11;
        // D s_133_13: or s_133_12 s_133_10
        let s_133_13: u128 = ((s_133_12) | (s_133_10));
        // D s_133_14: add s_133_9 s_133_11
        let s_133_14: u16 = (s_133_9 + s_133_11);
        // D s_133_15: create-bits s_133_13 s_133_14
        let s_133_15: Bits = Bits::new(s_133_13, s_133_14);
        // D s_133_16: cast reint s_133_15 -> u8
        let s_133_16: u8 = (s_133_15.value() as u8);
        // D s_133_17: cast zx s_133_16 -> bv
        let s_133_17: Bits = Bits::new(s_133_16 as u128, 2u16);
        // C s_133_18: const #3u : u8
        let s_133_18: u8 = 3;
        // C s_133_19: cast zx s_133_18 -> bv
        let s_133_19: Bits = Bits::new(s_133_18 as u128, 2u16);
        // D s_133_20: cmp-ne s_133_17 s_133_19
        let s_133_20: bool = ((s_133_17) != (s_133_19));
        // D s_133_21: write-var gs#87563 <= s_133_20
        fn_state.gs_87563 = s_133_20;
        // N s_133_22: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #() : ()
        let s_134_0: () = ();
        // S s_134_1: call EL2Enabled(s_134_0)
        let s_134_1: bool = EL2Enabled(state, tracer, s_134_0);
        // N s_134_2: branch s_134_1 b139 b135
        if s_134_1 {
            return block_139(state, tracer, fn_state);
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
        // D s_135_1: write-var gs#87573 <= s_135_0
        fn_state.gs_87573 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#87573:u8
        let s_136_0: bool = fn_state.gs_87573;
        // N s_136_1: branch s_136_0 b138 b137
        if s_136_0 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #24u : u8
        let s_137_0: u8 = 24;
        // C s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 8u16);
        // C s_137_2: cast zx s_137_1 -> i
        let s_137_2: i128 = (s_137_1.value() as i128);
        // C s_137_3: cast reint s_137_2 -> i64
        let s_137_3: i64 = (s_137_2 as i64);
        // C s_137_4: cast zx s_137_3 -> i
        let s_137_4: i128 = (i128::try_from(s_137_3).unwrap());
        // C s_137_5: const #440u : u32
        let s_137_5: u32 = 440;
        // D s_137_6: read-reg s_137_5:u8
        let s_137_6: u8 = {
            let value = state.read_register::<u8>(s_137_5 as isize);
            tracer.read_register(s_137_5 as isize, value);
            value
        };
        // D s_137_7: call AArch64_SystemAccessTrap(s_137_6, s_137_4)
        let s_137_7: () = AArch64_SystemAccessTrap(state, tracer, s_137_6, s_137_4);
        // N s_137_8: return
        return;
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #24u : u8
        let s_138_0: u8 = 24;
        // C s_138_1: cast zx s_138_0 -> bv
        let s_138_1: Bits = Bits::new(s_138_0 as u128, 8u16);
        // C s_138_2: cast zx s_138_1 -> i
        let s_138_2: i128 = (s_138_1.value() as i128);
        // C s_138_3: cast reint s_138_2 -> i64
        let s_138_3: i64 = (s_138_2 as i64);
        // C s_138_4: cast zx s_138_3 -> i
        let s_138_4: i128 = (i128::try_from(s_138_3).unwrap());
        // C s_138_5: const #432u : u32
        let s_138_5: u32 = 432;
        // D s_138_6: read-reg s_138_5:u8
        let s_138_6: u8 = {
            let value = state.read_register::<u8>(s_138_5 as isize);
            tracer.read_register(s_138_5 as isize, value);
            value
        };
        // D s_138_7: call AArch64_SystemAccessTrap(s_138_6, s_138_4)
        let s_138_7: () = AArch64_SystemAccessTrap(state, tracer, s_138_6, s_138_4);
        // N s_138_8: return
        return;
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var __HCR_EL2_TGE:u8
        let s_139_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_139_1: cast zx s_139_0 -> bv
        let s_139_1: Bits = Bits::new(s_139_0 as u128, 1u16);
        // C s_139_2: const #1u : u8
        let s_139_2: bool = true;
        // C s_139_3: cast zx s_139_2 -> bv
        let s_139_3: Bits = Bits::new(s_139_2 as u128, 1u16);
        // D s_139_4: cmp-eq s_139_1 s_139_3
        let s_139_4: bool = ((s_139_1) == (s_139_3));
        // D s_139_5: write-var gs#87573 <= s_139_4
        fn_state.gs_87573 = s_139_4;
        // N s_139_6: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var __PMUSERENR_EL0_EN:u8
        let s_140_0: bool = fn_state.u__PMUSERENR_EL0_EN;
        // D s_140_1: cast zx s_140_0 -> bv
        let s_140_1: Bits = Bits::new(s_140_0 as u128, 1u16);
        // C s_140_2: const #0u : u8
        let s_140_2: bool = false;
        // C s_140_3: cast zx s_140_2 -> bv
        let s_140_3: Bits = Bits::new(s_140_2 as u128, 1u16);
        // D s_140_4: cmp-eq s_140_1 s_140_3
        let s_140_4: bool = ((s_140_1) == (s_140_3));
        // D s_140_5: write-var gs#87561 <= s_140_4
        fn_state.gs_87561 = s_140_4;
        // N s_140_6: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #1u : u8
        let s_141_0: bool = true;
        // D s_141_1: write-var gs#87562 <= s_141_0
        fn_state.gs_87562 = s_141_0;
        // N s_141_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #102624u : u32
        let s_142_0: u32 = 102624;
        // D s_142_1: read-reg s_142_0:struct
        let s_142_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_142_0 as isize);
            tracer.read_register(s_142_0 as isize, value);
            value
        };
        // D s_142_2: call _get_PMUSERENR_EL0_Type_UEN(s_142_1)
        let s_142_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_142_1);
        // C s_142_3: const #102624u : u32
        let s_142_3: u32 = 102624;
        // D s_142_4: read-reg s_142_3:struct
        let s_142_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_142_3 as isize);
            tracer.read_register(s_142_3 as isize, value);
            value
        };
        // D s_142_5: call _get_PMUSERENR_EL0_Type_EN(s_142_4)
        let s_142_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_142_4);
        // D s_142_6: cast zx s_142_2 -> bv
        let s_142_6: Bits = Bits::new(s_142_2 as u128, 1u16);
        // D s_142_7: cast zx s_142_5 -> bv
        let s_142_7: Bits = Bits::new(s_142_5 as u128, 1u16);
        // D s_142_8: cast reint s_142_6 -> u128
        let s_142_8: u128 = (s_142_6.value() as u128);
        // D s_142_9: size-of s_142_6
        let s_142_9: u16 = s_142_6.length();
        // D s_142_10: cast reint s_142_7 -> u128
        let s_142_10: u128 = (s_142_7.value() as u128);
        // D s_142_11: size-of s_142_7
        let s_142_11: u16 = s_142_7.length();
        // D s_142_12: lsl s_142_8 s_142_11
        let s_142_12: u128 = s_142_8 << s_142_11;
        // D s_142_13: or s_142_12 s_142_10
        let s_142_13: u128 = ((s_142_12) | (s_142_10));
        // D s_142_14: add s_142_9 s_142_11
        let s_142_14: u16 = (s_142_9 + s_142_11);
        // D s_142_15: create-bits s_142_13 s_142_14
        let s_142_15: Bits = Bits::new(s_142_13, s_142_14);
        // D s_142_16: cast reint s_142_15 -> u8
        let s_142_16: u8 = (s_142_15.value() as u8);
        // D s_142_17: cast zx s_142_16 -> bv
        let s_142_17: Bits = Bits::new(s_142_16 as u128, 2u16);
        // C s_142_18: const #1u : u8
        let s_142_18: u8 = 1;
        // C s_142_19: cast zx s_142_18 -> bv
        let s_142_19: Bits = Bits::new(s_142_18 as u128, 2u16);
        // D s_142_20: cmp-ne s_142_17 s_142_19
        let s_142_20: bool = ((s_142_17) != (s_142_19));
        // D s_142_21: write-var gs#87560 <= s_142_20
        fn_state.gs_87560 = s_142_20;
        // N s_142_22: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_143_0: panic
        panic!("{:?}", ());
        // N s_143_1: return
        return;
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var __MDCR_EL3_TPM:u8
        let s_144_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_144_1: cast zx s_144_0 -> bv
        let s_144_1: Bits = Bits::new(s_144_0 as u128, 1u16);
        // C s_144_2: const #1u : u8
        let s_144_2: bool = true;
        // C s_144_3: cast zx s_144_2 -> bv
        let s_144_3: Bits = Bits::new(s_144_2 as u128, 1u16);
        // D s_144_4: cmp-eq s_144_1 s_144_3
        let s_144_4: bool = ((s_144_1) == (s_144_3));
        // D s_144_5: write-var gs#87559 <= s_144_4
        fn_state.gs_87559 = s_144_4;
        // N s_144_6: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_145_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_145_1: call __IMPDEF_boolean(s_145_0)
        let s_145_1: bool = u__IMPDEF_boolean(state, tracer, s_145_0);
        // D s_145_2: write-var gs#87558 <= s_145_1
        fn_state.gs_87558 = s_145_1;
        // N s_145_3: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var __EDSCR_SDD:u8
        let s_146_0: bool = fn_state.u__EDSCR_SDD;
        // D s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 1u16);
        // C s_146_2: const #1u : u8
        let s_146_2: bool = true;
        // C s_146_3: cast zx s_146_2 -> bv
        let s_146_3: Bits = Bits::new(s_146_2 as u128, 1u16);
        // D s_146_4: cmp-eq s_146_1 s_146_3
        let s_146_4: bool = ((s_146_1) == (s_146_3));
        // D s_146_5: write-var gs#87557 <= s_146_4
        fn_state.gs_87557 = s_146_4;
        // N s_146_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #424u : u32
        let s_147_0: u32 = 424;
        // D s_147_1: read-reg s_147_0:u8
        let s_147_1: u8 = {
            let value = state.read_register::<u8>(s_147_0 as isize);
            tracer.read_register(s_147_0 as isize, value);
            value
        };
        // C s_147_2: const #2u : u8
        let s_147_2: u8 = 2;
        // D s_147_3: cmp-lt s_147_1 s_147_2
        let s_147_3: bool = ((s_147_1) < (s_147_2));
        // D s_147_4: write-var gs#87556 <= s_147_3
        fn_state.gs_87556 = s_147_3;
        // N s_147_5: jump b82
        return block_82(state, tracer, fn_state);
    }
}
