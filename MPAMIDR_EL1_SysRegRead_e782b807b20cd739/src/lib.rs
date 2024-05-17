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
use u_get_MPAMHCR_EL2_Type_TRAP_MPAMIDR_EL1::*;
use EDSCR_read::*;
use u_get_MPAM3_EL3_Type_TRAPLOWER::*;
use u__get_MPAMIDR_EL1::*;
use X_set::*;
use MPAM3_EL3_read::*;
use u_get_MPAM2_EL2_Type_TIDR::*;
use Halted::*;
use u_get_EDSCR_Type_SDD::*;
use AArch64_SystemAccessTrap::*;
use EL2Enabled::*;
use u_get_MPAMIDR_EL1_Type_HAS_TIDR::*;
use u_get_MPAMIDR_EL1_Type_HAS_HCR::*;
use common::*;
pub fn MPAMIDR_EL1_SysRegRead_e782b807b20cd739<T: Tracer>(
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
        ga_71249: ProductType5c790c8ef59cc8b2,
        u__MPAM2_EL2_TIDR: bool,
        u__EDSCR_SDD: bool,
        gs_63346: bool,
        gs_63348: bool,
        gs_63347: bool,
        gs_63353: bool,
        u__MPAMIDR_EL1_HAS_TIDR: bool,
        gs_63349: bool,
        u__MPAMHCR_EL2_TRAP_MPAMIDR_EL1: bool,
        u__MPAMIDR_EL1_HAS_HCR: bool,
        gs_63350: bool,
        gs_63351: bool,
        gs_63344: bool,
        u__PSTATE_EL: u8,
        ga_71245: ProductType5c790c8ef59cc8b2,
        ga_71235: ProductType5c790c8ef59cc8b2,
        u__MPAM3_EL3_TRAPLOWER: bool,
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
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call MPAM3_EL3_read(s_0_3)
        let s_0_4: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_0_3);
        // S s_0_5: call _get_MPAM3_EL3_Type_TRAPLOWER(s_0_4)
        let s_0_5: bool = u_get_MPAM3_EL3_Type_TRAPLOWER(state, tracer, s_0_4);
        // D s_0_6: write-var __MPAM3_EL3_TRAPLOWER <= s_0_5
        fn_state.u__MPAM3_EL3_TRAPLOWER = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call EDSCR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_EDSCR_Type_SDD(s_0_8)
        let s_0_9: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_8);
        // D s_0_10: write-var __EDSCR_SDD <= s_0_9
        fn_state.u__EDSCR_SDD = s_0_9;
        // C s_0_11: const #11032u : u32
        let s_0_11: u32 = 11032;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_MPAMIDR_EL1_Type_HAS_HCR(s_0_12)
        let s_0_13: bool = u_get_MPAMIDR_EL1_Type_HAS_HCR(state, tracer, s_0_12);
        // D s_0_14: write-var __MPAMIDR_EL1_HAS_HCR <= s_0_13
        fn_state.u__MPAMIDR_EL1_HAS_HCR = s_0_13;
        // C s_0_15: const #17608u : u32
        let s_0_15: u32 = 17608;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_MPAMHCR_EL2_Type_TRAP_MPAMIDR_EL1(s_0_16)
        let s_0_17: bool = u_get_MPAMHCR_EL2_Type_TRAP_MPAMIDR_EL1(
            state,
            tracer,
            s_0_16,
        );
        // D s_0_18: write-var __MPAMHCR_EL2_TRAP_MPAMIDR_EL1 <= s_0_17
        fn_state.u__MPAMHCR_EL2_TRAP_MPAMIDR_EL1 = s_0_17;
        // C s_0_19: const #11032u : u32
        let s_0_19: u32 = 11032;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_MPAMIDR_EL1_Type_HAS_TIDR(s_0_20)
        let s_0_21: bool = u_get_MPAMIDR_EL1_Type_HAS_TIDR(state, tracer, s_0_20);
        // D s_0_22: write-var __MPAMIDR_EL1_HAS_TIDR <= s_0_21
        fn_state.u__MPAMIDR_EL1_HAS_TIDR = s_0_21;
        // C s_0_23: const #90504u : u32
        let s_0_23: u32 = 90504;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_MPAM2_EL2_Type_TIDR(s_0_24)
        let s_0_25: bool = u_get_MPAM2_EL2_Type_TIDR(state, tracer, s_0_24);
        // D s_0_26: write-var __MPAM2_EL2_TIDR <= s_0_25
        fn_state.u__MPAM2_EL2_TIDR = s_0_25;
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
        // N s_0_33: branch s_0_32 b44 b1
        if s_0_32 {
            return block_44(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b17 b2
        if s_1_5 {
            return block_17(state, tracer, fn_state);
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
        // C s_5_1: const #11032u : u32
        let s_5_1: u32 = 11032;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_MPAMIDR_EL1(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_MPAMIDR_EL1(
            state,
            tracer,
            s_5_2,
        );
        // D s_5_4: write-var ga#71249 <= s_5_3
        fn_state.ga_71249 = s_5_3;
        // D s_5_5: read-var ga#71249.0:struct
        let s_5_5: u64 = fn_state.ga_71249._0;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 64u16);
        // D s_5_7: read-var t:i
        let s_5_7: i128 = fn_state.t;
        // D s_5_8: call X_set(s_5_7, s_5_0, s_5_6)
        let s_5_8: () = X_set(state, tracer, s_5_7, s_5_0, s_5_6);
        // N s_5_9: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #424u : u32
        let s_6_0: u32 = 424;
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
        // N s_6_4: branch s_6_3 b16 b7
        if s_6_3 {
            return block_16(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#63344 <= s_7_0
        fn_state.gs_63344 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#63344:u8
        let s_8_0: bool = fn_state.gs_63344;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // C s_9_1: const #11032u : u32
        let s_9_1: u32 = 11032;
        // D s_9_2: read-reg s_9_1:struct
        let s_9_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_1 as isize);
            tracer.read_register(s_9_1 as isize, value);
            value
        };
        // D s_9_3: call __get_MPAMIDR_EL1(s_9_2)
        let s_9_3: ProductType5c790c8ef59cc8b2 = u__get_MPAMIDR_EL1(
            state,
            tracer,
            s_9_2,
        );
        // D s_9_4: write-var ga#71245 <= s_9_3
        fn_state.ga_71245 = s_9_3;
        // D s_9_5: read-var ga#71245.0:struct
        let s_9_5: u64 = fn_state.ga_71245._0;
        // D s_9_6: cast zx s_9_5 -> bv
        let s_9_6: Bits = Bits::new(s_9_5 as u128, 64u16);
        // D s_9_7: read-var t:i
        let s_9_7: i128 = fn_state.t;
        // D s_9_8: call X_set(s_9_7, s_9_0, s_9_6)
        let s_9_8: () = X_set(state, tracer, s_9_7, s_9_0, s_9_6);
        // N s_9_9: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call Halted(s_10_0)
        let s_10_1: bool = Halted(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b15 b11
        if s_10_1 {
            return block_15(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#63346 <= s_11_0
        fn_state.gs_63346 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#63346:u8
        let s_12_0: bool = fn_state.gs_63346;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #24u : u8
        let s_13_0: u8 = 24;
        // C s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 8u16);
        // C s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (s_13_1.value() as i128);
        // C s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #424u : u32
        let s_13_5: u32 = 424;
        // D s_13_6: read-reg s_13_5:u8
        let s_13_6: u8 = {
            let value = state.read_register::<u8>(s_13_5 as isize);
            tracer.read_register(s_13_5 as isize, value);
            value
        };
        // D s_13_7: call AArch64_SystemAccessTrap(s_13_6, s_13_4)
        let s_13_7: () = AArch64_SystemAccessTrap(state, tracer, s_13_6, s_13_4);
        // N s_13_8: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var __EDSCR_SDD:u8
        let s_15_0: bool = fn_state.u__EDSCR_SDD;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: write-var gs#63346 <= s_15_4
        fn_state.gs_63346 = s_15_4;
        // N s_15_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var __MPAM3_EL3_TRAPLOWER:u8
        let s_16_0: bool = fn_state.u__MPAM3_EL3_TRAPLOWER;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #1u : u8
        let s_16_2: bool = true;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: write-var gs#63344 <= s_16_4
        fn_state.gs_63344 = s_16_4;
        // N s_16_6: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_17_4: branch s_17_3 b43 b18
        if s_17_3 {
            return block_43(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#63347 <= s_18_0
        fn_state.gs_63347 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#63347:u8
        let s_19_0: bool = fn_state.gs_63347;
        // N s_19_1: branch s_19_0 b37 b20
        if s_19_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call EL2Enabled(s_20_0)
        let s_20_1: bool = EL2Enabled(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b36 b21
        if s_20_1 {
            return block_36(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#63348 <= s_21_0
        fn_state.gs_63348 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#63348:u8
        let s_22_0: bool = fn_state.gs_63348;
        // N s_22_1: branch s_22_0 b35 b23
        if s_22_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#63349 <= s_23_0
        fn_state.gs_63349 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#63349:u8
        let s_24_0: bool = fn_state.gs_63349;
        // N s_24_1: branch s_24_0 b34 b25
        if s_24_0 {
            return block_34(state, tracer, fn_state);
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
        // S s_25_1: call EL2Enabled(s_25_0)
        let s_25_1: bool = EL2Enabled(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b33 b26
        if s_25_1 {
            return block_33(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#63350 <= s_26_0
        fn_state.gs_63350 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#63350:u8
        let s_27_0: bool = fn_state.gs_63350;
        // N s_27_1: branch s_27_0 b32 b28
        if s_27_0 {
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
        // D s_28_1: write-var gs#63351 <= s_28_0
        fn_state.gs_63351 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#63351:u8
        let s_29_0: bool = fn_state.gs_63351;
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
        // C s_30_0: const #64s : i64
        let s_30_0: i64 = 64;
        // C s_30_1: const #11032u : u32
        let s_30_1: u32 = 11032;
        // D s_30_2: read-reg s_30_1:struct
        let s_30_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_1 as isize);
            tracer.read_register(s_30_1 as isize, value);
            value
        };
        // D s_30_3: call __get_MPAMIDR_EL1(s_30_2)
        let s_30_3: ProductType5c790c8ef59cc8b2 = u__get_MPAMIDR_EL1(
            state,
            tracer,
            s_30_2,
        );
        // D s_30_4: write-var ga#71235 <= s_30_3
        fn_state.ga_71235 = s_30_3;
        // D s_30_5: read-var ga#71235.0:struct
        let s_30_5: u64 = fn_state.ga_71235._0;
        // D s_30_6: cast zx s_30_5 -> bv
        let s_30_6: Bits = Bits::new(s_30_5 as u128, 64u16);
        // D s_30_7: read-var t:i
        let s_30_7: i128 = fn_state.t;
        // D s_30_8: call X_set(s_30_7, s_30_0, s_30_6)
        let s_30_8: () = X_set(state, tracer, s_30_7, s_30_0, s_30_6);
        // N s_30_9: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #24u : u8
        let s_31_0: u8 = 24;
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
        // D s_31_7: call AArch64_SystemAccessTrap(s_31_6, s_31_4)
        let s_31_7: () = AArch64_SystemAccessTrap(state, tracer, s_31_6, s_31_4);
        // N s_31_8: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __MPAM2_EL2_TIDR:u8
        let s_32_0: bool = fn_state.u__MPAM2_EL2_TIDR;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#63351 <= s_32_4
        fn_state.gs_63351 = s_32_4;
        // N s_32_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __MPAMIDR_EL1_HAS_TIDR:u8
        let s_33_0: bool = fn_state.u__MPAMIDR_EL1_HAS_TIDR;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#63350 <= s_33_4
        fn_state.gs_63350 = s_33_4;
        // N s_33_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #24u : u8
        let s_34_0: u8 = 24;
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
        // D s_35_0: read-var __MPAMHCR_EL2_TRAP_MPAMIDR_EL1:u8
        let s_35_0: bool = fn_state.u__MPAMHCR_EL2_TRAP_MPAMIDR_EL1;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 1u16);
        // C s_35_2: const #1u : u8
        let s_35_2: bool = true;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // D s_35_4: cmp-eq s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) == (s_35_3));
        // D s_35_5: write-var gs#63349 <= s_35_4
        fn_state.gs_63349 = s_35_4;
        // N s_35_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var __MPAMIDR_EL1_HAS_HCR:u8
        let s_36_0: bool = fn_state.u__MPAMIDR_EL1_HAS_HCR;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 1u16);
        // C s_36_2: const #1u : u8
        let s_36_2: bool = true;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: write-var gs#63348 <= s_36_4
        fn_state.gs_63348 = s_36_4;
        // N s_36_6: jump b22
        return block_22(state, tracer, fn_state);
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
        // N s_37_2: branch s_37_1 b42 b38
        if s_37_1 {
            return block_42(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#63353 <= s_38_0
        fn_state.gs_63353 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#63353:u8
        let s_39_0: bool = fn_state.gs_63353;
        // N s_39_1: branch s_39_0 b41 b40
        if s_39_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #24u : u8
        let s_40_0: u8 = 24;
        // C s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 8u16);
        // C s_40_2: cast zx s_40_1 -> i
        let s_40_2: i128 = (s_40_1.value() as i128);
        // C s_40_3: cast reint s_40_2 -> i64
        let s_40_3: i64 = (s_40_2 as i64);
        // C s_40_4: cast zx s_40_3 -> i
        let s_40_4: i128 = (i128::try_from(s_40_3).unwrap());
        // C s_40_5: const #424u : u32
        let s_40_5: u32 = 424;
        // D s_40_6: read-reg s_40_5:u8
        let s_40_6: u8 = {
            let value = state.read_register::<u8>(s_40_5 as isize);
            tracer.read_register(s_40_5 as isize, value);
            value
        };
        // D s_40_7: call AArch64_SystemAccessTrap(s_40_6, s_40_4)
        let s_40_7: () = AArch64_SystemAccessTrap(state, tracer, s_40_6, s_40_4);
        // N s_40_8: return
        return;
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
        // D s_42_0: read-var __EDSCR_SDD:u8
        let s_42_0: bool = fn_state.u__EDSCR_SDD;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 1u16);
        // C s_42_2: const #1u : u8
        let s_42_2: bool = true;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: write-var gs#63353 <= s_42_4
        fn_state.gs_63353 = s_42_4;
        // N s_42_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var __MPAM3_EL3_TRAPLOWER:u8
        let s_43_0: bool = fn_state.u__MPAM3_EL3_TRAPLOWER;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 1u16);
        // C s_43_2: const #1u : u8
        let s_43_2: bool = true;
        // C s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: write-var gs#63347 <= s_43_4
        fn_state.gs_63347 = s_43_4;
        // N s_43_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: panic
        panic!("{:?}", ());
        // N s_44_1: return
        return;
    }
}
