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
use u_get_CPACR_EL1_Type_FPEN::*;
use u_get_CPTR_EL2_Type_TFP::*;
use u_get_HCR_EL2_Type_E2H::*;
use Halted::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_CPTR_EL3_Type_TFP::*;
use u__get_FPCR::*;
use u_get_CPTR_EL2_Type_FPEN::*;
use X_set::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use EDSCR_read::*;
use common::*;
pub fn FPCR_SysRegRead_48b824e8174adfc0<T: Tracer>(
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
        u__HCR_EL2_E2H: bool,
        gs_61650: bool,
        gs_61637: bool,
        gs_61627: bool,
        gs_61646: bool,
        gs_61648: bool,
        gs_61629: bool,
        gs_61655: bool,
        gs_61653: bool,
        gs_61645: bool,
        gs_61651: bool,
        gs_61668: bool,
        ga_65477: ProductType5c790c8ef59cc8b2,
        gs_61614: bool,
        gs_61624: bool,
        gs_61616: bool,
        gs_61625: bool,
        gs_61631: bool,
        gs_61615: bool,
        gs_61663: bool,
        gs_61632: bool,
        gs_61639: bool,
        u__PSTATE_EL: u8,
        gs_61619: bool,
        gs_61658: bool,
        gs_61656: bool,
        gs_61652: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        gs_61649: bool,
        u__CPTR_EL2_FPEN: u8,
        gs_61638: bool,
        gs_61664: bool,
        gs_61669: bool,
        ga_65528: ProductType5c790c8ef59cc8b2,
        gs_61640: bool,
        ga_65505: ProductType5c790c8ef59cc8b2,
        u__CPTR_EL2_TFP: bool,
        gs_61665: bool,
        u__CPACR_EL1_FPEN: u8,
        gs_61617: bool,
        gs_61666: bool,
        gs_61628: bool,
        gs_61618: bool,
        ga_65534: ProductType5c790c8ef59cc8b2,
        gs_61630: bool,
        gs_61654: bool,
        u__CPTR_EL3_TFP: bool,
        gs_61657: bool,
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
        // C s_0_7: const #16840u : u32
        let s_0_7: u32 = 16840;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_CPTR_EL3_Type_TFP(s_0_8)
        let s_0_9: bool = u_get_CPTR_EL3_Type_TFP(state, tracer, s_0_8);
        // D s_0_10: write-var __CPTR_EL3_TFP <= s_0_9
        fn_state.u__CPTR_EL3_TFP = s_0_9;
        // C s_0_11: const #12088u : u32
        let s_0_11: u32 = 12088;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_CPACR_EL1_Type_FPEN(s_0_12)
        let s_0_13: u8 = u_get_CPACR_EL1_Type_FPEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CPACR_EL1_FPEN <= s_0_13
        fn_state.u__CPACR_EL1_FPEN = s_0_13;
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
        // C s_0_19: const #11088u : u32
        let s_0_19: u32 = 11088;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_CPTR_EL2_Type_FPEN(s_0_20)
        let s_0_21: u8 = u_get_CPTR_EL2_Type_FPEN(state, tracer, s_0_20);
        // D s_0_22: write-var __CPTR_EL2_FPEN <= s_0_21
        fn_state.u__CPTR_EL2_FPEN = s_0_21;
        // C s_0_23: const #102552u : u32
        let s_0_23: u32 = 102552;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HCR_EL2_Type_E2H(s_0_24)
        let s_0_25: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_24);
        // D s_0_26: write-var __HCR_EL2_E2H <= s_0_25
        fn_state.u__HCR_EL2_E2H = s_0_25;
        // C s_0_27: const #11088u : u32
        let s_0_27: u32 = 11088;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_CPTR_EL2_Type_TFP(s_0_28)
        let s_0_29: bool = u_get_CPTR_EL2_Type_TFP(state, tracer, s_0_28);
        // D s_0_30: write-var __CPTR_EL2_TFP <= s_0_29
        fn_state.u__CPTR_EL2_TFP = s_0_29;
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
        // N s_0_37: branch s_0_36 b95 b1
        if s_0_36 {
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
        // N s_1_6: branch s_1_5 b46 b2
        if s_1_5 {
            return block_46(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b8 b3
        if s_2_5 {
            return block_8(state, tracer, fn_state);
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
        // D s_5_0: read-var __CPTR_EL3_TFP:u8
        let s_5_0: bool = fn_state.u__CPTR_EL3_TFP;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #1u : u8
        let s_5_2: bool = true;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #12920u : u32
        let s_6_1: u32 = 12920;
        // D s_6_2: read-reg s_6_1:struct
        let s_6_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_1 as isize);
            tracer.read_register(s_6_1 as isize, value);
            value
        };
        // D s_6_3: call __get_FPCR(s_6_2)
        let s_6_3: ProductType5c790c8ef59cc8b2 = u__get_FPCR(state, tracer, s_6_2);
        // D s_6_4: write-var ga#65534 <= s_6_3
        fn_state.ga_65534 = s_6_3;
        // D s_6_5: read-var ga#65534.0:struct
        let s_6_5: u64 = fn_state.ga_65534._0;
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 64u16);
        // D s_6_7: read-var t:i
        let s_6_7: i128 = fn_state.t;
        // D s_6_8: call X_set(s_6_7, s_6_0, s_6_6)
        let s_6_8: () = X_set(state, tracer, s_6_7, s_6_0, s_6_6);
        // N s_6_9: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #7u : u8
        let s_7_0: u8 = 7;
        // C s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 8u16);
        // C s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (s_7_1.value() as i128);
        // C s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #424u : u32
        let s_7_5: u32 = 424;
        // D s_7_6: read-reg s_7_5:u8
        let s_7_6: u8 = {
            let value = state.read_register::<u8>(s_7_5 as isize);
            tracer.read_register(s_7_5 as isize, value);
            value
        };
        // D s_7_7: call AArch64_SystemAccessTrap(s_7_6, s_7_4)
        let s_7_7: () = AArch64_SystemAccessTrap(state, tracer, s_7_6, s_7_4);
        // N s_7_8: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call Halted(s_8_0)
        let s_8_1: bool = Halted(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b45 b9
        if s_8_1 {
            return block_45(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#61614 <= s_9_0
        fn_state.gs_61614 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#61614:u8
        let s_10_0: bool = fn_state.gs_61614;
        // N s_10_1: branch s_10_0 b44 b11
        if s_10_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#61615 <= s_11_0
        fn_state.gs_61615 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#61615:u8
        let s_12_0: bool = fn_state.gs_61615;
        // N s_12_1: branch s_12_0 b43 b13
        if s_12_0 {
            return block_43(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#61616 <= s_13_0
        fn_state.gs_61616 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#61616:u8
        let s_14_0: bool = fn_state.gs_61616;
        // N s_14_1: branch s_14_0 b42 b15
        if s_14_0 {
            return block_42(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#61617 <= s_15_0
        fn_state.gs_61617 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#61617:u8
        let s_16_0: bool = fn_state.gs_61617;
        // N s_16_1: branch s_16_0 b41 b17
        if s_16_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var __HCR_EL2_E2H:u8
        let s_17_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 1u16);
        // C s_17_2: const #0u : u8
        let s_17_2: bool = false;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // N s_17_5: branch s_17_4 b40 b18
        if s_17_4 {
            return block_40(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#61618 <= s_18_0
        fn_state.gs_61618 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#61618:u8
        let s_19_0: bool = fn_state.gs_61618;
        // N s_19_1: branch s_19_0 b39 b20
        if s_19_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var __HCR_EL2_E2H:u8
        let s_20_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 1u16);
        // C s_20_2: const #1u : u8
        let s_20_2: bool = true;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // N s_20_5: branch s_20_4 b35 b21
        if s_20_4 {
            return block_35(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#61624 <= s_21_0
        fn_state.gs_61624 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#61624:u8
        let s_22_0: bool = fn_state.gs_61624;
        // N s_22_1: branch s_22_0 b34 b23
        if s_22_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #424u : u32
        let s_23_0: u32 = 424;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // C s_23_2: const #2u : u8
        let s_23_2: u8 = 2;
        // D s_23_3: cmp-lt s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) < (s_23_2));
        // N s_23_4: branch s_23_3 b33 b24
        if s_23_3 {
            return block_33(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#61625 <= s_24_0
        fn_state.gs_61625 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#61625:u8
        let s_25_0: bool = fn_state.gs_61625;
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
        // C s_26_0: const #64s : i64
        let s_26_0: i64 = 64;
        // C s_26_1: const #12920u : u32
        let s_26_1: u32 = 12920;
        // D s_26_2: read-reg s_26_1:struct
        let s_26_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_1 as isize);
            tracer.read_register(s_26_1 as isize, value);
            value
        };
        // D s_26_3: call __get_FPCR(s_26_2)
        let s_26_3: ProductType5c790c8ef59cc8b2 = u__get_FPCR(state, tracer, s_26_2);
        // D s_26_4: write-var ga#65528 <= s_26_3
        fn_state.ga_65528 = s_26_3;
        // D s_26_5: read-var ga#65528.0:struct
        let s_26_5: u64 = fn_state.ga_65528._0;
        // D s_26_6: cast zx s_26_5 -> bv
        let s_26_6: Bits = Bits::new(s_26_5 as u128, 64u16);
        // D s_26_7: read-var t:i
        let s_26_7: i128 = fn_state.t;
        // D s_26_8: call X_set(s_26_7, s_26_0, s_26_6)
        let s_26_8: () = X_set(state, tracer, s_26_7, s_26_0, s_26_6);
        // N s_26_9: return
        return;
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
        // N s_27_2: branch s_27_1 b32 b28
        if s_27_1 {
            return block_32(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#61627 <= s_28_0
        fn_state.gs_61627 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#61627:u8
        let s_29_0: bool = fn_state.gs_61627;
        // N s_29_1: branch s_29_0 b31 b30
        if s_29_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #7u : u8
        let s_30_0: u8 = 7;
        // C s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 8u16);
        // C s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (s_30_1.value() as i128);
        // C s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: cast zx s_30_3 -> i
        let s_30_4: i128 = (i128::try_from(s_30_3).unwrap());
        // C s_30_5: const #424u : u32
        let s_30_5: u32 = 424;
        // D s_30_6: read-reg s_30_5:u8
        let s_30_6: u8 = {
            let value = state.read_register::<u8>(s_30_5 as isize);
            tracer.read_register(s_30_5 as isize, value);
            value
        };
        // D s_30_7: call AArch64_SystemAccessTrap(s_30_6, s_30_4)
        let s_30_7: () = AArch64_SystemAccessTrap(state, tracer, s_30_6, s_30_4);
        // N s_30_8: return
        return;
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
        // D s_32_0: read-var __EDSCR_SDD:u8
        let s_32_0: bool = fn_state.u__EDSCR_SDD;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#61627 <= s_32_4
        fn_state.gs_61627 = s_32_4;
        // N s_32_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __CPTR_EL3_TFP:u8
        let s_33_0: bool = fn_state.u__CPTR_EL3_TFP;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#61625 <= s_33_4
        fn_state.gs_61625 = s_33_4;
        // N s_33_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #7u : u8
        let s_34_0: u8 = 7;
        // C s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 8u16);
        // C s_34_2: cast zx s_34_1 -> i
        let s_34_2: i128 = (s_34_1.value() as i128);
        // C s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: cast zx s_34_3 -> i
        let s_34_4: i128 = (i128::try_from(s_34_3).unwrap());
        // C s_34_5: const #432u : u32
        let s_34_5: u32 = 432;
        // D s_34_6: read-reg s_34_5:u8
        let s_34_6: u8 = {
            let value = state.read_register::<u8>(s_34_5 as isize);
            tracer.read_register(s_34_5 as isize, value);
            value
        };
        // D s_34_7: call AArch64_SystemAccessTrap(s_34_6, s_34_4)
        let s_34_7: () = AArch64_SystemAccessTrap(state, tracer, s_34_6, s_34_4);
        // N s_34_8: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var __CPTR_EL2_FPEN:u8
        let s_35_0: u8 = fn_state.u__CPTR_EL2_FPEN;
        // C s_35_1: const #0s : i
        let s_35_1: i128 = 0;
        // D s_35_2: cast zx s_35_0 -> bv
        let s_35_2: Bits = Bits::new(s_35_0 as u128, 2u16);
        // C s_35_3: const #1s : i64
        let s_35_3: i64 = 1;
        // C s_35_4: cast zx s_35_3 -> i
        let s_35_4: i128 = (i128::try_from(s_35_3).unwrap());
        // C s_35_5: const #0s : i
        let s_35_5: i128 = 0;
        // C s_35_6: add s_35_5 s_35_4
        let s_35_6: i128 = (s_35_5 + s_35_4);
        // D s_35_7: bit-extract s_35_2 s_35_1 s_35_6
        let s_35_7: Bits = (Bits::new(
            ((s_35_2) >> (s_35_1)).value(),
            u16::try_from(s_35_6).unwrap(),
        ));
        // D s_35_8: cast reint s_35_7 -> u8
        let s_35_8: bool = ((s_35_7.value()) != 0);
        // D s_35_9: cast zx s_35_8 -> bv
        let s_35_9: Bits = Bits::new(s_35_8 as u128, 1u16);
        // C s_35_10: const #0u : u8
        let s_35_10: bool = false;
        // C s_35_11: cast zx s_35_10 -> bv
        let s_35_11: Bits = Bits::new(s_35_10 as u128, 1u16);
        // D s_35_12: cmp-eq s_35_9 s_35_11
        let s_35_12: bool = ((s_35_9) == (s_35_11));
        // D s_35_13: not s_35_12
        let s_35_13: bool = !s_35_12;
        // N s_35_14: branch s_35_13 b38 b36
        if s_35_13 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#61619 <= s_36_0
        fn_state.gs_61619 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#61619:u8
        let s_37_0: bool = fn_state.gs_61619;
        // D s_37_1: write-var gs#61624 <= s_37_0
        fn_state.gs_61624 = s_37_0;
        // N s_37_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#61619 <= s_38_0
        fn_state.gs_61619 = s_38_0;
        // N s_38_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #7u : u8
        let s_39_0: u8 = 7;
        // C s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 8u16);
        // C s_39_2: cast zx s_39_1 -> i
        let s_39_2: i128 = (s_39_1.value() as i128);
        // C s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: cast zx s_39_3 -> i
        let s_39_4: i128 = (i128::try_from(s_39_3).unwrap());
        // C s_39_5: const #432u : u32
        let s_39_5: u32 = 432;
        // D s_39_6: read-reg s_39_5:u8
        let s_39_6: u8 = {
            let value = state.read_register::<u8>(s_39_5 as isize);
            tracer.read_register(s_39_5 as isize, value);
            value
        };
        // D s_39_7: call AArch64_SystemAccessTrap(s_39_6, s_39_4)
        let s_39_7: () = AArch64_SystemAccessTrap(state, tracer, s_39_6, s_39_4);
        // N s_39_8: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var __CPTR_EL2_TFP:u8
        let s_40_0: bool = fn_state.u__CPTR_EL2_TFP;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #1u : u8
        let s_40_2: bool = true;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#61618 <= s_40_4
        fn_state.gs_61618 = s_40_4;
        // N s_40_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: panic
        panic!("{:?}", ());
        // N s_41_1: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var __CPTR_EL3_TFP:u8
        let s_42_0: bool = fn_state.u__CPTR_EL3_TFP;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 1u16);
        // C s_42_2: const #1u : u8
        let s_42_2: bool = true;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: write-var gs#61617 <= s_42_4
        fn_state.gs_61617 = s_42_4;
        // N s_42_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_43_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_43_1: call __IMPDEF_boolean(s_43_0)
        let s_43_1: bool = u__IMPDEF_boolean(state, tracer, s_43_0);
        // D s_43_2: write-var gs#61616 <= s_43_1
        fn_state.gs_61616 = s_43_1;
        // N s_43_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __EDSCR_SDD:u8
        let s_44_0: bool = fn_state.u__EDSCR_SDD;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #1u : u8
        let s_44_2: bool = true;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#61615 <= s_44_4
        fn_state.gs_61615 = s_44_4;
        // N s_44_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #424u : u32
        let s_45_0: u32 = 424;
        // D s_45_1: read-reg s_45_0:u8
        let s_45_1: u8 = {
            let value = state.read_register::<u8>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // C s_45_2: const #2u : u8
        let s_45_2: u8 = 2;
        // D s_45_3: cmp-lt s_45_1 s_45_2
        let s_45_3: bool = ((s_45_1) < (s_45_2));
        // D s_45_4: write-var gs#61614 <= s_45_3
        fn_state.gs_61614 = s_45_3;
        // N s_45_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call Halted(s_46_0)
        let s_46_1: bool = Halted(state, tracer, s_46_0);
        // N s_46_2: branch s_46_1 b94 b47
        if s_46_1 {
            return block_94(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#61628 <= s_47_0
        fn_state.gs_61628 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#61628:u8
        let s_48_0: bool = fn_state.gs_61628;
        // N s_48_1: branch s_48_0 b93 b49
        if s_48_0 {
            return block_93(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#61629 <= s_49_0
        fn_state.gs_61629 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#61629:u8
        let s_50_0: bool = fn_state.gs_61629;
        // N s_50_1: branch s_50_0 b92 b51
        if s_50_0 {
            return block_92(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#61630 <= s_51_0
        fn_state.gs_61630 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#61630:u8
        let s_52_0: bool = fn_state.gs_61630;
        // N s_52_1: branch s_52_0 b91 b53
        if s_52_0 {
            return block_91(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#61631 <= s_53_0
        fn_state.gs_61631 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#61631:u8
        let s_54_0: bool = fn_state.gs_61631;
        // N s_54_1: branch s_54_0 b90 b55
        if s_54_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var __CPACR_EL1_FPEN:u8
        let s_55_0: u8 = fn_state.u__CPACR_EL1_FPEN;
        // C s_55_1: const #0s : i
        let s_55_1: i128 = 0;
        // D s_55_2: cast zx s_55_0 -> bv
        let s_55_2: Bits = Bits::new(s_55_0 as u128, 2u16);
        // C s_55_3: const #1s : i64
        let s_55_3: i64 = 1;
        // C s_55_4: cast zx s_55_3 -> i
        let s_55_4: i128 = (i128::try_from(s_55_3).unwrap());
        // C s_55_5: const #0s : i
        let s_55_5: i128 = 0;
        // C s_55_6: add s_55_5 s_55_4
        let s_55_6: i128 = (s_55_5 + s_55_4);
        // D s_55_7: bit-extract s_55_2 s_55_1 s_55_6
        let s_55_7: Bits = (Bits::new(
            ((s_55_2) >> (s_55_1)).value(),
            u16::try_from(s_55_6).unwrap(),
        ));
        // D s_55_8: cast reint s_55_7 -> u8
        let s_55_8: bool = ((s_55_7.value()) != 0);
        // D s_55_9: cast zx s_55_8 -> bv
        let s_55_9: Bits = Bits::new(s_55_8 as u128, 1u16);
        // C s_55_10: const #0u : u8
        let s_55_10: bool = false;
        // C s_55_11: cast zx s_55_10 -> bv
        let s_55_11: Bits = Bits::new(s_55_10 as u128, 1u16);
        // D s_55_12: cmp-eq s_55_9 s_55_11
        let s_55_12: bool = ((s_55_9) == (s_55_11));
        // D s_55_13: not s_55_12
        let s_55_13: bool = !s_55_12;
        // N s_55_14: branch s_55_13 b89 b56
        if s_55_13 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#61632 <= s_56_0
        fn_state.gs_61632 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#61632:u8
        let s_57_0: bool = fn_state.gs_61632;
        // N s_57_1: branch s_57_0 b88 b58
        if s_57_0 {
            return block_88(state, tracer, fn_state);
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
        // N s_58_2: branch s_58_1 b87 b59
        if s_58_1 {
            return block_87(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#61637 <= s_59_0
        fn_state.gs_61637 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#61637:u8
        let s_60_0: bool = fn_state.gs_61637;
        // N s_60_1: branch s_60_0 b86 b61
        if s_60_0 {
            return block_86(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#61638 <= s_61_0
        fn_state.gs_61638 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#61638:u8
        let s_62_0: bool = fn_state.gs_61638;
        // N s_62_1: branch s_62_0 b85 b63
        if s_62_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call EL2Enabled(s_63_0)
        let s_63_1: bool = EL2Enabled(state, tracer, s_63_0);
        // N s_63_2: branch s_63_1 b84 b64
        if s_63_1 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #0u : u8
        let s_64_0: bool = false;
        // D s_64_1: write-var gs#61639 <= s_64_0
        fn_state.gs_61639 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#61639:u8
        let s_65_0: bool = fn_state.gs_61639;
        // N s_65_1: branch s_65_0 b80 b66
        if s_65_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // D s_66_1: write-var gs#61645 <= s_66_0
        fn_state.gs_61645 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#61645:u8
        let s_67_0: bool = fn_state.gs_61645;
        // N s_67_1: branch s_67_0 b79 b68
        if s_67_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #424u : u32
        let s_68_0: u32 = 424;
        // D s_68_1: read-reg s_68_0:u8
        let s_68_1: u8 = {
            let value = state.read_register::<u8>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // C s_68_2: const #2u : u8
        let s_68_2: u8 = 2;
        // D s_68_3: cmp-lt s_68_1 s_68_2
        let s_68_3: bool = ((s_68_1) < (s_68_2));
        // N s_68_4: branch s_68_3 b78 b69
        if s_68_3 {
            return block_78(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#61646 <= s_69_0
        fn_state.gs_61646 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#61646:u8
        let s_70_0: bool = fn_state.gs_61646;
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
        // C s_71_0: const #64s : i64
        let s_71_0: i64 = 64;
        // C s_71_1: const #12920u : u32
        let s_71_1: u32 = 12920;
        // D s_71_2: read-reg s_71_1:struct
        let s_71_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_71_1 as isize);
            tracer.read_register(s_71_1 as isize, value);
            value
        };
        // D s_71_3: call __get_FPCR(s_71_2)
        let s_71_3: ProductType5c790c8ef59cc8b2 = u__get_FPCR(state, tracer, s_71_2);
        // D s_71_4: write-var ga#65505 <= s_71_3
        fn_state.ga_65505 = s_71_3;
        // D s_71_5: read-var ga#65505.0:struct
        let s_71_5: u64 = fn_state.ga_65505._0;
        // D s_71_6: cast zx s_71_5 -> bv
        let s_71_6: Bits = Bits::new(s_71_5 as u128, 64u16);
        // D s_71_7: read-var t:i
        let s_71_7: i128 = fn_state.t;
        // D s_71_8: call X_set(s_71_7, s_71_0, s_71_6)
        let s_71_8: () = X_set(state, tracer, s_71_7, s_71_0, s_71_6);
        // N s_71_9: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call Halted(s_72_0)
        let s_72_1: bool = Halted(state, tracer, s_72_0);
        // N s_72_2: branch s_72_1 b77 b73
        if s_72_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_73_1: write-var gs#61648 <= s_73_0
        fn_state.gs_61648 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#61648:u8
        let s_74_0: bool = fn_state.gs_61648;
        // N s_74_1: branch s_74_0 b76 b75
        if s_74_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #7u : u8
        let s_75_0: u8 = 7;
        // C s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 8u16);
        // C s_75_2: cast zx s_75_1 -> i
        let s_75_2: i128 = (s_75_1.value() as i128);
        // C s_75_3: cast reint s_75_2 -> i64
        let s_75_3: i64 = (s_75_2 as i64);
        // C s_75_4: cast zx s_75_3 -> i
        let s_75_4: i128 = (i128::try_from(s_75_3).unwrap());
        // C s_75_5: const #424u : u32
        let s_75_5: u32 = 424;
        // D s_75_6: read-reg s_75_5:u8
        let s_75_6: u8 = {
            let value = state.read_register::<u8>(s_75_5 as isize);
            tracer.read_register(s_75_5 as isize, value);
            value
        };
        // D s_75_7: call AArch64_SystemAccessTrap(s_75_6, s_75_4)
        let s_75_7: () = AArch64_SystemAccessTrap(state, tracer, s_75_6, s_75_4);
        // N s_75_8: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_76_0: panic
        panic!("{:?}", ());
        // N s_76_1: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var __EDSCR_SDD:u8
        let s_77_0: bool = fn_state.u__EDSCR_SDD;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 1u16);
        // C s_77_2: const #1u : u8
        let s_77_2: bool = true;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: write-var gs#61648 <= s_77_4
        fn_state.gs_61648 = s_77_4;
        // N s_77_6: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var __CPTR_EL3_TFP:u8
        let s_78_0: bool = fn_state.u__CPTR_EL3_TFP;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #1u : u8
        let s_78_2: bool = true;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#61646 <= s_78_4
        fn_state.gs_61646 = s_78_4;
        // N s_78_6: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #7u : u8
        let s_79_0: u8 = 7;
        // C s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 8u16);
        // C s_79_2: cast zx s_79_1 -> i
        let s_79_2: i128 = (s_79_1.value() as i128);
        // C s_79_3: cast reint s_79_2 -> i64
        let s_79_3: i64 = (s_79_2 as i64);
        // C s_79_4: cast zx s_79_3 -> i
        let s_79_4: i128 = (i128::try_from(s_79_3).unwrap());
        // C s_79_5: const #432u : u32
        let s_79_5: u32 = 432;
        // D s_79_6: read-reg s_79_5:u8
        let s_79_6: u8 = {
            let value = state.read_register::<u8>(s_79_5 as isize);
            tracer.read_register(s_79_5 as isize, value);
            value
        };
        // D s_79_7: call AArch64_SystemAccessTrap(s_79_6, s_79_4)
        let s_79_7: () = AArch64_SystemAccessTrap(state, tracer, s_79_6, s_79_4);
        // N s_79_8: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __CPTR_EL2_FPEN:u8
        let s_80_0: u8 = fn_state.u__CPTR_EL2_FPEN;
        // C s_80_1: const #0s : i
        let s_80_1: i128 = 0;
        // D s_80_2: cast zx s_80_0 -> bv
        let s_80_2: Bits = Bits::new(s_80_0 as u128, 2u16);
        // C s_80_3: const #1s : i64
        let s_80_3: i64 = 1;
        // C s_80_4: cast zx s_80_3 -> i
        let s_80_4: i128 = (i128::try_from(s_80_3).unwrap());
        // C s_80_5: const #0s : i
        let s_80_5: i128 = 0;
        // C s_80_6: add s_80_5 s_80_4
        let s_80_6: i128 = (s_80_5 + s_80_4);
        // D s_80_7: bit-extract s_80_2 s_80_1 s_80_6
        let s_80_7: Bits = (Bits::new(
            ((s_80_2) >> (s_80_1)).value(),
            u16::try_from(s_80_6).unwrap(),
        ));
        // D s_80_8: cast reint s_80_7 -> u8
        let s_80_8: bool = ((s_80_7.value()) != 0);
        // D s_80_9: cast zx s_80_8 -> bv
        let s_80_9: Bits = Bits::new(s_80_8 as u128, 1u16);
        // C s_80_10: const #0u : u8
        let s_80_10: bool = false;
        // C s_80_11: cast zx s_80_10 -> bv
        let s_80_11: Bits = Bits::new(s_80_10 as u128, 1u16);
        // D s_80_12: cmp-eq s_80_9 s_80_11
        let s_80_12: bool = ((s_80_9) == (s_80_11));
        // D s_80_13: not s_80_12
        let s_80_13: bool = !s_80_12;
        // N s_80_14: branch s_80_13 b83 b81
        if s_80_13 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #1u : u8
        let s_81_0: bool = true;
        // D s_81_1: write-var gs#61640 <= s_81_0
        fn_state.gs_61640 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#61640:u8
        let s_82_0: bool = fn_state.gs_61640;
        // D s_82_1: write-var gs#61645 <= s_82_0
        fn_state.gs_61645 = s_82_0;
        // N s_82_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#61640 <= s_83_0
        fn_state.gs_61640 = s_83_0;
        // N s_83_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __HCR_EL2_E2H:u8
        let s_84_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#61639 <= s_84_4
        fn_state.gs_61639 = s_84_4;
        // N s_84_6: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #7u : u8
        let s_85_0: u8 = 7;
        // C s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 8u16);
        // C s_85_2: cast zx s_85_1 -> i
        let s_85_2: i128 = (s_85_1.value() as i128);
        // C s_85_3: cast reint s_85_2 -> i64
        let s_85_3: i64 = (s_85_2 as i64);
        // C s_85_4: cast zx s_85_3 -> i
        let s_85_4: i128 = (i128::try_from(s_85_3).unwrap());
        // C s_85_5: const #432u : u32
        let s_85_5: u32 = 432;
        // D s_85_6: read-reg s_85_5:u8
        let s_85_6: u8 = {
            let value = state.read_register::<u8>(s_85_5 as isize);
            tracer.read_register(s_85_5 as isize, value);
            value
        };
        // D s_85_7: call AArch64_SystemAccessTrap(s_85_6, s_85_4)
        let s_85_7: () = AArch64_SystemAccessTrap(state, tracer, s_85_6, s_85_4);
        // N s_85_8: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var __CPTR_EL2_TFP:u8
        let s_86_0: bool = fn_state.u__CPTR_EL2_TFP;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 1u16);
        // C s_86_2: const #1u : u8
        let s_86_2: bool = true;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: write-var gs#61638 <= s_86_4
        fn_state.gs_61638 = s_86_4;
        // N s_86_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var __HCR_EL2_E2H:u8
        let s_87_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 1u16);
        // C s_87_2: const #1u : u8
        let s_87_2: bool = true;
        // C s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 1u16);
        // D s_87_4: cmp-ne s_87_1 s_87_3
        let s_87_4: bool = ((s_87_1) != (s_87_3));
        // D s_87_5: write-var gs#61637 <= s_87_4
        fn_state.gs_61637 = s_87_4;
        // N s_87_6: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #7u : u8
        let s_88_0: u8 = 7;
        // C s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 8u16);
        // C s_88_2: cast zx s_88_1 -> i
        let s_88_2: i128 = (s_88_1.value() as i128);
        // C s_88_3: cast reint s_88_2 -> i64
        let s_88_3: i64 = (s_88_2 as i64);
        // C s_88_4: cast zx s_88_3 -> i
        let s_88_4: i128 = (i128::try_from(s_88_3).unwrap());
        // C s_88_5: const #440u : u32
        let s_88_5: u32 = 440;
        // D s_88_6: read-reg s_88_5:u8
        let s_88_6: u8 = {
            let value = state.read_register::<u8>(s_88_5 as isize);
            tracer.read_register(s_88_5 as isize, value);
            value
        };
        // D s_88_7: call AArch64_SystemAccessTrap(s_88_6, s_88_4)
        let s_88_7: () = AArch64_SystemAccessTrap(state, tracer, s_88_6, s_88_4);
        // N s_88_8: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#61632 <= s_89_0
        fn_state.gs_61632 = s_89_0;
        // N s_89_2: jump b57
        return block_57(state, tracer, fn_state);
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
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var __CPTR_EL3_TFP:u8
        let s_91_0: bool = fn_state.u__CPTR_EL3_TFP;
        // D s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 1u16);
        // C s_91_2: const #1u : u8
        let s_91_2: bool = true;
        // C s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // D s_91_4: cmp-eq s_91_1 s_91_3
        let s_91_4: bool = ((s_91_1) == (s_91_3));
        // D s_91_5: write-var gs#61631 <= s_91_4
        fn_state.gs_61631 = s_91_4;
        // N s_91_6: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_92_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_92_1: call __IMPDEF_boolean(s_92_0)
        let s_92_1: bool = u__IMPDEF_boolean(state, tracer, s_92_0);
        // D s_92_2: write-var gs#61630 <= s_92_1
        fn_state.gs_61630 = s_92_1;
        // N s_92_3: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var __EDSCR_SDD:u8
        let s_93_0: bool = fn_state.u__EDSCR_SDD;
        // D s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 1u16);
        // C s_93_2: const #1u : u8
        let s_93_2: bool = true;
        // C s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // D s_93_4: cmp-eq s_93_1 s_93_3
        let s_93_4: bool = ((s_93_1) == (s_93_3));
        // D s_93_5: write-var gs#61629 <= s_93_4
        fn_state.gs_61629 = s_93_4;
        // N s_93_6: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #424u : u32
        let s_94_0: u32 = 424;
        // D s_94_1: read-reg s_94_0:u8
        let s_94_1: u8 = {
            let value = state.read_register::<u8>(s_94_0 as isize);
            tracer.read_register(s_94_0 as isize, value);
            value
        };
        // C s_94_2: const #2u : u8
        let s_94_2: u8 = 2;
        // D s_94_3: cmp-lt s_94_1 s_94_2
        let s_94_3: bool = ((s_94_1) < (s_94_2));
        // D s_94_4: write-var gs#61628 <= s_94_3
        fn_state.gs_61628 = s_94_3;
        // N s_94_5: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #() : ()
        let s_95_0: () = ();
        // S s_95_1: call Halted(s_95_0)
        let s_95_1: bool = Halted(state, tracer, s_95_0);
        // N s_95_2: branch s_95_1 b159 b96
        if s_95_1 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var gs#61649 <= s_96_0
        fn_state.gs_61649 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#61649:u8
        let s_97_0: bool = fn_state.gs_61649;
        // N s_97_1: branch s_97_0 b158 b98
        if s_97_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#61650 <= s_98_0
        fn_state.gs_61650 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#61650:u8
        let s_99_0: bool = fn_state.gs_61650;
        // N s_99_1: branch s_99_0 b157 b100
        if s_99_0 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #0u : u8
        let s_100_0: bool = false;
        // D s_100_1: write-var gs#61651 <= s_100_0
        fn_state.gs_61651 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#61651:u8
        let s_101_0: bool = fn_state.gs_61651;
        // N s_101_1: branch s_101_0 b156 b102
        if s_101_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #0u : u8
        let s_102_0: bool = false;
        // D s_102_1: write-var gs#61652 <= s_102_0
        fn_state.gs_61652 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#61652:u8
        let s_103_0: bool = fn_state.gs_61652;
        // N s_103_1: branch s_103_0 b155 b104
        if s_103_0 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #() : ()
        let s_104_0: () = ();
        // S s_104_1: call EL2Enabled(s_104_0)
        let s_104_1: bool = EL2Enabled(state, tracer, s_104_0);
        // N s_104_2: branch s_104_1 b154 b105
        if s_104_1 {
            return block_154(state, tracer, fn_state);
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
        // D s_105_1: write-var gs#61653 <= s_105_0
        fn_state.gs_61653 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#61653:u8
        let s_106_0: bool = fn_state.gs_61653;
        // D s_106_1: not s_106_0
        let s_106_1: bool = !s_106_0;
        // N s_106_2: branch s_106_1 b153 b107
        if s_106_1 {
            return block_153(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#61654 <= s_107_0
        fn_state.gs_61654 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#61654:u8
        let s_108_0: bool = fn_state.gs_61654;
        // N s_108_1: branch s_108_0 b147 b109
        if s_108_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call EL2Enabled(s_109_0)
        let s_109_1: bool = EL2Enabled(state, tracer, s_109_0);
        // N s_109_2: branch s_109_1 b146 b110
        if s_109_1 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #0u : u8
        let s_110_0: bool = false;
        // D s_110_1: write-var gs#61655 <= s_110_0
        fn_state.gs_61655 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#61655:u8
        let s_111_0: bool = fn_state.gs_61655;
        // N s_111_1: branch s_111_0 b145 b112
        if s_111_0 {
            return block_145(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#61656 <= s_112_0
        fn_state.gs_61656 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#61656:u8
        let s_113_0: bool = fn_state.gs_61656;
        // N s_113_1: branch s_113_0 b144 b114
        if s_113_0 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #() : ()
        let s_114_0: () = ();
        // S s_114_1: call EL2Enabled(s_114_0)
        let s_114_1: bool = EL2Enabled(state, tracer, s_114_0);
        // N s_114_2: branch s_114_1 b143 b115
        if s_114_1 {
            return block_143(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#61657 <= s_115_0
        fn_state.gs_61657 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#61657:u8
        let s_116_0: bool = fn_state.gs_61657;
        // N s_116_1: branch s_116_0 b139 b117
        if s_116_0 {
            return block_139(state, tracer, fn_state);
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
        // D s_117_1: write-var gs#61663 <= s_117_0
        fn_state.gs_61663 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#61663:u8
        let s_118_0: bool = fn_state.gs_61663;
        // N s_118_1: branch s_118_0 b138 b119
        if s_118_0 {
            return block_138(state, tracer, fn_state);
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
        // N s_119_2: branch s_119_1 b137 b120
        if s_119_1 {
            return block_137(state, tracer, fn_state);
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
        // D s_120_1: write-var gs#61664 <= s_120_0
        fn_state.gs_61664 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#61664:u8
        let s_121_0: bool = fn_state.gs_61664;
        // N s_121_1: branch s_121_0 b136 b122
        if s_121_0 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #0u : u8
        let s_122_0: bool = false;
        // D s_122_1: write-var gs#61665 <= s_122_0
        fn_state.gs_61665 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#61665:u8
        let s_123_0: bool = fn_state.gs_61665;
        // N s_123_1: branch s_123_0 b135 b124
        if s_123_0 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #424u : u32
        let s_124_0: u32 = 424;
        // D s_124_1: read-reg s_124_0:u8
        let s_124_1: u8 = {
            let value = state.read_register::<u8>(s_124_0 as isize);
            tracer.read_register(s_124_0 as isize, value);
            value
        };
        // C s_124_2: const #2u : u8
        let s_124_2: u8 = 2;
        // D s_124_3: cmp-lt s_124_1 s_124_2
        let s_124_3: bool = ((s_124_1) < (s_124_2));
        // N s_124_4: branch s_124_3 b134 b125
        if s_124_3 {
            return block_134(state, tracer, fn_state);
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
        // D s_125_1: write-var gs#61666 <= s_125_0
        fn_state.gs_61666 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#61666:u8
        let s_126_0: bool = fn_state.gs_61666;
        // N s_126_1: branch s_126_0 b128 b127
        if s_126_0 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #64s : i64
        let s_127_0: i64 = 64;
        // C s_127_1: const #12920u : u32
        let s_127_1: u32 = 12920;
        // D s_127_2: read-reg s_127_1:struct
        let s_127_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_127_1 as isize);
            tracer.read_register(s_127_1 as isize, value);
            value
        };
        // D s_127_3: call __get_FPCR(s_127_2)
        let s_127_3: ProductType5c790c8ef59cc8b2 = u__get_FPCR(state, tracer, s_127_2);
        // D s_127_4: write-var ga#65477 <= s_127_3
        fn_state.ga_65477 = s_127_3;
        // D s_127_5: read-var ga#65477.0:struct
        let s_127_5: u64 = fn_state.ga_65477._0;
        // D s_127_6: cast zx s_127_5 -> bv
        let s_127_6: Bits = Bits::new(s_127_5 as u128, 64u16);
        // D s_127_7: read-var t:i
        let s_127_7: i128 = fn_state.t;
        // D s_127_8: call X_set(s_127_7, s_127_0, s_127_6)
        let s_127_8: () = X_set(state, tracer, s_127_7, s_127_0, s_127_6);
        // N s_127_9: return
        return;
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #() : ()
        let s_128_0: () = ();
        // S s_128_1: call Halted(s_128_0)
        let s_128_1: bool = Halted(state, tracer, s_128_0);
        // N s_128_2: branch s_128_1 b133 b129
        if s_128_1 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #0u : u8
        let s_129_0: bool = false;
        // D s_129_1: write-var gs#61668 <= s_129_0
        fn_state.gs_61668 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#61668:u8
        let s_130_0: bool = fn_state.gs_61668;
        // N s_130_1: branch s_130_0 b132 b131
        if s_130_0 {
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
        // C s_131_0: const #7u : u8
        let s_131_0: u8 = 7;
        // C s_131_1: cast zx s_131_0 -> bv
        let s_131_1: Bits = Bits::new(s_131_0 as u128, 8u16);
        // C s_131_2: cast zx s_131_1 -> i
        let s_131_2: i128 = (s_131_1.value() as i128);
        // C s_131_3: cast reint s_131_2 -> i64
        let s_131_3: i64 = (s_131_2 as i64);
        // C s_131_4: cast zx s_131_3 -> i
        let s_131_4: i128 = (i128::try_from(s_131_3).unwrap());
        // C s_131_5: const #424u : u32
        let s_131_5: u32 = 424;
        // D s_131_6: read-reg s_131_5:u8
        let s_131_6: u8 = {
            let value = state.read_register::<u8>(s_131_5 as isize);
            tracer.read_register(s_131_5 as isize, value);
            value
        };
        // D s_131_7: call AArch64_SystemAccessTrap(s_131_6, s_131_4)
        let s_131_7: () = AArch64_SystemAccessTrap(state, tracer, s_131_6, s_131_4);
        // N s_131_8: return
        return;
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_132_0: panic
        panic!("{:?}", ());
        // N s_132_1: return
        return;
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var __EDSCR_SDD:u8
        let s_133_0: bool = fn_state.u__EDSCR_SDD;
        // D s_133_1: cast zx s_133_0 -> bv
        let s_133_1: Bits = Bits::new(s_133_0 as u128, 1u16);
        // C s_133_2: const #1u : u8
        let s_133_2: bool = true;
        // C s_133_3: cast zx s_133_2 -> bv
        let s_133_3: Bits = Bits::new(s_133_2 as u128, 1u16);
        // D s_133_4: cmp-eq s_133_1 s_133_3
        let s_133_4: bool = ((s_133_1) == (s_133_3));
        // D s_133_5: write-var gs#61668 <= s_133_4
        fn_state.gs_61668 = s_133_4;
        // N s_133_6: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var __CPTR_EL3_TFP:u8
        let s_134_0: bool = fn_state.u__CPTR_EL3_TFP;
        // D s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 1u16);
        // C s_134_2: const #1u : u8
        let s_134_2: bool = true;
        // C s_134_3: cast zx s_134_2 -> bv
        let s_134_3: Bits = Bits::new(s_134_2 as u128, 1u16);
        // D s_134_4: cmp-eq s_134_1 s_134_3
        let s_134_4: bool = ((s_134_1) == (s_134_3));
        // D s_134_5: write-var gs#61666 <= s_134_4
        fn_state.gs_61666 = s_134_4;
        // N s_134_6: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #7u : u8
        let s_135_0: u8 = 7;
        // C s_135_1: cast zx s_135_0 -> bv
        let s_135_1: Bits = Bits::new(s_135_0 as u128, 8u16);
        // C s_135_2: cast zx s_135_1 -> i
        let s_135_2: i128 = (s_135_1.value() as i128);
        // C s_135_3: cast reint s_135_2 -> i64
        let s_135_3: i64 = (s_135_2 as i64);
        // C s_135_4: cast zx s_135_3 -> i
        let s_135_4: i128 = (i128::try_from(s_135_3).unwrap());
        // C s_135_5: const #432u : u32
        let s_135_5: u32 = 432;
        // D s_135_6: read-reg s_135_5:u8
        let s_135_6: u8 = {
            let value = state.read_register::<u8>(s_135_5 as isize);
            tracer.read_register(s_135_5 as isize, value);
            value
        };
        // D s_135_7: call AArch64_SystemAccessTrap(s_135_6, s_135_4)
        let s_135_7: () = AArch64_SystemAccessTrap(state, tracer, s_135_6, s_135_4);
        // N s_135_8: return
        return;
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var __CPTR_EL2_TFP:u8
        let s_136_0: bool = fn_state.u__CPTR_EL2_TFP;
        // D s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 1u16);
        // C s_136_2: const #1u : u8
        let s_136_2: bool = true;
        // C s_136_3: cast zx s_136_2 -> bv
        let s_136_3: Bits = Bits::new(s_136_2 as u128, 1u16);
        // D s_136_4: cmp-eq s_136_1 s_136_3
        let s_136_4: bool = ((s_136_1) == (s_136_3));
        // D s_136_5: write-var gs#61665 <= s_136_4
        fn_state.gs_61665 = s_136_4;
        // N s_136_6: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var __HCR_EL2_E2H:u8
        let s_137_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 1u16);
        // C s_137_2: const #1u : u8
        let s_137_2: bool = true;
        // C s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 1u16);
        // D s_137_4: cmp-ne s_137_1 s_137_3
        let s_137_4: bool = ((s_137_1) != (s_137_3));
        // D s_137_5: write-var gs#61664 <= s_137_4
        fn_state.gs_61664 = s_137_4;
        // N s_137_6: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #7u : u8
        let s_138_0: u8 = 7;
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
        // D s_139_0: read-var __CPTR_EL2_FPEN:u8
        let s_139_0: u8 = fn_state.u__CPTR_EL2_FPEN;
        // C s_139_1: const #0s : i
        let s_139_1: i128 = 0;
        // D s_139_2: cast zx s_139_0 -> bv
        let s_139_2: Bits = Bits::new(s_139_0 as u128, 2u16);
        // C s_139_3: const #1s : i64
        let s_139_3: i64 = 1;
        // C s_139_4: cast zx s_139_3 -> i
        let s_139_4: i128 = (i128::try_from(s_139_3).unwrap());
        // C s_139_5: const #0s : i
        let s_139_5: i128 = 0;
        // C s_139_6: add s_139_5 s_139_4
        let s_139_6: i128 = (s_139_5 + s_139_4);
        // D s_139_7: bit-extract s_139_2 s_139_1 s_139_6
        let s_139_7: Bits = (Bits::new(
            ((s_139_2) >> (s_139_1)).value(),
            u16::try_from(s_139_6).unwrap(),
        ));
        // D s_139_8: cast reint s_139_7 -> u8
        let s_139_8: bool = ((s_139_7.value()) != 0);
        // D s_139_9: cast zx s_139_8 -> bv
        let s_139_9: Bits = Bits::new(s_139_8 as u128, 1u16);
        // C s_139_10: const #0u : u8
        let s_139_10: bool = false;
        // C s_139_11: cast zx s_139_10 -> bv
        let s_139_11: Bits = Bits::new(s_139_10 as u128, 1u16);
        // D s_139_12: cmp-eq s_139_9 s_139_11
        let s_139_12: bool = ((s_139_9) == (s_139_11));
        // D s_139_13: not s_139_12
        let s_139_13: bool = !s_139_12;
        // N s_139_14: branch s_139_13 b142 b140
        if s_139_13 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #1u : u8
        let s_140_0: bool = true;
        // D s_140_1: write-var gs#61658 <= s_140_0
        fn_state.gs_61658 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#61658:u8
        let s_141_0: bool = fn_state.gs_61658;
        // D s_141_1: write-var gs#61663 <= s_141_0
        fn_state.gs_61663 = s_141_0;
        // N s_141_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #0u : u8
        let s_142_0: bool = false;
        // D s_142_1: write-var gs#61658 <= s_142_0
        fn_state.gs_61658 = s_142_0;
        // N s_142_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var __HCR_EL2_E2H:u8
        let s_143_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_143_1: cast zx s_143_0 -> bv
        let s_143_1: Bits = Bits::new(s_143_0 as u128, 1u16);
        // C s_143_2: const #1u : u8
        let s_143_2: bool = true;
        // C s_143_3: cast zx s_143_2 -> bv
        let s_143_3: Bits = Bits::new(s_143_2 as u128, 1u16);
        // D s_143_4: cmp-eq s_143_1 s_143_3
        let s_143_4: bool = ((s_143_1) == (s_143_3));
        // D s_143_5: write-var gs#61657 <= s_143_4
        fn_state.gs_61657 = s_143_4;
        // N s_143_6: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #7u : u8
        let s_144_0: u8 = 7;
        // C s_144_1: cast zx s_144_0 -> bv
        let s_144_1: Bits = Bits::new(s_144_0 as u128, 8u16);
        // C s_144_2: cast zx s_144_1 -> i
        let s_144_2: i128 = (s_144_1.value() as i128);
        // C s_144_3: cast reint s_144_2 -> i64
        let s_144_3: i64 = (s_144_2 as i64);
        // C s_144_4: cast zx s_144_3 -> i
        let s_144_4: i128 = (i128::try_from(s_144_3).unwrap());
        // C s_144_5: const #432u : u32
        let s_144_5: u32 = 432;
        // D s_144_6: read-reg s_144_5:u8
        let s_144_6: u8 = {
            let value = state.read_register::<u8>(s_144_5 as isize);
            tracer.read_register(s_144_5 as isize, value);
            value
        };
        // D s_144_7: call AArch64_SystemAccessTrap(s_144_6, s_144_4)
        let s_144_7: () = AArch64_SystemAccessTrap(state, tracer, s_144_6, s_144_4);
        // N s_144_8: return
        return;
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var __CPTR_EL2_FPEN:u8
        let s_145_0: u8 = fn_state.u__CPTR_EL2_FPEN;
        // D s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 2u16);
        // C s_145_2: const #3u : u8
        let s_145_2: u8 = 3;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 2u16);
        // D s_145_4: cmp-ne s_145_1 s_145_3
        let s_145_4: bool = ((s_145_1) != (s_145_3));
        // D s_145_5: write-var gs#61656 <= s_145_4
        fn_state.gs_61656 = s_145_4;
        // N s_145_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #102552u : u32
        let s_146_0: u32 = 102552;
        // D s_146_1: read-reg s_146_0:struct
        let s_146_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_146_0 as isize);
            tracer.read_register(s_146_0 as isize, value);
            value
        };
        // D s_146_2: call _get_HCR_EL2_Type_E2H(s_146_1)
        let s_146_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_146_1);
        // C s_146_3: const #102552u : u32
        let s_146_3: u32 = 102552;
        // D s_146_4: read-reg s_146_3:struct
        let s_146_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_146_3 as isize);
            tracer.read_register(s_146_3 as isize, value);
            value
        };
        // D s_146_5: call _get_HCR_EL2_Type_TGE(s_146_4)
        let s_146_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_146_4);
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
        // C s_146_18: const #3u : u8
        let s_146_18: u8 = 3;
        // C s_146_19: cast zx s_146_18 -> bv
        let s_146_19: Bits = Bits::new(s_146_18 as u128, 2u16);
        // D s_146_20: cmp-eq s_146_17 s_146_19
        let s_146_20: bool = ((s_146_17) == (s_146_19));
        // D s_146_21: write-var gs#61655 <= s_146_20
        fn_state.gs_61655 = s_146_20;
        // N s_146_22: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #() : ()
        let s_147_0: () = ();
        // S s_147_1: call EL2Enabled(s_147_0)
        let s_147_1: bool = EL2Enabled(state, tracer, s_147_0);
        // N s_147_2: branch s_147_1 b152 b148
        if s_147_1 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #0u : u8
        let s_148_0: bool = false;
        // D s_148_1: write-var gs#61669 <= s_148_0
        fn_state.gs_61669 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#61669:u8
        let s_149_0: bool = fn_state.gs_61669;
        // N s_149_1: branch s_149_0 b151 b150
        if s_149_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #7u : u8
        let s_150_0: u8 = 7;
        // C s_150_1: cast zx s_150_0 -> bv
        let s_150_1: Bits = Bits::new(s_150_0 as u128, 8u16);
        // C s_150_2: cast zx s_150_1 -> i
        let s_150_2: i128 = (s_150_1.value() as i128);
        // C s_150_3: cast reint s_150_2 -> i64
        let s_150_3: i64 = (s_150_2 as i64);
        // C s_150_4: cast zx s_150_3 -> i
        let s_150_4: i128 = (i128::try_from(s_150_3).unwrap());
        // C s_150_5: const #440u : u32
        let s_150_5: u32 = 440;
        // D s_150_6: read-reg s_150_5:u8
        let s_150_6: u8 = {
            let value = state.read_register::<u8>(s_150_5 as isize);
            tracer.read_register(s_150_5 as isize, value);
            value
        };
        // D s_150_7: call AArch64_SystemAccessTrap(s_150_6, s_150_4)
        let s_150_7: () = AArch64_SystemAccessTrap(state, tracer, s_150_6, s_150_4);
        // N s_150_8: return
        return;
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #0u : u8
        let s_151_0: u8 = 0;
        // C s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 8u16);
        // C s_151_2: cast zx s_151_1 -> i
        let s_151_2: i128 = (s_151_1.value() as i128);
        // C s_151_3: cast reint s_151_2 -> i64
        let s_151_3: i64 = (s_151_2 as i64);
        // C s_151_4: cast zx s_151_3 -> i
        let s_151_4: i128 = (i128::try_from(s_151_3).unwrap());
        // C s_151_5: const #432u : u32
        let s_151_5: u32 = 432;
        // D s_151_6: read-reg s_151_5:u8
        let s_151_6: u8 = {
            let value = state.read_register::<u8>(s_151_5 as isize);
            tracer.read_register(s_151_5 as isize, value);
            value
        };
        // D s_151_7: call AArch64_SystemAccessTrap(s_151_6, s_151_4)
        let s_151_7: () = AArch64_SystemAccessTrap(state, tracer, s_151_6, s_151_4);
        // N s_151_8: return
        return;
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var __HCR_EL2_TGE:u8
        let s_152_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_152_1: cast zx s_152_0 -> bv
        let s_152_1: Bits = Bits::new(s_152_0 as u128, 1u16);
        // C s_152_2: const #1u : u8
        let s_152_2: bool = true;
        // C s_152_3: cast zx s_152_2 -> bv
        let s_152_3: Bits = Bits::new(s_152_2 as u128, 1u16);
        // D s_152_4: cmp-eq s_152_1 s_152_3
        let s_152_4: bool = ((s_152_1) == (s_152_3));
        // D s_152_5: write-var gs#61669 <= s_152_4
        fn_state.gs_61669 = s_152_4;
        // N s_152_6: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var __CPACR_EL1_FPEN:u8
        let s_153_0: u8 = fn_state.u__CPACR_EL1_FPEN;
        // D s_153_1: cast zx s_153_0 -> bv
        let s_153_1: Bits = Bits::new(s_153_0 as u128, 2u16);
        // C s_153_2: const #3u : u8
        let s_153_2: u8 = 3;
        // C s_153_3: cast zx s_153_2 -> bv
        let s_153_3: Bits = Bits::new(s_153_2 as u128, 2u16);
        // D s_153_4: cmp-ne s_153_1 s_153_3
        let s_153_4: bool = ((s_153_1) != (s_153_3));
        // D s_153_5: write-var gs#61654 <= s_153_4
        fn_state.gs_61654 = s_153_4;
        // N s_153_6: jump b108
        return block_108(state, tracer, fn_state);
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
        // D s_154_20: cmp-eq s_154_17 s_154_19
        let s_154_20: bool = ((s_154_17) == (s_154_19));
        // D s_154_21: write-var gs#61653 <= s_154_20
        fn_state.gs_61653 = s_154_20;
        // N s_154_22: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_155_0: panic
        panic!("{:?}", ());
        // N s_155_1: return
        return;
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var __CPTR_EL3_TFP:u8
        let s_156_0: bool = fn_state.u__CPTR_EL3_TFP;
        // D s_156_1: cast zx s_156_0 -> bv
        let s_156_1: Bits = Bits::new(s_156_0 as u128, 1u16);
        // C s_156_2: const #1u : u8
        let s_156_2: bool = true;
        // C s_156_3: cast zx s_156_2 -> bv
        let s_156_3: Bits = Bits::new(s_156_2 as u128, 1u16);
        // D s_156_4: cmp-eq s_156_1 s_156_3
        let s_156_4: bool = ((s_156_1) == (s_156_3));
        // D s_156_5: write-var gs#61652 <= s_156_4
        fn_state.gs_61652 = s_156_4;
        // N s_156_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_157_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_157_1: call __IMPDEF_boolean(s_157_0)
        let s_157_1: bool = u__IMPDEF_boolean(state, tracer, s_157_0);
        // D s_157_2: write-var gs#61651 <= s_157_1
        fn_state.gs_61651 = s_157_1;
        // N s_157_3: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var __EDSCR_SDD:u8
        let s_158_0: bool = fn_state.u__EDSCR_SDD;
        // D s_158_1: cast zx s_158_0 -> bv
        let s_158_1: Bits = Bits::new(s_158_0 as u128, 1u16);
        // C s_158_2: const #1u : u8
        let s_158_2: bool = true;
        // C s_158_3: cast zx s_158_2 -> bv
        let s_158_3: Bits = Bits::new(s_158_2 as u128, 1u16);
        // D s_158_4: cmp-eq s_158_1 s_158_3
        let s_158_4: bool = ((s_158_1) == (s_158_3));
        // D s_158_5: write-var gs#61650 <= s_158_4
        fn_state.gs_61650 = s_158_4;
        // N s_158_6: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #424u : u32
        let s_159_0: u32 = 424;
        // D s_159_1: read-reg s_159_0:u8
        let s_159_1: u8 = {
            let value = state.read_register::<u8>(s_159_0 as isize);
            tracer.read_register(s_159_0 as isize, value);
            value
        };
        // C s_159_2: const #2u : u8
        let s_159_2: u8 = 2;
        // D s_159_3: cmp-lt s_159_1 s_159_2
        let s_159_3: bool = ((s_159_1) < (s_159_2));
        // D s_159_4: write-var gs#61649 <= s_159_3
        fn_state.gs_61649 = s_159_3;
        // N s_159_5: jump b97
        return block_97(state, tracer, fn_state);
    }
}
