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
use u_get_MDCR_EL2_Type_TPM::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HCR_EL2_Type_E2H::*;
use PMUSERENR_read::*;
use HDCR_read::*;
use Halted::*;
use u_get_HSTR_Type_T9::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_MDCR_EL3_Type_TPM::*;
use u_get_HDCR_Type_TPM::*;
use u_get_HDFGRTR_EL2_Type_PMUSERENR_EL0::*;
use u__get_PMUSERENR::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HSTR_EL2_Type_T9::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn PMUSERENR_SysRegRead32_a6257daec876c857<T: Tracer>(
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
        u__HSTR_EL2_T9: bool,
        gs_117148: bool,
        gs_117156: bool,
        gs_117157: bool,
        gs_117152: bool,
        gs_117123: bool,
        u__MDCR_EL3_TPM: bool,
        ga_196399: ProductType700c18a878c5601b,
        gs_117125: bool,
        gs_117139: bool,
        gs_117133: bool,
        gs_117128: bool,
        gs_117165: bool,
        gs_117135: bool,
        gs_117168: bool,
        gs_117142: bool,
        gs_117163: bool,
        gs_117155: bool,
        gs_117164: bool,
        gs_117132: bool,
        gs_117141: bool,
        u__HDFGRTR_EL2_PMUSERENR_EL0: bool,
        gs_117146: bool,
        u__PSTATE_EL: u8,
        gs_117144: bool,
        gs_117151: bool,
        u__MDCR_EL2_TPM: bool,
        gs_117143: bool,
        gs_117160: bool,
        ga_196427: ProductType700c18a878c5601b,
        gs_117149: bool,
        gs_117161: bool,
        gs_117122: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_117162: bool,
        gs_117129: bool,
        u__HSTR_T9: bool,
        gs_117158: bool,
        gs_117131: bool,
        gs_117154: bool,
        gs_117150: bool,
        gs_117145: bool,
        ga_196357: ProductType700c18a878c5601b,
        gs_117126: bool,
        gs_117138: bool,
        u__HDCR_TPM: bool,
        gs_117159: bool,
        gs_117153: bool,
        gs_117137: bool,
        gs_117136: bool,
        gs_117130: bool,
        gs_117140: bool,
        gs_117134: bool,
        gs_117124: bool,
        ga_196423: ProductType700c18a878c5601b,
        gs_117167: bool,
        gs_117166: bool,
        gs_117147: bool,
        gs_117127: bool,
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
        // D s_0_5: call _get_MDCR_EL3_Type_TPM(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_TPM(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_TPM <= s_0_5
        fn_state.u__MDCR_EL3_TPM = s_0_5;
        // C s_0_7: const #104936u : u32
        let s_0_7: u32 = 104936;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HSTR_EL2_Type_T9(s_0_8)
        let s_0_9: bool = u_get_HSTR_EL2_Type_T9(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_EL2_T9 <= s_0_9
        fn_state.u__HSTR_EL2_T9 = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call HSTR_read(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_11);
        // S s_0_13: call _get_HSTR_Type_T9(s_0_12)
        let s_0_13: bool = u_get_HSTR_Type_T9(state, tracer, s_0_12);
        // D s_0_14: write-var __HSTR_T9 <= s_0_13
        fn_state.u__HSTR_T9 = s_0_13;
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
        // C s_0_19: const #19144u : u32
        let s_0_19: u32 = 19144;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HDFGRTR_EL2_Type_PMUSERENR_EL0(s_0_20)
        let s_0_21: bool = u_get_HDFGRTR_EL2_Type_PMUSERENR_EL0(state, tracer, s_0_20);
        // D s_0_22: write-var __HDFGRTR_EL2_PMUSERENR_EL0 <= s_0_21
        fn_state.u__HDFGRTR_EL2_PMUSERENR_EL0 = s_0_21;
        // C s_0_23: const #104880u : u32
        let s_0_23: u32 = 104880;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_MDCR_EL2_Type_TPM(s_0_24)
        let s_0_25: bool = u_get_MDCR_EL2_Type_TPM(state, tracer, s_0_24);
        // D s_0_26: write-var __MDCR_EL2_TPM <= s_0_25
        fn_state.u__MDCR_EL2_TPM = s_0_25;
        // C s_0_27: const #() : ()
        let s_0_27: () = ();
        // S s_0_28: call HDCR_read(s_0_27)
        let s_0_28: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_27);
        // S s_0_29: call _get_HDCR_Type_TPM(s_0_28)
        let s_0_29: bool = u_get_HDCR_Type_TPM(state, tracer, s_0_28);
        // D s_0_30: write-var __HDCR_TPM <= s_0_29
        fn_state.u__HDCR_TPM = s_0_29;
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
        // N s_0_37: branch s_0_36 b100 b1
        if s_0_36 {
            return block_100(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b37 b2
        if s_1_5 {
            return block_37(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call PMUSERENR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = PMUSERENR_read(state, tracer, s_5_0);
        // S s_5_2: call __get_PMUSERENR(s_5_1)
        let s_5_2: ProductType700c18a878c5601b = u__get_PMUSERENR(state, tracer, s_5_1);
        // D s_5_3: write-var ga#196427 <= s_5_2
        fn_state.ga_196427 = s_5_2;
        // D s_5_4: read-var ga#196427.0:struct
        let s_5_4: u32 = fn_state.ga_196427._0;
        // D s_5_5: read-var t:i
        let s_5_5: i128 = fn_state.t;
        // D s_5_6: call R_set(s_5_5, s_5_4)
        let s_5_6: () = R_set(state, tracer, s_5_5, s_5_4);
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
        // N s_6_2: branch s_6_1 b36 b7
        if s_6_1 {
            return block_36(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#117122 <= s_7_0
        fn_state.gs_117122 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#117122:u8
        let s_8_0: bool = fn_state.gs_117122;
        // N s_8_1: branch s_8_0 b35 b9
        if s_8_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#117123 <= s_9_0
        fn_state.gs_117123 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#117123:u8
        let s_10_0: bool = fn_state.gs_117123;
        // N s_10_1: branch s_10_0 b34 b11
        if s_10_0 {
            return block_34(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#117124 <= s_11_0
        fn_state.gs_117124 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#117124:u8
        let s_12_0: bool = fn_state.gs_117124;
        // N s_12_1: branch s_12_0 b33 b13
        if s_12_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#117125 <= s_13_0
        fn_state.gs_117125 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#117125:u8
        let s_14_0: bool = fn_state.gs_117125;
        // N s_14_1: branch s_14_0 b32 b15
        if s_14_0 {
            return block_32(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#117126 <= s_15_0
        fn_state.gs_117126 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#117126:u8
        let s_16_0: bool = fn_state.gs_117126;
        // N s_16_1: branch s_16_0 b31 b17
        if s_16_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #424u : u32
        let s_17_0: u32 = 424;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // D s_17_3: cmp-lt s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) < (s_17_2));
        // N s_17_4: branch s_17_3 b30 b18
        if s_17_3 {
            return block_30(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#117127 <= s_18_0
        fn_state.gs_117127 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#117127:u8
        let s_19_0: bool = fn_state.gs_117127;
        // N s_19_1: branch s_19_0 b29 b20
        if s_19_0 {
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
        // D s_20_1: write-var gs#117128 <= s_20_0
        fn_state.gs_117128 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#117128:u8
        let s_21_0: bool = fn_state.gs_117128;
        // N s_21_1: branch s_21_0 b23 b22
        if s_21_0 {
            return block_23(state, tracer, fn_state);
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
        // S s_22_1: call PMUSERENR_read(s_22_0)
        let s_22_1: ProductType700c18a878c5601b = PMUSERENR_read(state, tracer, s_22_0);
        // S s_22_2: call __get_PMUSERENR(s_22_1)
        let s_22_2: ProductType700c18a878c5601b = u__get_PMUSERENR(
            state,
            tracer,
            s_22_1,
        );
        // D s_22_3: write-var ga#196423 <= s_22_2
        fn_state.ga_196423 = s_22_2;
        // D s_22_4: read-var ga#196423.0:struct
        let s_22_4: u32 = fn_state.ga_196423._0;
        // D s_22_5: read-var t:i
        let s_22_5: i128 = fn_state.t;
        // D s_22_6: call R_set(s_22_5, s_22_4)
        let s_22_6: () = R_set(state, tracer, s_22_5, s_22_4);
        // N s_22_7: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call Halted(s_23_0)
        let s_23_1: bool = Halted(state, tracer, s_23_0);
        // N s_23_2: branch s_23_1 b28 b24
        if s_23_1 {
            return block_28(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#117129 <= s_24_0
        fn_state.gs_117129 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#117129:u8
        let s_25_0: bool = fn_state.gs_117129;
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
        // C s_26_0: const #3u : u8
        let s_26_0: u8 = 3;
        // C s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 8u16);
        // C s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #424u : u32
        let s_26_5: u32 = 424;
        // D s_26_6: read-reg s_26_5:u8
        let s_26_6: u8 = {
            let value = state.read_register::<u8>(s_26_5 as isize);
            tracer.read_register(s_26_5 as isize, value);
            value
        };
        // D s_26_7: call AArch64_AArch32SystemAccessTrap(s_26_6, s_26_4)
        let s_26_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_26_6, s_26_4);
        // N s_26_8: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EDSCR_read(s_28_0)
        let s_28_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_28_0);
        // S s_28_2: call _get_EDSCR_Type_SDD(s_28_1)
        let s_28_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_28_1);
        // S s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // S s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#117129 <= s_28_6
        fn_state.gs_117129 = s_28_6;
        // N s_28_8: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __MDCR_EL3_TPM:u8
        let s_29_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#117128 <= s_29_4
        fn_state.gs_117128 = s_29_4;
        // N s_29_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #424u : u32
        let s_30_0: u32 = 424;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call ELUsingAArch32(s_30_1)
        let s_30_2: bool = ELUsingAArch32(state, tracer, s_30_1);
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // D s_30_4: write-var gs#117127 <= s_30_3
        fn_state.gs_117127 = s_30_3;
        // N s_30_5: jump b19
        return block_19(state, tracer, fn_state);
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
        // D s_32_0: read-var __MDCR_EL3_TPM:u8
        let s_32_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#117126 <= s_32_4
        fn_state.gs_117126 = s_32_4;
        // N s_32_6: jump b16
        return block_16(state, tracer, fn_state);
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
        // D s_33_2: call ELUsingAArch32(s_33_1)
        let s_33_2: bool = ELUsingAArch32(state, tracer, s_33_1);
        // D s_33_3: not s_33_2
        let s_33_3: bool = !s_33_2;
        // D s_33_4: write-var gs#117125 <= s_33_3
        fn_state.gs_117125 = s_33_3;
        // N s_33_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_34_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_34_1: call __IMPDEF_boolean(s_34_0)
        let s_34_1: bool = u__IMPDEF_boolean(state, tracer, s_34_0);
        // D s_34_2: write-var gs#117124 <= s_34_1
        fn_state.gs_117124 = s_34_1;
        // N s_34_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call EDSCR_read(s_35_0)
        let s_35_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_35_0);
        // S s_35_2: call _get_EDSCR_Type_SDD(s_35_1)
        let s_35_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_35_1);
        // S s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // C s_35_4: const #1u : u8
        let s_35_4: bool = true;
        // C s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 1u16);
        // S s_35_6: cmp-eq s_35_3 s_35_5
        let s_35_6: bool = ((s_35_3) == (s_35_5));
        // D s_35_7: write-var gs#117123 <= s_35_6
        fn_state.gs_117123 = s_35_6;
        // N s_35_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #424u : u32
        let s_36_0: u32 = 424;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // C s_36_2: const #2u : u8
        let s_36_2: u8 = 2;
        // D s_36_3: cmp-lt s_36_1 s_36_2
        let s_36_3: bool = ((s_36_1) < (s_36_2));
        // D s_36_4: write-var gs#117122 <= s_36_3
        fn_state.gs_117122 = s_36_3;
        // N s_36_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call Halted(s_37_0)
        let s_37_1: bool = Halted(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b99 b38
        if s_37_1 {
            return block_99(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#117130 <= s_38_0
        fn_state.gs_117130 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#117130:u8
        let s_39_0: bool = fn_state.gs_117130;
        // N s_39_1: branch s_39_0 b98 b40
        if s_39_0 {
            return block_98(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#117131 <= s_40_0
        fn_state.gs_117131 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#117131:u8
        let s_41_0: bool = fn_state.gs_117131;
        // N s_41_1: branch s_41_0 b97 b42
        if s_41_0 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#117132 <= s_42_0
        fn_state.gs_117132 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#117132:u8
        let s_43_0: bool = fn_state.gs_117132;
        // N s_43_1: branch s_43_0 b96 b44
        if s_43_0 {
            return block_96(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#117133 <= s_44_0
        fn_state.gs_117133 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#117133:u8
        let s_45_0: bool = fn_state.gs_117133;
        // N s_45_1: branch s_45_0 b95 b46
        if s_45_0 {
            return block_95(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#117134 <= s_46_0
        fn_state.gs_117134 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#117134:u8
        let s_47_0: bool = fn_state.gs_117134;
        // N s_47_1: branch s_47_0 b94 b48
        if s_47_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EL2Enabled(s_48_0)
        let s_48_1: bool = EL2Enabled(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b93 b49
        if s_48_1 {
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
        // D s_49_1: write-var gs#117135 <= s_49_0
        fn_state.gs_117135 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#117135:u8
        let s_50_0: bool = fn_state.gs_117135;
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
        // D s_51_1: write-var gs#117136 <= s_51_0
        fn_state.gs_117136 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#117136:u8
        let s_52_0: bool = fn_state.gs_117136;
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
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EL2Enabled(s_53_0)
        let s_53_1: bool = EL2Enabled(state, tracer, s_53_0);
        // N s_53_2: branch s_53_1 b90 b54
        if s_53_1 {
            return block_90(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#117137 <= s_54_0
        fn_state.gs_117137 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#117137:u8
        let s_55_0: bool = fn_state.gs_117137;
        // N s_55_1: branch s_55_0 b89 b56
        if s_55_0 {
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
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#117138 <= s_56_0
        fn_state.gs_117138 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#117138:u8
        let s_57_0: bool = fn_state.gs_117138;
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
        // D s_59_1: write-var gs#117139 <= s_59_0
        fn_state.gs_117139 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#117139:u8
        let s_60_0: bool = fn_state.gs_117139;
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
        // D s_61_1: write-var gs#117140 <= s_61_0
        fn_state.gs_117140 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#117140:u8
        let s_62_0: bool = fn_state.gs_117140;
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
        // D s_64_1: write-var gs#117141 <= s_64_0
        fn_state.gs_117141 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#117141:u8
        let s_65_0: bool = fn_state.gs_117141;
        // N s_65_1: branch s_65_0 b83 b66
        if s_65_0 {
            return block_83(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#117142 <= s_66_0
        fn_state.gs_117142 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#117142:u8
        let s_67_0: bool = fn_state.gs_117142;
        // N s_67_1: branch s_67_0 b82 b68
        if s_67_0 {
            return block_82(state, tracer, fn_state);
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
        // N s_68_4: branch s_68_3 b81 b69
        if s_68_3 {
            return block_81(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#117143 <= s_69_0
        fn_state.gs_117143 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#117143:u8
        let s_70_0: bool = fn_state.gs_117143;
        // N s_70_1: branch s_70_0 b80 b71
        if s_70_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#117144 <= s_71_0
        fn_state.gs_117144 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#117144:u8
        let s_72_0: bool = fn_state.gs_117144;
        // N s_72_1: branch s_72_0 b74 b73
        if s_72_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call PMUSERENR_read(s_73_0)
        let s_73_1: ProductType700c18a878c5601b = PMUSERENR_read(state, tracer, s_73_0);
        // S s_73_2: call __get_PMUSERENR(s_73_1)
        let s_73_2: ProductType700c18a878c5601b = u__get_PMUSERENR(
            state,
            tracer,
            s_73_1,
        );
        // D s_73_3: write-var ga#196399 <= s_73_2
        fn_state.ga_196399 = s_73_2;
        // D s_73_4: read-var ga#196399.0:struct
        let s_73_4: u32 = fn_state.ga_196399._0;
        // D s_73_5: read-var t:i
        let s_73_5: i128 = fn_state.t;
        // D s_73_6: call R_set(s_73_5, s_73_4)
        let s_73_6: () = R_set(state, tracer, s_73_5, s_73_4);
        // N s_73_7: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #() : ()
        let s_74_0: () = ();
        // S s_74_1: call Halted(s_74_0)
        let s_74_1: bool = Halted(state, tracer, s_74_0);
        // N s_74_2: branch s_74_1 b79 b75
        if s_74_1 {
            return block_79(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#117145 <= s_75_0
        fn_state.gs_117145 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#117145:u8
        let s_76_0: bool = fn_state.gs_117145;
        // N s_76_1: branch s_76_0 b78 b77
        if s_76_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #3u : u8
        let s_77_0: u8 = 3;
        // C s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 8u16);
        // C s_77_2: cast zx s_77_1 -> i
        let s_77_2: i128 = (s_77_1.value() as i128);
        // C s_77_3: cast reint s_77_2 -> i64
        let s_77_3: i64 = (s_77_2 as i64);
        // C s_77_4: cast zx s_77_3 -> i
        let s_77_4: i128 = (i128::try_from(s_77_3).unwrap());
        // C s_77_5: const #424u : u32
        let s_77_5: u32 = 424;
        // D s_77_6: read-reg s_77_5:u8
        let s_77_6: u8 = {
            let value = state.read_register::<u8>(s_77_5 as isize);
            tracer.read_register(s_77_5 as isize, value);
            value
        };
        // D s_77_7: call AArch64_AArch32SystemAccessTrap(s_77_6, s_77_4)
        let s_77_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_77_6, s_77_4);
        // N s_77_8: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_78_0: panic
        panic!("{:?}", ());
        // N s_78_1: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call EDSCR_read(s_79_0)
        let s_79_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_79_0);
        // S s_79_2: call _get_EDSCR_Type_SDD(s_79_1)
        let s_79_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_79_1);
        // S s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // C s_79_4: const #1u : u8
        let s_79_4: bool = true;
        // C s_79_5: cast zx s_79_4 -> bv
        let s_79_5: Bits = Bits::new(s_79_4 as u128, 1u16);
        // S s_79_6: cmp-eq s_79_3 s_79_5
        let s_79_6: bool = ((s_79_3) == (s_79_5));
        // D s_79_7: write-var gs#117145 <= s_79_6
        fn_state.gs_117145 = s_79_6;
        // N s_79_8: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __MDCR_EL3_TPM:u8
        let s_80_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#117144 <= s_80_4
        fn_state.gs_117144 = s_80_4;
        // N s_80_6: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #424u : u32
        let s_81_0: u32 = 424;
        // D s_81_1: read-reg s_81_0:u8
        let s_81_1: u8 = {
            let value = state.read_register::<u8>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call ELUsingAArch32(s_81_1)
        let s_81_2: bool = ELUsingAArch32(state, tracer, s_81_1);
        // D s_81_3: not s_81_2
        let s_81_3: bool = !s_81_2;
        // D s_81_4: write-var gs#117143 <= s_81_3
        fn_state.gs_117143 = s_81_3;
        // N s_81_5: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #3u : u8
        let s_82_0: u8 = 3;
        // C s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 8u16);
        // C s_82_2: cast zx s_82_1 -> i
        let s_82_2: i128 = (s_82_1.value() as i128);
        // C s_82_3: cast reint s_82_2 -> i64
        let s_82_3: i64 = (s_82_2 as i64);
        // C s_82_4: cast zx s_82_3 -> i
        let s_82_4: i128 = (i128::try_from(s_82_3).unwrap());
        // S s_82_5: call AArch32_TakeHypTrapException(s_82_4)
        let s_82_5: () = AArch32_TakeHypTrapException(state, tracer, s_82_4);
        // N s_82_6: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var __HDCR_TPM:u8
        let s_83_0: bool = fn_state.u__HDCR_TPM;
        // D s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 1u16);
        // C s_83_2: const #1u : u8
        let s_83_2: bool = true;
        // C s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 1u16);
        // D s_83_4: cmp-eq s_83_1 s_83_3
        let s_83_4: bool = ((s_83_1) == (s_83_3));
        // D s_83_5: write-var gs#117142 <= s_83_4
        fn_state.gs_117142 = s_83_4;
        // N s_83_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #432u : u32
        let s_84_0: u32 = 432;
        // D s_84_1: read-reg s_84_0:u8
        let s_84_1: u8 = {
            let value = state.read_register::<u8>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call ELUsingAArch32(s_84_1)
        let s_84_2: bool = ELUsingAArch32(state, tracer, s_84_1);
        // D s_84_3: write-var gs#117141 <= s_84_2
        fn_state.gs_117141 = s_84_2;
        // N s_84_4: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #3u : u8
        let s_85_0: u8 = 3;
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
        // D s_85_7: call AArch64_AArch32SystemAccessTrap(s_85_6, s_85_4)
        let s_85_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_85_6, s_85_4);
        // N s_85_8: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var __MDCR_EL2_TPM:u8
        let s_86_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 1u16);
        // C s_86_2: const #1u : u8
        let s_86_2: bool = true;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: write-var gs#117140 <= s_86_4
        fn_state.gs_117140 = s_86_4;
        // N s_86_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #432u : u32
        let s_87_0: u32 = 432;
        // D s_87_1: read-reg s_87_0:u8
        let s_87_1: u8 = {
            let value = state.read_register::<u8>(s_87_0 as isize);
            tracer.read_register(s_87_0 as isize, value);
            value
        };
        // D s_87_2: call ELUsingAArch32(s_87_1)
        let s_87_2: bool = ELUsingAArch32(state, tracer, s_87_1);
        // D s_87_3: not s_87_2
        let s_87_3: bool = !s_87_2;
        // D s_87_4: write-var gs#117139 <= s_87_3
        fn_state.gs_117139 = s_87_3;
        // N s_87_5: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #3u : u8
        let s_88_0: u8 = 3;
        // C s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 8u16);
        // C s_88_2: cast zx s_88_1 -> i
        let s_88_2: i128 = (s_88_1.value() as i128);
        // C s_88_3: cast reint s_88_2 -> i64
        let s_88_3: i64 = (s_88_2 as i64);
        // C s_88_4: cast zx s_88_3 -> i
        let s_88_4: i128 = (i128::try_from(s_88_3).unwrap());
        // S s_88_5: call AArch32_TakeHypTrapException(s_88_4)
        let s_88_5: () = AArch32_TakeHypTrapException(state, tracer, s_88_4);
        // N s_88_6: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var __HSTR_T9:u8
        let s_89_0: bool = fn_state.u__HSTR_T9;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #1u : u8
        let s_89_2: bool = true;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // D s_89_5: write-var gs#117138 <= s_89_4
        fn_state.gs_117138 = s_89_4;
        // N s_89_6: jump b57
        return block_57(state, tracer, fn_state);
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
        // D s_90_3: write-var gs#117137 <= s_90_2
        fn_state.gs_117137 = s_90_2;
        // N s_90_4: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #3u : u8
        let s_91_0: u8 = 3;
        // C s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 8u16);
        // C s_91_2: cast zx s_91_1 -> i
        let s_91_2: i128 = (s_91_1.value() as i128);
        // C s_91_3: cast reint s_91_2 -> i64
        let s_91_3: i64 = (s_91_2 as i64);
        // C s_91_4: cast zx s_91_3 -> i
        let s_91_4: i128 = (i128::try_from(s_91_3).unwrap());
        // C s_91_5: const #432u : u32
        let s_91_5: u32 = 432;
        // D s_91_6: read-reg s_91_5:u8
        let s_91_6: u8 = {
            let value = state.read_register::<u8>(s_91_5 as isize);
            tracer.read_register(s_91_5 as isize, value);
            value
        };
        // D s_91_7: call AArch64_AArch32SystemAccessTrap(s_91_6, s_91_4)
        let s_91_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_91_6, s_91_4);
        // N s_91_8: return
        return;
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var __HSTR_EL2_T9:u8
        let s_92_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#117136 <= s_92_4
        fn_state.gs_117136 = s_92_4;
        // N s_92_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #432u : u32
        let s_93_0: u32 = 432;
        // D s_93_1: read-reg s_93_0:u8
        let s_93_1: u8 = {
            let value = state.read_register::<u8>(s_93_0 as isize);
            tracer.read_register(s_93_0 as isize, value);
            value
        };
        // D s_93_2: call ELUsingAArch32(s_93_1)
        let s_93_2: bool = ELUsingAArch32(state, tracer, s_93_1);
        // D s_93_3: not s_93_2
        let s_93_3: bool = !s_93_2;
        // D s_93_4: write-var gs#117135 <= s_93_3
        fn_state.gs_117135 = s_93_3;
        // N s_93_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_94_0: panic
        panic!("{:?}", ());
        // N s_94_1: return
        return;
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var __MDCR_EL3_TPM:u8
        let s_95_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 1u16);
        // C s_95_2: const #1u : u8
        let s_95_2: bool = true;
        // C s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // D s_95_4: cmp-eq s_95_1 s_95_3
        let s_95_4: bool = ((s_95_1) == (s_95_3));
        // D s_95_5: write-var gs#117134 <= s_95_4
        fn_state.gs_117134 = s_95_4;
        // N s_95_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #424u : u32
        let s_96_0: u32 = 424;
        // D s_96_1: read-reg s_96_0:u8
        let s_96_1: u8 = {
            let value = state.read_register::<u8>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // D s_96_2: call ELUsingAArch32(s_96_1)
        let s_96_2: bool = ELUsingAArch32(state, tracer, s_96_1);
        // D s_96_3: not s_96_2
        let s_96_3: bool = !s_96_2;
        // D s_96_4: write-var gs#117133 <= s_96_3
        fn_state.gs_117133 = s_96_3;
        // N s_96_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_97_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_97_1: call __IMPDEF_boolean(s_97_0)
        let s_97_1: bool = u__IMPDEF_boolean(state, tracer, s_97_0);
        // D s_97_2: write-var gs#117132 <= s_97_1
        fn_state.gs_117132 = s_97_1;
        // N s_97_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call EDSCR_read(s_98_0)
        let s_98_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_98_0);
        // S s_98_2: call _get_EDSCR_Type_SDD(s_98_1)
        let s_98_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_98_1);
        // S s_98_3: cast zx s_98_2 -> bv
        let s_98_3: Bits = Bits::new(s_98_2 as u128, 1u16);
        // C s_98_4: const #1u : u8
        let s_98_4: bool = true;
        // C s_98_5: cast zx s_98_4 -> bv
        let s_98_5: Bits = Bits::new(s_98_4 as u128, 1u16);
        // S s_98_6: cmp-eq s_98_3 s_98_5
        let s_98_6: bool = ((s_98_3) == (s_98_5));
        // D s_98_7: write-var gs#117131 <= s_98_6
        fn_state.gs_117131 = s_98_6;
        // N s_98_8: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #424u : u32
        let s_99_0: u32 = 424;
        // D s_99_1: read-reg s_99_0:u8
        let s_99_1: u8 = {
            let value = state.read_register::<u8>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // C s_99_2: const #2u : u8
        let s_99_2: u8 = 2;
        // D s_99_3: cmp-lt s_99_1 s_99_2
        let s_99_3: bool = ((s_99_1) < (s_99_2));
        // D s_99_4: write-var gs#117130 <= s_99_3
        fn_state.gs_117130 = s_99_3;
        // N s_99_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #() : ()
        let s_100_0: () = ();
        // S s_100_1: call Halted(s_100_0)
        let s_100_1: bool = Halted(state, tracer, s_100_0);
        // N s_100_2: branch s_100_1 b185 b101
        if s_100_1 {
            return block_185(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#117146 <= s_101_0
        fn_state.gs_117146 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#117146:u8
        let s_102_0: bool = fn_state.gs_117146;
        // N s_102_1: branch s_102_0 b184 b103
        if s_102_0 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#117147 <= s_103_0
        fn_state.gs_117147 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#117147:u8
        let s_104_0: bool = fn_state.gs_117147;
        // N s_104_1: branch s_104_0 b183 b105
        if s_104_0 {
            return block_183(state, tracer, fn_state);
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
        // D s_105_1: write-var gs#117148 <= s_105_0
        fn_state.gs_117148 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#117148:u8
        let s_106_0: bool = fn_state.gs_117148;
        // N s_106_1: branch s_106_0 b182 b107
        if s_106_0 {
            return block_182(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#117149 <= s_107_0
        fn_state.gs_117149 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#117149:u8
        let s_108_0: bool = fn_state.gs_117149;
        // N s_108_1: branch s_108_0 b181 b109
        if s_108_0 {
            return block_181(state, tracer, fn_state);
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
        // D s_109_1: write-var gs#117150 <= s_109_0
        fn_state.gs_117150 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#117150:u8
        let s_110_0: bool = fn_state.gs_117150;
        // N s_110_1: branch s_110_0 b180 b111
        if s_110_0 {
            return block_180(state, tracer, fn_state);
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
        // N s_111_2: branch s_111_1 b179 b112
        if s_111_1 {
            return block_179(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#117151 <= s_112_0
        fn_state.gs_117151 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#117151:u8
        let s_113_0: bool = fn_state.gs_117151;
        // N s_113_1: branch s_113_0 b178 b114
        if s_113_0 {
            return block_178(state, tracer, fn_state);
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
        // D s_114_1: write-var gs#117152 <= s_114_0
        fn_state.gs_117152 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#117152:u8
        let s_115_0: bool = fn_state.gs_117152;
        // N s_115_1: branch s_115_0 b177 b116
        if s_115_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #0u : u8
        let s_116_0: bool = false;
        // D s_116_1: write-var gs#117153 <= s_116_0
        fn_state.gs_117153 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#117153:u8
        let s_117_0: bool = fn_state.gs_117153;
        // N s_117_1: branch s_117_0 b176 b118
        if s_117_0 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call EL2Enabled(s_118_0)
        let s_118_1: bool = EL2Enabled(state, tracer, s_118_0);
        // N s_118_2: branch s_118_1 b175 b119
        if s_118_1 {
            return block_175(state, tracer, fn_state);
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
        // D s_119_1: write-var gs#117154 <= s_119_0
        fn_state.gs_117154 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#117154:u8
        let s_120_0: bool = fn_state.gs_117154;
        // N s_120_1: branch s_120_0 b174 b121
        if s_120_0 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#117155 <= s_121_0
        fn_state.gs_117155 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#117155:u8
        let s_122_0: bool = fn_state.gs_117155;
        // N s_122_1: branch s_122_0 b173 b123
        if s_122_0 {
            return block_173(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #() : ()
        let s_123_0: () = ();
        // S s_123_1: call EL2Enabled(s_123_0)
        let s_123_1: bool = EL2Enabled(state, tracer, s_123_0);
        // N s_123_2: branch s_123_1 b172 b124
        if s_123_1 {
            return block_172(state, tracer, fn_state);
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
        // D s_124_1: write-var gs#117156 <= s_124_0
        fn_state.gs_117156 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#117156:u8
        let s_125_0: bool = fn_state.gs_117156;
        // N s_125_1: branch s_125_0 b171 b126
        if s_125_0 {
            return block_171(state, tracer, fn_state);
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
        // D s_126_1: write-var gs#117157 <= s_126_0
        fn_state.gs_117157 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#117157:u8
        let s_127_0: bool = fn_state.gs_117157;
        // N s_127_1: branch s_127_0 b170 b128
        if s_127_0 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #0u : u8
        let s_128_0: bool = false;
        // D s_128_1: write-var gs#117158 <= s_128_0
        fn_state.gs_117158 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#117158:u8
        let s_129_0: bool = fn_state.gs_117158;
        // N s_129_1: branch s_129_0 b166 b130
        if s_129_0 {
            return block_166(state, tracer, fn_state);
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
        // D s_130_1: write-var gs#117160 <= s_130_0
        fn_state.gs_117160 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#117160:u8
        let s_131_0: bool = fn_state.gs_117160;
        // N s_131_1: branch s_131_0 b165 b132
        if s_131_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #0u : u8
        let s_132_0: bool = false;
        // D s_132_1: write-var gs#117161 <= s_132_0
        fn_state.gs_117161 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#117161:u8
        let s_133_0: bool = fn_state.gs_117161;
        // N s_133_1: branch s_133_0 b164 b134
        if s_133_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
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
        // N s_134_2: branch s_134_1 b163 b135
        if s_134_1 {
            return block_163(state, tracer, fn_state);
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
        // D s_135_1: write-var gs#117162 <= s_135_0
        fn_state.gs_117162 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#117162:u8
        let s_136_0: bool = fn_state.gs_117162;
        // N s_136_1: branch s_136_0 b162 b137
        if s_136_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #0u : u8
        let s_137_0: bool = false;
        // D s_137_1: write-var gs#117163 <= s_137_0
        fn_state.gs_117163 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#117163:u8
        let s_138_0: bool = fn_state.gs_117163;
        // N s_138_1: branch s_138_0 b161 b139
        if s_138_0 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #() : ()
        let s_139_0: () = ();
        // S s_139_1: call EL2Enabled(s_139_0)
        let s_139_1: bool = EL2Enabled(state, tracer, s_139_0);
        // N s_139_2: branch s_139_1 b160 b140
        if s_139_1 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #0u : u8
        let s_140_0: bool = false;
        // D s_140_1: write-var gs#117164 <= s_140_0
        fn_state.gs_117164 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#117164:u8
        let s_141_0: bool = fn_state.gs_117164;
        // N s_141_1: branch s_141_0 b159 b142
        if s_141_0 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #0u : u8
        let s_142_0: bool = false;
        // D s_142_1: write-var gs#117165 <= s_142_0
        fn_state.gs_117165 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#117165:u8
        let s_143_0: bool = fn_state.gs_117165;
        // N s_143_1: branch s_143_0 b158 b144
        if s_143_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #424u : u32
        let s_144_0: u32 = 424;
        // D s_144_1: read-reg s_144_0:u8
        let s_144_1: u8 = {
            let value = state.read_register::<u8>(s_144_0 as isize);
            tracer.read_register(s_144_0 as isize, value);
            value
        };
        // C s_144_2: const #2u : u8
        let s_144_2: u8 = 2;
        // D s_144_3: cmp-lt s_144_1 s_144_2
        let s_144_3: bool = ((s_144_1) < (s_144_2));
        // N s_144_4: branch s_144_3 b157 b145
        if s_144_3 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #0u : u8
        let s_145_0: bool = false;
        // D s_145_1: write-var gs#117166 <= s_145_0
        fn_state.gs_117166 = s_145_0;
        // N s_145_2: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var gs#117166:u8
        let s_146_0: bool = fn_state.gs_117166;
        // N s_146_1: branch s_146_0 b156 b147
        if s_146_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #0u : u8
        let s_147_0: bool = false;
        // D s_147_1: write-var gs#117167 <= s_147_0
        fn_state.gs_117167 = s_147_0;
        // N s_147_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var gs#117167:u8
        let s_148_0: bool = fn_state.gs_117167;
        // N s_148_1: branch s_148_0 b150 b149
        if s_148_0 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_149(state, tracer, fn_state);
        };
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #() : ()
        let s_149_0: () = ();
        // S s_149_1: call PMUSERENR_read(s_149_0)
        let s_149_1: ProductType700c18a878c5601b = PMUSERENR_read(
            state,
            tracer,
            s_149_0,
        );
        // S s_149_2: call __get_PMUSERENR(s_149_1)
        let s_149_2: ProductType700c18a878c5601b = u__get_PMUSERENR(
            state,
            tracer,
            s_149_1,
        );
        // D s_149_3: write-var ga#196357 <= s_149_2
        fn_state.ga_196357 = s_149_2;
        // D s_149_4: read-var ga#196357.0:struct
        let s_149_4: u32 = fn_state.ga_196357._0;
        // D s_149_5: read-var t:i
        let s_149_5: i128 = fn_state.t;
        // D s_149_6: call R_set(s_149_5, s_149_4)
        let s_149_6: () = R_set(state, tracer, s_149_5, s_149_4);
        // N s_149_7: return
        return;
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #() : ()
        let s_150_0: () = ();
        // S s_150_1: call Halted(s_150_0)
        let s_150_1: bool = Halted(state, tracer, s_150_0);
        // N s_150_2: branch s_150_1 b155 b151
        if s_150_1 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #0u : u8
        let s_151_0: bool = false;
        // D s_151_1: write-var gs#117168 <= s_151_0
        fn_state.gs_117168 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#117168:u8
        let s_152_0: bool = fn_state.gs_117168;
        // N s_152_1: branch s_152_0 b154 b153
        if s_152_0 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #3u : u8
        let s_153_0: u8 = 3;
        // C s_153_1: cast zx s_153_0 -> bv
        let s_153_1: Bits = Bits::new(s_153_0 as u128, 8u16);
        // C s_153_2: cast zx s_153_1 -> i
        let s_153_2: i128 = (s_153_1.value() as i128);
        // C s_153_3: cast reint s_153_2 -> i64
        let s_153_3: i64 = (s_153_2 as i64);
        // C s_153_4: cast zx s_153_3 -> i
        let s_153_4: i128 = (i128::try_from(s_153_3).unwrap());
        // C s_153_5: const #424u : u32
        let s_153_5: u32 = 424;
        // D s_153_6: read-reg s_153_5:u8
        let s_153_6: u8 = {
            let value = state.read_register::<u8>(s_153_5 as isize);
            tracer.read_register(s_153_5 as isize, value);
            value
        };
        // D s_153_7: call AArch64_AArch32SystemAccessTrap(s_153_6, s_153_4)
        let s_153_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_153_6,
            s_153_4,
        );
        // N s_153_8: return
        return;
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_154_0: panic
        panic!("{:?}", ());
        // N s_154_1: return
        return;
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #() : ()
        let s_155_0: () = ();
        // S s_155_1: call EDSCR_read(s_155_0)
        let s_155_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_155_0);
        // S s_155_2: call _get_EDSCR_Type_SDD(s_155_1)
        let s_155_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_155_1);
        // S s_155_3: cast zx s_155_2 -> bv
        let s_155_3: Bits = Bits::new(s_155_2 as u128, 1u16);
        // C s_155_4: const #1u : u8
        let s_155_4: bool = true;
        // C s_155_5: cast zx s_155_4 -> bv
        let s_155_5: Bits = Bits::new(s_155_4 as u128, 1u16);
        // S s_155_6: cmp-eq s_155_3 s_155_5
        let s_155_6: bool = ((s_155_3) == (s_155_5));
        // D s_155_7: write-var gs#117168 <= s_155_6
        fn_state.gs_117168 = s_155_6;
        // N s_155_8: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var __MDCR_EL3_TPM:u8
        let s_156_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_156_1: cast zx s_156_0 -> bv
        let s_156_1: Bits = Bits::new(s_156_0 as u128, 1u16);
        // C s_156_2: const #1u : u8
        let s_156_2: bool = true;
        // C s_156_3: cast zx s_156_2 -> bv
        let s_156_3: Bits = Bits::new(s_156_2 as u128, 1u16);
        // D s_156_4: cmp-eq s_156_1 s_156_3
        let s_156_4: bool = ((s_156_1) == (s_156_3));
        // D s_156_5: write-var gs#117167 <= s_156_4
        fn_state.gs_117167 = s_156_4;
        // N s_156_6: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #424u : u32
        let s_157_0: u32 = 424;
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
        // D s_157_4: write-var gs#117166 <= s_157_3
        fn_state.gs_117166 = s_157_3;
        // N s_157_5: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #3u : u8
        let s_158_0: u8 = 3;
        // C s_158_1: cast zx s_158_0 -> bv
        let s_158_1: Bits = Bits::new(s_158_0 as u128, 8u16);
        // C s_158_2: cast zx s_158_1 -> i
        let s_158_2: i128 = (s_158_1.value() as i128);
        // C s_158_3: cast reint s_158_2 -> i64
        let s_158_3: i64 = (s_158_2 as i64);
        // C s_158_4: cast zx s_158_3 -> i
        let s_158_4: i128 = (i128::try_from(s_158_3).unwrap());
        // S s_158_5: call AArch32_TakeHypTrapException(s_158_4)
        let s_158_5: () = AArch32_TakeHypTrapException(state, tracer, s_158_4);
        // N s_158_6: return
        return;
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var __HDCR_TPM:u8
        let s_159_0: bool = fn_state.u__HDCR_TPM;
        // D s_159_1: cast zx s_159_0 -> bv
        let s_159_1: Bits = Bits::new(s_159_0 as u128, 1u16);
        // C s_159_2: const #1u : u8
        let s_159_2: bool = true;
        // C s_159_3: cast zx s_159_2 -> bv
        let s_159_3: Bits = Bits::new(s_159_2 as u128, 1u16);
        // D s_159_4: cmp-eq s_159_1 s_159_3
        let s_159_4: bool = ((s_159_1) == (s_159_3));
        // D s_159_5: write-var gs#117165 <= s_159_4
        fn_state.gs_117165 = s_159_4;
        // N s_159_6: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #432u : u32
        let s_160_0: u32 = 432;
        // D s_160_1: read-reg s_160_0:u8
        let s_160_1: u8 = {
            let value = state.read_register::<u8>(s_160_0 as isize);
            tracer.read_register(s_160_0 as isize, value);
            value
        };
        // D s_160_2: call ELUsingAArch32(s_160_1)
        let s_160_2: bool = ELUsingAArch32(state, tracer, s_160_1);
        // D s_160_3: write-var gs#117164 <= s_160_2
        fn_state.gs_117164 = s_160_2;
        // N s_160_4: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #3u : u8
        let s_161_0: u8 = 3;
        // C s_161_1: cast zx s_161_0 -> bv
        let s_161_1: Bits = Bits::new(s_161_0 as u128, 8u16);
        // C s_161_2: cast zx s_161_1 -> i
        let s_161_2: i128 = (s_161_1.value() as i128);
        // C s_161_3: cast reint s_161_2 -> i64
        let s_161_3: i64 = (s_161_2 as i64);
        // C s_161_4: cast zx s_161_3 -> i
        let s_161_4: i128 = (i128::try_from(s_161_3).unwrap());
        // C s_161_5: const #432u : u32
        let s_161_5: u32 = 432;
        // D s_161_6: read-reg s_161_5:u8
        let s_161_6: u8 = {
            let value = state.read_register::<u8>(s_161_5 as isize);
            tracer.read_register(s_161_5 as isize, value);
            value
        };
        // D s_161_7: call AArch64_AArch32SystemAccessTrap(s_161_6, s_161_4)
        let s_161_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_161_6,
            s_161_4,
        );
        // N s_161_8: return
        return;
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var __MDCR_EL2_TPM:u8
        let s_162_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_162_1: cast zx s_162_0 -> bv
        let s_162_1: Bits = Bits::new(s_162_0 as u128, 1u16);
        // C s_162_2: const #1u : u8
        let s_162_2: bool = true;
        // C s_162_3: cast zx s_162_2 -> bv
        let s_162_3: Bits = Bits::new(s_162_2 as u128, 1u16);
        // D s_162_4: cmp-eq s_162_1 s_162_3
        let s_162_4: bool = ((s_162_1) == (s_162_3));
        // D s_162_5: write-var gs#117163 <= s_162_4
        fn_state.gs_117163 = s_162_4;
        // N s_162_6: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #432u : u32
        let s_163_0: u32 = 432;
        // D s_163_1: read-reg s_163_0:u8
        let s_163_1: u8 = {
            let value = state.read_register::<u8>(s_163_0 as isize);
            tracer.read_register(s_163_0 as isize, value);
            value
        };
        // D s_163_2: call ELUsingAArch32(s_163_1)
        let s_163_2: bool = ELUsingAArch32(state, tracer, s_163_1);
        // D s_163_3: not s_163_2
        let s_163_3: bool = !s_163_2;
        // D s_163_4: write-var gs#117162 <= s_163_3
        fn_state.gs_117162 = s_163_3;
        // N s_163_5: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #3u : u8
        let s_164_0: u8 = 3;
        // C s_164_1: cast zx s_164_0 -> bv
        let s_164_1: Bits = Bits::new(s_164_0 as u128, 8u16);
        // C s_164_2: cast zx s_164_1 -> i
        let s_164_2: i128 = (s_164_1.value() as i128);
        // C s_164_3: cast reint s_164_2 -> i64
        let s_164_3: i64 = (s_164_2 as i64);
        // C s_164_4: cast zx s_164_3 -> i
        let s_164_4: i128 = (i128::try_from(s_164_3).unwrap());
        // C s_164_5: const #432u : u32
        let s_164_5: u32 = 432;
        // D s_164_6: read-reg s_164_5:u8
        let s_164_6: u8 = {
            let value = state.read_register::<u8>(s_164_5 as isize);
            tracer.read_register(s_164_5 as isize, value);
            value
        };
        // D s_164_7: call AArch64_AArch32SystemAccessTrap(s_164_6, s_164_4)
        let s_164_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_164_6,
            s_164_4,
        );
        // N s_164_8: return
        return;
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var __HDFGRTR_EL2_PMUSERENR_EL0:u8
        let s_165_0: bool = fn_state.u__HDFGRTR_EL2_PMUSERENR_EL0;
        // D s_165_1: cast zx s_165_0 -> bv
        let s_165_1: Bits = Bits::new(s_165_0 as u128, 1u16);
        // C s_165_2: const #1u : u8
        let s_165_2: bool = true;
        // C s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 1u16);
        // D s_165_4: cmp-eq s_165_1 s_165_3
        let s_165_4: bool = ((s_165_1) == (s_165_3));
        // D s_165_5: write-var gs#117161 <= s_165_4
        fn_state.gs_117161 = s_165_4;
        // N s_165_6: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #424u : u32
        let s_166_0: u32 = 424;
        // D s_166_1: read-reg s_166_0:u8
        let s_166_1: u8 = {
            let value = state.read_register::<u8>(s_166_0 as isize);
            tracer.read_register(s_166_0 as isize, value);
            value
        };
        // C s_166_2: const #2u : u8
        let s_166_2: u8 = 2;
        // D s_166_3: cmp-lt s_166_1 s_166_2
        let s_166_3: bool = ((s_166_1) < (s_166_2));
        // D s_166_4: not s_166_3
        let s_166_4: bool = !s_166_3;
        // N s_166_5: branch s_166_4 b169 b167
        if s_166_4 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var __SCR_EL3_FGTEn:u8
        let s_167_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_167_1: cast zx s_167_0 -> bv
        let s_167_1: Bits = Bits::new(s_167_0 as u128, 1u16);
        // C s_167_2: const #1u : u8
        let s_167_2: bool = true;
        // C s_167_3: cast zx s_167_2 -> bv
        let s_167_3: Bits = Bits::new(s_167_2 as u128, 1u16);
        // D s_167_4: cmp-eq s_167_1 s_167_3
        let s_167_4: bool = ((s_167_1) == (s_167_3));
        // D s_167_5: write-var gs#117159 <= s_167_4
        fn_state.gs_117159 = s_167_4;
        // N s_167_6: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var gs#117159:u8
        let s_168_0: bool = fn_state.gs_117159;
        // D s_168_1: write-var gs#117160 <= s_168_0
        fn_state.gs_117160 = s_168_0;
        // N s_168_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #1u : u8
        let s_169_0: bool = true;
        // D s_169_1: write-var gs#117159 <= s_169_0
        fn_state.gs_117159 = s_169_0;
        // N s_169_2: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #146u : u32
        let s_170_0: u32 = 146;
        // S s_170_1: call IsFeatureImplemented(s_170_0)
        let s_170_1: bool = IsFeatureImplemented(state, tracer, s_170_0);
        // D s_170_2: write-var gs#117158 <= s_170_1
        fn_state.gs_117158 = s_170_1;
        // N s_170_3: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #102552u : u32
        let s_171_0: u32 = 102552;
        // D s_171_1: read-reg s_171_0:struct
        let s_171_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_171_0 as isize);
            tracer.read_register(s_171_0 as isize, value);
            value
        };
        // D s_171_2: call _get_HCR_EL2_Type_E2H(s_171_1)
        let s_171_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_171_1);
        // C s_171_3: const #102552u : u32
        let s_171_3: u32 = 102552;
        // D s_171_4: read-reg s_171_3:struct
        let s_171_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_171_3 as isize);
            tracer.read_register(s_171_3 as isize, value);
            value
        };
        // D s_171_5: call _get_HCR_EL2_Type_TGE(s_171_4)
        let s_171_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_171_4);
        // D s_171_6: cast zx s_171_2 -> bv
        let s_171_6: Bits = Bits::new(s_171_2 as u128, 1u16);
        // D s_171_7: cast zx s_171_5 -> bv
        let s_171_7: Bits = Bits::new(s_171_5 as u128, 1u16);
        // D s_171_8: cast reint s_171_6 -> u128
        let s_171_8: u128 = (s_171_6.value() as u128);
        // D s_171_9: size-of s_171_6
        let s_171_9: u16 = s_171_6.length();
        // D s_171_10: cast reint s_171_7 -> u128
        let s_171_10: u128 = (s_171_7.value() as u128);
        // D s_171_11: size-of s_171_7
        let s_171_11: u16 = s_171_7.length();
        // D s_171_12: lsl s_171_8 s_171_11
        let s_171_12: u128 = s_171_8 << s_171_11;
        // D s_171_13: or s_171_12 s_171_10
        let s_171_13: u128 = ((s_171_12) | (s_171_10));
        // D s_171_14: add s_171_9 s_171_11
        let s_171_14: u16 = (s_171_9 + s_171_11);
        // D s_171_15: create-bits s_171_13 s_171_14
        let s_171_15: Bits = Bits::new(s_171_13, s_171_14);
        // D s_171_16: cast reint s_171_15 -> u8
        let s_171_16: u8 = (s_171_15.value() as u8);
        // D s_171_17: cast zx s_171_16 -> bv
        let s_171_17: Bits = Bits::new(s_171_16 as u128, 2u16);
        // C s_171_18: const #3u : u8
        let s_171_18: u8 = 3;
        // C s_171_19: cast zx s_171_18 -> bv
        let s_171_19: Bits = Bits::new(s_171_18 as u128, 2u16);
        // D s_171_20: cmp-ne s_171_17 s_171_19
        let s_171_20: bool = ((s_171_17) != (s_171_19));
        // D s_171_21: write-var gs#117157 <= s_171_20
        fn_state.gs_117157 = s_171_20;
        // N s_171_22: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #440u : u32
        let s_172_0: u32 = 440;
        // D s_172_1: read-reg s_172_0:u8
        let s_172_1: u8 = {
            let value = state.read_register::<u8>(s_172_0 as isize);
            tracer.read_register(s_172_0 as isize, value);
            value
        };
        // D s_172_2: call ELUsingAArch32(s_172_1)
        let s_172_2: bool = ELUsingAArch32(state, tracer, s_172_1);
        // D s_172_3: not s_172_2
        let s_172_3: bool = !s_172_2;
        // D s_172_4: write-var gs#117156 <= s_172_3
        fn_state.gs_117156 = s_172_3;
        // N s_172_5: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #3u : u8
        let s_173_0: u8 = 3;
        // C s_173_1: cast zx s_173_0 -> bv
        let s_173_1: Bits = Bits::new(s_173_0 as u128, 8u16);
        // C s_173_2: cast zx s_173_1 -> i
        let s_173_2: i128 = (s_173_1.value() as i128);
        // C s_173_3: cast reint s_173_2 -> i64
        let s_173_3: i64 = (s_173_2 as i64);
        // C s_173_4: cast zx s_173_3 -> i
        let s_173_4: i128 = (i128::try_from(s_173_3).unwrap());
        // S s_173_5: call AArch32_TakeHypTrapException(s_173_4)
        let s_173_5: () = AArch32_TakeHypTrapException(state, tracer, s_173_4);
        // N s_173_6: return
        return;
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var __HSTR_T9:u8
        let s_174_0: bool = fn_state.u__HSTR_T9;
        // D s_174_1: cast zx s_174_0 -> bv
        let s_174_1: Bits = Bits::new(s_174_0 as u128, 1u16);
        // C s_174_2: const #1u : u8
        let s_174_2: bool = true;
        // C s_174_3: cast zx s_174_2 -> bv
        let s_174_3: Bits = Bits::new(s_174_2 as u128, 1u16);
        // D s_174_4: cmp-eq s_174_1 s_174_3
        let s_174_4: bool = ((s_174_1) == (s_174_3));
        // D s_174_5: write-var gs#117155 <= s_174_4
        fn_state.gs_117155 = s_174_4;
        // N s_174_6: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #432u : u32
        let s_175_0: u32 = 432;
        // D s_175_1: read-reg s_175_0:u8
        let s_175_1: u8 = {
            let value = state.read_register::<u8>(s_175_0 as isize);
            tracer.read_register(s_175_0 as isize, value);
            value
        };
        // D s_175_2: call ELUsingAArch32(s_175_1)
        let s_175_2: bool = ELUsingAArch32(state, tracer, s_175_1);
        // D s_175_3: write-var gs#117154 <= s_175_2
        fn_state.gs_117154 = s_175_2;
        // N s_175_4: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #3u : u8
        let s_176_0: u8 = 3;
        // C s_176_1: cast zx s_176_0 -> bv
        let s_176_1: Bits = Bits::new(s_176_0 as u128, 8u16);
        // C s_176_2: cast zx s_176_1 -> i
        let s_176_2: i128 = (s_176_1.value() as i128);
        // C s_176_3: cast reint s_176_2 -> i64
        let s_176_3: i64 = (s_176_2 as i64);
        // C s_176_4: cast zx s_176_3 -> i
        let s_176_4: i128 = (i128::try_from(s_176_3).unwrap());
        // C s_176_5: const #432u : u32
        let s_176_5: u32 = 432;
        // D s_176_6: read-reg s_176_5:u8
        let s_176_6: u8 = {
            let value = state.read_register::<u8>(s_176_5 as isize);
            tracer.read_register(s_176_5 as isize, value);
            value
        };
        // D s_176_7: call AArch64_AArch32SystemAccessTrap(s_176_6, s_176_4)
        let s_176_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_176_6,
            s_176_4,
        );
        // N s_176_8: return
        return;
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_177_0: read-var __HSTR_EL2_T9:u8
        let s_177_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_177_1: cast zx s_177_0 -> bv
        let s_177_1: Bits = Bits::new(s_177_0 as u128, 1u16);
        // C s_177_2: const #1u : u8
        let s_177_2: bool = true;
        // C s_177_3: cast zx s_177_2 -> bv
        let s_177_3: Bits = Bits::new(s_177_2 as u128, 1u16);
        // D s_177_4: cmp-eq s_177_1 s_177_3
        let s_177_4: bool = ((s_177_1) == (s_177_3));
        // D s_177_5: write-var gs#117153 <= s_177_4
        fn_state.gs_117153 = s_177_4;
        // N s_177_6: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #102552u : u32
        let s_178_0: u32 = 102552;
        // D s_178_1: read-reg s_178_0:struct
        let s_178_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_178_0 as isize);
            tracer.read_register(s_178_0 as isize, value);
            value
        };
        // D s_178_2: call _get_HCR_EL2_Type_E2H(s_178_1)
        let s_178_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_178_1);
        // C s_178_3: const #102552u : u32
        let s_178_3: u32 = 102552;
        // D s_178_4: read-reg s_178_3:struct
        let s_178_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_178_3 as isize);
            tracer.read_register(s_178_3 as isize, value);
            value
        };
        // D s_178_5: call _get_HCR_EL2_Type_TGE(s_178_4)
        let s_178_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_178_4);
        // D s_178_6: cast zx s_178_2 -> bv
        let s_178_6: Bits = Bits::new(s_178_2 as u128, 1u16);
        // D s_178_7: cast zx s_178_5 -> bv
        let s_178_7: Bits = Bits::new(s_178_5 as u128, 1u16);
        // D s_178_8: cast reint s_178_6 -> u128
        let s_178_8: u128 = (s_178_6.value() as u128);
        // D s_178_9: size-of s_178_6
        let s_178_9: u16 = s_178_6.length();
        // D s_178_10: cast reint s_178_7 -> u128
        let s_178_10: u128 = (s_178_7.value() as u128);
        // D s_178_11: size-of s_178_7
        let s_178_11: u16 = s_178_7.length();
        // D s_178_12: lsl s_178_8 s_178_11
        let s_178_12: u128 = s_178_8 << s_178_11;
        // D s_178_13: or s_178_12 s_178_10
        let s_178_13: u128 = ((s_178_12) | (s_178_10));
        // D s_178_14: add s_178_9 s_178_11
        let s_178_14: u16 = (s_178_9 + s_178_11);
        // D s_178_15: create-bits s_178_13 s_178_14
        let s_178_15: Bits = Bits::new(s_178_13, s_178_14);
        // D s_178_16: cast reint s_178_15 -> u8
        let s_178_16: u8 = (s_178_15.value() as u8);
        // D s_178_17: cast zx s_178_16 -> bv
        let s_178_17: Bits = Bits::new(s_178_16 as u128, 2u16);
        // C s_178_18: const #3u : u8
        let s_178_18: u8 = 3;
        // C s_178_19: cast zx s_178_18 -> bv
        let s_178_19: Bits = Bits::new(s_178_18 as u128, 2u16);
        // D s_178_20: cmp-ne s_178_17 s_178_19
        let s_178_20: bool = ((s_178_17) != (s_178_19));
        // D s_178_21: write-var gs#117152 <= s_178_20
        fn_state.gs_117152 = s_178_20;
        // N s_178_22: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #432u : u32
        let s_179_0: u32 = 432;
        // D s_179_1: read-reg s_179_0:u8
        let s_179_1: u8 = {
            let value = state.read_register::<u8>(s_179_0 as isize);
            tracer.read_register(s_179_0 as isize, value);
            value
        };
        // D s_179_2: call ELUsingAArch32(s_179_1)
        let s_179_2: bool = ELUsingAArch32(state, tracer, s_179_1);
        // D s_179_3: not s_179_2
        let s_179_3: bool = !s_179_2;
        // D s_179_4: write-var gs#117151 <= s_179_3
        fn_state.gs_117151 = s_179_3;
        // N s_179_5: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_180_0: panic
        panic!("{:?}", ());
        // N s_180_1: return
        return;
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_181_0: read-var __MDCR_EL3_TPM:u8
        let s_181_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 1u16);
        // C s_181_2: const #1u : u8
        let s_181_2: bool = true;
        // C s_181_3: cast zx s_181_2 -> bv
        let s_181_3: Bits = Bits::new(s_181_2 as u128, 1u16);
        // D s_181_4: cmp-eq s_181_1 s_181_3
        let s_181_4: bool = ((s_181_1) == (s_181_3));
        // D s_181_5: write-var gs#117150 <= s_181_4
        fn_state.gs_117150 = s_181_4;
        // N s_181_6: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #424u : u32
        let s_182_0: u32 = 424;
        // D s_182_1: read-reg s_182_0:u8
        let s_182_1: u8 = {
            let value = state.read_register::<u8>(s_182_0 as isize);
            tracer.read_register(s_182_0 as isize, value);
            value
        };
        // D s_182_2: call ELUsingAArch32(s_182_1)
        let s_182_2: bool = ELUsingAArch32(state, tracer, s_182_1);
        // D s_182_3: not s_182_2
        let s_182_3: bool = !s_182_2;
        // D s_182_4: write-var gs#117149 <= s_182_3
        fn_state.gs_117149 = s_182_3;
        // N s_182_5: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_183_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_183_1: call __IMPDEF_boolean(s_183_0)
        let s_183_1: bool = u__IMPDEF_boolean(state, tracer, s_183_0);
        // D s_183_2: write-var gs#117148 <= s_183_1
        fn_state.gs_117148 = s_183_1;
        // N s_183_3: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #() : ()
        let s_184_0: () = ();
        // S s_184_1: call EDSCR_read(s_184_0)
        let s_184_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_184_0);
        // S s_184_2: call _get_EDSCR_Type_SDD(s_184_1)
        let s_184_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_184_1);
        // S s_184_3: cast zx s_184_2 -> bv
        let s_184_3: Bits = Bits::new(s_184_2 as u128, 1u16);
        // C s_184_4: const #1u : u8
        let s_184_4: bool = true;
        // C s_184_5: cast zx s_184_4 -> bv
        let s_184_5: Bits = Bits::new(s_184_4 as u128, 1u16);
        // S s_184_6: cmp-eq s_184_3 s_184_5
        let s_184_6: bool = ((s_184_3) == (s_184_5));
        // D s_184_7: write-var gs#117147 <= s_184_6
        fn_state.gs_117147 = s_184_6;
        // N s_184_8: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #424u : u32
        let s_185_0: u32 = 424;
        // D s_185_1: read-reg s_185_0:u8
        let s_185_1: u8 = {
            let value = state.read_register::<u8>(s_185_0 as isize);
            tracer.read_register(s_185_0 as isize, value);
            value
        };
        // C s_185_2: const #2u : u8
        let s_185_2: u8 = 2;
        // D s_185_3: cmp-lt s_185_1 s_185_2
        let s_185_3: bool = ((s_185_1) < (s_185_2));
        // D s_185_4: write-var gs#117146 <= s_185_3
        fn_state.gs_117146 = s_185_3;
        // N s_185_5: jump b102
        return block_102(state, tracer, fn_state);
    }
}
