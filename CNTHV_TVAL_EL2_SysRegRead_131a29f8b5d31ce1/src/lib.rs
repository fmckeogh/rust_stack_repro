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
use u_get_CNTHV_CTL_EL2_Type_ENABLE::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_E2H::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use Mk_CNTHV_TVAL_EL2_Type::*;
use u__get_CNTHV_TVAL_EL2::*;
use u_get_CNTHVS_CTL_EL2_Type_ENABLE::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_CNTHCTL_EL2_Type_EL1TVT::*;
use X_set::*;
use u__UNKNOWN_bits::*;
use ELUsingAArch32::*;
use u_get_CNTHCTL_EL2_Type_EL0VTEN::*;
use CNTVOFF_read::*;
use PhysicalCountInt::*;
use u_get_CNTKCTL_EL1_Type_EL0VTEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_CNTV_CTL_EL0_Type_ENABLE::*;
use common::*;
pub fn CNTHV_TVAL_EL2_SysRegRead_131a29f8b5d31ce1<T: Tracer>(
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
        ga_56157: ProductType5c790c8ef59cc8b2,
        ga_56204: ProductType5c790c8ef59cc8b2,
        u__CNTHVS_CTL_EL2_ENABLE: bool,
        ga_56299: ProductType5c790c8ef59cc8b2,
        gs_57943: bool,
        ga_56215: ProductType5c790c8ef59cc8b2,
        u__CNTHV_CTL_EL2_ENABLE: bool,
        u__CNTKCTL_EL1_EL0VTEN: bool,
        ga_56306: ProductType5c790c8ef59cc8b2,
        gs_57944: bool,
        gs_57941: bool,
        gs_57949: bool,
        u__HCR_EL2_TGE: bool,
        gs_57940: bool,
        gs_57909: bool,
        ga_56261: ProductType5c790c8ef59cc8b2,
        u__CNTV_CTL_EL0_ENABLE: bool,
        gs_57939: bool,
        ga_56176: ProductType5c790c8ef59cc8b2,
        gs_57908: bool,
        gs_57945: bool,
        gs_57948: bool,
        gs_57942: bool,
        u__CNTHCTL_EL2_EL1TVT: bool,
        ga_56247: ProductType5c790c8ef59cc8b2,
        ga_56187: ProductType5c790c8ef59cc8b2,
        ga_56232: ProductType5c790c8ef59cc8b2,
        ga_56272: ProductType5c790c8ef59cc8b2,
        gs_57915: bool,
        gs_57947: bool,
        u__PSTATE_EL: u8,
        gs_57938: bool,
        gs_57930: bool,
        gs_57917: bool,
        u__CNTHCTL_EL2_EL0VTEN: bool,
        gs_57962: bool,
        gs_57946: bool,
        gs_57937: bool,
        gs_57916: bool,
        ga_56288: ProductType5c790c8ef59cc8b2,
        ga_56139: ProductType5c790c8ef59cc8b2,
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
        // C s_0_3: const #22056u : u32
        let s_0_3: u32 = 22056;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_CNTKCTL_EL1_Type_EL0VTEN(s_0_4)
        let s_0_5: bool = u_get_CNTKCTL_EL1_Type_EL0VTEN(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTKCTL_EL1_EL0VTEN <= s_0_5
        fn_state.u__CNTKCTL_EL1_EL0VTEN = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_TGE(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_TGE <= s_0_9
        fn_state.u__HCR_EL2_TGE = s_0_9;
        // C s_0_11: const #12808u : u32
        let s_0_11: u32 = 12808;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_CNTHCTL_EL2_Type_EL0VTEN(s_0_12)
        let s_0_13: bool = u_get_CNTHCTL_EL2_Type_EL0VTEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTHCTL_EL2_EL0VTEN <= s_0_13
        fn_state.u__CNTHCTL_EL2_EL0VTEN = s_0_13;
        // C s_0_15: const #12808u : u32
        let s_0_15: u32 = 12808;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_CNTHCTL_EL2_Type_EL1TVT(s_0_16)
        let s_0_17: bool = u_get_CNTHCTL_EL2_Type_EL1TVT(state, tracer, s_0_16);
        // D s_0_18: write-var __CNTHCTL_EL2_EL1TVT <= s_0_17
        fn_state.u__CNTHCTL_EL2_EL1TVT = s_0_17;
        // C s_0_19: const #14872u : u32
        let s_0_19: u32 = 14872;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_CNTHVS_CTL_EL2_Type_ENABLE(s_0_20)
        let s_0_21: bool = u_get_CNTHVS_CTL_EL2_Type_ENABLE(state, tracer, s_0_20);
        // D s_0_22: write-var __CNTHVS_CTL_EL2_ENABLE <= s_0_21
        fn_state.u__CNTHVS_CTL_EL2_ENABLE = s_0_21;
        // C s_0_23: const #19280u : u32
        let s_0_23: u32 = 19280;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_CNTHV_CTL_EL2_Type_ENABLE(s_0_24)
        let s_0_25: bool = u_get_CNTHV_CTL_EL2_Type_ENABLE(state, tracer, s_0_24);
        // D s_0_26: write-var __CNTHV_CTL_EL2_ENABLE <= s_0_25
        fn_state.u__CNTHV_CTL_EL2_ENABLE = s_0_25;
        // C s_0_27: const #17200u : u32
        let s_0_27: u32 = 17200;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_CNTV_CTL_EL0_Type_ENABLE(s_0_28)
        let s_0_29: bool = u_get_CNTV_CTL_EL0_Type_ENABLE(state, tracer, s_0_28);
        // D s_0_30: write-var __CNTV_CTL_EL0_ENABLE <= s_0_29
        fn_state.u__CNTV_CTL_EL0_ENABLE = s_0_29;
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
        // N s_0_37: branch s_0_36 b54 b1
        if s_0_36 {
            return block_54(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b42 b2
        if s_1_5 {
            return block_42(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b18 b3
        if s_2_5 {
            return block_18(state, tracer, fn_state);
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
        // D s_5_0: read-var __CNTV_CTL_EL0_ENABLE:u8
        let s_5_0: bool = fn_state.u__CNTV_CTL_EL0_ENABLE;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b17 b6
        if s_5_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #432u : u32
        let s_6_0: u32 = 432;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // D s_6_3: cmp-lt s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) < (s_6_2));
        // N s_6_4: branch s_6_3 b16 b7
        if s_6_3 {
            return block_16(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#57908 <= s_7_0
        fn_state.gs_57908 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#57908:u8
        let s_8_0: bool = fn_state.gs_57908;
        // N s_8_1: branch s_8_0 b15 b9
        if s_8_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #432u : u32
        let s_9_0: u32 = 432;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // C s_9_2: const #2u : u8
        let s_9_2: u8 = 2;
        // D s_9_3: cmp-lt s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) < (s_9_2));
        // N s_9_4: branch s_9_3 b14 b10
        if s_9_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#57909 <= s_10_0
        fn_state.gs_57909 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#57909:u8
        let s_11_0: bool = fn_state.gs_57909;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #64s : i64
        let s_12_0: i64 = 64;
        // C s_12_1: const #23632u : u32
        let s_12_1: u32 = 23632;
        // D s_12_2: read-reg s_12_1:u64
        let s_12_2: u64 = {
            let value = state.read_register::<u64>(s_12_1 as isize);
            tracer.read_register(s_12_1 as isize, value);
            value
        };
        // C s_12_3: const #() : ()
        let s_12_3: () = ();
        // S s_12_4: call PhysicalCountInt(s_12_3)
        let s_12_4: u64 = PhysicalCountInt(state, tracer, s_12_3);
        // D s_12_5: cast zx s_12_2 -> bv
        let s_12_5: Bits = Bits::new(s_12_2 as u128, 64u16);
        // S s_12_6: cast zx s_12_4 -> bv
        let s_12_6: Bits = Bits::new(s_12_4 as u128, 64u16);
        // D s_12_7: sub s_12_5 s_12_6
        let s_12_7: Bits = ((s_12_5) - (s_12_6));
        // D s_12_8: cast reint s_12_7 -> u64
        let s_12_8: u64 = (s_12_7.value() as u64);
        // D s_12_9: call Mk_CNTHV_TVAL_EL2_Type(s_12_8)
        let s_12_9: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_12_8,
        );
        // D s_12_10: call __get_CNTHV_TVAL_EL2(s_12_9)
        let s_12_10: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_12_9,
        );
        // D s_12_11: write-var ga#56306 <= s_12_10
        fn_state.ga_56306 = s_12_10;
        // D s_12_12: read-var ga#56306.0:struct
        let s_12_12: u64 = fn_state.ga_56306._0;
        // D s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 64u16);
        // D s_12_14: read-var t:i
        let s_12_14: i128 = fn_state.t;
        // D s_12_15: call X_set(s_12_14, s_12_0, s_12_13)
        let s_12_15: () = X_set(state, tracer, s_12_14, s_12_0, s_12_13);
        // N s_12_16: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #64s : i64
        let s_13_0: i64 = 64;
        // C s_13_1: const #23632u : u32
        let s_13_1: u32 = 23632;
        // D s_13_2: read-reg s_13_1:u64
        let s_13_2: u64 = {
            let value = state.read_register::<u64>(s_13_1 as isize);
            tracer.read_register(s_13_1 as isize, value);
            value
        };
        // C s_13_3: const #() : ()
        let s_13_3: () = ();
        // S s_13_4: call PhysicalCountInt(s_13_3)
        let s_13_4: u64 = PhysicalCountInt(state, tracer, s_13_3);
        // C s_13_5: const #() : ()
        let s_13_5: () = ();
        // S s_13_6: call CNTVOFF_read(s_13_5)
        let s_13_6: u64 = CNTVOFF_read(state, tracer, s_13_5);
        // S s_13_7: cast zx s_13_4 -> bv
        let s_13_7: Bits = Bits::new(s_13_4 as u128, 64u16);
        // S s_13_8: cast zx s_13_6 -> bv
        let s_13_8: Bits = Bits::new(s_13_6 as u128, 64u16);
        // S s_13_9: sub s_13_7 s_13_8
        let s_13_9: Bits = ((s_13_7) - (s_13_8));
        // S s_13_10: cast reint s_13_9 -> u64
        let s_13_10: u64 = (s_13_9.value() as u64);
        // D s_13_11: cast zx s_13_2 -> bv
        let s_13_11: Bits = Bits::new(s_13_2 as u128, 64u16);
        // S s_13_12: cast zx s_13_10 -> bv
        let s_13_12: Bits = Bits::new(s_13_10 as u128, 64u16);
        // D s_13_13: sub s_13_11 s_13_12
        let s_13_13: Bits = ((s_13_11) - (s_13_12));
        // D s_13_14: cast reint s_13_13 -> u64
        let s_13_14: u64 = (s_13_13.value() as u64);
        // D s_13_15: call Mk_CNTHV_TVAL_EL2_Type(s_13_14)
        let s_13_15: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_13_14,
        );
        // D s_13_16: call __get_CNTHV_TVAL_EL2(s_13_15)
        let s_13_16: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_13_15,
        );
        // D s_13_17: write-var ga#56299 <= s_13_16
        fn_state.ga_56299 = s_13_16;
        // D s_13_18: read-var ga#56299.0:struct
        let s_13_18: u64 = fn_state.ga_56299._0;
        // D s_13_19: cast zx s_13_18 -> bv
        let s_13_19: Bits = Bits::new(s_13_18 as u128, 64u16);
        // D s_13_20: read-var t:i
        let s_13_20: i128 = fn_state.t;
        // D s_13_21: call X_set(s_13_20, s_13_0, s_13_19)
        let s_13_21: () = X_set(state, tracer, s_13_20, s_13_0, s_13_19);
        // N s_13_22: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #432u : u32
        let s_14_0: u32 = 432;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: u8 = {
            let value = state.read_register::<u8>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call ELUsingAArch32(s_14_1)
        let s_14_2: bool = ELUsingAArch32(state, tracer, s_14_1);
        // D s_14_3: write-var gs#57909 <= s_14_2
        fn_state.gs_57909 = s_14_2;
        // N s_14_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // C s_15_1: const #23632u : u32
        let s_15_1: u32 = 23632;
        // D s_15_2: read-reg s_15_1:u64
        let s_15_2: u64 = {
            let value = state.read_register::<u64>(s_15_1 as isize);
            tracer.read_register(s_15_1 as isize, value);
            value
        };
        // C s_15_3: const #() : ()
        let s_15_3: () = ();
        // S s_15_4: call PhysicalCountInt(s_15_3)
        let s_15_4: u64 = PhysicalCountInt(state, tracer, s_15_3);
        // S s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 64u16);
        // C s_15_6: const #22400u : u32
        let s_15_6: u32 = 22400;
        // D s_15_7: read-reg s_15_6:u64
        let s_15_7: u64 = {
            let value = state.read_register::<u64>(s_15_6 as isize);
            tracer.read_register(s_15_6 as isize, value);
            value
        };
        // D s_15_8: cast zx s_15_7 -> bv
        let s_15_8: Bits = Bits::new(s_15_7 as u128, 64u16);
        // D s_15_9: sub s_15_5 s_15_8
        let s_15_9: Bits = ((s_15_5) - (s_15_8));
        // D s_15_10: cast reint s_15_9 -> u64
        let s_15_10: u64 = (s_15_9.value() as u64);
        // D s_15_11: cast zx s_15_2 -> bv
        let s_15_11: Bits = Bits::new(s_15_2 as u128, 64u16);
        // D s_15_12: cast zx s_15_10 -> bv
        let s_15_12: Bits = Bits::new(s_15_10 as u128, 64u16);
        // D s_15_13: sub s_15_11 s_15_12
        let s_15_13: Bits = ((s_15_11) - (s_15_12));
        // D s_15_14: cast reint s_15_13 -> u64
        let s_15_14: u64 = (s_15_13.value() as u64);
        // D s_15_15: call Mk_CNTHV_TVAL_EL2_Type(s_15_14)
        let s_15_15: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_15_14,
        );
        // D s_15_16: call __get_CNTHV_TVAL_EL2(s_15_15)
        let s_15_16: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_15_15,
        );
        // D s_15_17: write-var ga#56288 <= s_15_16
        fn_state.ga_56288 = s_15_16;
        // D s_15_18: read-var ga#56288.0:struct
        let s_15_18: u64 = fn_state.ga_56288._0;
        // D s_15_19: cast zx s_15_18 -> bv
        let s_15_19: Bits = Bits::new(s_15_18 as u128, 64u16);
        // D s_15_20: read-var t:i
        let s_15_20: i128 = fn_state.t;
        // D s_15_21: call X_set(s_15_20, s_15_0, s_15_19)
        let s_15_21: () = X_set(state, tracer, s_15_20, s_15_0, s_15_19);
        // N s_15_22: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #432u : u32
        let s_16_0: u32 = 432;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call ELUsingAArch32(s_16_1)
        let s_16_2: bool = ELUsingAArch32(state, tracer, s_16_1);
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // D s_16_4: write-var gs#57908 <= s_16_3
        fn_state.gs_57908 = s_16_3;
        // N s_16_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #64s : i64
        let s_17_0: i64 = 64;
        // C s_17_1: const #64s : i64
        let s_17_1: i64 = 64;
        // C s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // S s_17_3: call __UNKNOWN_bits(s_17_2)
        let s_17_3: Bits = u__UNKNOWN_bits(state, tracer, s_17_2);
        // S s_17_4: cast reint s_17_3 -> u64
        let s_17_4: u64 = (s_17_3.value() as u64);
        // S s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 64u16);
        // D s_17_6: read-var t:i
        let s_17_6: i128 = fn_state.t;
        // D s_17_7: call X_set(s_17_6, s_17_0, s_17_5)
        let s_17_7: () = X_set(state, tracer, s_17_6, s_17_0, s_17_5);
        // N s_17_8: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #102552u : u32
        let s_18_0: u32 = 102552;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_HCR_EL2_Type_E2H(s_18_1)
        let s_18_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #1u : u8
        let s_18_4: bool = true;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // D s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // N s_18_7: branch s_18_6 b41 b19
        if s_18_6 {
            return block_41(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#57915 <= s_19_0
        fn_state.gs_57915 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#57915:u8
        let s_20_0: bool = fn_state.gs_57915;
        // N s_20_1: branch s_20_0 b40 b21
        if s_20_0 {
            return block_40(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#57916 <= s_21_0
        fn_state.gs_57916 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#57916:u8
        let s_22_0: bool = fn_state.gs_57916;
        // N s_22_1: branch s_22_0 b37 b23
        if s_22_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #102552u : u32
        let s_23_0: u32 = 102552;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_HCR_EL2_Type_E2H(s_23_1)
        let s_23_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // N s_23_7: branch s_23_6 b36 b24
        if s_23_6 {
            return block_36(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#57917 <= s_24_0
        fn_state.gs_57917 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#57917:u8
        let s_25_0: bool = fn_state.gs_57917;
        // N s_25_1: branch s_25_0 b33 b26
        if s_25_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #102552u : u32
        let s_26_0: u32 = 102552;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_HCR_EL2_Type_E2H(s_26_1)
        let s_26_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_26_1);
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // C s_26_4: const #0u : u8
        let s_26_4: bool = false;
        // C s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 1u16);
        // D s_26_6: cmp-eq s_26_3 s_26_5
        let s_26_6: bool = ((s_26_3) == (s_26_5));
        // N s_26_7: branch s_26_6 b30 b27
        if s_26_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __CNTV_CTL_EL0_ENABLE:u8
        let s_27_0: bool = fn_state.u__CNTV_CTL_EL0_ENABLE;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // N s_27_5: branch s_27_4 b29 b28
        if s_27_4 {
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
        // C s_28_0: const #64s : i64
        let s_28_0: i64 = 64;
        // C s_28_1: const #23632u : u32
        let s_28_1: u32 = 23632;
        // D s_28_2: read-reg s_28_1:u64
        let s_28_2: u64 = {
            let value = state.read_register::<u64>(s_28_1 as isize);
            tracer.read_register(s_28_1 as isize, value);
            value
        };
        // C s_28_3: const #() : ()
        let s_28_3: () = ();
        // S s_28_4: call PhysicalCountInt(s_28_3)
        let s_28_4: u64 = PhysicalCountInt(state, tracer, s_28_3);
        // D s_28_5: cast zx s_28_2 -> bv
        let s_28_5: Bits = Bits::new(s_28_2 as u128, 64u16);
        // S s_28_6: cast zx s_28_4 -> bv
        let s_28_6: Bits = Bits::new(s_28_4 as u128, 64u16);
        // D s_28_7: sub s_28_5 s_28_6
        let s_28_7: Bits = ((s_28_5) - (s_28_6));
        // D s_28_8: cast reint s_28_7 -> u64
        let s_28_8: u64 = (s_28_7.value() as u64);
        // D s_28_9: call Mk_CNTHV_TVAL_EL2_Type(s_28_8)
        let s_28_9: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_28_8,
        );
        // D s_28_10: call __get_CNTHV_TVAL_EL2(s_28_9)
        let s_28_10: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_28_9,
        );
        // D s_28_11: write-var ga#56272 <= s_28_10
        fn_state.ga_56272 = s_28_10;
        // D s_28_12: read-var ga#56272.0:struct
        let s_28_12: u64 = fn_state.ga_56272._0;
        // D s_28_13: cast zx s_28_12 -> bv
        let s_28_13: Bits = Bits::new(s_28_12 as u128, 64u16);
        // D s_28_14: read-var t:i
        let s_28_14: i128 = fn_state.t;
        // D s_28_15: call X_set(s_28_14, s_28_0, s_28_13)
        let s_28_15: () = X_set(state, tracer, s_28_14, s_28_0, s_28_13);
        // N s_28_16: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #64s : i64
        let s_29_0: i64 = 64;
        // C s_29_1: const #64s : i64
        let s_29_1: i64 = 64;
        // C s_29_2: cast zx s_29_1 -> i
        let s_29_2: i128 = (i128::try_from(s_29_1).unwrap());
        // S s_29_3: call __UNKNOWN_bits(s_29_2)
        let s_29_3: Bits = u__UNKNOWN_bits(state, tracer, s_29_2);
        // S s_29_4: cast reint s_29_3 -> u64
        let s_29_4: u64 = (s_29_3.value() as u64);
        // S s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 64u16);
        // D s_29_6: read-var t:i
        let s_29_6: i128 = fn_state.t;
        // D s_29_7: call X_set(s_29_6, s_29_0, s_29_5)
        let s_29_7: () = X_set(state, tracer, s_29_6, s_29_0, s_29_5);
        // N s_29_8: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __CNTV_CTL_EL0_ENABLE:u8
        let s_30_0: bool = fn_state.u__CNTV_CTL_EL0_ENABLE;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #0u : u8
        let s_30_2: bool = false;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // N s_30_5: branch s_30_4 b32 b31
        if s_30_4 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #64s : i64
        let s_31_0: i64 = 64;
        // C s_31_1: const #23632u : u32
        let s_31_1: u32 = 23632;
        // D s_31_2: read-reg s_31_1:u64
        let s_31_2: u64 = {
            let value = state.read_register::<u64>(s_31_1 as isize);
            tracer.read_register(s_31_1 as isize, value);
            value
        };
        // C s_31_3: const #() : ()
        let s_31_3: () = ();
        // S s_31_4: call PhysicalCountInt(s_31_3)
        let s_31_4: u64 = PhysicalCountInt(state, tracer, s_31_3);
        // S s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 64u16);
        // C s_31_6: const #22400u : u32
        let s_31_6: u32 = 22400;
        // D s_31_7: read-reg s_31_6:u64
        let s_31_7: u64 = {
            let value = state.read_register::<u64>(s_31_6 as isize);
            tracer.read_register(s_31_6 as isize, value);
            value
        };
        // D s_31_8: cast zx s_31_7 -> bv
        let s_31_8: Bits = Bits::new(s_31_7 as u128, 64u16);
        // D s_31_9: sub s_31_5 s_31_8
        let s_31_9: Bits = ((s_31_5) - (s_31_8));
        // D s_31_10: cast reint s_31_9 -> u64
        let s_31_10: u64 = (s_31_9.value() as u64);
        // D s_31_11: cast zx s_31_2 -> bv
        let s_31_11: Bits = Bits::new(s_31_2 as u128, 64u16);
        // D s_31_12: cast zx s_31_10 -> bv
        let s_31_12: Bits = Bits::new(s_31_10 as u128, 64u16);
        // D s_31_13: sub s_31_11 s_31_12
        let s_31_13: Bits = ((s_31_11) - (s_31_12));
        // D s_31_14: cast reint s_31_13 -> u64
        let s_31_14: u64 = (s_31_13.value() as u64);
        // D s_31_15: call Mk_CNTHV_TVAL_EL2_Type(s_31_14)
        let s_31_15: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_31_14,
        );
        // D s_31_16: call __get_CNTHV_TVAL_EL2(s_31_15)
        let s_31_16: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_31_15,
        );
        // D s_31_17: write-var ga#56261 <= s_31_16
        fn_state.ga_56261 = s_31_16;
        // D s_31_18: read-var ga#56261.0:struct
        let s_31_18: u64 = fn_state.ga_56261._0;
        // D s_31_19: cast zx s_31_18 -> bv
        let s_31_19: Bits = Bits::new(s_31_18 as u128, 64u16);
        // D s_31_20: read-var t:i
        let s_31_20: i128 = fn_state.t;
        // D s_31_21: call X_set(s_31_20, s_31_0, s_31_19)
        let s_31_21: () = X_set(state, tracer, s_31_20, s_31_0, s_31_19);
        // N s_31_22: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #64s : i64
        let s_32_0: i64 = 64;
        // C s_32_1: const #64s : i64
        let s_32_1: i64 = 64;
        // C s_32_2: cast zx s_32_1 -> i
        let s_32_2: i128 = (i128::try_from(s_32_1).unwrap());
        // S s_32_3: call __UNKNOWN_bits(s_32_2)
        let s_32_3: Bits = u__UNKNOWN_bits(state, tracer, s_32_2);
        // S s_32_4: cast reint s_32_3 -> u64
        let s_32_4: u64 = (s_32_3.value() as u64);
        // S s_32_5: cast zx s_32_4 -> bv
        let s_32_5: Bits = Bits::new(s_32_4 as u128, 64u16);
        // D s_32_6: read-var t:i
        let s_32_6: i128 = fn_state.t;
        // D s_32_7: call X_set(s_32_6, s_32_0, s_32_5)
        let s_32_7: () = X_set(state, tracer, s_32_6, s_32_0, s_32_5);
        // N s_32_8: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __CNTHV_CTL_EL2_ENABLE:u8
        let s_33_0: bool = fn_state.u__CNTHV_CTL_EL2_ENABLE;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #0u : u8
        let s_33_2: bool = false;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // N s_33_5: branch s_33_4 b35 b34
        if s_33_4 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #64s : i64
        let s_34_0: i64 = 64;
        // C s_34_1: const #103152u : u32
        let s_34_1: u32 = 103152;
        // D s_34_2: read-reg s_34_1:u64
        let s_34_2: u64 = {
            let value = state.read_register::<u64>(s_34_1 as isize);
            tracer.read_register(s_34_1 as isize, value);
            value
        };
        // C s_34_3: const #() : ()
        let s_34_3: () = ();
        // S s_34_4: call PhysicalCountInt(s_34_3)
        let s_34_4: u64 = PhysicalCountInt(state, tracer, s_34_3);
        // D s_34_5: cast zx s_34_2 -> bv
        let s_34_5: Bits = Bits::new(s_34_2 as u128, 64u16);
        // S s_34_6: cast zx s_34_4 -> bv
        let s_34_6: Bits = Bits::new(s_34_4 as u128, 64u16);
        // D s_34_7: sub s_34_5 s_34_6
        let s_34_7: Bits = ((s_34_5) - (s_34_6));
        // D s_34_8: cast reint s_34_7 -> u64
        let s_34_8: u64 = (s_34_7.value() as u64);
        // D s_34_9: call Mk_CNTHV_TVAL_EL2_Type(s_34_8)
        let s_34_9: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_34_8,
        );
        // D s_34_10: call __get_CNTHV_TVAL_EL2(s_34_9)
        let s_34_10: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_34_9,
        );
        // D s_34_11: write-var ga#56247 <= s_34_10
        fn_state.ga_56247 = s_34_10;
        // D s_34_12: read-var ga#56247.0:struct
        let s_34_12: u64 = fn_state.ga_56247._0;
        // D s_34_13: cast zx s_34_12 -> bv
        let s_34_13: Bits = Bits::new(s_34_12 as u128, 64u16);
        // D s_34_14: read-var t:i
        let s_34_14: i128 = fn_state.t;
        // D s_34_15: call X_set(s_34_14, s_34_0, s_34_13)
        let s_34_15: () = X_set(state, tracer, s_34_14, s_34_0, s_34_13);
        // N s_34_16: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #64s : i64
        let s_35_0: i64 = 64;
        // C s_35_1: const #64s : i64
        let s_35_1: i64 = 64;
        // C s_35_2: cast zx s_35_1 -> i
        let s_35_2: i128 = (i128::try_from(s_35_1).unwrap());
        // S s_35_3: call __UNKNOWN_bits(s_35_2)
        let s_35_3: Bits = u__UNKNOWN_bits(state, tracer, s_35_2);
        // S s_35_4: cast reint s_35_3 -> u64
        let s_35_4: u64 = (s_35_3.value() as u64);
        // S s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 64u16);
        // D s_35_6: read-var t:i
        let s_35_6: i128 = fn_state.t;
        // D s_35_7: call X_set(s_35_6, s_35_0, s_35_5)
        let s_35_7: () = X_set(state, tracer, s_35_6, s_35_0, s_35_5);
        // N s_35_8: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #90704u : u32
        let s_36_0: u32 = 90704;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_SCR_EL3_Type_NS(s_36_1)
        let s_36_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_36_1);
        // D s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // D s_36_6: cmp-eq s_36_3 s_36_5
        let s_36_6: bool = ((s_36_3) == (s_36_5));
        // D s_36_7: write-var gs#57917 <= s_36_6
        fn_state.gs_57917 = s_36_6;
        // N s_36_8: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var __CNTHVS_CTL_EL2_ENABLE:u8
        let s_37_0: bool = fn_state.u__CNTHVS_CTL_EL2_ENABLE;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #0u : u8
        let s_37_2: bool = false;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // N s_37_5: branch s_37_4 b39 b38
        if s_37_4 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #64s : i64
        let s_38_0: i64 = 64;
        // C s_38_1: const #10064u : u32
        let s_38_1: u32 = 10064;
        // D s_38_2: read-reg s_38_1:u64
        let s_38_2: u64 = {
            let value = state.read_register::<u64>(s_38_1 as isize);
            tracer.read_register(s_38_1 as isize, value);
            value
        };
        // C s_38_3: const #() : ()
        let s_38_3: () = ();
        // S s_38_4: call PhysicalCountInt(s_38_3)
        let s_38_4: u64 = PhysicalCountInt(state, tracer, s_38_3);
        // D s_38_5: cast zx s_38_2 -> bv
        let s_38_5: Bits = Bits::new(s_38_2 as u128, 64u16);
        // S s_38_6: cast zx s_38_4 -> bv
        let s_38_6: Bits = Bits::new(s_38_4 as u128, 64u16);
        // D s_38_7: sub s_38_5 s_38_6
        let s_38_7: Bits = ((s_38_5) - (s_38_6));
        // D s_38_8: cast reint s_38_7 -> u64
        let s_38_8: u64 = (s_38_7.value() as u64);
        // D s_38_9: call Mk_CNTHV_TVAL_EL2_Type(s_38_8)
        let s_38_9: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_38_8,
        );
        // D s_38_10: call __get_CNTHV_TVAL_EL2(s_38_9)
        let s_38_10: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_38_9,
        );
        // D s_38_11: write-var ga#56232 <= s_38_10
        fn_state.ga_56232 = s_38_10;
        // D s_38_12: read-var ga#56232.0:struct
        let s_38_12: u64 = fn_state.ga_56232._0;
        // D s_38_13: cast zx s_38_12 -> bv
        let s_38_13: Bits = Bits::new(s_38_12 as u128, 64u16);
        // D s_38_14: read-var t:i
        let s_38_14: i128 = fn_state.t;
        // D s_38_15: call X_set(s_38_14, s_38_0, s_38_13)
        let s_38_15: () = X_set(state, tracer, s_38_14, s_38_0, s_38_13);
        // N s_38_16: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #64s : i64
        let s_39_0: i64 = 64;
        // C s_39_1: const #64s : i64
        let s_39_1: i64 = 64;
        // C s_39_2: cast zx s_39_1 -> i
        let s_39_2: i128 = (i128::try_from(s_39_1).unwrap());
        // S s_39_3: call __UNKNOWN_bits(s_39_2)
        let s_39_3: Bits = u__UNKNOWN_bits(state, tracer, s_39_2);
        // S s_39_4: cast reint s_39_3 -> u64
        let s_39_4: u64 = (s_39_3.value() as u64);
        // S s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 64u16);
        // D s_39_6: read-var t:i
        let s_39_6: i128 = fn_state.t;
        // D s_39_7: call X_set(s_39_6, s_39_0, s_39_5)
        let s_39_7: () = X_set(state, tracer, s_39_6, s_39_0, s_39_5);
        // N s_39_8: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #117u : u32
        let s_40_0: u32 = 117;
        // S s_40_1: call IsFeatureImplemented(s_40_0)
        let s_40_1: bool = IsFeatureImplemented(state, tracer, s_40_0);
        // D s_40_2: write-var gs#57916 <= s_40_1
        fn_state.gs_57916 = s_40_1;
        // N s_40_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #90704u : u32
        let s_41_0: u32 = 90704;
        // D s_41_1: read-reg s_41_0:struct
        let s_41_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: call _get_SCR_EL3_Type_NS(s_41_1)
        let s_41_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_41_1);
        // D s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // C s_41_4: const #0u : u8
        let s_41_4: bool = false;
        // C s_41_5: cast zx s_41_4 -> bv
        let s_41_5: Bits = Bits::new(s_41_4 as u128, 1u16);
        // D s_41_6: cmp-eq s_41_3 s_41_5
        let s_41_6: bool = ((s_41_3) == (s_41_5));
        // D s_41_7: write-var gs#57915 <= s_41_6
        fn_state.gs_57915 = s_41_6;
        // N s_41_8: jump b20
        return block_20(state, tracer, fn_state);
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
        // N s_42_2: branch s_42_1 b53 b43
        if s_42_1 {
            return block_53(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#57930 <= s_43_0
        fn_state.gs_57930 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#57930:u8
        let s_44_0: bool = fn_state.gs_57930;
        // N s_44_1: branch s_44_0 b52 b45
        if s_44_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #432u : u32
        let s_45_0: u32 = 432;
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
        // N s_45_4: branch s_45_3 b49 b46
        if s_45_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var __CNTV_CTL_EL0_ENABLE:u8
        let s_46_0: bool = fn_state.u__CNTV_CTL_EL0_ENABLE;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #0u : u8
        let s_46_2: bool = false;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // N s_46_5: branch s_46_4 b48 b47
        if s_46_4 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #64s : i64
        let s_47_0: i64 = 64;
        // C s_47_1: const #23632u : u32
        let s_47_1: u32 = 23632;
        // D s_47_2: read-reg s_47_1:u64
        let s_47_2: u64 = {
            let value = state.read_register::<u64>(s_47_1 as isize);
            tracer.read_register(s_47_1 as isize, value);
            value
        };
        // C s_47_3: const #() : ()
        let s_47_3: () = ();
        // S s_47_4: call PhysicalCountInt(s_47_3)
        let s_47_4: u64 = PhysicalCountInt(state, tracer, s_47_3);
        // D s_47_5: cast zx s_47_2 -> bv
        let s_47_5: Bits = Bits::new(s_47_2 as u128, 64u16);
        // S s_47_6: cast zx s_47_4 -> bv
        let s_47_6: Bits = Bits::new(s_47_4 as u128, 64u16);
        // D s_47_7: sub s_47_5 s_47_6
        let s_47_7: Bits = ((s_47_5) - (s_47_6));
        // D s_47_8: cast reint s_47_7 -> u64
        let s_47_8: u64 = (s_47_7.value() as u64);
        // D s_47_9: call Mk_CNTHV_TVAL_EL2_Type(s_47_8)
        let s_47_9: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_47_8,
        );
        // D s_47_10: call __get_CNTHV_TVAL_EL2(s_47_9)
        let s_47_10: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_47_9,
        );
        // D s_47_11: write-var ga#56215 <= s_47_10
        fn_state.ga_56215 = s_47_10;
        // D s_47_12: read-var ga#56215.0:struct
        let s_47_12: u64 = fn_state.ga_56215._0;
        // D s_47_13: cast zx s_47_12 -> bv
        let s_47_13: Bits = Bits::new(s_47_12 as u128, 64u16);
        // D s_47_14: read-var t:i
        let s_47_14: i128 = fn_state.t;
        // D s_47_15: call X_set(s_47_14, s_47_0, s_47_13)
        let s_47_15: () = X_set(state, tracer, s_47_14, s_47_0, s_47_13);
        // N s_47_16: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #64s : i64
        let s_48_0: i64 = 64;
        // C s_48_1: const #64s : i64
        let s_48_1: i64 = 64;
        // C s_48_2: cast zx s_48_1 -> i
        let s_48_2: i128 = (i128::try_from(s_48_1).unwrap());
        // S s_48_3: call __UNKNOWN_bits(s_48_2)
        let s_48_3: Bits = u__UNKNOWN_bits(state, tracer, s_48_2);
        // S s_48_4: cast reint s_48_3 -> u64
        let s_48_4: u64 = (s_48_3.value() as u64);
        // S s_48_5: cast zx s_48_4 -> bv
        let s_48_5: Bits = Bits::new(s_48_4 as u128, 64u16);
        // D s_48_6: read-var t:i
        let s_48_6: i128 = fn_state.t;
        // D s_48_7: call X_set(s_48_6, s_48_0, s_48_5)
        let s_48_7: () = X_set(state, tracer, s_48_6, s_48_0, s_48_5);
        // N s_48_8: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var __CNTV_CTL_EL0_ENABLE:u8
        let s_49_0: bool = fn_state.u__CNTV_CTL_EL0_ENABLE;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 1u16);
        // C s_49_2: const #0u : u8
        let s_49_2: bool = false;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // N s_49_5: branch s_49_4 b51 b50
        if s_49_4 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #64s : i64
        let s_50_0: i64 = 64;
        // C s_50_1: const #23632u : u32
        let s_50_1: u32 = 23632;
        // D s_50_2: read-reg s_50_1:u64
        let s_50_2: u64 = {
            let value = state.read_register::<u64>(s_50_1 as isize);
            tracer.read_register(s_50_1 as isize, value);
            value
        };
        // C s_50_3: const #() : ()
        let s_50_3: () = ();
        // S s_50_4: call PhysicalCountInt(s_50_3)
        let s_50_4: u64 = PhysicalCountInt(state, tracer, s_50_3);
        // S s_50_5: cast zx s_50_4 -> bv
        let s_50_5: Bits = Bits::new(s_50_4 as u128, 64u16);
        // C s_50_6: const #22400u : u32
        let s_50_6: u32 = 22400;
        // D s_50_7: read-reg s_50_6:u64
        let s_50_7: u64 = {
            let value = state.read_register::<u64>(s_50_6 as isize);
            tracer.read_register(s_50_6 as isize, value);
            value
        };
        // D s_50_8: cast zx s_50_7 -> bv
        let s_50_8: Bits = Bits::new(s_50_7 as u128, 64u16);
        // D s_50_9: sub s_50_5 s_50_8
        let s_50_9: Bits = ((s_50_5) - (s_50_8));
        // D s_50_10: cast reint s_50_9 -> u64
        let s_50_10: u64 = (s_50_9.value() as u64);
        // D s_50_11: cast zx s_50_2 -> bv
        let s_50_11: Bits = Bits::new(s_50_2 as u128, 64u16);
        // D s_50_12: cast zx s_50_10 -> bv
        let s_50_12: Bits = Bits::new(s_50_10 as u128, 64u16);
        // D s_50_13: sub s_50_11 s_50_12
        let s_50_13: Bits = ((s_50_11) - (s_50_12));
        // D s_50_14: cast reint s_50_13 -> u64
        let s_50_14: u64 = (s_50_13.value() as u64);
        // D s_50_15: call Mk_CNTHV_TVAL_EL2_Type(s_50_14)
        let s_50_15: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_50_14,
        );
        // D s_50_16: call __get_CNTHV_TVAL_EL2(s_50_15)
        let s_50_16: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_50_15,
        );
        // D s_50_17: write-var ga#56204 <= s_50_16
        fn_state.ga_56204 = s_50_16;
        // D s_50_18: read-var ga#56204.0:struct
        let s_50_18: u64 = fn_state.ga_56204._0;
        // D s_50_19: cast zx s_50_18 -> bv
        let s_50_19: Bits = Bits::new(s_50_18 as u128, 64u16);
        // D s_50_20: read-var t:i
        let s_50_20: i128 = fn_state.t;
        // D s_50_21: call X_set(s_50_20, s_50_0, s_50_19)
        let s_50_21: () = X_set(state, tracer, s_50_20, s_50_0, s_50_19);
        // N s_50_22: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #64s : i64
        let s_51_0: i64 = 64;
        // C s_51_1: const #64s : i64
        let s_51_1: i64 = 64;
        // C s_51_2: cast zx s_51_1 -> i
        let s_51_2: i128 = (i128::try_from(s_51_1).unwrap());
        // S s_51_3: call __UNKNOWN_bits(s_51_2)
        let s_51_3: Bits = u__UNKNOWN_bits(state, tracer, s_51_2);
        // S s_51_4: cast reint s_51_3 -> u64
        let s_51_4: u64 = (s_51_3.value() as u64);
        // S s_51_5: cast zx s_51_4 -> bv
        let s_51_5: Bits = Bits::new(s_51_4 as u128, 64u16);
        // D s_51_6: read-var t:i
        let s_51_6: i128 = fn_state.t;
        // D s_51_7: call X_set(s_51_6, s_51_0, s_51_5)
        let s_51_7: () = X_set(state, tracer, s_51_6, s_51_0, s_51_5);
        // N s_51_8: return
        return;
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
        // C s_52_5: const #432u : u32
        let s_52_5: u32 = 432;
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
        // D s_53_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_53_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 1u16);
        // C s_53_2: const #1u : u8
        let s_53_2: bool = true;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // D s_53_5: write-var gs#57930 <= s_53_4
        fn_state.gs_57930 = s_53_4;
        // N s_53_6: jump b44
        return block_44(state, tracer, fn_state);
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
        // N s_54_2: branch s_54_1 b118 b55
        if s_54_1 {
            return block_118(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#57937 <= s_55_0
        fn_state.gs_57937 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#57937:u8
        let s_56_0: bool = fn_state.gs_57937;
        // D s_56_1: not s_56_0
        let s_56_1: bool = !s_56_0;
        // N s_56_2: branch s_56_1 b117 b57
        if s_56_1 {
            return block_117(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#57938 <= s_57_0
        fn_state.gs_57938 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#57938:u8
        let s_58_0: bool = fn_state.gs_57938;
        // N s_58_1: branch s_58_0 b111 b59
        if s_58_0 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call EL2Enabled(s_59_0)
        let s_59_1: bool = EL2Enabled(state, tracer, s_59_0);
        // N s_59_2: branch s_59_1 b110 b60
        if s_59_1 {
            return block_110(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#57939 <= s_60_0
        fn_state.gs_57939 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#57939:u8
        let s_61_0: bool = fn_state.gs_57939;
        // N s_61_1: branch s_61_0 b109 b62
        if s_61_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#57940 <= s_62_0
        fn_state.gs_57940 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#57940:u8
        let s_63_0: bool = fn_state.gs_57940;
        // N s_63_1: branch s_63_0 b108 b64
        if s_63_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call EL2Enabled(s_64_0)
        let s_64_1: bool = EL2Enabled(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b107 b65
        if s_64_1 {
            return block_107(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#57941 <= s_65_0
        fn_state.gs_57941 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#57941:u8
        let s_66_0: bool = fn_state.gs_57941;
        // N s_66_1: branch s_66_0 b106 b67
        if s_66_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#57942 <= s_67_0
        fn_state.gs_57942 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#57942:u8
        let s_68_0: bool = fn_state.gs_57942;
        // N s_68_1: branch s_68_0 b105 b69
        if s_68_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call EL2Enabled(s_69_0)
        let s_69_1: bool = EL2Enabled(state, tracer, s_69_0);
        // N s_69_2: branch s_69_1 b104 b70
        if s_69_1 {
            return block_104(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#57943 <= s_70_0
        fn_state.gs_57943 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#57943:u8
        let s_71_0: bool = fn_state.gs_57943;
        // N s_71_1: branch s_71_0 b103 b72
        if s_71_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var gs#57944 <= s_72_0
        fn_state.gs_57944 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#57944:u8
        let s_73_0: bool = fn_state.gs_57944;
        // N s_73_1: branch s_73_0 b102 b74
        if s_73_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#57945 <= s_74_0
        fn_state.gs_57945 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#57945:u8
        let s_75_0: bool = fn_state.gs_57945;
        // N s_75_1: branch s_75_0 b99 b76
        if s_75_0 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call EL2Enabled(s_76_0)
        let s_76_1: bool = EL2Enabled(state, tracer, s_76_0);
        // N s_76_2: branch s_76_1 b98 b77
        if s_76_1 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#57946 <= s_77_0
        fn_state.gs_57946 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#57946:u8
        let s_78_0: bool = fn_state.gs_57946;
        // N s_78_1: branch s_78_0 b97 b79
        if s_78_0 {
            return block_97(state, tracer, fn_state);
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
        // D s_79_1: write-var gs#57947 <= s_79_0
        fn_state.gs_57947 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#57947:u8
        let s_80_0: bool = fn_state.gs_57947;
        // N s_80_1: branch s_80_0 b94 b81
        if s_80_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #432u : u32
        let s_81_0: u32 = 432;
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
        // N s_81_4: branch s_81_3 b90 b82
        if s_81_3 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #0u : u8
        let s_82_0: bool = false;
        // D s_82_1: write-var gs#57949 <= s_82_0
        fn_state.gs_57949 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#57949:u8
        let s_83_0: bool = fn_state.gs_57949;
        // N s_83_1: branch s_83_0 b87 b84
        if s_83_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __CNTV_CTL_EL0_ENABLE:u8
        let s_84_0: bool = fn_state.u__CNTV_CTL_EL0_ENABLE;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #0u : u8
        let s_84_2: bool = false;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // N s_84_5: branch s_84_4 b86 b85
        if s_84_4 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #64s : i64
        let s_85_0: i64 = 64;
        // C s_85_1: const #23632u : u32
        let s_85_1: u32 = 23632;
        // D s_85_2: read-reg s_85_1:u64
        let s_85_2: u64 = {
            let value = state.read_register::<u64>(s_85_1 as isize);
            tracer.read_register(s_85_1 as isize, value);
            value
        };
        // C s_85_3: const #() : ()
        let s_85_3: () = ();
        // S s_85_4: call PhysicalCountInt(s_85_3)
        let s_85_4: u64 = PhysicalCountInt(state, tracer, s_85_3);
        // D s_85_5: cast zx s_85_2 -> bv
        let s_85_5: Bits = Bits::new(s_85_2 as u128, 64u16);
        // S s_85_6: cast zx s_85_4 -> bv
        let s_85_6: Bits = Bits::new(s_85_4 as u128, 64u16);
        // D s_85_7: sub s_85_5 s_85_6
        let s_85_7: Bits = ((s_85_5) - (s_85_6));
        // D s_85_8: cast reint s_85_7 -> u64
        let s_85_8: u64 = (s_85_7.value() as u64);
        // D s_85_9: call Mk_CNTHV_TVAL_EL2_Type(s_85_8)
        let s_85_9: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_85_8,
        );
        // D s_85_10: call __get_CNTHV_TVAL_EL2(s_85_9)
        let s_85_10: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_85_9,
        );
        // D s_85_11: write-var ga#56187 <= s_85_10
        fn_state.ga_56187 = s_85_10;
        // D s_85_12: read-var ga#56187.0:struct
        let s_85_12: u64 = fn_state.ga_56187._0;
        // D s_85_13: cast zx s_85_12 -> bv
        let s_85_13: Bits = Bits::new(s_85_12 as u128, 64u16);
        // D s_85_14: read-var t:i
        let s_85_14: i128 = fn_state.t;
        // D s_85_15: call X_set(s_85_14, s_85_0, s_85_13)
        let s_85_15: () = X_set(state, tracer, s_85_14, s_85_0, s_85_13);
        // N s_85_16: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #64s : i64
        let s_86_0: i64 = 64;
        // C s_86_1: const #64s : i64
        let s_86_1: i64 = 64;
        // C s_86_2: cast zx s_86_1 -> i
        let s_86_2: i128 = (i128::try_from(s_86_1).unwrap());
        // S s_86_3: call __UNKNOWN_bits(s_86_2)
        let s_86_3: Bits = u__UNKNOWN_bits(state, tracer, s_86_2);
        // S s_86_4: cast reint s_86_3 -> u64
        let s_86_4: u64 = (s_86_3.value() as u64);
        // S s_86_5: cast zx s_86_4 -> bv
        let s_86_5: Bits = Bits::new(s_86_4 as u128, 64u16);
        // D s_86_6: read-var t:i
        let s_86_6: i128 = fn_state.t;
        // D s_86_7: call X_set(s_86_6, s_86_0, s_86_5)
        let s_86_7: () = X_set(state, tracer, s_86_6, s_86_0, s_86_5);
        // N s_86_8: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var __CNTV_CTL_EL0_ENABLE:u8
        let s_87_0: bool = fn_state.u__CNTV_CTL_EL0_ENABLE;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 1u16);
        // C s_87_2: const #0u : u8
        let s_87_2: bool = false;
        // C s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 1u16);
        // D s_87_4: cmp-eq s_87_1 s_87_3
        let s_87_4: bool = ((s_87_1) == (s_87_3));
        // N s_87_5: branch s_87_4 b89 b88
        if s_87_4 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #64s : i64
        let s_88_0: i64 = 64;
        // C s_88_1: const #23632u : u32
        let s_88_1: u32 = 23632;
        // D s_88_2: read-reg s_88_1:u64
        let s_88_2: u64 = {
            let value = state.read_register::<u64>(s_88_1 as isize);
            tracer.read_register(s_88_1 as isize, value);
            value
        };
        // C s_88_3: const #() : ()
        let s_88_3: () = ();
        // S s_88_4: call PhysicalCountInt(s_88_3)
        let s_88_4: u64 = PhysicalCountInt(state, tracer, s_88_3);
        // S s_88_5: cast zx s_88_4 -> bv
        let s_88_5: Bits = Bits::new(s_88_4 as u128, 64u16);
        // C s_88_6: const #22400u : u32
        let s_88_6: u32 = 22400;
        // D s_88_7: read-reg s_88_6:u64
        let s_88_7: u64 = {
            let value = state.read_register::<u64>(s_88_6 as isize);
            tracer.read_register(s_88_6 as isize, value);
            value
        };
        // D s_88_8: cast zx s_88_7 -> bv
        let s_88_8: Bits = Bits::new(s_88_7 as u128, 64u16);
        // D s_88_9: sub s_88_5 s_88_8
        let s_88_9: Bits = ((s_88_5) - (s_88_8));
        // D s_88_10: cast reint s_88_9 -> u64
        let s_88_10: u64 = (s_88_9.value() as u64);
        // D s_88_11: cast zx s_88_2 -> bv
        let s_88_11: Bits = Bits::new(s_88_2 as u128, 64u16);
        // D s_88_12: cast zx s_88_10 -> bv
        let s_88_12: Bits = Bits::new(s_88_10 as u128, 64u16);
        // D s_88_13: sub s_88_11 s_88_12
        let s_88_13: Bits = ((s_88_11) - (s_88_12));
        // D s_88_14: cast reint s_88_13 -> u64
        let s_88_14: u64 = (s_88_13.value() as u64);
        // D s_88_15: call Mk_CNTHV_TVAL_EL2_Type(s_88_14)
        let s_88_15: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_88_14,
        );
        // D s_88_16: call __get_CNTHV_TVAL_EL2(s_88_15)
        let s_88_16: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_88_15,
        );
        // D s_88_17: write-var ga#56176 <= s_88_16
        fn_state.ga_56176 = s_88_16;
        // D s_88_18: read-var ga#56176.0:struct
        let s_88_18: u64 = fn_state.ga_56176._0;
        // D s_88_19: cast zx s_88_18 -> bv
        let s_88_19: Bits = Bits::new(s_88_18 as u128, 64u16);
        // D s_88_20: read-var t:i
        let s_88_20: i128 = fn_state.t;
        // D s_88_21: call X_set(s_88_20, s_88_0, s_88_19)
        let s_88_21: () = X_set(state, tracer, s_88_20, s_88_0, s_88_19);
        // N s_88_22: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #64s : i64
        let s_89_0: i64 = 64;
        // C s_89_1: const #64s : i64
        let s_89_1: i64 = 64;
        // C s_89_2: cast zx s_89_1 -> i
        let s_89_2: i128 = (i128::try_from(s_89_1).unwrap());
        // S s_89_3: call __UNKNOWN_bits(s_89_2)
        let s_89_3: Bits = u__UNKNOWN_bits(state, tracer, s_89_2);
        // S s_89_4: cast reint s_89_3 -> u64
        let s_89_4: u64 = (s_89_3.value() as u64);
        // S s_89_5: cast zx s_89_4 -> bv
        let s_89_5: Bits = Bits::new(s_89_4 as u128, 64u16);
        // D s_89_6: read-var t:i
        let s_89_6: i128 = fn_state.t;
        // D s_89_7: call X_set(s_89_6, s_89_0, s_89_5)
        let s_89_7: () = X_set(state, tracer, s_89_6, s_89_0, s_89_5);
        // N s_89_8: return
        return;
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #() : ()
        let s_90_0: () = ();
        // S s_90_1: call EL2Enabled(s_90_0)
        let s_90_1: bool = EL2Enabled(state, tracer, s_90_0);
        // S s_90_2: not s_90_1
        let s_90_2: bool = !s_90_1;
        // N s_90_3: branch s_90_2 b93 b91
        if s_90_2 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #102552u : u32
        let s_91_0: u32 = 102552;
        // D s_91_1: read-reg s_91_0:struct
        let s_91_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // D s_91_2: call _get_HCR_EL2_Type_E2H(s_91_1)
        let s_91_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_91_1);
        // C s_91_3: const #102552u : u32
        let s_91_3: u32 = 102552;
        // D s_91_4: read-reg s_91_3:struct
        let s_91_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_91_3 as isize);
            tracer.read_register(s_91_3 as isize, value);
            value
        };
        // D s_91_5: call _get_HCR_EL2_Type_TGE(s_91_4)
        let s_91_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_91_4);
        // D s_91_6: cast zx s_91_2 -> bv
        let s_91_6: Bits = Bits::new(s_91_2 as u128, 1u16);
        // D s_91_7: cast zx s_91_5 -> bv
        let s_91_7: Bits = Bits::new(s_91_5 as u128, 1u16);
        // D s_91_8: cast reint s_91_6 -> u128
        let s_91_8: u128 = (s_91_6.value() as u128);
        // D s_91_9: size-of s_91_6
        let s_91_9: u16 = s_91_6.length();
        // D s_91_10: cast reint s_91_7 -> u128
        let s_91_10: u128 = (s_91_7.value() as u128);
        // D s_91_11: size-of s_91_7
        let s_91_11: u16 = s_91_7.length();
        // D s_91_12: lsl s_91_8 s_91_11
        let s_91_12: u128 = s_91_8 << s_91_11;
        // D s_91_13: or s_91_12 s_91_10
        let s_91_13: u128 = ((s_91_12) | (s_91_10));
        // D s_91_14: add s_91_9 s_91_11
        let s_91_14: u16 = (s_91_9 + s_91_11);
        // D s_91_15: create-bits s_91_13 s_91_14
        let s_91_15: Bits = Bits::new(s_91_13, s_91_14);
        // D s_91_16: cast reint s_91_15 -> u8
        let s_91_16: u8 = (s_91_15.value() as u8);
        // D s_91_17: cast zx s_91_16 -> bv
        let s_91_17: Bits = Bits::new(s_91_16 as u128, 2u16);
        // C s_91_18: const #3u : u8
        let s_91_18: u8 = 3;
        // C s_91_19: cast zx s_91_18 -> bv
        let s_91_19: Bits = Bits::new(s_91_18 as u128, 2u16);
        // D s_91_20: cmp-ne s_91_17 s_91_19
        let s_91_20: bool = ((s_91_17) != (s_91_19));
        // D s_91_21: write-var gs#57948 <= s_91_20
        fn_state.gs_57948 = s_91_20;
        // N s_91_22: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#57948:u8
        let s_92_0: bool = fn_state.gs_57948;
        // D s_92_1: write-var gs#57949 <= s_92_0
        fn_state.gs_57949 = s_92_0;
        // N s_92_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #1u : u8
        let s_93_0: bool = true;
        // D s_93_1: write-var gs#57948 <= s_93_0
        fn_state.gs_57948 = s_93_0;
        // N s_93_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var __CNTHV_CTL_EL2_ENABLE:u8
        let s_94_0: bool = fn_state.u__CNTHV_CTL_EL2_ENABLE;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 1u16);
        // C s_94_2: const #0u : u8
        let s_94_2: bool = false;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 1u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // N s_94_5: branch s_94_4 b96 b95
        if s_94_4 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #64s : i64
        let s_95_0: i64 = 64;
        // C s_95_1: const #103152u : u32
        let s_95_1: u32 = 103152;
        // D s_95_2: read-reg s_95_1:u64
        let s_95_2: u64 = {
            let value = state.read_register::<u64>(s_95_1 as isize);
            tracer.read_register(s_95_1 as isize, value);
            value
        };
        // C s_95_3: const #() : ()
        let s_95_3: () = ();
        // S s_95_4: call PhysicalCountInt(s_95_3)
        let s_95_4: u64 = PhysicalCountInt(state, tracer, s_95_3);
        // D s_95_5: cast zx s_95_2 -> bv
        let s_95_5: Bits = Bits::new(s_95_2 as u128, 64u16);
        // S s_95_6: cast zx s_95_4 -> bv
        let s_95_6: Bits = Bits::new(s_95_4 as u128, 64u16);
        // D s_95_7: sub s_95_5 s_95_6
        let s_95_7: Bits = ((s_95_5) - (s_95_6));
        // D s_95_8: cast reint s_95_7 -> u64
        let s_95_8: u64 = (s_95_7.value() as u64);
        // D s_95_9: call Mk_CNTHV_TVAL_EL2_Type(s_95_8)
        let s_95_9: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_95_8,
        );
        // D s_95_10: call __get_CNTHV_TVAL_EL2(s_95_9)
        let s_95_10: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_95_9,
        );
        // D s_95_11: write-var ga#56157 <= s_95_10
        fn_state.ga_56157 = s_95_10;
        // D s_95_12: read-var ga#56157.0:struct
        let s_95_12: u64 = fn_state.ga_56157._0;
        // D s_95_13: cast zx s_95_12 -> bv
        let s_95_13: Bits = Bits::new(s_95_12 as u128, 64u16);
        // D s_95_14: read-var t:i
        let s_95_14: i128 = fn_state.t;
        // D s_95_15: call X_set(s_95_14, s_95_0, s_95_13)
        let s_95_15: () = X_set(state, tracer, s_95_14, s_95_0, s_95_13);
        // N s_95_16: return
        return;
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #64s : i64
        let s_96_0: i64 = 64;
        // C s_96_1: const #64s : i64
        let s_96_1: i64 = 64;
        // C s_96_2: cast zx s_96_1 -> i
        let s_96_2: i128 = (i128::try_from(s_96_1).unwrap());
        // S s_96_3: call __UNKNOWN_bits(s_96_2)
        let s_96_3: Bits = u__UNKNOWN_bits(state, tracer, s_96_2);
        // S s_96_4: cast reint s_96_3 -> u64
        let s_96_4: u64 = (s_96_3.value() as u64);
        // S s_96_5: cast zx s_96_4 -> bv
        let s_96_5: Bits = Bits::new(s_96_4 as u128, 64u16);
        // D s_96_6: read-var t:i
        let s_96_6: i128 = fn_state.t;
        // D s_96_7: call X_set(s_96_6, s_96_0, s_96_5)
        let s_96_7: () = X_set(state, tracer, s_96_6, s_96_0, s_96_5);
        // N s_96_8: return
        return;
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #90704u : u32
        let s_97_0: u32 = 90704;
        // D s_97_1: read-reg s_97_0:struct
        let s_97_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // D s_97_2: call _get_SCR_EL3_Type_NS(s_97_1)
        let s_97_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_97_1);
        // D s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 1u16);
        // C s_97_4: const #1u : u8
        let s_97_4: bool = true;
        // C s_97_5: cast zx s_97_4 -> bv
        let s_97_5: Bits = Bits::new(s_97_4 as u128, 1u16);
        // D s_97_6: cmp-eq s_97_3 s_97_5
        let s_97_6: bool = ((s_97_3) == (s_97_5));
        // D s_97_7: write-var gs#57947 <= s_97_6
        fn_state.gs_57947 = s_97_6;
        // N s_97_8: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #102552u : u32
        let s_98_0: u32 = 102552;
        // D s_98_1: read-reg s_98_0:struct
        let s_98_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // D s_98_2: call _get_HCR_EL2_Type_E2H(s_98_1)
        let s_98_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_98_1);
        // C s_98_3: const #102552u : u32
        let s_98_3: u32 = 102552;
        // D s_98_4: read-reg s_98_3:struct
        let s_98_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_98_3 as isize);
            tracer.read_register(s_98_3 as isize, value);
            value
        };
        // D s_98_5: call _get_HCR_EL2_Type_TGE(s_98_4)
        let s_98_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_98_4);
        // D s_98_6: cast zx s_98_2 -> bv
        let s_98_6: Bits = Bits::new(s_98_2 as u128, 1u16);
        // D s_98_7: cast zx s_98_5 -> bv
        let s_98_7: Bits = Bits::new(s_98_5 as u128, 1u16);
        // D s_98_8: cast reint s_98_6 -> u128
        let s_98_8: u128 = (s_98_6.value() as u128);
        // D s_98_9: size-of s_98_6
        let s_98_9: u16 = s_98_6.length();
        // D s_98_10: cast reint s_98_7 -> u128
        let s_98_10: u128 = (s_98_7.value() as u128);
        // D s_98_11: size-of s_98_7
        let s_98_11: u16 = s_98_7.length();
        // D s_98_12: lsl s_98_8 s_98_11
        let s_98_12: u128 = s_98_8 << s_98_11;
        // D s_98_13: or s_98_12 s_98_10
        let s_98_13: u128 = ((s_98_12) | (s_98_10));
        // D s_98_14: add s_98_9 s_98_11
        let s_98_14: u16 = (s_98_9 + s_98_11);
        // D s_98_15: create-bits s_98_13 s_98_14
        let s_98_15: Bits = Bits::new(s_98_13, s_98_14);
        // D s_98_16: cast reint s_98_15 -> u8
        let s_98_16: u8 = (s_98_15.value() as u8);
        // D s_98_17: cast zx s_98_16 -> bv
        let s_98_17: Bits = Bits::new(s_98_16 as u128, 2u16);
        // C s_98_18: const #3u : u8
        let s_98_18: u8 = 3;
        // C s_98_19: cast zx s_98_18 -> bv
        let s_98_19: Bits = Bits::new(s_98_18 as u128, 2u16);
        // D s_98_20: cmp-eq s_98_17 s_98_19
        let s_98_20: bool = ((s_98_17) == (s_98_19));
        // D s_98_21: write-var gs#57946 <= s_98_20
        fn_state.gs_57946 = s_98_20;
        // N s_98_22: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var __CNTHVS_CTL_EL2_ENABLE:u8
        let s_99_0: bool = fn_state.u__CNTHVS_CTL_EL2_ENABLE;
        // D s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 1u16);
        // C s_99_2: const #0u : u8
        let s_99_2: bool = false;
        // C s_99_3: cast zx s_99_2 -> bv
        let s_99_3: Bits = Bits::new(s_99_2 as u128, 1u16);
        // D s_99_4: cmp-eq s_99_1 s_99_3
        let s_99_4: bool = ((s_99_1) == (s_99_3));
        // N s_99_5: branch s_99_4 b101 b100
        if s_99_4 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #64s : i64
        let s_100_0: i64 = 64;
        // C s_100_1: const #10064u : u32
        let s_100_1: u32 = 10064;
        // D s_100_2: read-reg s_100_1:u64
        let s_100_2: u64 = {
            let value = state.read_register::<u64>(s_100_1 as isize);
            tracer.read_register(s_100_1 as isize, value);
            value
        };
        // C s_100_3: const #() : ()
        let s_100_3: () = ();
        // S s_100_4: call PhysicalCountInt(s_100_3)
        let s_100_4: u64 = PhysicalCountInt(state, tracer, s_100_3);
        // D s_100_5: cast zx s_100_2 -> bv
        let s_100_5: Bits = Bits::new(s_100_2 as u128, 64u16);
        // S s_100_6: cast zx s_100_4 -> bv
        let s_100_6: Bits = Bits::new(s_100_4 as u128, 64u16);
        // D s_100_7: sub s_100_5 s_100_6
        let s_100_7: Bits = ((s_100_5) - (s_100_6));
        // D s_100_8: cast reint s_100_7 -> u64
        let s_100_8: u64 = (s_100_7.value() as u64);
        // D s_100_9: call Mk_CNTHV_TVAL_EL2_Type(s_100_8)
        let s_100_9: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_TVAL_EL2_Type(
            state,
            tracer,
            s_100_8,
        );
        // D s_100_10: call __get_CNTHV_TVAL_EL2(s_100_9)
        let s_100_10: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_TVAL_EL2(
            state,
            tracer,
            s_100_9,
        );
        // D s_100_11: write-var ga#56139 <= s_100_10
        fn_state.ga_56139 = s_100_10;
        // D s_100_12: read-var ga#56139.0:struct
        let s_100_12: u64 = fn_state.ga_56139._0;
        // D s_100_13: cast zx s_100_12 -> bv
        let s_100_13: Bits = Bits::new(s_100_12 as u128, 64u16);
        // D s_100_14: read-var t:i
        let s_100_14: i128 = fn_state.t;
        // D s_100_15: call X_set(s_100_14, s_100_0, s_100_13)
        let s_100_15: () = X_set(state, tracer, s_100_14, s_100_0, s_100_13);
        // N s_100_16: return
        return;
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #64s : i64
        let s_101_0: i64 = 64;
        // C s_101_1: const #64s : i64
        let s_101_1: i64 = 64;
        // C s_101_2: cast zx s_101_1 -> i
        let s_101_2: i128 = (i128::try_from(s_101_1).unwrap());
        // S s_101_3: call __UNKNOWN_bits(s_101_2)
        let s_101_3: Bits = u__UNKNOWN_bits(state, tracer, s_101_2);
        // S s_101_4: cast reint s_101_3 -> u64
        let s_101_4: u64 = (s_101_3.value() as u64);
        // S s_101_5: cast zx s_101_4 -> bv
        let s_101_5: Bits = Bits::new(s_101_4 as u128, 64u16);
        // D s_101_6: read-var t:i
        let s_101_6: i128 = fn_state.t;
        // D s_101_7: call X_set(s_101_6, s_101_0, s_101_5)
        let s_101_7: () = X_set(state, tracer, s_101_6, s_101_0, s_101_5);
        // N s_101_8: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #117u : u32
        let s_102_0: u32 = 117;
        // S s_102_1: call IsFeatureImplemented(s_102_0)
        let s_102_1: bool = IsFeatureImplemented(state, tracer, s_102_0);
        // D s_102_2: write-var gs#57945 <= s_102_1
        fn_state.gs_57945 = s_102_1;
        // N s_102_3: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #90704u : u32
        let s_103_0: u32 = 90704;
        // D s_103_1: read-reg s_103_0:struct
        let s_103_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_103_0 as isize);
            tracer.read_register(s_103_0 as isize, value);
            value
        };
        // D s_103_2: call _get_SCR_EL3_Type_NS(s_103_1)
        let s_103_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_103_1);
        // D s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 1u16);
        // C s_103_4: const #0u : u8
        let s_103_4: bool = false;
        // C s_103_5: cast zx s_103_4 -> bv
        let s_103_5: Bits = Bits::new(s_103_4 as u128, 1u16);
        // D s_103_6: cmp-eq s_103_3 s_103_5
        let s_103_6: bool = ((s_103_3) == (s_103_5));
        // D s_103_7: write-var gs#57944 <= s_103_6
        fn_state.gs_57944 = s_103_6;
        // N s_103_8: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #102552u : u32
        let s_104_0: u32 = 102552;
        // D s_104_1: read-reg s_104_0:struct
        let s_104_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // D s_104_2: call _get_HCR_EL2_Type_E2H(s_104_1)
        let s_104_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_104_1);
        // C s_104_3: const #102552u : u32
        let s_104_3: u32 = 102552;
        // D s_104_4: read-reg s_104_3:struct
        let s_104_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_104_3 as isize);
            tracer.read_register(s_104_3 as isize, value);
            value
        };
        // D s_104_5: call _get_HCR_EL2_Type_TGE(s_104_4)
        let s_104_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_104_4);
        // D s_104_6: cast zx s_104_2 -> bv
        let s_104_6: Bits = Bits::new(s_104_2 as u128, 1u16);
        // D s_104_7: cast zx s_104_5 -> bv
        let s_104_7: Bits = Bits::new(s_104_5 as u128, 1u16);
        // D s_104_8: cast reint s_104_6 -> u128
        let s_104_8: u128 = (s_104_6.value() as u128);
        // D s_104_9: size-of s_104_6
        let s_104_9: u16 = s_104_6.length();
        // D s_104_10: cast reint s_104_7 -> u128
        let s_104_10: u128 = (s_104_7.value() as u128);
        // D s_104_11: size-of s_104_7
        let s_104_11: u16 = s_104_7.length();
        // D s_104_12: lsl s_104_8 s_104_11
        let s_104_12: u128 = s_104_8 << s_104_11;
        // D s_104_13: or s_104_12 s_104_10
        let s_104_13: u128 = ((s_104_12) | (s_104_10));
        // D s_104_14: add s_104_9 s_104_11
        let s_104_14: u16 = (s_104_9 + s_104_11);
        // D s_104_15: create-bits s_104_13 s_104_14
        let s_104_15: Bits = Bits::new(s_104_13, s_104_14);
        // D s_104_16: cast reint s_104_15 -> u8
        let s_104_16: u8 = (s_104_15.value() as u8);
        // D s_104_17: cast zx s_104_16 -> bv
        let s_104_17: Bits = Bits::new(s_104_16 as u128, 2u16);
        // C s_104_18: const #3u : u8
        let s_104_18: u8 = 3;
        // C s_104_19: cast zx s_104_18 -> bv
        let s_104_19: Bits = Bits::new(s_104_18 as u128, 2u16);
        // D s_104_20: cmp-eq s_104_17 s_104_19
        let s_104_20: bool = ((s_104_17) == (s_104_19));
        // D s_104_21: write-var gs#57943 <= s_104_20
        fn_state.gs_57943 = s_104_20;
        // N s_104_22: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #24u : u8
        let s_105_0: u8 = 24;
        // C s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 8u16);
        // C s_105_2: cast zx s_105_1 -> i
        let s_105_2: i128 = (s_105_1.value() as i128);
        // C s_105_3: cast reint s_105_2 -> i64
        let s_105_3: i64 = (s_105_2 as i64);
        // C s_105_4: cast zx s_105_3 -> i
        let s_105_4: i128 = (i128::try_from(s_105_3).unwrap());
        // C s_105_5: const #432u : u32
        let s_105_5: u32 = 432;
        // D s_105_6: read-reg s_105_5:u8
        let s_105_6: u8 = {
            let value = state.read_register::<u8>(s_105_5 as isize);
            tracer.read_register(s_105_5 as isize, value);
            value
        };
        // D s_105_7: call AArch64_SystemAccessTrap(s_105_6, s_105_4)
        let s_105_7: () = AArch64_SystemAccessTrap(state, tracer, s_105_6, s_105_4);
        // N s_105_8: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_106_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 1u16);
        // C s_106_2: const #1u : u8
        let s_106_2: bool = true;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: write-var gs#57942 <= s_106_4
        fn_state.gs_57942 = s_106_4;
        // N s_106_6: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #102552u : u32
        let s_107_0: u32 = 102552;
        // D s_107_1: read-reg s_107_0:struct
        let s_107_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: call _get_HCR_EL2_Type_E2H(s_107_1)
        let s_107_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_107_1);
        // C s_107_3: const #102552u : u32
        let s_107_3: u32 = 102552;
        // D s_107_4: read-reg s_107_3:struct
        let s_107_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_107_3 as isize);
            tracer.read_register(s_107_3 as isize, value);
            value
        };
        // D s_107_5: call _get_HCR_EL2_Type_TGE(s_107_4)
        let s_107_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_107_4);
        // D s_107_6: cast zx s_107_2 -> bv
        let s_107_6: Bits = Bits::new(s_107_2 as u128, 1u16);
        // D s_107_7: cast zx s_107_5 -> bv
        let s_107_7: Bits = Bits::new(s_107_5 as u128, 1u16);
        // D s_107_8: cast reint s_107_6 -> u128
        let s_107_8: u128 = (s_107_6.value() as u128);
        // D s_107_9: size-of s_107_6
        let s_107_9: u16 = s_107_6.length();
        // D s_107_10: cast reint s_107_7 -> u128
        let s_107_10: u128 = (s_107_7.value() as u128);
        // D s_107_11: size-of s_107_7
        let s_107_11: u16 = s_107_7.length();
        // D s_107_12: lsl s_107_8 s_107_11
        let s_107_12: u128 = s_107_8 << s_107_11;
        // D s_107_13: or s_107_12 s_107_10
        let s_107_13: u128 = ((s_107_12) | (s_107_10));
        // D s_107_14: add s_107_9 s_107_11
        let s_107_14: u16 = (s_107_9 + s_107_11);
        // D s_107_15: create-bits s_107_13 s_107_14
        let s_107_15: Bits = Bits::new(s_107_13, s_107_14);
        // D s_107_16: cast reint s_107_15 -> u8
        let s_107_16: u8 = (s_107_15.value() as u8);
        // D s_107_17: cast zx s_107_16 -> bv
        let s_107_17: Bits = Bits::new(s_107_16 as u128, 2u16);
        // C s_107_18: const #3u : u8
        let s_107_18: u8 = 3;
        // C s_107_19: cast zx s_107_18 -> bv
        let s_107_19: Bits = Bits::new(s_107_18 as u128, 2u16);
        // D s_107_20: cmp-ne s_107_17 s_107_19
        let s_107_20: bool = ((s_107_17) != (s_107_19));
        // D s_107_21: write-var gs#57941 <= s_107_20
        fn_state.gs_57941 = s_107_20;
        // N s_107_22: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #24u : u8
        let s_108_0: u8 = 24;
        // C s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 8u16);
        // C s_108_2: cast zx s_108_1 -> i
        let s_108_2: i128 = (s_108_1.value() as i128);
        // C s_108_3: cast reint s_108_2 -> i64
        let s_108_3: i64 = (s_108_2 as i64);
        // C s_108_4: cast zx s_108_3 -> i
        let s_108_4: i128 = (i128::try_from(s_108_3).unwrap());
        // C s_108_5: const #432u : u32
        let s_108_5: u32 = 432;
        // D s_108_6: read-reg s_108_5:u8
        let s_108_6: u8 = {
            let value = state.read_register::<u8>(s_108_5 as isize);
            tracer.read_register(s_108_5 as isize, value);
            value
        };
        // D s_108_7: call AArch64_SystemAccessTrap(s_108_6, s_108_4)
        let s_108_7: () = AArch64_SystemAccessTrap(state, tracer, s_108_6, s_108_4);
        // N s_108_8: return
        return;
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var __CNTHCTL_EL2_EL0VTEN:u8
        let s_109_0: bool = fn_state.u__CNTHCTL_EL2_EL0VTEN;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 1u16);
        // C s_109_2: const #0u : u8
        let s_109_2: bool = false;
        // C s_109_3: cast zx s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 1u16);
        // D s_109_4: cmp-eq s_109_1 s_109_3
        let s_109_4: bool = ((s_109_1) == (s_109_3));
        // D s_109_5: write-var gs#57940 <= s_109_4
        fn_state.gs_57940 = s_109_4;
        // N s_109_6: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #102552u : u32
        let s_110_0: u32 = 102552;
        // D s_110_1: read-reg s_110_0:struct
        let s_110_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_110_0 as isize);
            tracer.read_register(s_110_0 as isize, value);
            value
        };
        // D s_110_2: call _get_HCR_EL2_Type_E2H(s_110_1)
        let s_110_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_110_1);
        // C s_110_3: const #102552u : u32
        let s_110_3: u32 = 102552;
        // D s_110_4: read-reg s_110_3:struct
        let s_110_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_110_3 as isize);
            tracer.read_register(s_110_3 as isize, value);
            value
        };
        // D s_110_5: call _get_HCR_EL2_Type_TGE(s_110_4)
        let s_110_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_110_4);
        // D s_110_6: cast zx s_110_2 -> bv
        let s_110_6: Bits = Bits::new(s_110_2 as u128, 1u16);
        // D s_110_7: cast zx s_110_5 -> bv
        let s_110_7: Bits = Bits::new(s_110_5 as u128, 1u16);
        // D s_110_8: cast reint s_110_6 -> u128
        let s_110_8: u128 = (s_110_6.value() as u128);
        // D s_110_9: size-of s_110_6
        let s_110_9: u16 = s_110_6.length();
        // D s_110_10: cast reint s_110_7 -> u128
        let s_110_10: u128 = (s_110_7.value() as u128);
        // D s_110_11: size-of s_110_7
        let s_110_11: u16 = s_110_7.length();
        // D s_110_12: lsl s_110_8 s_110_11
        let s_110_12: u128 = s_110_8 << s_110_11;
        // D s_110_13: or s_110_12 s_110_10
        let s_110_13: u128 = ((s_110_12) | (s_110_10));
        // D s_110_14: add s_110_9 s_110_11
        let s_110_14: u16 = (s_110_9 + s_110_11);
        // D s_110_15: create-bits s_110_13 s_110_14
        let s_110_15: Bits = Bits::new(s_110_13, s_110_14);
        // D s_110_16: cast reint s_110_15 -> u8
        let s_110_16: u8 = (s_110_15.value() as u8);
        // D s_110_17: cast zx s_110_16 -> bv
        let s_110_17: Bits = Bits::new(s_110_16 as u128, 2u16);
        // C s_110_18: const #3u : u8
        let s_110_18: u8 = 3;
        // C s_110_19: cast zx s_110_18 -> bv
        let s_110_19: Bits = Bits::new(s_110_18 as u128, 2u16);
        // D s_110_20: cmp-eq s_110_17 s_110_19
        let s_110_20: bool = ((s_110_17) == (s_110_19));
        // D s_110_21: write-var gs#57939 <= s_110_20
        fn_state.gs_57939 = s_110_20;
        // N s_110_22: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #() : ()
        let s_111_0: () = ();
        // S s_111_1: call EL2Enabled(s_111_0)
        let s_111_1: bool = EL2Enabled(state, tracer, s_111_0);
        // N s_111_2: branch s_111_1 b116 b112
        if s_111_1 {
            return block_116(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#57962 <= s_112_0
        fn_state.gs_57962 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#57962:u8
        let s_113_0: bool = fn_state.gs_57962;
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
        // C s_114_0: const #24u : u8
        let s_114_0: u8 = 24;
        // C s_114_1: cast zx s_114_0 -> bv
        let s_114_1: Bits = Bits::new(s_114_0 as u128, 8u16);
        // C s_114_2: cast zx s_114_1 -> i
        let s_114_2: i128 = (s_114_1.value() as i128);
        // C s_114_3: cast reint s_114_2 -> i64
        let s_114_3: i64 = (s_114_2 as i64);
        // C s_114_4: cast zx s_114_3 -> i
        let s_114_4: i128 = (i128::try_from(s_114_3).unwrap());
        // C s_114_5: const #440u : u32
        let s_114_5: u32 = 440;
        // D s_114_6: read-reg s_114_5:u8
        let s_114_6: u8 = {
            let value = state.read_register::<u8>(s_114_5 as isize);
            tracer.read_register(s_114_5 as isize, value);
            value
        };
        // D s_114_7: call AArch64_SystemAccessTrap(s_114_6, s_114_4)
        let s_114_7: () = AArch64_SystemAccessTrap(state, tracer, s_114_6, s_114_4);
        // N s_114_8: return
        return;
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #24u : u8
        let s_115_0: u8 = 24;
        // C s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 8u16);
        // C s_115_2: cast zx s_115_1 -> i
        let s_115_2: i128 = (s_115_1.value() as i128);
        // C s_115_3: cast reint s_115_2 -> i64
        let s_115_3: i64 = (s_115_2 as i64);
        // C s_115_4: cast zx s_115_3 -> i
        let s_115_4: i128 = (i128::try_from(s_115_3).unwrap());
        // C s_115_5: const #432u : u32
        let s_115_5: u32 = 432;
        // D s_115_6: read-reg s_115_5:u8
        let s_115_6: u8 = {
            let value = state.read_register::<u8>(s_115_5 as isize);
            tracer.read_register(s_115_5 as isize, value);
            value
        };
        // D s_115_7: call AArch64_SystemAccessTrap(s_115_6, s_115_4)
        let s_115_7: () = AArch64_SystemAccessTrap(state, tracer, s_115_6, s_115_4);
        // N s_115_8: return
        return;
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var __HCR_EL2_TGE:u8
        let s_116_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 1u16);
        // C s_116_2: const #1u : u8
        let s_116_2: bool = true;
        // C s_116_3: cast zx s_116_2 -> bv
        let s_116_3: Bits = Bits::new(s_116_2 as u128, 1u16);
        // D s_116_4: cmp-eq s_116_1 s_116_3
        let s_116_4: bool = ((s_116_1) == (s_116_3));
        // D s_116_5: write-var gs#57962 <= s_116_4
        fn_state.gs_57962 = s_116_4;
        // N s_116_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var __CNTKCTL_EL1_EL0VTEN:u8
        let s_117_0: bool = fn_state.u__CNTKCTL_EL1_EL0VTEN;
        // D s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 1u16);
        // C s_117_2: const #0u : u8
        let s_117_2: bool = false;
        // C s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 1u16);
        // D s_117_4: cmp-eq s_117_1 s_117_3
        let s_117_4: bool = ((s_117_1) == (s_117_3));
        // D s_117_5: write-var gs#57938 <= s_117_4
        fn_state.gs_57938 = s_117_4;
        // N s_117_6: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #102552u : u32
        let s_118_0: u32 = 102552;
        // D s_118_1: read-reg s_118_0:struct
        let s_118_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_118_0 as isize);
            tracer.read_register(s_118_0 as isize, value);
            value
        };
        // D s_118_2: call _get_HCR_EL2_Type_E2H(s_118_1)
        let s_118_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_118_1);
        // C s_118_3: const #102552u : u32
        let s_118_3: u32 = 102552;
        // D s_118_4: read-reg s_118_3:struct
        let s_118_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_118_3 as isize);
            tracer.read_register(s_118_3 as isize, value);
            value
        };
        // D s_118_5: call _get_HCR_EL2_Type_TGE(s_118_4)
        let s_118_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_118_4);
        // D s_118_6: cast zx s_118_2 -> bv
        let s_118_6: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_7: cast zx s_118_5 -> bv
        let s_118_7: Bits = Bits::new(s_118_5 as u128, 1u16);
        // D s_118_8: cast reint s_118_6 -> u128
        let s_118_8: u128 = (s_118_6.value() as u128);
        // D s_118_9: size-of s_118_6
        let s_118_9: u16 = s_118_6.length();
        // D s_118_10: cast reint s_118_7 -> u128
        let s_118_10: u128 = (s_118_7.value() as u128);
        // D s_118_11: size-of s_118_7
        let s_118_11: u16 = s_118_7.length();
        // D s_118_12: lsl s_118_8 s_118_11
        let s_118_12: u128 = s_118_8 << s_118_11;
        // D s_118_13: or s_118_12 s_118_10
        let s_118_13: u128 = ((s_118_12) | (s_118_10));
        // D s_118_14: add s_118_9 s_118_11
        let s_118_14: u16 = (s_118_9 + s_118_11);
        // D s_118_15: create-bits s_118_13 s_118_14
        let s_118_15: Bits = Bits::new(s_118_13, s_118_14);
        // D s_118_16: cast reint s_118_15 -> u8
        let s_118_16: u8 = (s_118_15.value() as u8);
        // D s_118_17: cast zx s_118_16 -> bv
        let s_118_17: Bits = Bits::new(s_118_16 as u128, 2u16);
        // C s_118_18: const #3u : u8
        let s_118_18: u8 = 3;
        // C s_118_19: cast zx s_118_18 -> bv
        let s_118_19: Bits = Bits::new(s_118_18 as u128, 2u16);
        // D s_118_20: cmp-eq s_118_17 s_118_19
        let s_118_20: bool = ((s_118_17) == (s_118_19));
        // D s_118_21: write-var gs#57937 <= s_118_20
        fn_state.gs_57937 = s_118_20;
        // N s_118_22: jump b56
        return block_56(state, tracer, fn_state);
    }
}
