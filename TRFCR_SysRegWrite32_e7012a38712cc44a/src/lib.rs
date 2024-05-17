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
use u_get_HDCR_Type_TTRF::*;
use EL2Enabled::*;
use AArch32_TakeHypTrapException::*;
use u_get_HSTR_EL2_Type_T1::*;
use HDCR_read::*;
use Halted::*;
use Mk_TRFCR_Type::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_SDCR_Type_TTRF::*;
use R_read::*;
use u_get_MDCR_EL3_Type_TTRF::*;
use ELUsingAArch32::*;
use u_get_HSTR_Type_T1::*;
use u_get_EDSCR_Type_SDD::*;
use TRFCR_write::*;
use u_get_MDCR_EL2_Type_TTRF::*;
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use common::*;
pub fn TRFCR_SysRegWrite32_e7012a38712cc44a<T: Tracer>(
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
        gs_134355: bool,
        gs_134340: bool,
        gs_134345: bool,
        gs_134346: bool,
        gs_134372: bool,
        u__HSTR_EL2_T1: bool,
        u__HDCR_TTRF: bool,
        gs_134358: bool,
        gs_134339: bool,
        gs_134333: bool,
        u__HSTR_T1: bool,
        gs_134359: bool,
        gs_134332: bool,
        gs_134357: bool,
        u__SDCR_TTRF: bool,
        gs_134373: bool,
        gs_134351: bool,
        gs_134342: bool,
        gs_134353: bool,
        u__MDCR_EL2_TTRF: bool,
        u__MDCR_EL3_TTRF: bool,
        u__PSTATE_EL: u8,
        gs_134371: bool,
        gs_134336: bool,
        gs_134370: bool,
        gs_134364: bool,
        gs_134361: bool,
        gs_134331: bool,
        gs_134338: bool,
        gs_134337: bool,
        gs_134341: bool,
        gs_134368: bool,
        gs_134335: bool,
        gs_134344: bool,
        gs_134354: bool,
        gs_134352: bool,
        gs_134367: bool,
        gs_134348: bool,
        gs_134360: bool,
        gs_134369: bool,
        gs_134349: bool,
        gs_134363: bool,
        gs_134350: bool,
        gs_134356: bool,
        u__PSTATE_M: u8,
        gs_134362: bool,
        gs_134366: bool,
        gs_134343: bool,
        gs_134334: bool,
        gs_134365: bool,
        gs_134347: bool,
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
        // C s_0_3: const #22712u : u32
        let s_0_3: u32 = 22712;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDCR_EL3_Type_TTRF(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_TTRF(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_TTRF <= s_0_5
        fn_state.u__MDCR_EL3_TTRF = s_0_5;
        // C s_0_7: const #16983u : u32
        let s_0_7: u32 = 16983;
        // D s_0_8: read-reg s_0_7:u8
        let s_0_8: u8 = {
            let value = state.read_register::<u8>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: write-var __PSTATE_M <= s_0_8
        fn_state.u__PSTATE_M = s_0_8;
        // C s_0_10: const #15048u : u32
        let s_0_10: u32 = 15048;
        // D s_0_11: read-reg s_0_10:struct
        let s_0_11: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // D s_0_12: call _get_SDCR_Type_TTRF(s_0_11)
        let s_0_12: bool = u_get_SDCR_Type_TTRF(state, tracer, s_0_11);
        // D s_0_13: write-var __SDCR_TTRF <= s_0_12
        fn_state.u__SDCR_TTRF = s_0_12;
        // C s_0_14: const #104936u : u32
        let s_0_14: u32 = 104936;
        // D s_0_15: read-reg s_0_14:struct
        let s_0_15: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_14 as isize);
            tracer.read_register(s_0_14 as isize, value);
            value
        };
        // D s_0_16: call _get_HSTR_EL2_Type_T1(s_0_15)
        let s_0_16: bool = u_get_HSTR_EL2_Type_T1(state, tracer, s_0_15);
        // D s_0_17: write-var __HSTR_EL2_T1 <= s_0_16
        fn_state.u__HSTR_EL2_T1 = s_0_16;
        // C s_0_18: const #() : ()
        let s_0_18: () = ();
        // S s_0_19: call HSTR_read(s_0_18)
        let s_0_19: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_18);
        // S s_0_20: call _get_HSTR_Type_T1(s_0_19)
        let s_0_20: bool = u_get_HSTR_Type_T1(state, tracer, s_0_19);
        // D s_0_21: write-var __HSTR_T1 <= s_0_20
        fn_state.u__HSTR_T1 = s_0_20;
        // C s_0_22: const #104880u : u32
        let s_0_22: u32 = 104880;
        // D s_0_23: read-reg s_0_22:struct
        let s_0_23: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_22 as isize);
            tracer.read_register(s_0_22 as isize, value);
            value
        };
        // D s_0_24: call _get_MDCR_EL2_Type_TTRF(s_0_23)
        let s_0_24: bool = u_get_MDCR_EL2_Type_TTRF(state, tracer, s_0_23);
        // D s_0_25: write-var __MDCR_EL2_TTRF <= s_0_24
        fn_state.u__MDCR_EL2_TTRF = s_0_24;
        // C s_0_26: const #() : ()
        let s_0_26: () = ();
        // S s_0_27: call HDCR_read(s_0_26)
        let s_0_27: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_26);
        // S s_0_28: call _get_HDCR_Type_TTRF(s_0_27)
        let s_0_28: bool = u_get_HDCR_Type_TTRF(state, tracer, s_0_27);
        // D s_0_29: write-var __HDCR_TTRF <= s_0_28
        fn_state.u__HDCR_TTRF = s_0_28;
        // D s_0_30: read-var __PSTATE_EL:u8
        let s_0_30: u8 = fn_state.u__PSTATE_EL;
        // D s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 2u16);
        // C s_0_32: const #448u : u32
        let s_0_32: u32 = 448;
        // D s_0_33: read-reg s_0_32:u8
        let s_0_33: u8 = {
            let value = state.read_register::<u8>(s_0_32 as isize);
            tracer.read_register(s_0_32 as isize, value);
            value
        };
        // D s_0_34: cast zx s_0_33 -> bv
        let s_0_34: Bits = Bits::new(s_0_33 as u128, 2u16);
        // D s_0_35: cmp-eq s_0_31 s_0_34
        let s_0_35: bool = ((s_0_31) == (s_0_34));
        // N s_0_36: branch s_0_35 b171 b1
        if s_0_35 {
            return block_171(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b72 b2
        if s_1_5 {
            return block_72(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b11 b3
        if s_2_5 {
            return block_11(state, tracer, fn_state);
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
        // D s_5_0: read-var __PSTATE_M:u8
        let s_5_0: u8 = fn_state.u__PSTATE_M;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 5u16);
        // C s_5_2: const #384u : u32
        let s_5_2: u32 = 384;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 5u16);
        // D s_5_5: cmp-ne s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) != (s_5_4));
        // N s_5_6: branch s_5_5 b10 b6
        if s_5_5 {
            return block_10(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#134331 <= s_6_0
        fn_state.gs_134331 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#134331:u8
        let s_7_0: bool = fn_state.gs_134331;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var t:i
        let s_8_0: i128 = fn_state.t;
        // D s_8_1: call R_read(s_8_0)
        let s_8_1: u32 = R_read(state, tracer, s_8_0);
        // D s_8_2: call Mk_TRFCR_Type(s_8_1)
        let s_8_2: ProductType700c18a878c5601b = Mk_TRFCR_Type(state, tracer, s_8_1);
        // D s_8_3: call TRFCR_write(s_8_2)
        let s_8_3: () = TRFCR_write(state, tracer, s_8_2);
        // N s_8_4: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call AArch32_TakeMonitorTrapException(s_9_0)
        let s_9_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_9_0);
        // N s_9_2: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var __SDCR_TTRF:u8
        let s_10_0: bool = fn_state.u__SDCR_TTRF;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: write-var gs#134331 <= s_10_4
        fn_state.gs_134331 = s_10_4;
        // N s_10_6: jump b7
        return block_7(state, tracer, fn_state);
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
        // N s_11_2: branch s_11_1 b71 b12
        if s_11_1 {
            return block_71(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#134332 <= s_12_0
        fn_state.gs_134332 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#134332:u8
        let s_13_0: bool = fn_state.gs_134332;
        // N s_13_1: branch s_13_0 b70 b14
        if s_13_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#134333 <= s_14_0
        fn_state.gs_134333 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#134333:u8
        let s_15_0: bool = fn_state.gs_134333;
        // N s_15_1: branch s_15_0 b69 b16
        if s_15_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#134334 <= s_16_0
        fn_state.gs_134334 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#134334:u8
        let s_17_0: bool = fn_state.gs_134334;
        // N s_17_1: branch s_17_0 b68 b18
        if s_17_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#134335 <= s_18_0
        fn_state.gs_134335 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#134335:u8
        let s_19_0: bool = fn_state.gs_134335;
        // N s_19_1: branch s_19_0 b67 b20
        if s_19_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#134336 <= s_20_0
        fn_state.gs_134336 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#134336:u8
        let s_21_0: bool = fn_state.gs_134336;
        // N s_21_1: branch s_21_0 b66 b22
        if s_21_0 {
            return block_66(state, tracer, fn_state);
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
        // S s_22_1: call Halted(s_22_0)
        let s_22_1: bool = Halted(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b65 b23
        if s_22_1 {
            return block_65(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#134337 <= s_23_0
        fn_state.gs_134337 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#134337:u8
        let s_24_0: bool = fn_state.gs_134337;
        // N s_24_1: branch s_24_0 b64 b25
        if s_24_0 {
            return block_64(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#134338 <= s_25_0
        fn_state.gs_134338 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#134338:u8
        let s_26_0: bool = fn_state.gs_134338;
        // N s_26_1: branch s_26_0 b63 b27
        if s_26_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_27_1: write-var gs#134339 <= s_27_0
        fn_state.gs_134339 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#134339:u8
        let s_28_0: bool = fn_state.gs_134339;
        // N s_28_1: branch s_28_0 b62 b29
        if s_28_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#134340 <= s_29_0
        fn_state.gs_134340 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#134340:u8
        let s_30_0: bool = fn_state.gs_134340;
        // N s_30_1: branch s_30_0 b61 b31
        if s_30_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#134341 <= s_31_0
        fn_state.gs_134341 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#134341:u8
        let s_32_0: bool = fn_state.gs_134341;
        // N s_32_1: branch s_32_0 b60 b33
        if s_32_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #424u : u32
        let s_33_0: u32 = 424;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // C s_33_2: const #2u : u8
        let s_33_2: u8 = 2;
        // D s_33_3: cmp-lt s_33_1 s_33_2
        let s_33_3: bool = ((s_33_1) < (s_33_2));
        // N s_33_4: branch s_33_3 b59 b34
        if s_33_3 {
            return block_59(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#134342 <= s_34_0
        fn_state.gs_134342 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#134342:u8
        let s_35_0: bool = fn_state.gs_134342;
        // N s_35_1: branch s_35_0 b58 b36
        if s_35_0 {
            return block_58(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#134343 <= s_36_0
        fn_state.gs_134343 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#134343:u8
        let s_37_0: bool = fn_state.gs_134343;
        // N s_37_1: branch s_37_0 b52 b38
        if s_37_0 {
            return block_52(state, tracer, fn_state);
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
        // N s_38_4: branch s_38_3 b51 b39
        if s_38_3 {
            return block_51(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#134344 <= s_39_0
        fn_state.gs_134344 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#134344:u8
        let s_40_0: bool = fn_state.gs_134344;
        // N s_40_1: branch s_40_0 b50 b41
        if s_40_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#134345 <= s_41_0
        fn_state.gs_134345 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#134345:u8
        let s_42_0: bool = fn_state.gs_134345;
        // N s_42_1: branch s_42_0 b44 b43
        if s_42_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var t:i
        let s_43_0: i128 = fn_state.t;
        // D s_43_1: call R_read(s_43_0)
        let s_43_1: u32 = R_read(state, tracer, s_43_0);
        // D s_43_2: call Mk_TRFCR_Type(s_43_1)
        let s_43_2: ProductType700c18a878c5601b = Mk_TRFCR_Type(state, tracer, s_43_1);
        // D s_43_3: call TRFCR_write(s_43_2)
        let s_43_3: () = TRFCR_write(state, tracer, s_43_2);
        // N s_43_4: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call Halted(s_44_0)
        let s_44_1: bool = Halted(state, tracer, s_44_0);
        // N s_44_2: branch s_44_1 b49 b45
        if s_44_1 {
            return block_49(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#134346 <= s_45_0
        fn_state.gs_134346 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#134346:u8
        let s_46_0: bool = fn_state.gs_134346;
        // N s_46_1: branch s_46_0 b48 b47
        if s_46_0 {
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
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call AArch32_TakeMonitorTrapException(s_47_0)
        let s_47_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_47_0);
        // N s_47_2: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: panic
        panic!("{:?}", ());
        // N s_48_1: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call EDSCR_read(s_49_0)
        let s_49_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_49_0);
        // S s_49_2: call _get_EDSCR_Type_SDD(s_49_1)
        let s_49_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_49_1);
        // S s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // C s_49_4: const #1u : u8
        let s_49_4: bool = true;
        // C s_49_5: cast zx s_49_4 -> bv
        let s_49_5: Bits = Bits::new(s_49_4 as u128, 1u16);
        // S s_49_6: cmp-eq s_49_3 s_49_5
        let s_49_6: bool = ((s_49_3) == (s_49_5));
        // D s_49_7: write-var gs#134346 <= s_49_6
        fn_state.gs_134346 = s_49_6;
        // N s_49_8: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var __SDCR_TTRF:u8
        let s_50_0: bool = fn_state.u__SDCR_TTRF;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 1u16);
        // C s_50_2: const #1u : u8
        let s_50_2: bool = true;
        // C s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // D s_50_4: cmp-eq s_50_1 s_50_3
        let s_50_4: bool = ((s_50_1) == (s_50_3));
        // D s_50_5: write-var gs#134345 <= s_50_4
        fn_state.gs_134345 = s_50_4;
        // N s_50_6: jump b42
        return block_42(state, tracer, fn_state);
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
        // D s_51_3: write-var gs#134344 <= s_51_2
        fn_state.gs_134344 = s_51_2;
        // N s_51_4: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call Halted(s_52_0)
        let s_52_1: bool = Halted(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b57 b53
        if s_52_1 {
            return block_57(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#134347 <= s_53_0
        fn_state.gs_134347 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#134347:u8
        let s_54_0: bool = fn_state.gs_134347;
        // N s_54_1: branch s_54_0 b56 b55
        if s_54_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
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
        // C s_55_5: const #424u : u32
        let s_55_5: u32 = 424;
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
        // N s_56_0: panic
        panic!("{:?}", ());
        // N s_56_1: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call EDSCR_read(s_57_0)
        let s_57_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_57_0);
        // S s_57_2: call _get_EDSCR_Type_SDD(s_57_1)
        let s_57_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_57_1);
        // S s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // C s_57_4: const #1u : u8
        let s_57_4: bool = true;
        // C s_57_5: cast zx s_57_4 -> bv
        let s_57_5: Bits = Bits::new(s_57_4 as u128, 1u16);
        // S s_57_6: cmp-eq s_57_3 s_57_5
        let s_57_6: bool = ((s_57_3) == (s_57_5));
        // D s_57_7: write-var gs#134347 <= s_57_6
        fn_state.gs_134347 = s_57_6;
        // N s_57_8: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var __MDCR_EL3_TTRF:u8
        let s_58_0: bool = fn_state.u__MDCR_EL3_TTRF;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 1u16);
        // C s_58_2: const #1u : u8
        let s_58_2: bool = true;
        // C s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_4: cmp-eq s_58_1 s_58_3
        let s_58_4: bool = ((s_58_1) == (s_58_3));
        // D s_58_5: write-var gs#134343 <= s_58_4
        fn_state.gs_134343 = s_58_4;
        // N s_58_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #424u : u32
        let s_59_0: u32 = 424;
        // D s_59_1: read-reg s_59_0:u8
        let s_59_1: u8 = {
            let value = state.read_register::<u8>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call ELUsingAArch32(s_59_1)
        let s_59_2: bool = ELUsingAArch32(state, tracer, s_59_1);
        // D s_59_3: not s_59_2
        let s_59_3: bool = !s_59_2;
        // D s_59_4: write-var gs#134342 <= s_59_3
        fn_state.gs_134342 = s_59_3;
        // N s_59_5: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_60_0: panic
        panic!("{:?}", ());
        // N s_60_1: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var __SDCR_TTRF:u8
        let s_61_0: bool = fn_state.u__SDCR_TTRF;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 1u16);
        // C s_61_2: const #1u : u8
        let s_61_2: bool = true;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 1u16);
        // D s_61_4: cmp-eq s_61_1 s_61_3
        let s_61_4: bool = ((s_61_1) == (s_61_3));
        // D s_61_5: write-var gs#134341 <= s_61_4
        fn_state.gs_134341 = s_61_4;
        // N s_61_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #424u : u32
        let s_62_0: u32 = 424;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call ELUsingAArch32(s_62_1)
        let s_62_2: bool = ELUsingAArch32(state, tracer, s_62_1);
        // D s_62_3: write-var gs#134340 <= s_62_2
        fn_state.gs_134340 = s_62_2;
        // N s_62_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_63_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_63_1: call __IMPDEF_boolean(s_63_0)
        let s_63_1: bool = u__IMPDEF_boolean(state, tracer, s_63_0);
        // D s_63_2: write-var gs#134339 <= s_63_1
        fn_state.gs_134339 = s_63_1;
        // N s_63_3: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call EDSCR_read(s_64_0)
        let s_64_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_64_0);
        // S s_64_2: call _get_EDSCR_Type_SDD(s_64_1)
        let s_64_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_64_1);
        // S s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // C s_64_4: const #1u : u8
        let s_64_4: bool = true;
        // C s_64_5: cast zx s_64_4 -> bv
        let s_64_5: Bits = Bits::new(s_64_4 as u128, 1u16);
        // S s_64_6: cmp-eq s_64_3 s_64_5
        let s_64_6: bool = ((s_64_3) == (s_64_5));
        // D s_64_7: write-var gs#134338 <= s_64_6
        fn_state.gs_134338 = s_64_6;
        // N s_64_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #424u : u32
        let s_65_0: u32 = 424;
        // D s_65_1: read-reg s_65_0:u8
        let s_65_1: u8 = {
            let value = state.read_register::<u8>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // C s_65_2: const #2u : u8
        let s_65_2: u8 = 2;
        // D s_65_3: cmp-lt s_65_1 s_65_2
        let s_65_3: bool = ((s_65_1) < (s_65_2));
        // D s_65_4: write-var gs#134337 <= s_65_3
        fn_state.gs_134337 = s_65_3;
        // N s_65_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_66_0: panic
        panic!("{:?}", ());
        // N s_66_1: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __MDCR_EL3_TTRF:u8
        let s_67_0: bool = fn_state.u__MDCR_EL3_TTRF;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#134336 <= s_67_4
        fn_state.gs_134336 = s_67_4;
        // N s_67_6: jump b21
        return block_21(state, tracer, fn_state);
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
        // D s_68_2: call ELUsingAArch32(s_68_1)
        let s_68_2: bool = ELUsingAArch32(state, tracer, s_68_1);
        // D s_68_3: not s_68_2
        let s_68_3: bool = !s_68_2;
        // D s_68_4: write-var gs#134335 <= s_68_3
        fn_state.gs_134335 = s_68_3;
        // N s_68_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_69_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_69_1: call __IMPDEF_boolean(s_69_0)
        let s_69_1: bool = u__IMPDEF_boolean(state, tracer, s_69_0);
        // D s_69_2: write-var gs#134334 <= s_69_1
        fn_state.gs_134334 = s_69_1;
        // N s_69_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call EDSCR_read(s_70_0)
        let s_70_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_70_0);
        // S s_70_2: call _get_EDSCR_Type_SDD(s_70_1)
        let s_70_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_70_1);
        // S s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // C s_70_4: const #1u : u8
        let s_70_4: bool = true;
        // C s_70_5: cast zx s_70_4 -> bv
        let s_70_5: Bits = Bits::new(s_70_4 as u128, 1u16);
        // S s_70_6: cmp-eq s_70_3 s_70_5
        let s_70_6: bool = ((s_70_3) == (s_70_5));
        // D s_70_7: write-var gs#134333 <= s_70_6
        fn_state.gs_134333 = s_70_6;
        // N s_70_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #424u : u32
        let s_71_0: u32 = 424;
        // D s_71_1: read-reg s_71_0:u8
        let s_71_1: u8 = {
            let value = state.read_register::<u8>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // C s_71_2: const #2u : u8
        let s_71_2: u8 = 2;
        // D s_71_3: cmp-lt s_71_1 s_71_2
        let s_71_3: bool = ((s_71_1) < (s_71_2));
        // D s_71_4: write-var gs#134332 <= s_71_3
        fn_state.gs_134332 = s_71_3;
        // N s_71_5: jump b13
        return block_13(state, tracer, fn_state);
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
        // N s_72_2: branch s_72_1 b170 b73
        if s_72_1 {
            return block_170(state, tracer, fn_state);
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
        // D s_73_1: write-var gs#134348 <= s_73_0
        fn_state.gs_134348 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#134348:u8
        let s_74_0: bool = fn_state.gs_134348;
        // N s_74_1: branch s_74_0 b169 b75
        if s_74_0 {
            return block_169(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#134349 <= s_75_0
        fn_state.gs_134349 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#134349:u8
        let s_76_0: bool = fn_state.gs_134349;
        // N s_76_1: branch s_76_0 b168 b77
        if s_76_0 {
            return block_168(state, tracer, fn_state);
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
        // D s_77_1: write-var gs#134350 <= s_77_0
        fn_state.gs_134350 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#134350:u8
        let s_78_0: bool = fn_state.gs_134350;
        // N s_78_1: branch s_78_0 b167 b79
        if s_78_0 {
            return block_167(state, tracer, fn_state);
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
        // D s_79_1: write-var gs#134351 <= s_79_0
        fn_state.gs_134351 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#134351:u8
        let s_80_0: bool = fn_state.gs_134351;
        // N s_80_1: branch s_80_0 b166 b81
        if s_80_0 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#134352 <= s_81_0
        fn_state.gs_134352 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#134352:u8
        let s_82_0: bool = fn_state.gs_134352;
        // N s_82_1: branch s_82_0 b165 b83
        if s_82_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call Halted(s_83_0)
        let s_83_1: bool = Halted(state, tracer, s_83_0);
        // N s_83_2: branch s_83_1 b164 b84
        if s_83_1 {
            return block_164(state, tracer, fn_state);
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
        // D s_84_1: write-var gs#134353 <= s_84_0
        fn_state.gs_134353 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#134353:u8
        let s_85_0: bool = fn_state.gs_134353;
        // N s_85_1: branch s_85_0 b163 b86
        if s_85_0 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#134354 <= s_86_0
        fn_state.gs_134354 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#134354:u8
        let s_87_0: bool = fn_state.gs_134354;
        // N s_87_1: branch s_87_0 b162 b88
        if s_87_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#134355 <= s_88_0
        fn_state.gs_134355 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#134355:u8
        let s_89_0: bool = fn_state.gs_134355;
        // N s_89_1: branch s_89_0 b161 b90
        if s_89_0 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#134356 <= s_90_0
        fn_state.gs_134356 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#134356:u8
        let s_91_0: bool = fn_state.gs_134356;
        // N s_91_1: branch s_91_0 b160 b92
        if s_91_0 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#134357 <= s_92_0
        fn_state.gs_134357 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#134357:u8
        let s_93_0: bool = fn_state.gs_134357;
        // N s_93_1: branch s_93_0 b159 b94
        if s_93_0 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#134358 <= s_94_0
        fn_state.gs_134358 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#134358:u8
        let s_95_0: bool = fn_state.gs_134358;
        // N s_95_1: branch s_95_0 b158 b96
        if s_95_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #() : ()
        let s_96_0: () = ();
        // S s_96_1: call EL2Enabled(s_96_0)
        let s_96_1: bool = EL2Enabled(state, tracer, s_96_0);
        // N s_96_2: branch s_96_1 b157 b97
        if s_96_1 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#134359 <= s_97_0
        fn_state.gs_134359 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#134359:u8
        let s_98_0: bool = fn_state.gs_134359;
        // N s_98_1: branch s_98_0 b156 b99
        if s_98_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#134360 <= s_99_0
        fn_state.gs_134360 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#134360:u8
        let s_100_0: bool = fn_state.gs_134360;
        // N s_100_1: branch s_100_0 b155 b101
        if s_100_0 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #() : ()
        let s_101_0: () = ();
        // S s_101_1: call EL2Enabled(s_101_0)
        let s_101_1: bool = EL2Enabled(state, tracer, s_101_0);
        // N s_101_2: branch s_101_1 b154 b102
        if s_101_1 {
            return block_154(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#134361 <= s_102_0
        fn_state.gs_134361 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#134361:u8
        let s_103_0: bool = fn_state.gs_134361;
        // N s_103_1: branch s_103_0 b153 b104
        if s_103_0 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #0u : u8
        let s_104_0: bool = false;
        // D s_104_1: write-var gs#134362 <= s_104_0
        fn_state.gs_134362 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#134362:u8
        let s_105_0: bool = fn_state.gs_134362;
        // N s_105_1: branch s_105_0 b152 b106
        if s_105_0 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
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
        // N s_106_2: branch s_106_1 b151 b107
        if s_106_1 {
            return block_151(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#134363 <= s_107_0
        fn_state.gs_134363 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#134363:u8
        let s_108_0: bool = fn_state.gs_134363;
        // N s_108_1: branch s_108_0 b150 b109
        if s_108_0 {
            return block_150(state, tracer, fn_state);
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
        // D s_109_1: write-var gs#134364 <= s_109_0
        fn_state.gs_134364 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#134364:u8
        let s_110_0: bool = fn_state.gs_134364;
        // N s_110_1: branch s_110_0 b149 b111
        if s_110_0 {
            return block_149(state, tracer, fn_state);
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
        // N s_111_2: branch s_111_1 b148 b112
        if s_111_1 {
            return block_148(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#134365 <= s_112_0
        fn_state.gs_134365 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#134365:u8
        let s_113_0: bool = fn_state.gs_134365;
        // N s_113_1: branch s_113_0 b147 b114
        if s_113_0 {
            return block_147(state, tracer, fn_state);
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
        // D s_114_1: write-var gs#134366 <= s_114_0
        fn_state.gs_134366 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#134366:u8
        let s_115_0: bool = fn_state.gs_134366;
        // N s_115_1: branch s_115_0 b146 b116
        if s_115_0 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #424u : u32
        let s_116_0: u32 = 424;
        // D s_116_1: read-reg s_116_0:u8
        let s_116_1: u8 = {
            let value = state.read_register::<u8>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // C s_116_2: const #2u : u8
        let s_116_2: u8 = 2;
        // D s_116_3: cmp-lt s_116_1 s_116_2
        let s_116_3: bool = ((s_116_1) < (s_116_2));
        // N s_116_4: branch s_116_3 b145 b117
        if s_116_3 {
            return block_145(state, tracer, fn_state);
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
        // D s_117_1: write-var gs#134367 <= s_117_0
        fn_state.gs_134367 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#134367:u8
        let s_118_0: bool = fn_state.gs_134367;
        // N s_118_1: branch s_118_0 b144 b119
        if s_118_0 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#134368 <= s_119_0
        fn_state.gs_134368 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#134368:u8
        let s_120_0: bool = fn_state.gs_134368;
        // N s_120_1: branch s_120_0 b138 b121
        if s_120_0 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #424u : u32
        let s_121_0: u32 = 424;
        // D s_121_1: read-reg s_121_0:u8
        let s_121_1: u8 = {
            let value = state.read_register::<u8>(s_121_0 as isize);
            tracer.read_register(s_121_0 as isize, value);
            value
        };
        // C s_121_2: const #2u : u8
        let s_121_2: u8 = 2;
        // D s_121_3: cmp-lt s_121_1 s_121_2
        let s_121_3: bool = ((s_121_1) < (s_121_2));
        // N s_121_4: branch s_121_3 b137 b122
        if s_121_3 {
            return block_137(state, tracer, fn_state);
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
        // D s_122_1: write-var gs#134369 <= s_122_0
        fn_state.gs_134369 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#134369:u8
        let s_123_0: bool = fn_state.gs_134369;
        // N s_123_1: branch s_123_0 b136 b124
        if s_123_0 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #0u : u8
        let s_124_0: bool = false;
        // D s_124_1: write-var gs#134370 <= s_124_0
        fn_state.gs_134370 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#134370:u8
        let s_125_0: bool = fn_state.gs_134370;
        // N s_125_1: branch s_125_0 b135 b126
        if s_125_0 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #0u : u8
        let s_126_0: bool = false;
        // D s_126_1: write-var gs#134371 <= s_126_0
        fn_state.gs_134371 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#134371:u8
        let s_127_0: bool = fn_state.gs_134371;
        // N s_127_1: branch s_127_0 b129 b128
        if s_127_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var t:i
        let s_128_0: i128 = fn_state.t;
        // D s_128_1: call R_read(s_128_0)
        let s_128_1: u32 = R_read(state, tracer, s_128_0);
        // D s_128_2: call Mk_TRFCR_Type(s_128_1)
        let s_128_2: ProductType700c18a878c5601b = Mk_TRFCR_Type(state, tracer, s_128_1);
        // D s_128_3: call TRFCR_write(s_128_2)
        let s_128_3: () = TRFCR_write(state, tracer, s_128_2);
        // N s_128_4: return
        return;
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #() : ()
        let s_129_0: () = ();
        // S s_129_1: call Halted(s_129_0)
        let s_129_1: bool = Halted(state, tracer, s_129_0);
        // N s_129_2: branch s_129_1 b134 b130
        if s_129_1 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #0u : u8
        let s_130_0: bool = false;
        // D s_130_1: write-var gs#134372 <= s_130_0
        fn_state.gs_134372 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#134372:u8
        let s_131_0: bool = fn_state.gs_134372;
        // N s_131_1: branch s_131_0 b133 b132
        if s_131_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #() : ()
        let s_132_0: () = ();
        // S s_132_1: call AArch32_TakeMonitorTrapException(s_132_0)
        let s_132_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_132_0);
        // N s_132_2: return
        return;
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_133_0: panic
        panic!("{:?}", ());
        // N s_133_1: return
        return;
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #() : ()
        let s_134_0: () = ();
        // S s_134_1: call EDSCR_read(s_134_0)
        let s_134_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_134_0);
        // S s_134_2: call _get_EDSCR_Type_SDD(s_134_1)
        let s_134_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_134_1);
        // S s_134_3: cast zx s_134_2 -> bv
        let s_134_3: Bits = Bits::new(s_134_2 as u128, 1u16);
        // C s_134_4: const #1u : u8
        let s_134_4: bool = true;
        // C s_134_5: cast zx s_134_4 -> bv
        let s_134_5: Bits = Bits::new(s_134_4 as u128, 1u16);
        // S s_134_6: cmp-eq s_134_3 s_134_5
        let s_134_6: bool = ((s_134_3) == (s_134_5));
        // D s_134_7: write-var gs#134372 <= s_134_6
        fn_state.gs_134372 = s_134_6;
        // N s_134_8: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var __SDCR_TTRF:u8
        let s_135_0: bool = fn_state.u__SDCR_TTRF;
        // D s_135_1: cast zx s_135_0 -> bv
        let s_135_1: Bits = Bits::new(s_135_0 as u128, 1u16);
        // C s_135_2: const #1u : u8
        let s_135_2: bool = true;
        // C s_135_3: cast zx s_135_2 -> bv
        let s_135_3: Bits = Bits::new(s_135_2 as u128, 1u16);
        // D s_135_4: cmp-eq s_135_1 s_135_3
        let s_135_4: bool = ((s_135_1) == (s_135_3));
        // D s_135_5: write-var gs#134371 <= s_135_4
        fn_state.gs_134371 = s_135_4;
        // N s_135_6: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var __PSTATE_M:u8
        let s_136_0: u8 = fn_state.u__PSTATE_M;
        // D s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 5u16);
        // C s_136_2: const #384u : u32
        let s_136_2: u32 = 384;
        // D s_136_3: read-reg s_136_2:u8
        let s_136_3: u8 = {
            let value = state.read_register::<u8>(s_136_2 as isize);
            tracer.read_register(s_136_2 as isize, value);
            value
        };
        // D s_136_4: cast zx s_136_3 -> bv
        let s_136_4: Bits = Bits::new(s_136_3 as u128, 5u16);
        // D s_136_5: cmp-ne s_136_1 s_136_4
        let s_136_5: bool = ((s_136_1) != (s_136_4));
        // D s_136_6: write-var gs#134370 <= s_136_5
        fn_state.gs_134370 = s_136_5;
        // N s_136_7: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #424u : u32
        let s_137_0: u32 = 424;
        // D s_137_1: read-reg s_137_0:u8
        let s_137_1: u8 = {
            let value = state.read_register::<u8>(s_137_0 as isize);
            tracer.read_register(s_137_0 as isize, value);
            value
        };
        // D s_137_2: call ELUsingAArch32(s_137_1)
        let s_137_2: bool = ELUsingAArch32(state, tracer, s_137_1);
        // D s_137_3: write-var gs#134369 <= s_137_2
        fn_state.gs_134369 = s_137_2;
        // N s_137_4: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #() : ()
        let s_138_0: () = ();
        // S s_138_1: call Halted(s_138_0)
        let s_138_1: bool = Halted(state, tracer, s_138_0);
        // N s_138_2: branch s_138_1 b143 b139
        if s_138_1 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #0u : u8
        let s_139_0: bool = false;
        // D s_139_1: write-var gs#134373 <= s_139_0
        fn_state.gs_134373 = s_139_0;
        // N s_139_2: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var gs#134373:u8
        let s_140_0: bool = fn_state.gs_134373;
        // N s_140_1: branch s_140_0 b142 b141
        if s_140_0 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #3u : u8
        let s_141_0: u8 = 3;
        // C s_141_1: cast zx s_141_0 -> bv
        let s_141_1: Bits = Bits::new(s_141_0 as u128, 8u16);
        // C s_141_2: cast zx s_141_1 -> i
        let s_141_2: i128 = (s_141_1.value() as i128);
        // C s_141_3: cast reint s_141_2 -> i64
        let s_141_3: i64 = (s_141_2 as i64);
        // C s_141_4: cast zx s_141_3 -> i
        let s_141_4: i128 = (i128::try_from(s_141_3).unwrap());
        // C s_141_5: const #424u : u32
        let s_141_5: u32 = 424;
        // D s_141_6: read-reg s_141_5:u8
        let s_141_6: u8 = {
            let value = state.read_register::<u8>(s_141_5 as isize);
            tracer.read_register(s_141_5 as isize, value);
            value
        };
        // D s_141_7: call AArch64_AArch32SystemAccessTrap(s_141_6, s_141_4)
        let s_141_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_141_6,
            s_141_4,
        );
        // N s_141_8: return
        return;
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_142_0: panic
        panic!("{:?}", ());
        // N s_142_1: return
        return;
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #() : ()
        let s_143_0: () = ();
        // S s_143_1: call EDSCR_read(s_143_0)
        let s_143_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_143_0);
        // S s_143_2: call _get_EDSCR_Type_SDD(s_143_1)
        let s_143_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_143_1);
        // S s_143_3: cast zx s_143_2 -> bv
        let s_143_3: Bits = Bits::new(s_143_2 as u128, 1u16);
        // C s_143_4: const #1u : u8
        let s_143_4: bool = true;
        // C s_143_5: cast zx s_143_4 -> bv
        let s_143_5: Bits = Bits::new(s_143_4 as u128, 1u16);
        // S s_143_6: cmp-eq s_143_3 s_143_5
        let s_143_6: bool = ((s_143_3) == (s_143_5));
        // D s_143_7: write-var gs#134373 <= s_143_6
        fn_state.gs_134373 = s_143_6;
        // N s_143_8: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var __MDCR_EL3_TTRF:u8
        let s_144_0: bool = fn_state.u__MDCR_EL3_TTRF;
        // D s_144_1: cast zx s_144_0 -> bv
        let s_144_1: Bits = Bits::new(s_144_0 as u128, 1u16);
        // C s_144_2: const #1u : u8
        let s_144_2: bool = true;
        // C s_144_3: cast zx s_144_2 -> bv
        let s_144_3: Bits = Bits::new(s_144_2 as u128, 1u16);
        // D s_144_4: cmp-eq s_144_1 s_144_3
        let s_144_4: bool = ((s_144_1) == (s_144_3));
        // D s_144_5: write-var gs#134368 <= s_144_4
        fn_state.gs_134368 = s_144_4;
        // N s_144_6: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #424u : u32
        let s_145_0: u32 = 424;
        // D s_145_1: read-reg s_145_0:u8
        let s_145_1: u8 = {
            let value = state.read_register::<u8>(s_145_0 as isize);
            tracer.read_register(s_145_0 as isize, value);
            value
        };
        // D s_145_2: call ELUsingAArch32(s_145_1)
        let s_145_2: bool = ELUsingAArch32(state, tracer, s_145_1);
        // D s_145_3: not s_145_2
        let s_145_3: bool = !s_145_2;
        // D s_145_4: write-var gs#134367 <= s_145_3
        fn_state.gs_134367 = s_145_3;
        // N s_145_5: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #3u : u8
        let s_146_0: u8 = 3;
        // C s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 8u16);
        // C s_146_2: cast zx s_146_1 -> i
        let s_146_2: i128 = (s_146_1.value() as i128);
        // C s_146_3: cast reint s_146_2 -> i64
        let s_146_3: i64 = (s_146_2 as i64);
        // C s_146_4: cast zx s_146_3 -> i
        let s_146_4: i128 = (i128::try_from(s_146_3).unwrap());
        // S s_146_5: call AArch32_TakeHypTrapException(s_146_4)
        let s_146_5: () = AArch32_TakeHypTrapException(state, tracer, s_146_4);
        // N s_146_6: return
        return;
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var __HDCR_TTRF:u8
        let s_147_0: bool = fn_state.u__HDCR_TTRF;
        // D s_147_1: cast zx s_147_0 -> bv
        let s_147_1: Bits = Bits::new(s_147_0 as u128, 1u16);
        // C s_147_2: const #1u : u8
        let s_147_2: bool = true;
        // C s_147_3: cast zx s_147_2 -> bv
        let s_147_3: Bits = Bits::new(s_147_2 as u128, 1u16);
        // D s_147_4: cmp-eq s_147_1 s_147_3
        let s_147_4: bool = ((s_147_1) == (s_147_3));
        // D s_147_5: write-var gs#134366 <= s_147_4
        fn_state.gs_134366 = s_147_4;
        // N s_147_6: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #432u : u32
        let s_148_0: u32 = 432;
        // D s_148_1: read-reg s_148_0:u8
        let s_148_1: u8 = {
            let value = state.read_register::<u8>(s_148_0 as isize);
            tracer.read_register(s_148_0 as isize, value);
            value
        };
        // D s_148_2: call ELUsingAArch32(s_148_1)
        let s_148_2: bool = ELUsingAArch32(state, tracer, s_148_1);
        // D s_148_3: write-var gs#134365 <= s_148_2
        fn_state.gs_134365 = s_148_2;
        // N s_148_4: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #3u : u8
        let s_149_0: u8 = 3;
        // C s_149_1: cast zx s_149_0 -> bv
        let s_149_1: Bits = Bits::new(s_149_0 as u128, 8u16);
        // C s_149_2: cast zx s_149_1 -> i
        let s_149_2: i128 = (s_149_1.value() as i128);
        // C s_149_3: cast reint s_149_2 -> i64
        let s_149_3: i64 = (s_149_2 as i64);
        // C s_149_4: cast zx s_149_3 -> i
        let s_149_4: i128 = (i128::try_from(s_149_3).unwrap());
        // C s_149_5: const #432u : u32
        let s_149_5: u32 = 432;
        // D s_149_6: read-reg s_149_5:u8
        let s_149_6: u8 = {
            let value = state.read_register::<u8>(s_149_5 as isize);
            tracer.read_register(s_149_5 as isize, value);
            value
        };
        // D s_149_7: call AArch64_AArch32SystemAccessTrap(s_149_6, s_149_4)
        let s_149_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_149_6,
            s_149_4,
        );
        // N s_149_8: return
        return;
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var __MDCR_EL2_TTRF:u8
        let s_150_0: bool = fn_state.u__MDCR_EL2_TTRF;
        // D s_150_1: cast zx s_150_0 -> bv
        let s_150_1: Bits = Bits::new(s_150_0 as u128, 1u16);
        // C s_150_2: const #1u : u8
        let s_150_2: bool = true;
        // C s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 1u16);
        // D s_150_4: cmp-eq s_150_1 s_150_3
        let s_150_4: bool = ((s_150_1) == (s_150_3));
        // D s_150_5: write-var gs#134364 <= s_150_4
        fn_state.gs_134364 = s_150_4;
        // N s_150_6: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #432u : u32
        let s_151_0: u32 = 432;
        // D s_151_1: read-reg s_151_0:u8
        let s_151_1: u8 = {
            let value = state.read_register::<u8>(s_151_0 as isize);
            tracer.read_register(s_151_0 as isize, value);
            value
        };
        // D s_151_2: call ELUsingAArch32(s_151_1)
        let s_151_2: bool = ELUsingAArch32(state, tracer, s_151_1);
        // D s_151_3: not s_151_2
        let s_151_3: bool = !s_151_2;
        // D s_151_4: write-var gs#134363 <= s_151_3
        fn_state.gs_134363 = s_151_3;
        // N s_151_5: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #3u : u8
        let s_152_0: u8 = 3;
        // C s_152_1: cast zx s_152_0 -> bv
        let s_152_1: Bits = Bits::new(s_152_0 as u128, 8u16);
        // C s_152_2: cast zx s_152_1 -> i
        let s_152_2: i128 = (s_152_1.value() as i128);
        // C s_152_3: cast reint s_152_2 -> i64
        let s_152_3: i64 = (s_152_2 as i64);
        // C s_152_4: cast zx s_152_3 -> i
        let s_152_4: i128 = (i128::try_from(s_152_3).unwrap());
        // S s_152_5: call AArch32_TakeHypTrapException(s_152_4)
        let s_152_5: () = AArch32_TakeHypTrapException(state, tracer, s_152_4);
        // N s_152_6: return
        return;
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var __HSTR_T1:u8
        let s_153_0: bool = fn_state.u__HSTR_T1;
        // D s_153_1: cast zx s_153_0 -> bv
        let s_153_1: Bits = Bits::new(s_153_0 as u128, 1u16);
        // C s_153_2: const #1u : u8
        let s_153_2: bool = true;
        // C s_153_3: cast zx s_153_2 -> bv
        let s_153_3: Bits = Bits::new(s_153_2 as u128, 1u16);
        // D s_153_4: cmp-eq s_153_1 s_153_3
        let s_153_4: bool = ((s_153_1) == (s_153_3));
        // D s_153_5: write-var gs#134362 <= s_153_4
        fn_state.gs_134362 = s_153_4;
        // N s_153_6: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #432u : u32
        let s_154_0: u32 = 432;
        // D s_154_1: read-reg s_154_0:u8
        let s_154_1: u8 = {
            let value = state.read_register::<u8>(s_154_0 as isize);
            tracer.read_register(s_154_0 as isize, value);
            value
        };
        // D s_154_2: call ELUsingAArch32(s_154_1)
        let s_154_2: bool = ELUsingAArch32(state, tracer, s_154_1);
        // D s_154_3: write-var gs#134361 <= s_154_2
        fn_state.gs_134361 = s_154_2;
        // N s_154_4: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #3u : u8
        let s_155_0: u8 = 3;
        // C s_155_1: cast zx s_155_0 -> bv
        let s_155_1: Bits = Bits::new(s_155_0 as u128, 8u16);
        // C s_155_2: cast zx s_155_1 -> i
        let s_155_2: i128 = (s_155_1.value() as i128);
        // C s_155_3: cast reint s_155_2 -> i64
        let s_155_3: i64 = (s_155_2 as i64);
        // C s_155_4: cast zx s_155_3 -> i
        let s_155_4: i128 = (i128::try_from(s_155_3).unwrap());
        // C s_155_5: const #432u : u32
        let s_155_5: u32 = 432;
        // D s_155_6: read-reg s_155_5:u8
        let s_155_6: u8 = {
            let value = state.read_register::<u8>(s_155_5 as isize);
            tracer.read_register(s_155_5 as isize, value);
            value
        };
        // D s_155_7: call AArch64_AArch32SystemAccessTrap(s_155_6, s_155_4)
        let s_155_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_155_6,
            s_155_4,
        );
        // N s_155_8: return
        return;
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var __HSTR_EL2_T1:u8
        let s_156_0: bool = fn_state.u__HSTR_EL2_T1;
        // D s_156_1: cast zx s_156_0 -> bv
        let s_156_1: Bits = Bits::new(s_156_0 as u128, 1u16);
        // C s_156_2: const #1u : u8
        let s_156_2: bool = true;
        // C s_156_3: cast zx s_156_2 -> bv
        let s_156_3: Bits = Bits::new(s_156_2 as u128, 1u16);
        // D s_156_4: cmp-eq s_156_1 s_156_3
        let s_156_4: bool = ((s_156_1) == (s_156_3));
        // D s_156_5: write-var gs#134360 <= s_156_4
        fn_state.gs_134360 = s_156_4;
        // N s_156_6: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #432u : u32
        let s_157_0: u32 = 432;
        // D s_157_1: read-reg s_157_0:u8
        let s_157_1: u8 = {
            let value = state.read_register::<u8>(s_157_0 as isize);
            tracer.read_register(s_157_0 as isize, value);
            value
        };
        // D s_157_2: call ELUsingAArch32(s_157_1)
        let s_157_2: bool = ELUsingAArch32(state, tracer, s_157_1);
        // D s_157_3: not s_157_2
        let s_157_3: bool = !s_157_2;
        // D s_157_4: write-var gs#134359 <= s_157_3
        fn_state.gs_134359 = s_157_3;
        // N s_157_5: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_158_0: panic
        panic!("{:?}", ());
        // N s_158_1: return
        return;
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var __SDCR_TTRF:u8
        let s_159_0: bool = fn_state.u__SDCR_TTRF;
        // D s_159_1: cast zx s_159_0 -> bv
        let s_159_1: Bits = Bits::new(s_159_0 as u128, 1u16);
        // C s_159_2: const #1u : u8
        let s_159_2: bool = true;
        // C s_159_3: cast zx s_159_2 -> bv
        let s_159_3: Bits = Bits::new(s_159_2 as u128, 1u16);
        // D s_159_4: cmp-eq s_159_1 s_159_3
        let s_159_4: bool = ((s_159_1) == (s_159_3));
        // D s_159_5: write-var gs#134358 <= s_159_4
        fn_state.gs_134358 = s_159_4;
        // N s_159_6: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var __PSTATE_M:u8
        let s_160_0: u8 = fn_state.u__PSTATE_M;
        // D s_160_1: cast zx s_160_0 -> bv
        let s_160_1: Bits = Bits::new(s_160_0 as u128, 5u16);
        // C s_160_2: const #384u : u32
        let s_160_2: u32 = 384;
        // D s_160_3: read-reg s_160_2:u8
        let s_160_3: u8 = {
            let value = state.read_register::<u8>(s_160_2 as isize);
            tracer.read_register(s_160_2 as isize, value);
            value
        };
        // D s_160_4: cast zx s_160_3 -> bv
        let s_160_4: Bits = Bits::new(s_160_3 as u128, 5u16);
        // D s_160_5: cmp-ne s_160_1 s_160_4
        let s_160_5: bool = ((s_160_1) != (s_160_4));
        // D s_160_6: write-var gs#134357 <= s_160_5
        fn_state.gs_134357 = s_160_5;
        // N s_160_7: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #424u : u32
        let s_161_0: u32 = 424;
        // D s_161_1: read-reg s_161_0:u8
        let s_161_1: u8 = {
            let value = state.read_register::<u8>(s_161_0 as isize);
            tracer.read_register(s_161_0 as isize, value);
            value
        };
        // D s_161_2: call ELUsingAArch32(s_161_1)
        let s_161_2: bool = ELUsingAArch32(state, tracer, s_161_1);
        // D s_161_3: write-var gs#134356 <= s_161_2
        fn_state.gs_134356 = s_161_2;
        // N s_161_4: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_162_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_162_1: call __IMPDEF_boolean(s_162_0)
        let s_162_1: bool = u__IMPDEF_boolean(state, tracer, s_162_0);
        // D s_162_2: write-var gs#134355 <= s_162_1
        fn_state.gs_134355 = s_162_1;
        // N s_162_3: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #() : ()
        let s_163_0: () = ();
        // S s_163_1: call EDSCR_read(s_163_0)
        let s_163_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_163_0);
        // S s_163_2: call _get_EDSCR_Type_SDD(s_163_1)
        let s_163_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_163_1);
        // S s_163_3: cast zx s_163_2 -> bv
        let s_163_3: Bits = Bits::new(s_163_2 as u128, 1u16);
        // C s_163_4: const #1u : u8
        let s_163_4: bool = true;
        // C s_163_5: cast zx s_163_4 -> bv
        let s_163_5: Bits = Bits::new(s_163_4 as u128, 1u16);
        // S s_163_6: cmp-eq s_163_3 s_163_5
        let s_163_6: bool = ((s_163_3) == (s_163_5));
        // D s_163_7: write-var gs#134354 <= s_163_6
        fn_state.gs_134354 = s_163_6;
        // N s_163_8: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #424u : u32
        let s_164_0: u32 = 424;
        // D s_164_1: read-reg s_164_0:u8
        let s_164_1: u8 = {
            let value = state.read_register::<u8>(s_164_0 as isize);
            tracer.read_register(s_164_0 as isize, value);
            value
        };
        // C s_164_2: const #2u : u8
        let s_164_2: u8 = 2;
        // D s_164_3: cmp-lt s_164_1 s_164_2
        let s_164_3: bool = ((s_164_1) < (s_164_2));
        // D s_164_4: write-var gs#134353 <= s_164_3
        fn_state.gs_134353 = s_164_3;
        // N s_164_5: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_165_0: panic
        panic!("{:?}", ());
        // N s_165_1: return
        return;
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var __MDCR_EL3_TTRF:u8
        let s_166_0: bool = fn_state.u__MDCR_EL3_TTRF;
        // D s_166_1: cast zx s_166_0 -> bv
        let s_166_1: Bits = Bits::new(s_166_0 as u128, 1u16);
        // C s_166_2: const #1u : u8
        let s_166_2: bool = true;
        // C s_166_3: cast zx s_166_2 -> bv
        let s_166_3: Bits = Bits::new(s_166_2 as u128, 1u16);
        // D s_166_4: cmp-eq s_166_1 s_166_3
        let s_166_4: bool = ((s_166_1) == (s_166_3));
        // D s_166_5: write-var gs#134352 <= s_166_4
        fn_state.gs_134352 = s_166_4;
        // N s_166_6: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #424u : u32
        let s_167_0: u32 = 424;
        // D s_167_1: read-reg s_167_0:u8
        let s_167_1: u8 = {
            let value = state.read_register::<u8>(s_167_0 as isize);
            tracer.read_register(s_167_0 as isize, value);
            value
        };
        // D s_167_2: call ELUsingAArch32(s_167_1)
        let s_167_2: bool = ELUsingAArch32(state, tracer, s_167_1);
        // D s_167_3: not s_167_2
        let s_167_3: bool = !s_167_2;
        // D s_167_4: write-var gs#134351 <= s_167_3
        fn_state.gs_134351 = s_167_3;
        // N s_167_5: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_168_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_168_1: call __IMPDEF_boolean(s_168_0)
        let s_168_1: bool = u__IMPDEF_boolean(state, tracer, s_168_0);
        // D s_168_2: write-var gs#134350 <= s_168_1
        fn_state.gs_134350 = s_168_1;
        // N s_168_3: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #() : ()
        let s_169_0: () = ();
        // S s_169_1: call EDSCR_read(s_169_0)
        let s_169_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_169_0);
        // S s_169_2: call _get_EDSCR_Type_SDD(s_169_1)
        let s_169_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_169_1);
        // S s_169_3: cast zx s_169_2 -> bv
        let s_169_3: Bits = Bits::new(s_169_2 as u128, 1u16);
        // C s_169_4: const #1u : u8
        let s_169_4: bool = true;
        // C s_169_5: cast zx s_169_4 -> bv
        let s_169_5: Bits = Bits::new(s_169_4 as u128, 1u16);
        // S s_169_6: cmp-eq s_169_3 s_169_5
        let s_169_6: bool = ((s_169_3) == (s_169_5));
        // D s_169_7: write-var gs#134349 <= s_169_6
        fn_state.gs_134349 = s_169_6;
        // N s_169_8: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #424u : u32
        let s_170_0: u32 = 424;
        // D s_170_1: read-reg s_170_0:u8
        let s_170_1: u8 = {
            let value = state.read_register::<u8>(s_170_0 as isize);
            tracer.read_register(s_170_0 as isize, value);
            value
        };
        // C s_170_2: const #2u : u8
        let s_170_2: u8 = 2;
        // D s_170_3: cmp-lt s_170_1 s_170_2
        let s_170_3: bool = ((s_170_1) < (s_170_2));
        // D s_170_4: write-var gs#134348 <= s_170_3
        fn_state.gs_134348 = s_170_3;
        // N s_170_5: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_171_0: panic
        panic!("{:?}", ());
        // N s_171_1: return
        return;
    }
}
