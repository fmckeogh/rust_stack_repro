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
use u_get_HDFGWTR_EL2_Type_PMBLIMITR_EL1::*;
use Mk_PMBLIMITR_EL1_Type::*;
use Halted::*;
use IsFeatureImplemented::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u_get_MDCR_EL3_Type_NSPB::*;
use u_get_SCR_EL3_Type_NS::*;
use u__IMPDEF_boolean::*;
use NVMem_set::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_SCR_EL3_Type_NSE::*;
use u_get_MDCR_EL3_Type_NSPBE::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_MDCR_EL2_Type_E2PB::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn PMBLIMITR_EL1_SysRegWrite_9f8902df06f4b415<T: Tracer>(
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
        gs_87231: bool,
        gs_87272: bool,
        gs_87254: bool,
        gs_87259: bool,
        gs_87271: bool,
        gs_87258: bool,
        u__MDCR_EL3_NSPBE: bool,
        gs_87243: bool,
        gs_87238: bool,
        gs_87246: bool,
        gs_87276: bool,
        gs_87270: bool,
        gs_87255: bool,
        u__PSTATE_EL: u8,
        gs_87269: bool,
        gs_87225: bool,
        gs_87239: bool,
        gs_87264: bool,
        gs_87233: bool,
        gs_87273: bool,
        gs_87241: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_87240: bool,
        u__HDFGWTR_EL2_PMBLIMITR_EL1: bool,
        gs_87256: bool,
        gs_87223: bool,
        gs_87224: bool,
        gs_87245: bool,
        gs_87252: bool,
        gs_87251: bool,
        gs_87230: bool,
        gs_87232: bool,
        gs_87244: bool,
        gs_87253: bool,
        u__SCR_EL3_NSE: bool,
        u__MDCR_EL2_E2PB: u8,
        gs_87257: bool,
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
        // C s_0_3: const #22712u : u32
        let s_0_3: u32 = 22712;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDCR_EL3_Type_NSPBE(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_NSPBE(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_NSPBE <= s_0_5
        fn_state.u__MDCR_EL3_NSPBE = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_NSE(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_NSE(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_NSE <= s_0_9
        fn_state.u__SCR_EL3_NSE = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_FGTEn(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_FGTEn <= s_0_13
        fn_state.u__SCR_EL3_FGTEn = s_0_13;
        // C s_0_15: const #17360u : u32
        let s_0_15: u32 = 17360;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HDFGWTR_EL2_Type_PMBLIMITR_EL1(s_0_16)
        let s_0_17: bool = u_get_HDFGWTR_EL2_Type_PMBLIMITR_EL1(state, tracer, s_0_16);
        // D s_0_18: write-var __HDFGWTR_EL2_PMBLIMITR_EL1 <= s_0_17
        fn_state.u__HDFGWTR_EL2_PMBLIMITR_EL1 = s_0_17;
        // C s_0_19: const #104880u : u32
        let s_0_19: u32 = 104880;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_MDCR_EL2_Type_E2PB(s_0_20)
        let s_0_21: u8 = u_get_MDCR_EL2_Type_E2PB(state, tracer, s_0_20);
        // D s_0_22: write-var __MDCR_EL2_E2PB <= s_0_21
        fn_state.u__MDCR_EL2_E2PB = s_0_21;
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
        // N s_0_29: branch s_0_28 b119 b1
        if s_0_28 {
            return block_119(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b49 b2
        if s_1_5 {
            return block_49(state, tracer, fn_state);
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
        // D s_5_4: call Mk_PMBLIMITR_EL1_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_PMBLIMITR_EL1_Type(
            state,
            tracer,
            s_5_3,
        );
        // C s_5_5: const #20480u : u32
        let s_5_5: u32 = 20480;
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
        // N s_6_2: branch s_6_1 b48 b7
        if s_6_1 {
            return block_48(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#87223 <= s_7_0
        fn_state.gs_87223 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#87223:u8
        let s_8_0: bool = fn_state.gs_87223;
        // N s_8_1: branch s_8_0 b47 b9
        if s_8_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#87224 <= s_9_0
        fn_state.gs_87224 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#87224:u8
        let s_10_0: bool = fn_state.gs_87224;
        // N s_10_1: branch s_10_0 b46 b11
        if s_10_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#87225 <= s_11_0
        fn_state.gs_87225 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#87225:u8
        let s_12_0: bool = fn_state.gs_87225;
        // N s_12_1: branch s_12_0 b36 b13
        if s_12_0 {
            return block_36(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#87233 <= s_13_0
        fn_state.gs_87233 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#87233:u8
        let s_14_0: bool = fn_state.gs_87233;
        // N s_14_1: branch s_14_0 b35 b15
        if s_14_0 {
            return block_35(state, tracer, fn_state);
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
        // N s_15_4: branch s_15_3 b25 b16
        if s_15_3 {
            return block_25(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#87241 <= s_16_0
        fn_state.gs_87241 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#87241:u8
        let s_17_0: bool = fn_state.gs_87241;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #64s : i64
        let s_18_0: i64 = 64;
        // D s_18_1: read-var t:i
        let s_18_1: i128 = fn_state.t;
        // D s_18_2: call X_read(s_18_1, s_18_0)
        let s_18_2: Bits = X_read(state, tracer, s_18_1, s_18_0);
        // D s_18_3: cast reint s_18_2 -> u64
        let s_18_3: u64 = (s_18_2.value() as u64);
        // D s_18_4: call Mk_PMBLIMITR_EL1_Type(s_18_3)
        let s_18_4: ProductType5c790c8ef59cc8b2 = Mk_PMBLIMITR_EL1_Type(
            state,
            tracer,
            s_18_3,
        );
        // C s_18_5: const #20480u : u32
        let s_18_5: u32 = 20480;
        // N s_18_6: write-reg s_18_5 <= s_18_4
        let s_18_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_18_5 as isize, s_18_4);
            tracer.write_register(s_18_5 as isize, s_18_4);
        };
        // N s_18_7: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call Halted(s_19_0)
        let s_19_1: bool = Halted(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b24 b20
        if s_19_1 {
            return block_24(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#87243 <= s_20_0
        fn_state.gs_87243 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#87243:u8
        let s_21_0: bool = fn_state.gs_87243;
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
        // C s_22_0: const #24u : u8
        let s_22_0: u8 = 24;
        // C s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 8u16);
        // C s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (s_22_1.value() as i128);
        // C s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #424u : u32
        let s_22_5: u32 = 424;
        // D s_22_6: read-reg s_22_5:u8
        let s_22_6: u8 = {
            let value = state.read_register::<u8>(s_22_5 as isize);
            tracer.read_register(s_22_5 as isize, value);
            value
        };
        // D s_22_7: call AArch64_SystemAccessTrap(s_22_6, s_22_4)
        let s_22_7: () = AArch64_SystemAccessTrap(state, tracer, s_22_6, s_22_4);
        // N s_22_8: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call EDSCR_read(s_24_0)
        let s_24_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_24_0);
        // S s_24_2: call _get_EDSCR_Type_SDD(s_24_1)
        let s_24_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_24_1);
        // S s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // S s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // D s_24_7: write-var gs#87243 <= s_24_6
        fn_state.gs_87243 = s_24_6;
        // N s_24_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #22712u : u32
        let s_25_0: u32 = 22712;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_MDCR_EL3_Type_NSPB(s_25_1)
        let s_25_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_25_1);
        // C s_25_3: const #0s : i
        let s_25_3: i128 = 0;
        // D s_25_4: cast zx s_25_2 -> bv
        let s_25_4: Bits = Bits::new(s_25_2 as u128, 2u16);
        // C s_25_5: const #1u : u64
        let s_25_5: u64 = 1;
        // D s_25_6: bit-extract s_25_4 s_25_3 s_25_5
        let s_25_6: Bits = (Bits::new(
            ((s_25_4) >> (s_25_3)).value(),
            u16::try_from(s_25_5).unwrap(),
        ));
        // D s_25_7: cast reint s_25_6 -> u8
        let s_25_7: bool = ((s_25_6.value()) != 0);
        // C s_25_8: const #0s : i
        let s_25_8: i128 = 0;
        // C s_25_9: const #0u : u64
        let s_25_9: u64 = 0;
        // D s_25_10: cast zx s_25_7 -> u64
        let s_25_10: u64 = (s_25_7 as u64);
        // C s_25_11: const #1u : u64
        let s_25_11: u64 = 1;
        // D s_25_12: and s_25_10 s_25_11
        let s_25_12: u64 = ((s_25_10) & (s_25_11));
        // D s_25_13: cmp-eq s_25_12 s_25_11
        let s_25_13: bool = ((s_25_12) == (s_25_11));
        // D s_25_14: lsl s_25_10 s_25_8
        let s_25_14: u64 = s_25_10 << s_25_8;
        // D s_25_15: or s_25_9 s_25_14
        let s_25_15: u64 = ((s_25_9) | (s_25_14));
        // D s_25_16: cmpl s_25_14
        let s_25_16: u64 = !s_25_14;
        // D s_25_17: and s_25_9 s_25_16
        let s_25_17: u64 = ((s_25_9) & (s_25_16));
        // D s_25_18: select s_25_13 s_25_15 s_25_17
        let s_25_18: u64 = if s_25_13 { s_25_15 } else { s_25_17 };
        // D s_25_19: cast trunc s_25_18 -> u8
        let s_25_19: bool = ((s_25_18) != 0);
        // D s_25_20: cast zx s_25_19 -> bv
        let s_25_20: Bits = Bits::new(s_25_19 as u128, 1u16);
        // C s_25_21: const #0u : u8
        let s_25_21: bool = false;
        // C s_25_22: cast zx s_25_21 -> bv
        let s_25_22: Bits = Bits::new(s_25_21 as u128, 1u16);
        // D s_25_23: cmp-eq s_25_20 s_25_22
        let s_25_23: bool = ((s_25_20) == (s_25_22));
        // N s_25_24: branch s_25_23 b34 b26
        if s_25_23 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #22712u : u32
        let s_26_0: u32 = 22712;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_MDCR_EL3_Type_NSPB(s_26_1)
        let s_26_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_26_1);
        // C s_26_3: const #1s : i
        let s_26_3: i128 = 1;
        // D s_26_4: cast zx s_26_2 -> bv
        let s_26_4: Bits = Bits::new(s_26_2 as u128, 2u16);
        // C s_26_5: const #1u : u64
        let s_26_5: u64 = 1;
        // D s_26_6: bit-extract s_26_4 s_26_3 s_26_5
        let s_26_6: Bits = (Bits::new(
            ((s_26_4) >> (s_26_3)).value(),
            u16::try_from(s_26_5).unwrap(),
        ));
        // D s_26_7: cast reint s_26_6 -> u8
        let s_26_7: bool = ((s_26_6.value()) != 0);
        // C s_26_8: const #0s : i
        let s_26_8: i128 = 0;
        // C s_26_9: const #0u : u64
        let s_26_9: u64 = 0;
        // D s_26_10: cast zx s_26_7 -> u64
        let s_26_10: u64 = (s_26_7 as u64);
        // C s_26_11: const #1u : u64
        let s_26_11: u64 = 1;
        // D s_26_12: and s_26_10 s_26_11
        let s_26_12: u64 = ((s_26_10) & (s_26_11));
        // D s_26_13: cmp-eq s_26_12 s_26_11
        let s_26_13: bool = ((s_26_12) == (s_26_11));
        // D s_26_14: lsl s_26_10 s_26_8
        let s_26_14: u64 = s_26_10 << s_26_8;
        // D s_26_15: or s_26_9 s_26_14
        let s_26_15: u64 = ((s_26_9) | (s_26_14));
        // D s_26_16: cmpl s_26_14
        let s_26_16: u64 = !s_26_14;
        // D s_26_17: and s_26_9 s_26_16
        let s_26_17: u64 = ((s_26_9) & (s_26_16));
        // D s_26_18: select s_26_13 s_26_15 s_26_17
        let s_26_18: u64 = if s_26_13 { s_26_15 } else { s_26_17 };
        // D s_26_19: cast trunc s_26_18 -> u8
        let s_26_19: bool = ((s_26_18) != 0);
        // C s_26_20: const #90704u : u32
        let s_26_20: u32 = 90704;
        // D s_26_21: read-reg s_26_20:struct
        let s_26_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_20 as isize);
            tracer.read_register(s_26_20 as isize, value);
            value
        };
        // D s_26_22: call _get_SCR_EL3_Type_NS(s_26_21)
        let s_26_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_26_21);
        // D s_26_23: cast zx s_26_19 -> bv
        let s_26_23: Bits = Bits::new(s_26_19 as u128, 1u16);
        // D s_26_24: cast zx s_26_22 -> bv
        let s_26_24: Bits = Bits::new(s_26_22 as u128, 1u16);
        // D s_26_25: cmp-ne s_26_23 s_26_24
        let s_26_25: bool = ((s_26_23) != (s_26_24));
        // D s_26_26: write-var gs#87238 <= s_26_25
        fn_state.gs_87238 = s_26_25;
        // N s_26_27: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#87238:u8
        let s_27_0: bool = fn_state.gs_87238;
        // N s_27_1: branch s_27_0 b33 b28
        if s_27_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #232u : u32
        let s_28_0: u32 = 232;
        // S s_28_1: call IsFeatureImplemented(s_28_0)
        let s_28_1: bool = IsFeatureImplemented(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b32 b29
        if s_28_1 {
            return block_32(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#87239 <= s_29_0
        fn_state.gs_87239 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#87239:u8
        let s_30_0: bool = fn_state.gs_87239;
        // D s_30_1: write-var gs#87240 <= s_30_0
        fn_state.gs_87240 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#87240:u8
        let s_31_0: bool = fn_state.gs_87240;
        // D s_31_1: write-var gs#87241 <= s_31_0
        fn_state.gs_87241 = s_31_0;
        // N s_31_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __MDCR_EL3_NSPBE:u8
        let s_32_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // D s_32_2: read-var __SCR_EL3_NSE:u8
        let s_32_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-ne s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) != (s_32_3));
        // D s_32_5: write-var gs#87239 <= s_32_4
        fn_state.gs_87239 = s_32_4;
        // N s_32_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#87240 <= s_33_0
        fn_state.gs_87240 = s_33_0;
        // N s_33_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#87238 <= s_34_0
        fn_state.gs_87238 = s_34_0;
        // N s_34_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: panic
        panic!("{:?}", ());
        // N s_35_1: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #22712u : u32
        let s_36_0: u32 = 22712;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_MDCR_EL3_Type_NSPB(s_36_1)
        let s_36_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_36_1);
        // C s_36_3: const #0s : i
        let s_36_3: i128 = 0;
        // D s_36_4: cast zx s_36_2 -> bv
        let s_36_4: Bits = Bits::new(s_36_2 as u128, 2u16);
        // C s_36_5: const #1u : u64
        let s_36_5: u64 = 1;
        // D s_36_6: bit-extract s_36_4 s_36_3 s_36_5
        let s_36_6: Bits = (Bits::new(
            ((s_36_4) >> (s_36_3)).value(),
            u16::try_from(s_36_5).unwrap(),
        ));
        // D s_36_7: cast reint s_36_6 -> u8
        let s_36_7: bool = ((s_36_6.value()) != 0);
        // C s_36_8: const #0s : i
        let s_36_8: i128 = 0;
        // C s_36_9: const #0u : u64
        let s_36_9: u64 = 0;
        // D s_36_10: cast zx s_36_7 -> u64
        let s_36_10: u64 = (s_36_7 as u64);
        // C s_36_11: const #1u : u64
        let s_36_11: u64 = 1;
        // D s_36_12: and s_36_10 s_36_11
        let s_36_12: u64 = ((s_36_10) & (s_36_11));
        // D s_36_13: cmp-eq s_36_12 s_36_11
        let s_36_13: bool = ((s_36_12) == (s_36_11));
        // D s_36_14: lsl s_36_10 s_36_8
        let s_36_14: u64 = s_36_10 << s_36_8;
        // D s_36_15: or s_36_9 s_36_14
        let s_36_15: u64 = ((s_36_9) | (s_36_14));
        // D s_36_16: cmpl s_36_14
        let s_36_16: u64 = !s_36_14;
        // D s_36_17: and s_36_9 s_36_16
        let s_36_17: u64 = ((s_36_9) & (s_36_16));
        // D s_36_18: select s_36_13 s_36_15 s_36_17
        let s_36_18: u64 = if s_36_13 { s_36_15 } else { s_36_17 };
        // D s_36_19: cast trunc s_36_18 -> u8
        let s_36_19: bool = ((s_36_18) != 0);
        // D s_36_20: cast zx s_36_19 -> bv
        let s_36_20: Bits = Bits::new(s_36_19 as u128, 1u16);
        // C s_36_21: const #0u : u8
        let s_36_21: bool = false;
        // C s_36_22: cast zx s_36_21 -> bv
        let s_36_22: Bits = Bits::new(s_36_21 as u128, 1u16);
        // D s_36_23: cmp-eq s_36_20 s_36_22
        let s_36_23: bool = ((s_36_20) == (s_36_22));
        // N s_36_24: branch s_36_23 b45 b37
        if s_36_23 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #22712u : u32
        let s_37_0: u32 = 22712;
        // D s_37_1: read-reg s_37_0:struct
        let s_37_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call _get_MDCR_EL3_Type_NSPB(s_37_1)
        let s_37_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_37_1);
        // C s_37_3: const #1s : i
        let s_37_3: i128 = 1;
        // D s_37_4: cast zx s_37_2 -> bv
        let s_37_4: Bits = Bits::new(s_37_2 as u128, 2u16);
        // C s_37_5: const #1u : u64
        let s_37_5: u64 = 1;
        // D s_37_6: bit-extract s_37_4 s_37_3 s_37_5
        let s_37_6: Bits = (Bits::new(
            ((s_37_4) >> (s_37_3)).value(),
            u16::try_from(s_37_5).unwrap(),
        ));
        // D s_37_7: cast reint s_37_6 -> u8
        let s_37_7: bool = ((s_37_6.value()) != 0);
        // C s_37_8: const #0s : i
        let s_37_8: i128 = 0;
        // C s_37_9: const #0u : u64
        let s_37_9: u64 = 0;
        // D s_37_10: cast zx s_37_7 -> u64
        let s_37_10: u64 = (s_37_7 as u64);
        // C s_37_11: const #1u : u64
        let s_37_11: u64 = 1;
        // D s_37_12: and s_37_10 s_37_11
        let s_37_12: u64 = ((s_37_10) & (s_37_11));
        // D s_37_13: cmp-eq s_37_12 s_37_11
        let s_37_13: bool = ((s_37_12) == (s_37_11));
        // D s_37_14: lsl s_37_10 s_37_8
        let s_37_14: u64 = s_37_10 << s_37_8;
        // D s_37_15: or s_37_9 s_37_14
        let s_37_15: u64 = ((s_37_9) | (s_37_14));
        // D s_37_16: cmpl s_37_14
        let s_37_16: u64 = !s_37_14;
        // D s_37_17: and s_37_9 s_37_16
        let s_37_17: u64 = ((s_37_9) & (s_37_16));
        // D s_37_18: select s_37_13 s_37_15 s_37_17
        let s_37_18: u64 = if s_37_13 { s_37_15 } else { s_37_17 };
        // D s_37_19: cast trunc s_37_18 -> u8
        let s_37_19: bool = ((s_37_18) != 0);
        // C s_37_20: const #90704u : u32
        let s_37_20: u32 = 90704;
        // D s_37_21: read-reg s_37_20:struct
        let s_37_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_37_20 as isize);
            tracer.read_register(s_37_20 as isize, value);
            value
        };
        // D s_37_22: call _get_SCR_EL3_Type_NS(s_37_21)
        let s_37_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_37_21);
        // D s_37_23: cast zx s_37_19 -> bv
        let s_37_23: Bits = Bits::new(s_37_19 as u128, 1u16);
        // D s_37_24: cast zx s_37_22 -> bv
        let s_37_24: Bits = Bits::new(s_37_22 as u128, 1u16);
        // D s_37_25: cmp-ne s_37_23 s_37_24
        let s_37_25: bool = ((s_37_23) != (s_37_24));
        // D s_37_26: write-var gs#87230 <= s_37_25
        fn_state.gs_87230 = s_37_25;
        // N s_37_27: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#87230:u8
        let s_38_0: bool = fn_state.gs_87230;
        // N s_38_1: branch s_38_0 b44 b39
        if s_38_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #232u : u32
        let s_39_0: u32 = 232;
        // S s_39_1: call IsFeatureImplemented(s_39_0)
        let s_39_1: bool = IsFeatureImplemented(state, tracer, s_39_0);
        // N s_39_2: branch s_39_1 b43 b40
        if s_39_1 {
            return block_43(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#87231 <= s_40_0
        fn_state.gs_87231 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#87231:u8
        let s_41_0: bool = fn_state.gs_87231;
        // D s_41_1: write-var gs#87232 <= s_41_0
        fn_state.gs_87232 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#87232:u8
        let s_42_0: bool = fn_state.gs_87232;
        // D s_42_1: write-var gs#87233 <= s_42_0
        fn_state.gs_87233 = s_42_0;
        // N s_42_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var __MDCR_EL3_NSPBE:u8
        let s_43_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 1u16);
        // D s_43_2: read-var __SCR_EL3_NSE:u8
        let s_43_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_4: cmp-ne s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) != (s_43_3));
        // D s_43_5: write-var gs#87231 <= s_43_4
        fn_state.gs_87231 = s_43_4;
        // N s_43_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#87232 <= s_44_0
        fn_state.gs_87232 = s_44_0;
        // N s_44_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#87230 <= s_45_0
        fn_state.gs_87230 = s_45_0;
        // N s_45_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_46_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_46_1: call __IMPDEF_boolean(s_46_0)
        let s_46_1: bool = u__IMPDEF_boolean(state, tracer, s_46_0);
        // D s_46_2: write-var gs#87225 <= s_46_1
        fn_state.gs_87225 = s_46_1;
        // N s_46_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EDSCR_read(s_47_0)
        let s_47_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_47_0);
        // S s_47_2: call _get_EDSCR_Type_SDD(s_47_1)
        let s_47_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_47_1);
        // S s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // C s_47_4: const #1u : u8
        let s_47_4: bool = true;
        // C s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 1u16);
        // S s_47_6: cmp-eq s_47_3 s_47_5
        let s_47_6: bool = ((s_47_3) == (s_47_5));
        // D s_47_7: write-var gs#87224 <= s_47_6
        fn_state.gs_87224 = s_47_6;
        // N s_47_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #424u : u32
        let s_48_0: u32 = 424;
        // D s_48_1: read-reg s_48_0:u8
        let s_48_1: u8 = {
            let value = state.read_register::<u8>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // C s_48_2: const #2u : u8
        let s_48_2: u8 = 2;
        // D s_48_3: cmp-lt s_48_1 s_48_2
        let s_48_3: bool = ((s_48_1) < (s_48_2));
        // D s_48_4: write-var gs#87223 <= s_48_3
        fn_state.gs_87223 = s_48_3;
        // N s_48_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call Halted(s_49_0)
        let s_49_1: bool = Halted(state, tracer, s_49_0);
        // N s_49_2: branch s_49_1 b118 b50
        if s_49_1 {
            return block_118(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#87244 <= s_50_0
        fn_state.gs_87244 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#87244:u8
        let s_51_0: bool = fn_state.gs_87244;
        // N s_51_1: branch s_51_0 b117 b52
        if s_51_0 {
            return block_117(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#87245 <= s_52_0
        fn_state.gs_87245 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#87245:u8
        let s_53_0: bool = fn_state.gs_87245;
        // N s_53_1: branch s_53_0 b116 b54
        if s_53_0 {
            return block_116(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#87246 <= s_54_0
        fn_state.gs_87246 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#87246:u8
        let s_55_0: bool = fn_state.gs_87246;
        // N s_55_1: branch s_55_0 b106 b56
        if s_55_0 {
            return block_106(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#87254 <= s_56_0
        fn_state.gs_87254 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#87254:u8
        let s_57_0: bool = fn_state.gs_87254;
        // N s_57_1: branch s_57_0 b105 b58
        if s_57_0 {
            return block_105(state, tracer, fn_state);
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
        // N s_58_2: branch s_58_1 b104 b59
        if s_58_1 {
            return block_104(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#87255 <= s_59_0
        fn_state.gs_87255 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#87255:u8
        let s_60_0: bool = fn_state.gs_87255;
        // N s_60_1: branch s_60_0 b100 b61
        if s_60_0 {
            return block_100(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#87257 <= s_61_0
        fn_state.gs_87257 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#87257:u8
        let s_62_0: bool = fn_state.gs_87257;
        // N s_62_1: branch s_62_0 b99 b63
        if s_62_0 {
            return block_99(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#87258 <= s_63_0
        fn_state.gs_87258 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#87258:u8
        let s_64_0: bool = fn_state.gs_87258;
        // N s_64_1: branch s_64_0 b98 b65
        if s_64_0 {
            return block_98(state, tracer, fn_state);
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
        // N s_65_2: branch s_65_1 b94 b66
        if s_65_1 {
            return block_94(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#87264 <= s_66_0
        fn_state.gs_87264 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#87264:u8
        let s_67_0: bool = fn_state.gs_87264;
        // N s_67_1: branch s_67_0 b93 b68
        if s_67_0 {
            return block_93(state, tracer, fn_state);
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
        // N s_68_4: branch s_68_3 b83 b69
        if s_68_3 {
            return block_83(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#87272 <= s_69_0
        fn_state.gs_87272 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#87272:u8
        let s_70_0: bool = fn_state.gs_87272;
        // N s_70_1: branch s_70_0 b77 b71
        if s_70_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call EL2Enabled(s_71_0)
        let s_71_1: bool = EL2Enabled(state, tracer, s_71_0);
        // N s_71_2: branch s_71_1 b76 b72
        if s_71_1 {
            return block_76(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#87273 <= s_72_0
        fn_state.gs_87273 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#87273:u8
        let s_73_0: bool = fn_state.gs_87273;
        // N s_73_1: branch s_73_0 b75 b74
        if s_73_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #64s : i64
        let s_74_0: i64 = 64;
        // D s_74_1: read-var t:i
        let s_74_1: i128 = fn_state.t;
        // D s_74_2: call X_read(s_74_1, s_74_0)
        let s_74_2: Bits = X_read(state, tracer, s_74_1, s_74_0);
        // D s_74_3: cast reint s_74_2 -> u64
        let s_74_3: u64 = (s_74_2.value() as u64);
        // D s_74_4: call Mk_PMBLIMITR_EL1_Type(s_74_3)
        let s_74_4: ProductType5c790c8ef59cc8b2 = Mk_PMBLIMITR_EL1_Type(
            state,
            tracer,
            s_74_3,
        );
        // C s_74_5: const #20480u : u32
        let s_74_5: u32 = 20480;
        // N s_74_6: write-reg s_74_5 <= s_74_4
        let s_74_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_74_5 as isize, s_74_4);
            tracer.write_register(s_74_5 as isize, s_74_4);
        };
        // N s_74_7: return
        return;
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #2048u : u12
        let s_75_0: u16 = 2048;
        // C s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 12u16);
        // C s_75_2: cast zx s_75_1 -> i
        let s_75_2: i128 = (s_75_1.value() as i128);
        // C s_75_3: cast reint s_75_2 -> i64
        let s_75_3: i64 = (s_75_2 as i64);
        // C s_75_4: const #64s : i64
        let s_75_4: i64 = 64;
        // D s_75_5: read-var t:i
        let s_75_5: i128 = fn_state.t;
        // D s_75_6: call X_read(s_75_5, s_75_4)
        let s_75_6: Bits = X_read(state, tracer, s_75_5, s_75_4);
        // D s_75_7: cast reint s_75_6 -> u64
        let s_75_7: u64 = (s_75_6.value() as u64);
        // C s_75_8: cast zx s_75_3 -> i
        let s_75_8: i128 = (i128::try_from(s_75_3).unwrap());
        // D s_75_9: call NVMem_set(s_75_8, s_75_7)
        let s_75_9: () = NVMem_set(state, tracer, s_75_8, s_75_7);
        // N s_75_10: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #102552u : u32
        let s_76_0: u32 = 102552;
        // D s_76_1: read-reg s_76_0:struct
        let s_76_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // D s_76_2: call _get_HCR_EL2_Type_NV2(s_76_1)
        let s_76_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_76_1);
        // C s_76_3: const #102552u : u32
        let s_76_3: u32 = 102552;
        // D s_76_4: read-reg s_76_3:struct
        let s_76_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_3 as isize);
            tracer.read_register(s_76_3 as isize, value);
            value
        };
        // D s_76_5: call _get_HCR_EL2_Type_NV(s_76_4)
        let s_76_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_76_4);
        // D s_76_6: cast zx s_76_2 -> bv
        let s_76_6: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_7: cast zx s_76_5 -> bv
        let s_76_7: Bits = Bits::new(s_76_5 as u128, 1u16);
        // D s_76_8: cast reint s_76_6 -> u128
        let s_76_8: u128 = (s_76_6.value() as u128);
        // D s_76_9: size-of s_76_6
        let s_76_9: u16 = s_76_6.length();
        // D s_76_10: cast reint s_76_7 -> u128
        let s_76_10: u128 = (s_76_7.value() as u128);
        // D s_76_11: size-of s_76_7
        let s_76_11: u16 = s_76_7.length();
        // D s_76_12: lsl s_76_8 s_76_11
        let s_76_12: u128 = s_76_8 << s_76_11;
        // D s_76_13: or s_76_12 s_76_10
        let s_76_13: u128 = ((s_76_12) | (s_76_10));
        // D s_76_14: add s_76_9 s_76_11
        let s_76_14: u16 = (s_76_9 + s_76_11);
        // D s_76_15: create-bits s_76_13 s_76_14
        let s_76_15: Bits = Bits::new(s_76_13, s_76_14);
        // D s_76_16: cast reint s_76_15 -> u8
        let s_76_16: u8 = (s_76_15.value() as u8);
        // D s_76_17: cast zx s_76_16 -> bv
        let s_76_17: Bits = Bits::new(s_76_16 as u128, 2u16);
        // C s_76_18: const #3u : u8
        let s_76_18: u8 = 3;
        // C s_76_19: cast zx s_76_18 -> bv
        let s_76_19: Bits = Bits::new(s_76_18 as u128, 2u16);
        // D s_76_20: cmp-eq s_76_17 s_76_19
        let s_76_20: bool = ((s_76_17) == (s_76_19));
        // D s_76_21: write-var gs#87273 <= s_76_20
        fn_state.gs_87273 = s_76_20;
        // N s_76_22: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #() : ()
        let s_77_0: () = ();
        // S s_77_1: call Halted(s_77_0)
        let s_77_1: bool = Halted(state, tracer, s_77_0);
        // N s_77_2: branch s_77_1 b82 b78
        if s_77_1 {
            return block_82(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#87276 <= s_78_0
        fn_state.gs_87276 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#87276:u8
        let s_79_0: bool = fn_state.gs_87276;
        // N s_79_1: branch s_79_0 b81 b80
        if s_79_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #24u : u8
        let s_80_0: u8 = 24;
        // C s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 8u16);
        // C s_80_2: cast zx s_80_1 -> i
        let s_80_2: i128 = (s_80_1.value() as i128);
        // C s_80_3: cast reint s_80_2 -> i64
        let s_80_3: i64 = (s_80_2 as i64);
        // C s_80_4: cast zx s_80_3 -> i
        let s_80_4: i128 = (i128::try_from(s_80_3).unwrap());
        // C s_80_5: const #424u : u32
        let s_80_5: u32 = 424;
        // D s_80_6: read-reg s_80_5:u8
        let s_80_6: u8 = {
            let value = state.read_register::<u8>(s_80_5 as isize);
            tracer.read_register(s_80_5 as isize, value);
            value
        };
        // D s_80_7: call AArch64_SystemAccessTrap(s_80_6, s_80_4)
        let s_80_7: () = AArch64_SystemAccessTrap(state, tracer, s_80_6, s_80_4);
        // N s_80_8: return
        return;
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_81_0: panic
        panic!("{:?}", ());
        // N s_81_1: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call EDSCR_read(s_82_0)
        let s_82_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_82_0);
        // S s_82_2: call _get_EDSCR_Type_SDD(s_82_1)
        let s_82_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_82_1);
        // S s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // C s_82_4: const #1u : u8
        let s_82_4: bool = true;
        // C s_82_5: cast zx s_82_4 -> bv
        let s_82_5: Bits = Bits::new(s_82_4 as u128, 1u16);
        // S s_82_6: cmp-eq s_82_3 s_82_5
        let s_82_6: bool = ((s_82_3) == (s_82_5));
        // D s_82_7: write-var gs#87276 <= s_82_6
        fn_state.gs_87276 = s_82_6;
        // N s_82_8: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #22712u : u32
        let s_83_0: u32 = 22712;
        // D s_83_1: read-reg s_83_0:struct
        let s_83_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: call _get_MDCR_EL3_Type_NSPB(s_83_1)
        let s_83_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_83_1);
        // C s_83_3: const #0s : i
        let s_83_3: i128 = 0;
        // D s_83_4: cast zx s_83_2 -> bv
        let s_83_4: Bits = Bits::new(s_83_2 as u128, 2u16);
        // C s_83_5: const #1u : u64
        let s_83_5: u64 = 1;
        // D s_83_6: bit-extract s_83_4 s_83_3 s_83_5
        let s_83_6: Bits = (Bits::new(
            ((s_83_4) >> (s_83_3)).value(),
            u16::try_from(s_83_5).unwrap(),
        ));
        // D s_83_7: cast reint s_83_6 -> u8
        let s_83_7: bool = ((s_83_6.value()) != 0);
        // C s_83_8: const #0s : i
        let s_83_8: i128 = 0;
        // C s_83_9: const #0u : u64
        let s_83_9: u64 = 0;
        // D s_83_10: cast zx s_83_7 -> u64
        let s_83_10: u64 = (s_83_7 as u64);
        // C s_83_11: const #1u : u64
        let s_83_11: u64 = 1;
        // D s_83_12: and s_83_10 s_83_11
        let s_83_12: u64 = ((s_83_10) & (s_83_11));
        // D s_83_13: cmp-eq s_83_12 s_83_11
        let s_83_13: bool = ((s_83_12) == (s_83_11));
        // D s_83_14: lsl s_83_10 s_83_8
        let s_83_14: u64 = s_83_10 << s_83_8;
        // D s_83_15: or s_83_9 s_83_14
        let s_83_15: u64 = ((s_83_9) | (s_83_14));
        // D s_83_16: cmpl s_83_14
        let s_83_16: u64 = !s_83_14;
        // D s_83_17: and s_83_9 s_83_16
        let s_83_17: u64 = ((s_83_9) & (s_83_16));
        // D s_83_18: select s_83_13 s_83_15 s_83_17
        let s_83_18: u64 = if s_83_13 { s_83_15 } else { s_83_17 };
        // D s_83_19: cast trunc s_83_18 -> u8
        let s_83_19: bool = ((s_83_18) != 0);
        // D s_83_20: cast zx s_83_19 -> bv
        let s_83_20: Bits = Bits::new(s_83_19 as u128, 1u16);
        // C s_83_21: const #0u : u8
        let s_83_21: bool = false;
        // C s_83_22: cast zx s_83_21 -> bv
        let s_83_22: Bits = Bits::new(s_83_21 as u128, 1u16);
        // D s_83_23: cmp-eq s_83_20 s_83_22
        let s_83_23: bool = ((s_83_20) == (s_83_22));
        // N s_83_24: branch s_83_23 b92 b84
        if s_83_23 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #22712u : u32
        let s_84_0: u32 = 22712;
        // D s_84_1: read-reg s_84_0:struct
        let s_84_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call _get_MDCR_EL3_Type_NSPB(s_84_1)
        let s_84_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_84_1);
        // C s_84_3: const #1s : i
        let s_84_3: i128 = 1;
        // D s_84_4: cast zx s_84_2 -> bv
        let s_84_4: Bits = Bits::new(s_84_2 as u128, 2u16);
        // C s_84_5: const #1u : u64
        let s_84_5: u64 = 1;
        // D s_84_6: bit-extract s_84_4 s_84_3 s_84_5
        let s_84_6: Bits = (Bits::new(
            ((s_84_4) >> (s_84_3)).value(),
            u16::try_from(s_84_5).unwrap(),
        ));
        // D s_84_7: cast reint s_84_6 -> u8
        let s_84_7: bool = ((s_84_6.value()) != 0);
        // C s_84_8: const #0s : i
        let s_84_8: i128 = 0;
        // C s_84_9: const #0u : u64
        let s_84_9: u64 = 0;
        // D s_84_10: cast zx s_84_7 -> u64
        let s_84_10: u64 = (s_84_7 as u64);
        // C s_84_11: const #1u : u64
        let s_84_11: u64 = 1;
        // D s_84_12: and s_84_10 s_84_11
        let s_84_12: u64 = ((s_84_10) & (s_84_11));
        // D s_84_13: cmp-eq s_84_12 s_84_11
        let s_84_13: bool = ((s_84_12) == (s_84_11));
        // D s_84_14: lsl s_84_10 s_84_8
        let s_84_14: u64 = s_84_10 << s_84_8;
        // D s_84_15: or s_84_9 s_84_14
        let s_84_15: u64 = ((s_84_9) | (s_84_14));
        // D s_84_16: cmpl s_84_14
        let s_84_16: u64 = !s_84_14;
        // D s_84_17: and s_84_9 s_84_16
        let s_84_17: u64 = ((s_84_9) & (s_84_16));
        // D s_84_18: select s_84_13 s_84_15 s_84_17
        let s_84_18: u64 = if s_84_13 { s_84_15 } else { s_84_17 };
        // D s_84_19: cast trunc s_84_18 -> u8
        let s_84_19: bool = ((s_84_18) != 0);
        // C s_84_20: const #90704u : u32
        let s_84_20: u32 = 90704;
        // D s_84_21: read-reg s_84_20:struct
        let s_84_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_20 as isize);
            tracer.read_register(s_84_20 as isize, value);
            value
        };
        // D s_84_22: call _get_SCR_EL3_Type_NS(s_84_21)
        let s_84_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_84_21);
        // D s_84_23: cast zx s_84_19 -> bv
        let s_84_23: Bits = Bits::new(s_84_19 as u128, 1u16);
        // D s_84_24: cast zx s_84_22 -> bv
        let s_84_24: Bits = Bits::new(s_84_22 as u128, 1u16);
        // D s_84_25: cmp-ne s_84_23 s_84_24
        let s_84_25: bool = ((s_84_23) != (s_84_24));
        // D s_84_26: write-var gs#87269 <= s_84_25
        fn_state.gs_87269 = s_84_25;
        // N s_84_27: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#87269:u8
        let s_85_0: bool = fn_state.gs_87269;
        // N s_85_1: branch s_85_0 b91 b86
        if s_85_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #232u : u32
        let s_86_0: u32 = 232;
        // S s_86_1: call IsFeatureImplemented(s_86_0)
        let s_86_1: bool = IsFeatureImplemented(state, tracer, s_86_0);
        // N s_86_2: branch s_86_1 b90 b87
        if s_86_1 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#87270 <= s_87_0
        fn_state.gs_87270 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#87270:u8
        let s_88_0: bool = fn_state.gs_87270;
        // D s_88_1: write-var gs#87271 <= s_88_0
        fn_state.gs_87271 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#87271:u8
        let s_89_0: bool = fn_state.gs_87271;
        // D s_89_1: write-var gs#87272 <= s_89_0
        fn_state.gs_87272 = s_89_0;
        // N s_89_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var __MDCR_EL3_NSPBE:u8
        let s_90_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 1u16);
        // D s_90_2: read-var __SCR_EL3_NSE:u8
        let s_90_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // D s_90_4: cmp-ne s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) != (s_90_3));
        // D s_90_5: write-var gs#87270 <= s_90_4
        fn_state.gs_87270 = s_90_4;
        // N s_90_6: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #1u : u8
        let s_91_0: bool = true;
        // D s_91_1: write-var gs#87271 <= s_91_0
        fn_state.gs_87271 = s_91_0;
        // N s_91_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #1u : u8
        let s_92_0: bool = true;
        // D s_92_1: write-var gs#87269 <= s_92_0
        fn_state.gs_87269 = s_92_0;
        // N s_92_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #24u : u8
        let s_93_0: u8 = 24;
        // C s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 8u16);
        // C s_93_2: cast zx s_93_1 -> i
        let s_93_2: i128 = (s_93_1.value() as i128);
        // C s_93_3: cast reint s_93_2 -> i64
        let s_93_3: i64 = (s_93_2 as i64);
        // C s_93_4: cast zx s_93_3 -> i
        let s_93_4: i128 = (i128::try_from(s_93_3).unwrap());
        // C s_93_5: const #432u : u32
        let s_93_5: u32 = 432;
        // D s_93_6: read-reg s_93_5:u8
        let s_93_6: u8 = {
            let value = state.read_register::<u8>(s_93_5 as isize);
            tracer.read_register(s_93_5 as isize, value);
            value
        };
        // D s_93_7: call AArch64_SystemAccessTrap(s_93_6, s_93_4)
        let s_93_7: () = AArch64_SystemAccessTrap(state, tracer, s_93_6, s_93_4);
        // N s_93_8: return
        return;
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var __MDCR_EL2_E2PB:u8
        let s_94_0: u8 = fn_state.u__MDCR_EL2_E2PB;
        // C s_94_1: const #0s : i
        let s_94_1: i128 = 0;
        // D s_94_2: cast zx s_94_0 -> bv
        let s_94_2: Bits = Bits::new(s_94_0 as u128, 2u16);
        // C s_94_3: const #1s : i64
        let s_94_3: i64 = 1;
        // C s_94_4: cast zx s_94_3 -> i
        let s_94_4: i128 = (i128::try_from(s_94_3).unwrap());
        // C s_94_5: const #0s : i
        let s_94_5: i128 = 0;
        // C s_94_6: add s_94_5 s_94_4
        let s_94_6: i128 = (s_94_5 + s_94_4);
        // D s_94_7: bit-extract s_94_2 s_94_1 s_94_6
        let s_94_7: Bits = (Bits::new(
            ((s_94_2) >> (s_94_1)).value(),
            u16::try_from(s_94_6).unwrap(),
        ));
        // D s_94_8: cast reint s_94_7 -> u8
        let s_94_8: bool = ((s_94_7.value()) != 0);
        // D s_94_9: cast zx s_94_8 -> bv
        let s_94_9: Bits = Bits::new(s_94_8 as u128, 1u16);
        // C s_94_10: const #0u : u8
        let s_94_10: bool = false;
        // C s_94_11: cast zx s_94_10 -> bv
        let s_94_11: Bits = Bits::new(s_94_10 as u128, 1u16);
        // D s_94_12: cmp-eq s_94_9 s_94_11
        let s_94_12: bool = ((s_94_9) == (s_94_11));
        // D s_94_13: not s_94_12
        let s_94_13: bool = !s_94_12;
        // N s_94_14: branch s_94_13 b97 b95
        if s_94_13 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #1u : u8
        let s_95_0: bool = true;
        // D s_95_1: write-var gs#87259 <= s_95_0
        fn_state.gs_87259 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#87259:u8
        let s_96_0: bool = fn_state.gs_87259;
        // D s_96_1: write-var gs#87264 <= s_96_0
        fn_state.gs_87264 = s_96_0;
        // N s_96_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#87259 <= s_97_0
        fn_state.gs_87259 = s_97_0;
        // N s_97_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #24u : u8
        let s_98_0: u8 = 24;
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
        // D s_98_7: call AArch64_SystemAccessTrap(s_98_6, s_98_4)
        let s_98_7: () = AArch64_SystemAccessTrap(state, tracer, s_98_6, s_98_4);
        // N s_98_8: return
        return;
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var __HDFGWTR_EL2_PMBLIMITR_EL1:u8
        let s_99_0: bool = fn_state.u__HDFGWTR_EL2_PMBLIMITR_EL1;
        // D s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 1u16);
        // C s_99_2: const #1u : u8
        let s_99_2: bool = true;
        // C s_99_3: cast zx s_99_2 -> bv
        let s_99_3: Bits = Bits::new(s_99_2 as u128, 1u16);
        // D s_99_4: cmp-eq s_99_1 s_99_3
        let s_99_4: bool = ((s_99_1) == (s_99_3));
        // D s_99_5: write-var gs#87258 <= s_99_4
        fn_state.gs_87258 = s_99_4;
        // N s_99_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #424u : u32
        let s_100_0: u32 = 424;
        // D s_100_1: read-reg s_100_0:u8
        let s_100_1: u8 = {
            let value = state.read_register::<u8>(s_100_0 as isize);
            tracer.read_register(s_100_0 as isize, value);
            value
        };
        // C s_100_2: const #2u : u8
        let s_100_2: u8 = 2;
        // D s_100_3: cmp-lt s_100_1 s_100_2
        let s_100_3: bool = ((s_100_1) < (s_100_2));
        // D s_100_4: not s_100_3
        let s_100_4: bool = !s_100_3;
        // N s_100_5: branch s_100_4 b103 b101
        if s_100_4 {
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
        // D s_101_0: read-var __SCR_EL3_FGTEn:u8
        let s_101_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 1u16);
        // C s_101_2: const #1u : u8
        let s_101_2: bool = true;
        // C s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 1u16);
        // D s_101_4: cmp-eq s_101_1 s_101_3
        let s_101_4: bool = ((s_101_1) == (s_101_3));
        // D s_101_5: write-var gs#87256 <= s_101_4
        fn_state.gs_87256 = s_101_4;
        // N s_101_6: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#87256:u8
        let s_102_0: bool = fn_state.gs_87256;
        // D s_102_1: write-var gs#87257 <= s_102_0
        fn_state.gs_87257 = s_102_0;
        // N s_102_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #1u : u8
        let s_103_0: bool = true;
        // D s_103_1: write-var gs#87256 <= s_103_0
        fn_state.gs_87256 = s_103_0;
        // N s_103_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #146u : u32
        let s_104_0: u32 = 146;
        // S s_104_1: call IsFeatureImplemented(s_104_0)
        let s_104_1: bool = IsFeatureImplemented(state, tracer, s_104_0);
        // D s_104_2: write-var gs#87255 <= s_104_1
        fn_state.gs_87255 = s_104_1;
        // N s_104_3: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_105_0: panic
        panic!("{:?}", ());
        // N s_105_1: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #22712u : u32
        let s_106_0: u32 = 22712;
        // D s_106_1: read-reg s_106_0:struct
        let s_106_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // D s_106_2: call _get_MDCR_EL3_Type_NSPB(s_106_1)
        let s_106_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_106_1);
        // C s_106_3: const #0s : i
        let s_106_3: i128 = 0;
        // D s_106_4: cast zx s_106_2 -> bv
        let s_106_4: Bits = Bits::new(s_106_2 as u128, 2u16);
        // C s_106_5: const #1u : u64
        let s_106_5: u64 = 1;
        // D s_106_6: bit-extract s_106_4 s_106_3 s_106_5
        let s_106_6: Bits = (Bits::new(
            ((s_106_4) >> (s_106_3)).value(),
            u16::try_from(s_106_5).unwrap(),
        ));
        // D s_106_7: cast reint s_106_6 -> u8
        let s_106_7: bool = ((s_106_6.value()) != 0);
        // C s_106_8: const #0s : i
        let s_106_8: i128 = 0;
        // C s_106_9: const #0u : u64
        let s_106_9: u64 = 0;
        // D s_106_10: cast zx s_106_7 -> u64
        let s_106_10: u64 = (s_106_7 as u64);
        // C s_106_11: const #1u : u64
        let s_106_11: u64 = 1;
        // D s_106_12: and s_106_10 s_106_11
        let s_106_12: u64 = ((s_106_10) & (s_106_11));
        // D s_106_13: cmp-eq s_106_12 s_106_11
        let s_106_13: bool = ((s_106_12) == (s_106_11));
        // D s_106_14: lsl s_106_10 s_106_8
        let s_106_14: u64 = s_106_10 << s_106_8;
        // D s_106_15: or s_106_9 s_106_14
        let s_106_15: u64 = ((s_106_9) | (s_106_14));
        // D s_106_16: cmpl s_106_14
        let s_106_16: u64 = !s_106_14;
        // D s_106_17: and s_106_9 s_106_16
        let s_106_17: u64 = ((s_106_9) & (s_106_16));
        // D s_106_18: select s_106_13 s_106_15 s_106_17
        let s_106_18: u64 = if s_106_13 { s_106_15 } else { s_106_17 };
        // D s_106_19: cast trunc s_106_18 -> u8
        let s_106_19: bool = ((s_106_18) != 0);
        // D s_106_20: cast zx s_106_19 -> bv
        let s_106_20: Bits = Bits::new(s_106_19 as u128, 1u16);
        // C s_106_21: const #0u : u8
        let s_106_21: bool = false;
        // C s_106_22: cast zx s_106_21 -> bv
        let s_106_22: Bits = Bits::new(s_106_21 as u128, 1u16);
        // D s_106_23: cmp-eq s_106_20 s_106_22
        let s_106_23: bool = ((s_106_20) == (s_106_22));
        // N s_106_24: branch s_106_23 b115 b107
        if s_106_23 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #22712u : u32
        let s_107_0: u32 = 22712;
        // D s_107_1: read-reg s_107_0:struct
        let s_107_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: call _get_MDCR_EL3_Type_NSPB(s_107_1)
        let s_107_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_107_1);
        // C s_107_3: const #1s : i
        let s_107_3: i128 = 1;
        // D s_107_4: cast zx s_107_2 -> bv
        let s_107_4: Bits = Bits::new(s_107_2 as u128, 2u16);
        // C s_107_5: const #1u : u64
        let s_107_5: u64 = 1;
        // D s_107_6: bit-extract s_107_4 s_107_3 s_107_5
        let s_107_6: Bits = (Bits::new(
            ((s_107_4) >> (s_107_3)).value(),
            u16::try_from(s_107_5).unwrap(),
        ));
        // D s_107_7: cast reint s_107_6 -> u8
        let s_107_7: bool = ((s_107_6.value()) != 0);
        // C s_107_8: const #0s : i
        let s_107_8: i128 = 0;
        // C s_107_9: const #0u : u64
        let s_107_9: u64 = 0;
        // D s_107_10: cast zx s_107_7 -> u64
        let s_107_10: u64 = (s_107_7 as u64);
        // C s_107_11: const #1u : u64
        let s_107_11: u64 = 1;
        // D s_107_12: and s_107_10 s_107_11
        let s_107_12: u64 = ((s_107_10) & (s_107_11));
        // D s_107_13: cmp-eq s_107_12 s_107_11
        let s_107_13: bool = ((s_107_12) == (s_107_11));
        // D s_107_14: lsl s_107_10 s_107_8
        let s_107_14: u64 = s_107_10 << s_107_8;
        // D s_107_15: or s_107_9 s_107_14
        let s_107_15: u64 = ((s_107_9) | (s_107_14));
        // D s_107_16: cmpl s_107_14
        let s_107_16: u64 = !s_107_14;
        // D s_107_17: and s_107_9 s_107_16
        let s_107_17: u64 = ((s_107_9) & (s_107_16));
        // D s_107_18: select s_107_13 s_107_15 s_107_17
        let s_107_18: u64 = if s_107_13 { s_107_15 } else { s_107_17 };
        // D s_107_19: cast trunc s_107_18 -> u8
        let s_107_19: bool = ((s_107_18) != 0);
        // C s_107_20: const #90704u : u32
        let s_107_20: u32 = 90704;
        // D s_107_21: read-reg s_107_20:struct
        let s_107_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_107_20 as isize);
            tracer.read_register(s_107_20 as isize, value);
            value
        };
        // D s_107_22: call _get_SCR_EL3_Type_NS(s_107_21)
        let s_107_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_107_21);
        // D s_107_23: cast zx s_107_19 -> bv
        let s_107_23: Bits = Bits::new(s_107_19 as u128, 1u16);
        // D s_107_24: cast zx s_107_22 -> bv
        let s_107_24: Bits = Bits::new(s_107_22 as u128, 1u16);
        // D s_107_25: cmp-ne s_107_23 s_107_24
        let s_107_25: bool = ((s_107_23) != (s_107_24));
        // D s_107_26: write-var gs#87251 <= s_107_25
        fn_state.gs_87251 = s_107_25;
        // N s_107_27: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#87251:u8
        let s_108_0: bool = fn_state.gs_87251;
        // N s_108_1: branch s_108_0 b114 b109
        if s_108_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #232u : u32
        let s_109_0: u32 = 232;
        // S s_109_1: call IsFeatureImplemented(s_109_0)
        let s_109_1: bool = IsFeatureImplemented(state, tracer, s_109_0);
        // N s_109_2: branch s_109_1 b113 b110
        if s_109_1 {
            return block_113(state, tracer, fn_state);
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
        // D s_110_1: write-var gs#87252 <= s_110_0
        fn_state.gs_87252 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#87252:u8
        let s_111_0: bool = fn_state.gs_87252;
        // D s_111_1: write-var gs#87253 <= s_111_0
        fn_state.gs_87253 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#87253:u8
        let s_112_0: bool = fn_state.gs_87253;
        // D s_112_1: write-var gs#87254 <= s_112_0
        fn_state.gs_87254 = s_112_0;
        // N s_112_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var __MDCR_EL3_NSPBE:u8
        let s_113_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_113_1: cast zx s_113_0 -> bv
        let s_113_1: Bits = Bits::new(s_113_0 as u128, 1u16);
        // D s_113_2: read-var __SCR_EL3_NSE:u8
        let s_113_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_113_3: cast zx s_113_2 -> bv
        let s_113_3: Bits = Bits::new(s_113_2 as u128, 1u16);
        // D s_113_4: cmp-ne s_113_1 s_113_3
        let s_113_4: bool = ((s_113_1) != (s_113_3));
        // D s_113_5: write-var gs#87252 <= s_113_4
        fn_state.gs_87252 = s_113_4;
        // N s_113_6: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #1u : u8
        let s_114_0: bool = true;
        // D s_114_1: write-var gs#87253 <= s_114_0
        fn_state.gs_87253 = s_114_0;
        // N s_114_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #1u : u8
        let s_115_0: bool = true;
        // D s_115_1: write-var gs#87251 <= s_115_0
        fn_state.gs_87251 = s_115_0;
        // N s_115_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_116_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_116_1: call __IMPDEF_boolean(s_116_0)
        let s_116_1: bool = u__IMPDEF_boolean(state, tracer, s_116_0);
        // D s_116_2: write-var gs#87246 <= s_116_1
        fn_state.gs_87246 = s_116_1;
        // N s_116_3: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #() : ()
        let s_117_0: () = ();
        // S s_117_1: call EDSCR_read(s_117_0)
        let s_117_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_117_0);
        // S s_117_2: call _get_EDSCR_Type_SDD(s_117_1)
        let s_117_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_117_1);
        // S s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 1u16);
        // C s_117_4: const #1u : u8
        let s_117_4: bool = true;
        // C s_117_5: cast zx s_117_4 -> bv
        let s_117_5: Bits = Bits::new(s_117_4 as u128, 1u16);
        // S s_117_6: cmp-eq s_117_3 s_117_5
        let s_117_6: bool = ((s_117_3) == (s_117_5));
        // D s_117_7: write-var gs#87245 <= s_117_6
        fn_state.gs_87245 = s_117_6;
        // N s_117_8: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #424u : u32
        let s_118_0: u32 = 424;
        // D s_118_1: read-reg s_118_0:u8
        let s_118_1: u8 = {
            let value = state.read_register::<u8>(s_118_0 as isize);
            tracer.read_register(s_118_0 as isize, value);
            value
        };
        // C s_118_2: const #2u : u8
        let s_118_2: u8 = 2;
        // D s_118_3: cmp-lt s_118_1 s_118_2
        let s_118_3: bool = ((s_118_1) < (s_118_2));
        // D s_118_4: write-var gs#87244 <= s_118_3
        fn_state.gs_87244 = s_118_3;
        // N s_118_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_119_0: panic
        panic!("{:?}", ());
        // N s_119_1: return
        return;
    }
}
