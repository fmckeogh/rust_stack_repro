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
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HFGWTR_EL2_Type_TCR_EL1::*;
use Halted::*;
use IsFeatureImplemented::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use Mk_TCR2_EL1_Type::*;
use u__IMPDEF_boolean::*;
use NVMem_set::*;
use u_get_SCR_EL3_Type_TCR2En::*;
use u_get_HCR_EL2_Type_NV1::*;
use IsHCRXEL2Enabled::*;
use u_get_HCR_EL2_Type_TVM::*;
use u_get_HCR_EL2_Type_NV::*;
use Mk_TCR2_EL2_Type::*;
use u_get_HCRX_EL2_Type_TCR2En::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn TCR2_EL2_SysRegWrite_c04bf19f053d56ef<T: Tracer>(
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
        u__HCRX_EL2_TCR2En: bool,
        gs_96676: bool,
        gs_96685: bool,
        gs_96679: bool,
        gs_96683: bool,
        u__HCR_EL2_E2H: bool,
        gs_96680: bool,
        u__EDSCR_SDD: bool,
        u__SCR_EL3_TCR2En: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_96682: bool,
        gs_96666: bool,
        gs_96673: bool,
        gs_96674: bool,
        gs_96688: bool,
        u__HCR_EL2_TVM: bool,
        gs_96684: bool,
        gs_96672: bool,
        gs_96665: bool,
        gs_96677: bool,
        gs_96675: bool,
        gs_96681: bool,
        gs_96678: bool,
        u__PSTATE_EL: u8,
        gs_96669: bool,
        gs_96668: bool,
        u__HFGWTR_EL2_TCR_EL1: bool,
        gs_96667: bool,
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
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_TCR2En(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_TCR2En(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_TCR2En <= s_0_9
        fn_state.u__SCR_EL3_TCR2En = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TVM(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TVM(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TVM <= s_0_13
        fn_state.u__HCR_EL2_TVM = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_FGTEn(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_FGTEn <= s_0_17
        fn_state.u__SCR_EL3_FGTEn = s_0_17;
        // C s_0_19: const #100992u : u32
        let s_0_19: u32 = 100992;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HFGWTR_EL2_Type_TCR_EL1(s_0_20)
        let s_0_21: bool = u_get_HFGWTR_EL2_Type_TCR_EL1(state, tracer, s_0_20);
        // D s_0_22: write-var __HFGWTR_EL2_TCR_EL1 <= s_0_21
        fn_state.u__HFGWTR_EL2_TCR_EL1 = s_0_21;
        // C s_0_23: const #22528u : u32
        let s_0_23: u32 = 22528;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HCRX_EL2_Type_TCR2En(s_0_24)
        let s_0_25: bool = u_get_HCRX_EL2_Type_TCR2En(state, tracer, s_0_24);
        // D s_0_26: write-var __HCRX_EL2_TCR2En <= s_0_25
        fn_state.u__HCRX_EL2_TCR2En = s_0_25;
        // C s_0_27: const #102552u : u32
        let s_0_27: u32 = 102552;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_HCR_EL2_Type_E2H(s_0_28)
        let s_0_29: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_28);
        // D s_0_30: write-var __HCR_EL2_E2H <= s_0_29
        fn_state.u__HCR_EL2_E2H = s_0_29;
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
        // N s_0_37: branch s_0_36 b90 b1
        if s_0_36 {
            return block_90(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b33 b2
        if s_1_5 {
            return block_33(state, tracer, fn_state);
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
        // D s_5_4: call Mk_TCR2_EL1_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_TCR2_EL1_Type(state, tracer, s_5_3);
        // C s_5_5: const #14776u : u32
        let s_5_5: u32 = 14776;
        // N s_5_6: write-reg s_5_5 <= s_5_4
        let s_5_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_5 as isize, s_5_4);
            tracer.write_register(s_5_5 as isize, s_5_4);
        };
        // N s_5_7: return
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
        // N s_6_2: branch s_6_1 b32 b7
        if s_6_1 {
            return block_32(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#96665 <= s_7_0
        fn_state.gs_96665 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#96665:u8
        let s_8_0: bool = fn_state.gs_96665;
        // N s_8_1: branch s_8_0 b31 b9
        if s_8_0 {
            return block_31(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#96666 <= s_9_0
        fn_state.gs_96666 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#96666:u8
        let s_10_0: bool = fn_state.gs_96666;
        // N s_10_1: branch s_10_0 b30 b11
        if s_10_0 {
            return block_30(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#96667 <= s_11_0
        fn_state.gs_96667 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#96667:u8
        let s_12_0: bool = fn_state.gs_96667;
        // N s_12_1: branch s_12_0 b29 b13
        if s_12_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#96668 <= s_13_0
        fn_state.gs_96668 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#96668:u8
        let s_14_0: bool = fn_state.gs_96668;
        // N s_14_1: branch s_14_0 b28 b15
        if s_14_0 {
            return block_28(state, tracer, fn_state);
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
        // N s_15_4: branch s_15_3 b27 b16
        if s_15_3 {
            return block_27(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#96669 <= s_16_0
        fn_state.gs_96669 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#96669:u8
        let s_17_0: bool = fn_state.gs_96669;
        // N s_17_1: branch s_17_0 b21 b18
        if s_17_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var __HCR_EL2_E2H:u8
        let s_18_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b20 b19
        if s_18_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #64s : i64
        let s_19_0: i64 = 64;
        // D s_19_1: read-var t:i
        let s_19_1: i128 = fn_state.t;
        // D s_19_2: call X_read(s_19_1, s_19_0)
        let s_19_2: Bits = X_read(state, tracer, s_19_1, s_19_0);
        // D s_19_3: cast reint s_19_2 -> u64
        let s_19_3: u64 = (s_19_2.value() as u64);
        // D s_19_4: call Mk_TCR2_EL1_Type(s_19_3)
        let s_19_4: ProductType5c790c8ef59cc8b2 = Mk_TCR2_EL1_Type(
            state,
            tracer,
            s_19_3,
        );
        // C s_19_5: const #14776u : u32
        let s_19_5: u32 = 14776;
        // N s_19_6: write-reg s_19_5 <= s_19_4
        let s_19_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_19_5 as isize, s_19_4);
            tracer.write_register(s_19_5 as isize, s_19_4);
        };
        // N s_19_7: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #64s : i64
        let s_20_0: i64 = 64;
        // D s_20_1: read-var t:i
        let s_20_1: i128 = fn_state.t;
        // D s_20_2: call X_read(s_20_1, s_20_0)
        let s_20_2: Bits = X_read(state, tracer, s_20_1, s_20_0);
        // D s_20_3: cast reint s_20_2 -> u64
        let s_20_3: u64 = (s_20_2.value() as u64);
        // D s_20_4: call Mk_TCR2_EL2_Type(s_20_3)
        let s_20_4: ProductType5c790c8ef59cc8b2 = Mk_TCR2_EL2_Type(
            state,
            tracer,
            s_20_3,
        );
        // C s_20_5: const #15752u : u32
        let s_20_5: u32 = 15752;
        // N s_20_6: write-reg s_20_5 <= s_20_4
        let s_20_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_20_5 as isize, s_20_4);
            tracer.write_register(s_20_5 as isize, s_20_4);
        };
        // N s_20_7: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call Halted(s_21_0)
        let s_21_1: bool = Halted(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b26 b22
        if s_21_1 {
            return block_26(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#96672 <= s_22_0
        fn_state.gs_96672 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#96672:u8
        let s_23_0: bool = fn_state.gs_96672;
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
        // C s_24_0: const #24u : u8
        let s_24_0: u8 = 24;
        // C s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 8u16);
        // C s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (s_24_1.value() as i128);
        // C s_24_3: cast reint s_24_2 -> i64
        let s_24_3: i64 = (s_24_2 as i64);
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // C s_24_5: const #424u : u32
        let s_24_5: u32 = 424;
        // D s_24_6: read-reg s_24_5:u8
        let s_24_6: u8 = {
            let value = state.read_register::<u8>(s_24_5 as isize);
            tracer.read_register(s_24_5 as isize, value);
            value
        };
        // D s_24_7: call AArch64_SystemAccessTrap(s_24_6, s_24_4)
        let s_24_7: () = AArch64_SystemAccessTrap(state, tracer, s_24_6, s_24_4);
        // N s_24_8: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __EDSCR_SDD:u8
        let s_26_0: bool = fn_state.u__EDSCR_SDD;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#96672 <= s_26_4
        fn_state.gs_96672 = s_26_4;
        // N s_26_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __SCR_EL3_TCR2En:u8
        let s_27_0: bool = fn_state.u__SCR_EL3_TCR2En;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#96669 <= s_27_4
        fn_state.gs_96669 = s_27_4;
        // N s_27_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __SCR_EL3_TCR2En:u8
        let s_29_0: bool = fn_state.u__SCR_EL3_TCR2En;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #0u : u8
        let s_29_2: bool = false;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#96668 <= s_29_4
        fn_state.gs_96668 = s_29_4;
        // N s_29_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_30_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_30_1: call __IMPDEF_boolean(s_30_0)
        let s_30_1: bool = u__IMPDEF_boolean(state, tracer, s_30_0);
        // D s_30_2: write-var gs#96667 <= s_30_1
        fn_state.gs_96667 = s_30_1;
        // N s_30_3: jump b12
        return block_12(state, tracer, fn_state);
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
        // D s_31_5: write-var gs#96666 <= s_31_4
        fn_state.gs_96666 = s_31_4;
        // N s_31_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #424u : u32
        let s_32_0: u32 = 424;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // C s_32_2: const #2u : u8
        let s_32_2: u8 = 2;
        // D s_32_3: cmp-lt s_32_1 s_32_2
        let s_32_3: bool = ((s_32_1) < (s_32_2));
        // D s_32_4: write-var gs#96665 <= s_32_3
        fn_state.gs_96665 = s_32_3;
        // N s_32_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call Halted(s_33_0)
        let s_33_1: bool = Halted(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b89 b34
        if s_33_1 {
            return block_89(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#96673 <= s_34_0
        fn_state.gs_96673 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#96673:u8
        let s_35_0: bool = fn_state.gs_96673;
        // N s_35_1: branch s_35_0 b88 b36
        if s_35_0 {
            return block_88(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#96674 <= s_36_0
        fn_state.gs_96674 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#96674:u8
        let s_37_0: bool = fn_state.gs_96674;
        // N s_37_1: branch s_37_0 b87 b38
        if s_37_0 {
            return block_87(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#96675 <= s_38_0
        fn_state.gs_96675 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#96675:u8
        let s_39_0: bool = fn_state.gs_96675;
        // N s_39_1: branch s_39_0 b86 b40
        if s_39_0 {
            return block_86(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#96676 <= s_40_0
        fn_state.gs_96676 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#96676:u8
        let s_41_0: bool = fn_state.gs_96676;
        // N s_41_1: branch s_41_0 b85 b42
        if s_41_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call EL2Enabled(s_42_0)
        let s_42_1: bool = EL2Enabled(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b84 b43
        if s_42_1 {
            return block_84(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#96677 <= s_43_0
        fn_state.gs_96677 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#96677:u8
        let s_44_0: bool = fn_state.gs_96677;
        // N s_44_1: branch s_44_0 b83 b45
        if s_44_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call EL2Enabled(s_45_0)
        let s_45_1: bool = EL2Enabled(state, tracer, s_45_0);
        // N s_45_2: branch s_45_1 b82 b46
        if s_45_1 {
            return block_82(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#96678 <= s_46_0
        fn_state.gs_96678 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#96678:u8
        let s_47_0: bool = fn_state.gs_96678;
        // N s_47_1: branch s_47_0 b78 b48
        if s_47_0 {
            return block_78(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#96680 <= s_48_0
        fn_state.gs_96680 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#96680:u8
        let s_49_0: bool = fn_state.gs_96680;
        // N s_49_1: branch s_49_0 b77 b50
        if s_49_0 {
            return block_77(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#96681 <= s_50_0
        fn_state.gs_96681 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#96681:u8
        let s_51_0: bool = fn_state.gs_96681;
        // N s_51_1: branch s_51_0 b76 b52
        if s_51_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EL2Enabled(s_52_0)
        let s_52_1: bool = EL2Enabled(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b72 b53
        if s_52_1 {
            return block_72(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#96683 <= s_53_0
        fn_state.gs_96683 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#96683:u8
        let s_54_0: bool = fn_state.gs_96683;
        // N s_54_1: branch s_54_0 b71 b55
        if s_54_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #424u : u32
        let s_55_0: u32 = 424;
        // D s_55_1: read-reg s_55_0:u8
        let s_55_1: u8 = {
            let value = state.read_register::<u8>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // C s_55_2: const #2u : u8
        let s_55_2: u8 = 2;
        // D s_55_3: cmp-lt s_55_1 s_55_2
        let s_55_3: bool = ((s_55_1) < (s_55_2));
        // N s_55_4: branch s_55_3 b70 b56
        if s_55_3 {
            return block_70(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#96684 <= s_56_0
        fn_state.gs_96684 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#96684:u8
        let s_57_0: bool = fn_state.gs_96684;
        // N s_57_1: branch s_57_0 b64 b58
        if s_57_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call EL2Enabled(s_58_0)
        let s_58_1: bool = EL2Enabled(state, tracer, s_58_0);
        // N s_58_2: branch s_58_1 b63 b59
        if s_58_1 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#96685 <= s_59_0
        fn_state.gs_96685 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#96685:u8
        let s_60_0: bool = fn_state.gs_96685;
        // N s_60_1: branch s_60_0 b62 b61
        if s_60_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #64s : i64
        let s_61_0: i64 = 64;
        // D s_61_1: read-var t:i
        let s_61_1: i128 = fn_state.t;
        // D s_61_2: call X_read(s_61_1, s_61_0)
        let s_61_2: Bits = X_read(state, tracer, s_61_1, s_61_0);
        // D s_61_3: cast reint s_61_2 -> u64
        let s_61_3: u64 = (s_61_2.value() as u64);
        // D s_61_4: call Mk_TCR2_EL1_Type(s_61_3)
        let s_61_4: ProductType5c790c8ef59cc8b2 = Mk_TCR2_EL1_Type(
            state,
            tracer,
            s_61_3,
        );
        // C s_61_5: const #14776u : u32
        let s_61_5: u32 = 14776;
        // N s_61_6: write-reg s_61_5 <= s_61_4
        let s_61_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_61_5 as isize, s_61_4);
            tracer.write_register(s_61_5 as isize, s_61_4);
        };
        // N s_61_7: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #624u : u12
        let s_62_0: u16 = 624;
        // C s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 12u16);
        // C s_62_2: cast zx s_62_1 -> i
        let s_62_2: i128 = (s_62_1.value() as i128);
        // C s_62_3: cast reint s_62_2 -> i64
        let s_62_3: i64 = (s_62_2 as i64);
        // C s_62_4: const #64s : i64
        let s_62_4: i64 = 64;
        // D s_62_5: read-var t:i
        let s_62_5: i128 = fn_state.t;
        // D s_62_6: call X_read(s_62_5, s_62_4)
        let s_62_6: Bits = X_read(state, tracer, s_62_5, s_62_4);
        // D s_62_7: cast reint s_62_6 -> u64
        let s_62_7: u64 = (s_62_6.value() as u64);
        // C s_62_8: cast zx s_62_3 -> i
        let s_62_8: i128 = (i128::try_from(s_62_3).unwrap());
        // D s_62_9: call NVMem_set(s_62_8, s_62_7)
        let s_62_9: () = NVMem_set(state, tracer, s_62_8, s_62_7);
        // N s_62_10: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #102552u : u32
        let s_63_0: u32 = 102552;
        // D s_63_1: read-reg s_63_0:struct
        let s_63_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call _get_HCR_EL2_Type_NV2(s_63_1)
        let s_63_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_63_1);
        // C s_63_3: const #102552u : u32
        let s_63_3: u32 = 102552;
        // D s_63_4: read-reg s_63_3:struct
        let s_63_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_3 as isize);
            tracer.read_register(s_63_3 as isize, value);
            value
        };
        // D s_63_5: call _get_HCR_EL2_Type_NV1(s_63_4)
        let s_63_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_63_4);
        // C s_63_6: const #102552u : u32
        let s_63_6: u32 = 102552;
        // D s_63_7: read-reg s_63_6:struct
        let s_63_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_6 as isize);
            tracer.read_register(s_63_6 as isize, value);
            value
        };
        // D s_63_8: call _get_HCR_EL2_Type_NV(s_63_7)
        let s_63_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_63_7);
        // D s_63_9: cast zx s_63_5 -> bv
        let s_63_9: Bits = Bits::new(s_63_5 as u128, 1u16);
        // D s_63_10: cast zx s_63_8 -> bv
        let s_63_10: Bits = Bits::new(s_63_8 as u128, 1u16);
        // D s_63_11: cast reint s_63_9 -> u128
        let s_63_11: u128 = (s_63_9.value() as u128);
        // D s_63_12: size-of s_63_9
        let s_63_12: u16 = s_63_9.length();
        // D s_63_13: cast reint s_63_10 -> u128
        let s_63_13: u128 = (s_63_10.value() as u128);
        // D s_63_14: size-of s_63_10
        let s_63_14: u16 = s_63_10.length();
        // D s_63_15: lsl s_63_11 s_63_14
        let s_63_15: u128 = s_63_11 << s_63_14;
        // D s_63_16: or s_63_15 s_63_13
        let s_63_16: u128 = ((s_63_15) | (s_63_13));
        // D s_63_17: add s_63_12 s_63_14
        let s_63_17: u16 = (s_63_12 + s_63_14);
        // D s_63_18: create-bits s_63_16 s_63_17
        let s_63_18: Bits = Bits::new(s_63_16, s_63_17);
        // D s_63_19: cast reint s_63_18 -> u8
        let s_63_19: u8 = (s_63_18.value() as u8);
        // D s_63_20: cast zx s_63_2 -> bv
        let s_63_20: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_21: cast zx s_63_19 -> bv
        let s_63_21: Bits = Bits::new(s_63_19 as u128, 2u16);
        // D s_63_22: cast reint s_63_20 -> u128
        let s_63_22: u128 = (s_63_20.value() as u128);
        // D s_63_23: size-of s_63_20
        let s_63_23: u16 = s_63_20.length();
        // D s_63_24: cast reint s_63_21 -> u128
        let s_63_24: u128 = (s_63_21.value() as u128);
        // D s_63_25: size-of s_63_21
        let s_63_25: u16 = s_63_21.length();
        // D s_63_26: lsl s_63_22 s_63_25
        let s_63_26: u128 = s_63_22 << s_63_25;
        // D s_63_27: or s_63_26 s_63_24
        let s_63_27: u128 = ((s_63_26) | (s_63_24));
        // D s_63_28: add s_63_23 s_63_25
        let s_63_28: u16 = (s_63_23 + s_63_25);
        // D s_63_29: create-bits s_63_27 s_63_28
        let s_63_29: Bits = Bits::new(s_63_27, s_63_28);
        // D s_63_30: cast reint s_63_29 -> u8
        let s_63_30: u8 = (s_63_29.value() as u8);
        // D s_63_31: cast zx s_63_30 -> bv
        let s_63_31: Bits = Bits::new(s_63_30 as u128, 3u16);
        // C s_63_32: const #7u : u8
        let s_63_32: u8 = 7;
        // C s_63_33: cast zx s_63_32 -> bv
        let s_63_33: Bits = Bits::new(s_63_32 as u128, 3u16);
        // D s_63_34: cmp-eq s_63_31 s_63_33
        let s_63_34: bool = ((s_63_31) == (s_63_33));
        // D s_63_35: write-var gs#96685 <= s_63_34
        fn_state.gs_96685 = s_63_34;
        // N s_63_36: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call Halted(s_64_0)
        let s_64_1: bool = Halted(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b69 b65
        if s_64_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#96688 <= s_65_0
        fn_state.gs_96688 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#96688:u8
        let s_66_0: bool = fn_state.gs_96688;
        // N s_66_1: branch s_66_0 b68 b67
        if s_66_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #24u : u8
        let s_67_0: u8 = 24;
        // C s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 8u16);
        // C s_67_2: cast zx s_67_1 -> i
        let s_67_2: i128 = (s_67_1.value() as i128);
        // C s_67_3: cast reint s_67_2 -> i64
        let s_67_3: i64 = (s_67_2 as i64);
        // C s_67_4: cast zx s_67_3 -> i
        let s_67_4: i128 = (i128::try_from(s_67_3).unwrap());
        // C s_67_5: const #424u : u32
        let s_67_5: u32 = 424;
        // D s_67_6: read-reg s_67_5:u8
        let s_67_6: u8 = {
            let value = state.read_register::<u8>(s_67_5 as isize);
            tracer.read_register(s_67_5 as isize, value);
            value
        };
        // D s_67_7: call AArch64_SystemAccessTrap(s_67_6, s_67_4)
        let s_67_7: () = AArch64_SystemAccessTrap(state, tracer, s_67_6, s_67_4);
        // N s_67_8: return
        return;
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
        // D s_69_0: read-var __EDSCR_SDD:u8
        let s_69_0: bool = fn_state.u__EDSCR_SDD;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#96688 <= s_69_4
        fn_state.gs_96688 = s_69_4;
        // N s_69_6: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __SCR_EL3_TCR2En:u8
        let s_70_0: bool = fn_state.u__SCR_EL3_TCR2En;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #0u : u8
        let s_70_2: bool = false;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#96684 <= s_70_4
        fn_state.gs_96684 = s_70_4;
        // N s_70_6: jump b57
        return block_57(state, tracer, fn_state);
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
        // C s_71_5: const #432u : u32
        let s_71_5: u32 = 432;
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
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call IsHCRXEL2Enabled(s_72_0)
        let s_72_1: bool = IsHCRXEL2Enabled(state, tracer, s_72_0);
        // S s_72_2: not s_72_1
        let s_72_2: bool = !s_72_1;
        // N s_72_3: branch s_72_2 b75 b73
        if s_72_2 {
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
        // D s_73_0: read-var __HCRX_EL2_TCR2En:u8
        let s_73_0: bool = fn_state.u__HCRX_EL2_TCR2En;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #0u : u8
        let s_73_2: bool = false;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#96682 <= s_73_4
        fn_state.gs_96682 = s_73_4;
        // N s_73_6: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#96682:u8
        let s_74_0: bool = fn_state.gs_96682;
        // D s_74_1: write-var gs#96683 <= s_74_0
        fn_state.gs_96683 = s_74_0;
        // N s_74_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#96682 <= s_75_0
        fn_state.gs_96682 = s_75_0;
        // N s_75_2: jump b74
        return block_74(state, tracer, fn_state);
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
        // D s_77_0: read-var __HFGWTR_EL2_TCR_EL1:u8
        let s_77_0: bool = fn_state.u__HFGWTR_EL2_TCR_EL1;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 1u16);
        // C s_77_2: const #1u : u8
        let s_77_2: bool = true;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: write-var gs#96681 <= s_77_4
        fn_state.gs_96681 = s_77_4;
        // N s_77_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #424u : u32
        let s_78_0: u32 = 424;
        // D s_78_1: read-reg s_78_0:u8
        let s_78_1: u8 = {
            let value = state.read_register::<u8>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // C s_78_2: const #2u : u8
        let s_78_2: u8 = 2;
        // D s_78_3: cmp-lt s_78_1 s_78_2
        let s_78_3: bool = ((s_78_1) < (s_78_2));
        // D s_78_4: not s_78_3
        let s_78_4: bool = !s_78_3;
        // N s_78_5: branch s_78_4 b81 b79
        if s_78_4 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var __SCR_EL3_FGTEn:u8
        let s_79_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 1u16);
        // C s_79_2: const #1u : u8
        let s_79_2: bool = true;
        // C s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_4: cmp-eq s_79_1 s_79_3
        let s_79_4: bool = ((s_79_1) == (s_79_3));
        // D s_79_5: write-var gs#96679 <= s_79_4
        fn_state.gs_96679 = s_79_4;
        // N s_79_6: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#96679:u8
        let s_80_0: bool = fn_state.gs_96679;
        // D s_80_1: write-var gs#96680 <= s_80_0
        fn_state.gs_96680 = s_80_0;
        // N s_80_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #1u : u8
        let s_81_0: bool = true;
        // D s_81_1: write-var gs#96679 <= s_81_0
        fn_state.gs_96679 = s_81_0;
        // N s_81_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #146u : u32
        let s_82_0: u32 = 146;
        // S s_82_1: call IsFeatureImplemented(s_82_0)
        let s_82_1: bool = IsFeatureImplemented(state, tracer, s_82_0);
        // D s_82_2: write-var gs#96678 <= s_82_1
        fn_state.gs_96678 = s_82_1;
        // N s_82_3: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #24u : u8
        let s_83_0: u8 = 24;
        // C s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 8u16);
        // C s_83_2: cast zx s_83_1 -> i
        let s_83_2: i128 = (s_83_1.value() as i128);
        // C s_83_3: cast reint s_83_2 -> i64
        let s_83_3: i64 = (s_83_2 as i64);
        // C s_83_4: cast zx s_83_3 -> i
        let s_83_4: i128 = (i128::try_from(s_83_3).unwrap());
        // C s_83_5: const #432u : u32
        let s_83_5: u32 = 432;
        // D s_83_6: read-reg s_83_5:u8
        let s_83_6: u8 = {
            let value = state.read_register::<u8>(s_83_5 as isize);
            tracer.read_register(s_83_5 as isize, value);
            value
        };
        // D s_83_7: call AArch64_SystemAccessTrap(s_83_6, s_83_4)
        let s_83_7: () = AArch64_SystemAccessTrap(state, tracer, s_83_6, s_83_4);
        // N s_83_8: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __HCR_EL2_TVM:u8
        let s_84_0: bool = fn_state.u__HCR_EL2_TVM;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#96677 <= s_84_4
        fn_state.gs_96677 = s_84_4;
        // N s_84_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_85_0: panic
        panic!("{:?}", ());
        // N s_85_1: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var __SCR_EL3_TCR2En:u8
        let s_86_0: bool = fn_state.u__SCR_EL3_TCR2En;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 1u16);
        // C s_86_2: const #0u : u8
        let s_86_2: bool = false;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: write-var gs#96676 <= s_86_4
        fn_state.gs_96676 = s_86_4;
        // N s_86_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_87_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_87_1: call __IMPDEF_boolean(s_87_0)
        let s_87_1: bool = u__IMPDEF_boolean(state, tracer, s_87_0);
        // D s_87_2: write-var gs#96675 <= s_87_1
        fn_state.gs_96675 = s_87_1;
        // N s_87_3: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var __EDSCR_SDD:u8
        let s_88_0: bool = fn_state.u__EDSCR_SDD;
        // D s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 1u16);
        // C s_88_2: const #1u : u8
        let s_88_2: bool = true;
        // C s_88_3: cast zx s_88_2 -> bv
        let s_88_3: Bits = Bits::new(s_88_2 as u128, 1u16);
        // D s_88_4: cmp-eq s_88_1 s_88_3
        let s_88_4: bool = ((s_88_1) == (s_88_3));
        // D s_88_5: write-var gs#96674 <= s_88_4
        fn_state.gs_96674 = s_88_4;
        // N s_88_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #424u : u32
        let s_89_0: u32 = 424;
        // D s_89_1: read-reg s_89_0:u8
        let s_89_1: u8 = {
            let value = state.read_register::<u8>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // C s_89_2: const #2u : u8
        let s_89_2: u8 = 2;
        // D s_89_3: cmp-lt s_89_1 s_89_2
        let s_89_3: bool = ((s_89_1) < (s_89_2));
        // D s_89_4: write-var gs#96673 <= s_89_3
        fn_state.gs_96673 = s_89_3;
        // N s_89_5: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_90_0: panic
        panic!("{:?}", ());
        // N s_90_1: return
        return;
    }
}
