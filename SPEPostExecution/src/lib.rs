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
use SPEBufferFilled::*;
use u_get_PMBLIMITR_EL1_Type_FM::*;
use PMUEvent::*;
use SPEResetSampleStorage::*;
use SPEBufferIsFull::*;
use SPECollectRecord::*;
use SPEConstructRecord::*;
use SPEStopCounter::*;
use HaveStatisticalProfilingv1p2::*;
use common::*;
pub fn SPEPostExecution<T: Tracer>(state: &mut State, tracer: &T, gs_26227: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        discard: bool,
        gs_26240: bool,
        gs_26232: i64,
        counter_index: i64,
        gs_26227: (),
    }
    let fn_state = FunctionState {
        gs_26227,
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
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // C s_2_1: const #22416u : u32
        let s_2_1: u32 = 22416;
        // N s_2_2: write-reg s_2_1 <= s_2_0
        let s_2_2: () = {
            state.write_register::<bool>(s_2_1 as isize, s_2_0);
            tracer.write_register(s_2_1 as isize, s_2_0);
        };
        // C s_2_3: const #160u : u32
        let s_2_3: u32 = 160;
        // D s_2_4: read-reg s_2_3:u16
        let s_2_4: u16 = {
            let value = state.read_register::<u16>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: call PMUEvent(s_2_4)
        let s_2_5: () = PMUEvent(state, tracer, s_2_4);
        // C s_2_6: const #0s : i64
        let s_2_6: i64 = 0;
        // C s_2_7: const #1s : i
        let s_2_7: i128 = 1;
        // C s_2_8: const #1000u : u32
        let s_2_8: u32 = 1000;
        // D s_2_9: read-reg s_2_8:i64
        let s_2_9: i64 = {
            let value = state.read_register::<i64>(s_2_8 as isize);
            tracer.read_register(s_2_8 as isize, value);
            value
        };
        // D s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_11: sub s_2_10 s_2_7
        let s_2_11: i128 = ((s_2_10) - (s_2_7));
        // D s_2_12: cast reint s_2_11 -> i64
        let s_2_12: i64 = (s_2_11 as i64);
        // D s_2_13: write-var gs#26232 <= s_2_12
        fn_state.gs_26232 = s_2_12;
        // D s_2_14: write-var counter_index <= s_2_6
        fn_state.counter_index = s_2_6;
        // N s_2_15: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var counter_index:i64
        let s_3_0: i64 = fn_state.counter_index;
        // D s_3_1: read-var gs#26232:i64
        let s_3_1: i64 = fn_state.gs_26232;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b8 b4
        if s_3_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #102648u : u32
        let s_4_0: u32 = 102648;
        // D s_4_1: read-reg s_4_0:[u8; 32]
        let s_4_1: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: read-var counter_index:i64
        let s_4_2: i64 = fn_state.counter_index;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-element s_4_1[s_4_3]
        let s_4_4: bool = s_4_1[(s_4_3) as usize];
        // N s_4_5: branch s_4_4 b7 b5
        if s_4_4 {
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
        // D s_6_0: read-var counter_index:i64
        let s_6_0: i64 = fn_state.counter_index;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var counter_index <= s_6_2
        fn_state.counter_index = s_6_2;
        // N s_6_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var counter_index:i64
        let s_7_0: i64 = fn_state.counter_index;
        // D s_7_1: call SPEStopCounter(s_7_0)
        let s_7_1: () = SPEStopCounter(state, tracer, s_7_0);
        // N s_7_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var discard <= s_8_0
        fn_state.discard = s_8_0;
        // C s_8_2: const #() : ()
        let s_8_2: () = ();
        // S s_8_3: call HaveStatisticalProfilingv1p2(s_8_2)
        let s_8_3: bool = HaveStatisticalProfilingv1p2(state, tracer, s_8_2);
        // N s_8_4: branch s_8_3 b21 b9
        if s_8_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #18464u : u32
        let s_10_0: u32 = 18464;
        // D s_10_1: read-reg s_10_0:[i; 32]
        let s_10_1: [i128; 32usize] = {
            let value = state.read_register::<[i128; 32usize]>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // C s_10_2: const #1056u : u32
        let s_10_2: u32 = 1056;
        // D s_10_3: read-reg s_10_2:i64
        let s_10_3: i64 = {
            let value = state.read_register::<i64>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: read-element s_10_1[s_10_4]
        let s_10_5: i128 = s_10_1[(s_10_4) as usize];
        // C s_10_6: const #104856u : u32
        let s_10_6: u32 = 104856;
        // D s_10_7: read-reg s_10_6:u64
        let s_10_7: u64 = {
            let value = state.read_register::<u64>(s_10_6 as isize);
            tracer.read_register(s_10_6 as isize, value);
            value
        };
        // C s_10_8: const #19040u : u32
        let s_10_8: u32 = 19040;
        // D s_10_9: read-reg s_10_8:u32
        let s_10_9: u32 = {
            let value = state.read_register::<u32>(s_10_8 as isize);
            tracer.read_register(s_10_8 as isize, value);
            value
        };
        // D s_10_10: call SPECollectRecord(s_10_7, s_10_5, s_10_9)
        let s_10_10: bool = SPECollectRecord(state, tracer, s_10_7, s_10_5, s_10_9);
        // N s_10_11: branch s_10_10 b20 b11
        if s_10_10 {
            return block_20(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#26240 <= s_11_0
        fn_state.gs_26240 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#26240:u8
        let s_12_0: bool = fn_state.gs_26240;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
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
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call SPEResetSampleStorage(s_14_0)
        let s_14_1: () = SPEResetSampleStorage(state, tracer, s_14_0);
        // N s_14_2: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call SPEConstructRecord(s_15_0)
        let s_15_1: () = SPEConstructRecord(state, tracer, s_15_0);
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call SPEBufferIsFull(s_16_0)
        let s_16_1: bool = SPEBufferIsFull(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b19 b17
        if s_16_1 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call SPEBufferFilled(s_19_0)
        let s_19_1: () = SPEBufferFilled(state, tracer, s_19_0);
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var discard:u8
        let s_20_0: bool = fn_state.discard;
        // D s_20_1: not s_20_0
        let s_20_1: bool = !s_20_0;
        // D s_20_2: write-var gs#26240 <= s_20_1
        fn_state.gs_26240 = s_20_1;
        // N s_20_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #20480u : u32
        let s_21_0: u32 = 20480;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_PMBLIMITR_EL1_Type_FM(s_21_1)
        let s_21_2: u8 = u_get_PMBLIMITR_EL1_Type_FM(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 2u16);
        // C s_21_4: const #2u : u8
        let s_21_4: u8 = 2;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 2u16);
        // D s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // D s_21_7: write-var discard <= s_21_6
        fn_state.discard = s_21_6;
        // N s_21_8: jump b10
        return block_10(state, tracer, fn_state);
    }
}
