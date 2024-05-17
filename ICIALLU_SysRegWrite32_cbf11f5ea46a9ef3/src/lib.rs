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
use u_get_HSTR_Type_T7::*;
use u_get_HCR_Type_TPU::*;
use u_get_HCR_Type_FB::*;
use HCR_read::*;
use u_get_HSTR_EL2_Type_T7::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HCR_EL2_Type_TPU::*;
use u_get_HCR2_Type_TOCU::*;
use AArch32_IC::*;
use ELUsingAArch32::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TOCU::*;
use HCR2_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn ICIALLU_SysRegWrite32_cbf11f5ea46a9ef3<T: Tracer>(
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
        gs_129150: bool,
        u__HSTR_T7: bool,
        gs_129155: bool,
        gs_129147: bool,
        gs_129145: bool,
        gs_129146: bool,
        gs_129154: bool,
        u__HCR_EL2_TOCU: bool,
        gs_129149: bool,
        u__PSTATE_EL: u8,
        gs_129153: bool,
        u__HCR2_TOCU: bool,
        u__HCR_EL2_TPU: bool,
        gs_129144: bool,
        gs_129151: bool,
        gs_129142: bool,
        gs_129148: bool,
        u__HCR_TPU: bool,
        gs_129152: bool,
        u__HCR_FB: bool,
        gs_129143: bool,
        u__HSTR_EL2_T7: bool,
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
        // C s_0_3: const #104936u : u32
        let s_0_3: u32 = 104936;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HSTR_EL2_Type_T7(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T7(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T7 <= s_0_5
        fn_state.u__HSTR_EL2_T7 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T7(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T7(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T7 <= s_0_9
        fn_state.u__HSTR_T7 = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TPU(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TPU(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TPU <= s_0_13
        fn_state.u__HCR_EL2_TPU = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_TOCU(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_TOCU(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_TOCU <= s_0_17
        fn_state.u__HCR_EL2_TOCU = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call HCR_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_19);
        // S s_0_21: call _get_HCR_Type_TPU(s_0_20)
        let s_0_21: bool = u_get_HCR_Type_TPU(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_TPU <= s_0_21
        fn_state.u__HCR_TPU = s_0_21;
        // C s_0_23: const #() : ()
        let s_0_23: () = ();
        // S s_0_24: call HCR2_read(s_0_23)
        let s_0_24: ProductType700c18a878c5601b = HCR2_read(state, tracer, s_0_23);
        // S s_0_25: call _get_HCR2_Type_TOCU(s_0_24)
        let s_0_25: bool = u_get_HCR2_Type_TOCU(state, tracer, s_0_24);
        // D s_0_26: write-var __HCR2_TOCU <= s_0_25
        fn_state.u__HCR2_TOCU = s_0_25;
        // C s_0_27: const #() : ()
        let s_0_27: () = ();
        // S s_0_28: call HCR_read(s_0_27)
        let s_0_28: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_27);
        // S s_0_29: call _get_HCR_Type_FB(s_0_28)
        let s_0_29: bool = u_get_HCR_Type_FB(state, tracer, s_0_28);
        // D s_0_30: write-var __HCR_FB <= s_0_29
        fn_state.u__HCR_FB = s_0_29;
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
        // N s_0_37: branch s_0_36 b64 b1
        if s_0_36 {
            return block_64(state, tracer, fn_state);
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
        // C s_5_0: const #7u : u32
        let s_5_0: u32 = 7;
        // S s_5_1: call AArch32_IC(s_5_0)
        let s_5_1: () = AArch32_IC(state, tracer, s_5_0);
        // N s_5_2: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #7u : u32
        let s_6_0: u32 = 7;
        // S s_6_1: call AArch32_IC(s_6_0)
        let s_6_1: () = AArch32_IC(state, tracer, s_6_0);
        // N s_6_2: return
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
        // N s_7_2: branch s_7_1 b63 b8
        if s_7_1 {
            return block_63(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#129142 <= s_8_0
        fn_state.gs_129142 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#129142:u8
        let s_9_0: bool = fn_state.gs_129142;
        // N s_9_1: branch s_9_0 b62 b10
        if s_9_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#129143 <= s_10_0
        fn_state.gs_129143 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#129143:u8
        let s_11_0: bool = fn_state.gs_129143;
        // N s_11_1: branch s_11_0 b61 b12
        if s_11_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call EL2Enabled(s_12_0)
        let s_12_1: bool = EL2Enabled(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b60 b13
        if s_12_1 {
            return block_60(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#129144 <= s_13_0
        fn_state.gs_129144 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#129144:u8
        let s_14_0: bool = fn_state.gs_129144;
        // N s_14_1: branch s_14_0 b59 b15
        if s_14_0 {
            return block_59(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#129145 <= s_15_0
        fn_state.gs_129145 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#129145:u8
        let s_16_0: bool = fn_state.gs_129145;
        // N s_16_1: branch s_16_0 b58 b17
        if s_16_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call EL2Enabled(s_17_0)
        let s_17_1: bool = EL2Enabled(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b57 b18
        if s_17_1 {
            return block_57(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#129146 <= s_18_0
        fn_state.gs_129146 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#129146:u8
        let s_19_0: bool = fn_state.gs_129146;
        // N s_19_1: branch s_19_0 b56 b20
        if s_19_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#129147 <= s_20_0
        fn_state.gs_129147 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#129147:u8
        let s_21_0: bool = fn_state.gs_129147;
        // N s_21_1: branch s_21_0 b55 b22
        if s_21_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call EL2Enabled(s_22_0)
        let s_22_1: bool = EL2Enabled(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b54 b23
        if s_22_1 {
            return block_54(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#129148 <= s_23_0
        fn_state.gs_129148 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#129148:u8
        let s_24_0: bool = fn_state.gs_129148;
        // N s_24_1: branch s_24_0 b53 b25
        if s_24_0 {
            return block_53(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#129149 <= s_25_0
        fn_state.gs_129149 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#129149:u8
        let s_26_0: bool = fn_state.gs_129149;
        // N s_26_1: branch s_26_0 b52 b27
        if s_26_0 {
            return block_52(state, tracer, fn_state);
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
        // S s_27_1: call EL2Enabled(s_27_0)
        let s_27_1: bool = EL2Enabled(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b51 b28
        if s_27_1 {
            return block_51(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#129150 <= s_28_0
        fn_state.gs_129150 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#129150:u8
        let s_29_0: bool = fn_state.gs_129150;
        // N s_29_1: branch s_29_0 b50 b30
        if s_29_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#129151 <= s_30_0
        fn_state.gs_129151 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#129151:u8
        let s_31_0: bool = fn_state.gs_129151;
        // N s_31_1: branch s_31_0 b49 b32
        if s_31_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call EL2Enabled(s_32_0)
        let s_32_1: bool = EL2Enabled(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b48 b33
        if s_32_1 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#129152 <= s_33_0
        fn_state.gs_129152 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#129152:u8
        let s_34_0: bool = fn_state.gs_129152;
        // N s_34_1: branch s_34_0 b47 b35
        if s_34_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#129153 <= s_35_0
        fn_state.gs_129153 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#129153:u8
        let s_36_0: bool = fn_state.gs_129153;
        // N s_36_1: branch s_36_0 b46 b37
        if s_36_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call EL2Enabled(s_37_0)
        let s_37_1: bool = EL2Enabled(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b45 b38
        if s_37_1 {
            return block_45(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#129154 <= s_38_0
        fn_state.gs_129154 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#129154:u8
        let s_39_0: bool = fn_state.gs_129154;
        // N s_39_1: branch s_39_0 b44 b40
        if s_39_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#129155 <= s_40_0
        fn_state.gs_129155 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#129155:u8
        let s_41_0: bool = fn_state.gs_129155;
        // N s_41_1: branch s_41_0 b43 b42
        if s_41_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #7u : u32
        let s_42_0: u32 = 7;
        // S s_42_1: call AArch32_IC(s_42_0)
        let s_42_1: () = AArch32_IC(state, tracer, s_42_0);
        // N s_42_2: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #8u : u32
        let s_43_0: u32 = 8;
        // S s_43_1: call AArch32_IC(s_43_0)
        let s_43_1: () = AArch32_IC(state, tracer, s_43_0);
        // N s_43_2: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __HCR_FB:u8
        let s_44_0: bool = fn_state.u__HCR_FB;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #1u : u8
        let s_44_2: bool = true;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#129155 <= s_44_4
        fn_state.gs_129155 = s_44_4;
        // N s_44_6: jump b41
        return block_41(state, tracer, fn_state);
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
        // D s_45_2: call ELUsingAArch32(s_45_1)
        let s_45_2: bool = ELUsingAArch32(state, tracer, s_45_1);
        // D s_45_3: write-var gs#129154 <= s_45_2
        fn_state.gs_129154 = s_45_2;
        // N s_45_4: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #3u : u8
        let s_46_0: u8 = 3;
        // C s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 8u16);
        // C s_46_2: cast zx s_46_1 -> i
        let s_46_2: i128 = (s_46_1.value() as i128);
        // C s_46_3: cast reint s_46_2 -> i64
        let s_46_3: i64 = (s_46_2 as i64);
        // C s_46_4: cast zx s_46_3 -> i
        let s_46_4: i128 = (i128::try_from(s_46_3).unwrap());
        // S s_46_5: call AArch32_TakeHypTrapException(s_46_4)
        let s_46_5: () = AArch32_TakeHypTrapException(state, tracer, s_46_4);
        // N s_46_6: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var __HCR2_TOCU:u8
        let s_47_0: bool = fn_state.u__HCR2_TOCU;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 1u16);
        // C s_47_2: const #1u : u8
        let s_47_2: bool = true;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: write-var gs#129153 <= s_47_4
        fn_state.gs_129153 = s_47_4;
        // N s_47_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #432u : u32
        let s_48_0: u32 = 432;
        // D s_48_1: read-reg s_48_0:u8
        let s_48_1: u8 = {
            let value = state.read_register::<u8>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: call ELUsingAArch32(s_48_1)
        let s_48_2: bool = ELUsingAArch32(state, tracer, s_48_1);
        // D s_48_3: write-var gs#129152 <= s_48_2
        fn_state.gs_129152 = s_48_2;
        // N s_48_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #3u : u8
        let s_49_0: u8 = 3;
        // C s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 8u16);
        // C s_49_2: cast zx s_49_1 -> i
        let s_49_2: i128 = (s_49_1.value() as i128);
        // C s_49_3: cast reint s_49_2 -> i64
        let s_49_3: i64 = (s_49_2 as i64);
        // C s_49_4: cast zx s_49_3 -> i
        let s_49_4: i128 = (i128::try_from(s_49_3).unwrap());
        // S s_49_5: call AArch32_TakeHypTrapException(s_49_4)
        let s_49_5: () = AArch32_TakeHypTrapException(state, tracer, s_49_4);
        // N s_49_6: return
        return;
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var __HCR_TPU:u8
        let s_50_0: bool = fn_state.u__HCR_TPU;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 1u16);
        // C s_50_2: const #1u : u8
        let s_50_2: bool = true;
        // C s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // D s_50_4: cmp-eq s_50_1 s_50_3
        let s_50_4: bool = ((s_50_1) == (s_50_3));
        // D s_50_5: write-var gs#129151 <= s_50_4
        fn_state.gs_129151 = s_50_4;
        // N s_50_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #432u : u32
        let s_51_0: u32 = 432;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: u8 = {
            let value = state.read_register::<u8>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call ELUsingAArch32(s_51_1)
        let s_51_2: bool = ELUsingAArch32(state, tracer, s_51_1);
        // D s_51_3: write-var gs#129150 <= s_51_2
        fn_state.gs_129150 = s_51_2;
        // N s_51_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #3u : u8
        let s_52_0: u8 = 3;
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
        // D s_52_7: call AArch64_AArch32SystemAccessTrap(s_52_6, s_52_4)
        let s_52_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_52_6, s_52_4);
        // N s_52_8: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var __HCR_EL2_TOCU:u8
        let s_53_0: bool = fn_state.u__HCR_EL2_TOCU;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 1u16);
        // C s_53_2: const #1u : u8
        let s_53_2: bool = true;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // D s_53_5: write-var gs#129149 <= s_53_4
        fn_state.gs_129149 = s_53_4;
        // N s_53_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #432u : u32
        let s_54_0: u32 = 432;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // D s_54_2: call ELUsingAArch32(s_54_1)
        let s_54_2: bool = ELUsingAArch32(state, tracer, s_54_1);
        // D s_54_3: not s_54_2
        let s_54_3: bool = !s_54_2;
        // D s_54_4: write-var gs#129148 <= s_54_3
        fn_state.gs_129148 = s_54_3;
        // N s_54_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #3u : u8
        let s_55_0: u8 = 3;
        // C s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 8u16);
        // C s_55_2: cast zx s_55_1 -> i
        let s_55_2: i128 = (s_55_1.value() as i128);
        // C s_55_3: cast reint s_55_2 -> i64
        let s_55_3: i64 = (s_55_2 as i64);
        // C s_55_4: cast zx s_55_3 -> i
        let s_55_4: i128 = (i128::try_from(s_55_3).unwrap());
        // C s_55_5: const #432u : u32
        let s_55_5: u32 = 432;
        // D s_55_6: read-reg s_55_5:u8
        let s_55_6: u8 = {
            let value = state.read_register::<u8>(s_55_5 as isize);
            tracer.read_register(s_55_5 as isize, value);
            value
        };
        // D s_55_7: call AArch64_AArch32SystemAccessTrap(s_55_6, s_55_4)
        let s_55_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_55_6, s_55_4);
        // N s_55_8: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __HCR_EL2_TPU:u8
        let s_56_0: bool = fn_state.u__HCR_EL2_TPU;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #1u : u8
        let s_56_2: bool = true;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#129147 <= s_56_4
        fn_state.gs_129147 = s_56_4;
        // N s_56_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #432u : u32
        let s_57_0: u32 = 432;
        // D s_57_1: read-reg s_57_0:u8
        let s_57_1: u8 = {
            let value = state.read_register::<u8>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: call ELUsingAArch32(s_57_1)
        let s_57_2: bool = ELUsingAArch32(state, tracer, s_57_1);
        // D s_57_3: not s_57_2
        let s_57_3: bool = !s_57_2;
        // D s_57_4: write-var gs#129146 <= s_57_3
        fn_state.gs_129146 = s_57_3;
        // N s_57_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #3u : u8
        let s_58_0: u8 = 3;
        // C s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 8u16);
        // C s_58_2: cast zx s_58_1 -> i
        let s_58_2: i128 = (s_58_1.value() as i128);
        // C s_58_3: cast reint s_58_2 -> i64
        let s_58_3: i64 = (s_58_2 as i64);
        // C s_58_4: cast zx s_58_3 -> i
        let s_58_4: i128 = (i128::try_from(s_58_3).unwrap());
        // S s_58_5: call AArch32_TakeHypTrapException(s_58_4)
        let s_58_5: () = AArch32_TakeHypTrapException(state, tracer, s_58_4);
        // N s_58_6: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var __HSTR_T7:u8
        let s_59_0: bool = fn_state.u__HSTR_T7;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 1u16);
        // C s_59_2: const #1u : u8
        let s_59_2: bool = true;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: write-var gs#129145 <= s_59_4
        fn_state.gs_129145 = s_59_4;
        // N s_59_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #432u : u32
        let s_60_0: u32 = 432;
        // D s_60_1: read-reg s_60_0:u8
        let s_60_1: u8 = {
            let value = state.read_register::<u8>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call ELUsingAArch32(s_60_1)
        let s_60_2: bool = ELUsingAArch32(state, tracer, s_60_1);
        // D s_60_3: write-var gs#129144 <= s_60_2
        fn_state.gs_129144 = s_60_2;
        // N s_60_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #3u : u8
        let s_61_0: u8 = 3;
        // C s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 8u16);
        // C s_61_2: cast zx s_61_1 -> i
        let s_61_2: i128 = (s_61_1.value() as i128);
        // C s_61_3: cast reint s_61_2 -> i64
        let s_61_3: i64 = (s_61_2 as i64);
        // C s_61_4: cast zx s_61_3 -> i
        let s_61_4: i128 = (i128::try_from(s_61_3).unwrap());
        // C s_61_5: const #432u : u32
        let s_61_5: u32 = 432;
        // D s_61_6: read-reg s_61_5:u8
        let s_61_6: u8 = {
            let value = state.read_register::<u8>(s_61_5 as isize);
            tracer.read_register(s_61_5 as isize, value);
            value
        };
        // D s_61_7: call AArch64_AArch32SystemAccessTrap(s_61_6, s_61_4)
        let s_61_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_61_6, s_61_4);
        // N s_61_8: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var __HSTR_EL2_T7:u8
        let s_62_0: bool = fn_state.u__HSTR_EL2_T7;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#129143 <= s_62_4
        fn_state.gs_129143 = s_62_4;
        // N s_62_6: jump b11
        return block_11(state, tracer, fn_state);
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
        // D s_63_4: write-var gs#129142 <= s_63_3
        fn_state.gs_129142 = s_63_3;
        // N s_63_5: jump b9
        return block_9(state, tracer, fn_state);
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
}
