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
use NVMem_set__1::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_SCR_EL3_Type_D128En::*;
use Halted::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use X_read::*;
use Mk_TTBR1_EL1_Type::*;
use u_get_HCR_EL2_Type_NV1::*;
use u_get_HCR_EL2_Type_NV::*;
use ELUsingAArch32::*;
use u_get_EDSCR_Type_SDD::*;
use TTBR1_EL1_write::*;
use TTBR1_EL1_read::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn TTBR1_EL1_SysRegWrite128_d20c29029eb4ed94<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_137902: bool,
        gs_137912: bool,
        gs_137909: bool,
        u__HCR_EL2_E2H: bool,
        u__SCR_EL3_D128En: bool,
        u__EDSCR_SDD: bool,
        ga_240300: ProductType782ac6922b48c20d,
        gs_137921: bool,
        gs_137903: bool,
        gs_137920: bool,
        gs_137910: bool,
        gs_137913: bool,
        gs_137919: bool,
        gs_137911: bool,
        u__PSTATE_EL: u8,
        u__HCR_EL2_NV: bool,
        ga_240284: ProductType782ac6922b48c20d,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
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
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_E2H(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_E2H <= s_0_9
        fn_state.u__HCR_EL2_E2H = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call EDSCR_read(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_11);
        // S s_0_13: call _get_EDSCR_Type_SDD(s_0_12)
        let s_0_13: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_12);
        // D s_0_14: write-var __EDSCR_SDD <= s_0_13
        fn_state.u__EDSCR_SDD = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_D128En(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_D128En(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_D128En <= s_0_17
        fn_state.u__SCR_EL3_D128En = s_0_17;
        // D s_0_19: read-var __PSTATE_EL:u8
        let s_0_19: u8 = fn_state.u__PSTATE_EL;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 2u16);
        // C s_0_21: const #448u : u32
        let s_0_21: u32 = 448;
        // D s_0_22: read-reg s_0_21:u8
        let s_0_22: u8 = {
            let value = state.read_register::<u8>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 2u16);
        // D s_0_24: cmp-eq s_0_20 s_0_23
        let s_0_24: bool = ((s_0_20) == (s_0_23));
        // N s_0_25: branch s_0_24 b52 b1
        if s_0_24 {
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
        // N s_1_6: branch s_1_5 b41 b2
        if s_1_5 {
            return block_41(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b14 b3
        if s_2_5 {
            return block_14(state, tracer, fn_state);
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
        // S s_5_1: call EL2Enabled(s_5_0)
        let s_5_1: bool = EL2Enabled(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b13 b6
        if s_5_1 {
            return block_13(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#137902 <= s_6_0
        fn_state.gs_137902 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#137902:u8
        let s_7_0: bool = fn_state.gs_137902;
        // N s_7_1: branch s_7_0 b12 b8
        if s_7_0 {
            return block_12(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#137903 <= s_8_0
        fn_state.gs_137903 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#137903:u8
        let s_9_0: bool = fn_state.gs_137903;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
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
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call TTBR1_EL1_read(s_11_0)
        let s_11_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_11_0);
        // D s_11_2: write-var ga#240300 <= s_11_1
        fn_state.ga_240300 = s_11_1;
        // D s_11_3: read-var ga#240300.0:struct
        let s_11_3: u128 = fn_state.ga_240300._0;
        // C s_11_4: const #1s : i
        let s_11_4: i128 = 1;
        // D s_11_5: read-var t:i
        let s_11_5: i128 = fn_state.t;
        // D s_11_6: add s_11_5 s_11_4
        let s_11_6: i128 = (s_11_5 + s_11_4);
        // C s_11_7: const #64s : i64
        let s_11_7: i64 = 64;
        // D s_11_8: call X_read(s_11_6, s_11_7)
        let s_11_8: Bits = X_read(state, tracer, s_11_6, s_11_7);
        // D s_11_9: cast reint s_11_8 -> u64
        let s_11_9: u64 = (s_11_8.value() as u64);
        // C s_11_10: const #64s : i64
        let s_11_10: i64 = 64;
        // D s_11_11: read-var t:i
        let s_11_11: i128 = fn_state.t;
        // D s_11_12: call X_read(s_11_11, s_11_10)
        let s_11_12: Bits = X_read(state, tracer, s_11_11, s_11_10);
        // D s_11_13: cast reint s_11_12 -> u64
        let s_11_13: u64 = (s_11_12.value() as u64);
        // D s_11_14: cast zx s_11_9 -> bv
        let s_11_14: Bits = Bits::new(s_11_9 as u128, 64u16);
        // D s_11_15: cast zx s_11_13 -> bv
        let s_11_15: Bits = Bits::new(s_11_13 as u128, 64u16);
        // D s_11_16: cast reint s_11_14 -> u128
        let s_11_16: u128 = (s_11_14.value() as u128);
        // D s_11_17: size-of s_11_14
        let s_11_17: u16 = s_11_14.length();
        // D s_11_18: cast reint s_11_15 -> u128
        let s_11_18: u128 = (s_11_15.value() as u128);
        // D s_11_19: size-of s_11_15
        let s_11_19: u16 = s_11_15.length();
        // D s_11_20: lsl s_11_16 s_11_19
        let s_11_20: u128 = s_11_16 << s_11_19;
        // D s_11_21: or s_11_20 s_11_18
        let s_11_21: u128 = ((s_11_20) | (s_11_18));
        // D s_11_22: add s_11_17 s_11_19
        let s_11_22: u16 = (s_11_17 + s_11_19);
        // D s_11_23: create-bits s_11_21 s_11_22
        let s_11_23: Bits = Bits::new(s_11_21, s_11_22);
        // D s_11_24: cast reint s_11_23 -> u128
        let s_11_24: u128 = (s_11_23.value() as u128);
        // C s_11_25: const #0s : i
        let s_11_25: i128 = 0;
        // D s_11_26: cast zx s_11_3 -> bv
        let s_11_26: Bits = Bits::new(s_11_3 as u128, 128u16);
        // D s_11_27: cast zx s_11_24 -> bv
        let s_11_27: Bits = Bits::new(s_11_24 as u128, 128u16);
        // C s_11_28: const #127s : i
        let s_11_28: i128 = 127;
        // C s_11_29: const #1u : u64
        let s_11_29: u64 = 1;
        // C s_11_30: cast zx s_11_29 -> bv
        let s_11_30: Bits = Bits::new(s_11_29 as u128, 64u16);
        // C s_11_31: lsl s_11_30 s_11_28
        let s_11_31: Bits = s_11_30 << s_11_28;
        // C s_11_32: sub s_11_31 s_11_30
        let s_11_32: Bits = ((s_11_31) - (s_11_30));
        // D s_11_33: and s_11_27 s_11_32
        let s_11_33: Bits = ((s_11_27) & (s_11_32));
        // D s_11_34: lsl s_11_33 s_11_25
        let s_11_34: Bits = s_11_33 << s_11_25;
        // C s_11_35: lsl s_11_32 s_11_25
        let s_11_35: Bits = s_11_32 << s_11_25;
        // C s_11_36: cmpl s_11_35
        let s_11_36: Bits = !s_11_35;
        // D s_11_37: and s_11_26 s_11_36
        let s_11_37: Bits = ((s_11_26) & (s_11_36));
        // D s_11_38: or s_11_37 s_11_34
        let s_11_38: Bits = ((s_11_37) | (s_11_34));
        // D s_11_39: cast reint s_11_38 -> u128
        let s_11_39: u128 = (s_11_38.value() as u128);
        // D s_11_40: call Mk_TTBR1_EL1_Type(s_11_39)
        let s_11_40: ProductType782ac6922b48c20d = Mk_TTBR1_EL1_Type(
            state,
            tracer,
            s_11_39,
        );
        // D s_11_41: call TTBR1_EL1_write(s_11_40)
        let s_11_41: () = TTBR1_EL1_write(state, tracer, s_11_40);
        // N s_11_42: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var __HCR_EL2_E2H:u8
        let s_12_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#137903 <= s_12_4
        fn_state.gs_137903 = s_12_4;
        // N s_12_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #432u : u32
        let s_13_0: u32 = 432;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call ELUsingAArch32(s_13_1)
        let s_13_2: bool = ELUsingAArch32(state, tracer, s_13_1);
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // D s_13_4: write-var gs#137902 <= s_13_3
        fn_state.gs_137902 = s_13_3;
        // N s_13_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var __HCR_EL2_E2H:u8
        let s_14_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // N s_14_5: branch s_14_4 b16 b15
        if s_14_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call Halted(s_16_0)
        let s_16_1: bool = Halted(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b40 b17
        if s_16_1 {
            return block_40(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#137909 <= s_17_0
        fn_state.gs_137909 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#137909:u8
        let s_18_0: bool = fn_state.gs_137909;
        // N s_18_1: branch s_18_0 b39 b19
        if s_18_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#137910 <= s_19_0
        fn_state.gs_137910 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#137910:u8
        let s_20_0: bool = fn_state.gs_137910;
        // N s_20_1: branch s_20_0 b38 b21
        if s_20_0 {
            return block_38(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#137911 <= s_21_0
        fn_state.gs_137911 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#137911:u8
        let s_22_0: bool = fn_state.gs_137911;
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
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#137912 <= s_23_0
        fn_state.gs_137912 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#137912:u8
        let s_24_0: bool = fn_state.gs_137912;
        // N s_24_1: branch s_24_0 b36 b25
        if s_24_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #424u : u32
        let s_25_0: u32 = 424;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // C s_25_2: const #2u : u8
        let s_25_2: u8 = 2;
        // D s_25_3: cmp-lt s_25_1 s_25_2
        let s_25_3: bool = ((s_25_1) < (s_25_2));
        // N s_25_4: branch s_25_3 b35 b26
        if s_25_3 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#137913 <= s_26_0
        fn_state.gs_137913 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#137913:u8
        let s_27_0: bool = fn_state.gs_137913;
        // N s_27_1: branch s_27_0 b29 b28
        if s_27_0 {
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
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call TTBR1_EL1_read(s_28_0)
        let s_28_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_28_0);
        // D s_28_2: write-var ga#240284 <= s_28_1
        fn_state.ga_240284 = s_28_1;
        // D s_28_3: read-var ga#240284.0:struct
        let s_28_3: u128 = fn_state.ga_240284._0;
        // C s_28_4: const #1s : i
        let s_28_4: i128 = 1;
        // D s_28_5: read-var t:i
        let s_28_5: i128 = fn_state.t;
        // D s_28_6: add s_28_5 s_28_4
        let s_28_6: i128 = (s_28_5 + s_28_4);
        // C s_28_7: const #64s : i64
        let s_28_7: i64 = 64;
        // D s_28_8: call X_read(s_28_6, s_28_7)
        let s_28_8: Bits = X_read(state, tracer, s_28_6, s_28_7);
        // D s_28_9: cast reint s_28_8 -> u64
        let s_28_9: u64 = (s_28_8.value() as u64);
        // C s_28_10: const #64s : i64
        let s_28_10: i64 = 64;
        // D s_28_11: read-var t:i
        let s_28_11: i128 = fn_state.t;
        // D s_28_12: call X_read(s_28_11, s_28_10)
        let s_28_12: Bits = X_read(state, tracer, s_28_11, s_28_10);
        // D s_28_13: cast reint s_28_12 -> u64
        let s_28_13: u64 = (s_28_12.value() as u64);
        // D s_28_14: cast zx s_28_9 -> bv
        let s_28_14: Bits = Bits::new(s_28_9 as u128, 64u16);
        // D s_28_15: cast zx s_28_13 -> bv
        let s_28_15: Bits = Bits::new(s_28_13 as u128, 64u16);
        // D s_28_16: cast reint s_28_14 -> u128
        let s_28_16: u128 = (s_28_14.value() as u128);
        // D s_28_17: size-of s_28_14
        let s_28_17: u16 = s_28_14.length();
        // D s_28_18: cast reint s_28_15 -> u128
        let s_28_18: u128 = (s_28_15.value() as u128);
        // D s_28_19: size-of s_28_15
        let s_28_19: u16 = s_28_15.length();
        // D s_28_20: lsl s_28_16 s_28_19
        let s_28_20: u128 = s_28_16 << s_28_19;
        // D s_28_21: or s_28_20 s_28_18
        let s_28_21: u128 = ((s_28_20) | (s_28_18));
        // D s_28_22: add s_28_17 s_28_19
        let s_28_22: u16 = (s_28_17 + s_28_19);
        // D s_28_23: create-bits s_28_21 s_28_22
        let s_28_23: Bits = Bits::new(s_28_21, s_28_22);
        // D s_28_24: cast reint s_28_23 -> u128
        let s_28_24: u128 = (s_28_23.value() as u128);
        // C s_28_25: const #0s : i
        let s_28_25: i128 = 0;
        // D s_28_26: cast zx s_28_3 -> bv
        let s_28_26: Bits = Bits::new(s_28_3 as u128, 128u16);
        // D s_28_27: cast zx s_28_24 -> bv
        let s_28_27: Bits = Bits::new(s_28_24 as u128, 128u16);
        // C s_28_28: const #127s : i
        let s_28_28: i128 = 127;
        // C s_28_29: const #1u : u64
        let s_28_29: u64 = 1;
        // C s_28_30: cast zx s_28_29 -> bv
        let s_28_30: Bits = Bits::new(s_28_29 as u128, 64u16);
        // C s_28_31: lsl s_28_30 s_28_28
        let s_28_31: Bits = s_28_30 << s_28_28;
        // C s_28_32: sub s_28_31 s_28_30
        let s_28_32: Bits = ((s_28_31) - (s_28_30));
        // D s_28_33: and s_28_27 s_28_32
        let s_28_33: Bits = ((s_28_27) & (s_28_32));
        // D s_28_34: lsl s_28_33 s_28_25
        let s_28_34: Bits = s_28_33 << s_28_25;
        // C s_28_35: lsl s_28_32 s_28_25
        let s_28_35: Bits = s_28_32 << s_28_25;
        // C s_28_36: cmpl s_28_35
        let s_28_36: Bits = !s_28_35;
        // D s_28_37: and s_28_26 s_28_36
        let s_28_37: Bits = ((s_28_26) & (s_28_36));
        // D s_28_38: or s_28_37 s_28_34
        let s_28_38: Bits = ((s_28_37) | (s_28_34));
        // D s_28_39: cast reint s_28_38 -> u128
        let s_28_39: u128 = (s_28_38.value() as u128);
        // D s_28_40: call Mk_TTBR1_EL1_Type(s_28_39)
        let s_28_40: ProductType782ac6922b48c20d = Mk_TTBR1_EL1_Type(
            state,
            tracer,
            s_28_39,
        );
        // D s_28_41: call TTBR1_EL1_write(s_28_40)
        let s_28_41: () = TTBR1_EL1_write(state, tracer, s_28_40);
        // N s_28_42: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call Halted(s_29_0)
        let s_29_1: bool = Halted(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b34 b30
        if s_29_1 {
            return block_34(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#137919 <= s_30_0
        fn_state.gs_137919 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#137919:u8
        let s_31_0: bool = fn_state.gs_137919;
        // N s_31_1: branch s_31_0 b33 b32
        if s_31_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #20u : u8
        let s_32_0: u8 = 20;
        // C s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 8u16);
        // C s_32_2: cast zx s_32_1 -> i
        let s_32_2: i128 = (s_32_1.value() as i128);
        // C s_32_3: cast reint s_32_2 -> i64
        let s_32_3: i64 = (s_32_2 as i64);
        // C s_32_4: cast zx s_32_3 -> i
        let s_32_4: i128 = (i128::try_from(s_32_3).unwrap());
        // C s_32_5: const #424u : u32
        let s_32_5: u32 = 424;
        // D s_32_6: read-reg s_32_5:u8
        let s_32_6: u8 = {
            let value = state.read_register::<u8>(s_32_5 as isize);
            tracer.read_register(s_32_5 as isize, value);
            value
        };
        // D s_32_7: call AArch64_SystemAccessTrap(s_32_6, s_32_4)
        let s_32_7: () = AArch64_SystemAccessTrap(state, tracer, s_32_6, s_32_4);
        // N s_32_8: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: panic
        panic!("{:?}", ());
        // N s_33_1: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var __EDSCR_SDD:u8
        let s_34_0: bool = fn_state.u__EDSCR_SDD;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: write-var gs#137919 <= s_34_4
        fn_state.gs_137919 = s_34_4;
        // N s_34_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var __SCR_EL3_D128En:u8
        let s_35_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 1u16);
        // C s_35_2: const #0u : u8
        let s_35_2: bool = false;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // D s_35_4: cmp-eq s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) == (s_35_3));
        // D s_35_5: write-var gs#137913 <= s_35_4
        fn_state.gs_137913 = s_35_4;
        // N s_35_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: panic
        panic!("{:?}", ());
        // N s_36_1: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var __SCR_EL3_D128En:u8
        let s_37_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #0u : u8
        let s_37_2: bool = false;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#137912 <= s_37_4
        fn_state.gs_137912 = s_37_4;
        // N s_37_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_38_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_38_1: call __IMPDEF_boolean(s_38_0)
        let s_38_1: bool = u__IMPDEF_boolean(state, tracer, s_38_0);
        // D s_38_2: write-var gs#137911 <= s_38_1
        fn_state.gs_137911 = s_38_1;
        // N s_38_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var __EDSCR_SDD:u8
        let s_39_0: bool = fn_state.u__EDSCR_SDD;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#137910 <= s_39_4
        fn_state.gs_137910 = s_39_4;
        // N s_39_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #424u : u32
        let s_40_0: u32 = 424;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // C s_40_2: const #2u : u8
        let s_40_2: u8 = 2;
        // D s_40_3: cmp-lt s_40_1 s_40_2
        let s_40_3: bool = ((s_40_1) < (s_40_2));
        // D s_40_4: write-var gs#137909 <= s_40_3
        fn_state.gs_137909 = s_40_3;
        // N s_40_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call EL2Enabled(s_41_0)
        let s_41_1: bool = EL2Enabled(state, tracer, s_41_0);
        // N s_41_2: branch s_41_1 b51 b42
        if s_41_1 {
            return block_51(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#137920 <= s_42_0
        fn_state.gs_137920 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#137920:u8
        let s_43_0: bool = fn_state.gs_137920;
        // N s_43_1: branch s_43_0 b50 b44
        if s_43_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call EL2Enabled(s_44_0)
        let s_44_1: bool = EL2Enabled(state, tracer, s_44_0);
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
        // D s_45_1: write-var gs#137921 <= s_45_0
        fn_state.gs_137921 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#137921:u8
        let s_46_0: bool = fn_state.gs_137921;
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
        // N s_47_0: panic
        panic!("{:?}", ());
        // N s_47_1: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #20u : u8
        let s_48_0: u8 = 20;
        // C s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 8u16);
        // C s_48_2: cast zx s_48_1 -> i
        let s_48_2: i128 = (s_48_1.value() as i128);
        // C s_48_3: cast reint s_48_2 -> i64
        let s_48_3: i64 = (s_48_2 as i64);
        // C s_48_4: cast zx s_48_3 -> i
        let s_48_4: i128 = (i128::try_from(s_48_3).unwrap());
        // C s_48_5: const #432u : u32
        let s_48_5: u32 = 432;
        // D s_48_6: read-reg s_48_5:u8
        let s_48_6: u8 = {
            let value = state.read_register::<u8>(s_48_5 as isize);
            tracer.read_register(s_48_5 as isize, value);
            value
        };
        // D s_48_7: call AArch64_SystemAccessTrap(s_48_6, s_48_4)
        let s_48_7: () = AArch64_SystemAccessTrap(state, tracer, s_48_6, s_48_4);
        // N s_48_8: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var __HCR_EL2_NV:u8
        let s_49_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 1u16);
        // C s_49_2: const #1u : u8
        let s_49_2: bool = true;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // D s_49_5: write-var gs#137921 <= s_49_4
        fn_state.gs_137921 = s_49_4;
        // N s_49_6: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #528u : u12
        let s_50_0: u16 = 528;
        // C s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 12u16);
        // C s_50_2: cast zx s_50_1 -> i
        let s_50_2: i128 = (s_50_1.value() as i128);
        // C s_50_3: cast reint s_50_2 -> i64
        let s_50_3: i64 = (s_50_2 as i64);
        // C s_50_4: const #128s : i64
        let s_50_4: i64 = 128;
        // C s_50_5: const #1s : i
        let s_50_5: i128 = 1;
        // D s_50_6: read-var t:i
        let s_50_6: i128 = fn_state.t;
        // D s_50_7: add s_50_6 s_50_5
        let s_50_7: i128 = (s_50_6 + s_50_5);
        // C s_50_8: const #64s : i64
        let s_50_8: i64 = 64;
        // D s_50_9: call X_read(s_50_7, s_50_8)
        let s_50_9: Bits = X_read(state, tracer, s_50_7, s_50_8);
        // D s_50_10: cast reint s_50_9 -> u64
        let s_50_10: u64 = (s_50_9.value() as u64);
        // C s_50_11: const #64s : i64
        let s_50_11: i64 = 64;
        // D s_50_12: read-var t:i
        let s_50_12: i128 = fn_state.t;
        // D s_50_13: call X_read(s_50_12, s_50_11)
        let s_50_13: Bits = X_read(state, tracer, s_50_12, s_50_11);
        // D s_50_14: cast reint s_50_13 -> u64
        let s_50_14: u64 = (s_50_13.value() as u64);
        // D s_50_15: cast zx s_50_10 -> bv
        let s_50_15: Bits = Bits::new(s_50_10 as u128, 64u16);
        // D s_50_16: cast zx s_50_14 -> bv
        let s_50_16: Bits = Bits::new(s_50_14 as u128, 64u16);
        // D s_50_17: cast reint s_50_15 -> u128
        let s_50_17: u128 = (s_50_15.value() as u128);
        // D s_50_18: size-of s_50_15
        let s_50_18: u16 = s_50_15.length();
        // D s_50_19: cast reint s_50_16 -> u128
        let s_50_19: u128 = (s_50_16.value() as u128);
        // D s_50_20: size-of s_50_16
        let s_50_20: u16 = s_50_16.length();
        // D s_50_21: lsl s_50_17 s_50_20
        let s_50_21: u128 = s_50_17 << s_50_20;
        // D s_50_22: or s_50_21 s_50_19
        let s_50_22: u128 = ((s_50_21) | (s_50_19));
        // D s_50_23: add s_50_18 s_50_20
        let s_50_23: u16 = (s_50_18 + s_50_20);
        // D s_50_24: create-bits s_50_22 s_50_23
        let s_50_24: Bits = Bits::new(s_50_22, s_50_23);
        // D s_50_25: cast reint s_50_24 -> u128
        let s_50_25: u128 = (s_50_24.value() as u128);
        // C s_50_26: cast zx s_50_3 -> i
        let s_50_26: i128 = (i128::try_from(s_50_3).unwrap());
        // D s_50_27: cast zx s_50_25 -> bv
        let s_50_27: Bits = Bits::new(s_50_25 as u128, 128u16);
        // D s_50_28: call NVMem_set__1(s_50_26, s_50_4, s_50_27)
        let s_50_28: () = NVMem_set__1(state, tracer, s_50_26, s_50_4, s_50_27);
        // N s_50_29: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #102552u : u32
        let s_51_0: u32 = 102552;
        // D s_51_1: read-reg s_51_0:struct
        let s_51_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call _get_HCR_EL2_Type_NV2(s_51_1)
        let s_51_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_51_1);
        // C s_51_3: const #102552u : u32
        let s_51_3: u32 = 102552;
        // D s_51_4: read-reg s_51_3:struct
        let s_51_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_51_3 as isize);
            tracer.read_register(s_51_3 as isize, value);
            value
        };
        // D s_51_5: call _get_HCR_EL2_Type_NV1(s_51_4)
        let s_51_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_51_4);
        // C s_51_6: const #102552u : u32
        let s_51_6: u32 = 102552;
        // D s_51_7: read-reg s_51_6:struct
        let s_51_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_51_6 as isize);
            tracer.read_register(s_51_6 as isize, value);
            value
        };
        // D s_51_8: call _get_HCR_EL2_Type_NV(s_51_7)
        let s_51_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_51_7);
        // D s_51_9: cast zx s_51_5 -> bv
        let s_51_9: Bits = Bits::new(s_51_5 as u128, 1u16);
        // D s_51_10: cast zx s_51_8 -> bv
        let s_51_10: Bits = Bits::new(s_51_8 as u128, 1u16);
        // D s_51_11: cast reint s_51_9 -> u128
        let s_51_11: u128 = (s_51_9.value() as u128);
        // D s_51_12: size-of s_51_9
        let s_51_12: u16 = s_51_9.length();
        // D s_51_13: cast reint s_51_10 -> u128
        let s_51_13: u128 = (s_51_10.value() as u128);
        // D s_51_14: size-of s_51_10
        let s_51_14: u16 = s_51_10.length();
        // D s_51_15: lsl s_51_11 s_51_14
        let s_51_15: u128 = s_51_11 << s_51_14;
        // D s_51_16: or s_51_15 s_51_13
        let s_51_16: u128 = ((s_51_15) | (s_51_13));
        // D s_51_17: add s_51_12 s_51_14
        let s_51_17: u16 = (s_51_12 + s_51_14);
        // D s_51_18: create-bits s_51_16 s_51_17
        let s_51_18: Bits = Bits::new(s_51_16, s_51_17);
        // D s_51_19: cast reint s_51_18 -> u8
        let s_51_19: u8 = (s_51_18.value() as u8);
        // D s_51_20: cast zx s_51_2 -> bv
        let s_51_20: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_21: cast zx s_51_19 -> bv
        let s_51_21: Bits = Bits::new(s_51_19 as u128, 2u16);
        // D s_51_22: cast reint s_51_20 -> u128
        let s_51_22: u128 = (s_51_20.value() as u128);
        // D s_51_23: size-of s_51_20
        let s_51_23: u16 = s_51_20.length();
        // D s_51_24: cast reint s_51_21 -> u128
        let s_51_24: u128 = (s_51_21.value() as u128);
        // D s_51_25: size-of s_51_21
        let s_51_25: u16 = s_51_21.length();
        // D s_51_26: lsl s_51_22 s_51_25
        let s_51_26: u128 = s_51_22 << s_51_25;
        // D s_51_27: or s_51_26 s_51_24
        let s_51_27: u128 = ((s_51_26) | (s_51_24));
        // D s_51_28: add s_51_23 s_51_25
        let s_51_28: u16 = (s_51_23 + s_51_25);
        // D s_51_29: create-bits s_51_27 s_51_28
        let s_51_29: Bits = Bits::new(s_51_27, s_51_28);
        // D s_51_30: cast reint s_51_29 -> u8
        let s_51_30: u8 = (s_51_29.value() as u8);
        // D s_51_31: cast zx s_51_30 -> bv
        let s_51_31: Bits = Bits::new(s_51_30 as u128, 3u16);
        // C s_51_32: const #5u : u8
        let s_51_32: u8 = 5;
        // C s_51_33: cast zx s_51_32 -> bv
        let s_51_33: Bits = Bits::new(s_51_32 as u128, 3u16);
        // D s_51_34: cmp-eq s_51_31 s_51_33
        let s_51_34: bool = ((s_51_31) == (s_51_33));
        // D s_51_35: write-var gs#137920 <= s_51_34
        fn_state.gs_137920 = s_51_34;
        // N s_51_36: jump b43
        return block_43(state, tracer, fn_state);
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
