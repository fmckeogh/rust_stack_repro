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
use neq_int::*;
use IncrementInstructionCounter::*;
use GetNumEventCounters::*;
use PMUEvent__2::*;
use SPEEvent::*;
use HaveAArch64::*;
use HavePMUv3ICNTR::*;
use u_get_MDCR_EL2_Type_HPME::*;
use u_get_PMCR_EL0_Type_E::*;
use common::*;
pub fn PMUEvent__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    pmuevent: u16,
    increment_name: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_2707: i64,
        idx: i64,
        gs_2701: bool,
        gs_2700: bool,
        gs_2695: bool,
        counters: i128,
        pmuevent: u16,
        increment_name: i128,
    }
    let fn_state = FunctionState {
        pmuevent,
        increment_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #22416u : u32
        let s_0_0: u32 = 22416;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: bool = {
            let value = state.read_register::<bool>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // N s_0_2: branch s_0_1 b22 b1
        if s_0_1 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #21016u : u32
        let s_2_0: u32 = 21016;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_PMCR_EL0_Type_E(s_2_1)
        let s_2_2: bool = u_get_PMCR_EL0_Type_E(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #0u : u8
        let s_2_4: bool = false;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b21 b3
        if s_2_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#2695 <= s_3_0
        fn_state.gs_2695 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#2695:u8
        let s_4_0: bool = fn_state.gs_2695;
        // N s_4_1: branch s_4_0 b20 b5
        if s_4_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call GetNumEventCounters(s_5_0)
        let s_5_1: i128 = GetNumEventCounters(state, tracer, s_5_0);
        // D s_5_2: write-var counters <= s_5_1
        fn_state.counters = s_5_1;
        // C s_5_3: const #0s : i
        let s_5_3: i128 = 0;
        // D s_5_4: read-var counters:i
        let s_5_4: i128 = fn_state.counters;
        // D s_5_5: call neq_int(s_5_4, s_5_3)
        let s_5_5: bool = neq_int(state, tracer, s_5_4, s_5_3);
        // N s_5_6: branch s_5_5 b16 b6
        if s_5_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call HaveAArch64(s_7_0)
        let s_7_1: bool = HaveAArch64(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b15 b8
        if s_7_1 {
            return block_15(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#2700 <= s_8_0
        fn_state.gs_2700 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#2700:u8
        let s_9_0: bool = fn_state.gs_2700;
        // N s_9_1: branch s_9_0 b14 b10
        if s_9_0 {
            return block_14(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#2701 <= s_10_0
        fn_state.gs_2701 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#2701:u8
        let s_11_0: bool = fn_state.gs_2701;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var increment_name:i
        let s_13_0: i128 = fn_state.increment_name;
        // D s_13_1: call IncrementInstructionCounter(s_13_0)
        let s_13_1: () = IncrementInstructionCounter(state, tracer, s_13_0);
        // N s_13_2: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var pmuevent:u16
        let s_14_0: u16 = fn_state.pmuevent;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 16u16);
        // C s_14_2: const #40u : u32
        let s_14_2: u32 = 40;
        // D s_14_3: read-reg s_14_2:u16
        let s_14_3: u16 = {
            let value = state.read_register::<u16>(s_14_2 as isize);
            tracer.read_register(s_14_2 as isize, value);
            value
        };
        // D s_14_4: cast zx s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 16u16);
        // D s_14_5: cmp-eq s_14_1 s_14_4
        let s_14_5: bool = ((s_14_1) == (s_14_4));
        // D s_14_6: write-var gs#2701 <= s_14_5
        fn_state.gs_2701 = s_14_5;
        // N s_14_7: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HavePMUv3ICNTR(s_15_0)
        let s_15_1: bool = HavePMUv3ICNTR(state, tracer, s_15_0);
        // D s_15_2: write-var gs#2700 <= s_15_1
        fn_state.gs_2700 = s_15_1;
        // N s_15_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0s : i64
        let s_16_0: i64 = 0;
        // C s_16_1: const #1s : i
        let s_16_1: i128 = 1;
        // D s_16_2: read-var counters:i
        let s_16_2: i128 = fn_state.counters;
        // D s_16_3: sub s_16_2 s_16_1
        let s_16_3: i128 = ((s_16_2) - (s_16_1));
        // D s_16_4: cast reint s_16_3 -> i64
        let s_16_4: i64 = (s_16_3 as i64);
        // D s_16_5: write-var gs#2707 <= s_16_4
        fn_state.gs_2707 = s_16_4;
        // D s_16_6: write-var idx <= s_16_0
        fn_state.idx = s_16_0;
        // N s_16_7: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var idx:i64
        let s_17_0: i64 = fn_state.idx;
        // D s_17_1: read-var gs#2707:i64
        let s_17_1: i64 = fn_state.gs_2707;
        // D s_17_2: cmp-gt s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) > (s_17_1));
        // N s_17_3: branch s_17_2 b19 b18
        if s_17_2 {
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
        // D s_18_0: read-var idx:i64
        let s_18_0: i64 = fn_state.idx;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: read-var pmuevent:u16
        let s_18_2: u16 = fn_state.pmuevent;
        // D s_18_3: read-var increment_name:i
        let s_18_3: i128 = fn_state.increment_name;
        // D s_18_4: call PMUEvent__2(s_18_2, s_18_3, s_18_1)
        let s_18_4: () = PMUEvent__2(state, tracer, s_18_2, s_18_3, s_18_1);
        // D s_18_5: read-var idx:i64
        let s_18_5: i64 = fn_state.idx;
        // C s_18_6: const #1s : i64
        let s_18_6: i64 = 1;
        // D s_18_7: add s_18_5 s_18_6
        let s_18_7: i64 = (s_18_5 + s_18_6);
        // D s_18_8: write-var idx <= s_18_7
        fn_state.idx = s_18_7;
        // N s_18_9: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #104880u : u32
        let s_21_0: u32 = 104880;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_MDCR_EL2_Type_HPME(s_21_1)
        let s_21_2: bool = u_get_MDCR_EL2_Type_HPME(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #0u : u8
        let s_21_4: bool = false;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // D s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // D s_21_7: write-var gs#2695 <= s_21_6
        fn_state.gs_2695 = s_21_6;
        // N s_21_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var pmuevent:u16
        let s_22_0: u16 = fn_state.pmuevent;
        // D s_22_1: call SPEEvent(s_22_0)
        let s_22_1: () = SPEEvent(state, tracer, s_22_0);
        // N s_22_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
