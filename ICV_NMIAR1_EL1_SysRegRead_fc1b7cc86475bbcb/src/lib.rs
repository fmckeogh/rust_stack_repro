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
use EL2Enabled::*;
use u__get_ICC_NMIAR1_EL1::*;
use u_get_SCTLR_EL3_Type_NMI::*;
use Halted::*;
use u_get_SCR_EL3_Type_IRQ::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use ICC_SRE_EL1_read::*;
use X_set::*;
use u_get_ICH_HCR_EL2_Type_TALL1::*;
use u_get_SCTLR_EL2_Type_NMI::*;
use u_get_ICC_SRE_EL1_Type_SRE::*;
use u_get_ICC_SRE_EL2_Type_SRE::*;
use u__get_ICV_NMIAR1_EL1::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_EL2_Type_IMO::*;
use u_get_ICC_SRE_EL3_Type_SRE::*;
use EDSCR_read::*;
use u_get_SCTLR_EL1_Type_NMI::*;
use common::*;
pub fn ICV_NMIAR1_EL1_SysRegRead_fc1b7cc86475bbcb<T: Tracer>(
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
        u__SCTLR_EL1_NMI: bool,
        u__SCR_EL3_IRQ: bool,
        ga_69090: ProductType5c790c8ef59cc8b2,
        gs_62683: bool,
        u__EDSCR_SDD: bool,
        u__HCR_EL2_IMO: bool,
        u__ICC_SRE_EL1_SRE: bool,
        u__SCTLR_EL3_NMI: bool,
        ga_69081: ProductType5c790c8ef59cc8b2,
        gs_62687: bool,
        ga_69118: ProductType5c790c8ef59cc8b2,
        u__ICC_SRE_EL3_SRE: bool,
        u__ICH_HCR_EL2_TALL1: bool,
        gs_62689: bool,
        gs_62692: bool,
        u__ICC_SRE_EL2_SRE: bool,
        u__SCTLR_EL2_NMI: bool,
        gs_62691: bool,
        gs_62690: bool,
        gs_62694: bool,
        gs_62684: bool,
        gs_62681: bool,
        gs_62685: bool,
        gs_62682: bool,
        gs_62693: bool,
        gs_62696: bool,
        ga_69110: ProductType5c790c8ef59cc8b2,
        u__PSTATE_EL: u8,
        gs_62688: bool,
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
        // C s_0_3: const #90272u : u32
        let s_0_3: u32 = 90272;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCTLR_EL1_Type_NMI(s_0_4)
        let s_0_5: bool = u_get_SCTLR_EL1_Type_NMI(state, tracer, s_0_4);
        // D s_0_6: write-var __SCTLR_EL1_NMI <= s_0_5
        fn_state.u__SCTLR_EL1_NMI = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call EDSCR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_EDSCR_Type_SDD(s_0_8)
        let s_0_9: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_8);
        // D s_0_10: write-var __EDSCR_SDD <= s_0_9
        fn_state.u__EDSCR_SDD = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_IRQ(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_IRQ <= s_0_13
        fn_state.u__SCR_EL3_IRQ = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call ICC_SRE_EL1_read(s_0_15)
        let s_0_16: ProductType5c790c8ef59cc8b2 = ICC_SRE_EL1_read(
            state,
            tracer,
            s_0_15,
        );
        // S s_0_17: call _get_ICC_SRE_EL1_Type_SRE(s_0_16)
        let s_0_17: bool = u_get_ICC_SRE_EL1_Type_SRE(state, tracer, s_0_16);
        // D s_0_18: write-var __ICC_SRE_EL1_SRE <= s_0_17
        fn_state.u__ICC_SRE_EL1_SRE = s_0_17;
        // C s_0_19: const #20992u : u32
        let s_0_19: u32 = 20992;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_ICH_HCR_EL2_Type_TALL1(s_0_20)
        let s_0_21: bool = u_get_ICH_HCR_EL2_Type_TALL1(state, tracer, s_0_20);
        // D s_0_22: write-var __ICH_HCR_EL2_TALL1 <= s_0_21
        fn_state.u__ICH_HCR_EL2_TALL1 = s_0_21;
        // C s_0_23: const #102552u : u32
        let s_0_23: u32 = 102552;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HCR_EL2_Type_IMO(s_0_24)
        let s_0_25: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_0_24);
        // D s_0_26: write-var __HCR_EL2_IMO <= s_0_25
        fn_state.u__HCR_EL2_IMO = s_0_25;
        // C s_0_27: const #20784u : u32
        let s_0_27: u32 = 20784;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_SCTLR_EL2_Type_NMI(s_0_28)
        let s_0_29: bool = u_get_SCTLR_EL2_Type_NMI(state, tracer, s_0_28);
        // D s_0_30: write-var __SCTLR_EL2_NMI <= s_0_29
        fn_state.u__SCTLR_EL2_NMI = s_0_29;
        // C s_0_31: const #16368u : u32
        let s_0_31: u32 = 16368;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_ICC_SRE_EL2_Type_SRE(s_0_32)
        let s_0_33: bool = u_get_ICC_SRE_EL2_Type_SRE(state, tracer, s_0_32);
        // D s_0_34: write-var __ICC_SRE_EL2_SRE <= s_0_33
        fn_state.u__ICC_SRE_EL2_SRE = s_0_33;
        // C s_0_35: const #17072u : u32
        let s_0_35: u32 = 17072;
        // D s_0_36: read-reg s_0_35:struct
        let s_0_36: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_35 as isize);
            tracer.read_register(s_0_35 as isize, value);
            value
        };
        // D s_0_37: call _get_SCTLR_EL3_Type_NMI(s_0_36)
        let s_0_37: bool = u_get_SCTLR_EL3_Type_NMI(state, tracer, s_0_36);
        // D s_0_38: write-var __SCTLR_EL3_NMI <= s_0_37
        fn_state.u__SCTLR_EL3_NMI = s_0_37;
        // C s_0_39: const #10200u : u32
        let s_0_39: u32 = 10200;
        // D s_0_40: read-reg s_0_39:struct
        let s_0_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_39 as isize);
            tracer.read_register(s_0_39 as isize, value);
            value
        };
        // D s_0_41: call _get_ICC_SRE_EL3_Type_SRE(s_0_40)
        let s_0_41: bool = u_get_ICC_SRE_EL3_Type_SRE(state, tracer, s_0_40);
        // D s_0_42: write-var __ICC_SRE_EL3_SRE <= s_0_41
        fn_state.u__ICC_SRE_EL3_SRE = s_0_41;
        // D s_0_43: read-var __PSTATE_EL:u8
        let s_0_43: u8 = fn_state.u__PSTATE_EL;
        // D s_0_44: cast zx s_0_43 -> bv
        let s_0_44: Bits = Bits::new(s_0_43 as u128, 2u16);
        // C s_0_45: const #448u : u32
        let s_0_45: u32 = 448;
        // D s_0_46: read-reg s_0_45:u8
        let s_0_46: u8 = {
            let value = state.read_register::<u8>(s_0_45 as isize);
            tracer.read_register(s_0_45 as isize, value);
            value
        };
        // D s_0_47: cast zx s_0_46 -> bv
        let s_0_47: Bits = Bits::new(s_0_46 as u128, 2u16);
        // D s_0_48: cmp-eq s_0_44 s_0_47
        let s_0_48: bool = ((s_0_44) == (s_0_47));
        // N s_0_49: branch s_0_48 b78 b1
        if s_0_48 {
            return block_78(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b39 b2
        if s_1_5 {
            return block_39(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b10 b3
        if s_2_5 {
            return block_10(state, tracer, fn_state);
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
        // D s_5_0: read-var __SCTLR_EL3_NMI:u8
        let s_5_0: bool = fn_state.u__SCTLR_EL3_NMI;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b9 b6
        if s_5_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __ICC_SRE_EL3_SRE:u8
        let s_6_0: bool = fn_state.u__ICC_SRE_EL3_SRE;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #0u : u8
        let s_6_2: bool = false;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
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
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #11728u : u32
        let s_7_1: u32 = 11728;
        // D s_7_2: read-reg s_7_1:struct
        let s_7_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_1 as isize);
            tracer.read_register(s_7_1 as isize, value);
            value
        };
        // D s_7_3: call __get_ICC_NMIAR1_EL1(s_7_2)
        let s_7_3: ProductType5c790c8ef59cc8b2 = u__get_ICC_NMIAR1_EL1(
            state,
            tracer,
            s_7_2,
        );
        // D s_7_4: write-var ga#69118 <= s_7_3
        fn_state.ga_69118 = s_7_3;
        // D s_7_5: read-var ga#69118.0:struct
        let s_7_5: u64 = fn_state.ga_69118._0;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 64u16);
        // D s_7_7: read-var t:i
        let s_7_7: i128 = fn_state.t;
        // D s_7_8: call X_set(s_7_7, s_7_0, s_7_6)
        let s_7_8: () = X_set(state, tracer, s_7_7, s_7_0, s_7_6);
        // N s_7_9: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #24u : u8
        let s_8_0: u8 = 24;
        // C s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 8u16);
        // C s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (s_8_1.value() as i128);
        // C s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #424u : u32
        let s_8_5: u32 = 424;
        // D s_8_6: read-reg s_8_5:u8
        let s_8_6: u8 = {
            let value = state.read_register::<u8>(s_8_5 as isize);
            tracer.read_register(s_8_5 as isize, value);
            value
        };
        // D s_8_7: call AArch64_SystemAccessTrap(s_8_6, s_8_4)
        let s_8_7: () = AArch64_SystemAccessTrap(state, tracer, s_8_6, s_8_4);
        // N s_8_8: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var __SCTLR_EL2_NMI:u8
        let s_10_0: bool = fn_state.u__SCTLR_EL2_NMI;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #0u : u8
        let s_10_2: bool = false;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b38 b11
        if s_10_4 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
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
        // D s_12_1: write-var gs#62681 <= s_12_0
        fn_state.gs_62681 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#62681:u8
        let s_13_0: bool = fn_state.gs_62681;
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
        // D s_14_1: write-var gs#62682 <= s_14_0
        fn_state.gs_62682 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#62682:u8
        let s_15_0: bool = fn_state.gs_62682;
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
        // D s_16_1: write-var gs#62683 <= s_16_0
        fn_state.gs_62683 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#62683:u8
        let s_17_0: bool = fn_state.gs_62683;
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
        // D s_18_1: write-var gs#62684 <= s_18_0
        fn_state.gs_62684 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#62684:u8
        let s_19_0: bool = fn_state.gs_62684;
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
        // D s_20_0: read-var __ICC_SRE_EL2_SRE:u8
        let s_20_0: bool = fn_state.u__ICC_SRE_EL2_SRE;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 1u16);
        // C s_20_2: const #0u : u8
        let s_20_2: bool = false;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // N s_20_5: branch s_20_4 b32 b21
        if s_20_4 {
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
        // C s_21_0: const #424u : u32
        let s_21_0: u32 = 424;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: u8 = {
            let value = state.read_register::<u8>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // C s_21_2: const #2u : u8
        let s_21_2: u8 = 2;
        // D s_21_3: cmp-lt s_21_1 s_21_2
        let s_21_3: bool = ((s_21_1) < (s_21_2));
        // N s_21_4: branch s_21_3 b31 b22
        if s_21_3 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#62685 <= s_22_0
        fn_state.gs_62685 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#62685:u8
        let s_23_0: bool = fn_state.gs_62685;
        // N s_23_1: branch s_23_0 b25 b24
        if s_23_0 {
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
        // C s_24_1: const #11728u : u32
        let s_24_1: u32 = 11728;
        // D s_24_2: read-reg s_24_1:struct
        let s_24_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_1 as isize);
            tracer.read_register(s_24_1 as isize, value);
            value
        };
        // D s_24_3: call __get_ICC_NMIAR1_EL1(s_24_2)
        let s_24_3: ProductType5c790c8ef59cc8b2 = u__get_ICC_NMIAR1_EL1(
            state,
            tracer,
            s_24_2,
        );
        // D s_24_4: write-var ga#69110 <= s_24_3
        fn_state.ga_69110 = s_24_3;
        // D s_24_5: read-var ga#69110.0:struct
        let s_24_5: u64 = fn_state.ga_69110._0;
        // D s_24_6: cast zx s_24_5 -> bv
        let s_24_6: Bits = Bits::new(s_24_5 as u128, 64u16);
        // D s_24_7: read-var t:i
        let s_24_7: i128 = fn_state.t;
        // D s_24_8: call X_set(s_24_7, s_24_0, s_24_6)
        let s_24_8: () = X_set(state, tracer, s_24_7, s_24_0, s_24_6);
        // N s_24_9: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call Halted(s_25_0)
        let s_25_1: bool = Halted(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b30 b26
        if s_25_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#62687 <= s_26_0
        fn_state.gs_62687 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#62687:u8
        let s_27_0: bool = fn_state.gs_62687;
        // N s_27_1: branch s_27_0 b29 b28
        if s_27_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #24u : u8
        let s_28_0: u8 = 24;
        // C s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 8u16);
        // C s_28_2: cast zx s_28_1 -> i
        let s_28_2: i128 = (s_28_1.value() as i128);
        // C s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // C s_28_5: const #424u : u32
        let s_28_5: u32 = 424;
        // D s_28_6: read-reg s_28_5:u8
        let s_28_6: u8 = {
            let value = state.read_register::<u8>(s_28_5 as isize);
            tracer.read_register(s_28_5 as isize, value);
            value
        };
        // D s_28_7: call AArch64_SystemAccessTrap(s_28_6, s_28_4)
        let s_28_7: () = AArch64_SystemAccessTrap(state, tracer, s_28_6, s_28_4);
        // N s_28_8: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: panic
        panic!("{:?}", ());
        // N s_29_1: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __EDSCR_SDD:u8
        let s_30_0: bool = fn_state.u__EDSCR_SDD;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#62687 <= s_30_4
        fn_state.gs_62687 = s_30_4;
        // N s_30_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var __SCR_EL3_IRQ:u8
        let s_31_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#62685 <= s_31_4
        fn_state.gs_62685 = s_31_4;
        // N s_31_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #24u : u8
        let s_32_0: u8 = 24;
        // C s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 8u16);
        // C s_32_2: cast zx s_32_1 -> i
        let s_32_2: i128 = (s_32_1.value() as i128);
        // C s_32_3: cast reint s_32_2 -> i64
        let s_32_3: i64 = (s_32_2 as i64);
        // C s_32_4: cast zx s_32_3 -> i
        let s_32_4: i128 = (i128::try_from(s_32_3).unwrap());
        // C s_32_5: const #432u : u32
        let s_32_5: u32 = 432;
        // D s_32_6: read-reg s_32_5:u8
        let s_32_6: u8 = {
            let value = state.read_register::<u8>(s_32_5 as isize);
            tracer.read_register(s_32_5 as isize, value);
            value
        };
        // D s_32_7: call AArch64_SystemAccessTrap(s_32_6, s_32_4)
        let s_32_7: () = AArch64_SystemAccessTrap(state, tracer, s_32_6, s_32_4);
        // N s_32_8: return
        return;
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
        // D s_34_0: read-var __SCR_EL3_IRQ:u8
        let s_34_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: write-var gs#62684 <= s_34_4
        fn_state.gs_62684 = s_34_4;
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
        // D s_35_2: write-var gs#62683 <= s_35_1
        fn_state.gs_62683 = s_35_1;
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
        // D s_36_5: write-var gs#62682 <= s_36_4
        fn_state.gs_62682 = s_36_4;
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
        // D s_37_4: write-var gs#62681 <= s_37_3
        fn_state.gs_62681 = s_37_3;
        // N s_37_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: panic
        panic!("{:?}", ());
        // N s_38_1: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var __SCTLR_EL1_NMI:u8
        let s_39_0: bool = fn_state.u__SCTLR_EL1_NMI;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #0u : u8
        let s_39_2: bool = false;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // N s_39_5: branch s_39_4 b77 b40
        if s_39_4 {
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
        // S s_40_1: call Halted(s_40_0)
        let s_40_1: bool = Halted(state, tracer, s_40_0);
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
        // D s_41_1: write-var gs#62688 <= s_41_0
        fn_state.gs_62688 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#62688:u8
        let s_42_0: bool = fn_state.gs_62688;
        // N s_42_1: branch s_42_0 b75 b43
        if s_42_0 {
            return block_75(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#62689 <= s_43_0
        fn_state.gs_62689 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#62689:u8
        let s_44_0: bool = fn_state.gs_62689;
        // N s_44_1: branch s_44_0 b74 b45
        if s_44_0 {
            return block_74(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#62690 <= s_45_0
        fn_state.gs_62690 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#62690:u8
        let s_46_0: bool = fn_state.gs_62690;
        // N s_46_1: branch s_46_0 b73 b47
        if s_46_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#62691 <= s_47_0
        fn_state.gs_62691 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#62691:u8
        let s_48_0: bool = fn_state.gs_62691;
        // N s_48_1: branch s_48_0 b72 b49
        if s_48_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var __ICC_SRE_EL1_SRE:u8
        let s_49_0: bool = fn_state.u__ICC_SRE_EL1_SRE;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 1u16);
        // C s_49_2: const #0u : u8
        let s_49_2: bool = false;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // N s_49_5: branch s_49_4 b71 b50
        if s_49_4 {
            return block_71(state, tracer, fn_state);
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
        // N s_50_2: branch s_50_1 b70 b51
        if s_50_1 {
            return block_70(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#62692 <= s_51_0
        fn_state.gs_62692 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#62692:u8
        let s_52_0: bool = fn_state.gs_62692;
        // N s_52_1: branch s_52_0 b69 b53
        if s_52_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EL2Enabled(s_53_0)
        let s_53_1: bool = EL2Enabled(state, tracer, s_53_0);
        // N s_53_2: branch s_53_1 b68 b54
        if s_53_1 {
            return block_68(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#62693 <= s_54_0
        fn_state.gs_62693 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#62693:u8
        let s_55_0: bool = fn_state.gs_62693;
        // N s_55_1: branch s_55_0 b67 b56
        if s_55_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #424u : u32
        let s_56_0: u32 = 424;
        // D s_56_1: read-reg s_56_0:u8
        let s_56_1: u8 = {
            let value = state.read_register::<u8>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // C s_56_2: const #2u : u8
        let s_56_2: u8 = 2;
        // D s_56_3: cmp-lt s_56_1 s_56_2
        let s_56_3: bool = ((s_56_1) < (s_56_2));
        // N s_56_4: branch s_56_3 b66 b57
        if s_56_3 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#62694 <= s_57_0
        fn_state.gs_62694 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#62694:u8
        let s_58_0: bool = fn_state.gs_62694;
        // N s_58_1: branch s_58_0 b60 b59
        if s_58_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #64s : i64
        let s_59_0: i64 = 64;
        // C s_59_1: const #11728u : u32
        let s_59_1: u32 = 11728;
        // D s_59_2: read-reg s_59_1:struct
        let s_59_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_1 as isize);
            tracer.read_register(s_59_1 as isize, value);
            value
        };
        // D s_59_3: call __get_ICC_NMIAR1_EL1(s_59_2)
        let s_59_3: ProductType5c790c8ef59cc8b2 = u__get_ICC_NMIAR1_EL1(
            state,
            tracer,
            s_59_2,
        );
        // D s_59_4: write-var ga#69090 <= s_59_3
        fn_state.ga_69090 = s_59_3;
        // D s_59_5: read-var ga#69090.0:struct
        let s_59_5: u64 = fn_state.ga_69090._0;
        // D s_59_6: cast zx s_59_5 -> bv
        let s_59_6: Bits = Bits::new(s_59_5 as u128, 64u16);
        // D s_59_7: read-var t:i
        let s_59_7: i128 = fn_state.t;
        // D s_59_8: call X_set(s_59_7, s_59_0, s_59_6)
        let s_59_8: () = X_set(state, tracer, s_59_7, s_59_0, s_59_6);
        // N s_59_9: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call Halted(s_60_0)
        let s_60_1: bool = Halted(state, tracer, s_60_0);
        // N s_60_2: branch s_60_1 b65 b61
        if s_60_1 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#62696 <= s_61_0
        fn_state.gs_62696 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#62696:u8
        let s_62_0: bool = fn_state.gs_62696;
        // N s_62_1: branch s_62_0 b64 b63
        if s_62_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #24u : u8
        let s_63_0: u8 = 24;
        // C s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 8u16);
        // C s_63_2: cast zx s_63_1 -> i
        let s_63_2: i128 = (s_63_1.value() as i128);
        // C s_63_3: cast reint s_63_2 -> i64
        let s_63_3: i64 = (s_63_2 as i64);
        // C s_63_4: cast zx s_63_3 -> i
        let s_63_4: i128 = (i128::try_from(s_63_3).unwrap());
        // C s_63_5: const #424u : u32
        let s_63_5: u32 = 424;
        // D s_63_6: read-reg s_63_5:u8
        let s_63_6: u8 = {
            let value = state.read_register::<u8>(s_63_5 as isize);
            tracer.read_register(s_63_5 as isize, value);
            value
        };
        // D s_63_7: call AArch64_SystemAccessTrap(s_63_6, s_63_4)
        let s_63_7: () = AArch64_SystemAccessTrap(state, tracer, s_63_6, s_63_4);
        // N s_63_8: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_64_0: panic
        panic!("{:?}", ());
        // N s_64_1: return
        return;
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var __EDSCR_SDD:u8
        let s_65_0: bool = fn_state.u__EDSCR_SDD;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #1u : u8
        let s_65_2: bool = true;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#62696 <= s_65_4
        fn_state.gs_62696 = s_65_4;
        // N s_65_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var __SCR_EL3_IRQ:u8
        let s_66_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #1u : u8
        let s_66_2: bool = true;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#62694 <= s_66_4
        fn_state.gs_62694 = s_66_4;
        // N s_66_6: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #64s : i64
        let s_67_0: i64 = 64;
        // C s_67_1: const #90880u : u32
        let s_67_1: u32 = 90880;
        // D s_67_2: read-reg s_67_1:struct
        let s_67_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_67_1 as isize);
            tracer.read_register(s_67_1 as isize, value);
            value
        };
        // D s_67_3: call __get_ICV_NMIAR1_EL1(s_67_2)
        let s_67_3: ProductType5c790c8ef59cc8b2 = u__get_ICV_NMIAR1_EL1(
            state,
            tracer,
            s_67_2,
        );
        // D s_67_4: write-var ga#69081 <= s_67_3
        fn_state.ga_69081 = s_67_3;
        // D s_67_5: read-var ga#69081.0:struct
        let s_67_5: u64 = fn_state.ga_69081._0;
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
        // D s_68_0: read-var __HCR_EL2_IMO:u8
        let s_68_0: bool = fn_state.u__HCR_EL2_IMO;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 1u16);
        // C s_68_2: const #1u : u8
        let s_68_2: bool = true;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // D s_68_5: write-var gs#62693 <= s_68_4
        fn_state.gs_62693 = s_68_4;
        // N s_68_6: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #24u : u8
        let s_69_0: u8 = 24;
        // C s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 8u16);
        // C s_69_2: cast zx s_69_1 -> i
        let s_69_2: i128 = (s_69_1.value() as i128);
        // C s_69_3: cast reint s_69_2 -> i64
        let s_69_3: i64 = (s_69_2 as i64);
        // C s_69_4: cast zx s_69_3 -> i
        let s_69_4: i128 = (i128::try_from(s_69_3).unwrap());
        // C s_69_5: const #432u : u32
        let s_69_5: u32 = 432;
        // D s_69_6: read-reg s_69_5:u8
        let s_69_6: u8 = {
            let value = state.read_register::<u8>(s_69_5 as isize);
            tracer.read_register(s_69_5 as isize, value);
            value
        };
        // D s_69_7: call AArch64_SystemAccessTrap(s_69_6, s_69_4)
        let s_69_7: () = AArch64_SystemAccessTrap(state, tracer, s_69_6, s_69_4);
        // N s_69_8: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __ICH_HCR_EL2_TALL1:u8
        let s_70_0: bool = fn_state.u__ICH_HCR_EL2_TALL1;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#62692 <= s_70_4
        fn_state.gs_62692 = s_70_4;
        // N s_70_6: jump b52
        return block_52(state, tracer, fn_state);
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
        // C s_71_5: const #440u : u32
        let s_71_5: u32 = 440;
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
        // D s_73_0: read-var __SCR_EL3_IRQ:u8
        let s_73_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#62691 <= s_73_4
        fn_state.gs_62691 = s_73_4;
        // N s_73_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_74_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_74_1: call __IMPDEF_boolean(s_74_0)
        let s_74_1: bool = u__IMPDEF_boolean(state, tracer, s_74_0);
        // D s_74_2: write-var gs#62690 <= s_74_1
        fn_state.gs_62690 = s_74_1;
        // N s_74_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var __EDSCR_SDD:u8
        let s_75_0: bool = fn_state.u__EDSCR_SDD;
        // D s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 1u16);
        // C s_75_2: const #1u : u8
        let s_75_2: bool = true;
        // C s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // D s_75_4: cmp-eq s_75_1 s_75_3
        let s_75_4: bool = ((s_75_1) == (s_75_3));
        // D s_75_5: write-var gs#62689 <= s_75_4
        fn_state.gs_62689 = s_75_4;
        // N s_75_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #424u : u32
        let s_76_0: u32 = 424;
        // D s_76_1: read-reg s_76_0:u8
        let s_76_1: u8 = {
            let value = state.read_register::<u8>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // C s_76_2: const #2u : u8
        let s_76_2: u8 = 2;
        // D s_76_3: cmp-lt s_76_1 s_76_2
        let s_76_3: bool = ((s_76_1) < (s_76_2));
        // D s_76_4: write-var gs#62688 <= s_76_3
        fn_state.gs_62688 = s_76_3;
        // N s_76_5: jump b42
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
        // N s_78_0: panic
        panic!("{:?}", ());
        // N s_78_1: return
        return;
    }
}
