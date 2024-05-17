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
use CNTVOFF_read::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_E2H::*;
use AArch32_TakeHypTrapException::*;
use HCR_read::*;
use u_get_CNTV_CTL_Type_ENABLE::*;
use IsFeatureImplemented::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_CNTHVS_CTL_EL2_Type_ENABLE::*;
use CNTV_CTL_read::*;
use u_get_CNTHCTL_EL2_Type_EL1TVT::*;
use u_get_HCR_Type_TGE::*;
use u_get_CNTKCTL_Type_PL0VTEN::*;
use u__UNKNOWN_bits::*;
use CNTV_CVAL_read::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_CNTHCTL_EL2_Type_EL0VTEN::*;
use CNTKCTL_read__1::*;
use PhysicalCountInt::*;
use u_get_CNTKCTL_EL1_Type_EL0VTEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_CNTHV_CTL_EL2_Type_ENABLE::*;
use common::*;
pub fn CNTV_TVAL_SysRegRead32_ac1a47c2d11f8dd9<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRn: u8,
    opc2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_107117: bool,
        ga_167065: ProductType5c790c8ef59cc8b2,
        gs_107116: bool,
        gs_107150: bool,
        gs_107118: bool,
        u__CNTHVS_CTL_EL2_ENABLE: bool,
        gs_107119: bool,
        gs_107124: bool,
        ga_167037: ProductType5c790c8ef59cc8b2,
        ga_166998: ProductType5c790c8ef59cc8b2,
        u__CNTHCTL_EL2_EL1TVT: bool,
        gs_107106: bool,
        gs_107121: bool,
        u__HCR_TGE: bool,
        u__CNTHV_CTL_EL2_ENABLE: bool,
        ga_166990: ProductType5c790c8ef59cc8b2,
        u__CNTKCTL_PL0VTEN: bool,
        u__CNTKCTL_EL1_EL0VTEN: bool,
        gs_107130: bool,
        ga_167005: ProductType5c790c8ef59cc8b2,
        u__PSTATE_EL: u8,
        gs_107151: bool,
        gs_107120: bool,
        gs_107115: bool,
        gs_107152: bool,
        ga_167022: ProductType5c790c8ef59cc8b2,
        ga_167058: ProductType5c790c8ef59cc8b2,
        u__HCR_EL2_TGE: bool,
        gs_107147: bool,
        gs_107122: bool,
        gs_107148: bool,
        u__CNTV_CTL_ENABLE: bool,
        u__CNTHCTL_EL2_EL0VTEN: bool,
        gs_107149: bool,
        gs_107125: bool,
        gs_107133: bool,
        gs_107123: bool,
        gs_107127: bool,
        gs_107131: bool,
        ga_167046: ProductType5c790c8ef59cc8b2,
        gs_107128: bool,
        gs_107107: bool,
        gs_107129: bool,
        gs_107126: bool,
        gs_107132: bool,
        gs_107104: bool,
        gs_107105: bool,
        ga_167030: ProductType5c790c8ef59cc8b2,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRn: u8,
        opc2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRn,
        opc2,
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
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call CNTKCTL_read__1(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = CNTKCTL_read__1(state, tracer, s_0_11);
        // S s_0_13: call _get_CNTKCTL_Type_PL0VTEN(s_0_12)
        let s_0_13: bool = u_get_CNTKCTL_Type_PL0VTEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTKCTL_PL0VTEN <= s_0_13
        fn_state.u__CNTKCTL_PL0VTEN = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HCR_Type_TGE(s_0_16)
        let s_0_17: bool = u_get_HCR_Type_TGE(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_TGE <= s_0_17
        fn_state.u__HCR_TGE = s_0_17;
        // C s_0_19: const #12808u : u32
        let s_0_19: u32 = 12808;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_CNTHCTL_EL2_Type_EL0VTEN(s_0_20)
        let s_0_21: bool = u_get_CNTHCTL_EL2_Type_EL0VTEN(state, tracer, s_0_20);
        // D s_0_22: write-var __CNTHCTL_EL2_EL0VTEN <= s_0_21
        fn_state.u__CNTHCTL_EL2_EL0VTEN = s_0_21;
        // C s_0_23: const #12808u : u32
        let s_0_23: u32 = 12808;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_CNTHCTL_EL2_Type_EL1TVT(s_0_24)
        let s_0_25: bool = u_get_CNTHCTL_EL2_Type_EL1TVT(state, tracer, s_0_24);
        // D s_0_26: write-var __CNTHCTL_EL2_EL1TVT <= s_0_25
        fn_state.u__CNTHCTL_EL2_EL1TVT = s_0_25;
        // C s_0_27: const #14872u : u32
        let s_0_27: u32 = 14872;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_CNTHVS_CTL_EL2_Type_ENABLE(s_0_28)
        let s_0_29: bool = u_get_CNTHVS_CTL_EL2_Type_ENABLE(state, tracer, s_0_28);
        // D s_0_30: write-var __CNTHVS_CTL_EL2_ENABLE <= s_0_29
        fn_state.u__CNTHVS_CTL_EL2_ENABLE = s_0_29;
        // C s_0_31: const #19280u : u32
        let s_0_31: u32 = 19280;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_CNTHV_CTL_EL2_Type_ENABLE(s_0_32)
        let s_0_33: bool = u_get_CNTHV_CTL_EL2_Type_ENABLE(state, tracer, s_0_32);
        // D s_0_34: write-var __CNTHV_CTL_EL2_ENABLE <= s_0_33
        fn_state.u__CNTHV_CTL_EL2_ENABLE = s_0_33;
        // C s_0_35: const #() : ()
        let s_0_35: () = ();
        // S s_0_36: call CNTV_CTL_read(s_0_35)
        let s_0_36: ProductType700c18a878c5601b = CNTV_CTL_read(state, tracer, s_0_35);
        // S s_0_37: call _get_CNTV_CTL_Type_ENABLE(s_0_36)
        let s_0_37: bool = u_get_CNTV_CTL_Type_ENABLE(state, tracer, s_0_36);
        // D s_0_38: write-var __CNTV_CTL_ENABLE <= s_0_37
        fn_state.u__CNTV_CTL_ENABLE = s_0_37;
        // D s_0_39: read-var __PSTATE_EL:u8
        let s_0_39: u8 = fn_state.u__PSTATE_EL;
        // D s_0_40: cast zx s_0_39 -> bv
        let s_0_40: Bits = Bits::new(s_0_39 as u128, 2u16);
        // C s_0_41: const #448u : u32
        let s_0_41: u32 = 448;
        // D s_0_42: read-reg s_0_41:u8
        let s_0_42: u8 = {
            let value = state.read_register::<u8>(s_0_41 as isize);
            tracer.read_register(s_0_41 as isize, value);
            value
        };
        // D s_0_43: cast zx s_0_42 -> bv
        let s_0_43: Bits = Bits::new(s_0_42 as u128, 2u16);
        // D s_0_44: cmp-eq s_0_40 s_0_43
        let s_0_44: bool = ((s_0_40) == (s_0_43));
        // N s_0_45: branch s_0_44 b34 b1
        if s_0_44 {
            return block_34(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b13 b2
        if s_1_5 {
            return block_13(state, tracer, fn_state);
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
        // D s_5_0: read-var __CNTV_CTL_ENABLE:u8
        let s_5_0: bool = fn_state.u__CNTV_CTL_ENABLE;
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
        // N s_6_4: branch s_6_3 b8 b7
        if s_6_3 {
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
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call CNTV_CVAL_read(s_7_0)
        let s_7_1: ProductType5c790c8ef59cc8b2 = CNTV_CVAL_read(state, tracer, s_7_0);
        // D s_7_2: write-var ga#167065 <= s_7_1
        fn_state.ga_167065 = s_7_1;
        // D s_7_3: read-var ga#167065.0:struct
        let s_7_3: u64 = fn_state.ga_167065._0;
        // C s_7_4: const #() : ()
        let s_7_4: () = ();
        // S s_7_5: call PhysicalCountInt(s_7_4)
        let s_7_5: u64 = PhysicalCountInt(state, tracer, s_7_4);
        // D s_7_6: cast zx s_7_3 -> bv
        let s_7_6: Bits = Bits::new(s_7_3 as u128, 64u16);
        // S s_7_7: cast zx s_7_5 -> bv
        let s_7_7: Bits = Bits::new(s_7_5 as u128, 64u16);
        // D s_7_8: sub s_7_6 s_7_7
        let s_7_8: Bits = ((s_7_6) - (s_7_7));
        // D s_7_9: cast reint s_7_8 -> u64
        let s_7_9: u64 = (s_7_8.value() as u64);
        // C s_7_10: const #0s : i
        let s_7_10: i128 = 0;
        // D s_7_11: cast zx s_7_9 -> bv
        let s_7_11: Bits = Bits::new(s_7_9 as u128, 64u16);
        // C s_7_12: const #1s : i64
        let s_7_12: i64 = 1;
        // C s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // C s_7_14: const #31s : i
        let s_7_14: i128 = 31;
        // C s_7_15: add s_7_14 s_7_13
        let s_7_15: i128 = (s_7_14 + s_7_13);
        // D s_7_16: bit-extract s_7_11 s_7_10 s_7_15
        let s_7_16: Bits = (Bits::new(
            ((s_7_11) >> (s_7_10)).value(),
            u16::try_from(s_7_15).unwrap(),
        ));
        // D s_7_17: cast reint s_7_16 -> u32
        let s_7_17: u32 = (s_7_16.value() as u32);
        // D s_7_18: read-var t:i
        let s_7_18: i128 = fn_state.t;
        // D s_7_19: call R_set(s_7_18, s_7_17)
        let s_7_19: () = R_set(state, tracer, s_7_18, s_7_17);
        // N s_7_20: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call CNTV_CVAL_read(s_8_0)
        let s_8_1: ProductType5c790c8ef59cc8b2 = CNTV_CVAL_read(state, tracer, s_8_0);
        // D s_8_2: write-var ga#167058 <= s_8_1
        fn_state.ga_167058 = s_8_1;
        // D s_8_3: read-var ga#167058.0:struct
        let s_8_3: u64 = fn_state.ga_167058._0;
        // C s_8_4: const #() : ()
        let s_8_4: () = ();
        // S s_8_5: call PhysicalCountInt(s_8_4)
        let s_8_5: u64 = PhysicalCountInt(state, tracer, s_8_4);
        // C s_8_6: const #() : ()
        let s_8_6: () = ();
        // S s_8_7: call CNTVOFF_read(s_8_6)
        let s_8_7: u64 = CNTVOFF_read(state, tracer, s_8_6);
        // S s_8_8: cast zx s_8_5 -> bv
        let s_8_8: Bits = Bits::new(s_8_5 as u128, 64u16);
        // S s_8_9: cast zx s_8_7 -> bv
        let s_8_9: Bits = Bits::new(s_8_7 as u128, 64u16);
        // S s_8_10: sub s_8_8 s_8_9
        let s_8_10: Bits = ((s_8_8) - (s_8_9));
        // S s_8_11: cast reint s_8_10 -> u64
        let s_8_11: u64 = (s_8_10.value() as u64);
        // D s_8_12: cast zx s_8_3 -> bv
        let s_8_12: Bits = Bits::new(s_8_3 as u128, 64u16);
        // S s_8_13: cast zx s_8_11 -> bv
        let s_8_13: Bits = Bits::new(s_8_11 as u128, 64u16);
        // D s_8_14: sub s_8_12 s_8_13
        let s_8_14: Bits = ((s_8_12) - (s_8_13));
        // D s_8_15: cast reint s_8_14 -> u64
        let s_8_15: u64 = (s_8_14.value() as u64);
        // C s_8_16: const #0s : i
        let s_8_16: i128 = 0;
        // D s_8_17: cast zx s_8_15 -> bv
        let s_8_17: Bits = Bits::new(s_8_15 as u128, 64u16);
        // C s_8_18: const #1s : i64
        let s_8_18: i64 = 1;
        // C s_8_19: cast zx s_8_18 -> i
        let s_8_19: i128 = (i128::try_from(s_8_18).unwrap());
        // C s_8_20: const #31s : i
        let s_8_20: i128 = 31;
        // C s_8_21: add s_8_20 s_8_19
        let s_8_21: i128 = (s_8_20 + s_8_19);
        // D s_8_22: bit-extract s_8_17 s_8_16 s_8_21
        let s_8_22: Bits = (Bits::new(
            ((s_8_17) >> (s_8_16)).value(),
            u16::try_from(s_8_21).unwrap(),
        ));
        // D s_8_23: cast reint s_8_22 -> u32
        let s_8_23: u32 = (s_8_22.value() as u32);
        // D s_8_24: read-var t:i
        let s_8_24: i128 = fn_state.t;
        // D s_8_25: call R_set(s_8_24, s_8_23)
        let s_8_25: () = R_set(state, tracer, s_8_24, s_8_23);
        // N s_8_26: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #32s : i64
        let s_9_0: i64 = 32;
        // C s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // S s_9_2: call __UNKNOWN_bits(s_9_1)
        let s_9_2: Bits = u__UNKNOWN_bits(state, tracer, s_9_1);
        // S s_9_3: cast reint s_9_2 -> u32
        let s_9_3: u32 = (s_9_2.value() as u32);
        // D s_9_4: read-var t:i
        let s_9_4: i128 = fn_state.t;
        // D s_9_5: call R_set(s_9_4, s_9_3)
        let s_9_5: () = R_set(state, tracer, s_9_4, s_9_3);
        // N s_9_6: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var __CNTV_CTL_ENABLE:u8
        let s_10_0: bool = fn_state.u__CNTV_CTL_ENABLE;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #0u : u8
        let s_10_2: bool = false;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
            return block_12(state, tracer, fn_state);
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
        // S s_11_1: call CNTV_CVAL_read(s_11_0)
        let s_11_1: ProductType5c790c8ef59cc8b2 = CNTV_CVAL_read(state, tracer, s_11_0);
        // D s_11_2: write-var ga#167046 <= s_11_1
        fn_state.ga_167046 = s_11_1;
        // D s_11_3: read-var ga#167046.0:struct
        let s_11_3: u64 = fn_state.ga_167046._0;
        // C s_11_4: const #() : ()
        let s_11_4: () = ();
        // S s_11_5: call PhysicalCountInt(s_11_4)
        let s_11_5: u64 = PhysicalCountInt(state, tracer, s_11_4);
        // C s_11_6: const #() : ()
        let s_11_6: () = ();
        // S s_11_7: call CNTVOFF_read(s_11_6)
        let s_11_7: u64 = CNTVOFF_read(state, tracer, s_11_6);
        // S s_11_8: cast zx s_11_5 -> bv
        let s_11_8: Bits = Bits::new(s_11_5 as u128, 64u16);
        // S s_11_9: cast zx s_11_7 -> bv
        let s_11_9: Bits = Bits::new(s_11_7 as u128, 64u16);
        // S s_11_10: sub s_11_8 s_11_9
        let s_11_10: Bits = ((s_11_8) - (s_11_9));
        // S s_11_11: cast reint s_11_10 -> u64
        let s_11_11: u64 = (s_11_10.value() as u64);
        // D s_11_12: cast zx s_11_3 -> bv
        let s_11_12: Bits = Bits::new(s_11_3 as u128, 64u16);
        // S s_11_13: cast zx s_11_11 -> bv
        let s_11_13: Bits = Bits::new(s_11_11 as u128, 64u16);
        // D s_11_14: sub s_11_12 s_11_13
        let s_11_14: Bits = ((s_11_12) - (s_11_13));
        // D s_11_15: cast reint s_11_14 -> u64
        let s_11_15: u64 = (s_11_14.value() as u64);
        // C s_11_16: const #0s : i
        let s_11_16: i128 = 0;
        // D s_11_17: cast zx s_11_15 -> bv
        let s_11_17: Bits = Bits::new(s_11_15 as u128, 64u16);
        // C s_11_18: const #1s : i64
        let s_11_18: i64 = 1;
        // C s_11_19: cast zx s_11_18 -> i
        let s_11_19: i128 = (i128::try_from(s_11_18).unwrap());
        // C s_11_20: const #31s : i
        let s_11_20: i128 = 31;
        // C s_11_21: add s_11_20 s_11_19
        let s_11_21: i128 = (s_11_20 + s_11_19);
        // D s_11_22: bit-extract s_11_17 s_11_16 s_11_21
        let s_11_22: Bits = (Bits::new(
            ((s_11_17) >> (s_11_16)).value(),
            u16::try_from(s_11_21).unwrap(),
        ));
        // D s_11_23: cast reint s_11_22 -> u32
        let s_11_23: u32 = (s_11_22.value() as u32);
        // D s_11_24: read-var t:i
        let s_11_24: i128 = fn_state.t;
        // D s_11_25: call R_set(s_11_24, s_11_23)
        let s_11_25: () = R_set(state, tracer, s_11_24, s_11_23);
        // N s_11_26: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #32s : i64
        let s_12_0: i64 = 32;
        // C s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // S s_12_2: call __UNKNOWN_bits(s_12_1)
        let s_12_2: Bits = u__UNKNOWN_bits(state, tracer, s_12_1);
        // S s_12_3: cast reint s_12_2 -> u32
        let s_12_3: u32 = (s_12_2.value() as u32);
        // D s_12_4: read-var t:i
        let s_12_4: i128 = fn_state.t;
        // D s_12_5: call R_set(s_12_4, s_12_3)
        let s_12_5: () = R_set(state, tracer, s_12_4, s_12_3);
        // N s_12_6: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call EL2Enabled(s_13_0)
        let s_13_1: bool = EL2Enabled(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b33 b14
        if s_13_1 {
            return block_33(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#107104 <= s_14_0
        fn_state.gs_107104 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#107104:u8
        let s_15_0: bool = fn_state.gs_107104;
        // N s_15_1: branch s_15_0 b32 b16
        if s_15_0 {
            return block_32(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#107105 <= s_16_0
        fn_state.gs_107105 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#107105:u8
        let s_17_0: bool = fn_state.gs_107105;
        // N s_17_1: branch s_17_0 b31 b18
        if s_17_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var __CNTV_CTL_ENABLE:u8
        let s_18_0: bool = fn_state.u__CNTV_CTL_ENABLE;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #0u : u8
        let s_18_2: bool = false;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b30 b19
        if s_18_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #432u : u32
        let s_19_0: u32 = 432;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // C s_19_2: const #2u : u8
        let s_19_2: u8 = 2;
        // D s_19_3: cmp-lt s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) < (s_19_2));
        // N s_19_4: branch s_19_3 b29 b20
        if s_19_3 {
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
        // D s_20_1: write-var gs#107106 <= s_20_0
        fn_state.gs_107106 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#107106:u8
        let s_21_0: bool = fn_state.gs_107106;
        // N s_21_1: branch s_21_0 b28 b22
        if s_21_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #432u : u32
        let s_22_0: u32 = 432;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // C s_22_2: const #2u : u8
        let s_22_2: u8 = 2;
        // D s_22_3: cmp-lt s_22_1 s_22_2
        let s_22_3: bool = ((s_22_1) < (s_22_2));
        // N s_22_4: branch s_22_3 b27 b23
        if s_22_3 {
            return block_27(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#107107 <= s_23_0
        fn_state.gs_107107 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#107107:u8
        let s_24_0: bool = fn_state.gs_107107;
        // N s_24_1: branch s_24_0 b26 b25
        if s_24_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call CNTV_CVAL_read(s_25_0)
        let s_25_1: ProductType5c790c8ef59cc8b2 = CNTV_CVAL_read(state, tracer, s_25_0);
        // D s_25_2: write-var ga#167037 <= s_25_1
        fn_state.ga_167037 = s_25_1;
        // D s_25_3: read-var ga#167037.0:struct
        let s_25_3: u64 = fn_state.ga_167037._0;
        // C s_25_4: const #() : ()
        let s_25_4: () = ();
        // S s_25_5: call PhysicalCountInt(s_25_4)
        let s_25_5: u64 = PhysicalCountInt(state, tracer, s_25_4);
        // D s_25_6: cast zx s_25_3 -> bv
        let s_25_6: Bits = Bits::new(s_25_3 as u128, 64u16);
        // S s_25_7: cast zx s_25_5 -> bv
        let s_25_7: Bits = Bits::new(s_25_5 as u128, 64u16);
        // D s_25_8: sub s_25_6 s_25_7
        let s_25_8: Bits = ((s_25_6) - (s_25_7));
        // D s_25_9: cast reint s_25_8 -> u64
        let s_25_9: u64 = (s_25_8.value() as u64);
        // C s_25_10: const #0s : i
        let s_25_10: i128 = 0;
        // D s_25_11: cast zx s_25_9 -> bv
        let s_25_11: Bits = Bits::new(s_25_9 as u128, 64u16);
        // C s_25_12: const #1s : i64
        let s_25_12: i64 = 1;
        // C s_25_13: cast zx s_25_12 -> i
        let s_25_13: i128 = (i128::try_from(s_25_12).unwrap());
        // C s_25_14: const #31s : i
        let s_25_14: i128 = 31;
        // C s_25_15: add s_25_14 s_25_13
        let s_25_15: i128 = (s_25_14 + s_25_13);
        // D s_25_16: bit-extract s_25_11 s_25_10 s_25_15
        let s_25_16: Bits = (Bits::new(
            ((s_25_11) >> (s_25_10)).value(),
            u16::try_from(s_25_15).unwrap(),
        ));
        // D s_25_17: cast reint s_25_16 -> u32
        let s_25_17: u32 = (s_25_16.value() as u32);
        // D s_25_18: read-var t:i
        let s_25_18: i128 = fn_state.t;
        // D s_25_19: call R_set(s_25_18, s_25_17)
        let s_25_19: () = R_set(state, tracer, s_25_18, s_25_17);
        // N s_25_20: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call CNTV_CVAL_read(s_26_0)
        let s_26_1: ProductType5c790c8ef59cc8b2 = CNTV_CVAL_read(state, tracer, s_26_0);
        // D s_26_2: write-var ga#167030 <= s_26_1
        fn_state.ga_167030 = s_26_1;
        // D s_26_3: read-var ga#167030.0:struct
        let s_26_3: u64 = fn_state.ga_167030._0;
        // C s_26_4: const #() : ()
        let s_26_4: () = ();
        // S s_26_5: call PhysicalCountInt(s_26_4)
        let s_26_5: u64 = PhysicalCountInt(state, tracer, s_26_4);
        // C s_26_6: const #() : ()
        let s_26_6: () = ();
        // S s_26_7: call CNTVOFF_read(s_26_6)
        let s_26_7: u64 = CNTVOFF_read(state, tracer, s_26_6);
        // S s_26_8: cast zx s_26_5 -> bv
        let s_26_8: Bits = Bits::new(s_26_5 as u128, 64u16);
        // S s_26_9: cast zx s_26_7 -> bv
        let s_26_9: Bits = Bits::new(s_26_7 as u128, 64u16);
        // S s_26_10: sub s_26_8 s_26_9
        let s_26_10: Bits = ((s_26_8) - (s_26_9));
        // S s_26_11: cast reint s_26_10 -> u64
        let s_26_11: u64 = (s_26_10.value() as u64);
        // D s_26_12: cast zx s_26_3 -> bv
        let s_26_12: Bits = Bits::new(s_26_3 as u128, 64u16);
        // S s_26_13: cast zx s_26_11 -> bv
        let s_26_13: Bits = Bits::new(s_26_11 as u128, 64u16);
        // D s_26_14: sub s_26_12 s_26_13
        let s_26_14: Bits = ((s_26_12) - (s_26_13));
        // D s_26_15: cast reint s_26_14 -> u64
        let s_26_15: u64 = (s_26_14.value() as u64);
        // C s_26_16: const #0s : i
        let s_26_16: i128 = 0;
        // D s_26_17: cast zx s_26_15 -> bv
        let s_26_17: Bits = Bits::new(s_26_15 as u128, 64u16);
        // C s_26_18: const #1s : i64
        let s_26_18: i64 = 1;
        // C s_26_19: cast zx s_26_18 -> i
        let s_26_19: i128 = (i128::try_from(s_26_18).unwrap());
        // C s_26_20: const #31s : i
        let s_26_20: i128 = 31;
        // C s_26_21: add s_26_20 s_26_19
        let s_26_21: i128 = (s_26_20 + s_26_19);
        // D s_26_22: bit-extract s_26_17 s_26_16 s_26_21
        let s_26_22: Bits = (Bits::new(
            ((s_26_17) >> (s_26_16)).value(),
            u16::try_from(s_26_21).unwrap(),
        ));
        // D s_26_23: cast reint s_26_22 -> u32
        let s_26_23: u32 = (s_26_22.value() as u32);
        // D s_26_24: read-var t:i
        let s_26_24: i128 = fn_state.t;
        // D s_26_25: call R_set(s_26_24, s_26_23)
        let s_26_25: () = R_set(state, tracer, s_26_24, s_26_23);
        // N s_26_26: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #432u : u32
        let s_27_0: u32 = 432;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call ELUsingAArch32(s_27_1)
        let s_27_2: bool = ELUsingAArch32(state, tracer, s_27_1);
        // D s_27_3: write-var gs#107107 <= s_27_2
        fn_state.gs_107107 = s_27_2;
        // N s_27_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call CNTV_CVAL_read(s_28_0)
        let s_28_1: ProductType5c790c8ef59cc8b2 = CNTV_CVAL_read(state, tracer, s_28_0);
        // D s_28_2: write-var ga#167022 <= s_28_1
        fn_state.ga_167022 = s_28_1;
        // D s_28_3: read-var ga#167022.0:struct
        let s_28_3: u64 = fn_state.ga_167022._0;
        // C s_28_4: const #() : ()
        let s_28_4: () = ();
        // S s_28_5: call PhysicalCountInt(s_28_4)
        let s_28_5: u64 = PhysicalCountInt(state, tracer, s_28_4);
        // S s_28_6: cast zx s_28_5 -> bv
        let s_28_6: Bits = Bits::new(s_28_5 as u128, 64u16);
        // C s_28_7: const #22400u : u32
        let s_28_7: u32 = 22400;
        // D s_28_8: read-reg s_28_7:u64
        let s_28_8: u64 = {
            let value = state.read_register::<u64>(s_28_7 as isize);
            tracer.read_register(s_28_7 as isize, value);
            value
        };
        // D s_28_9: cast zx s_28_8 -> bv
        let s_28_9: Bits = Bits::new(s_28_8 as u128, 64u16);
        // D s_28_10: sub s_28_6 s_28_9
        let s_28_10: Bits = ((s_28_6) - (s_28_9));
        // D s_28_11: cast reint s_28_10 -> u64
        let s_28_11: u64 = (s_28_10.value() as u64);
        // D s_28_12: cast zx s_28_3 -> bv
        let s_28_12: Bits = Bits::new(s_28_3 as u128, 64u16);
        // D s_28_13: cast zx s_28_11 -> bv
        let s_28_13: Bits = Bits::new(s_28_11 as u128, 64u16);
        // D s_28_14: sub s_28_12 s_28_13
        let s_28_14: Bits = ((s_28_12) - (s_28_13));
        // D s_28_15: cast reint s_28_14 -> u64
        let s_28_15: u64 = (s_28_14.value() as u64);
        // C s_28_16: const #0s : i
        let s_28_16: i128 = 0;
        // D s_28_17: cast zx s_28_15 -> bv
        let s_28_17: Bits = Bits::new(s_28_15 as u128, 64u16);
        // C s_28_18: const #1s : i64
        let s_28_18: i64 = 1;
        // C s_28_19: cast zx s_28_18 -> i
        let s_28_19: i128 = (i128::try_from(s_28_18).unwrap());
        // C s_28_20: const #31s : i
        let s_28_20: i128 = 31;
        // C s_28_21: add s_28_20 s_28_19
        let s_28_21: i128 = (s_28_20 + s_28_19);
        // D s_28_22: bit-extract s_28_17 s_28_16 s_28_21
        let s_28_22: Bits = (Bits::new(
            ((s_28_17) >> (s_28_16)).value(),
            u16::try_from(s_28_21).unwrap(),
        ));
        // D s_28_23: cast reint s_28_22 -> u32
        let s_28_23: u32 = (s_28_22.value() as u32);
        // D s_28_24: read-var t:i
        let s_28_24: i128 = fn_state.t;
        // D s_28_25: call R_set(s_28_24, s_28_23)
        let s_28_25: () = R_set(state, tracer, s_28_24, s_28_23);
        // N s_28_26: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #432u : u32
        let s_29_0: u32 = 432;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call ELUsingAArch32(s_29_1)
        let s_29_2: bool = ELUsingAArch32(state, tracer, s_29_1);
        // D s_29_3: not s_29_2
        let s_29_3: bool = !s_29_2;
        // D s_29_4: write-var gs#107106 <= s_29_3
        fn_state.gs_107106 = s_29_3;
        // N s_29_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #32s : i64
        let s_30_0: i64 = 32;
        // C s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // S s_30_2: call __UNKNOWN_bits(s_30_1)
        let s_30_2: Bits = u__UNKNOWN_bits(state, tracer, s_30_1);
        // S s_30_3: cast reint s_30_2 -> u32
        let s_30_3: u32 = (s_30_2.value() as u32);
        // D s_30_4: read-var t:i
        let s_30_4: i128 = fn_state.t;
        // D s_30_5: call R_set(s_30_4, s_30_3)
        let s_30_5: () = R_set(state, tracer, s_30_4, s_30_3);
        // N s_30_6: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #3u : u8
        let s_31_0: u8 = 3;
        // C s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 8u16);
        // C s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (s_31_1.value() as i128);
        // C s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: cast zx s_31_3 -> i
        let s_31_4: i128 = (i128::try_from(s_31_3).unwrap());
        // C s_31_5: const #432u : u32
        let s_31_5: u32 = 432;
        // D s_31_6: read-reg s_31_5:u8
        let s_31_6: u8 = {
            let value = state.read_register::<u8>(s_31_5 as isize);
            tracer.read_register(s_31_5 as isize, value);
            value
        };
        // D s_31_7: call AArch64_AArch32SystemAccessTrap(s_31_6, s_31_4)
        let s_31_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_31_6, s_31_4);
        // N s_31_8: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_32_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#107105 <= s_32_4
        fn_state.gs_107105 = s_32_4;
        // N s_32_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #432u : u32
        let s_33_0: u32 = 432;
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
        // D s_33_4: write-var gs#107104 <= s_33_3
        fn_state.gs_107104 = s_33_3;
        // N s_33_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #440u : u32
        let s_34_0: u32 = 440;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: call ELUsingAArch32(s_34_1)
        let s_34_2: bool = ELUsingAArch32(state, tracer, s_34_1);
        // D s_34_3: not s_34_2
        let s_34_3: bool = !s_34_2;
        // N s_34_4: branch s_34_3 b134 b35
        if s_34_3 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#107116 <= s_35_0
        fn_state.gs_107116 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#107116:u8
        let s_36_0: bool = fn_state.gs_107116;
        // N s_36_1: branch s_36_0 b133 b37
        if s_36_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#107117 <= s_37_0
        fn_state.gs_107117 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#107117:u8
        let s_38_0: bool = fn_state.gs_107117;
        // N s_38_1: branch s_38_0 b124 b39
        if s_38_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #440u : u32
        let s_39_0: u32 = 440;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: u8 = {
            let value = state.read_register::<u8>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call ELUsingAArch32(s_39_1)
        let s_39_2: bool = ELUsingAArch32(state, tracer, s_39_1);
        // N s_39_3: branch s_39_2 b123 b40
        if s_39_2 {
            return block_123(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#107118 <= s_40_0
        fn_state.gs_107118 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#107118:u8
        let s_41_0: bool = fn_state.gs_107118;
        // N s_41_1: branch s_41_0 b106 b42
        if s_41_0 {
            return block_106(state, tracer, fn_state);
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
        // N s_42_2: branch s_42_1 b105 b43
        if s_42_1 {
            return block_105(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#107119 <= s_43_0
        fn_state.gs_107119 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#107119:u8
        let s_44_0: bool = fn_state.gs_107119;
        // N s_44_1: branch s_44_0 b104 b45
        if s_44_0 {
            return block_104(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#107120 <= s_45_0
        fn_state.gs_107120 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#107120:u8
        let s_46_0: bool = fn_state.gs_107120;
        // N s_46_1: branch s_46_0 b103 b47
        if s_46_0 {
            return block_103(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#107121 <= s_47_0
        fn_state.gs_107121 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#107121:u8
        let s_48_0: bool = fn_state.gs_107121;
        // N s_48_1: branch s_48_0 b102 b49
        if s_48_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call EL2Enabled(s_49_0)
        let s_49_1: bool = EL2Enabled(state, tracer, s_49_0);
        // N s_49_2: branch s_49_1 b101 b50
        if s_49_1 {
            return block_101(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#107122 <= s_50_0
        fn_state.gs_107122 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#107122:u8
        let s_51_0: bool = fn_state.gs_107122;
        // N s_51_1: branch s_51_0 b100 b52
        if s_51_0 {
            return block_100(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#107123 <= s_52_0
        fn_state.gs_107123 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#107123:u8
        let s_53_0: bool = fn_state.gs_107123;
        // N s_53_1: branch s_53_0 b99 b54
        if s_53_0 {
            return block_99(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#107124 <= s_54_0
        fn_state.gs_107124 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#107124:u8
        let s_55_0: bool = fn_state.gs_107124;
        // N s_55_1: branch s_55_0 b98 b56
        if s_55_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call EL2Enabled(s_56_0)
        let s_56_1: bool = EL2Enabled(state, tracer, s_56_0);
        // N s_56_2: branch s_56_1 b97 b57
        if s_56_1 {
            return block_97(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#107125 <= s_57_0
        fn_state.gs_107125 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#107125:u8
        let s_58_0: bool = fn_state.gs_107125;
        // N s_58_1: branch s_58_0 b96 b59
        if s_58_0 {
            return block_96(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#107126 <= s_59_0
        fn_state.gs_107126 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#107126:u8
        let s_60_0: bool = fn_state.gs_107126;
        // N s_60_1: branch s_60_0 b95 b61
        if s_60_0 {
            return block_95(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#107127 <= s_61_0
        fn_state.gs_107127 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#107127:u8
        let s_62_0: bool = fn_state.gs_107127;
        // N s_62_1: branch s_62_0 b94 b63
        if s_62_0 {
            return block_94(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#107128 <= s_63_0
        fn_state.gs_107128 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#107128:u8
        let s_64_0: bool = fn_state.gs_107128;
        // N s_64_1: branch s_64_0 b91 b65
        if s_64_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call EL2Enabled(s_65_0)
        let s_65_1: bool = EL2Enabled(state, tracer, s_65_0);
        // N s_65_2: branch s_65_1 b90 b66
        if s_65_1 {
            return block_90(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#107129 <= s_66_0
        fn_state.gs_107129 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#107129:u8
        let s_67_0: bool = fn_state.gs_107129;
        // N s_67_1: branch s_67_0 b89 b68
        if s_67_0 {
            return block_89(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#107130 <= s_68_0
        fn_state.gs_107130 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#107130:u8
        let s_69_0: bool = fn_state.gs_107130;
        // N s_69_1: branch s_69_0 b88 b70
        if s_69_0 {
            return block_88(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#107131 <= s_70_0
        fn_state.gs_107131 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#107131:u8
        let s_71_0: bool = fn_state.gs_107131;
        // N s_71_1: branch s_71_0 b85 b72
        if s_71_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var __CNTV_CTL_ENABLE:u8
        let s_72_0: bool = fn_state.u__CNTV_CTL_ENABLE;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #0u : u8
        let s_72_2: bool = false;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // N s_72_5: branch s_72_4 b84 b73
        if s_72_4 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #432u : u32
        let s_73_0: u32 = 432;
        // D s_73_1: read-reg s_73_0:u8
        let s_73_1: u8 = {
            let value = state.read_register::<u8>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // C s_73_2: const #2u : u8
        let s_73_2: u8 = 2;
        // D s_73_3: cmp-lt s_73_1 s_73_2
        let s_73_3: bool = ((s_73_1) < (s_73_2));
        // N s_73_4: branch s_73_3 b83 b74
        if s_73_3 {
            return block_83(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#107132 <= s_74_0
        fn_state.gs_107132 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#107132:u8
        let s_75_0: bool = fn_state.gs_107132;
        // N s_75_1: branch s_75_0 b82 b76
        if s_75_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #432u : u32
        let s_76_0: u32 = 432;
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
        // N s_76_4: branch s_76_3 b81 b77
        if s_76_3 {
            return block_81(state, tracer, fn_state);
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
        // D s_77_1: write-var gs#107133 <= s_77_0
        fn_state.gs_107133 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#107133:u8
        let s_78_0: bool = fn_state.gs_107133;
        // N s_78_1: branch s_78_0 b80 b79
        if s_78_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call CNTV_CVAL_read(s_79_0)
        let s_79_1: ProductType5c790c8ef59cc8b2 = CNTV_CVAL_read(state, tracer, s_79_0);
        // D s_79_2: write-var ga#167005 <= s_79_1
        fn_state.ga_167005 = s_79_1;
        // D s_79_3: read-var ga#167005.0:struct
        let s_79_3: u64 = fn_state.ga_167005._0;
        // C s_79_4: const #() : ()
        let s_79_4: () = ();
        // S s_79_5: call PhysicalCountInt(s_79_4)
        let s_79_5: u64 = PhysicalCountInt(state, tracer, s_79_4);
        // D s_79_6: cast zx s_79_3 -> bv
        let s_79_6: Bits = Bits::new(s_79_3 as u128, 64u16);
        // S s_79_7: cast zx s_79_5 -> bv
        let s_79_7: Bits = Bits::new(s_79_5 as u128, 64u16);
        // D s_79_8: sub s_79_6 s_79_7
        let s_79_8: Bits = ((s_79_6) - (s_79_7));
        // D s_79_9: cast reint s_79_8 -> u64
        let s_79_9: u64 = (s_79_8.value() as u64);
        // C s_79_10: const #0s : i
        let s_79_10: i128 = 0;
        // D s_79_11: cast zx s_79_9 -> bv
        let s_79_11: Bits = Bits::new(s_79_9 as u128, 64u16);
        // C s_79_12: const #1s : i64
        let s_79_12: i64 = 1;
        // C s_79_13: cast zx s_79_12 -> i
        let s_79_13: i128 = (i128::try_from(s_79_12).unwrap());
        // C s_79_14: const #31s : i
        let s_79_14: i128 = 31;
        // C s_79_15: add s_79_14 s_79_13
        let s_79_15: i128 = (s_79_14 + s_79_13);
        // D s_79_16: bit-extract s_79_11 s_79_10 s_79_15
        let s_79_16: Bits = (Bits::new(
            ((s_79_11) >> (s_79_10)).value(),
            u16::try_from(s_79_15).unwrap(),
        ));
        // D s_79_17: cast reint s_79_16 -> u32
        let s_79_17: u32 = (s_79_16.value() as u32);
        // D s_79_18: read-var t:i
        let s_79_18: i128 = fn_state.t;
        // D s_79_19: call R_set(s_79_18, s_79_17)
        let s_79_19: () = R_set(state, tracer, s_79_18, s_79_17);
        // N s_79_20: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call CNTV_CVAL_read(s_80_0)
        let s_80_1: ProductType5c790c8ef59cc8b2 = CNTV_CVAL_read(state, tracer, s_80_0);
        // D s_80_2: write-var ga#166998 <= s_80_1
        fn_state.ga_166998 = s_80_1;
        // D s_80_3: read-var ga#166998.0:struct
        let s_80_3: u64 = fn_state.ga_166998._0;
        // C s_80_4: const #() : ()
        let s_80_4: () = ();
        // S s_80_5: call PhysicalCountInt(s_80_4)
        let s_80_5: u64 = PhysicalCountInt(state, tracer, s_80_4);
        // C s_80_6: const #() : ()
        let s_80_6: () = ();
        // S s_80_7: call CNTVOFF_read(s_80_6)
        let s_80_7: u64 = CNTVOFF_read(state, tracer, s_80_6);
        // S s_80_8: cast zx s_80_5 -> bv
        let s_80_8: Bits = Bits::new(s_80_5 as u128, 64u16);
        // S s_80_9: cast zx s_80_7 -> bv
        let s_80_9: Bits = Bits::new(s_80_7 as u128, 64u16);
        // S s_80_10: sub s_80_8 s_80_9
        let s_80_10: Bits = ((s_80_8) - (s_80_9));
        // S s_80_11: cast reint s_80_10 -> u64
        let s_80_11: u64 = (s_80_10.value() as u64);
        // D s_80_12: cast zx s_80_3 -> bv
        let s_80_12: Bits = Bits::new(s_80_3 as u128, 64u16);
        // S s_80_13: cast zx s_80_11 -> bv
        let s_80_13: Bits = Bits::new(s_80_11 as u128, 64u16);
        // D s_80_14: sub s_80_12 s_80_13
        let s_80_14: Bits = ((s_80_12) - (s_80_13));
        // D s_80_15: cast reint s_80_14 -> u64
        let s_80_15: u64 = (s_80_14.value() as u64);
        // C s_80_16: const #0s : i
        let s_80_16: i128 = 0;
        // D s_80_17: cast zx s_80_15 -> bv
        let s_80_17: Bits = Bits::new(s_80_15 as u128, 64u16);
        // C s_80_18: const #1s : i64
        let s_80_18: i64 = 1;
        // C s_80_19: cast zx s_80_18 -> i
        let s_80_19: i128 = (i128::try_from(s_80_18).unwrap());
        // C s_80_20: const #31s : i
        let s_80_20: i128 = 31;
        // C s_80_21: add s_80_20 s_80_19
        let s_80_21: i128 = (s_80_20 + s_80_19);
        // D s_80_22: bit-extract s_80_17 s_80_16 s_80_21
        let s_80_22: Bits = (Bits::new(
            ((s_80_17) >> (s_80_16)).value(),
            u16::try_from(s_80_21).unwrap(),
        ));
        // D s_80_23: cast reint s_80_22 -> u32
        let s_80_23: u32 = (s_80_22.value() as u32);
        // D s_80_24: read-var t:i
        let s_80_24: i128 = fn_state.t;
        // D s_80_25: call R_set(s_80_24, s_80_23)
        let s_80_25: () = R_set(state, tracer, s_80_24, s_80_23);
        // N s_80_26: return
        return;
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
        // D s_81_2: call ELUsingAArch32(s_81_1)
        let s_81_2: bool = ELUsingAArch32(state, tracer, s_81_1);
        // D s_81_3: write-var gs#107133 <= s_81_2
        fn_state.gs_107133 = s_81_2;
        // N s_81_4: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call CNTV_CVAL_read(s_82_0)
        let s_82_1: ProductType5c790c8ef59cc8b2 = CNTV_CVAL_read(state, tracer, s_82_0);
        // D s_82_2: write-var ga#166990 <= s_82_1
        fn_state.ga_166990 = s_82_1;
        // D s_82_3: read-var ga#166990.0:struct
        let s_82_3: u64 = fn_state.ga_166990._0;
        // C s_82_4: const #() : ()
        let s_82_4: () = ();
        // S s_82_5: call PhysicalCountInt(s_82_4)
        let s_82_5: u64 = PhysicalCountInt(state, tracer, s_82_4);
        // S s_82_6: cast zx s_82_5 -> bv
        let s_82_6: Bits = Bits::new(s_82_5 as u128, 64u16);
        // C s_82_7: const #22400u : u32
        let s_82_7: u32 = 22400;
        // D s_82_8: read-reg s_82_7:u64
        let s_82_8: u64 = {
            let value = state.read_register::<u64>(s_82_7 as isize);
            tracer.read_register(s_82_7 as isize, value);
            value
        };
        // D s_82_9: cast zx s_82_8 -> bv
        let s_82_9: Bits = Bits::new(s_82_8 as u128, 64u16);
        // D s_82_10: sub s_82_6 s_82_9
        let s_82_10: Bits = ((s_82_6) - (s_82_9));
        // D s_82_11: cast reint s_82_10 -> u64
        let s_82_11: u64 = (s_82_10.value() as u64);
        // D s_82_12: cast zx s_82_3 -> bv
        let s_82_12: Bits = Bits::new(s_82_3 as u128, 64u16);
        // D s_82_13: cast zx s_82_11 -> bv
        let s_82_13: Bits = Bits::new(s_82_11 as u128, 64u16);
        // D s_82_14: sub s_82_12 s_82_13
        let s_82_14: Bits = ((s_82_12) - (s_82_13));
        // D s_82_15: cast reint s_82_14 -> u64
        let s_82_15: u64 = (s_82_14.value() as u64);
        // C s_82_16: const #0s : i
        let s_82_16: i128 = 0;
        // D s_82_17: cast zx s_82_15 -> bv
        let s_82_17: Bits = Bits::new(s_82_15 as u128, 64u16);
        // C s_82_18: const #1s : i64
        let s_82_18: i64 = 1;
        // C s_82_19: cast zx s_82_18 -> i
        let s_82_19: i128 = (i128::try_from(s_82_18).unwrap());
        // C s_82_20: const #31s : i
        let s_82_20: i128 = 31;
        // C s_82_21: add s_82_20 s_82_19
        let s_82_21: i128 = (s_82_20 + s_82_19);
        // D s_82_22: bit-extract s_82_17 s_82_16 s_82_21
        let s_82_22: Bits = (Bits::new(
            ((s_82_17) >> (s_82_16)).value(),
            u16::try_from(s_82_21).unwrap(),
        ));
        // D s_82_23: cast reint s_82_22 -> u32
        let s_82_23: u32 = (s_82_22.value() as u32);
        // D s_82_24: read-var t:i
        let s_82_24: i128 = fn_state.t;
        // D s_82_25: call R_set(s_82_24, s_82_23)
        let s_82_25: () = R_set(state, tracer, s_82_24, s_82_23);
        // N s_82_26: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #432u : u32
        let s_83_0: u32 = 432;
        // D s_83_1: read-reg s_83_0:u8
        let s_83_1: u8 = {
            let value = state.read_register::<u8>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: call ELUsingAArch32(s_83_1)
        let s_83_2: bool = ELUsingAArch32(state, tracer, s_83_1);
        // D s_83_3: not s_83_2
        let s_83_3: bool = !s_83_2;
        // D s_83_4: write-var gs#107132 <= s_83_3
        fn_state.gs_107132 = s_83_3;
        // N s_83_5: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #32s : i64
        let s_84_0: i64 = 32;
        // C s_84_1: cast zx s_84_0 -> i
        let s_84_1: i128 = (i128::try_from(s_84_0).unwrap());
        // S s_84_2: call __UNKNOWN_bits(s_84_1)
        let s_84_2: Bits = u__UNKNOWN_bits(state, tracer, s_84_1);
        // S s_84_3: cast reint s_84_2 -> u32
        let s_84_3: u32 = (s_84_2.value() as u32);
        // D s_84_4: read-var t:i
        let s_84_4: i128 = fn_state.t;
        // D s_84_5: call R_set(s_84_4, s_84_3)
        let s_84_5: () = R_set(state, tracer, s_84_4, s_84_3);
        // N s_84_6: return
        return;
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var __CNTHV_CTL_EL2_ENABLE:u8
        let s_85_0: bool = fn_state.u__CNTHV_CTL_EL2_ENABLE;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // C s_85_2: const #0u : u8
        let s_85_2: bool = false;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // N s_85_5: branch s_85_4 b87 b86
        if s_85_4 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #103152u : u32
        let s_86_0: u32 = 103152;
        // D s_86_1: read-reg s_86_0:u64
        let s_86_1: u64 = {
            let value = state.read_register::<u64>(s_86_0 as isize);
            tracer.read_register(s_86_0 as isize, value);
            value
        };
        // C s_86_2: const #() : ()
        let s_86_2: () = ();
        // S s_86_3: call PhysicalCountInt(s_86_2)
        let s_86_3: u64 = PhysicalCountInt(state, tracer, s_86_2);
        // D s_86_4: cast zx s_86_1 -> bv
        let s_86_4: Bits = Bits::new(s_86_1 as u128, 64u16);
        // S s_86_5: cast zx s_86_3 -> bv
        let s_86_5: Bits = Bits::new(s_86_3 as u128, 64u16);
        // D s_86_6: sub s_86_4 s_86_5
        let s_86_6: Bits = ((s_86_4) - (s_86_5));
        // D s_86_7: cast reint s_86_6 -> u64
        let s_86_7: u64 = (s_86_6.value() as u64);
        // C s_86_8: const #0s : i
        let s_86_8: i128 = 0;
        // D s_86_9: cast zx s_86_7 -> bv
        let s_86_9: Bits = Bits::new(s_86_7 as u128, 64u16);
        // C s_86_10: const #1s : i64
        let s_86_10: i64 = 1;
        // C s_86_11: cast zx s_86_10 -> i
        let s_86_11: i128 = (i128::try_from(s_86_10).unwrap());
        // C s_86_12: const #31s : i
        let s_86_12: i128 = 31;
        // C s_86_13: add s_86_12 s_86_11
        let s_86_13: i128 = (s_86_12 + s_86_11);
        // D s_86_14: bit-extract s_86_9 s_86_8 s_86_13
        let s_86_14: Bits = (Bits::new(
            ((s_86_9) >> (s_86_8)).value(),
            u16::try_from(s_86_13).unwrap(),
        ));
        // D s_86_15: cast reint s_86_14 -> u32
        let s_86_15: u32 = (s_86_14.value() as u32);
        // D s_86_16: read-var t:i
        let s_86_16: i128 = fn_state.t;
        // D s_86_17: call R_set(s_86_16, s_86_15)
        let s_86_17: () = R_set(state, tracer, s_86_16, s_86_15);
        // N s_86_18: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #32s : i64
        let s_87_0: i64 = 32;
        // C s_87_1: cast zx s_87_0 -> i
        let s_87_1: i128 = (i128::try_from(s_87_0).unwrap());
        // S s_87_2: call __UNKNOWN_bits(s_87_1)
        let s_87_2: Bits = u__UNKNOWN_bits(state, tracer, s_87_1);
        // S s_87_3: cast reint s_87_2 -> u32
        let s_87_3: u32 = (s_87_2.value() as u32);
        // D s_87_4: read-var t:i
        let s_87_4: i128 = fn_state.t;
        // D s_87_5: call R_set(s_87_4, s_87_3)
        let s_87_5: () = R_set(state, tracer, s_87_4, s_87_3);
        // N s_87_6: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #90704u : u32
        let s_88_0: u32 = 90704;
        // D s_88_1: read-reg s_88_0:struct
        let s_88_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_88_0 as isize);
            tracer.read_register(s_88_0 as isize, value);
            value
        };
        // D s_88_2: call _get_SCR_EL3_Type_NS(s_88_1)
        let s_88_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_88_1);
        // D s_88_3: cast zx s_88_2 -> bv
        let s_88_3: Bits = Bits::new(s_88_2 as u128, 1u16);
        // C s_88_4: const #1u : u8
        let s_88_4: bool = true;
        // C s_88_5: cast zx s_88_4 -> bv
        let s_88_5: Bits = Bits::new(s_88_4 as u128, 1u16);
        // D s_88_6: cmp-eq s_88_3 s_88_5
        let s_88_6: bool = ((s_88_3) == (s_88_5));
        // D s_88_7: write-var gs#107131 <= s_88_6
        fn_state.gs_107131 = s_88_6;
        // N s_88_8: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #102552u : u32
        let s_89_0: u32 = 102552;
        // D s_89_1: read-reg s_89_0:struct
        let s_89_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // D s_89_2: call _get_HCR_EL2_Type_E2H(s_89_1)
        let s_89_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_89_1);
        // C s_89_3: const #102552u : u32
        let s_89_3: u32 = 102552;
        // D s_89_4: read-reg s_89_3:struct
        let s_89_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_89_3 as isize);
            tracer.read_register(s_89_3 as isize, value);
            value
        };
        // D s_89_5: call _get_HCR_EL2_Type_TGE(s_89_4)
        let s_89_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_89_4);
        // D s_89_6: cast zx s_89_2 -> bv
        let s_89_6: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_7: cast zx s_89_5 -> bv
        let s_89_7: Bits = Bits::new(s_89_5 as u128, 1u16);
        // D s_89_8: cast reint s_89_6 -> u128
        let s_89_8: u128 = (s_89_6.value() as u128);
        // D s_89_9: size-of s_89_6
        let s_89_9: u16 = s_89_6.length();
        // D s_89_10: cast reint s_89_7 -> u128
        let s_89_10: u128 = (s_89_7.value() as u128);
        // D s_89_11: size-of s_89_7
        let s_89_11: u16 = s_89_7.length();
        // D s_89_12: lsl s_89_8 s_89_11
        let s_89_12: u128 = s_89_8 << s_89_11;
        // D s_89_13: or s_89_12 s_89_10
        let s_89_13: u128 = ((s_89_12) | (s_89_10));
        // D s_89_14: add s_89_9 s_89_11
        let s_89_14: u16 = (s_89_9 + s_89_11);
        // D s_89_15: create-bits s_89_13 s_89_14
        let s_89_15: Bits = Bits::new(s_89_13, s_89_14);
        // D s_89_16: cast reint s_89_15 -> u8
        let s_89_16: u8 = (s_89_15.value() as u8);
        // D s_89_17: cast zx s_89_16 -> bv
        let s_89_17: Bits = Bits::new(s_89_16 as u128, 2u16);
        // C s_89_18: const #3u : u8
        let s_89_18: u8 = 3;
        // C s_89_19: cast zx s_89_18 -> bv
        let s_89_19: Bits = Bits::new(s_89_18 as u128, 2u16);
        // D s_89_20: cmp-eq s_89_17 s_89_19
        let s_89_20: bool = ((s_89_17) == (s_89_19));
        // D s_89_21: write-var gs#107130 <= s_89_20
        fn_state.gs_107130 = s_89_20;
        // N s_89_22: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #432u : u32
        let s_90_0: u32 = 432;
        // D s_90_1: read-reg s_90_0:u8
        let s_90_1: u8 = {
            let value = state.read_register::<u8>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // D s_90_2: call ELUsingAArch32(s_90_1)
        let s_90_2: bool = ELUsingAArch32(state, tracer, s_90_1);
        // D s_90_3: not s_90_2
        let s_90_3: bool = !s_90_2;
        // D s_90_4: write-var gs#107129 <= s_90_3
        fn_state.gs_107129 = s_90_3;
        // N s_90_5: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var __CNTHVS_CTL_EL2_ENABLE:u8
        let s_91_0: bool = fn_state.u__CNTHVS_CTL_EL2_ENABLE;
        // D s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 1u16);
        // C s_91_2: const #0u : u8
        let s_91_2: bool = false;
        // C s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // D s_91_4: cmp-eq s_91_1 s_91_3
        let s_91_4: bool = ((s_91_1) == (s_91_3));
        // N s_91_5: branch s_91_4 b93 b92
        if s_91_4 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #10064u : u32
        let s_92_0: u32 = 10064;
        // D s_92_1: read-reg s_92_0:u64
        let s_92_1: u64 = {
            let value = state.read_register::<u64>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // C s_92_2: const #() : ()
        let s_92_2: () = ();
        // S s_92_3: call PhysicalCountInt(s_92_2)
        let s_92_3: u64 = PhysicalCountInt(state, tracer, s_92_2);
        // D s_92_4: cast zx s_92_1 -> bv
        let s_92_4: Bits = Bits::new(s_92_1 as u128, 64u16);
        // S s_92_5: cast zx s_92_3 -> bv
        let s_92_5: Bits = Bits::new(s_92_3 as u128, 64u16);
        // D s_92_6: sub s_92_4 s_92_5
        let s_92_6: Bits = ((s_92_4) - (s_92_5));
        // D s_92_7: cast reint s_92_6 -> u64
        let s_92_7: u64 = (s_92_6.value() as u64);
        // C s_92_8: const #0s : i
        let s_92_8: i128 = 0;
        // D s_92_9: cast zx s_92_7 -> bv
        let s_92_9: Bits = Bits::new(s_92_7 as u128, 64u16);
        // C s_92_10: const #1s : i64
        let s_92_10: i64 = 1;
        // C s_92_11: cast zx s_92_10 -> i
        let s_92_11: i128 = (i128::try_from(s_92_10).unwrap());
        // C s_92_12: const #31s : i
        let s_92_12: i128 = 31;
        // C s_92_13: add s_92_12 s_92_11
        let s_92_13: i128 = (s_92_12 + s_92_11);
        // D s_92_14: bit-extract s_92_9 s_92_8 s_92_13
        let s_92_14: Bits = (Bits::new(
            ((s_92_9) >> (s_92_8)).value(),
            u16::try_from(s_92_13).unwrap(),
        ));
        // D s_92_15: cast reint s_92_14 -> u32
        let s_92_15: u32 = (s_92_14.value() as u32);
        // D s_92_16: read-var t:i
        let s_92_16: i128 = fn_state.t;
        // D s_92_17: call R_set(s_92_16, s_92_15)
        let s_92_17: () = R_set(state, tracer, s_92_16, s_92_15);
        // N s_92_18: return
        return;
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #32s : i64
        let s_93_0: i64 = 32;
        // C s_93_1: cast zx s_93_0 -> i
        let s_93_1: i128 = (i128::try_from(s_93_0).unwrap());
        // S s_93_2: call __UNKNOWN_bits(s_93_1)
        let s_93_2: Bits = u__UNKNOWN_bits(state, tracer, s_93_1);
        // S s_93_3: cast reint s_93_2 -> u32
        let s_93_3: u32 = (s_93_2.value() as u32);
        // D s_93_4: read-var t:i
        let s_93_4: i128 = fn_state.t;
        // D s_93_5: call R_set(s_93_4, s_93_3)
        let s_93_5: () = R_set(state, tracer, s_93_4, s_93_3);
        // N s_93_6: return
        return;
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #117u : u32
        let s_94_0: u32 = 117;
        // S s_94_1: call IsFeatureImplemented(s_94_0)
        let s_94_1: bool = IsFeatureImplemented(state, tracer, s_94_0);
        // D s_94_2: write-var gs#107128 <= s_94_1
        fn_state.gs_107128 = s_94_1;
        // N s_94_3: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #90704u : u32
        let s_95_0: u32 = 90704;
        // D s_95_1: read-reg s_95_0:struct
        let s_95_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // D s_95_2: call _get_SCR_EL3_Type_NS(s_95_1)
        let s_95_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_95_1);
        // D s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // C s_95_4: const #0u : u8
        let s_95_4: bool = false;
        // C s_95_5: cast zx s_95_4 -> bv
        let s_95_5: Bits = Bits::new(s_95_4 as u128, 1u16);
        // D s_95_6: cmp-eq s_95_3 s_95_5
        let s_95_6: bool = ((s_95_3) == (s_95_5));
        // D s_95_7: write-var gs#107127 <= s_95_6
        fn_state.gs_107127 = s_95_6;
        // N s_95_8: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #102552u : u32
        let s_96_0: u32 = 102552;
        // D s_96_1: read-reg s_96_0:struct
        let s_96_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // D s_96_2: call _get_HCR_EL2_Type_E2H(s_96_1)
        let s_96_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_96_1);
        // C s_96_3: const #102552u : u32
        let s_96_3: u32 = 102552;
        // D s_96_4: read-reg s_96_3:struct
        let s_96_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_96_3 as isize);
            tracer.read_register(s_96_3 as isize, value);
            value
        };
        // D s_96_5: call _get_HCR_EL2_Type_TGE(s_96_4)
        let s_96_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_96_4);
        // D s_96_6: cast zx s_96_2 -> bv
        let s_96_6: Bits = Bits::new(s_96_2 as u128, 1u16);
        // D s_96_7: cast zx s_96_5 -> bv
        let s_96_7: Bits = Bits::new(s_96_5 as u128, 1u16);
        // D s_96_8: cast reint s_96_6 -> u128
        let s_96_8: u128 = (s_96_6.value() as u128);
        // D s_96_9: size-of s_96_6
        let s_96_9: u16 = s_96_6.length();
        // D s_96_10: cast reint s_96_7 -> u128
        let s_96_10: u128 = (s_96_7.value() as u128);
        // D s_96_11: size-of s_96_7
        let s_96_11: u16 = s_96_7.length();
        // D s_96_12: lsl s_96_8 s_96_11
        let s_96_12: u128 = s_96_8 << s_96_11;
        // D s_96_13: or s_96_12 s_96_10
        let s_96_13: u128 = ((s_96_12) | (s_96_10));
        // D s_96_14: add s_96_9 s_96_11
        let s_96_14: u16 = (s_96_9 + s_96_11);
        // D s_96_15: create-bits s_96_13 s_96_14
        let s_96_15: Bits = Bits::new(s_96_13, s_96_14);
        // D s_96_16: cast reint s_96_15 -> u8
        let s_96_16: u8 = (s_96_15.value() as u8);
        // D s_96_17: cast zx s_96_16 -> bv
        let s_96_17: Bits = Bits::new(s_96_16 as u128, 2u16);
        // C s_96_18: const #3u : u8
        let s_96_18: u8 = 3;
        // C s_96_19: cast zx s_96_18 -> bv
        let s_96_19: Bits = Bits::new(s_96_18 as u128, 2u16);
        // D s_96_20: cmp-eq s_96_17 s_96_19
        let s_96_20: bool = ((s_96_17) == (s_96_19));
        // D s_96_21: write-var gs#107126 <= s_96_20
        fn_state.gs_107126 = s_96_20;
        // N s_96_22: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #432u : u32
        let s_97_0: u32 = 432;
        // D s_97_1: read-reg s_97_0:u8
        let s_97_1: u8 = {
            let value = state.read_register::<u8>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // D s_97_2: call ELUsingAArch32(s_97_1)
        let s_97_2: bool = ELUsingAArch32(state, tracer, s_97_1);
        // D s_97_3: not s_97_2
        let s_97_3: bool = !s_97_2;
        // D s_97_4: write-var gs#107125 <= s_97_3
        fn_state.gs_107125 = s_97_3;
        // N s_97_5: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #3u : u8
        let s_98_0: u8 = 3;
        // C s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 8u16);
        // C s_98_2: cast zx s_98_1 -> i
        let s_98_2: i128 = (s_98_1.value() as i128);
        // C s_98_3: cast reint s_98_2 -> i64
        let s_98_3: i64 = (s_98_2 as i64);
        // C s_98_4: cast zx s_98_3 -> i
        let s_98_4: i128 = (i128::try_from(s_98_3).unwrap());
        // C s_98_5: const #432u : u32
        let s_98_5: u32 = 432;
        // D s_98_6: read-reg s_98_5:u8
        let s_98_6: u8 = {
            let value = state.read_register::<u8>(s_98_5 as isize);
            tracer.read_register(s_98_5 as isize, value);
            value
        };
        // D s_98_7: call AArch64_AArch32SystemAccessTrap(s_98_6, s_98_4)
        let s_98_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_98_6, s_98_4);
        // N s_98_8: return
        return;
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_99_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 1u16);
        // C s_99_2: const #1u : u8
        let s_99_2: bool = true;
        // C s_99_3: cast zx s_99_2 -> bv
        let s_99_3: Bits = Bits::new(s_99_2 as u128, 1u16);
        // D s_99_4: cmp-eq s_99_1 s_99_3
        let s_99_4: bool = ((s_99_1) == (s_99_3));
        // D s_99_5: write-var gs#107124 <= s_99_4
        fn_state.gs_107124 = s_99_4;
        // N s_99_6: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #102552u : u32
        let s_100_0: u32 = 102552;
        // D s_100_1: read-reg s_100_0:struct
        let s_100_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_100_0 as isize);
            tracer.read_register(s_100_0 as isize, value);
            value
        };
        // D s_100_2: call _get_HCR_EL2_Type_E2H(s_100_1)
        let s_100_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_100_1);
        // C s_100_3: const #102552u : u32
        let s_100_3: u32 = 102552;
        // D s_100_4: read-reg s_100_3:struct
        let s_100_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_100_3 as isize);
            tracer.read_register(s_100_3 as isize, value);
            value
        };
        // D s_100_5: call _get_HCR_EL2_Type_TGE(s_100_4)
        let s_100_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_100_4);
        // D s_100_6: cast zx s_100_2 -> bv
        let s_100_6: Bits = Bits::new(s_100_2 as u128, 1u16);
        // D s_100_7: cast zx s_100_5 -> bv
        let s_100_7: Bits = Bits::new(s_100_5 as u128, 1u16);
        // D s_100_8: cast reint s_100_6 -> u128
        let s_100_8: u128 = (s_100_6.value() as u128);
        // D s_100_9: size-of s_100_6
        let s_100_9: u16 = s_100_6.length();
        // D s_100_10: cast reint s_100_7 -> u128
        let s_100_10: u128 = (s_100_7.value() as u128);
        // D s_100_11: size-of s_100_7
        let s_100_11: u16 = s_100_7.length();
        // D s_100_12: lsl s_100_8 s_100_11
        let s_100_12: u128 = s_100_8 << s_100_11;
        // D s_100_13: or s_100_12 s_100_10
        let s_100_13: u128 = ((s_100_12) | (s_100_10));
        // D s_100_14: add s_100_9 s_100_11
        let s_100_14: u16 = (s_100_9 + s_100_11);
        // D s_100_15: create-bits s_100_13 s_100_14
        let s_100_15: Bits = Bits::new(s_100_13, s_100_14);
        // D s_100_16: cast reint s_100_15 -> u8
        let s_100_16: u8 = (s_100_15.value() as u8);
        // D s_100_17: cast zx s_100_16 -> bv
        let s_100_17: Bits = Bits::new(s_100_16 as u128, 2u16);
        // C s_100_18: const #3u : u8
        let s_100_18: u8 = 3;
        // C s_100_19: cast zx s_100_18 -> bv
        let s_100_19: Bits = Bits::new(s_100_18 as u128, 2u16);
        // D s_100_20: cmp-ne s_100_17 s_100_19
        let s_100_20: bool = ((s_100_17) != (s_100_19));
        // D s_100_21: write-var gs#107123 <= s_100_20
        fn_state.gs_107123 = s_100_20;
        // N s_100_22: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #432u : u32
        let s_101_0: u32 = 432;
        // D s_101_1: read-reg s_101_0:u8
        let s_101_1: u8 = {
            let value = state.read_register::<u8>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: call ELUsingAArch32(s_101_1)
        let s_101_2: bool = ELUsingAArch32(state, tracer, s_101_1);
        // D s_101_3: not s_101_2
        let s_101_3: bool = !s_101_2;
        // D s_101_4: write-var gs#107122 <= s_101_3
        fn_state.gs_107122 = s_101_3;
        // N s_101_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #3u : u8
        let s_102_0: u8 = 3;
        // C s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 8u16);
        // C s_102_2: cast zx s_102_1 -> i
        let s_102_2: i128 = (s_102_1.value() as i128);
        // C s_102_3: cast reint s_102_2 -> i64
        let s_102_3: i64 = (s_102_2 as i64);
        // C s_102_4: cast zx s_102_3 -> i
        let s_102_4: i128 = (i128::try_from(s_102_3).unwrap());
        // C s_102_5: const #432u : u32
        let s_102_5: u32 = 432;
        // D s_102_6: read-reg s_102_5:u8
        let s_102_6: u8 = {
            let value = state.read_register::<u8>(s_102_5 as isize);
            tracer.read_register(s_102_5 as isize, value);
            value
        };
        // D s_102_7: call AArch64_AArch32SystemAccessTrap(s_102_6, s_102_4)
        let s_102_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_102_6,
            s_102_4,
        );
        // N s_102_8: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var __CNTHCTL_EL2_EL0VTEN:u8
        let s_103_0: bool = fn_state.u__CNTHCTL_EL2_EL0VTEN;
        // D s_103_1: cast zx s_103_0 -> bv
        let s_103_1: Bits = Bits::new(s_103_0 as u128, 1u16);
        // C s_103_2: const #0u : u8
        let s_103_2: bool = false;
        // C s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 1u16);
        // D s_103_4: cmp-eq s_103_1 s_103_3
        let s_103_4: bool = ((s_103_1) == (s_103_3));
        // D s_103_5: write-var gs#107121 <= s_103_4
        fn_state.gs_107121 = s_103_4;
        // N s_103_6: jump b48
        return block_48(state, tracer, fn_state);
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
        // D s_104_21: write-var gs#107120 <= s_104_20
        fn_state.gs_107120 = s_104_20;
        // N s_104_22: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #432u : u32
        let s_105_0: u32 = 432;
        // D s_105_1: read-reg s_105_0:u8
        let s_105_1: u8 = {
            let value = state.read_register::<u8>(s_105_0 as isize);
            tracer.read_register(s_105_0 as isize, value);
            value
        };
        // D s_105_2: call ELUsingAArch32(s_105_1)
        let s_105_2: bool = ELUsingAArch32(state, tracer, s_105_1);
        // D s_105_3: not s_105_2
        let s_105_3: bool = !s_105_2;
        // D s_105_4: write-var gs#107119 <= s_105_3
        fn_state.gs_107119 = s_105_3;
        // N s_105_5: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #() : ()
        let s_106_0: () = ();
        // S s_106_1: call EL2Enabled(s_106_0)
        let s_106_1: bool = EL2Enabled(state, tracer, s_106_0);
        // N s_106_2: branch s_106_1 b122 b107
        if s_106_1 {
            return block_122(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#107147 <= s_107_0
        fn_state.gs_107147 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#107147:u8
        let s_108_0: bool = fn_state.gs_107147;
        // N s_108_1: branch s_108_0 b121 b109
        if s_108_0 {
            return block_121(state, tracer, fn_state);
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
        // D s_109_1: write-var gs#107148 <= s_109_0
        fn_state.gs_107148 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#107148:u8
        let s_110_0: bool = fn_state.gs_107148;
        // N s_110_1: branch s_110_0 b120 b111
        if s_110_0 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
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
        // N s_111_2: branch s_111_1 b119 b112
        if s_111_1 {
            return block_119(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#107149 <= s_112_0
        fn_state.gs_107149 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#107149:u8
        let s_113_0: bool = fn_state.gs_107149;
        // N s_113_1: branch s_113_0 b118 b114
        if s_113_0 {
            return block_118(state, tracer, fn_state);
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
        // D s_114_1: write-var gs#107150 <= s_114_0
        fn_state.gs_107150 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#107150:u8
        let s_115_0: bool = fn_state.gs_107150;
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
        // N s_116_0: panic
        panic!("{:?}", ());
        // N s_116_1: return
        return;
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #0u : u8
        let s_117_0: u8 = 0;
        // C s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 8u16);
        // C s_117_2: cast zx s_117_1 -> i
        let s_117_2: i128 = (s_117_1.value() as i128);
        // C s_117_3: cast reint s_117_2 -> i64
        let s_117_3: i64 = (s_117_2 as i64);
        // C s_117_4: cast zx s_117_3 -> i
        let s_117_4: i128 = (i128::try_from(s_117_3).unwrap());
        // S s_117_5: call AArch32_TakeHypTrapException(s_117_4)
        let s_117_5: () = AArch32_TakeHypTrapException(state, tracer, s_117_4);
        // N s_117_6: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var __HCR_TGE:u8
        let s_118_0: bool = fn_state.u__HCR_TGE;
        // D s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 1u16);
        // C s_118_2: const #1u : u8
        let s_118_2: bool = true;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_4: cmp-eq s_118_1 s_118_3
        let s_118_4: bool = ((s_118_1) == (s_118_3));
        // D s_118_5: write-var gs#107150 <= s_118_4
        fn_state.gs_107150 = s_118_4;
        // N s_118_6: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #432u : u32
        let s_119_0: u32 = 432;
        // D s_119_1: read-reg s_119_0:u8
        let s_119_1: u8 = {
            let value = state.read_register::<u8>(s_119_0 as isize);
            tracer.read_register(s_119_0 as isize, value);
            value
        };
        // D s_119_2: call ELUsingAArch32(s_119_1)
        let s_119_2: bool = ELUsingAArch32(state, tracer, s_119_1);
        // D s_119_3: write-var gs#107149 <= s_119_2
        fn_state.gs_107149 = s_119_2;
        // N s_119_4: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #3u : u8
        let s_120_0: u8 = 3;
        // C s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 8u16);
        // C s_120_2: cast zx s_120_1 -> i
        let s_120_2: i128 = (s_120_1.value() as i128);
        // C s_120_3: cast reint s_120_2 -> i64
        let s_120_3: i64 = (s_120_2 as i64);
        // C s_120_4: cast zx s_120_3 -> i
        let s_120_4: i128 = (i128::try_from(s_120_3).unwrap());
        // C s_120_5: const #432u : u32
        let s_120_5: u32 = 432;
        // D s_120_6: read-reg s_120_5:u8
        let s_120_6: u8 = {
            let value = state.read_register::<u8>(s_120_5 as isize);
            tracer.read_register(s_120_5 as isize, value);
            value
        };
        // D s_120_7: call AArch64_AArch32SystemAccessTrap(s_120_6, s_120_4)
        let s_120_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_120_6,
            s_120_4,
        );
        // N s_120_8: return
        return;
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var __HCR_EL2_TGE:u8
        let s_121_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 1u16);
        // C s_121_2: const #1u : u8
        let s_121_2: bool = true;
        // C s_121_3: cast zx s_121_2 -> bv
        let s_121_3: Bits = Bits::new(s_121_2 as u128, 1u16);
        // D s_121_4: cmp-eq s_121_1 s_121_3
        let s_121_4: bool = ((s_121_1) == (s_121_3));
        // D s_121_5: write-var gs#107148 <= s_121_4
        fn_state.gs_107148 = s_121_4;
        // N s_121_6: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #432u : u32
        let s_122_0: u32 = 432;
        // D s_122_1: read-reg s_122_0:u8
        let s_122_1: u8 = {
            let value = state.read_register::<u8>(s_122_0 as isize);
            tracer.read_register(s_122_0 as isize, value);
            value
        };
        // D s_122_2: call ELUsingAArch32(s_122_1)
        let s_122_2: bool = ELUsingAArch32(state, tracer, s_122_1);
        // D s_122_3: not s_122_2
        let s_122_3: bool = !s_122_2;
        // D s_122_4: write-var gs#107147 <= s_122_3
        fn_state.gs_107147 = s_122_3;
        // N s_122_5: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var __CNTKCTL_PL0VTEN:u8
        let s_123_0: bool = fn_state.u__CNTKCTL_PL0VTEN;
        // D s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 1u16);
        // C s_123_2: const #0u : u8
        let s_123_2: bool = false;
        // C s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 1u16);
        // D s_123_4: cmp-eq s_123_1 s_123_3
        let s_123_4: bool = ((s_123_1) == (s_123_3));
        // D s_123_5: write-var gs#107118 <= s_123_4
        fn_state.gs_107118 = s_123_4;
        // N s_123_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #() : ()
        let s_124_0: () = ();
        // S s_124_1: call EL2Enabled(s_124_0)
        let s_124_1: bool = EL2Enabled(state, tracer, s_124_0);
        // N s_124_2: branch s_124_1 b132 b125
        if s_124_1 {
            return block_132(state, tracer, fn_state);
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
        // D s_125_1: write-var gs#107151 <= s_125_0
        fn_state.gs_107151 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#107151:u8
        let s_126_0: bool = fn_state.gs_107151;
        // N s_126_1: branch s_126_0 b131 b127
        if s_126_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #0u : u8
        let s_127_0: bool = false;
        // D s_127_1: write-var gs#107152 <= s_127_0
        fn_state.gs_107152 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#107152:u8
        let s_128_0: bool = fn_state.gs_107152;
        // N s_128_1: branch s_128_0 b130 b129
        if s_128_0 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #3u : u8
        let s_129_0: u8 = 3;
        // C s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 8u16);
        // C s_129_2: cast zx s_129_1 -> i
        let s_129_2: i128 = (s_129_1.value() as i128);
        // C s_129_3: cast reint s_129_2 -> i64
        let s_129_3: i64 = (s_129_2 as i64);
        // C s_129_4: cast zx s_129_3 -> i
        let s_129_4: i128 = (i128::try_from(s_129_3).unwrap());
        // C s_129_5: const #440u : u32
        let s_129_5: u32 = 440;
        // D s_129_6: read-reg s_129_5:u8
        let s_129_6: u8 = {
            let value = state.read_register::<u8>(s_129_5 as isize);
            tracer.read_register(s_129_5 as isize, value);
            value
        };
        // D s_129_7: call AArch64_AArch32SystemAccessTrap(s_129_6, s_129_4)
        let s_129_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_129_6,
            s_129_4,
        );
        // N s_129_8: return
        return;
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #3u : u8
        let s_130_0: u8 = 3;
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
        // D s_130_7: call AArch64_AArch32SystemAccessTrap(s_130_6, s_130_4)
        let s_130_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_130_6,
            s_130_4,
        );
        // N s_130_8: return
        return;
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var __HCR_EL2_TGE:u8
        let s_131_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_131_1: cast zx s_131_0 -> bv
        let s_131_1: Bits = Bits::new(s_131_0 as u128, 1u16);
        // C s_131_2: const #1u : u8
        let s_131_2: bool = true;
        // C s_131_3: cast zx s_131_2 -> bv
        let s_131_3: Bits = Bits::new(s_131_2 as u128, 1u16);
        // D s_131_4: cmp-eq s_131_1 s_131_3
        let s_131_4: bool = ((s_131_1) == (s_131_3));
        // D s_131_5: write-var gs#107152 <= s_131_4
        fn_state.gs_107152 = s_131_4;
        // N s_131_6: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #432u : u32
        let s_132_0: u32 = 432;
        // D s_132_1: read-reg s_132_0:u8
        let s_132_1: u8 = {
            let value = state.read_register::<u8>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // D s_132_2: call ELUsingAArch32(s_132_1)
        let s_132_2: bool = ELUsingAArch32(state, tracer, s_132_1);
        // D s_132_3: not s_132_2
        let s_132_3: bool = !s_132_2;
        // D s_132_4: write-var gs#107151 <= s_132_3
        fn_state.gs_107151 = s_132_3;
        // N s_132_5: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var __CNTKCTL_EL1_EL0VTEN:u8
        let s_133_0: bool = fn_state.u__CNTKCTL_EL1_EL0VTEN;
        // D s_133_1: cast zx s_133_0 -> bv
        let s_133_1: Bits = Bits::new(s_133_0 as u128, 1u16);
        // C s_133_2: const #0u : u8
        let s_133_2: bool = false;
        // C s_133_3: cast zx s_133_2 -> bv
        let s_133_3: Bits = Bits::new(s_133_2 as u128, 1u16);
        // D s_133_4: cmp-eq s_133_1 s_133_3
        let s_133_4: bool = ((s_133_1) == (s_133_3));
        // D s_133_5: write-var gs#107117 <= s_133_4
        fn_state.gs_107117 = s_133_4;
        // N s_133_6: jump b38
        return block_38(state, tracer, fn_state);
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
        // N s_134_2: branch s_134_1 b137 b135
        if s_134_1 {
            return block_137(state, tracer, fn_state);
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
        // D s_135_1: write-var gs#107115 <= s_135_0
        fn_state.gs_107115 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#107115:u8
        let s_136_0: bool = fn_state.gs_107115;
        // D s_136_1: not s_136_0
        let s_136_1: bool = !s_136_0;
        // D s_136_2: write-var gs#107116 <= s_136_1
        fn_state.gs_107116 = s_136_1;
        // N s_136_3: jump b36
        return block_36(state, tracer, fn_state);
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
        // D s_137_20: cmp-eq s_137_17 s_137_19
        let s_137_20: bool = ((s_137_17) == (s_137_19));
        // D s_137_21: write-var gs#107115 <= s_137_20
        fn_state.gs_107115 = s_137_20;
        // N s_137_22: jump b136
        return block_136(state, tracer, fn_state);
    }
}
