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
use u_get_CPTR_EL2_Type_SMEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use Mk_SMCR_EL2_Type::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_CPTR_EL2_Type_TSM::*;
use Halted::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_CPTR_EL3_Type_ESM::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use EDSCR_read::*;
use EL2Enabled::*;
use common::*;
pub fn SMCR_EL2_SysRegWrite_89c07e726ee82646<T: Tracer>(
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
        gs_91974: bool,
        gs_91976: bool,
        u__HCR_EL2_E2H: bool,
        u__EDSCR_SDD: bool,
        gs_91973: bool,
        gs_91981: bool,
        gs_91972: bool,
        gs_91971: bool,
        u__CPTR_EL2_TSM: bool,
        u__CPTR_EL3_ESM: bool,
        gs_91982: bool,
        u__PSTATE_EL: u8,
        gs_91975: bool,
        u__CPTR_EL2_SMEN: u8,
        gs_91984: bool,
        u__HCR_EL2_NV: bool,
        gs_91985: bool,
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
        // C s_0_3: const #102552u : u32
        let s_0_3: u32 = 102552;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HCR_EL2_Type_NV(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_NV <= s_0_5
        fn_state.u__HCR_EL2_NV = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call EDSCR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_EDSCR_Type_SDD(s_0_8)
        let s_0_9: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_8);
        // D s_0_10: write-var __EDSCR_SDD <= s_0_9
        fn_state.u__EDSCR_SDD = s_0_9;
        // C s_0_11: const #16840u : u32
        let s_0_11: u32 = 16840;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_CPTR_EL3_Type_ESM(s_0_12)
        let s_0_13: bool = u_get_CPTR_EL3_Type_ESM(state, tracer, s_0_12);
        // D s_0_14: write-var __CPTR_EL3_ESM <= s_0_13
        fn_state.u__CPTR_EL3_ESM = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_E2H(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_E2H <= s_0_17
        fn_state.u__HCR_EL2_E2H = s_0_17;
        // C s_0_19: const #11088u : u32
        let s_0_19: u32 = 11088;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_CPTR_EL2_Type_TSM(s_0_20)
        let s_0_21: bool = u_get_CPTR_EL2_Type_TSM(state, tracer, s_0_20);
        // D s_0_22: write-var __CPTR_EL2_TSM <= s_0_21
        fn_state.u__CPTR_EL2_TSM = s_0_21;
        // C s_0_23: const #11088u : u32
        let s_0_23: u32 = 11088;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_CPTR_EL2_Type_SMEN(s_0_24)
        let s_0_25: u8 = u_get_CPTR_EL2_Type_SMEN(state, tracer, s_0_24);
        // D s_0_26: write-var __CPTR_EL2_SMEN <= s_0_25
        fn_state.u__CPTR_EL2_SMEN = s_0_25;
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
        // N s_0_33: branch s_0_32 b52 b1
        if s_0_32 {
            return block_52(state, tracer, fn_state);
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
        // D s_5_0: read-var __CPTR_EL3_ESM:u8
        let s_5_0: bool = fn_state.u__CPTR_EL3_ESM;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
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
        // D s_6_1: read-var t:i
        let s_6_1: i128 = fn_state.t;
        // D s_6_2: call X_read(s_6_1, s_6_0)
        let s_6_2: Bits = X_read(state, tracer, s_6_1, s_6_0);
        // D s_6_3: cast reint s_6_2 -> u64
        let s_6_3: u64 = (s_6_2.value() as u64);
        // D s_6_4: call Mk_SMCR_EL2_Type(s_6_3)
        let s_6_4: ProductType5c790c8ef59cc8b2 = Mk_SMCR_EL2_Type(state, tracer, s_6_3);
        // C s_6_5: const #10376u : u32
        let s_6_5: u32 = 10376;
        // N s_6_6: write-reg s_6_5 <= s_6_4
        let s_6_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_5 as isize, s_6_4);
            tracer.write_register(s_6_5 as isize, s_6_4);
        };
        // N s_6_7: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #29u : u8
        let s_7_0: u8 = 29;
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
        // D s_9_1: write-var gs#91971 <= s_9_0
        fn_state.gs_91971 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#91971:u8
        let s_10_0: bool = fn_state.gs_91971;
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
        // D s_11_1: write-var gs#91972 <= s_11_0
        fn_state.gs_91972 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#91972:u8
        let s_12_0: bool = fn_state.gs_91972;
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
        // D s_13_1: write-var gs#91973 <= s_13_0
        fn_state.gs_91973 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#91973:u8
        let s_14_0: bool = fn_state.gs_91973;
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
        // D s_15_1: write-var gs#91974 <= s_15_0
        fn_state.gs_91974 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#91974:u8
        let s_16_0: bool = fn_state.gs_91974;
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
        // D s_18_1: write-var gs#91975 <= s_18_0
        fn_state.gs_91975 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#91975:u8
        let s_19_0: bool = fn_state.gs_91975;
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
        // D s_21_1: write-var gs#91981 <= s_21_0
        fn_state.gs_91981 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#91981:u8
        let s_22_0: bool = fn_state.gs_91981;
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
        // D s_24_1: write-var gs#91982 <= s_24_0
        fn_state.gs_91982 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#91982:u8
        let s_25_0: bool = fn_state.gs_91982;
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
        // D s_26_1: read-var t:i
        let s_26_1: i128 = fn_state.t;
        // D s_26_2: call X_read(s_26_1, s_26_0)
        let s_26_2: Bits = X_read(state, tracer, s_26_1, s_26_0);
        // D s_26_3: cast reint s_26_2 -> u64
        let s_26_3: u64 = (s_26_2.value() as u64);
        // D s_26_4: call Mk_SMCR_EL2_Type(s_26_3)
        let s_26_4: ProductType5c790c8ef59cc8b2 = Mk_SMCR_EL2_Type(
            state,
            tracer,
            s_26_3,
        );
        // C s_26_5: const #10376u : u32
        let s_26_5: u32 = 10376;
        // N s_26_6: write-reg s_26_5 <= s_26_4
        let s_26_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_26_5 as isize, s_26_4);
            tracer.write_register(s_26_5 as isize, s_26_4);
        };
        // N s_26_7: return
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
        // D s_28_1: write-var gs#91984 <= s_28_0
        fn_state.gs_91984 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#91984:u8
        let s_29_0: bool = fn_state.gs_91984;
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
        // C s_30_0: const #29u : u8
        let s_30_0: u8 = 29;
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
        // D s_32_5: write-var gs#91984 <= s_32_4
        fn_state.gs_91984 = s_32_4;
        // N s_32_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __CPTR_EL3_ESM:u8
        let s_33_0: bool = fn_state.u__CPTR_EL3_ESM;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #0u : u8
        let s_33_2: bool = false;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#91982 <= s_33_4
        fn_state.gs_91982 = s_33_4;
        // N s_33_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #29u : u8
        let s_34_0: u8 = 29;
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
        // D s_35_0: read-var __CPTR_EL2_SMEN:u8
        let s_35_0: u8 = fn_state.u__CPTR_EL2_SMEN;
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
        // D s_36_1: write-var gs#91976 <= s_36_0
        fn_state.gs_91976 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#91976:u8
        let s_37_0: bool = fn_state.gs_91976;
        // D s_37_1: write-var gs#91981 <= s_37_0
        fn_state.gs_91981 = s_37_0;
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
        // D s_38_1: write-var gs#91976 <= s_38_0
        fn_state.gs_91976 = s_38_0;
        // N s_38_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #29u : u8
        let s_39_0: u8 = 29;
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
        // D s_40_0: read-var __CPTR_EL2_TSM:u8
        let s_40_0: bool = fn_state.u__CPTR_EL2_TSM;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #1u : u8
        let s_40_2: bool = true;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#91975 <= s_40_4
        fn_state.gs_91975 = s_40_4;
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
        // D s_42_0: read-var __CPTR_EL3_ESM:u8
        let s_42_0: bool = fn_state.u__CPTR_EL3_ESM;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 1u16);
        // C s_42_2: const #0u : u8
        let s_42_2: bool = false;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: write-var gs#91974 <= s_42_4
        fn_state.gs_91974 = s_42_4;
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
        // D s_43_2: write-var gs#91973 <= s_43_1
        fn_state.gs_91973 = s_43_1;
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
        // D s_44_5: write-var gs#91972 <= s_44_4
        fn_state.gs_91972 = s_44_4;
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
        // D s_45_4: write-var gs#91971 <= s_45_3
        fn_state.gs_91971 = s_45_3;
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
        // S s_46_1: call EL2Enabled(s_46_0)
        let s_46_1: bool = EL2Enabled(state, tracer, s_46_0);
        // N s_46_2: branch s_46_1 b51 b47
        if s_46_1 {
            return block_51(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#91985 <= s_47_0
        fn_state.gs_91985 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#91985:u8
        let s_48_0: bool = fn_state.gs_91985;
        // N s_48_1: branch s_48_0 b50 b49
        if s_48_0 {
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
        // N s_49_0: panic
        panic!("{:?}", ());
        // N s_49_1: return
        return;
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #24u : u8
        let s_50_0: u8 = 24;
        // C s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 8u16);
        // C s_50_2: cast zx s_50_1 -> i
        let s_50_2: i128 = (s_50_1.value() as i128);
        // C s_50_3: cast reint s_50_2 -> i64
        let s_50_3: i64 = (s_50_2 as i64);
        // C s_50_4: cast zx s_50_3 -> i
        let s_50_4: i128 = (i128::try_from(s_50_3).unwrap());
        // C s_50_5: const #432u : u32
        let s_50_5: u32 = 432;
        // D s_50_6: read-reg s_50_5:u8
        let s_50_6: u8 = {
            let value = state.read_register::<u8>(s_50_5 as isize);
            tracer.read_register(s_50_5 as isize, value);
            value
        };
        // D s_50_7: call AArch64_SystemAccessTrap(s_50_6, s_50_4)
        let s_50_7: () = AArch64_SystemAccessTrap(state, tracer, s_50_6, s_50_4);
        // N s_50_8: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var __HCR_EL2_NV:u8
        let s_51_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #1u : u8
        let s_51_2: bool = true;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#91985 <= s_51_4
        fn_state.gs_91985 = s_51_4;
        // N s_51_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: panic
        panic!("{:?}", ());
        // N s_52_1: return
        return;
    }
}
