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
use PMUEvent::*;
use HavePMUv3::*;
use GetNumEventCounters::*;
use AArch64_PMUCycle::*;
use HDCR_read::*;
use BRBEFreeze::*;
use neq_int::*;
use AArch32_IncrementCycleCounter::*;
use u_get_PMCR_Type_E::*;
use CheckForPMUOverflow::*;
use u__id::*;
use CountPMUEvents::*;
use PMCR_read::*;
use u_get_HDCR_Type_HPME::*;
use HaveAArch64::*;
use AArch32_IncrementEventCounter::*;
use ShouldBRBEFreeze::*;
use common::*;
pub fn AArch32_PMUCycle<T: Tracer>(state: &mut State, tracer: &T, gs_32264: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_32284: bool,
        gs_32287: bool,
        idx: i64,
        gs_32292: bool,
        gs_32279: i64,
        gs_32265: bool,
        counters: i128,
        gs_32264: (),
    }
    let fn_state = FunctionState {
        gs_32264,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call PMCR_read(s_0_0)
        let s_0_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_0_0);
        // S s_0_2: call _get_PMCR_Type_E(s_0_1)
        let s_0_2: bool = u_get_PMCR_Type_E(state, tracer, s_0_1);
        // S s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 1u16);
        // S s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // N s_0_7: branch s_0_6 b29 b1
        if s_0_6 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#32265 <= s_1_0
        fn_state.gs_32265 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#32265:u8
        let s_2_0: bool = fn_state.gs_32265;
        // N s_2_1: branch s_2_0 b26 b3
        if s_2_0 {
            return block_26(state, tracer, fn_state);
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
        // S s_3_1: call HaveAArch64(s_3_0)
        let s_3_1: bool = HaveAArch64(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b25 b4
        if s_3_1 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call HavePMUv3(s_4_0)
        let s_4_1: bool = HavePMUv3(state, tracer, s_4_0);
        // S s_4_2: not s_4_1
        let s_4_2: bool = !s_4_1;
        // N s_4_3: branch s_4_2 b24 b5
        if s_4_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64u : u32
        let s_5_0: u32 = 64;
        // D s_5_1: read-reg s_5_0:u16
        let s_5_1: u16 = {
            let value = state.read_register::<u16>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call PMUEvent(s_5_1)
        let s_5_2: () = PMUEvent(state, tracer, s_5_1);
        // C s_5_3: const #() : ()
        let s_5_3: () = ();
        // S s_5_4: call GetNumEventCounters(s_5_3)
        let s_5_4: i128 = GetNumEventCounters(state, tracer, s_5_3);
        // D s_5_5: write-var counters <= s_5_4
        fn_state.counters = s_5_4;
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // D s_5_7: read-var counters:i
        let s_5_7: i128 = fn_state.counters;
        // D s_5_8: call neq_int(s_5_7, s_5_6)
        let s_5_8: bool = neq_int(state, tracer, s_5_7, s_5_6);
        // N s_5_9: branch s_5_8 b8 b6
        if s_5_8 {
            return block_8(state, tracer, fn_state);
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
        // S s_7_1: call AArch32_IncrementCycleCounter(s_7_0)
        let s_7_1: () = AArch32_IncrementCycleCounter(state, tracer, s_7_0);
        // C s_7_2: const #() : ()
        let s_7_2: () = ();
        // S s_7_3: call CheckForPMUOverflow(s_7_2)
        let s_7_3: () = CheckForPMUOverflow(state, tracer, s_7_2);
        // N s_7_4: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0s : i64
        let s_8_0: i64 = 0;
        // C s_8_1: const #1s : i
        let s_8_1: i128 = 1;
        // D s_8_2: read-var counters:i
        let s_8_2: i128 = fn_state.counters;
        // D s_8_3: sub s_8_2 s_8_1
        let s_8_3: i128 = ((s_8_2) - (s_8_1));
        // D s_8_4: cast reint s_8_3 -> i64
        let s_8_4: i64 = (s_8_3 as i64);
        // D s_8_5: write-var gs#32279 <= s_8_4
        fn_state.gs_32279 = s_8_4;
        // D s_8_6: write-var idx <= s_8_0
        fn_state.idx = s_8_0;
        // N s_8_7: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var idx:i64
        let s_9_0: i64 = fn_state.idx;
        // D s_9_1: read-var gs#32279:i64
        let s_9_1: i64 = fn_state.gs_32279;
        // D s_9_2: cmp-gt s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) > (s_9_1));
        // N s_9_3: branch s_9_2 b23 b10
        if s_9_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var idx:i64
        let s_10_0: i64 = fn_state.idx;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call __id(s_10_1)
        let s_10_2: i128 = u__id(state, tracer, s_10_1);
        // C s_10_3: const #0s : i
        let s_10_3: i128 = 0;
        // D s_10_4: cmp-le s_10_3 s_10_2
        let s_10_4: bool = ((s_10_3) <= (s_10_2));
        // N s_10_5: branch s_10_4 b22 b11
        if s_10_4 {
            return block_22(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#32284 <= s_11_0
        fn_state.gs_32284 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#32284:u8
        let s_12_0: bool = fn_state.gs_32284;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var idx:i64
        let s_12_2: i64 = fn_state.idx;
        // D s_12_3: call CountPMUEvents(s_12_2)
        let s_12_3: bool = CountPMUEvents(state, tracer, s_12_2);
        // N s_12_4: branch s_12_3 b18 b13
        if s_12_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var idx:i64
        let s_14_0: i64 = fn_state.idx;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call __id(s_14_1)
        let s_14_2: i128 = u__id(state, tracer, s_14_1);
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: const #0s : i
        let s_14_4: i128 = 0;
        // D s_14_5: cast zx s_14_3 -> i
        let s_14_5: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_6: cmp-le s_14_4 s_14_5
        let s_14_6: bool = ((s_14_4) <= (s_14_5));
        // N s_14_7: branch s_14_6 b17 b15
        if s_14_6 {
            return block_17(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#32287 <= s_15_0
        fn_state.gs_32287 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#32287:u8
        let s_16_0: bool = fn_state.gs_32287;
        // N s_16_1: assert s_16_0
        let s_16_1: () = assert!(s_16_0);
        // C s_16_2: const #0s : i
        let s_16_2: i128 = 0;
        // C s_16_3: const #89632u : u32
        let s_16_3: u32 = 89632;
        // D s_16_4: read-reg s_16_3:[i; 31]
        let s_16_4: [i128; 31usize] = {
            let value = state.read_register::<[i128; 31usize]>(s_16_3 as isize);
            tracer.read_register(s_16_3 as isize, value);
            value
        };
        // D s_16_5: read-var idx:i64
        let s_16_5: i64 = fn_state.idx;
        // D s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // D s_16_7: mutate-element s_16_4[s_16_6] <= s_16_2
        let s_16_7: [i128; 31usize] = {
            let mut local = s_16_4.clone();
            local[(s_16_6) as usize] = s_16_2;
            local
        };
        // D s_16_8: cast cvt s_16_7 -> [i; 0]
        let s_16_8: alloc::vec::Vec<i128> = alloc::vec::Vec::from(s_16_7);
        // D s_16_9: cast cvt s_16_8 -> [i; 31]
        let s_16_9: [i128; 31usize] = {
            let mut buf = [Default::default(); 31usize];
            buf.copy_from_slice(&s_16_8);
            buf
        };
        // C s_16_10: const #89632u : u32
        let s_16_10: u32 = 89632;
        // N s_16_11: write-reg s_16_10 <= s_16_9
        let s_16_11: () = {
            state.write_register::<[i128; 31usize]>(s_16_10 as isize, s_16_9);
            tracer.write_register(s_16_10 as isize, s_16_9);
        };
        // D s_16_12: read-var idx:i64
        let s_16_12: i64 = fn_state.idx;
        // C s_16_13: const #1s : i64
        let s_16_13: i64 = 1;
        // D s_16_14: add s_16_12 s_16_13
        let s_16_14: i64 = (s_16_12 + s_16_13);
        // D s_16_15: write-var idx <= s_16_14
        fn_state.idx = s_16_14;
        // N s_16_16: jump b9
        return block_9(state, tracer, fn_state);
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
        // C s_17_4: const #31s : i
        let s_17_4: i128 = 31;
        // D s_17_5: cast zx s_17_3 -> i
        let s_17_5: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_6: cmp-lt s_17_5 s_17_4
        let s_17_6: bool = ((s_17_5) < (s_17_4));
        // D s_17_7: write-var gs#32287 <= s_17_6
        fn_state.gs_32287 = s_17_6;
        // N s_17_8: jump b16
        return block_16(state, tracer, fn_state);
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
        // D s_18_2: call __id(s_18_1)
        let s_18_2: i128 = u__id(state, tracer, s_18_1);
        // D s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: const #0s : i
        let s_18_4: i128 = 0;
        // D s_18_5: cast zx s_18_3 -> i
        let s_18_5: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_6: cmp-le s_18_4 s_18_5
        let s_18_6: bool = ((s_18_4) <= (s_18_5));
        // N s_18_7: branch s_18_6 b21 b19
        if s_18_6 {
            return block_21(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#32292 <= s_19_0
        fn_state.gs_32292 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#32292:u8
        let s_20_0: bool = fn_state.gs_32292;
        // N s_20_1: assert s_20_0
        let s_20_1: () = assert!(s_20_0);
        // C s_20_2: const #89632u : u32
        let s_20_2: u32 = 89632;
        // D s_20_3: read-reg s_20_2:[i; 31]
        let s_20_3: [i128; 31usize] = {
            let value = state.read_register::<[i128; 31usize]>(s_20_2 as isize);
            tracer.read_register(s_20_2 as isize, value);
            value
        };
        // D s_20_4: read-var idx:i64
        let s_20_4: i64 = fn_state.idx;
        // D s_20_5: cast zx s_20_4 -> i
        let s_20_5: i128 = (i128::try_from(s_20_4).unwrap());
        // D s_20_6: read-element s_20_3[s_20_5]
        let s_20_6: i128 = s_20_3[(s_20_5) as usize];
        // D s_20_7: read-var idx:i64
        let s_20_7: i64 = fn_state.idx;
        // D s_20_8: call AArch32_IncrementEventCounter(s_20_7, s_20_6)
        let s_20_8: () = AArch32_IncrementEventCounter(state, tracer, s_20_7, s_20_6);
        // N s_20_9: jump b14
        return block_14(state, tracer, fn_state);
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
        // D s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: const #31s : i
        let s_21_4: i128 = 31;
        // D s_21_5: cast zx s_21_3 -> i
        let s_21_5: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_6: cmp-lt s_21_5 s_21_4
        let s_21_6: bool = ((s_21_5) < (s_21_4));
        // D s_21_7: write-var gs#32292 <= s_21_6
        fn_state.gs_32292 = s_21_6;
        // N s_21_8: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var idx:i64
        let s_22_0: i64 = fn_state.idx;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: call __id(s_22_1)
        let s_22_2: i128 = u__id(state, tracer, s_22_1);
        // C s_22_3: const #32s : i
        let s_22_3: i128 = 32;
        // D s_22_4: cmp-le s_22_2 s_22_3
        let s_22_4: bool = ((s_22_2) <= (s_22_3));
        // D s_22_5: write-var gs#32284 <= s_22_4
        fn_state.gs_32284 = s_22_4;
        // N s_22_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call AArch64_PMUCycle(s_25_0)
        let s_25_1: () = AArch64_PMUCycle(state, tracer, s_25_0);
        // N s_25_2: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call ShouldBRBEFreeze(s_26_0)
        let s_26_1: bool = ShouldBRBEFreeze(state, tracer, s_26_0);
        // N s_26_2: branch s_26_1 b28 b27
        if s_26_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call BRBEFreeze(s_28_0)
        let s_28_1: () = BRBEFreeze(state, tracer, s_28_0);
        // N s_28_2: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call HDCR_read(s_29_0)
        let s_29_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_29_0);
        // S s_29_2: call _get_HDCR_Type_HPME(s_29_1)
        let s_29_2: bool = u_get_HDCR_Type_HPME(state, tracer, s_29_1);
        // S s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // C s_29_4: const #0u : u8
        let s_29_4: bool = false;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 1u16);
        // S s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // D s_29_7: write-var gs#32265 <= s_29_6
        fn_state.gs_32265 = s_29_6;
        // N s_29_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
