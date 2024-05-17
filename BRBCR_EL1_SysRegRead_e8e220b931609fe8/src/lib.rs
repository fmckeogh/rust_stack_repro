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
use u_get_HCR_EL2_Type_E2H::*;
use Halted::*;
use u_get_MDCR_EL3_Type_SBRBE::*;
use NVMem_read::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u__get_BRBCR_EL1::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_HCR_EL2_Type_NV1::*;
use X_set::*;
use u_get_HCR_EL2_Type_NV::*;
use ELUsingAArch32::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn BRBCR_EL1_SysRegRead_e8e220b931609fe8<T: Tracer>(
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
        gs_49845: bool,
        u__SCR_EL3_NS: bool,
        gs_49868: bool,
        u__HCR_EL2_E2H: bool,
        gs_49838: bool,
        gs_49862: bool,
        u__EDSCR_SDD: bool,
        gs_49840: bool,
        gs_49855: bool,
        gs_49843: bool,
        gs_49853: bool,
        ga_44704: ProductType5c790c8ef59cc8b2,
        gs_49867: bool,
        gs_49848: bool,
        gs_49842: bool,
        gs_49857: bool,
        gs_49844: bool,
        gs_49846: bool,
        gs_49866: bool,
        gs_49856: bool,
        u__MDCR_EL3_SBRBE: u8,
        gs_49841: bool,
        gs_49854: bool,
        ga_44695: ProductType5c790c8ef59cc8b2,
        gs_49863: bool,
        gs_49865: bool,
        u__PSTATE_EL: u8,
        gs_49837: bool,
        u__HCR_EL2_NV: bool,
        ga_44658: u64,
        gs_49847: bool,
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
        // C s_0_3: const #102552u : u32
        let s_0_3: u32 = 102552;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HCR_EL2_Type_NV(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_NV <= s_0_5
        fn_state.u__HCR_EL2_NV = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_E2H(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_E2H <= s_0_9
        fn_state.u__HCR_EL2_E2H = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call EDSCR_read(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_11);
        // S s_0_13: call _get_EDSCR_Type_SDD(s_0_12)
        let s_0_13: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_12);
        // D s_0_14: write-var __EDSCR_SDD <= s_0_13
        fn_state.u__EDSCR_SDD = s_0_13;
        // C s_0_15: const #22712u : u32
        let s_0_15: u32 = 22712;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_MDCR_EL3_Type_SBRBE(s_0_16)
        let s_0_17: u8 = u_get_MDCR_EL3_Type_SBRBE(state, tracer, s_0_16);
        // D s_0_18: write-var __MDCR_EL3_SBRBE <= s_0_17
        fn_state.u__MDCR_EL3_SBRBE = s_0_17;
        // C s_0_19: const #90704u : u32
        let s_0_19: u32 = 90704;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_SCR_EL3_Type_NS(s_0_20)
        let s_0_21: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_0_20);
        // D s_0_22: write-var __SCR_EL3_NS <= s_0_21
        fn_state.u__SCR_EL3_NS = s_0_21;
        // D s_0_23: read-var __PSTATE_EL:u8
        let s_0_23: u8 = fn_state.u__PSTATE_EL;
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 2u16);
        // C s_0_25: const #448u : u32
        let s_0_25: u32 = 448;
        // D s_0_26: read-reg s_0_25:u8
        let s_0_26: u8 = {
            let value = state.read_register::<u8>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 2u16);
        // D s_0_28: cmp-eq s_0_24 s_0_27
        let s_0_28: bool = ((s_0_24) == (s_0_27));
        // N s_0_29: branch s_0_28 b95 b1
        if s_0_28 {
            return block_95(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b83 b2
        if s_1_5 {
            return block_83(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b14 b3
        if s_2_5 {
            return block_14(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call EL2Enabled(s_5_0)
        let s_5_1: bool = EL2Enabled(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b13 b6
        if s_5_1 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#49837 <= s_6_0
        fn_state.gs_49837 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#49837:u8
        let s_7_0: bool = fn_state.gs_49837;
        // N s_7_1: branch s_7_0 b12 b8
        if s_7_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#49838 <= s_8_0
        fn_state.gs_49838 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#49838:u8
        let s_9_0: bool = fn_state.gs_49838;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // C s_11_1: const #90640u : u32
        let s_11_1: u32 = 90640;
        // D s_11_2: read-reg s_11_1:struct
        let s_11_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_1 as isize);
            tracer.read_register(s_11_1 as isize, value);
            value
        };
        // D s_11_3: call __get_BRBCR_EL1(s_11_2)
        let s_11_3: ProductType5c790c8ef59cc8b2 = u__get_BRBCR_EL1(
            state,
            tracer,
            s_11_2,
        );
        // D s_11_4: write-var ga#44704 <= s_11_3
        fn_state.ga_44704 = s_11_3;
        // D s_11_5: read-var ga#44704.0:struct
        let s_11_5: u64 = fn_state.ga_44704._0;
        // D s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 64u16);
        // D s_11_7: read-var t:i
        let s_11_7: i128 = fn_state.t;
        // D s_11_8: call X_set(s_11_7, s_11_0, s_11_6)
        let s_11_8: () = X_set(state, tracer, s_11_7, s_11_0, s_11_6);
        // N s_11_9: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var __HCR_EL2_E2H:u8
        let s_12_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#49838 <= s_12_4
        fn_state.gs_49838 = s_12_4;
        // N s_12_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #432u : u32
        let s_13_0: u32 = 432;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call ELUsingAArch32(s_13_1)
        let s_13_2: bool = ELUsingAArch32(state, tracer, s_13_1);
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // D s_13_4: write-var gs#49837 <= s_13_3
        fn_state.gs_49837 = s_13_3;
        // N s_13_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var __HCR_EL2_E2H:u8
        let s_14_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // N s_14_5: branch s_14_4 b16 b15
        if s_14_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call Halted(s_16_0)
        let s_16_1: bool = Halted(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b82 b17
        if s_16_1 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#49840 <= s_17_0
        fn_state.gs_49840 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#49840:u8
        let s_18_0: bool = fn_state.gs_49840;
        // N s_18_1: branch s_18_0 b81 b19
        if s_18_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#49841 <= s_19_0
        fn_state.gs_49841 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#49841:u8
        let s_20_0: bool = fn_state.gs_49841;
        // N s_20_1: branch s_20_0 b80 b21
        if s_20_0 {
            return block_80(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#49842 <= s_21_0
        fn_state.gs_49842 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#49842:u8
        let s_22_0: bool = fn_state.gs_49842;
        // N s_22_1: branch s_22_0 b79 b23
        if s_22_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#49843 <= s_23_0
        fn_state.gs_49843 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#49843:u8
        let s_24_0: bool = fn_state.gs_49843;
        // N s_24_1: branch s_24_0 b78 b25
        if s_24_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#49844 <= s_25_0
        fn_state.gs_49844 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#49844:u8
        let s_26_0: bool = fn_state.gs_49844;
        // N s_26_1: branch s_26_0 b77 b27
        if s_26_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call Halted(s_27_0)
        let s_27_1: bool = Halted(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b76 b28
        if s_27_1 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#49845 <= s_28_0
        fn_state.gs_49845 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#49845:u8
        let s_29_0: bool = fn_state.gs_49845;
        // N s_29_1: branch s_29_0 b75 b30
        if s_29_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#49846 <= s_30_0
        fn_state.gs_49846 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#49846:u8
        let s_31_0: bool = fn_state.gs_49846;
        // N s_31_1: branch s_31_0 b74 b32
        if s_31_0 {
            return block_74(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#49847 <= s_32_0
        fn_state.gs_49847 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#49847:u8
        let s_33_0: bool = fn_state.gs_49847;
        // N s_33_1: branch s_33_0 b70 b34
        if s_33_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#49853 <= s_34_0
        fn_state.gs_49853 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#49853:u8
        let s_35_0: bool = fn_state.gs_49853;
        // N s_35_1: branch s_35_0 b69 b36
        if s_35_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#49854 <= s_36_0
        fn_state.gs_49854 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#49854:u8
        let s_37_0: bool = fn_state.gs_49854;
        // N s_37_1: branch s_37_0 b68 b38
        if s_37_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #424u : u32
        let s_38_0: u32 = 424;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // C s_38_2: const #2u : u8
        let s_38_2: u8 = 2;
        // D s_38_3: cmp-lt s_38_1 s_38_2
        let s_38_3: bool = ((s_38_1) < (s_38_2));
        // N s_38_4: branch s_38_3 b67 b39
        if s_38_3 {
            return block_67(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#49855 <= s_39_0
        fn_state.gs_49855 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#49855:u8
        let s_40_0: bool = fn_state.gs_49855;
        // N s_40_1: branch s_40_0 b66 b41
        if s_40_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#49856 <= s_41_0
        fn_state.gs_49856 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#49856:u8
        let s_42_0: bool = fn_state.gs_49856;
        // N s_42_1: branch s_42_0 b60 b43
        if s_42_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #424u : u32
        let s_43_0: u32 = 424;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // C s_43_2: const #2u : u8
        let s_43_2: u8 = 2;
        // D s_43_3: cmp-lt s_43_1 s_43_2
        let s_43_3: bool = ((s_43_1) < (s_43_2));
        // N s_43_4: branch s_43_3 b56 b44
        if s_43_3 {
            return block_56(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#49862 <= s_44_0
        fn_state.gs_49862 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#49862:u8
        let s_45_0: bool = fn_state.gs_49862;
        // N s_45_1: branch s_45_0 b55 b46
        if s_45_0 {
            return block_55(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#49863 <= s_46_0
        fn_state.gs_49863 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#49863:u8
        let s_47_0: bool = fn_state.gs_49863;
        // N s_47_1: branch s_47_0 b49 b48
        if s_47_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #64s : i64
        let s_48_0: i64 = 64;
        // C s_48_1: const #90640u : u32
        let s_48_1: u32 = 90640;
        // D s_48_2: read-reg s_48_1:struct
        let s_48_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_48_1 as isize);
            tracer.read_register(s_48_1 as isize, value);
            value
        };
        // D s_48_3: call __get_BRBCR_EL1(s_48_2)
        let s_48_3: ProductType5c790c8ef59cc8b2 = u__get_BRBCR_EL1(
            state,
            tracer,
            s_48_2,
        );
        // D s_48_4: write-var ga#44695 <= s_48_3
        fn_state.ga_44695 = s_48_3;
        // D s_48_5: read-var ga#44695.0:struct
        let s_48_5: u64 = fn_state.ga_44695._0;
        // D s_48_6: cast zx s_48_5 -> bv
        let s_48_6: Bits = Bits::new(s_48_5 as u128, 64u16);
        // D s_48_7: read-var t:i
        let s_48_7: i128 = fn_state.t;
        // D s_48_8: call X_set(s_48_7, s_48_0, s_48_6)
        let s_48_8: () = X_set(state, tracer, s_48_7, s_48_0, s_48_6);
        // N s_48_9: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call Halted(s_49_0)
        let s_49_1: bool = Halted(state, tracer, s_49_0);
        // N s_49_2: branch s_49_1 b54 b50
        if s_49_1 {
            return block_54(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#49865 <= s_50_0
        fn_state.gs_49865 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#49865:u8
        let s_51_0: bool = fn_state.gs_49865;
        // N s_51_1: branch s_51_0 b53 b52
        if s_51_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #24u : u8
        let s_52_0: u8 = 24;
        // C s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 8u16);
        // C s_52_2: cast zx s_52_1 -> i
        let s_52_2: i128 = (s_52_1.value() as i128);
        // C s_52_3: cast reint s_52_2 -> i64
        let s_52_3: i64 = (s_52_2 as i64);
        // C s_52_4: cast zx s_52_3 -> i
        let s_52_4: i128 = (i128::try_from(s_52_3).unwrap());
        // C s_52_5: const #424u : u32
        let s_52_5: u32 = 424;
        // D s_52_6: read-reg s_52_5:u8
        let s_52_6: u8 = {
            let value = state.read_register::<u8>(s_52_5 as isize);
            tracer.read_register(s_52_5 as isize, value);
            value
        };
        // D s_52_7: call AArch64_SystemAccessTrap(s_52_6, s_52_4)
        let s_52_7: () = AArch64_SystemAccessTrap(state, tracer, s_52_6, s_52_4);
        // N s_52_8: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: panic
        panic!("{:?}", ());
        // N s_53_1: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var __EDSCR_SDD:u8
        let s_54_0: bool = fn_state.u__EDSCR_SDD;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 1u16);
        // C s_54_2: const #1u : u8
        let s_54_2: bool = true;
        // C s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 1u16);
        // D s_54_4: cmp-eq s_54_1 s_54_3
        let s_54_4: bool = ((s_54_1) == (s_54_3));
        // D s_54_5: write-var gs#49865 <= s_54_4
        fn_state.gs_49865 = s_54_4;
        // N s_54_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var __SCR_EL3_NS:u8
        let s_55_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 1u16);
        // C s_55_2: const #1u : u8
        let s_55_2: bool = true;
        // C s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // D s_55_4: cmp-eq s_55_1 s_55_3
        let s_55_4: bool = ((s_55_1) == (s_55_3));
        // D s_55_5: write-var gs#49863 <= s_55_4
        fn_state.gs_49863 = s_55_4;
        // N s_55_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __MDCR_EL3_SBRBE:u8
        let s_56_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_56_1: const #0s : i
        let s_56_1: i128 = 0;
        // D s_56_2: cast zx s_56_0 -> bv
        let s_56_2: Bits = Bits::new(s_56_0 as u128, 2u16);
        // C s_56_3: const #1s : i64
        let s_56_3: i64 = 1;
        // C s_56_4: cast zx s_56_3 -> i
        let s_56_4: i128 = (i128::try_from(s_56_3).unwrap());
        // C s_56_5: const #0s : i
        let s_56_5: i128 = 0;
        // C s_56_6: add s_56_5 s_56_4
        let s_56_6: i128 = (s_56_5 + s_56_4);
        // D s_56_7: bit-extract s_56_2 s_56_1 s_56_6
        let s_56_7: Bits = (Bits::new(
            ((s_56_2) >> (s_56_1)).value(),
            u16::try_from(s_56_6).unwrap(),
        ));
        // D s_56_8: cast reint s_56_7 -> u8
        let s_56_8: bool = ((s_56_7.value()) != 0);
        // D s_56_9: cast zx s_56_8 -> bv
        let s_56_9: Bits = Bits::new(s_56_8 as u128, 1u16);
        // C s_56_10: const #0u : u8
        let s_56_10: bool = false;
        // C s_56_11: cast zx s_56_10 -> bv
        let s_56_11: Bits = Bits::new(s_56_10 as u128, 1u16);
        // D s_56_12: cmp-eq s_56_9 s_56_11
        let s_56_12: bool = ((s_56_9) == (s_56_11));
        // D s_56_13: not s_56_12
        let s_56_13: bool = !s_56_12;
        // N s_56_14: branch s_56_13 b59 b57
        if s_56_13 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#49857 <= s_57_0
        fn_state.gs_49857 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#49857:u8
        let s_58_0: bool = fn_state.gs_49857;
        // D s_58_1: write-var gs#49862 <= s_58_0
        fn_state.gs_49862 = s_58_0;
        // N s_58_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#49857 <= s_59_0
        fn_state.gs_49857 = s_59_0;
        // N s_59_2: jump b58
        return block_58(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#49866 <= s_61_0
        fn_state.gs_49866 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#49866:u8
        let s_62_0: bool = fn_state.gs_49866;
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
        // D s_65_5: write-var gs#49866 <= s_65_4
        fn_state.gs_49866 = s_65_4;
        // N s_65_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var __SCR_EL3_NS:u8
        let s_66_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #0u : u8
        let s_66_2: bool = false;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#49856 <= s_66_4
        fn_state.gs_49856 = s_66_4;
        // N s_66_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __MDCR_EL3_SBRBE:u8
        let s_67_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 2u16);
        // C s_67_2: const #3u : u8
        let s_67_2: u8 = 3;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 2u16);
        // D s_67_4: cmp-ne s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) != (s_67_3));
        // D s_67_5: write-var gs#49855 <= s_67_4
        fn_state.gs_49855 = s_67_4;
        // N s_67_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_68_0: panic
        panic!("{:?}", ());
        // N s_68_1: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __SCR_EL3_NS:u8
        let s_69_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#49854 <= s_69_4
        fn_state.gs_49854 = s_69_4;
        // N s_69_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __MDCR_EL3_SBRBE:u8
        let s_70_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_70_1: const #0s : i
        let s_70_1: i128 = 0;
        // D s_70_2: cast zx s_70_0 -> bv
        let s_70_2: Bits = Bits::new(s_70_0 as u128, 2u16);
        // C s_70_3: const #1s : i64
        let s_70_3: i64 = 1;
        // C s_70_4: cast zx s_70_3 -> i
        let s_70_4: i128 = (i128::try_from(s_70_3).unwrap());
        // C s_70_5: const #0s : i
        let s_70_5: i128 = 0;
        // C s_70_6: add s_70_5 s_70_4
        let s_70_6: i128 = (s_70_5 + s_70_4);
        // D s_70_7: bit-extract s_70_2 s_70_1 s_70_6
        let s_70_7: Bits = (Bits::new(
            ((s_70_2) >> (s_70_1)).value(),
            u16::try_from(s_70_6).unwrap(),
        ));
        // D s_70_8: cast reint s_70_7 -> u8
        let s_70_8: bool = ((s_70_7.value()) != 0);
        // D s_70_9: cast zx s_70_8 -> bv
        let s_70_9: Bits = Bits::new(s_70_8 as u128, 1u16);
        // C s_70_10: const #0u : u8
        let s_70_10: bool = false;
        // C s_70_11: cast zx s_70_10 -> bv
        let s_70_11: Bits = Bits::new(s_70_10 as u128, 1u16);
        // D s_70_12: cmp-eq s_70_9 s_70_11
        let s_70_12: bool = ((s_70_9) == (s_70_11));
        // D s_70_13: not s_70_12
        let s_70_13: bool = !s_70_12;
        // N s_70_14: branch s_70_13 b73 b71
        if s_70_13 {
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
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#49848 <= s_71_0
        fn_state.gs_49848 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#49848:u8
        let s_72_0: bool = fn_state.gs_49848;
        // D s_72_1: write-var gs#49853 <= s_72_0
        fn_state.gs_49853 = s_72_0;
        // N s_72_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#49848 <= s_73_0
        fn_state.gs_49848 = s_73_0;
        // N s_73_2: jump b72
        return block_72(state, tracer, fn_state);
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
        // D s_74_2: write-var gs#49847 <= s_74_1
        fn_state.gs_49847 = s_74_1;
        // N s_74_3: jump b33
        return block_33(state, tracer, fn_state);
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
        // D s_75_5: write-var gs#49846 <= s_75_4
        fn_state.gs_49846 = s_75_4;
        // N s_75_6: jump b31
        return block_31(state, tracer, fn_state);
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
        // D s_76_4: write-var gs#49845 <= s_76_3
        fn_state.gs_49845 = s_76_3;
        // N s_76_5: jump b29
        return block_29(state, tracer, fn_state);
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
        // D s_78_0: read-var __SCR_EL3_NS:u8
        let s_78_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #0u : u8
        let s_78_2: bool = false;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#49844 <= s_78_4
        fn_state.gs_49844 = s_78_4;
        // N s_78_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var __MDCR_EL3_SBRBE:u8
        let s_79_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 2u16);
        // C s_79_2: const #3u : u8
        let s_79_2: u8 = 3;
        // C s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 2u16);
        // D s_79_4: cmp-ne s_79_1 s_79_3
        let s_79_4: bool = ((s_79_1) != (s_79_3));
        // D s_79_5: write-var gs#49843 <= s_79_4
        fn_state.gs_49843 = s_79_4;
        // N s_79_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_80_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_80_1: call __IMPDEF_boolean(s_80_0)
        let s_80_1: bool = u__IMPDEF_boolean(state, tracer, s_80_0);
        // D s_80_2: write-var gs#49842 <= s_80_1
        fn_state.gs_49842 = s_80_1;
        // N s_80_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var __EDSCR_SDD:u8
        let s_81_0: bool = fn_state.u__EDSCR_SDD;
        // D s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 1u16);
        // C s_81_2: const #1u : u8
        let s_81_2: bool = true;
        // C s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // D s_81_4: cmp-eq s_81_1 s_81_3
        let s_81_4: bool = ((s_81_1) == (s_81_3));
        // D s_81_5: write-var gs#49841 <= s_81_4
        fn_state.gs_49841 = s_81_4;
        // N s_81_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #424u : u32
        let s_82_0: u32 = 424;
        // D s_82_1: read-reg s_82_0:u8
        let s_82_1: u8 = {
            let value = state.read_register::<u8>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // C s_82_2: const #2u : u8
        let s_82_2: u8 = 2;
        // D s_82_3: cmp-lt s_82_1 s_82_2
        let s_82_3: bool = ((s_82_1) < (s_82_2));
        // D s_82_4: write-var gs#49840 <= s_82_3
        fn_state.gs_49840 = s_82_3;
        // N s_82_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call EL2Enabled(s_83_0)
        let s_83_1: bool = EL2Enabled(state, tracer, s_83_0);
        // N s_83_2: branch s_83_1 b94 b84
        if s_83_1 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var gs#49867 <= s_84_0
        fn_state.gs_49867 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#49867:u8
        let s_85_0: bool = fn_state.gs_49867;
        // N s_85_1: branch s_85_0 b92 b86
        if s_85_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #() : ()
        let s_86_0: () = ();
        // S s_86_1: call EL2Enabled(s_86_0)
        let s_86_1: bool = EL2Enabled(state, tracer, s_86_0);
        // N s_86_2: branch s_86_1 b91 b87
        if s_86_1 {
            return block_91(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#49868 <= s_87_0
        fn_state.gs_49868 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#49868:u8
        let s_88_0: bool = fn_state.gs_49868;
        // N s_88_1: branch s_88_0 b90 b89
        if s_88_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
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
        // C s_90_0: const #24u : u8
        let s_90_0: u8 = 24;
        // C s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 8u16);
        // C s_90_2: cast zx s_90_1 -> i
        let s_90_2: i128 = (s_90_1.value() as i128);
        // C s_90_3: cast reint s_90_2 -> i64
        let s_90_3: i64 = (s_90_2 as i64);
        // C s_90_4: cast zx s_90_3 -> i
        let s_90_4: i128 = (i128::try_from(s_90_3).unwrap());
        // C s_90_5: const #432u : u32
        let s_90_5: u32 = 432;
        // D s_90_6: read-reg s_90_5:u8
        let s_90_6: u8 = {
            let value = state.read_register::<u8>(s_90_5 as isize);
            tracer.read_register(s_90_5 as isize, value);
            value
        };
        // D s_90_7: call AArch64_SystemAccessTrap(s_90_6, s_90_4)
        let s_90_7: () = AArch64_SystemAccessTrap(state, tracer, s_90_6, s_90_4);
        // N s_90_8: return
        return;
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var __HCR_EL2_NV:u8
        let s_91_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 1u16);
        // C s_91_2: const #1u : u8
        let s_91_2: bool = true;
        // C s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // D s_91_4: cmp-eq s_91_1 s_91_3
        let s_91_4: bool = ((s_91_1) == (s_91_3));
        // D s_91_5: write-var gs#49868 <= s_91_4
        fn_state.gs_49868 = s_91_4;
        // N s_91_6: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #2272u : u12
        let s_92_0: u16 = 2272;
        // C s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 12u16);
        // C s_92_2: cast zx s_92_1 -> i
        let s_92_2: i128 = (s_92_1.value() as i128);
        // C s_92_3: cast reint s_92_2 -> i64
        let s_92_3: i64 = (s_92_2 as i64);
        // C s_92_4: cast zx s_92_3 -> i
        let s_92_4: i128 = (i128::try_from(s_92_3).unwrap());
        // S s_92_5: call NVMem_read(s_92_4)
        let s_92_5: u64 = NVMem_read(state, tracer, s_92_4);
        // D s_92_6: write-var ga#44658 <= s_92_5
        fn_state.ga_44658 = s_92_5;
        // N s_92_7: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var ga#44658:u64
        let s_93_0: u64 = fn_state.ga_44658;
        // D s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 64u16);
        // D s_93_2: read-var t:i
        let s_93_2: i128 = fn_state.t;
        // C s_93_3: const #64s : i64
        let s_93_3: i64 = 64;
        // D s_93_4: call X_set(s_93_2, s_93_3, s_93_1)
        let s_93_4: () = X_set(state, tracer, s_93_2, s_93_3, s_93_1);
        // N s_93_5: return
        return;
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #102552u : u32
        let s_94_0: u32 = 102552;
        // D s_94_1: read-reg s_94_0:struct
        let s_94_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_94_0 as isize);
            tracer.read_register(s_94_0 as isize, value);
            value
        };
        // D s_94_2: call _get_HCR_EL2_Type_NV2(s_94_1)
        let s_94_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_94_1);
        // C s_94_3: const #102552u : u32
        let s_94_3: u32 = 102552;
        // D s_94_4: read-reg s_94_3:struct
        let s_94_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_94_3 as isize);
            tracer.read_register(s_94_3 as isize, value);
            value
        };
        // D s_94_5: call _get_HCR_EL2_Type_NV1(s_94_4)
        let s_94_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_94_4);
        // C s_94_6: const #102552u : u32
        let s_94_6: u32 = 102552;
        // D s_94_7: read-reg s_94_6:struct
        let s_94_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_94_6 as isize);
            tracer.read_register(s_94_6 as isize, value);
            value
        };
        // D s_94_8: call _get_HCR_EL2_Type_NV(s_94_7)
        let s_94_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_94_7);
        // D s_94_9: cast zx s_94_5 -> bv
        let s_94_9: Bits = Bits::new(s_94_5 as u128, 1u16);
        // D s_94_10: cast zx s_94_8 -> bv
        let s_94_10: Bits = Bits::new(s_94_8 as u128, 1u16);
        // D s_94_11: cast reint s_94_9 -> u128
        let s_94_11: u128 = (s_94_9.value() as u128);
        // D s_94_12: size-of s_94_9
        let s_94_12: u16 = s_94_9.length();
        // D s_94_13: cast reint s_94_10 -> u128
        let s_94_13: u128 = (s_94_10.value() as u128);
        // D s_94_14: size-of s_94_10
        let s_94_14: u16 = s_94_10.length();
        // D s_94_15: lsl s_94_11 s_94_14
        let s_94_15: u128 = s_94_11 << s_94_14;
        // D s_94_16: or s_94_15 s_94_13
        let s_94_16: u128 = ((s_94_15) | (s_94_13));
        // D s_94_17: add s_94_12 s_94_14
        let s_94_17: u16 = (s_94_12 + s_94_14);
        // D s_94_18: create-bits s_94_16 s_94_17
        let s_94_18: Bits = Bits::new(s_94_16, s_94_17);
        // D s_94_19: cast reint s_94_18 -> u8
        let s_94_19: u8 = (s_94_18.value() as u8);
        // D s_94_20: cast zx s_94_2 -> bv
        let s_94_20: Bits = Bits::new(s_94_2 as u128, 1u16);
        // D s_94_21: cast zx s_94_19 -> bv
        let s_94_21: Bits = Bits::new(s_94_19 as u128, 2u16);
        // D s_94_22: cast reint s_94_20 -> u128
        let s_94_22: u128 = (s_94_20.value() as u128);
        // D s_94_23: size-of s_94_20
        let s_94_23: u16 = s_94_20.length();
        // D s_94_24: cast reint s_94_21 -> u128
        let s_94_24: u128 = (s_94_21.value() as u128);
        // D s_94_25: size-of s_94_21
        let s_94_25: u16 = s_94_21.length();
        // D s_94_26: lsl s_94_22 s_94_25
        let s_94_26: u128 = s_94_22 << s_94_25;
        // D s_94_27: or s_94_26 s_94_24
        let s_94_27: u128 = ((s_94_26) | (s_94_24));
        // D s_94_28: add s_94_23 s_94_25
        let s_94_28: u16 = (s_94_23 + s_94_25);
        // D s_94_29: create-bits s_94_27 s_94_28
        let s_94_29: Bits = Bits::new(s_94_27, s_94_28);
        // D s_94_30: cast reint s_94_29 -> u8
        let s_94_30: u8 = (s_94_29.value() as u8);
        // D s_94_31: cast zx s_94_30 -> bv
        let s_94_31: Bits = Bits::new(s_94_30 as u128, 3u16);
        // C s_94_32: const #5u : u8
        let s_94_32: u8 = 5;
        // C s_94_33: cast zx s_94_32 -> bv
        let s_94_33: Bits = Bits::new(s_94_32 as u128, 3u16);
        // D s_94_34: cmp-eq s_94_31 s_94_33
        let s_94_34: bool = ((s_94_31) == (s_94_33));
        // D s_94_35: write-var gs#49867 <= s_94_34
        fn_state.gs_49867 = s_94_34;
        // N s_94_36: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_95_0: panic
        panic!("{:?}", ());
        // N s_95_1: return
        return;
    }
}
