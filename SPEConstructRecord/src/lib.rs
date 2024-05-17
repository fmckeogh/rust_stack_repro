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
use SPEAddByteToRecord::*;
use fmod_int::*;
use u__IMPDEF_boolean::*;
use Zeros::*;
use SPEGetDataSourcePayloadSize::*;
use StatisticalProfilingEnabled__1::*;
use SPEGetEventsPayloadSize::*;
use neq_int::*;
use u__id::*;
use integer_subrange::*;
use u_get_PMBIDR_EL1_Type_Align::*;
use SPEWriteToBuffer::*;
use SPEAddPacketToRecord::*;
use SPEEmptyRecord::*;
use HaveStatisticalProfilingv1p2::*;
use common::*;
pub fn SPEConstructRecord<T: Tracer>(state: &mut State, tracer: &T, gs_26060: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        include_prev_br_name: bool,
        counter_index: i64,
        gs_26149: bool,
        address_index: i64,
        payload_sizeshadow_493: i128,
        gs_26126: bool,
        payload_sizeshadow_494: i128,
        gs_26147: bool,
        gs_26113: bool,
        gs_26083: i64,
        gs_26064: i64,
        gs_26104: bool,
        gs_26072: bool,
        gs_26148: bool,
        large_counters: bool,
        gs_26079: bool,
        gs_26060: (),
    }
    let fn_state = FunctionState {
        gs_26060,
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
        // S s_0_1: call SPEEmptyRecord(s_0_0)
        let s_0_1: () = SPEEmptyRecord(state, tracer, s_0_0);
        // C s_0_2: const #20160u : u32
        let s_0_2: u32 = 20160;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: bool = {
            let value = state.read_register::<bool>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // N s_0_4: branch s_0_3 b71 b1
        if s_0_3 {
            return block_71(state, tracer, fn_state);
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
        // C s_2_0: const #14880u : u32
        let s_2_0: u32 = 14880;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: bool = {
            let value = state.read_register::<bool>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // N s_2_2: branch s_2_1 b70 b3
        if s_2_1 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i64
        let s_4_0: i64 = 0;
        // C s_4_1: const #1s : i
        let s_4_1: i128 = 1;
        // C s_4_2: const #1000u : u32
        let s_4_2: u32 = 1000;
        // D s_4_3: read-reg s_4_2:i64
        let s_4_3: i64 = {
            let value = state.read_register::<i64>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: sub s_4_4 s_4_1
        let s_4_5: i128 = ((s_4_4) - (s_4_1));
        // D s_4_6: cast reint s_4_5 -> i64
        let s_4_6: i64 = (s_4_5 as i64);
        // D s_4_7: write-var gs#26064 <= s_4_6
        fn_state.gs_26064 = s_4_6;
        // D s_4_8: write-var counter_index <= s_4_0
        fn_state.counter_index = s_4_0;
        // N s_4_9: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var counter_index:i64
        let s_5_0: i64 = fn_state.counter_index;
        // D s_5_1: read-var gs#26064:i64
        let s_5_1: i64 = fn_state.gs_26064;
        // D s_5_2: cmp-gt s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) > (s_5_1));
        // N s_5_3: branch s_5_2 b21 b6
        if s_5_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #23056u : u32
        let s_6_0: u32 = 23056;
        // D s_6_1: read-reg s_6_0:[u8; 32]
        let s_6_1: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: read-var counter_index:i64
        let s_6_2: i64 = fn_state.counter_index;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: read-element s_6_1[s_6_3]
        let s_6_4: bool = s_6_1[(s_6_3) as usize];
        // N s_6_5: branch s_6_4 b9 b7
        if s_6_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var counter_index:i64
        let s_8_0: i64 = fn_state.counter_index;
        // C s_8_1: const #1s : i64
        let s_8_1: i64 = 1;
        // D s_8_2: add s_8_0 s_8_1
        let s_8_2: i64 = (s_8_0 + s_8_1);
        // D s_8_3: write-var counter_index <= s_8_2
        fn_state.counter_index = s_8_2;
        // N s_8_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #8s : i
        let s_9_0: i128 = 8;
        // D s_9_1: read-var counter_index:i64
        let s_9_1: i64 = fn_state.counter_index;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cmp-ge s_9_2 s_9_0
        let s_9_3: bool = ((s_9_2) >= (s_9_0));
        // N s_9_4: branch s_9_3 b20 b10
        if s_9_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #"SPE 16bit counters" : str
        let s_11_0: &'static str = "SPE 16bit counters";
        // S s_11_1: call __IMPDEF_boolean(s_11_0)
        let s_11_1: bool = u__IMPDEF_boolean(state, tracer, s_11_0);
        // D s_11_2: write-var large_counters <= s_11_1
        fn_state.large_counters = s_11_1;
        // C s_11_3: const #18464u : u32
        let s_11_3: u32 = 18464;
        // D s_11_4: read-reg s_11_3:[i; 32]
        let s_11_4: [i128; 32usize] = {
            let value = state.read_register::<[i128; 32usize]>(s_11_3 as isize);
            tracer.read_register(s_11_3 as isize, value);
            value
        };
        // D s_11_5: read-var counter_index:i64
        let s_11_5: i64 = fn_state.counter_index;
        // D s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_7: read-element s_11_4[s_11_6]
        let s_11_7: i128 = s_11_4[(s_11_6) as usize];
        // C s_11_8: const #65535u : u16
        let s_11_8: u16 = 65535;
        // C s_11_9: cast zx s_11_8 -> bv
        let s_11_9: Bits = Bits::new(s_11_8 as u128, 16u16);
        // C s_11_10: cast zx s_11_9 -> i
        let s_11_10: i128 = (s_11_9.value() as i128);
        // C s_11_11: cast reint s_11_10 -> i64
        let s_11_11: i64 = (s_11_10 as i64);
        // C s_11_12: cast zx s_11_11 -> i
        let s_11_12: i128 = (i128::try_from(s_11_11).unwrap());
        // D s_11_13: cmp-gt s_11_7 s_11_12
        let s_11_13: bool = ((s_11_7) > (s_11_12));
        // N s_11_14: branch s_11_13 b19 b12
        if s_11_13 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#26072 <= s_12_0
        fn_state.gs_26072 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#26072:u8
        let s_13_0: bool = fn_state.gs_26072;
        // N s_13_1: branch s_13_0 b18 b14
        if s_13_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #18464u : u32
        let s_14_0: u32 = 18464;
        // D s_14_1: read-reg s_14_0:[i; 32]
        let s_14_1: [i128; 32usize] = {
            let value = state.read_register::<[i128; 32usize]>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: read-var counter_index:i64
        let s_14_2: i64 = fn_state.counter_index;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: read-element s_14_1[s_14_3]
        let s_14_4: i128 = s_14_1[(s_14_3) as usize];
        // C s_14_5: const #4095u : u12
        let s_14_5: u16 = 4095;
        // C s_14_6: cast zx s_14_5 -> bv
        let s_14_6: Bits = Bits::new(s_14_5 as u128, 12u16);
        // C s_14_7: cast zx s_14_6 -> i
        let s_14_7: i128 = (s_14_6.value() as i128);
        // C s_14_8: cast reint s_14_7 -> i64
        let s_14_8: i64 = (s_14_7 as i64);
        // C s_14_9: cast zx s_14_8 -> i
        let s_14_9: i128 = (i128::try_from(s_14_8).unwrap());
        // D s_14_10: cmp-gt s_14_4 s_14_9
        let s_14_10: bool = ((s_14_4) > (s_14_9));
        // N s_14_11: branch s_14_10 b17 b15
        if s_14_10 {
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
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #2s : i
        let s_16_0: i128 = 2;
        // C s_16_1: const #0s : i
        let s_16_1: i128 = 0;
        // D s_16_2: read-var counter_index:i64
        let s_16_2: i64 = fn_state.counter_index;
        // D s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_4: call integer_subrange(s_16_3, s_16_0, s_16_1)
        let s_16_4: Bits = integer_subrange(state, tracer, s_16_3, s_16_0, s_16_1);
        // D s_16_5: cast reint s_16_4 -> u8
        let s_16_5: u8 = (s_16_4.value() as u8);
        // C s_16_6: const #1u : u8
        let s_16_6: bool = true;
        // C s_16_7: cast zx s_16_6 -> bv
        let s_16_7: Bits = Bits::new(s_16_6 as u128, 1u16);
        // D s_16_8: cast zx s_16_5 -> bv
        let s_16_8: Bits = Bits::new(s_16_5 as u128, 3u16);
        // C s_16_9: cast reint s_16_7 -> u128
        let s_16_9: u128 = (s_16_7.value() as u128);
        // D s_16_10: size-of s_16_7
        let s_16_10: u16 = s_16_7.length();
        // D s_16_11: cast reint s_16_8 -> u128
        let s_16_11: u128 = (s_16_8.value() as u128);
        // D s_16_12: size-of s_16_8
        let s_16_12: u16 = s_16_8.length();
        // D s_16_13: lsl s_16_9 s_16_12
        let s_16_13: u128 = s_16_9 << s_16_12;
        // D s_16_14: or s_16_13 s_16_11
        let s_16_14: u128 = ((s_16_13) | (s_16_11));
        // D s_16_15: add s_16_10 s_16_12
        let s_16_15: u16 = (s_16_10 + s_16_12);
        // D s_16_16: create-bits s_16_14 s_16_15
        let s_16_16: Bits = Bits::new(s_16_14, s_16_15);
        // D s_16_17: cast reint s_16_16 -> u8
        let s_16_17: u8 = (s_16_16.value() as u8);
        // C s_16_18: const #18464u : u32
        let s_16_18: u32 = 18464;
        // D s_16_19: read-reg s_16_18:[i; 32]
        let s_16_19: [i128; 32usize] = {
            let value = state.read_register::<[i128; 32usize]>(s_16_18 as isize);
            tracer.read_register(s_16_18 as isize, value);
            value
        };
        // D s_16_20: read-var counter_index:i64
        let s_16_20: i64 = fn_state.counter_index;
        // D s_16_21: cast zx s_16_20 -> i
        let s_16_21: i128 = (i128::try_from(s_16_20).unwrap());
        // D s_16_22: read-element s_16_19[s_16_21]
        let s_16_22: i128 = s_16_19[(s_16_21) as usize];
        // C s_16_23: const #15s : i
        let s_16_23: i128 = 15;
        // C s_16_24: const #0s : i
        let s_16_24: i128 = 0;
        // D s_16_25: call integer_subrange(s_16_22, s_16_23, s_16_24)
        let s_16_25: Bits = integer_subrange(state, tracer, s_16_22, s_16_23, s_16_24);
        // D s_16_26: cast reint s_16_25 -> u16
        let s_16_26: u16 = (s_16_25.value() as u16);
        // D s_16_27: cast zx s_16_26 -> bv
        let s_16_27: Bits = Bits::new(s_16_26 as u128, 16u16);
        // C s_16_28: const #2u : u8
        let s_16_28: u8 = 2;
        // D s_16_29: call SPEAddPacketToRecord(s_16_28, s_16_17, s_16_27)
        let s_16_29: () = SPEAddPacketToRecord(state, tracer, s_16_28, s_16_17, s_16_27);
        // N s_16_30: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #4095u : u12
        let s_17_0: u16 = 4095;
        // C s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 12u16);
        // C s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (s_17_1.value() as i128);
        // C s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // C s_17_4: const #18464u : u32
        let s_17_4: u32 = 18464;
        // D s_17_5: read-reg s_17_4:[i; 32]
        let s_17_5: [i128; 32usize] = {
            let value = state.read_register::<[i128; 32usize]>(s_17_4 as isize);
            tracer.read_register(s_17_4 as isize, value);
            value
        };
        // D s_17_6: read-var counter_index:i64
        let s_17_6: i64 = fn_state.counter_index;
        // D s_17_7: cast zx s_17_6 -> i
        let s_17_7: i128 = (i128::try_from(s_17_6).unwrap());
        // C s_17_8: cast zx s_17_3 -> i
        let s_17_8: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_9: mutate-element s_17_5[s_17_7] <= s_17_8
        let s_17_9: [i128; 32usize] = {
            let mut local = s_17_5.clone();
            local[(s_17_7) as usize] = s_17_8;
            local
        };
        // D s_17_10: cast cvt s_17_9 -> [i; 0]
        let s_17_10: alloc::vec::Vec<i128> = alloc::vec::Vec::from(s_17_9);
        // D s_17_11: cast cvt s_17_10 -> [i; 32]
        let s_17_11: [i128; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_17_10);
            buf
        };
        // C s_17_12: const #18464u : u32
        let s_17_12: u32 = 18464;
        // N s_17_13: write-reg s_17_12 <= s_17_11
        let s_17_13: () = {
            state.write_register::<[i128; 32usize]>(s_17_12 as isize, s_17_11);
            tracer.write_register(s_17_12 as isize, s_17_11);
        };
        // N s_17_14: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #65535u : u16
        let s_18_0: u16 = 65535;
        // C s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 16u16);
        // C s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (s_18_1.value() as i128);
        // C s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: const #18464u : u32
        let s_18_4: u32 = 18464;
        // D s_18_5: read-reg s_18_4:[i; 32]
        let s_18_5: [i128; 32usize] = {
            let value = state.read_register::<[i128; 32usize]>(s_18_4 as isize);
            tracer.read_register(s_18_4 as isize, value);
            value
        };
        // D s_18_6: read-var counter_index:i64
        let s_18_6: i64 = fn_state.counter_index;
        // D s_18_7: cast zx s_18_6 -> i
        let s_18_7: i128 = (i128::try_from(s_18_6).unwrap());
        // C s_18_8: cast zx s_18_3 -> i
        let s_18_8: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_9: mutate-element s_18_5[s_18_7] <= s_18_8
        let s_18_9: [i128; 32usize] = {
            let mut local = s_18_5.clone();
            local[(s_18_7) as usize] = s_18_8;
            local
        };
        // D s_18_10: cast cvt s_18_9 -> [i; 0]
        let s_18_10: alloc::vec::Vec<i128> = alloc::vec::Vec::from(s_18_9);
        // D s_18_11: cast cvt s_18_10 -> [i; 32]
        let s_18_11: [i128; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_18_10);
            buf
        };
        // C s_18_12: const #18464u : u32
        let s_18_12: u32 = 18464;
        // N s_18_13: write-reg s_18_12 <= s_18_11
        let s_18_13: () = {
            state.write_register::<[i128; 32usize]>(s_18_12 as isize, s_18_11);
            tracer.write_register(s_18_12 as isize, s_18_11);
        };
        // N s_18_14: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var large_counters:u8
        let s_19_0: bool = fn_state.large_counters;
        // D s_19_1: write-var gs#26072 <= s_19_0
        fn_state.gs_26072 = s_19_0;
        // N s_19_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #4s : i
        let s_20_0: i128 = 4;
        // C s_20_1: const #3s : i
        let s_20_1: i128 = 3;
        // D s_20_2: read-var counter_index:i64
        let s_20_2: i64 = fn_state.counter_index;
        // D s_20_3: cast zx s_20_2 -> i
        let s_20_3: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_4: call integer_subrange(s_20_3, s_20_0, s_20_1)
        let s_20_4: Bits = integer_subrange(state, tracer, s_20_3, s_20_0, s_20_1);
        // D s_20_5: cast reint s_20_4 -> u8
        let s_20_5: u8 = (s_20_4.value() as u8);
        // C s_20_6: const #8u : u8
        let s_20_6: u8 = 8;
        // C s_20_7: cast zx s_20_6 -> bv
        let s_20_7: Bits = Bits::new(s_20_6 as u128, 6u16);
        // D s_20_8: cast zx s_20_5 -> bv
        let s_20_8: Bits = Bits::new(s_20_5 as u128, 2u16);
        // C s_20_9: cast reint s_20_7 -> u128
        let s_20_9: u128 = (s_20_7.value() as u128);
        // D s_20_10: size-of s_20_7
        let s_20_10: u16 = s_20_7.length();
        // D s_20_11: cast reint s_20_8 -> u128
        let s_20_11: u128 = (s_20_8.value() as u128);
        // D s_20_12: size-of s_20_8
        let s_20_12: u16 = s_20_8.length();
        // D s_20_13: lsl s_20_9 s_20_12
        let s_20_13: u128 = s_20_9 << s_20_12;
        // D s_20_14: or s_20_13 s_20_11
        let s_20_14: u128 = ((s_20_13) | (s_20_11));
        // D s_20_15: add s_20_10 s_20_12
        let s_20_15: u16 = (s_20_10 + s_20_12);
        // D s_20_16: create-bits s_20_14 s_20_15
        let s_20_16: Bits = Bits::new(s_20_14, s_20_15);
        // D s_20_17: cast reint s_20_16 -> u8
        let s_20_17: u8 = (s_20_16.value() as u8);
        // D s_20_18: call SPEAddByteToRecord(s_20_17)
        let s_20_18: () = SPEAddByteToRecord(state, tracer, s_20_17);
        // N s_20_19: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call HaveStatisticalProfilingv1p2(s_21_0)
        let s_21_1: bool = HaveStatisticalProfilingv1p2(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b63 b22
        if s_21_1 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #432u : u32
        let s_23_0: u32 = 432;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call StatisticalProfilingEnabled__1(s_23_1)
        let s_23_2: bool = StatisticalProfilingEnabled__1(state, tracer, s_23_1);
        // D s_23_3: not s_23_2
        let s_23_3: bool = !s_23_2;
        // N s_23_4: branch s_23_3 b62 b24
        if s_23_3 {
            return block_62(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#26079 <= s_24_0
        fn_state.gs_26079 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#26079:u8
        let s_25_0: bool = fn_state.gs_26079;
        // N s_25_1: branch s_25_0 b61 b26
        if s_25_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0s : i64
        let s_27_0: i64 = 0;
        // C s_27_1: const #1s : i
        let s_27_1: i128 = 1;
        // C s_27_2: const #992u : u32
        let s_27_2: u32 = 992;
        // D s_27_3: read-reg s_27_2:i64
        let s_27_3: i64 = {
            let value = state.read_register::<i64>(s_27_2 as isize);
            tracer.read_register(s_27_2 as isize, value);
            value
        };
        // D s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_5: sub s_27_4 s_27_1
        let s_27_5: i128 = ((s_27_4) - (s_27_1));
        // D s_27_6: cast reint s_27_5 -> i64
        let s_27_6: i64 = (s_27_5 as i64);
        // D s_27_7: write-var gs#26083 <= s_27_6
        fn_state.gs_26083 = s_27_6;
        // D s_27_8: write-var address_index <= s_27_0
        fn_state.address_index = s_27_0;
        // N s_27_9: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var address_index:i64
        let s_28_0: i64 = fn_state.address_index;
        // D s_28_1: read-var gs#26083:i64
        let s_28_1: i64 = fn_state.gs_26083;
        // D s_28_2: cmp-gt s_28_0 s_28_1
        let s_28_2: bool = ((s_28_0) > (s_28_1));
        // N s_28_3: branch s_28_2 b36 b29
        if s_28_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #11136u : u32
        let s_29_0: u32 = 11136;
        // D s_29_1: read-reg s_29_0:[u8; 32]
        let s_29_1: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: read-var address_index:i64
        let s_29_2: i64 = fn_state.address_index;
        // D s_29_3: cast zx s_29_2 -> i
        let s_29_3: i128 = (i128::try_from(s_29_2).unwrap());
        // D s_29_4: read-element s_29_1[s_29_3]
        let s_29_4: bool = s_29_1[(s_29_3) as usize];
        // N s_29_5: branch s_29_4 b32 b30
        if s_29_4 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var address_index:i64
        let s_31_0: i64 = fn_state.address_index;
        // C s_31_1: const #1s : i64
        let s_31_1: i64 = 1;
        // D s_31_2: add s_31_0 s_31_1
        let s_31_2: i64 = (s_31_0 + s_31_1);
        // D s_31_3: write-var address_index <= s_31_2
        fn_state.address_index = s_31_2;
        // N s_31_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #8s : i
        let s_32_0: i128 = 8;
        // D s_32_1: read-var address_index:i64
        let s_32_1: i64 = fn_state.address_index;
        // D s_32_2: cast zx s_32_1 -> i
        let s_32_2: i128 = (i128::try_from(s_32_1).unwrap());
        // D s_32_3: cmp-ge s_32_2 s_32_0
        let s_32_3: bool = ((s_32_2) >= (s_32_0));
        // N s_32_4: branch s_32_3 b35 b33
        if s_32_3 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #2s : i
        let s_34_0: i128 = 2;
        // C s_34_1: const #0s : i
        let s_34_1: i128 = 0;
        // D s_34_2: read-var address_index:i64
        let s_34_2: i64 = fn_state.address_index;
        // D s_34_3: cast zx s_34_2 -> i
        let s_34_3: i128 = (i128::try_from(s_34_2).unwrap());
        // D s_34_4: call integer_subrange(s_34_3, s_34_0, s_34_1)
        let s_34_4: Bits = integer_subrange(state, tracer, s_34_3, s_34_0, s_34_1);
        // D s_34_5: cast reint s_34_4 -> u8
        let s_34_5: u8 = (s_34_4.value() as u8);
        // C s_34_6: const #0u : u8
        let s_34_6: bool = false;
        // C s_34_7: cast zx s_34_6 -> bv
        let s_34_7: Bits = Bits::new(s_34_6 as u128, 1u16);
        // D s_34_8: cast zx s_34_5 -> bv
        let s_34_8: Bits = Bits::new(s_34_5 as u128, 3u16);
        // C s_34_9: cast reint s_34_7 -> u128
        let s_34_9: u128 = (s_34_7.value() as u128);
        // D s_34_10: size-of s_34_7
        let s_34_10: u16 = s_34_7.length();
        // D s_34_11: cast reint s_34_8 -> u128
        let s_34_11: u128 = (s_34_8.value() as u128);
        // D s_34_12: size-of s_34_8
        let s_34_12: u16 = s_34_8.length();
        // D s_34_13: lsl s_34_9 s_34_12
        let s_34_13: u128 = s_34_9 << s_34_12;
        // D s_34_14: or s_34_13 s_34_11
        let s_34_14: u128 = ((s_34_13) | (s_34_11));
        // D s_34_15: add s_34_10 s_34_12
        let s_34_15: u16 = (s_34_10 + s_34_12);
        // D s_34_16: create-bits s_34_14 s_34_15
        let s_34_16: Bits = Bits::new(s_34_14, s_34_15);
        // D s_34_17: cast reint s_34_16 -> u8
        let s_34_17: u8 = (s_34_16.value() as u8);
        // C s_34_18: const #13776u : u32
        let s_34_18: u32 = 13776;
        // D s_34_19: read-reg s_34_18:[u64; 32]
        let s_34_19: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_34_18 as isize);
            tracer.read_register(s_34_18 as isize, value);
            value
        };
        // D s_34_20: read-var address_index:i64
        let s_34_20: i64 = fn_state.address_index;
        // D s_34_21: cast zx s_34_20 -> i
        let s_34_21: i128 = (i128::try_from(s_34_20).unwrap());
        // D s_34_22: read-element s_34_19[s_34_21]
        let s_34_22: u64 = s_34_19[(s_34_21) as usize];
        // D s_34_23: cast zx s_34_22 -> bv
        let s_34_23: Bits = Bits::new(s_34_22 as u128, 64u16);
        // C s_34_24: const #2u : u8
        let s_34_24: u8 = 2;
        // D s_34_25: call SPEAddPacketToRecord(s_34_24, s_34_17, s_34_23)
        let s_34_25: () = SPEAddPacketToRecord(state, tracer, s_34_24, s_34_17, s_34_23);
        // N s_34_26: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #4s : i
        let s_35_0: i128 = 4;
        // C s_35_1: const #3s : i
        let s_35_1: i128 = 3;
        // D s_35_2: read-var address_index:i64
        let s_35_2: i64 = fn_state.address_index;
        // D s_35_3: cast zx s_35_2 -> i
        let s_35_3: i128 = (i128::try_from(s_35_2).unwrap());
        // D s_35_4: call integer_subrange(s_35_3, s_35_0, s_35_1)
        let s_35_4: Bits = integer_subrange(state, tracer, s_35_3, s_35_0, s_35_1);
        // D s_35_5: cast reint s_35_4 -> u8
        let s_35_5: u8 = (s_35_4.value() as u8);
        // C s_35_6: const #8u : u8
        let s_35_6: u8 = 8;
        // C s_35_7: cast zx s_35_6 -> bv
        let s_35_7: Bits = Bits::new(s_35_6 as u128, 6u16);
        // D s_35_8: cast zx s_35_5 -> bv
        let s_35_8: Bits = Bits::new(s_35_5 as u128, 2u16);
        // C s_35_9: cast reint s_35_7 -> u128
        let s_35_9: u128 = (s_35_7.value() as u128);
        // D s_35_10: size-of s_35_7
        let s_35_10: u16 = s_35_7.length();
        // D s_35_11: cast reint s_35_8 -> u128
        let s_35_11: u128 = (s_35_8.value() as u128);
        // D s_35_12: size-of s_35_8
        let s_35_12: u16 = s_35_8.length();
        // D s_35_13: lsl s_35_9 s_35_12
        let s_35_13: u128 = s_35_9 << s_35_12;
        // D s_35_14: or s_35_13 s_35_11
        let s_35_14: u128 = ((s_35_13) | (s_35_11));
        // D s_35_15: add s_35_10 s_35_12
        let s_35_15: u16 = (s_35_10 + s_35_12);
        // D s_35_16: create-bits s_35_14 s_35_15
        let s_35_16: Bits = Bits::new(s_35_14, s_35_15);
        // D s_35_17: cast reint s_35_16 -> u8
        let s_35_17: u8 = (s_35_16.value() as u8);
        // D s_35_18: call SPEAddByteToRecord(s_35_17)
        let s_35_18: () = SPEAddByteToRecord(state, tracer, s_35_17);
        // N s_35_19: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #14184u : u32
        let s_36_0: u32 = 14184;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: bool = {
            let value = state.read_register::<bool>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // N s_36_2: branch s_36_1 b57 b37
        if s_36_1 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #2u : u8
        let s_38_0: u8 = 2;
        // C s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 2u16);
        // C s_38_2: const #17136u : u32
        let s_38_2: u32 = 17136;
        // D s_38_3: read-reg s_38_2:u8
        let s_38_3: u8 = {
            let value = state.read_register::<u8>(s_38_2 as isize);
            tracer.read_register(s_38_2 as isize, value);
            value
        };
        // D s_38_4: cast zx s_38_3 -> bv
        let s_38_4: Bits = Bits::new(s_38_3 as u128, 2u16);
        // C s_38_5: cast reint s_38_1 -> u128
        let s_38_5: u128 = (s_38_1.value() as u128);
        // D s_38_6: size-of s_38_1
        let s_38_6: u16 = s_38_1.length();
        // D s_38_7: cast reint s_38_4 -> u128
        let s_38_7: u128 = (s_38_4.value() as u128);
        // D s_38_8: size-of s_38_4
        let s_38_8: u16 = s_38_4.length();
        // D s_38_9: lsl s_38_5 s_38_8
        let s_38_9: u128 = s_38_5 << s_38_8;
        // D s_38_10: or s_38_9 s_38_7
        let s_38_10: u128 = ((s_38_9) | (s_38_7));
        // D s_38_11: add s_38_6 s_38_8
        let s_38_11: u16 = (s_38_6 + s_38_8);
        // D s_38_12: create-bits s_38_10 s_38_11
        let s_38_12: Bits = Bits::new(s_38_10, s_38_11);
        // D s_38_13: cast reint s_38_12 -> u8
        let s_38_13: u8 = (s_38_12.value() as u8);
        // C s_38_14: const #13528u : u32
        let s_38_14: u32 = 13528;
        // D s_38_15: read-reg s_38_14:u8
        let s_38_15: u8 = {
            let value = state.read_register::<u8>(s_38_14 as isize);
            tracer.read_register(s_38_14 as isize, value);
            value
        };
        // D s_38_16: cast zx s_38_15 -> bv
        let s_38_16: Bits = Bits::new(s_38_15 as u128, 8u16);
        // C s_38_17: const #1u : u8
        let s_38_17: u8 = 1;
        // D s_38_18: call SPEAddPacketToRecord(s_38_17, s_38_13, s_38_16)
        let s_38_18: () = SPEAddPacketToRecord(state, tracer, s_38_17, s_38_13, s_38_16);
        // C s_38_19: const #() : ()
        let s_38_19: () = ();
        // S s_38_20: call SPEGetEventsPayloadSize(s_38_19)
        let s_38_20: i128 = SPEGetEventsPayloadSize(state, tracer, s_38_19);
        // D s_38_21: write-var payload_sizeshadow#493 <= s_38_20
        fn_state.payload_sizeshadow_493 = s_38_20;
        // D s_38_22: read-var payload_sizeshadow#493:i
        let s_38_22: i128 = fn_state.payload_sizeshadow_493;
        // D s_38_23: call __id(s_38_22)
        let s_38_23: i128 = u__id(state, tracer, s_38_22);
        // C s_38_24: const #8s : i
        let s_38_24: i128 = 8;
        // D s_38_25: mul s_38_24 s_38_23
        let s_38_25: i128 = ((s_38_24) * (s_38_23));
        // C s_38_26: const #1s : i
        let s_38_26: i128 = 1;
        // D s_38_27: sub s_38_25 s_38_26
        let s_38_27: i128 = ((s_38_25) - (s_38_26));
        // C s_38_28: const #0s : i
        let s_38_28: i128 = 0;
        // D s_38_29: cmp-le s_38_28 s_38_27
        let s_38_29: bool = ((s_38_28) <= (s_38_27));
        // N s_38_30: branch s_38_29 b56 b39
        if s_38_29 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#26126 <= s_39_0
        fn_state.gs_26126 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#26126:u8
        let s_40_0: bool = fn_state.gs_26126;
        // N s_40_1: assert s_40_0
        let s_40_1: () = assert!(s_40_0);
        // D s_40_2: read-var payload_sizeshadow#493:i
        let s_40_2: i128 = fn_state.payload_sizeshadow_493;
        // D s_40_3: call __id(s_40_2)
        let s_40_3: i128 = u__id(state, tracer, s_40_2);
        // D s_40_4: cast reint s_40_3 -> i64
        let s_40_4: i64 = (s_40_3 as i64);
        // C s_40_5: const #8s : i
        let s_40_5: i128 = 8;
        // D s_40_6: cast zx s_40_4 -> i
        let s_40_6: i128 = (i128::try_from(s_40_4).unwrap());
        // D s_40_7: mul s_40_5 s_40_6
        let s_40_7: i128 = ((s_40_5) * (s_40_6));
        // D s_40_8: cast reint s_40_7 -> i64
        let s_40_8: i64 = (s_40_7 as i64);
        // C s_40_9: const #1s : i
        let s_40_9: i128 = 1;
        // D s_40_10: cast zx s_40_8 -> i
        let s_40_10: i128 = (i128::try_from(s_40_8).unwrap());
        // D s_40_11: sub s_40_10 s_40_9
        let s_40_11: i128 = ((s_40_10) - (s_40_9));
        // D s_40_12: cast reint s_40_11 -> i64
        let s_40_12: i64 = (s_40_11 as i64);
        // C s_40_13: const #0s : i
        let s_40_13: i128 = 0;
        // D s_40_14: cast zx s_40_12 -> i
        let s_40_14: i128 = (i128::try_from(s_40_12).unwrap());
        // D s_40_15: sub s_40_14 s_40_13
        let s_40_15: i128 = ((s_40_14) - (s_40_13));
        // D s_40_16: cast reint s_40_15 -> i64
        let s_40_16: i64 = (s_40_15 as i64);
        // C s_40_17: const #1s : i
        let s_40_17: i128 = 1;
        // D s_40_18: cast zx s_40_16 -> i
        let s_40_18: i128 = (i128::try_from(s_40_16).unwrap());
        // D s_40_19: add s_40_18 s_40_17
        let s_40_19: i128 = (s_40_18 + s_40_17);
        // D s_40_20: cast reint s_40_19 -> i64
        let s_40_20: i64 = (s_40_19 as i64);
        // C s_40_21: const #8s : i
        let s_40_21: i128 = 8;
        // D s_40_22: cast zx s_40_20 -> i
        let s_40_22: i128 = (i128::try_from(s_40_20).unwrap());
        // D s_40_23: cmp-eq s_40_22 s_40_21
        let s_40_23: bool = ((s_40_22) == (s_40_21));
        // N s_40_24: branch s_40_23 b55 b41
        if s_40_23 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var payload_sizeshadow#493:i
        let s_41_0: i128 = fn_state.payload_sizeshadow_493;
        // D s_41_1: call __id(s_41_0)
        let s_41_1: i128 = u__id(state, tracer, s_41_0);
        // D s_41_2: cast reint s_41_1 -> i64
        let s_41_2: i64 = (s_41_1 as i64);
        // C s_41_3: const #8s : i
        let s_41_3: i128 = 8;
        // D s_41_4: cast zx s_41_2 -> i
        let s_41_4: i128 = (i128::try_from(s_41_2).unwrap());
        // D s_41_5: mul s_41_3 s_41_4
        let s_41_5: i128 = ((s_41_3) * (s_41_4));
        // D s_41_6: cast reint s_41_5 -> i64
        let s_41_6: i64 = (s_41_5 as i64);
        // C s_41_7: const #1s : i
        let s_41_7: i128 = 1;
        // D s_41_8: cast zx s_41_6 -> i
        let s_41_8: i128 = (i128::try_from(s_41_6).unwrap());
        // D s_41_9: sub s_41_8 s_41_7
        let s_41_9: i128 = ((s_41_8) - (s_41_7));
        // D s_41_10: cast reint s_41_9 -> i64
        let s_41_10: i64 = (s_41_9 as i64);
        // C s_41_11: const #0s : i
        let s_41_11: i128 = 0;
        // D s_41_12: cast zx s_41_10 -> i
        let s_41_12: i128 = (i128::try_from(s_41_10).unwrap());
        // D s_41_13: sub s_41_12 s_41_11
        let s_41_13: i128 = ((s_41_12) - (s_41_11));
        // D s_41_14: cast reint s_41_13 -> i64
        let s_41_14: i64 = (s_41_13 as i64);
        // C s_41_15: const #1s : i
        let s_41_15: i128 = 1;
        // D s_41_16: cast zx s_41_14 -> i
        let s_41_16: i128 = (i128::try_from(s_41_14).unwrap());
        // D s_41_17: add s_41_16 s_41_15
        let s_41_17: i128 = (s_41_16 + s_41_15);
        // D s_41_18: cast reint s_41_17 -> i64
        let s_41_18: i64 = (s_41_17 as i64);
        // C s_41_19: const #16s : i
        let s_41_19: i128 = 16;
        // D s_41_20: cast zx s_41_18 -> i
        let s_41_20: i128 = (i128::try_from(s_41_18).unwrap());
        // D s_41_21: cmp-eq s_41_20 s_41_19
        let s_41_21: bool = ((s_41_20) == (s_41_19));
        // N s_41_22: branch s_41_21 b54 b42
        if s_41_21 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var payload_sizeshadow#493:i
        let s_42_0: i128 = fn_state.payload_sizeshadow_493;
        // D s_42_1: call __id(s_42_0)
        let s_42_1: i128 = u__id(state, tracer, s_42_0);
        // D s_42_2: cast reint s_42_1 -> i64
        let s_42_2: i64 = (s_42_1 as i64);
        // C s_42_3: const #8s : i
        let s_42_3: i128 = 8;
        // D s_42_4: cast zx s_42_2 -> i
        let s_42_4: i128 = (i128::try_from(s_42_2).unwrap());
        // D s_42_5: mul s_42_3 s_42_4
        let s_42_5: i128 = ((s_42_3) * (s_42_4));
        // D s_42_6: cast reint s_42_5 -> i64
        let s_42_6: i64 = (s_42_5 as i64);
        // C s_42_7: const #1s : i
        let s_42_7: i128 = 1;
        // D s_42_8: cast zx s_42_6 -> i
        let s_42_8: i128 = (i128::try_from(s_42_6).unwrap());
        // D s_42_9: sub s_42_8 s_42_7
        let s_42_9: i128 = ((s_42_8) - (s_42_7));
        // D s_42_10: cast reint s_42_9 -> i64
        let s_42_10: i64 = (s_42_9 as i64);
        // C s_42_11: const #0s : i
        let s_42_11: i128 = 0;
        // D s_42_12: cast zx s_42_10 -> i
        let s_42_12: i128 = (i128::try_from(s_42_10).unwrap());
        // D s_42_13: sub s_42_12 s_42_11
        let s_42_13: i128 = ((s_42_12) - (s_42_11));
        // D s_42_14: cast reint s_42_13 -> i64
        let s_42_14: i64 = (s_42_13 as i64);
        // C s_42_15: const #1s : i
        let s_42_15: i128 = 1;
        // D s_42_16: cast zx s_42_14 -> i
        let s_42_16: i128 = (i128::try_from(s_42_14).unwrap());
        // D s_42_17: add s_42_16 s_42_15
        let s_42_17: i128 = (s_42_16 + s_42_15);
        // D s_42_18: cast reint s_42_17 -> i64
        let s_42_18: i64 = (s_42_17 as i64);
        // C s_42_19: const #32s : i
        let s_42_19: i128 = 32;
        // D s_42_20: cast zx s_42_18 -> i
        let s_42_20: i128 = (i128::try_from(s_42_18).unwrap());
        // D s_42_21: cmp-eq s_42_20 s_42_19
        let s_42_21: bool = ((s_42_20) == (s_42_19));
        // N s_42_22: branch s_42_21 b53 b43
        if s_42_21 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var payload_sizeshadow#493:i
        let s_43_0: i128 = fn_state.payload_sizeshadow_493;
        // D s_43_1: call __id(s_43_0)
        let s_43_1: i128 = u__id(state, tracer, s_43_0);
        // D s_43_2: cast reint s_43_1 -> i64
        let s_43_2: i64 = (s_43_1 as i64);
        // C s_43_3: const #8s : i
        let s_43_3: i128 = 8;
        // D s_43_4: cast zx s_43_2 -> i
        let s_43_4: i128 = (i128::try_from(s_43_2).unwrap());
        // D s_43_5: mul s_43_3 s_43_4
        let s_43_5: i128 = ((s_43_3) * (s_43_4));
        // D s_43_6: cast reint s_43_5 -> i64
        let s_43_6: i64 = (s_43_5 as i64);
        // C s_43_7: const #1s : i
        let s_43_7: i128 = 1;
        // D s_43_8: cast zx s_43_6 -> i
        let s_43_8: i128 = (i128::try_from(s_43_6).unwrap());
        // D s_43_9: sub s_43_8 s_43_7
        let s_43_9: i128 = ((s_43_8) - (s_43_7));
        // D s_43_10: cast reint s_43_9 -> i64
        let s_43_10: i64 = (s_43_9 as i64);
        // C s_43_11: const #0s : i
        let s_43_11: i128 = 0;
        // D s_43_12: cast zx s_43_10 -> i
        let s_43_12: i128 = (i128::try_from(s_43_10).unwrap());
        // D s_43_13: sub s_43_12 s_43_11
        let s_43_13: i128 = ((s_43_12) - (s_43_11));
        // D s_43_14: cast reint s_43_13 -> i64
        let s_43_14: i64 = (s_43_13 as i64);
        // C s_43_15: const #1s : i
        let s_43_15: i128 = 1;
        // D s_43_16: cast zx s_43_14 -> i
        let s_43_16: i128 = (i128::try_from(s_43_14).unwrap());
        // D s_43_17: add s_43_16 s_43_15
        let s_43_17: i128 = (s_43_16 + s_43_15);
        // D s_43_18: cast reint s_43_17 -> i64
        let s_43_18: i64 = (s_43_17 as i64);
        // C s_43_19: const #64s : i
        let s_43_19: i128 = 64;
        // D s_43_20: cast zx s_43_18 -> i
        let s_43_20: i128 = (i128::try_from(s_43_18).unwrap());
        // D s_43_21: cmp-eq s_43_20 s_43_19
        let s_43_21: bool = ((s_43_20) == (s_43_19));
        // D s_43_22: write-var gs#26147 <= s_43_21
        fn_state.gs_26147 = s_43_21;
        // N s_43_23: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#26147:u8
        let s_44_0: bool = fn_state.gs_26147;
        // D s_44_1: write-var gs#26148 <= s_44_0
        fn_state.gs_26148 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#26148:u8
        let s_45_0: bool = fn_state.gs_26148;
        // D s_45_1: write-var gs#26149 <= s_45_0
        fn_state.gs_26149 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#26149:u8
        let s_46_0: bool = fn_state.gs_26149;
        // N s_46_1: assert s_46_0
        let s_46_1: () = assert!(s_46_0);
        // C s_46_2: const #8s : i
        let s_46_2: i128 = 8;
        // D s_46_3: read-var payload_sizeshadow#493:i
        let s_46_3: i128 = fn_state.payload_sizeshadow_493;
        // D s_46_4: mul s_46_2 s_46_3
        let s_46_4: i128 = ((s_46_2) * (s_46_3));
        // D s_46_5: cast reint s_46_4 -> i64
        let s_46_5: i64 = (s_46_4 as i64);
        // C s_46_6: const #1s : i
        let s_46_6: i128 = 1;
        // D s_46_7: cast zx s_46_5 -> i
        let s_46_7: i128 = (i128::try_from(s_46_5).unwrap());
        // D s_46_8: sub s_46_7 s_46_6
        let s_46_8: i128 = ((s_46_7) - (s_46_6));
        // D s_46_9: cast reint s_46_8 -> i64
        let s_46_9: i64 = (s_46_8 as i64);
        // C s_46_10: const #0s : i
        let s_46_10: i128 = 0;
        // C s_46_11: const #104856u : u32
        let s_46_11: u32 = 104856;
        // D s_46_12: read-reg s_46_11:u64
        let s_46_12: u64 = {
            let value = state.read_register::<u64>(s_46_11 as isize);
            tracer.read_register(s_46_11 as isize, value);
            value
        };
        // D s_46_13: cast zx s_46_12 -> bv
        let s_46_13: Bits = Bits::new(s_46_12 as u128, 64u16);
        // D s_46_14: cast zx s_46_9 -> i
        let s_46_14: i128 = (i128::try_from(s_46_9).unwrap());
        // C s_46_15: const #1s : i64
        let s_46_15: i64 = 1;
        // C s_46_16: cast zx s_46_15 -> i
        let s_46_16: i128 = (i128::try_from(s_46_15).unwrap());
        // D s_46_17: sub s_46_14 s_46_10
        let s_46_17: i128 = ((s_46_14) - (s_46_10));
        // D s_46_18: add s_46_17 s_46_16
        let s_46_18: i128 = (s_46_17 + s_46_16);
        // D s_46_19: bit-extract s_46_13 s_46_10 s_46_18
        let s_46_19: Bits = (Bits::new(
            ((s_46_13) >> (s_46_10)).value(),
            u16::try_from(s_46_18).unwrap(),
        ));
        // C s_46_20: const #1u : u8
        let s_46_20: u8 = 1;
        // C s_46_21: const #2u : u8
        let s_46_21: u8 = 2;
        // D s_46_22: call SPEAddPacketToRecord(s_46_20, s_46_21, s_46_19)
        let s_46_22: () = SPEAddPacketToRecord(state, tracer, s_46_20, s_46_21, s_46_19);
        // C s_46_23: const #17616u : u32
        let s_46_23: u32 = 17616;
        // D s_46_24: read-reg s_46_23:u8
        let s_46_24: bool = {
            let value = state.read_register::<bool>(s_46_23 as isize);
            tracer.read_register(s_46_23 as isize, value);
            value
        };
        // N s_46_25: branch s_46_24 b52 b47
        if s_46_24 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: u8 = 1;
        // S s_47_1: call SPEAddByteToRecord(s_47_0)
        let s_47_1: () = SPEAddByteToRecord(state, tracer, s_47_0);
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #15704u : u32
        let s_49_0: u32 = 15704;
        // D s_49_1: read-reg s_49_0:struct
        let s_49_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: call _get_PMBIDR_EL1_Type_Align(s_49_1)
        let s_49_2: u8 = u_get_PMBIDR_EL1_Type_Align(state, tracer, s_49_1);
        // D s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 4u16);
        // D s_49_4: cast zx s_49_3 -> i
        let s_49_4: i128 = (s_49_3.value() as i128);
        // D s_49_5: cast reint s_49_4 -> i64
        let s_49_5: i64 = (s_49_4 as i64);
        // C s_49_6: const #1s : i
        let s_49_6: i128 = 1;
        // D s_49_7: cast zx s_49_5 -> i
        let s_49_7: i128 = (i128::try_from(s_49_5).unwrap());
        // D s_49_8: lsl s_49_6 s_49_7
        let s_49_8: i128 = s_49_6 << s_49_7;
        // C s_49_9: const #10384u : u32
        let s_49_9: u32 = 10384;
        // D s_49_10: read-reg s_49_9:i
        let s_49_10: i128 = {
            let value = state.read_register::<i128>(s_49_9 as isize);
            tracer.read_register(s_49_9 as isize, value);
            value
        };
        // D s_49_11: call fmod_int(s_49_10, s_49_8)
        let s_49_11: i128 = fmod_int(state, tracer, s_49_10, s_49_8);
        // C s_49_12: const #0s : i
        let s_49_12: i128 = 0;
        // D s_49_13: call neq_int(s_49_11, s_49_12)
        let s_49_13: bool = neq_int(state, tracer, s_49_11, s_49_12);
        // D s_49_14: not s_49_13
        let s_49_14: bool = !s_49_13;
        // N s_49_15: branch s_49_14 b51 b50
        if s_49_14 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #8s : i
        let s_50_0: i128 = 8;
        // S s_50_1: call Zeros(s_50_0)
        let s_50_1: Bits = Zeros(state, tracer, s_50_0);
        // S s_50_2: cast reint s_50_1 -> u8
        let s_50_2: u8 = (s_50_1.value() as u8);
        // S s_50_3: call SPEAddByteToRecord(s_50_2)
        let s_50_3: () = SPEAddByteToRecord(state, tracer, s_50_2);
        // N s_50_4: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call SPEWriteToBuffer(s_51_0)
        let s_51_1: () = SPEWriteToBuffer(state, tracer, s_51_0);
        // N s_51_2: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #13504u : u32
        let s_52_0: u32 = 13504;
        // D s_52_1: read-reg s_52_0:u64
        let s_52_1: u64 = {
            let value = state.read_register::<u64>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: cast zx s_52_1 -> bv
        let s_52_2: Bits = Bits::new(s_52_1 as u128, 64u16);
        // C s_52_3: const #1u : u8
        let s_52_3: u8 = 1;
        // C s_52_4: const #1u : u8
        let s_52_4: u8 = 1;
        // D s_52_5: call SPEAddPacketToRecord(s_52_3, s_52_4, s_52_2)
        let s_52_5: () = SPEAddPacketToRecord(state, tracer, s_52_3, s_52_4, s_52_2);
        // N s_52_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#26147 <= s_53_0
        fn_state.gs_26147 = s_53_0;
        // N s_53_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#26148 <= s_54_0
        fn_state.gs_26148 = s_54_0;
        // N s_54_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#26149 <= s_55_0
        fn_state.gs_26149 = s_55_0;
        // N s_55_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var payload_sizeshadow#493:i
        let s_56_0: i128 = fn_state.payload_sizeshadow_493;
        // D s_56_1: call __id(s_56_0)
        let s_56_1: i128 = u__id(state, tracer, s_56_0);
        // C s_56_2: const #8s : i
        let s_56_2: i128 = 8;
        // D s_56_3: mul s_56_2 s_56_1
        let s_56_3: i128 = ((s_56_2) * (s_56_1));
        // C s_56_4: const #1s : i
        let s_56_4: i128 = 1;
        // D s_56_5: sub s_56_3 s_56_4
        let s_56_5: i128 = ((s_56_3) - (s_56_4));
        // C s_56_6: const #64s : i
        let s_56_6: i128 = 64;
        // D s_56_7: cmp-lt s_56_5 s_56_6
        let s_56_7: bool = ((s_56_5) < (s_56_6));
        // D s_56_8: write-var gs#26126 <= s_56_7
        fn_state.gs_26126 = s_56_7;
        // N s_56_9: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call SPEGetDataSourcePayloadSize(s_57_0)
        let s_57_1: i128 = SPEGetDataSourcePayloadSize(state, tracer, s_57_0);
        // D s_57_2: write-var payload_sizeshadow#494 <= s_57_1
        fn_state.payload_sizeshadow_494 = s_57_1;
        // D s_57_3: read-var payload_sizeshadow#494:i
        let s_57_3: i128 = fn_state.payload_sizeshadow_494;
        // D s_57_4: call __id(s_57_3)
        let s_57_4: i128 = u__id(state, tracer, s_57_3);
        // C s_57_5: const #8s : i
        let s_57_5: i128 = 8;
        // D s_57_6: mul s_57_5 s_57_4
        let s_57_6: i128 = ((s_57_5) * (s_57_4));
        // C s_57_7: const #1s : i
        let s_57_7: i128 = 1;
        // D s_57_8: sub s_57_6 s_57_7
        let s_57_8: i128 = ((s_57_6) - (s_57_7));
        // C s_57_9: const #0s : i
        let s_57_9: i128 = 0;
        // D s_57_10: cmp-le s_57_9 s_57_8
        let s_57_10: bool = ((s_57_9) <= (s_57_8));
        // N s_57_11: branch s_57_10 b60 b58
        if s_57_10 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#26104 <= s_58_0
        fn_state.gs_26104 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#26104:u8
        let s_59_0: bool = fn_state.gs_26104;
        // N s_59_1: assert s_59_0
        let s_59_1: () = assert!(s_59_0);
        // C s_59_2: const #8s : i
        let s_59_2: i128 = 8;
        // D s_59_3: read-var payload_sizeshadow#494:i
        let s_59_3: i128 = fn_state.payload_sizeshadow_494;
        // D s_59_4: mul s_59_2 s_59_3
        let s_59_4: i128 = ((s_59_2) * (s_59_3));
        // D s_59_5: cast reint s_59_4 -> i64
        let s_59_5: i64 = (s_59_4 as i64);
        // C s_59_6: const #1s : i
        let s_59_6: i128 = 1;
        // D s_59_7: cast zx s_59_5 -> i
        let s_59_7: i128 = (i128::try_from(s_59_5).unwrap());
        // D s_59_8: sub s_59_7 s_59_6
        let s_59_8: i128 = ((s_59_7) - (s_59_6));
        // D s_59_9: cast reint s_59_8 -> i64
        let s_59_9: i64 = (s_59_8 as i64);
        // C s_59_10: const #0s : i
        let s_59_10: i128 = 0;
        // C s_59_11: const #17552u : u32
        let s_59_11: u32 = 17552;
        // D s_59_12: read-reg s_59_11:u16
        let s_59_12: u16 = {
            let value = state.read_register::<u16>(s_59_11 as isize);
            tracer.read_register(s_59_11 as isize, value);
            value
        };
        // D s_59_13: cast zx s_59_12 -> bv
        let s_59_13: Bits = Bits::new(s_59_12 as u128, 16u16);
        // D s_59_14: cast zx s_59_9 -> i
        let s_59_14: i128 = (i128::try_from(s_59_9).unwrap());
        // C s_59_15: const #1s : i64
        let s_59_15: i64 = 1;
        // C s_59_16: cast zx s_59_15 -> i
        let s_59_16: i128 = (i128::try_from(s_59_15).unwrap());
        // D s_59_17: sub s_59_14 s_59_10
        let s_59_17: i128 = ((s_59_14) - (s_59_10));
        // D s_59_18: add s_59_17 s_59_16
        let s_59_18: i128 = (s_59_17 + s_59_16);
        // D s_59_19: bit-extract s_59_13 s_59_10 s_59_18
        let s_59_19: Bits = (Bits::new(
            ((s_59_13) >> (s_59_10)).value(),
            u16::try_from(s_59_18).unwrap(),
        ));
        // C s_59_20: const #1u : u8
        let s_59_20: u8 = 1;
        // C s_59_21: const #3u : u8
        let s_59_21: u8 = 3;
        // D s_59_22: call SPEAddPacketToRecord(s_59_20, s_59_21, s_59_19)
        let s_59_22: () = SPEAddPacketToRecord(state, tracer, s_59_20, s_59_21, s_59_19);
        // N s_59_23: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var payload_sizeshadow#494:i
        let s_60_0: i128 = fn_state.payload_sizeshadow_494;
        // D s_60_1: call __id(s_60_0)
        let s_60_1: i128 = u__id(state, tracer, s_60_0);
        // C s_60_2: const #8s : i
        let s_60_2: i128 = 8;
        // D s_60_3: mul s_60_2 s_60_1
        let s_60_3: i128 = ((s_60_2) * (s_60_1));
        // C s_60_4: const #1s : i
        let s_60_4: i128 = 1;
        // D s_60_5: sub s_60_3 s_60_4
        let s_60_5: i128 = ((s_60_3) - (s_60_4));
        // C s_60_6: const #16s : i
        let s_60_6: i128 = 16;
        // D s_60_7: cmp-lt s_60_5 s_60_6
        let s_60_7: bool = ((s_60_5) < (s_60_6));
        // D s_60_8: write-var gs#26104 <= s_60_7
        fn_state.gs_26104 = s_60_7;
        // N s_60_9: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #11136u : u32
        let s_61_0: u32 = 11136;
        // D s_61_1: read-reg s_61_0:[u8; 32]
        let s_61_1: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // C s_61_2: const #1032u : u32
        let s_61_2: u32 = 1032;
        // D s_61_3: read-reg s_61_2:i64
        let s_61_3: i64 = {
            let value = state.read_register::<i64>(s_61_2 as isize);
            tracer.read_register(s_61_2 as isize, value);
            value
        };
        // D s_61_4: cast zx s_61_3 -> i
        let s_61_4: i128 = (i128::try_from(s_61_3).unwrap());
        // C s_61_5: const #0u : u8
        let s_61_5: bool = false;
        // D s_61_6: mutate-element s_61_1[s_61_4] <= s_61_5
        let s_61_6: [bool; 32usize] = {
            let mut local = s_61_1.clone();
            local[(s_61_4) as usize] = s_61_5;
            local
        };
        // D s_61_7: cast cvt s_61_6 -> [u8; 0]
        let s_61_7: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_61_6);
        // D s_61_8: cast cvt s_61_7 -> [u8; 32]
        let s_61_8: [bool; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_61_7);
            buf
        };
        // C s_61_9: const #11136u : u32
        let s_61_9: u32 = 11136;
        // N s_61_10: write-reg s_61_9 <= s_61_8
        let s_61_10: () = {
            state.write_register::<[bool; 32usize]>(s_61_9 as isize, s_61_8);
            tracer.write_register(s_61_9 as isize, s_61_8);
        };
        // N s_61_11: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #10528u : u32
        let s_62_0: u32 = 10528;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: bool = {
            let value = state.read_register::<bool>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: write-var gs#26079 <= s_62_1
        fn_state.gs_26079 = s_62_1;
        // N s_62_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #"SPE get prev br if not br" : str
        let s_63_0: &'static str = "SPE get prev br if not br";
        // S s_63_1: call __IMPDEF_boolean(s_63_0)
        let s_63_1: bool = u__IMPDEF_boolean(state, tracer, s_63_0);
        // D s_63_2: write-var include_prev_br_name <= s_63_1
        fn_state.include_prev_br_name = s_63_1;
        // C s_63_3: const #19040u : u32
        let s_63_3: u32 = 19040;
        // D s_63_4: read-reg s_63_3:u32
        let s_63_4: u32 = {
            let value = state.read_register::<u32>(s_63_3 as isize);
            tracer.read_register(s_63_3 as isize, value);
            value
        };
        // C s_63_5: const #3u : u32
        let s_63_5: u32 = 3;
        // D s_63_6: cmp-eq s_63_4 s_63_5
        let s_63_6: bool = ((s_63_4) == (s_63_5));
        // N s_63_7: branch s_63_6 b69 b64
        if s_63_6 {
            return block_69(state, tracer, fn_state);
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
        // D s_64_1: write-var gs#26113 <= s_64_0
        fn_state.gs_26113 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#26113:u8
        let s_65_0: bool = fn_state.gs_26113;
        // N s_65_1: branch s_65_0 b68 b66
        if s_65_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_66_0: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_67_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #11136u : u32
        let s_68_0: u32 = 11136;
        // D s_68_1: read-reg s_68_0:[u8; 32]
        let s_68_1: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // C s_68_2: const #1048u : u32
        let s_68_2: u32 = 1048;
        // D s_68_3: read-reg s_68_2:i64
        let s_68_3: i64 = {
            let value = state.read_register::<i64>(s_68_2 as isize);
            tracer.read_register(s_68_2 as isize, value);
            value
        };
        // D s_68_4: cast zx s_68_3 -> i
        let s_68_4: i128 = (i128::try_from(s_68_3).unwrap());
        // C s_68_5: const #0u : u8
        let s_68_5: bool = false;
        // D s_68_6: mutate-element s_68_1[s_68_4] <= s_68_5
        let s_68_6: [bool; 32usize] = {
            let mut local = s_68_1.clone();
            local[(s_68_4) as usize] = s_68_5;
            local
        };
        // D s_68_7: cast cvt s_68_6 -> [u8; 0]
        let s_68_7: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_68_6);
        // D s_68_8: cast cvt s_68_7 -> [u8; 32]
        let s_68_8: [bool; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_68_7);
            buf
        };
        // C s_68_9: const #11136u : u32
        let s_68_9: u32 = 11136;
        // N s_68_10: write-reg s_68_9 <= s_68_8
        let s_68_10: () = {
            state.write_register::<[bool; 32usize]>(s_68_9 as isize, s_68_8);
            tracer.write_register(s_68_9 as isize, s_68_8);
        };
        // N s_68_11: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var include_prev_br_name:u8
        let s_69_0: bool = fn_state.include_prev_br_name;
        // D s_69_1: not s_69_0
        let s_69_1: bool = !s_69_0;
        // D s_69_2: write-var gs#26113 <= s_69_1
        fn_state.gs_26113 = s_69_1;
        // N s_69_3: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #20880u : u32
        let s_70_0: u32 = 20880;
        // D s_70_1: read-reg s_70_0:u32
        let s_70_1: u32 = {
            let value = state.read_register::<u32>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: cast zx s_70_1 -> bv
        let s_70_2: Bits = Bits::new(s_70_1 as u128, 32u16);
        // C s_70_3: const #1u : u8
        let s_70_3: u8 = 1;
        // C s_70_4: const #5u : u8
        let s_70_4: u8 = 5;
        // D s_70_5: call SPEAddPacketToRecord(s_70_3, s_70_4, s_70_2)
        let s_70_5: () = SPEAddPacketToRecord(state, tracer, s_70_3, s_70_4, s_70_2);
        // N s_70_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #104568u : u32
        let s_71_0: u32 = 104568;
        // D s_71_1: read-reg s_71_0:u32
        let s_71_1: u32 = {
            let value = state.read_register::<u32>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // D s_71_2: cast zx s_71_1 -> bv
        let s_71_2: Bits = Bits::new(s_71_1 as u128, 32u16);
        // C s_71_3: const #1u : u8
        let s_71_3: u8 = 1;
        // C s_71_4: const #4u : u8
        let s_71_4: u8 = 4;
        // D s_71_5: call SPEAddPacketToRecord(s_71_3, s_71_4, s_71_2)
        let s_71_5: () = SPEAddPacketToRecord(state, tracer, s_71_3, s_71_4, s_71_2);
        // N s_71_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
