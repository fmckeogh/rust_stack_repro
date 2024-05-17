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
use HCR_read::*;
use u_get_HCR_EL2_Type_TRVM::*;
use u_get_SCR_Type_NS::*;
use HSTR_read::*;
use u_get_HCR_Type_TRVM::*;
use EAEisOne::*;
use u_get_HSTR_EL2_Type_T10::*;
use NMRR_read::*;
use MAIR1_S_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use R_set::*;
use MAIR1_NS_read::*;
use ELUsingAArch32::*;
use u_get_HSTR_Type_T10::*;
use MAIR1_read::*;
use EL2Enabled::*;
use NMRR_NS_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn NMRR_SysRegRead32_f8e132fd5a9e9c17<T: Tracer>(
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
        gs_112621: bool,
        ga_184226: ProductType700c18a878c5601b,
        gs_112614: bool,
        ga_184219: ProductType700c18a878c5601b,
        gs_112616: bool,
        ga_184206: ProductType700c18a878c5601b,
        ga_184237: ProductType700c18a878c5601b,
        gs_112620: bool,
        ga_184211: ProductType700c18a878c5601b,
        gs_112613: bool,
        u__HCR_TRVM: bool,
        gs_112622: bool,
        gs_112618: bool,
        gs_112619: bool,
        ga_184231: ProductType700c18a878c5601b,
        ga_184233: ProductType700c18a878c5601b,
        ga_184224: ProductType700c18a878c5601b,
        ga_184213: ProductType700c18a878c5601b,
        u__PSTATE_EL: u8,
        u__HSTR_T10: bool,
        u__HCR_EL2_TRVM: bool,
        gs_112615: bool,
        u__SCR_NS: bool,
        ga_184208: ProductType700c18a878c5601b,
        ga_184221: ProductType700c18a878c5601b,
        u__HSTR_EL2_T10: bool,
        gs_112617: bool,
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
        // D s_0_5: call _get_HSTR_EL2_Type_T10(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T10(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T10 <= s_0_5
        fn_state.u__HSTR_EL2_T10 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T10(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T10(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T10 <= s_0_9
        fn_state.u__HSTR_T10 = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TRVM(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TRVM(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TRVM <= s_0_13
        fn_state.u__HCR_EL2_TRVM = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HCR_Type_TRVM(s_0_16)
        let s_0_17: bool = u_get_HCR_Type_TRVM(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_TRVM <= s_0_17
        fn_state.u__HCR_TRVM = s_0_17;
        // C s_0_19: const #20920u : u32
        let s_0_19: u32 = 20920;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_SCR_Type_NS(s_0_20)
        let s_0_21: bool = u_get_SCR_Type_NS(state, tracer, s_0_20);
        // D s_0_22: write-var __SCR_NS <= s_0_21
        fn_state.u__SCR_NS = s_0_21;
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
        // N s_0_29: branch s_0_28 b64 b1
        if s_0_28 {
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
        // N s_1_6: branch s_1_5 b22 b2
        if s_1_5 {
            return block_22(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b12 b3
        if s_2_5 {
            return block_12(state, tracer, fn_state);
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
        // S s_5_1: call EAEisOne(s_5_0)
        let s_5_1: bool = EAEisOne(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b9 b6
        if s_5_1 {
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
        // D s_6_0: read-var __SCR_NS:u8
        let s_6_0: bool = fn_state.u__SCR_NS;
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
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call NMRR_NS_read(s_7_0)
        let s_7_1: ProductType700c18a878c5601b = NMRR_NS_read(state, tracer, s_7_0);
        // D s_7_2: write-var ga#184237 <= s_7_1
        fn_state.ga_184237 = s_7_1;
        // D s_7_3: read-var ga#184237.0:struct
        let s_7_3: u32 = fn_state.ga_184237._0;
        // D s_7_4: read-var t:i
        let s_7_4: i128 = fn_state.t;
        // D s_7_5: call R_set(s_7_4, s_7_3)
        let s_7_5: () = R_set(state, tracer, s_7_4, s_7_3);
        // N s_7_6: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #100840u : u32
        let s_8_0: u32 = 100840;
        // D s_8_1: read-reg s_8_0:u32
        let s_8_1: u32 = {
            let value = state.read_register::<u32>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: read-var t:i
        let s_8_2: i128 = fn_state.t;
        // D s_8_3: call R_set(s_8_2, s_8_1)
        let s_8_3: () = R_set(state, tracer, s_8_2, s_8_1);
        // N s_8_4: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var __SCR_NS:u8
        let s_9_0: bool = fn_state.u__SCR_NS;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // C s_9_2: const #0u : u8
        let s_9_2: bool = false;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
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
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call MAIR1_NS_read(s_10_0)
        let s_10_1: ProductType700c18a878c5601b = MAIR1_NS_read(state, tracer, s_10_0);
        // D s_10_2: write-var ga#184233 <= s_10_1
        fn_state.ga_184233 = s_10_1;
        // D s_10_3: read-var ga#184233.0:struct
        let s_10_3: u32 = fn_state.ga_184233._0;
        // D s_10_4: read-var t:i
        let s_10_4: i128 = fn_state.t;
        // D s_10_5: call R_set(s_10_4, s_10_3)
        let s_10_5: () = R_set(state, tracer, s_10_4, s_10_3);
        // N s_10_6: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call MAIR1_S_read(s_11_0)
        let s_11_1: ProductType700c18a878c5601b = MAIR1_S_read(state, tracer, s_11_0);
        // D s_11_2: write-var ga#184231 <= s_11_1
        fn_state.ga_184231 = s_11_1;
        // D s_11_3: read-var ga#184231.0:struct
        let s_11_3: u32 = fn_state.ga_184231._0;
        // D s_11_4: read-var t:i
        let s_11_4: i128 = fn_state.t;
        // D s_11_5: call R_set(s_11_4, s_11_3)
        let s_11_5: () = R_set(state, tracer, s_11_4, s_11_3);
        // N s_11_6: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #424u : u32
        let s_12_0: u32 = 424;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #2u : u8
        let s_12_2: u8 = 2;
        // D s_12_3: cmp-lt s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) < (s_12_2));
        // N s_12_4: branch s_12_3 b21 b13
        if s_12_3 {
            return block_21(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#112613 <= s_13_0
        fn_state.gs_112613 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#112613:u8
        let s_14_0: bool = fn_state.gs_112613;
        // N s_14_1: branch s_14_0 b18 b15
        if s_14_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call EAEisOne(s_15_0)
        let s_15_1: bool = EAEisOne(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b17 b16
        if s_15_1 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call NMRR_read(s_16_0)
        let s_16_1: ProductType700c18a878c5601b = NMRR_read(state, tracer, s_16_0);
        // D s_16_2: write-var ga#184226 <= s_16_1
        fn_state.ga_184226 = s_16_1;
        // D s_16_3: read-var ga#184226.0:struct
        let s_16_3: u32 = fn_state.ga_184226._0;
        // D s_16_4: read-var t:i
        let s_16_4: i128 = fn_state.t;
        // D s_16_5: call R_set(s_16_4, s_16_3)
        let s_16_5: () = R_set(state, tracer, s_16_4, s_16_3);
        // N s_16_6: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call MAIR1_read(s_17_0)
        let s_17_1: ProductType700c18a878c5601b = MAIR1_read(state, tracer, s_17_0);
        // D s_17_2: write-var ga#184224 <= s_17_1
        fn_state.ga_184224 = s_17_1;
        // D s_17_3: read-var ga#184224.0:struct
        let s_17_3: u32 = fn_state.ga_184224._0;
        // D s_17_4: read-var t:i
        let s_17_4: i128 = fn_state.t;
        // D s_17_5: call R_set(s_17_4, s_17_3)
        let s_17_5: () = R_set(state, tracer, s_17_4, s_17_3);
        // N s_17_6: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call EAEisOne(s_18_0)
        let s_18_1: bool = EAEisOne(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b20 b19
        if s_18_1 {
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
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call NMRR_NS_read(s_19_0)
        let s_19_1: ProductType700c18a878c5601b = NMRR_NS_read(state, tracer, s_19_0);
        // D s_19_2: write-var ga#184221 <= s_19_1
        fn_state.ga_184221 = s_19_1;
        // D s_19_3: read-var ga#184221.0:struct
        let s_19_3: u32 = fn_state.ga_184221._0;
        // D s_19_4: read-var t:i
        let s_19_4: i128 = fn_state.t;
        // D s_19_5: call R_set(s_19_4, s_19_3)
        let s_19_5: () = R_set(state, tracer, s_19_4, s_19_3);
        // N s_19_6: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call MAIR1_NS_read(s_20_0)
        let s_20_1: ProductType700c18a878c5601b = MAIR1_NS_read(state, tracer, s_20_0);
        // D s_20_2: write-var ga#184219 <= s_20_1
        fn_state.ga_184219 = s_20_1;
        // D s_20_3: read-var ga#184219.0:struct
        let s_20_3: u32 = fn_state.ga_184219._0;
        // D s_20_4: read-var t:i
        let s_20_4: i128 = fn_state.t;
        // D s_20_5: call R_set(s_20_4, s_20_3)
        let s_20_5: () = R_set(state, tracer, s_20_4, s_20_3);
        // N s_20_6: return
        return;
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
        // D s_21_2: call ELUsingAArch32(s_21_1)
        let s_21_2: bool = ELUsingAArch32(state, tracer, s_21_1);
        // D s_21_3: write-var gs#112613 <= s_21_2
        fn_state.gs_112613 = s_21_2;
        // N s_21_4: jump b14
        return block_14(state, tracer, fn_state);
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
        // N s_22_2: branch s_22_1 b63 b23
        if s_22_1 {
            return block_63(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#112614 <= s_23_0
        fn_state.gs_112614 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#112614:u8
        let s_24_0: bool = fn_state.gs_112614;
        // N s_24_1: branch s_24_0 b62 b25
        if s_24_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#112615 <= s_25_0
        fn_state.gs_112615 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#112615:u8
        let s_26_0: bool = fn_state.gs_112615;
        // N s_26_1: branch s_26_0 b61 b27
        if s_26_0 {
            return block_61(state, tracer, fn_state);
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
        // N s_27_2: branch s_27_1 b60 b28
        if s_27_1 {
            return block_60(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#112616 <= s_28_0
        fn_state.gs_112616 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#112616:u8
        let s_29_0: bool = fn_state.gs_112616;
        // N s_29_1: branch s_29_0 b59 b30
        if s_29_0 {
            return block_59(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#112617 <= s_30_0
        fn_state.gs_112617 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#112617:u8
        let s_31_0: bool = fn_state.gs_112617;
        // N s_31_1: branch s_31_0 b58 b32
        if s_31_0 {
            return block_58(state, tracer, fn_state);
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
        // N s_32_2: branch s_32_1 b57 b33
        if s_32_1 {
            return block_57(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#112618 <= s_33_0
        fn_state.gs_112618 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#112618:u8
        let s_34_0: bool = fn_state.gs_112618;
        // N s_34_1: branch s_34_0 b56 b35
        if s_34_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#112619 <= s_35_0
        fn_state.gs_112619 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#112619:u8
        let s_36_0: bool = fn_state.gs_112619;
        // N s_36_1: branch s_36_0 b55 b37
        if s_36_0 {
            return block_55(state, tracer, fn_state);
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
        // N s_37_2: branch s_37_1 b54 b38
        if s_37_1 {
            return block_54(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#112620 <= s_38_0
        fn_state.gs_112620 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#112620:u8
        let s_39_0: bool = fn_state.gs_112620;
        // N s_39_1: branch s_39_0 b53 b40
        if s_39_0 {
            return block_53(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#112621 <= s_40_0
        fn_state.gs_112621 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#112621:u8
        let s_41_0: bool = fn_state.gs_112621;
        // N s_41_1: branch s_41_0 b52 b42
        if s_41_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #424u : u32
        let s_42_0: u32 = 424;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: u8 = {
            let value = state.read_register::<u8>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // C s_42_2: const #2u : u8
        let s_42_2: u8 = 2;
        // D s_42_3: cmp-lt s_42_1 s_42_2
        let s_42_3: bool = ((s_42_1) < (s_42_2));
        // N s_42_4: branch s_42_3 b51 b43
        if s_42_3 {
            return block_51(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#112622 <= s_43_0
        fn_state.gs_112622 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#112622:u8
        let s_44_0: bool = fn_state.gs_112622;
        // N s_44_1: branch s_44_0 b48 b45
        if s_44_0 {
            return block_48(state, tracer, fn_state);
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
        // S s_45_1: call EAEisOne(s_45_0)
        let s_45_1: bool = EAEisOne(state, tracer, s_45_0);
        // N s_45_2: branch s_45_1 b47 b46
        if s_45_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call NMRR_read(s_46_0)
        let s_46_1: ProductType700c18a878c5601b = NMRR_read(state, tracer, s_46_0);
        // D s_46_2: write-var ga#184213 <= s_46_1
        fn_state.ga_184213 = s_46_1;
        // D s_46_3: read-var ga#184213.0:struct
        let s_46_3: u32 = fn_state.ga_184213._0;
        // D s_46_4: read-var t:i
        let s_46_4: i128 = fn_state.t;
        // D s_46_5: call R_set(s_46_4, s_46_3)
        let s_46_5: () = R_set(state, tracer, s_46_4, s_46_3);
        // N s_46_6: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call MAIR1_read(s_47_0)
        let s_47_1: ProductType700c18a878c5601b = MAIR1_read(state, tracer, s_47_0);
        // D s_47_2: write-var ga#184211 <= s_47_1
        fn_state.ga_184211 = s_47_1;
        // D s_47_3: read-var ga#184211.0:struct
        let s_47_3: u32 = fn_state.ga_184211._0;
        // D s_47_4: read-var t:i
        let s_47_4: i128 = fn_state.t;
        // D s_47_5: call R_set(s_47_4, s_47_3)
        let s_47_5: () = R_set(state, tracer, s_47_4, s_47_3);
        // N s_47_6: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EAEisOne(s_48_0)
        let s_48_1: bool = EAEisOne(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b50 b49
        if s_48_1 {
            return block_50(state, tracer, fn_state);
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
        // S s_49_1: call NMRR_NS_read(s_49_0)
        let s_49_1: ProductType700c18a878c5601b = NMRR_NS_read(state, tracer, s_49_0);
        // D s_49_2: write-var ga#184208 <= s_49_1
        fn_state.ga_184208 = s_49_1;
        // D s_49_3: read-var ga#184208.0:struct
        let s_49_3: u32 = fn_state.ga_184208._0;
        // D s_49_4: read-var t:i
        let s_49_4: i128 = fn_state.t;
        // D s_49_5: call R_set(s_49_4, s_49_3)
        let s_49_5: () = R_set(state, tracer, s_49_4, s_49_3);
        // N s_49_6: return
        return;
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call MAIR1_NS_read(s_50_0)
        let s_50_1: ProductType700c18a878c5601b = MAIR1_NS_read(state, tracer, s_50_0);
        // D s_50_2: write-var ga#184206 <= s_50_1
        fn_state.ga_184206 = s_50_1;
        // D s_50_3: read-var ga#184206.0:struct
        let s_50_3: u32 = fn_state.ga_184206._0;
        // D s_50_4: read-var t:i
        let s_50_4: i128 = fn_state.t;
        // D s_50_5: call R_set(s_50_4, s_50_3)
        let s_50_5: () = R_set(state, tracer, s_50_4, s_50_3);
        // N s_50_6: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #424u : u32
        let s_51_0: u32 = 424;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: u8 = {
            let value = state.read_register::<u8>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call ELUsingAArch32(s_51_1)
        let s_51_2: bool = ELUsingAArch32(state, tracer, s_51_1);
        // D s_51_3: write-var gs#112622 <= s_51_2
        fn_state.gs_112622 = s_51_2;
        // N s_51_4: jump b44
        return block_44(state, tracer, fn_state);
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
        // S s_52_5: call AArch32_TakeHypTrapException(s_52_4)
        let s_52_5: () = AArch32_TakeHypTrapException(state, tracer, s_52_4);
        // N s_52_6: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var __HCR_TRVM:u8
        let s_53_0: bool = fn_state.u__HCR_TRVM;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 1u16);
        // C s_53_2: const #1u : u8
        let s_53_2: bool = true;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // D s_53_5: write-var gs#112621 <= s_53_4
        fn_state.gs_112621 = s_53_4;
        // N s_53_6: jump b41
        return block_41(state, tracer, fn_state);
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
        // D s_54_3: write-var gs#112620 <= s_54_2
        fn_state.gs_112620 = s_54_2;
        // N s_54_4: jump b39
        return block_39(state, tracer, fn_state);
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
        // D s_56_0: read-var __HCR_EL2_TRVM:u8
        let s_56_0: bool = fn_state.u__HCR_EL2_TRVM;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #1u : u8
        let s_56_2: bool = true;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#112619 <= s_56_4
        fn_state.gs_112619 = s_56_4;
        // N s_56_6: jump b36
        return block_36(state, tracer, fn_state);
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
        // D s_57_4: write-var gs#112618 <= s_57_3
        fn_state.gs_112618 = s_57_3;
        // N s_57_5: jump b34
        return block_34(state, tracer, fn_state);
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
        // D s_59_0: read-var __HSTR_T10:u8
        let s_59_0: bool = fn_state.u__HSTR_T10;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 1u16);
        // C s_59_2: const #1u : u8
        let s_59_2: bool = true;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: write-var gs#112617 <= s_59_4
        fn_state.gs_112617 = s_59_4;
        // N s_59_6: jump b31
        return block_31(state, tracer, fn_state);
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
        // D s_60_3: write-var gs#112616 <= s_60_2
        fn_state.gs_112616 = s_60_2;
        // N s_60_4: jump b29
        return block_29(state, tracer, fn_state);
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
        // D s_62_0: read-var __HSTR_EL2_T10:u8
        let s_62_0: bool = fn_state.u__HSTR_EL2_T10;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#112615 <= s_62_4
        fn_state.gs_112615 = s_62_4;
        // N s_62_6: jump b26
        return block_26(state, tracer, fn_state);
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
        // D s_63_4: write-var gs#112614 <= s_63_3
        fn_state.gs_112614 = s_63_3;
        // N s_63_5: jump b24
        return block_24(state, tracer, fn_state);
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
