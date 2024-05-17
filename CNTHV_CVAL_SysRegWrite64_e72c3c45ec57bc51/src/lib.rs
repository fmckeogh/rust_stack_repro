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
use Mk_CNTHVS_CVAL_EL2_Type::*;
use AArch32_TakeHypTrapException::*;
use u_get_HCR_EL2_Type_E2H::*;
use HCR_read::*;
use IsFeatureImplemented::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_HCR_Type_TGE::*;
use u_get_CNTHCTL_EL2_Type_EL1TVT::*;
use CNTV_CVAL_write::*;
use u_get_CNTKCTL_Type_PL0VTEN::*;
use R_read::*;
use ELUsingAArch32::*;
use Mk_CNTHV_CVAL_EL2_Type::*;
use u_get_CNTHCTL_EL2_Type_EL0VTEN::*;
use CNTKCTL_read__1::*;
use EL2Enabled::*;
use u_get_CNTKCTL_EL1_Type_EL0VTEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use Mk_CNTV_CVAL_Type::*;
use common::*;
pub fn CNTHV_CVAL_SysRegWrite64_e72c3c45ec57bc51<T: Tracer>(
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
        gs_104802: bool,
        gs_104813: bool,
        gs_104809: bool,
        u__HCR_EL2_TGE: bool,
        gs_104798: bool,
        gs_104800: bool,
        u__CNTHCTL_EL2_EL0VTEN: bool,
        gs_104814: bool,
        gs_104810: bool,
        u__CNTHCTL_EL2_EL1TVT: bool,
        gs_104799: bool,
        gs_104796: bool,
        gs_104797: bool,
        gs_104816: bool,
        gs_104807: bool,
        u__HCR_TGE: bool,
        gs_104804: bool,
        gs_104815: bool,
        gs_104811: bool,
        gs_104794: bool,
        u__CNTKCTL_PL0VTEN: bool,
        u__CNTKCTL_EL1_EL0VTEN: bool,
        gs_104795: bool,
        u__PSTATE_EL: u8,
        gs_104817: bool,
        gs_104806: bool,
        gs_104801: bool,
        gs_104808: bool,
        gs_104812: bool,
        gs_104805: bool,
        gs_104793: bool,
        gs_104803: bool,
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
        // D s_0_27: read-var __PSTATE_EL:u8
        let s_0_27: u8 = fn_state.u__PSTATE_EL;
        // D s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 2u16);
        // C s_0_29: const #448u : u32
        let s_0_29: u32 = 448;
        // D s_0_30: read-reg s_0_29:u8
        let s_0_30: u8 = {
            let value = state.read_register::<u8>(s_0_29 as isize);
            tracer.read_register(s_0_29 as isize, value);
            value
        };
        // D s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 2u16);
        // D s_0_32: cmp-eq s_0_28 s_0_31
        let s_0_32: bool = ((s_0_28) == (s_0_31));
        // N s_0_33: branch s_0_32 b16 b1
        if s_0_32 {
            return block_16(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b7 b2
        if s_1_5 {
            return block_7(state, tracer, fn_state);
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
        // D s_5_0: read-var t2:i
        let s_5_0: i128 = fn_state.t2;
        // D s_5_1: call R_read(s_5_0)
        let s_5_1: u32 = R_read(state, tracer, s_5_0);
        // D s_5_2: read-var t:i
        let s_5_2: i128 = fn_state.t;
        // D s_5_3: call R_read(s_5_2)
        let s_5_3: u32 = R_read(state, tracer, s_5_2);
        // D s_5_4: cast zx s_5_1 -> bv
        let s_5_4: Bits = Bits::new(s_5_1 as u128, 32u16);
        // D s_5_5: cast zx s_5_3 -> bv
        let s_5_5: Bits = Bits::new(s_5_3 as u128, 32u16);
        // D s_5_6: cast reint s_5_4 -> u128
        let s_5_6: u128 = (s_5_4.value() as u128);
        // D s_5_7: size-of s_5_4
        let s_5_7: u16 = s_5_4.length();
        // D s_5_8: cast reint s_5_5 -> u128
        let s_5_8: u128 = (s_5_5.value() as u128);
        // D s_5_9: size-of s_5_5
        let s_5_9: u16 = s_5_5.length();
        // D s_5_10: lsl s_5_6 s_5_9
        let s_5_10: u128 = s_5_6 << s_5_9;
        // D s_5_11: or s_5_10 s_5_8
        let s_5_11: u128 = ((s_5_10) | (s_5_8));
        // D s_5_12: add s_5_7 s_5_9
        let s_5_12: u16 = (s_5_7 + s_5_9);
        // D s_5_13: create-bits s_5_11 s_5_12
        let s_5_13: Bits = Bits::new(s_5_11, s_5_12);
        // D s_5_14: cast reint s_5_13 -> u64
        let s_5_14: u64 = (s_5_13.value() as u64);
        // D s_5_15: call Mk_CNTV_CVAL_Type(s_5_14)
        let s_5_15: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_5_14,
        );
        // D s_5_16: call CNTV_CVAL_write(s_5_15)
        let s_5_16: () = CNTV_CVAL_write(state, tracer, s_5_15);
        // N s_5_17: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var t2:i
        let s_6_0: i128 = fn_state.t2;
        // D s_6_1: call R_read(s_6_0)
        let s_6_1: u32 = R_read(state, tracer, s_6_0);
        // D s_6_2: read-var t:i
        let s_6_2: i128 = fn_state.t;
        // D s_6_3: call R_read(s_6_2)
        let s_6_3: u32 = R_read(state, tracer, s_6_2);
        // D s_6_4: cast zx s_6_1 -> bv
        let s_6_4: Bits = Bits::new(s_6_1 as u128, 32u16);
        // D s_6_5: cast zx s_6_3 -> bv
        let s_6_5: Bits = Bits::new(s_6_3 as u128, 32u16);
        // D s_6_6: cast reint s_6_4 -> u128
        let s_6_6: u128 = (s_6_4.value() as u128);
        // D s_6_7: size-of s_6_4
        let s_6_7: u16 = s_6_4.length();
        // D s_6_8: cast reint s_6_5 -> u128
        let s_6_8: u128 = (s_6_5.value() as u128);
        // D s_6_9: size-of s_6_5
        let s_6_9: u16 = s_6_5.length();
        // D s_6_10: lsl s_6_6 s_6_9
        let s_6_10: u128 = s_6_6 << s_6_9;
        // D s_6_11: or s_6_10 s_6_8
        let s_6_11: u128 = ((s_6_10) | (s_6_8));
        // D s_6_12: add s_6_7 s_6_9
        let s_6_12: u16 = (s_6_7 + s_6_9);
        // D s_6_13: create-bits s_6_11 s_6_12
        let s_6_13: Bits = Bits::new(s_6_11, s_6_12);
        // D s_6_14: cast reint s_6_13 -> u64
        let s_6_14: u64 = (s_6_13.value() as u64);
        // D s_6_15: call Mk_CNTV_CVAL_Type(s_6_14)
        let s_6_15: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_6_14,
        );
        // D s_6_16: call CNTV_CVAL_write(s_6_15)
        let s_6_16: () = CNTV_CVAL_write(state, tracer, s_6_15);
        // N s_6_17: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call EL2Enabled(s_7_0)
        let s_7_1: bool = EL2Enabled(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b15 b8
        if s_7_1 {
            return block_15(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#104793 <= s_8_0
        fn_state.gs_104793 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#104793:u8
        let s_9_0: bool = fn_state.gs_104793;
        // N s_9_1: branch s_9_0 b14 b10
        if s_9_0 {
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
        // D s_10_1: write-var gs#104794 <= s_10_0
        fn_state.gs_104794 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#104794:u8
        let s_11_0: bool = fn_state.gs_104794;
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
        // D s_12_0: read-var t2:i
        let s_12_0: i128 = fn_state.t2;
        // D s_12_1: call R_read(s_12_0)
        let s_12_1: u32 = R_read(state, tracer, s_12_0);
        // D s_12_2: read-var t:i
        let s_12_2: i128 = fn_state.t;
        // D s_12_3: call R_read(s_12_2)
        let s_12_3: u32 = R_read(state, tracer, s_12_2);
        // D s_12_4: cast zx s_12_1 -> bv
        let s_12_4: Bits = Bits::new(s_12_1 as u128, 32u16);
        // D s_12_5: cast zx s_12_3 -> bv
        let s_12_5: Bits = Bits::new(s_12_3 as u128, 32u16);
        // D s_12_6: cast reint s_12_4 -> u128
        let s_12_6: u128 = (s_12_4.value() as u128);
        // D s_12_7: size-of s_12_4
        let s_12_7: u16 = s_12_4.length();
        // D s_12_8: cast reint s_12_5 -> u128
        let s_12_8: u128 = (s_12_5.value() as u128);
        // D s_12_9: size-of s_12_5
        let s_12_9: u16 = s_12_5.length();
        // D s_12_10: lsl s_12_6 s_12_9
        let s_12_10: u128 = s_12_6 << s_12_9;
        // D s_12_11: or s_12_10 s_12_8
        let s_12_11: u128 = ((s_12_10) | (s_12_8));
        // D s_12_12: add s_12_7 s_12_9
        let s_12_12: u16 = (s_12_7 + s_12_9);
        // D s_12_13: create-bits s_12_11 s_12_12
        let s_12_13: Bits = Bits::new(s_12_11, s_12_12);
        // D s_12_14: cast reint s_12_13 -> u64
        let s_12_14: u64 = (s_12_13.value() as u64);
        // D s_12_15: call Mk_CNTV_CVAL_Type(s_12_14)
        let s_12_15: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_12_14,
        );
        // D s_12_16: call CNTV_CVAL_write(s_12_15)
        let s_12_16: () = CNTV_CVAL_write(state, tracer, s_12_15);
        // N s_12_17: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #4u : u8
        let s_13_0: u8 = 4;
        // C s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 8u16);
        // C s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (s_13_1.value() as i128);
        // C s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #432u : u32
        let s_13_5: u32 = 432;
        // D s_13_6: read-reg s_13_5:u8
        let s_13_6: u8 = {
            let value = state.read_register::<u8>(s_13_5 as isize);
            tracer.read_register(s_13_5 as isize, value);
            value
        };
        // D s_13_7: call AArch64_AArch32SystemAccessTrap(s_13_6, s_13_4)
        let s_13_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_13_6, s_13_4);
        // N s_13_8: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_14_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: write-var gs#104794 <= s_14_4
        fn_state.gs_104794 = s_14_4;
        // N s_14_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #432u : u32
        let s_15_0: u32 = 432;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call ELUsingAArch32(s_15_1)
        let s_15_2: bool = ELUsingAArch32(state, tracer, s_15_1);
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // D s_15_4: write-var gs#104793 <= s_15_3
        fn_state.gs_104793 = s_15_3;
        // N s_15_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #440u : u32
        let s_16_0: u32 = 440;
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
        // N s_16_4: branch s_16_3 b100 b17
        if s_16_3 {
            return block_100(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#104796 <= s_17_0
        fn_state.gs_104796 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#104796:u8
        let s_18_0: bool = fn_state.gs_104796;
        // N s_18_1: branch s_18_0 b99 b19
        if s_18_0 {
            return block_99(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#104797 <= s_19_0
        fn_state.gs_104797 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#104797:u8
        let s_20_0: bool = fn_state.gs_104797;
        // N s_20_1: branch s_20_0 b90 b21
        if s_20_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #440u : u32
        let s_21_0: u32 = 440;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: u8 = {
            let value = state.read_register::<u8>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call ELUsingAArch32(s_21_1)
        let s_21_2: bool = ELUsingAArch32(state, tracer, s_21_1);
        // N s_21_3: branch s_21_2 b89 b22
        if s_21_2 {
            return block_89(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#104798 <= s_22_0
        fn_state.gs_104798 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#104798:u8
        let s_23_0: bool = fn_state.gs_104798;
        // N s_23_1: branch s_23_0 b72 b24
        if s_23_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call EL2Enabled(s_24_0)
        let s_24_1: bool = EL2Enabled(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b71 b25
        if s_24_1 {
            return block_71(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#104799 <= s_25_0
        fn_state.gs_104799 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#104799:u8
        let s_26_0: bool = fn_state.gs_104799;
        // N s_26_1: branch s_26_0 b70 b27
        if s_26_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_27_1: write-var gs#104800 <= s_27_0
        fn_state.gs_104800 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#104800:u8
        let s_28_0: bool = fn_state.gs_104800;
        // N s_28_1: branch s_28_0 b69 b29
        if s_28_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#104801 <= s_29_0
        fn_state.gs_104801 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#104801:u8
        let s_30_0: bool = fn_state.gs_104801;
        // N s_30_1: branch s_30_0 b68 b31
        if s_30_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call EL2Enabled(s_31_0)
        let s_31_1: bool = EL2Enabled(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b67 b32
        if s_31_1 {
            return block_67(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#104802 <= s_32_0
        fn_state.gs_104802 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#104802:u8
        let s_33_0: bool = fn_state.gs_104802;
        // N s_33_1: branch s_33_0 b66 b34
        if s_33_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#104803 <= s_34_0
        fn_state.gs_104803 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#104803:u8
        let s_35_0: bool = fn_state.gs_104803;
        // N s_35_1: branch s_35_0 b65 b36
        if s_35_0 {
            return block_65(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#104804 <= s_36_0
        fn_state.gs_104804 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#104804:u8
        let s_37_0: bool = fn_state.gs_104804;
        // N s_37_1: branch s_37_0 b64 b38
        if s_37_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call EL2Enabled(s_38_0)
        let s_38_1: bool = EL2Enabled(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b63 b39
        if s_38_1 {
            return block_63(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#104805 <= s_39_0
        fn_state.gs_104805 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#104805:u8
        let s_40_0: bool = fn_state.gs_104805;
        // N s_40_1: branch s_40_0 b62 b41
        if s_40_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#104806 <= s_41_0
        fn_state.gs_104806 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#104806:u8
        let s_42_0: bool = fn_state.gs_104806;
        // N s_42_1: branch s_42_0 b61 b43
        if s_42_0 {
            return block_61(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#104807 <= s_43_0
        fn_state.gs_104807 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#104807:u8
        let s_44_0: bool = fn_state.gs_104807;
        // N s_44_1: branch s_44_0 b60 b45
        if s_44_0 {
            return block_60(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#104808 <= s_45_0
        fn_state.gs_104808 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#104808:u8
        let s_46_0: bool = fn_state.gs_104808;
        // N s_46_1: branch s_46_0 b59 b47
        if s_46_0 {
            return block_59(state, tracer, fn_state);
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
        // N s_47_2: branch s_47_1 b58 b48
        if s_47_1 {
            return block_58(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#104809 <= s_48_0
        fn_state.gs_104809 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#104809:u8
        let s_49_0: bool = fn_state.gs_104809;
        // N s_49_1: branch s_49_0 b57 b50
        if s_49_0 {
            return block_57(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#104810 <= s_50_0
        fn_state.gs_104810 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#104810:u8
        let s_51_0: bool = fn_state.gs_104810;
        // N s_51_1: branch s_51_0 b56 b52
        if s_51_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#104811 <= s_52_0
        fn_state.gs_104811 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#104811:u8
        let s_53_0: bool = fn_state.gs_104811;
        // N s_53_1: branch s_53_0 b55 b54
        if s_53_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var t2:i
        let s_54_0: i128 = fn_state.t2;
        // D s_54_1: call R_read(s_54_0)
        let s_54_1: u32 = R_read(state, tracer, s_54_0);
        // D s_54_2: read-var t:i
        let s_54_2: i128 = fn_state.t;
        // D s_54_3: call R_read(s_54_2)
        let s_54_3: u32 = R_read(state, tracer, s_54_2);
        // D s_54_4: cast zx s_54_1 -> bv
        let s_54_4: Bits = Bits::new(s_54_1 as u128, 32u16);
        // D s_54_5: cast zx s_54_3 -> bv
        let s_54_5: Bits = Bits::new(s_54_3 as u128, 32u16);
        // D s_54_6: cast reint s_54_4 -> u128
        let s_54_6: u128 = (s_54_4.value() as u128);
        // D s_54_7: size-of s_54_4
        let s_54_7: u16 = s_54_4.length();
        // D s_54_8: cast reint s_54_5 -> u128
        let s_54_8: u128 = (s_54_5.value() as u128);
        // D s_54_9: size-of s_54_5
        let s_54_9: u16 = s_54_5.length();
        // D s_54_10: lsl s_54_6 s_54_9
        let s_54_10: u128 = s_54_6 << s_54_9;
        // D s_54_11: or s_54_10 s_54_8
        let s_54_11: u128 = ((s_54_10) | (s_54_8));
        // D s_54_12: add s_54_7 s_54_9
        let s_54_12: u16 = (s_54_7 + s_54_9);
        // D s_54_13: create-bits s_54_11 s_54_12
        let s_54_13: Bits = Bits::new(s_54_11, s_54_12);
        // D s_54_14: cast reint s_54_13 -> u64
        let s_54_14: u64 = (s_54_13.value() as u64);
        // D s_54_15: call Mk_CNTV_CVAL_Type(s_54_14)
        let s_54_15: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_54_14,
        );
        // D s_54_16: call CNTV_CVAL_write(s_54_15)
        let s_54_16: () = CNTV_CVAL_write(state, tracer, s_54_15);
        // N s_54_17: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var t2:i
        let s_55_0: i128 = fn_state.t2;
        // D s_55_1: call R_read(s_55_0)
        let s_55_1: u32 = R_read(state, tracer, s_55_0);
        // D s_55_2: read-var t:i
        let s_55_2: i128 = fn_state.t;
        // D s_55_3: call R_read(s_55_2)
        let s_55_3: u32 = R_read(state, tracer, s_55_2);
        // D s_55_4: cast zx s_55_1 -> bv
        let s_55_4: Bits = Bits::new(s_55_1 as u128, 32u16);
        // D s_55_5: cast zx s_55_3 -> bv
        let s_55_5: Bits = Bits::new(s_55_3 as u128, 32u16);
        // D s_55_6: cast reint s_55_4 -> u128
        let s_55_6: u128 = (s_55_4.value() as u128);
        // D s_55_7: size-of s_55_4
        let s_55_7: u16 = s_55_4.length();
        // D s_55_8: cast reint s_55_5 -> u128
        let s_55_8: u128 = (s_55_5.value() as u128);
        // D s_55_9: size-of s_55_5
        let s_55_9: u16 = s_55_5.length();
        // D s_55_10: lsl s_55_6 s_55_9
        let s_55_10: u128 = s_55_6 << s_55_9;
        // D s_55_11: or s_55_10 s_55_8
        let s_55_11: u128 = ((s_55_10) | (s_55_8));
        // D s_55_12: add s_55_7 s_55_9
        let s_55_12: u16 = (s_55_7 + s_55_9);
        // D s_55_13: create-bits s_55_11 s_55_12
        let s_55_13: Bits = Bits::new(s_55_11, s_55_12);
        // D s_55_14: cast reint s_55_13 -> u64
        let s_55_14: u64 = (s_55_13.value() as u64);
        // D s_55_15: call Mk_CNTHV_CVAL_EL2_Type(s_55_14)
        let s_55_15: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_CVAL_EL2_Type(
            state,
            tracer,
            s_55_14,
        );
        // C s_55_16: const #103152u : u32
        let s_55_16: u32 = 103152;
        // N s_55_17: write-reg s_55_16 <= s_55_15
        let s_55_17: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_55_16 as isize, s_55_15);
            tracer.write_register(s_55_16 as isize, s_55_15);
        };
        // N s_55_18: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #90704u : u32
        let s_56_0: u32 = 90704;
        // D s_56_1: read-reg s_56_0:struct
        let s_56_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: call _get_SCR_EL3_Type_NS(s_56_1)
        let s_56_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_56_1);
        // D s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // C s_56_4: const #1u : u8
        let s_56_4: bool = true;
        // C s_56_5: cast zx s_56_4 -> bv
        let s_56_5: Bits = Bits::new(s_56_4 as u128, 1u16);
        // D s_56_6: cmp-eq s_56_3 s_56_5
        let s_56_6: bool = ((s_56_3) == (s_56_5));
        // D s_56_7: write-var gs#104811 <= s_56_6
        fn_state.gs_104811 = s_56_6;
        // N s_56_8: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #102552u : u32
        let s_57_0: u32 = 102552;
        // D s_57_1: read-reg s_57_0:struct
        let s_57_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: call _get_HCR_EL2_Type_E2H(s_57_1)
        let s_57_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_57_1);
        // C s_57_3: const #102552u : u32
        let s_57_3: u32 = 102552;
        // D s_57_4: read-reg s_57_3:struct
        let s_57_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_57_3 as isize);
            tracer.read_register(s_57_3 as isize, value);
            value
        };
        // D s_57_5: call _get_HCR_EL2_Type_TGE(s_57_4)
        let s_57_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_57_4);
        // D s_57_6: cast zx s_57_2 -> bv
        let s_57_6: Bits = Bits::new(s_57_2 as u128, 1u16);
        // D s_57_7: cast zx s_57_5 -> bv
        let s_57_7: Bits = Bits::new(s_57_5 as u128, 1u16);
        // D s_57_8: cast reint s_57_6 -> u128
        let s_57_8: u128 = (s_57_6.value() as u128);
        // D s_57_9: size-of s_57_6
        let s_57_9: u16 = s_57_6.length();
        // D s_57_10: cast reint s_57_7 -> u128
        let s_57_10: u128 = (s_57_7.value() as u128);
        // D s_57_11: size-of s_57_7
        let s_57_11: u16 = s_57_7.length();
        // D s_57_12: lsl s_57_8 s_57_11
        let s_57_12: u128 = s_57_8 << s_57_11;
        // D s_57_13: or s_57_12 s_57_10
        let s_57_13: u128 = ((s_57_12) | (s_57_10));
        // D s_57_14: add s_57_9 s_57_11
        let s_57_14: u16 = (s_57_9 + s_57_11);
        // D s_57_15: create-bits s_57_13 s_57_14
        let s_57_15: Bits = Bits::new(s_57_13, s_57_14);
        // D s_57_16: cast reint s_57_15 -> u8
        let s_57_16: u8 = (s_57_15.value() as u8);
        // D s_57_17: cast zx s_57_16 -> bv
        let s_57_17: Bits = Bits::new(s_57_16 as u128, 2u16);
        // C s_57_18: const #3u : u8
        let s_57_18: u8 = 3;
        // C s_57_19: cast zx s_57_18 -> bv
        let s_57_19: Bits = Bits::new(s_57_18 as u128, 2u16);
        // D s_57_20: cmp-eq s_57_17 s_57_19
        let s_57_20: bool = ((s_57_17) == (s_57_19));
        // D s_57_21: write-var gs#104810 <= s_57_20
        fn_state.gs_104810 = s_57_20;
        // N s_57_22: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #432u : u32
        let s_58_0: u32 = 432;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: u8 = {
            let value = state.read_register::<u8>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call ELUsingAArch32(s_58_1)
        let s_58_2: bool = ELUsingAArch32(state, tracer, s_58_1);
        // D s_58_3: not s_58_2
        let s_58_3: bool = !s_58_2;
        // D s_58_4: write-var gs#104809 <= s_58_3
        fn_state.gs_104809 = s_58_3;
        // N s_58_5: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var t2:i
        let s_59_0: i128 = fn_state.t2;
        // D s_59_1: call R_read(s_59_0)
        let s_59_1: u32 = R_read(state, tracer, s_59_0);
        // D s_59_2: read-var t:i
        let s_59_2: i128 = fn_state.t;
        // D s_59_3: call R_read(s_59_2)
        let s_59_3: u32 = R_read(state, tracer, s_59_2);
        // D s_59_4: cast zx s_59_1 -> bv
        let s_59_4: Bits = Bits::new(s_59_1 as u128, 32u16);
        // D s_59_5: cast zx s_59_3 -> bv
        let s_59_5: Bits = Bits::new(s_59_3 as u128, 32u16);
        // D s_59_6: cast reint s_59_4 -> u128
        let s_59_6: u128 = (s_59_4.value() as u128);
        // D s_59_7: size-of s_59_4
        let s_59_7: u16 = s_59_4.length();
        // D s_59_8: cast reint s_59_5 -> u128
        let s_59_8: u128 = (s_59_5.value() as u128);
        // D s_59_9: size-of s_59_5
        let s_59_9: u16 = s_59_5.length();
        // D s_59_10: lsl s_59_6 s_59_9
        let s_59_10: u128 = s_59_6 << s_59_9;
        // D s_59_11: or s_59_10 s_59_8
        let s_59_11: u128 = ((s_59_10) | (s_59_8));
        // D s_59_12: add s_59_7 s_59_9
        let s_59_12: u16 = (s_59_7 + s_59_9);
        // D s_59_13: create-bits s_59_11 s_59_12
        let s_59_13: Bits = Bits::new(s_59_11, s_59_12);
        // D s_59_14: cast reint s_59_13 -> u64
        let s_59_14: u64 = (s_59_13.value() as u64);
        // D s_59_15: call Mk_CNTHVS_CVAL_EL2_Type(s_59_14)
        let s_59_15: ProductType5c790c8ef59cc8b2 = Mk_CNTHVS_CVAL_EL2_Type(
            state,
            tracer,
            s_59_14,
        );
        // C s_59_16: const #10064u : u32
        let s_59_16: u32 = 10064;
        // N s_59_17: write-reg s_59_16 <= s_59_15
        let s_59_17: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_59_16 as isize, s_59_15);
            tracer.write_register(s_59_16 as isize, s_59_15);
        };
        // N s_59_18: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #117u : u32
        let s_60_0: u32 = 117;
        // S s_60_1: call IsFeatureImplemented(s_60_0)
        let s_60_1: bool = IsFeatureImplemented(state, tracer, s_60_0);
        // D s_60_2: write-var gs#104808 <= s_60_1
        fn_state.gs_104808 = s_60_1;
        // N s_60_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #90704u : u32
        let s_61_0: u32 = 90704;
        // D s_61_1: read-reg s_61_0:struct
        let s_61_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call _get_SCR_EL3_Type_NS(s_61_1)
        let s_61_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_61_1);
        // D s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 1u16);
        // C s_61_4: const #0u : u8
        let s_61_4: bool = false;
        // C s_61_5: cast zx s_61_4 -> bv
        let s_61_5: Bits = Bits::new(s_61_4 as u128, 1u16);
        // D s_61_6: cmp-eq s_61_3 s_61_5
        let s_61_6: bool = ((s_61_3) == (s_61_5));
        // D s_61_7: write-var gs#104807 <= s_61_6
        fn_state.gs_104807 = s_61_6;
        // N s_61_8: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #102552u : u32
        let s_62_0: u32 = 102552;
        // D s_62_1: read-reg s_62_0:struct
        let s_62_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call _get_HCR_EL2_Type_E2H(s_62_1)
        let s_62_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_62_1);
        // C s_62_3: const #102552u : u32
        let s_62_3: u32 = 102552;
        // D s_62_4: read-reg s_62_3:struct
        let s_62_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_3 as isize);
            tracer.read_register(s_62_3 as isize, value);
            value
        };
        // D s_62_5: call _get_HCR_EL2_Type_TGE(s_62_4)
        let s_62_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_62_4);
        // D s_62_6: cast zx s_62_2 -> bv
        let s_62_6: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_7: cast zx s_62_5 -> bv
        let s_62_7: Bits = Bits::new(s_62_5 as u128, 1u16);
        // D s_62_8: cast reint s_62_6 -> u128
        let s_62_8: u128 = (s_62_6.value() as u128);
        // D s_62_9: size-of s_62_6
        let s_62_9: u16 = s_62_6.length();
        // D s_62_10: cast reint s_62_7 -> u128
        let s_62_10: u128 = (s_62_7.value() as u128);
        // D s_62_11: size-of s_62_7
        let s_62_11: u16 = s_62_7.length();
        // D s_62_12: lsl s_62_8 s_62_11
        let s_62_12: u128 = s_62_8 << s_62_11;
        // D s_62_13: or s_62_12 s_62_10
        let s_62_13: u128 = ((s_62_12) | (s_62_10));
        // D s_62_14: add s_62_9 s_62_11
        let s_62_14: u16 = (s_62_9 + s_62_11);
        // D s_62_15: create-bits s_62_13 s_62_14
        let s_62_15: Bits = Bits::new(s_62_13, s_62_14);
        // D s_62_16: cast reint s_62_15 -> u8
        let s_62_16: u8 = (s_62_15.value() as u8);
        // D s_62_17: cast zx s_62_16 -> bv
        let s_62_17: Bits = Bits::new(s_62_16 as u128, 2u16);
        // C s_62_18: const #3u : u8
        let s_62_18: u8 = 3;
        // C s_62_19: cast zx s_62_18 -> bv
        let s_62_19: Bits = Bits::new(s_62_18 as u128, 2u16);
        // D s_62_20: cmp-eq s_62_17 s_62_19
        let s_62_20: bool = ((s_62_17) == (s_62_19));
        // D s_62_21: write-var gs#104806 <= s_62_20
        fn_state.gs_104806 = s_62_20;
        // N s_62_22: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #432u : u32
        let s_63_0: u32 = 432;
        // D s_63_1: read-reg s_63_0:u8
        let s_63_1: u8 = {
            let value = state.read_register::<u8>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call ELUsingAArch32(s_63_1)
        let s_63_2: bool = ELUsingAArch32(state, tracer, s_63_1);
        // D s_63_3: not s_63_2
        let s_63_3: bool = !s_63_2;
        // D s_63_4: write-var gs#104805 <= s_63_3
        fn_state.gs_104805 = s_63_3;
        // N s_63_5: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #4u : u8
        let s_64_0: u8 = 4;
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
        // D s_64_7: call AArch64_AArch32SystemAccessTrap(s_64_6, s_64_4)
        let s_64_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_64_6, s_64_4);
        // N s_64_8: return
        return;
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_65_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #1u : u8
        let s_65_2: bool = true;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#104804 <= s_65_4
        fn_state.gs_104804 = s_65_4;
        // N s_65_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #102552u : u32
        let s_66_0: u32 = 102552;
        // D s_66_1: read-reg s_66_0:struct
        let s_66_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_0 as isize);
            tracer.read_register(s_66_0 as isize, value);
            value
        };
        // D s_66_2: call _get_HCR_EL2_Type_E2H(s_66_1)
        let s_66_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_66_1);
        // C s_66_3: const #102552u : u32
        let s_66_3: u32 = 102552;
        // D s_66_4: read-reg s_66_3:struct
        let s_66_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_3 as isize);
            tracer.read_register(s_66_3 as isize, value);
            value
        };
        // D s_66_5: call _get_HCR_EL2_Type_TGE(s_66_4)
        let s_66_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_66_4);
        // D s_66_6: cast zx s_66_2 -> bv
        let s_66_6: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_7: cast zx s_66_5 -> bv
        let s_66_7: Bits = Bits::new(s_66_5 as u128, 1u16);
        // D s_66_8: cast reint s_66_6 -> u128
        let s_66_8: u128 = (s_66_6.value() as u128);
        // D s_66_9: size-of s_66_6
        let s_66_9: u16 = s_66_6.length();
        // D s_66_10: cast reint s_66_7 -> u128
        let s_66_10: u128 = (s_66_7.value() as u128);
        // D s_66_11: size-of s_66_7
        let s_66_11: u16 = s_66_7.length();
        // D s_66_12: lsl s_66_8 s_66_11
        let s_66_12: u128 = s_66_8 << s_66_11;
        // D s_66_13: or s_66_12 s_66_10
        let s_66_13: u128 = ((s_66_12) | (s_66_10));
        // D s_66_14: add s_66_9 s_66_11
        let s_66_14: u16 = (s_66_9 + s_66_11);
        // D s_66_15: create-bits s_66_13 s_66_14
        let s_66_15: Bits = Bits::new(s_66_13, s_66_14);
        // D s_66_16: cast reint s_66_15 -> u8
        let s_66_16: u8 = (s_66_15.value() as u8);
        // D s_66_17: cast zx s_66_16 -> bv
        let s_66_17: Bits = Bits::new(s_66_16 as u128, 2u16);
        // C s_66_18: const #3u : u8
        let s_66_18: u8 = 3;
        // C s_66_19: cast zx s_66_18 -> bv
        let s_66_19: Bits = Bits::new(s_66_18 as u128, 2u16);
        // D s_66_20: cmp-ne s_66_17 s_66_19
        let s_66_20: bool = ((s_66_17) != (s_66_19));
        // D s_66_21: write-var gs#104803 <= s_66_20
        fn_state.gs_104803 = s_66_20;
        // N s_66_22: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #432u : u32
        let s_67_0: u32 = 432;
        // D s_67_1: read-reg s_67_0:u8
        let s_67_1: u8 = {
            let value = state.read_register::<u8>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // D s_67_2: call ELUsingAArch32(s_67_1)
        let s_67_2: bool = ELUsingAArch32(state, tracer, s_67_1);
        // D s_67_3: not s_67_2
        let s_67_3: bool = !s_67_2;
        // D s_67_4: write-var gs#104802 <= s_67_3
        fn_state.gs_104802 = s_67_3;
        // N s_67_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #4u : u8
        let s_68_0: u8 = 4;
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
        // D s_68_7: call AArch64_AArch32SystemAccessTrap(s_68_6, s_68_4)
        let s_68_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_68_6, s_68_4);
        // N s_68_8: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __CNTHCTL_EL2_EL0VTEN:u8
        let s_69_0: bool = fn_state.u__CNTHCTL_EL2_EL0VTEN;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #0u : u8
        let s_69_2: bool = false;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#104801 <= s_69_4
        fn_state.gs_104801 = s_69_4;
        // N s_69_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #102552u : u32
        let s_70_0: u32 = 102552;
        // D s_70_1: read-reg s_70_0:struct
        let s_70_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: call _get_HCR_EL2_Type_E2H(s_70_1)
        let s_70_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_70_1);
        // C s_70_3: const #102552u : u32
        let s_70_3: u32 = 102552;
        // D s_70_4: read-reg s_70_3:struct
        let s_70_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_70_3 as isize);
            tracer.read_register(s_70_3 as isize, value);
            value
        };
        // D s_70_5: call _get_HCR_EL2_Type_TGE(s_70_4)
        let s_70_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_70_4);
        // D s_70_6: cast zx s_70_2 -> bv
        let s_70_6: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_7: cast zx s_70_5 -> bv
        let s_70_7: Bits = Bits::new(s_70_5 as u128, 1u16);
        // D s_70_8: cast reint s_70_6 -> u128
        let s_70_8: u128 = (s_70_6.value() as u128);
        // D s_70_9: size-of s_70_6
        let s_70_9: u16 = s_70_6.length();
        // D s_70_10: cast reint s_70_7 -> u128
        let s_70_10: u128 = (s_70_7.value() as u128);
        // D s_70_11: size-of s_70_7
        let s_70_11: u16 = s_70_7.length();
        // D s_70_12: lsl s_70_8 s_70_11
        let s_70_12: u128 = s_70_8 << s_70_11;
        // D s_70_13: or s_70_12 s_70_10
        let s_70_13: u128 = ((s_70_12) | (s_70_10));
        // D s_70_14: add s_70_9 s_70_11
        let s_70_14: u16 = (s_70_9 + s_70_11);
        // D s_70_15: create-bits s_70_13 s_70_14
        let s_70_15: Bits = Bits::new(s_70_13, s_70_14);
        // D s_70_16: cast reint s_70_15 -> u8
        let s_70_16: u8 = (s_70_15.value() as u8);
        // D s_70_17: cast zx s_70_16 -> bv
        let s_70_17: Bits = Bits::new(s_70_16 as u128, 2u16);
        // C s_70_18: const #3u : u8
        let s_70_18: u8 = 3;
        // C s_70_19: cast zx s_70_18 -> bv
        let s_70_19: Bits = Bits::new(s_70_18 as u128, 2u16);
        // D s_70_20: cmp-eq s_70_17 s_70_19
        let s_70_20: bool = ((s_70_17) == (s_70_19));
        // D s_70_21: write-var gs#104800 <= s_70_20
        fn_state.gs_104800 = s_70_20;
        // N s_70_22: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #432u : u32
        let s_71_0: u32 = 432;
        // D s_71_1: read-reg s_71_0:u8
        let s_71_1: u8 = {
            let value = state.read_register::<u8>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // D s_71_2: call ELUsingAArch32(s_71_1)
        let s_71_2: bool = ELUsingAArch32(state, tracer, s_71_1);
        // D s_71_3: not s_71_2
        let s_71_3: bool = !s_71_2;
        // D s_71_4: write-var gs#104799 <= s_71_3
        fn_state.gs_104799 = s_71_3;
        // N s_71_5: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call EL2Enabled(s_72_0)
        let s_72_1: bool = EL2Enabled(state, tracer, s_72_0);
        // N s_72_2: branch s_72_1 b88 b73
        if s_72_1 {
            return block_88(state, tracer, fn_state);
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
        // D s_73_1: write-var gs#104812 <= s_73_0
        fn_state.gs_104812 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#104812:u8
        let s_74_0: bool = fn_state.gs_104812;
        // N s_74_1: branch s_74_0 b87 b75
        if s_74_0 {
            return block_87(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#104813 <= s_75_0
        fn_state.gs_104813 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#104813:u8
        let s_76_0: bool = fn_state.gs_104813;
        // N s_76_1: branch s_76_0 b86 b77
        if s_76_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #() : ()
        let s_77_0: () = ();
        // S s_77_1: call EL2Enabled(s_77_0)
        let s_77_1: bool = EL2Enabled(state, tracer, s_77_0);
        // N s_77_2: branch s_77_1 b85 b78
        if s_77_1 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #0u : u8
        let s_78_0: bool = false;
        // D s_78_1: write-var gs#104814 <= s_78_0
        fn_state.gs_104814 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#104814:u8
        let s_79_0: bool = fn_state.gs_104814;
        // N s_79_1: branch s_79_0 b84 b80
        if s_79_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var gs#104815 <= s_80_0
        fn_state.gs_104815 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#104815:u8
        let s_81_0: bool = fn_state.gs_104815;
        // N s_81_1: branch s_81_0 b83 b82
        if s_81_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
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
        // C s_83_0: const #0u : u8
        let s_83_0: u8 = 0;
        // C s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 8u16);
        // C s_83_2: cast zx s_83_1 -> i
        let s_83_2: i128 = (s_83_1.value() as i128);
        // C s_83_3: cast reint s_83_2 -> i64
        let s_83_3: i64 = (s_83_2 as i64);
        // C s_83_4: cast zx s_83_3 -> i
        let s_83_4: i128 = (i128::try_from(s_83_3).unwrap());
        // S s_83_5: call AArch32_TakeHypTrapException(s_83_4)
        let s_83_5: () = AArch32_TakeHypTrapException(state, tracer, s_83_4);
        // N s_83_6: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __HCR_TGE:u8
        let s_84_0: bool = fn_state.u__HCR_TGE;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#104815 <= s_84_4
        fn_state.gs_104815 = s_84_4;
        // N s_84_6: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #432u : u32
        let s_85_0: u32 = 432;
        // D s_85_1: read-reg s_85_0:u8
        let s_85_1: u8 = {
            let value = state.read_register::<u8>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call ELUsingAArch32(s_85_1)
        let s_85_2: bool = ELUsingAArch32(state, tracer, s_85_1);
        // D s_85_3: write-var gs#104814 <= s_85_2
        fn_state.gs_104814 = s_85_2;
        // N s_85_4: jump b79
        return block_79(state, tracer, fn_state);
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
        // C s_86_5: const #432u : u32
        let s_86_5: u32 = 432;
        // D s_86_6: read-reg s_86_5:u8
        let s_86_6: u8 = {
            let value = state.read_register::<u8>(s_86_5 as isize);
            tracer.read_register(s_86_5 as isize, value);
            value
        };
        // D s_86_7: call AArch64_AArch32SystemAccessTrap(s_86_6, s_86_4)
        let s_86_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_86_6, s_86_4);
        // N s_86_8: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var __HCR_EL2_TGE:u8
        let s_87_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 1u16);
        // C s_87_2: const #1u : u8
        let s_87_2: bool = true;
        // C s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 1u16);
        // D s_87_4: cmp-eq s_87_1 s_87_3
        let s_87_4: bool = ((s_87_1) == (s_87_3));
        // D s_87_5: write-var gs#104813 <= s_87_4
        fn_state.gs_104813 = s_87_4;
        // N s_87_6: jump b76
        return block_76(state, tracer, fn_state);
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
        // D s_88_3: not s_88_2
        let s_88_3: bool = !s_88_2;
        // D s_88_4: write-var gs#104812 <= s_88_3
        fn_state.gs_104812 = s_88_3;
        // N s_88_5: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var __CNTKCTL_PL0VTEN:u8
        let s_89_0: bool = fn_state.u__CNTKCTL_PL0VTEN;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #0u : u8
        let s_89_2: bool = false;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // D s_89_5: write-var gs#104798 <= s_89_4
        fn_state.gs_104798 = s_89_4;
        // N s_89_6: jump b23
        return block_23(state, tracer, fn_state);
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
        // N s_90_2: branch s_90_1 b98 b91
        if s_90_1 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #0u : u8
        let s_91_0: bool = false;
        // D s_91_1: write-var gs#104816 <= s_91_0
        fn_state.gs_104816 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#104816:u8
        let s_92_0: bool = fn_state.gs_104816;
        // N s_92_1: branch s_92_0 b97 b93
        if s_92_0 {
            return block_97(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#104817 <= s_93_0
        fn_state.gs_104817 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#104817:u8
        let s_94_0: bool = fn_state.gs_104817;
        // N s_94_1: branch s_94_0 b96 b95
        if s_94_0 {
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
        // C s_95_0: const #4u : u8
        let s_95_0: u8 = 4;
        // C s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 8u16);
        // C s_95_2: cast zx s_95_1 -> i
        let s_95_2: i128 = (s_95_1.value() as i128);
        // C s_95_3: cast reint s_95_2 -> i64
        let s_95_3: i64 = (s_95_2 as i64);
        // C s_95_4: cast zx s_95_3 -> i
        let s_95_4: i128 = (i128::try_from(s_95_3).unwrap());
        // C s_95_5: const #440u : u32
        let s_95_5: u32 = 440;
        // D s_95_6: read-reg s_95_5:u8
        let s_95_6: u8 = {
            let value = state.read_register::<u8>(s_95_5 as isize);
            tracer.read_register(s_95_5 as isize, value);
            value
        };
        // D s_95_7: call AArch64_AArch32SystemAccessTrap(s_95_6, s_95_4)
        let s_95_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_95_6, s_95_4);
        // N s_95_8: return
        return;
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
        // D s_97_0: read-var __HCR_EL2_TGE:u8
        let s_97_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 1u16);
        // C s_97_2: const #1u : u8
        let s_97_2: bool = true;
        // C s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 1u16);
        // D s_97_4: cmp-eq s_97_1 s_97_3
        let s_97_4: bool = ((s_97_1) == (s_97_3));
        // D s_97_5: write-var gs#104817 <= s_97_4
        fn_state.gs_104817 = s_97_4;
        // N s_97_6: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #432u : u32
        let s_98_0: u32 = 432;
        // D s_98_1: read-reg s_98_0:u8
        let s_98_1: u8 = {
            let value = state.read_register::<u8>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // D s_98_2: call ELUsingAArch32(s_98_1)
        let s_98_2: bool = ELUsingAArch32(state, tracer, s_98_1);
        // D s_98_3: not s_98_2
        let s_98_3: bool = !s_98_2;
        // D s_98_4: write-var gs#104816 <= s_98_3
        fn_state.gs_104816 = s_98_3;
        // N s_98_5: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var __CNTKCTL_EL1_EL0VTEN:u8
        let s_99_0: bool = fn_state.u__CNTKCTL_EL1_EL0VTEN;
        // D s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 1u16);
        // C s_99_2: const #0u : u8
        let s_99_2: bool = false;
        // C s_99_3: cast zx s_99_2 -> bv
        let s_99_3: Bits = Bits::new(s_99_2 as u128, 1u16);
        // D s_99_4: cmp-eq s_99_1 s_99_3
        let s_99_4: bool = ((s_99_1) == (s_99_3));
        // D s_99_5: write-var gs#104797 <= s_99_4
        fn_state.gs_104797 = s_99_4;
        // N s_99_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #() : ()
        let s_100_0: () = ();
        // S s_100_1: call EL2Enabled(s_100_0)
        let s_100_1: bool = EL2Enabled(state, tracer, s_100_0);
        // N s_100_2: branch s_100_1 b103 b101
        if s_100_1 {
            return block_103(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#104795 <= s_101_0
        fn_state.gs_104795 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#104795:u8
        let s_102_0: bool = fn_state.gs_104795;
        // D s_102_1: not s_102_0
        let s_102_1: bool = !s_102_0;
        // D s_102_2: write-var gs#104796 <= s_102_1
        fn_state.gs_104796 = s_102_1;
        // N s_102_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #102552u : u32
        let s_103_0: u32 = 102552;
        // D s_103_1: read-reg s_103_0:struct
        let s_103_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_103_0 as isize);
            tracer.read_register(s_103_0 as isize, value);
            value
        };
        // D s_103_2: call _get_HCR_EL2_Type_E2H(s_103_1)
        let s_103_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_103_1);
        // C s_103_3: const #102552u : u32
        let s_103_3: u32 = 102552;
        // D s_103_4: read-reg s_103_3:struct
        let s_103_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_103_3 as isize);
            tracer.read_register(s_103_3 as isize, value);
            value
        };
        // D s_103_5: call _get_HCR_EL2_Type_TGE(s_103_4)
        let s_103_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_103_4);
        // D s_103_6: cast zx s_103_2 -> bv
        let s_103_6: Bits = Bits::new(s_103_2 as u128, 1u16);
        // D s_103_7: cast zx s_103_5 -> bv
        let s_103_7: Bits = Bits::new(s_103_5 as u128, 1u16);
        // D s_103_8: cast reint s_103_6 -> u128
        let s_103_8: u128 = (s_103_6.value() as u128);
        // D s_103_9: size-of s_103_6
        let s_103_9: u16 = s_103_6.length();
        // D s_103_10: cast reint s_103_7 -> u128
        let s_103_10: u128 = (s_103_7.value() as u128);
        // D s_103_11: size-of s_103_7
        let s_103_11: u16 = s_103_7.length();
        // D s_103_12: lsl s_103_8 s_103_11
        let s_103_12: u128 = s_103_8 << s_103_11;
        // D s_103_13: or s_103_12 s_103_10
        let s_103_13: u128 = ((s_103_12) | (s_103_10));
        // D s_103_14: add s_103_9 s_103_11
        let s_103_14: u16 = (s_103_9 + s_103_11);
        // D s_103_15: create-bits s_103_13 s_103_14
        let s_103_15: Bits = Bits::new(s_103_13, s_103_14);
        // D s_103_16: cast reint s_103_15 -> u8
        let s_103_16: u8 = (s_103_15.value() as u8);
        // D s_103_17: cast zx s_103_16 -> bv
        let s_103_17: Bits = Bits::new(s_103_16 as u128, 2u16);
        // C s_103_18: const #3u : u8
        let s_103_18: u8 = 3;
        // C s_103_19: cast zx s_103_18 -> bv
        let s_103_19: Bits = Bits::new(s_103_18 as u128, 2u16);
        // D s_103_20: cmp-eq s_103_17 s_103_19
        let s_103_20: bool = ((s_103_17) == (s_103_19));
        // D s_103_21: write-var gs#104795 <= s_103_20
        fn_state.gs_104795 = s_103_20;
        // N s_103_22: jump b102
        return block_102(state, tracer, fn_state);
    }
}
