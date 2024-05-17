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
use HCR_read::*;
use AArch32_IC__1::*;
use u_get_HSTR_EL2_Type_T7::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HCR_EL2_Type_TPU::*;
use R_read::*;
use u_get_HCR2_Type_TOCU::*;
use ELUsingAArch32::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TOCU::*;
use HCR2_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn ICIMVAU_SysRegWrite32_98e4788d39c19ba0<T: Tracer>(
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
        gs_129157: bool,
        gs_129161: bool,
        gs_129159: bool,
        u__HSTR_T7: bool,
        gs_129156: bool,
        gs_129163: bool,
        gs_129162: bool,
        gs_129165: bool,
        u__HCR_EL2_TOCU: bool,
        gs_129166: bool,
        u__PSTATE_EL: u8,
        gs_129167: bool,
        u__HCR2_TOCU: bool,
        gs_129158: bool,
        gs_129160: bool,
        u__HCR_EL2_TPU: bool,
        u__HCR_TPU: bool,
        gs_129164: bool,
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
        // N s_0_33: branch s_0_32 b56 b1
        if s_0_32 {
            return block_56(state, tracer, fn_state);
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
        // D s_5_0: read-var t:i
        let s_5_0: i128 = fn_state.t;
        // D s_5_1: call R_read(s_5_0)
        let s_5_1: u32 = R_read(state, tracer, s_5_0);
        // C s_5_2: const #1u : u32
        let s_5_2: u32 = 1;
        // D s_5_3: call AArch32_IC__1(s_5_1, s_5_2)
        let s_5_3: () = AArch32_IC__1(state, tracer, s_5_1, s_5_2);
        // N s_5_4: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var t:i
        let s_6_0: i128 = fn_state.t;
        // D s_6_1: call R_read(s_6_0)
        let s_6_1: u32 = R_read(state, tracer, s_6_0);
        // C s_6_2: const #1u : u32
        let s_6_2: u32 = 1;
        // D s_6_3: call AArch32_IC__1(s_6_1, s_6_2)
        let s_6_3: () = AArch32_IC__1(state, tracer, s_6_1, s_6_2);
        // N s_6_4: return
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
        // N s_7_2: branch s_7_1 b55 b8
        if s_7_1 {
            return block_55(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#129156 <= s_8_0
        fn_state.gs_129156 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#129156:u8
        let s_9_0: bool = fn_state.gs_129156;
        // N s_9_1: branch s_9_0 b54 b10
        if s_9_0 {
            return block_54(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#129157 <= s_10_0
        fn_state.gs_129157 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#129157:u8
        let s_11_0: bool = fn_state.gs_129157;
        // N s_11_1: branch s_11_0 b53 b12
        if s_11_0 {
            return block_53(state, tracer, fn_state);
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
        // N s_12_2: branch s_12_1 b52 b13
        if s_12_1 {
            return block_52(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#129158 <= s_13_0
        fn_state.gs_129158 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#129158:u8
        let s_14_0: bool = fn_state.gs_129158;
        // N s_14_1: branch s_14_0 b51 b15
        if s_14_0 {
            return block_51(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#129159 <= s_15_0
        fn_state.gs_129159 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#129159:u8
        let s_16_0: bool = fn_state.gs_129159;
        // N s_16_1: branch s_16_0 b50 b17
        if s_16_0 {
            return block_50(state, tracer, fn_state);
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
        // N s_17_2: branch s_17_1 b49 b18
        if s_17_1 {
            return block_49(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#129160 <= s_18_0
        fn_state.gs_129160 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#129160:u8
        let s_19_0: bool = fn_state.gs_129160;
        // N s_19_1: branch s_19_0 b48 b20
        if s_19_0 {
            return block_48(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#129161 <= s_20_0
        fn_state.gs_129161 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#129161:u8
        let s_21_0: bool = fn_state.gs_129161;
        // N s_21_1: branch s_21_0 b47 b22
        if s_21_0 {
            return block_47(state, tracer, fn_state);
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
        // N s_22_2: branch s_22_1 b46 b23
        if s_22_1 {
            return block_46(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#129162 <= s_23_0
        fn_state.gs_129162 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#129162:u8
        let s_24_0: bool = fn_state.gs_129162;
        // N s_24_1: branch s_24_0 b45 b25
        if s_24_0 {
            return block_45(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#129163 <= s_25_0
        fn_state.gs_129163 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#129163:u8
        let s_26_0: bool = fn_state.gs_129163;
        // N s_26_1: branch s_26_0 b44 b27
        if s_26_0 {
            return block_44(state, tracer, fn_state);
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
        // N s_27_2: branch s_27_1 b43 b28
        if s_27_1 {
            return block_43(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#129164 <= s_28_0
        fn_state.gs_129164 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#129164:u8
        let s_29_0: bool = fn_state.gs_129164;
        // N s_29_1: branch s_29_0 b42 b30
        if s_29_0 {
            return block_42(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#129165 <= s_30_0
        fn_state.gs_129165 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#129165:u8
        let s_31_0: bool = fn_state.gs_129165;
        // N s_31_1: branch s_31_0 b41 b32
        if s_31_0 {
            return block_41(state, tracer, fn_state);
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
        // N s_32_2: branch s_32_1 b40 b33
        if s_32_1 {
            return block_40(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#129166 <= s_33_0
        fn_state.gs_129166 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#129166:u8
        let s_34_0: bool = fn_state.gs_129166;
        // N s_34_1: branch s_34_0 b39 b35
        if s_34_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#129167 <= s_35_0
        fn_state.gs_129167 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#129167:u8
        let s_36_0: bool = fn_state.gs_129167;
        // N s_36_1: branch s_36_0 b38 b37
        if s_36_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var t:i
        let s_37_0: i128 = fn_state.t;
        // D s_37_1: call R_read(s_37_0)
        let s_37_1: u32 = R_read(state, tracer, s_37_0);
        // C s_37_2: const #1u : u32
        let s_37_2: u32 = 1;
        // D s_37_3: call AArch32_IC__1(s_37_1, s_37_2)
        let s_37_3: () = AArch32_IC__1(state, tracer, s_37_1, s_37_2);
        // N s_37_4: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #3u : u8
        let s_38_0: u8 = 3;
        // C s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 8u16);
        // C s_38_2: cast zx s_38_1 -> i
        let s_38_2: i128 = (s_38_1.value() as i128);
        // C s_38_3: cast reint s_38_2 -> i64
        let s_38_3: i64 = (s_38_2 as i64);
        // C s_38_4: cast zx s_38_3 -> i
        let s_38_4: i128 = (i128::try_from(s_38_3).unwrap());
        // S s_38_5: call AArch32_TakeHypTrapException(s_38_4)
        let s_38_5: () = AArch32_TakeHypTrapException(state, tracer, s_38_4);
        // N s_38_6: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var __HCR2_TOCU:u8
        let s_39_0: bool = fn_state.u__HCR2_TOCU;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#129167 <= s_39_4
        fn_state.gs_129167 = s_39_4;
        // N s_39_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #432u : u32
        let s_40_0: u32 = 432;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: call ELUsingAArch32(s_40_1)
        let s_40_2: bool = ELUsingAArch32(state, tracer, s_40_1);
        // D s_40_3: write-var gs#129166 <= s_40_2
        fn_state.gs_129166 = s_40_2;
        // N s_40_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #3u : u8
        let s_41_0: u8 = 3;
        // C s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 8u16);
        // C s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (s_41_1.value() as i128);
        // C s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: cast zx s_41_3 -> i
        let s_41_4: i128 = (i128::try_from(s_41_3).unwrap());
        // S s_41_5: call AArch32_TakeHypTrapException(s_41_4)
        let s_41_5: () = AArch32_TakeHypTrapException(state, tracer, s_41_4);
        // N s_41_6: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var __HCR_TPU:u8
        let s_42_0: bool = fn_state.u__HCR_TPU;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 1u16);
        // C s_42_2: const #1u : u8
        let s_42_2: bool = true;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: write-var gs#129165 <= s_42_4
        fn_state.gs_129165 = s_42_4;
        // N s_42_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #432u : u32
        let s_43_0: u32 = 432;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call ELUsingAArch32(s_43_1)
        let s_43_2: bool = ELUsingAArch32(state, tracer, s_43_1);
        // D s_43_3: write-var gs#129164 <= s_43_2
        fn_state.gs_129164 = s_43_2;
        // N s_43_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #3u : u8
        let s_44_0: u8 = 3;
        // C s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 8u16);
        // C s_44_2: cast zx s_44_1 -> i
        let s_44_2: i128 = (s_44_1.value() as i128);
        // C s_44_3: cast reint s_44_2 -> i64
        let s_44_3: i64 = (s_44_2 as i64);
        // C s_44_4: cast zx s_44_3 -> i
        let s_44_4: i128 = (i128::try_from(s_44_3).unwrap());
        // C s_44_5: const #432u : u32
        let s_44_5: u32 = 432;
        // D s_44_6: read-reg s_44_5:u8
        let s_44_6: u8 = {
            let value = state.read_register::<u8>(s_44_5 as isize);
            tracer.read_register(s_44_5 as isize, value);
            value
        };
        // D s_44_7: call AArch64_AArch32SystemAccessTrap(s_44_6, s_44_4)
        let s_44_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_44_6, s_44_4);
        // N s_44_8: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var __HCR_EL2_TOCU:u8
        let s_45_0: bool = fn_state.u__HCR_EL2_TOCU;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 1u16);
        // C s_45_2: const #1u : u8
        let s_45_2: bool = true;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: write-var gs#129163 <= s_45_4
        fn_state.gs_129163 = s_45_4;
        // N s_45_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #432u : u32
        let s_46_0: u32 = 432;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: u8 = {
            let value = state.read_register::<u8>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call ELUsingAArch32(s_46_1)
        let s_46_2: bool = ELUsingAArch32(state, tracer, s_46_1);
        // D s_46_3: not s_46_2
        let s_46_3: bool = !s_46_2;
        // D s_46_4: write-var gs#129162 <= s_46_3
        fn_state.gs_129162 = s_46_3;
        // N s_46_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #3u : u8
        let s_47_0: u8 = 3;
        // C s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 8u16);
        // C s_47_2: cast zx s_47_1 -> i
        let s_47_2: i128 = (s_47_1.value() as i128);
        // C s_47_3: cast reint s_47_2 -> i64
        let s_47_3: i64 = (s_47_2 as i64);
        // C s_47_4: cast zx s_47_3 -> i
        let s_47_4: i128 = (i128::try_from(s_47_3).unwrap());
        // C s_47_5: const #432u : u32
        let s_47_5: u32 = 432;
        // D s_47_6: read-reg s_47_5:u8
        let s_47_6: u8 = {
            let value = state.read_register::<u8>(s_47_5 as isize);
            tracer.read_register(s_47_5 as isize, value);
            value
        };
        // D s_47_7: call AArch64_AArch32SystemAccessTrap(s_47_6, s_47_4)
        let s_47_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_47_6, s_47_4);
        // N s_47_8: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __HCR_EL2_TPU:u8
        let s_48_0: bool = fn_state.u__HCR_EL2_TPU;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #1u : u8
        let s_48_2: bool = true;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#129161 <= s_48_4
        fn_state.gs_129161 = s_48_4;
        // N s_48_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #432u : u32
        let s_49_0: u32 = 432;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: call ELUsingAArch32(s_49_1)
        let s_49_2: bool = ELUsingAArch32(state, tracer, s_49_1);
        // D s_49_3: not s_49_2
        let s_49_3: bool = !s_49_2;
        // D s_49_4: write-var gs#129160 <= s_49_3
        fn_state.gs_129160 = s_49_3;
        // N s_49_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #3u : u8
        let s_50_0: u8 = 3;
        // C s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 8u16);
        // C s_50_2: cast zx s_50_1 -> i
        let s_50_2: i128 = (s_50_1.value() as i128);
        // C s_50_3: cast reint s_50_2 -> i64
        let s_50_3: i64 = (s_50_2 as i64);
        // C s_50_4: cast zx s_50_3 -> i
        let s_50_4: i128 = (i128::try_from(s_50_3).unwrap());
        // S s_50_5: call AArch32_TakeHypTrapException(s_50_4)
        let s_50_5: () = AArch32_TakeHypTrapException(state, tracer, s_50_4);
        // N s_50_6: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var __HSTR_T7:u8
        let s_51_0: bool = fn_state.u__HSTR_T7;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #1u : u8
        let s_51_2: bool = true;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#129159 <= s_51_4
        fn_state.gs_129159 = s_51_4;
        // N s_51_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #432u : u32
        let s_52_0: u32 = 432;
        // D s_52_1: read-reg s_52_0:u8
        let s_52_1: u8 = {
            let value = state.read_register::<u8>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: call ELUsingAArch32(s_52_1)
        let s_52_2: bool = ELUsingAArch32(state, tracer, s_52_1);
        // D s_52_3: write-var gs#129158 <= s_52_2
        fn_state.gs_129158 = s_52_2;
        // N s_52_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #3u : u8
        let s_53_0: u8 = 3;
        // C s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 8u16);
        // C s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (s_53_1.value() as i128);
        // C s_53_3: cast reint s_53_2 -> i64
        let s_53_3: i64 = (s_53_2 as i64);
        // C s_53_4: cast zx s_53_3 -> i
        let s_53_4: i128 = (i128::try_from(s_53_3).unwrap());
        // C s_53_5: const #432u : u32
        let s_53_5: u32 = 432;
        // D s_53_6: read-reg s_53_5:u8
        let s_53_6: u8 = {
            let value = state.read_register::<u8>(s_53_5 as isize);
            tracer.read_register(s_53_5 as isize, value);
            value
        };
        // D s_53_7: call AArch64_AArch32SystemAccessTrap(s_53_6, s_53_4)
        let s_53_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_53_6, s_53_4);
        // N s_53_8: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var __HSTR_EL2_T7:u8
        let s_54_0: bool = fn_state.u__HSTR_EL2_T7;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 1u16);
        // C s_54_2: const #1u : u8
        let s_54_2: bool = true;
        // C s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 1u16);
        // D s_54_4: cmp-eq s_54_1 s_54_3
        let s_54_4: bool = ((s_54_1) == (s_54_3));
        // D s_54_5: write-var gs#129157 <= s_54_4
        fn_state.gs_129157 = s_54_4;
        // N s_54_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #432u : u32
        let s_55_0: u32 = 432;
        // D s_55_1: read-reg s_55_0:u8
        let s_55_1: u8 = {
            let value = state.read_register::<u8>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // D s_55_2: call ELUsingAArch32(s_55_1)
        let s_55_2: bool = ELUsingAArch32(state, tracer, s_55_1);
        // D s_55_3: not s_55_2
        let s_55_3: bool = !s_55_2;
        // D s_55_4: write-var gs#129156 <= s_55_3
        fn_state.gs_129156 = s_55_3;
        // N s_55_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_56_0: panic
        panic!("{:?}", ());
        // N s_56_1: return
        return;
    }
}
