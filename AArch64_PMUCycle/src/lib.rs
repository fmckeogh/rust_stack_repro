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
use CheckForPMUOverflow::*;
use PMUEvent::*;
use HavePMUv3::*;
use GetNumEventCounters::*;
use u__id::*;
use CountPMUEvents::*;
use AArch64_IncrementCycleCounter::*;
use AArch64_IncrementEventCounter::*;
use ShouldBRBEFreeze::*;
use BRBEFreeze::*;
use u_get_MDCR_EL2_Type_HPME::*;
use u_get_PMCR_EL0_Type_E::*;
use common::*;
pub fn AArch64_PMUCycle<T: Tracer>(state: &mut State, tracer: &T, gs_25468: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        counters: i128,
        gs_25494: bool,
        idx: i64,
        gs_25489: bool,
        gs_25481: i64,
        gs_25486: bool,
        gs_25469: bool,
        gs_25468: (),
    }
    let fn_state = FunctionState {
        gs_25468,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #21016u : u32
        let s_0_0: u32 = 21016;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_PMCR_EL0_Type_E(s_0_1)
        let s_0_2: bool = u_get_PMCR_EL0_Type_E(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 1u16);
        // D s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // N s_0_7: branch s_0_6 b27 b1
        if s_0_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#25469 <= s_1_0
        fn_state.gs_25469 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25469:u8
        let s_2_0: bool = fn_state.gs_25469;
        // N s_2_1: branch s_2_0 b24 b3
        if s_2_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HavePMUv3(s_3_0)
        let s_3_1: bool = HavePMUv3(state, tracer, s_3_0);
        // S s_3_2: not s_3_1
        let s_3_2: bool = !s_3_1;
        // N s_3_3: branch s_3_2 b23 b4
        if s_3_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64u : u32
        let s_4_0: u32 = 64;
        // D s_4_1: read-reg s_4_0:u16
        let s_4_1: u16 = {
            let value = state.read_register::<u16>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call PMUEvent(s_4_1)
        let s_4_2: () = PMUEvent(state, tracer, s_4_1);
        // C s_4_3: const #() : ()
        let s_4_3: () = ();
        // S s_4_4: call GetNumEventCounters(s_4_3)
        let s_4_4: i128 = GetNumEventCounters(state, tracer, s_4_3);
        // D s_4_5: write-var counters <= s_4_4
        fn_state.counters = s_4_4;
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // D s_4_7: read-var counters:i
        let s_4_7: i128 = fn_state.counters;
        // D s_4_8: call neq_int(s_4_7, s_4_6)
        let s_4_8: bool = neq_int(state, tracer, s_4_7, s_4_6);
        // N s_4_9: branch s_4_8 b7 b5
        if s_4_8 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call AArch64_IncrementCycleCounter(s_6_0)
        let s_6_1: () = AArch64_IncrementCycleCounter(state, tracer, s_6_0);
        // C s_6_2: const #() : ()
        let s_6_2: () = ();
        // S s_6_3: call CheckForPMUOverflow(s_6_2)
        let s_6_3: () = CheckForPMUOverflow(state, tracer, s_6_2);
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i64
        let s_7_0: i64 = 0;
        // C s_7_1: const #1s : i
        let s_7_1: i128 = 1;
        // D s_7_2: read-var counters:i
        let s_7_2: i128 = fn_state.counters;
        // D s_7_3: sub s_7_2 s_7_1
        let s_7_3: i128 = ((s_7_2) - (s_7_1));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: write-var gs#25481 <= s_7_4
        fn_state.gs_25481 = s_7_4;
        // D s_7_6: write-var idx <= s_7_0
        fn_state.idx = s_7_0;
        // N s_7_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var idx:i64
        let s_8_0: i64 = fn_state.idx;
        // D s_8_1: read-var gs#25481:i64
        let s_8_1: i64 = fn_state.gs_25481;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b22 b9
        if s_8_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var idx:i64
        let s_9_0: i64 = fn_state.idx;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call __id(s_9_1)
        let s_9_2: i128 = u__id(state, tracer, s_9_1);
        // C s_9_3: const #0s : i
        let s_9_3: i128 = 0;
        // D s_9_4: cmp-le s_9_3 s_9_2
        let s_9_4: bool = ((s_9_3) <= (s_9_2));
        // N s_9_5: branch s_9_4 b21 b10
        if s_9_4 {
            return block_21(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#25486 <= s_10_0
        fn_state.gs_25486 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#25486:u8
        let s_11_0: bool = fn_state.gs_25486;
        // N s_11_1: assert s_11_0
        let s_11_1: () = assert!(s_11_0);
        // D s_11_2: read-var idx:i64
        let s_11_2: i64 = fn_state.idx;
        // D s_11_3: call CountPMUEvents(s_11_2)
        let s_11_3: bool = CountPMUEvents(state, tracer, s_11_2);
        // N s_11_4: branch s_11_3 b17 b12
        if s_11_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var idx:i64
        let s_13_0: i64 = fn_state.idx;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call __id(s_13_1)
        let s_13_2: i128 = u__id(state, tracer, s_13_1);
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: const #0s : i
        let s_13_4: i128 = 0;
        // D s_13_5: cast zx s_13_3 -> i
        let s_13_5: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_6: cmp-le s_13_4 s_13_5
        let s_13_6: bool = ((s_13_4) <= (s_13_5));
        // N s_13_7: branch s_13_6 b16 b14
        if s_13_6 {
            return block_16(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#25489 <= s_14_0
        fn_state.gs_25489 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#25489:u8
        let s_15_0: bool = fn_state.gs_25489;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // C s_15_2: const #0s : i
        let s_15_2: i128 = 0;
        // C s_15_3: const #89632u : u32
        let s_15_3: u32 = 89632;
        // D s_15_4: read-reg s_15_3:[i; 31]
        let s_15_4: [i128; 31usize] = {
            let value = state.read_register::<[i128; 31usize]>(s_15_3 as isize);
            tracer.read_register(s_15_3 as isize, value);
            value
        };
        // D s_15_5: read-var idx:i64
        let s_15_5: i64 = fn_state.idx;
        // D s_15_6: cast zx s_15_5 -> i
        let s_15_6: i128 = (i128::try_from(s_15_5).unwrap());
        // D s_15_7: mutate-element s_15_4[s_15_6] <= s_15_2
        let s_15_7: [i128; 31usize] = {
            let mut local = s_15_4.clone();
            local[(s_15_6) as usize] = s_15_2;
            local
        };
        // D s_15_8: cast cvt s_15_7 -> [i; 0]
        let s_15_8: alloc::vec::Vec<i128> = alloc::vec::Vec::from(s_15_7);
        // D s_15_9: cast cvt s_15_8 -> [i; 31]
        let s_15_9: [i128; 31usize] = {
            let mut buf = [Default::default(); 31usize];
            buf.copy_from_slice(&s_15_8);
            buf
        };
        // C s_15_10: const #89632u : u32
        let s_15_10: u32 = 89632;
        // N s_15_11: write-reg s_15_10 <= s_15_9
        let s_15_11: () = {
            state.write_register::<[i128; 31usize]>(s_15_10 as isize, s_15_9);
            tracer.write_register(s_15_10 as isize, s_15_9);
        };
        // D s_15_12: read-var idx:i64
        let s_15_12: i64 = fn_state.idx;
        // C s_15_13: const #1s : i64
        let s_15_13: i64 = 1;
        // D s_15_14: add s_15_12 s_15_13
        let s_15_14: i64 = (s_15_12 + s_15_13);
        // D s_15_15: write-var idx <= s_15_14
        fn_state.idx = s_15_14;
        // N s_15_16: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var idx:i64
        let s_16_0: i64 = fn_state.idx;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call __id(s_16_1)
        let s_16_2: i128 = u__id(state, tracer, s_16_1);
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: const #31s : i
        let s_16_4: i128 = 31;
        // D s_16_5: cast zx s_16_3 -> i
        let s_16_5: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_6: cmp-lt s_16_5 s_16_4
        let s_16_6: bool = ((s_16_5) < (s_16_4));
        // D s_16_7: write-var gs#25489 <= s_16_6
        fn_state.gs_25489 = s_16_6;
        // N s_16_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var idx:i64
        let s_17_0: i64 = fn_state.idx;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: call __id(s_17_1)
        let s_17_2: i128 = u__id(state, tracer, s_17_1);
        // D s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // C s_17_4: const #0s : i
        let s_17_4: i128 = 0;
        // D s_17_5: cast zx s_17_3 -> i
        let s_17_5: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_6: cmp-le s_17_4 s_17_5
        let s_17_6: bool = ((s_17_4) <= (s_17_5));
        // N s_17_7: branch s_17_6 b20 b18
        if s_17_6 {
            return block_20(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#25494 <= s_18_0
        fn_state.gs_25494 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#25494:u8
        let s_19_0: bool = fn_state.gs_25494;
        // N s_19_1: assert s_19_0
        let s_19_1: () = assert!(s_19_0);
        // C s_19_2: const #89632u : u32
        let s_19_2: u32 = 89632;
        // D s_19_3: read-reg s_19_2:[i; 31]
        let s_19_3: [i128; 31usize] = {
            let value = state.read_register::<[i128; 31usize]>(s_19_2 as isize);
            tracer.read_register(s_19_2 as isize, value);
            value
        };
        // D s_19_4: read-var idx:i64
        let s_19_4: i64 = fn_state.idx;
        // D s_19_5: cast zx s_19_4 -> i
        let s_19_5: i128 = (i128::try_from(s_19_4).unwrap());
        // D s_19_6: read-element s_19_3[s_19_5]
        let s_19_6: i128 = s_19_3[(s_19_5) as usize];
        // D s_19_7: read-var idx:i64
        let s_19_7: i64 = fn_state.idx;
        // D s_19_8: call AArch64_IncrementEventCounter(s_19_7, s_19_6)
        let s_19_8: () = AArch64_IncrementEventCounter(state, tracer, s_19_7, s_19_6);
        // N s_19_9: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var idx:i64
        let s_20_0: i64 = fn_state.idx;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: call __id(s_20_1)
        let s_20_2: i128 = u__id(state, tracer, s_20_1);
        // D s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: const #31s : i
        let s_20_4: i128 = 31;
        // D s_20_5: cast zx s_20_3 -> i
        let s_20_5: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_6: cmp-lt s_20_5 s_20_4
        let s_20_6: bool = ((s_20_5) < (s_20_4));
        // D s_20_7: write-var gs#25494 <= s_20_6
        fn_state.gs_25494 = s_20_6;
        // N s_20_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var idx:i64
        let s_21_0: i64 = fn_state.idx;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: call __id(s_21_1)
        let s_21_2: i128 = u__id(state, tracer, s_21_1);
        // C s_21_3: const #32s : i
        let s_21_3: i128 = 32;
        // D s_21_4: cmp-le s_21_2 s_21_3
        let s_21_4: bool = ((s_21_2) <= (s_21_3));
        // D s_21_5: write-var gs#25486 <= s_21_4
        fn_state.gs_25486 = s_21_4;
        // N s_21_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call ShouldBRBEFreeze(s_24_0)
        let s_24_1: bool = ShouldBRBEFreeze(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b26 b25
        if s_24_1 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call BRBEFreeze(s_26_0)
        let s_26_1: () = BRBEFreeze(state, tracer, s_26_0);
        // N s_26_2: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #104880u : u32
        let s_27_0: u32 = 104880;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_MDCR_EL2_Type_HPME(s_27_1)
        let s_27_2: bool = u_get_MDCR_EL2_Type_HPME(state, tracer, s_27_1);
        // D s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // C s_27_4: const #0u : u8
        let s_27_4: bool = false;
        // C s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 1u16);
        // D s_27_6: cmp-eq s_27_3 s_27_5
        let s_27_6: bool = ((s_27_3) == (s_27_5));
        // D s_27_7: write-var gs#25469 <= s_27_6
        fn_state.gs_25469 = s_27_6;
        // N s_27_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
