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
use X_set::*;
use u__get_DISR_EL1::*;
use Halted::*;
use u_get_SCR_EL3_Type_EA::*;
use Zeros::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_AMO::*;
use u__get_VDISR_EL2::*;
use common::*;
pub fn VDISR_EL2_SysRegRead_07ff39acd959bbaf<T: Tracer>(
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
        u__HCR_EL2_AMO: bool,
        u__SCR_EL3_EA: bool,
        ga_100016: ProductType5c790c8ef59cc8b2,
        gs_74554: bool,
        ga_100006: ProductType5c790c8ef59cc8b2,
        gs_74552: bool,
        gs_74548: bool,
        ga_100020: ProductType5c790c8ef59cc8b2,
        u__PSTATE_EL: u8,
        gs_74553: bool,
        gs_74547: bool,
        ga_99997: ProductType5c790c8ef59cc8b2,
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
        // D s_0_5: call _get_HCR_EL2_Type_AMO(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_AMO <= s_0_5
        fn_state.u__HCR_EL2_AMO = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_EA(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_EA(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_EA <= s_0_9
        fn_state.u__SCR_EL3_EA = s_0_9;
        // D s_0_11: read-var __PSTATE_EL:u8
        let s_0_11: u8 = fn_state.u__PSTATE_EL;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_13: const #448u : u32
        let s_0_13: u32 = 448;
        // D s_0_14: read-reg s_0_13:u8
        let s_0_14: u8 = {
            let value = state.read_register::<u8>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cmp-eq s_0_12 s_0_15
        let s_0_16: bool = ((s_0_12) == (s_0_15));
        // N s_0_17: branch s_0_16 b29 b1
        if s_0_16 {
            return block_29(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b15 b2
        if s_1_5 {
            return block_15(state, tracer, fn_state);
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
        // C s_5_1: const #13632u : u32
        let s_5_1: u32 = 13632;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_DISR_EL1(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_DISR_EL1(state, tracer, s_5_2);
        // D s_5_4: write-var ga#100020 <= s_5_3
        fn_state.ga_100020 = s_5_3;
        // D s_5_5: read-var ga#100020.0:struct
        let s_5_5: u64 = fn_state.ga_100020._0;
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
        // N s_6_4: branch s_6_3 b14 b7
        if s_6_3 {
            return block_14(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#74547 <= s_7_0
        fn_state.gs_74547 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#74547:u8
        let s_8_0: bool = fn_state.gs_74547;
        // N s_8_1: branch s_8_0 b13 b9
        if s_8_0 {
            return block_13(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#74548 <= s_9_0
        fn_state.gs_74548 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#74548:u8
        let s_10_0: bool = fn_state.gs_74548;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // C s_11_1: const #13632u : u32
        let s_11_1: u32 = 13632;
        // D s_11_2: read-reg s_11_1:struct
        let s_11_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_1 as isize);
            tracer.read_register(s_11_1 as isize, value);
            value
        };
        // D s_11_3: call __get_DISR_EL1(s_11_2)
        let s_11_3: ProductType5c790c8ef59cc8b2 = u__get_DISR_EL1(state, tracer, s_11_2);
        // D s_11_4: write-var ga#100016 <= s_11_3
        fn_state.ga_100016 = s_11_3;
        // D s_11_5: read-var ga#100016.0:struct
        let s_11_5: u64 = fn_state.ga_100016._0;
        // D s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 64u16);
        // D s_11_7: read-var t:i
        let s_11_7: i128 = fn_state.t;
        // D s_11_8: call X_set(s_11_7, s_11_0, s_11_6)
        let s_11_8: () = X_set(state, tracer, s_11_7, s_11_0, s_11_6);
        // N s_11_9: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #64s : i64
        let s_12_0: i64 = 64;
        // C s_12_1: const #64s : i
        let s_12_1: i128 = 64;
        // S s_12_2: call Zeros(s_12_1)
        let s_12_2: Bits = Zeros(state, tracer, s_12_1);
        // S s_12_3: cast reint s_12_2 -> u64
        let s_12_3: u64 = (s_12_2.value() as u64);
        // S s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 64u16);
        // D s_12_5: read-var t:i
        let s_12_5: i128 = fn_state.t;
        // D s_12_6: call X_set(s_12_5, s_12_0, s_12_4)
        let s_12_6: () = X_set(state, tracer, s_12_5, s_12_0, s_12_4);
        // N s_12_7: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var __SCR_EL3_EA:u8
        let s_13_0: bool = fn_state.u__SCR_EL3_EA;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #1u : u8
        let s_13_2: bool = true;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: write-var gs#74548 <= s_13_4
        fn_state.gs_74548 = s_13_4;
        // N s_13_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call Halted(s_14_0)
        let s_14_1: bool = Halted(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // D s_14_3: write-var gs#74547 <= s_14_2
        fn_state.gs_74547 = s_14_2;
        // N s_14_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call EL2Enabled(s_15_0)
        let s_15_1: bool = EL2Enabled(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b28 b16
        if s_15_1 {
            return block_28(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#74552 <= s_16_0
        fn_state.gs_74552 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#74552:u8
        let s_17_0: bool = fn_state.gs_74552;
        // N s_17_1: branch s_17_0 b27 b18
        if s_17_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #424u : u32
        let s_18_0: u32 = 424;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // C s_18_2: const #2u : u8
        let s_18_2: u8 = 2;
        // D s_18_3: cmp-lt s_18_1 s_18_2
        let s_18_3: bool = ((s_18_1) < (s_18_2));
        // N s_18_4: branch s_18_3 b26 b19
        if s_18_3 {
            return block_26(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#74553 <= s_19_0
        fn_state.gs_74553 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#74553:u8
        let s_20_0: bool = fn_state.gs_74553;
        // N s_20_1: branch s_20_0 b25 b21
        if s_20_0 {
            return block_25(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#74554 <= s_21_0
        fn_state.gs_74554 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#74554:u8
        let s_22_0: bool = fn_state.gs_74554;
        // N s_22_1: branch s_22_0 b24 b23
        if s_22_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #64s : i64
        let s_23_0: i64 = 64;
        // C s_23_1: const #13632u : u32
        let s_23_1: u32 = 13632;
        // D s_23_2: read-reg s_23_1:struct
        let s_23_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_1 as isize);
            tracer.read_register(s_23_1 as isize, value);
            value
        };
        // D s_23_3: call __get_DISR_EL1(s_23_2)
        let s_23_3: ProductType5c790c8ef59cc8b2 = u__get_DISR_EL1(state, tracer, s_23_2);
        // D s_23_4: write-var ga#100006 <= s_23_3
        fn_state.ga_100006 = s_23_3;
        // D s_23_5: read-var ga#100006.0:struct
        let s_23_5: u64 = fn_state.ga_100006._0;
        // D s_23_6: cast zx s_23_5 -> bv
        let s_23_6: Bits = Bits::new(s_23_5 as u128, 64u16);
        // D s_23_7: read-var t:i
        let s_23_7: i128 = fn_state.t;
        // D s_23_8: call X_set(s_23_7, s_23_0, s_23_6)
        let s_23_8: () = X_set(state, tracer, s_23_7, s_23_0, s_23_6);
        // N s_23_9: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #64s : i64
        let s_24_0: i64 = 64;
        // C s_24_1: const #64s : i
        let s_24_1: i128 = 64;
        // S s_24_2: call Zeros(s_24_1)
        let s_24_2: Bits = Zeros(state, tracer, s_24_1);
        // S s_24_3: cast reint s_24_2 -> u64
        let s_24_3: u64 = (s_24_2.value() as u64);
        // S s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 64u16);
        // D s_24_5: read-var t:i
        let s_24_5: i128 = fn_state.t;
        // D s_24_6: call X_set(s_24_5, s_24_0, s_24_4)
        let s_24_6: () = X_set(state, tracer, s_24_5, s_24_0, s_24_4);
        // N s_24_7: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __SCR_EL3_EA:u8
        let s_25_0: bool = fn_state.u__SCR_EL3_EA;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#74554 <= s_25_4
        fn_state.gs_74554 = s_25_4;
        // N s_25_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call Halted(s_26_0)
        let s_26_1: bool = Halted(state, tracer, s_26_0);
        // S s_26_2: not s_26_1
        let s_26_2: bool = !s_26_1;
        // D s_26_3: write-var gs#74553 <= s_26_2
        fn_state.gs_74553 = s_26_2;
        // N s_26_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #64s : i64
        let s_27_0: i64 = 64;
        // C s_27_1: const #19328u : u32
        let s_27_1: u32 = 19328;
        // D s_27_2: read-reg s_27_1:struct
        let s_27_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_1 as isize);
            tracer.read_register(s_27_1 as isize, value);
            value
        };
        // D s_27_3: call __get_VDISR_EL2(s_27_2)
        let s_27_3: ProductType5c790c8ef59cc8b2 = u__get_VDISR_EL2(
            state,
            tracer,
            s_27_2,
        );
        // D s_27_4: write-var ga#99997 <= s_27_3
        fn_state.ga_99997 = s_27_3;
        // D s_27_5: read-var ga#99997.0:struct
        let s_27_5: u64 = fn_state.ga_99997._0;
        // D s_27_6: cast zx s_27_5 -> bv
        let s_27_6: Bits = Bits::new(s_27_5 as u128, 64u16);
        // D s_27_7: read-var t:i
        let s_27_7: i128 = fn_state.t;
        // D s_27_8: call X_set(s_27_7, s_27_0, s_27_6)
        let s_27_8: () = X_set(state, tracer, s_27_7, s_27_0, s_27_6);
        // N s_27_9: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var __HCR_EL2_AMO:u8
        let s_28_0: bool = fn_state.u__HCR_EL2_AMO;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 1u16);
        // C s_28_2: const #1u : u8
        let s_28_2: bool = true;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: write-var gs#74552 <= s_28_4
        fn_state.gs_74552 = s_28_4;
        // N s_28_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: panic
        panic!("{:?}", ());
        // N s_29_1: return
        return;
    }
}
